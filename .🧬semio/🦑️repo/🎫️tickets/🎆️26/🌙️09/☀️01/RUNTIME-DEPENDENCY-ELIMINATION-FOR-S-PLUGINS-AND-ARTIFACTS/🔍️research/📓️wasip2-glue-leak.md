# 🔍️ wasip2 glue leak — narrowing `target_arch = "wasm32"` to exclude `wasm32-wasip2`

## The bug, and why it matters

`rustc --print cfg --target wasm32-wasip2` reports `target_arch="wasm32"` (also `target_env="p2"`,
`target_os="wasi"`). Any manifest or `#[cfg(target_arch = "wasm32")]` block written to mean
"browser only" is **also active for the WASI component target**. The house-established fix
(already present in `🧩️puzzle`'s manifest before this pass) is:

```toml
[target.'cfg(all(target_arch = "wasm32", not(target_env = "p2")))'.dependencies]
```
and, in Rust code, `#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]`.

## cargo tree -i evidence

### Before (baseline, `semio-s-plugin-puzzle`, `--target wasm32-wasip2`)

- `wasm-bindgen`: **present**, 94 lines (`🗑️generated/before-wasm-bindgen.txt`)
- `js-sys`: **present**, 77 lines (`🗑️generated/before-js-sys.txt`)
- `web-sys`: **present**, 43 lines (`🗑️generated/before-web-sys.txt`)

### After this pass

- `semio-s-plugin-draw-fsm`: `wasm-bindgen` / `js-sys` / `web-sys` — **all three absent**
  (`cargo tree -i` prints "nothing to print" for all three). **Clean.**
- `semio-s-plugin-puzzle`: `wasm-bindgen` / `js-sys` / `web-sys` — **still present**, all three,
  via one single remaining path (`🗑️generated/final-tree.txt`):
  ```
  semio-s-plugin-puzzle → semio-framework-os-infinite → semio-framework (+ semio-framework-plugin)
    → semio-framework-ui → wgpu → { wasm-bindgen-futures → js-sys → wasm-bindgen, web-sys }
  semio-s-plugin-puzzle → semio-framework-os-infinite → vello / vello_svg → wgpu → (same)
  ```
  Root cause and why it was **not** fixed in this pass: see "Genuinely blocked" below.

So: the 13 named manifests' *own* leaks are closed. `draw-fsm` reaches zero third-party wasip2
deps. `puzzle` does not, because of one additional, deeper coupling discovered while fixing the
named manifests (`semio-framework-os-infinite` → `semio-framework-ui`'s wgpu-engine tier), which
turned out to require real first-party surgery beyond a `cfg` narrowing.

## Per-crate table — cfg blocks narrowed and (a)/(b) decision

| Crate / file | What was narrowed | Decision | Justification |
|---|---|---|---|
| `🎭️actor` `Cargo.toml` + `📦️glue.rs` | `[target.wasm32].deps` (`semio-framework-async`, `wasm-bindgen`); `extern crate … as wasm_bindgen_futures`; `mod kernel_host` mount | (a) browser-only | `KernelHost` is a `#[wasm_bindgen]` wrapper for React-web/wgpu-web hosts (`ShardKind::WebWorker`). wasip2 guest path is `semio-framework-plugin`'s `component-guest` feature, unrelated to this bridge. |
| `🔄️machine` `Cargo.toml` + `🦀️.rs` | `[target.wasm32].deps` (`wasm-bindgen`, `js-sys`); `mod wasm_bridge` (`WasmHost`), `mod wasm_smoke`, `export_wasm_machine` re-export | (a) browser-only, wasip2 already served | `WasmHost` uses `js_sys::Function` JS callback + `js_sys::Date`. wasip2 doesn't need it: `NativeHost` (same file) is backed by `std::time::Instant`, which WASI's clock supports, so it already covers the component target. |
| `📡️replication` `Cargo.toml` | `wasm-bindgen` dep | removed, then **restored** by a concurrent peer with the correct `not(target_env = "p2")` gate | Initially found zero call sites *inside* `📡️replication/`, so removed as dead. A peer's edit revealed the real call site lives in the path-mounted sibling `⚠️diagnostic/🦀️component.rs` (`fault_to_js`/`result_fault_to_js`, called from `🧩️puzzle`'s own wasm bridge) — genuinely browser-only, correctly restored with the narrowed gate. Lesson: grep the crate's *compiled* file set (path-mounts included), not just its own directory. |
| `🗺️surface` `Cargo.toml` + 4 component files (`🏔️terrain`, `🕸️node-graph`, `🗺️tiled-map`, `🎨️paint`) | `[target.wasm32].deps`; every `wasm_bridge`/`wasm_session` mod (session wrappers around `HtmlCanvasElement`) | (a) browser-only | Each wraps a pure `*SessionCore`/`*Host` type that stays target-neutral; only the wasm-bindgen wrapper is excluded. |
| `🧮️math` `Cargo.toml` | `wasm-bindgen` dep | removed (dead) | Zero call sites anywhere in the crate. |
| `⏳️async` `Cargo.toml` + `🦀️.rs` | `[target.wasm32].deps`; `pub mod browser` (`spawn_local`/`future_to_promise`/`JsFuture`, JS `Promise`/`JsValue` bridge) | (a) browser-only, wasip2 already served | wasip2 task execution runs through this same file's `WorkerPool` (`wasm_pool` variant, cooperative, **zero** js_sys/wasm_bindgen reference — correctly left on the bare `target_arch = "wasm32"` gate since it's needed for wasip2 too and has no browser dependency). `block_on`'s own `not(target_arch = "wasm32")` split is about `std::thread::park`/`unpark` availability, not JS — also correctly left bare. |
| `🖱️ui/🖥️host` `Cargo.toml` + `🦀️backend_alias.rs` | `[target.wasm32].deps` (`backend_webgpu`); `ActiveBackend` type alias | (a) browser-only; **no wasip2 arm exists** | `backend_webgpu` is browser-WebGPU-via-wasm-bindgen. This crate is the OS product's **native desktop windowing host**, never embedded inside a shipped plugin component — confirmed no s plugin depends on it. Left with no wasip2 `ActiveBackend` deliberately (out of scope; a headless component has no platform GPU surface to alias to). |
| `🖱️ui/🖼️render/🎯️targets/🧊️webgpu` (`backend-webgpu`) `Cargo.toml` + `📦️glue.rs` + `surface_adapter.rs` | `[target.wasm32].deps` (`wgpu` with `webgpu` feature); all 15 device-shaped module gates | (a) browser-only | "The ONLY place wgpu is permitted" per its own docstring, confined to browser builds. Not reachable from any s plugin directly (only from `ui-host` and the OS renderer engine's wgpu target). |
| `🖱️ui` (`semio-framework-ui`) `Cargo.toml` + `⌨️tui/🦀️component.rs` (`bindgen_host`/`TuiHost`) | `[target.wasm32].deps` (`wgpu` webgpu variant, `js-sys`, `wasm-bindgen`, `web-sys`); `tui-bindgen`'s `bindgen_host` mod | (a) browser-only *for the specific js-sys/wasm-bindgen/web-sys-referencing code* | `tui-bindgen`'s xterm.js bridge wraps a pure `WasmHost`. **This fix alone was insufficient to reach zero for `puzzle`** — see "Genuinely blocked" below; it was ultimately reverted at the *module-mount* level (kept at the *function/struct* level, see next row) because the wgpu-engine tier as a whole is not exclusively browser code. |
| `🖱️ui/🎯️targets/🧊️wgpu/{cursor,prepared,host,text,gpu}.rs` | Every individual `#[cfg(target_arch = "wasm32")]` fn/struct that references `wasm_bindgen`/`js_sys`/`web_sys` (`apply_canvas_cursor`, `OffscreenPresentToken`, `clipboard_read_text`/`write_text`, `fetch_font_bytes`'s wasm arm, `from_offscreen_canvas`, `begin_prepared_offscreen`) | (a) browser-only; **kept, not reverted** | These are genuinely DOM/Clipboard/fetch bindings with no wasip2 in-guest equivalent (host-mediated instead). Narrowing them individually is safe and correct regardless of whether the *module* they live in is wasip2-excluded — confirmed by two full narrow→revert→re-narrow cycles without regression. Two of the crate's `pub use` re-export lines needed the same fix at the glue-file level (`cursor::apply_canvas_cursor`, `prepared::OffscreenPresentToken`) to match. `gpu.rs`'s pre-existing `schedule_frame`/`apply_window_cursor` gates already used the equivalent `not(target_os = "wasi")` idiom — left as-is, not rewritten. |
| `✍️editor` `Cargo.toml` + `🦀️component.rs` | `[target.wasm32].deps`; `EditorSession` wasm bridge (10 cfg sites) | (a) browser-only | Canvas-attach session bridge around a pure `EditorHost`, same shape as `🗺️surface`. |
| `💻️os/🌊️flow` `Cargo.toml` + `📐️brep-geometry/🦀️component.rs` | `[target.wasm32].deps` (`getrandom` wasm_js backend, `wasm-bindgen`); `mod wasm_bridge` (`tessellate`/`dispose` JS exports) | (a) browser-only | `getrandom`'s `wasm_js` backend needs `Math.random`/`crypto.getRandomValues`; the tessellation bridge is the wasm-pack-built `createFlowSession` bundle the React renderer imports. |
| `💻️os/♾️infinite` `Cargo.toml` + `📦️glue.rs` + `🖼️canvas/🦀️component.rs` + `🎲️board/…/🕸️dag/🦀️component.rs` | `[target.wasm32].deps`; `extern crate … as wasm_bindgen_futures`; `gpu_session` mod (`CanvasGpuSession`); `wasm_session`/`wasm_bridge` mods in `dag`; `vello_backend::{util, wgpu}` re-exports | (a) mostly browser-only | Same session-bridge shape. **Two genuine (b) cases found in `dag`'s file** — see next two rows. |
| `♾️infinite/🎲️board/…/🕸️dag/🦀️component.rs`: `dag_debug_log` | Widened the *native* arm to `any(not(target_arch = "wasm32"), target_env = "p2")`, narrowed the browser arm | **(b) wasip2 gets its own implementation** | wasip2 has real stderr (WASI); `eprintln!` (the existing native path) works there, so wasip2 takes that arm instead of `web_sys::console::log_1`. |
| `♾️infinite/…/🕸️dag/🦀️component.rs`: `pointer_event_now_ms` | Same widening pattern | **(b)**, reusing an existing no-op | Pointer-event timestamps are a browser canvas concept the wasip2 guest has no pointer events to timestamp in the first place; shares the existing native `0.0` no-op rather than reading a WASI clock for zero consumers. |
| `💻️os` (`semio-framework-os-kernel`) `Cargo.toml` + `📦️glue.rs` | `[target.wasm32].deps`; `extern crate … as wasm_bindgen_futures`; `👷️worker` mount (Web Worker `postMessage` bridge, unconditional internal `wasm_bindgen` use, no internal cfg of its own) | (a) browser-only | `worker`'s mount narrowed to `not(target_env = "p2")` since the file itself has zero internal target split — wasip2 has no "Web Worker" concept for an in-guest plugin. Not currently reachable (no plugin activates the `worker`/`sync` features), fixed anyway for correctness. |
| `💻️os/🪪️identity/🦀️component.rs`: `fill_entropy` | Narrowed the browser (`crypto.getRandomValues`) arm to exclude p2; **widened the pre-existing catch-all `Err(EntropyError)` arm** to include p2 | **(b), but honestly incomplete — see below** | A correct WASI `wasi:random/random` component import needs a hand-rolled canonical-ABI binding (same shape as `ui-host`'s `semio_browser_host` raw import, but for the component model). That is real first-party work beyond this slice — **not implemented**. wasip2 instead falls into the pre-existing "every other platform" `Err` arm (not a new stub — that arm already existed for platforms with no entropy delegation), and `time_ordered_id` (same file) already degrades gracefully from an entropy failure via a clock/pid-seeded `splitmix64`. Nothing panics; nothing fabricates cryptographic strength it doesn't have. |
| `💻️os/📇️directory/🔌️client/🦀️component.rs`: `pub mod browser` | Narrowed to exclude p2 | (a) browser-only, and explicitly marked dead-experimental by its own docstring | "NOT the production browser path today" per its own doc; the `DirectoryTransportPlatform`/`DirectoryConnectionPlatform` *marker traits* (Send-ness split) were deliberately **left untouched** — they reference no third-party crate, so they're out of this bug's scope even though they're conservatively non-Send for wasip2 too. |
| `💻️os/🏪️store/🔄️sync/🦀️component.rs`: `now_ms`, `wasm_actor` | Same widen/narrow pattern as `dag`'s clock fn; `wasm_actor` mod narrowed to exclude p2 | (b) for `now_ms`, (a) for `wasm_actor` | `now_ms`: WASI's clock backs `SystemTime` fine, so wasip2 takes the native arm. `wasm_actor`: real browser WebSocket bridge, no wasip2 use — and the `sync`/`worker` features that reach this file aren't activated by any current plugin anyway. |
| `💻️os/🏪️store/🦀️component.rs`: `attach_backbone_uri`, `resolve_backbone` | **Left untouched, deliberately** | not a violation | "Only available inside the wasm sandbox" reads like a browser-only comment but the implementation (`PortBackbone`/`BackboneChannelPort`/`BackboneChannelPorts`, currently a zero-variant enum) is 100% target-neutral Rust — zero `wasm_bindgen`/`js_sys`/`web_sys` reference anywhere. The bare `target_arch = "wasm32"` gate here is *already correct* for wasip2 too (a genuinely wasm-sandbox-generic host-channel abstraction), so narrowing it would have been wrong. |

## Genuinely blocked — `semio-framework-os-infinite` → wgpu-engine tier

**This is the one open item, and it is why `semio-s-plugin-puzzle` does not yet reach zero.**

`♾️infinite`'s `🌍️world/🦀️component.rs` (14,021 lines, mounted **unconditionally** — no `cfg` at
the mount site at all) is real board/mesh3d/action-queue domain logic, not renderer glue. It
unconditionally names ~26 symbols from `semio-framework-ui`'s `"wgpu-engine"` feature tier —
`Mesh3dFault`, `BoundedActionFault`/`BoundedActionClaim`, `checked_action_string_bytes`,
`InputState`, `project_point`, `screen_segment_distance`, `DrawList`, the whole
`world3d_snapshot_*` admission state machine, and more (192 call sites, `ui_wgpu::wgpu::…`) — at
192 call sites across the file.

Those symbols live inside files (`draw.rs`, `action.rs`, `input.rs`, under
`🖱️ui/🎯️targets/🧊️wgpu/`) that **also** carry genuine top-level `use wgpu::…` (the real GPU crate,
e.g. `draw.rs`: `use wgpu::util::DeviceExt;`, unconditional). An attempt was made to:

1. Narrow every `#[cfg(feature = "wgpu-engine")]` module mount in
   `🖱️ui/🎯️targets/🧊️wgpu/📦️glue.rs` to `not(target_env = "p2")` (51 sites, mechanical).
2. Split `♾️infinite`'s `vello`/`vello_svg` dependencies (their `wgpu` feature, which is what
   actually pulls the real `wgpu` crate — `vello`'s own `default = ["wgpu", "wgpu_default"]`, and
   `vello_svg`'s `wgpu = ["vello/wgpu"]`) behind a `not(target_env = "p2")` target table, keeping
   only `Scene`/`peniko`/`kurbo` (confirmed target-neutral, ungated in vello's own source)
   unconditional.
3. Remove `semio-framework-ui`'s own unconditional base `wgpu` dependency (it duplicated what the
   per-target tables already declared).

This closed the leak in `cargo tree -i` completely (verified: all three names printed "nothing to
print" for `puzzle`) — **but broke the build**: `world.rs` failed with ~90 `E0433`/`E0425`
"cannot find X in wgpu" errors, because those 26 symbols simply stopped existing for the wasip2
target once their owning modules were excluded.

Untangling the target-neutral value/fault/queue types from the GPU-drawing code that currently
shares their file is real first-party surgery (splitting `draw.rs`/`action.rs`/`input.rs` into a
declarative tier + a GPU tier, or relocating those ~26 symbols to a target-neutral module) — not a
`cfg` narrowing, and not safe to rush. Per this ticket's explicit instruction ("do not stub
anything… an honest partial is far more useful than a component that builds but panics at
runtime"), **all three changes above were reverted** (docstrings left in place at both revert
sites — `🖱️ui/🎯️targets/🧊️wgpu/📦️glue.rs`'s top comment and `♾️infinite`'s `Cargo.toml` — explaining
exactly what was tried, why it broke, and pointing here) rather than shipping a state that builds
green but is dishonest about what's actually excluded, or one that doesn't build at all.

**Recommended follow-up** (separate ticket/slice): move `Mesh3dFault`, the `BoundedAction*`
family, `InputState`, `project_point`, `screen_segment_distance`, `checked_action_string_bytes`,
and the `world3d_snapshot_*` admission state machine out of `draw.rs`/`action.rs`/`input.rs` into
a new target-neutral module (or a sibling file within the same modules, cfg-split so only the
actual `wgpu`-touching code carries the browser-only gate). Once done, re-apply the three reverted
changes above; `cargo tree -i` should then show `puzzle` clean too.

## Build results

- `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-puzzle`: **could not be confirmed
  green in this pass.** Blocked by an **unrelated, concurrent, in-flight wave**
  (`semio-framework-os-kernel`'s `ToValue`/`FromValue`/`Serialize` derive gaps on
  `SpaceAlternative`, `SpaceCheckpoint`, `HybridLogicalTimestamp`, `Author`, etc. — the same
  serde-elimination wave `📓️status.md` already documents as "while it is red nothing downstream
  can be verified"). Confirmed these errors are 100% unrelated to this pass: zero mention of
  `wasm_bindgen`/`js_sys`/`web_sys`/`wgpu` anywhere in them (`🗑️generated/build-puzzle-3.txt`).
  `semio-s-plugin-draw-fsm` hits the identical unrelated os-kernel errors, for the same reason
  (shared dependency).
  One transient `wasm-component-ld`/`rust-lld` SIGSEGV was also observed on `semio-framework-actor`
  under `-j 8` (default), consistent with the ticket's documented resource-contention pattern (load
  average 30–36 during this session); it did **not** reproduce building `semio-framework-actor`
  alone, nor under `-j 1`/`-j 2` for the same crate in isolation — not attributable to this pass's
  changes.
- Individually-verified crates, `cargo build`/`cargo check --target wasm32-wasip2`, all green
  (before os-kernel's concurrent breakage and independent of it — none of these depend on
  os-kernel): `semio-framework-actor`, `semio-framework-machine`, `semio-framework-async`,
  `semio-framework-replication`, `semio-framework-math`.
- **Browser target (`wasm32-unknown-unknown`) — proven NOT broken** for every crate whose
  `wasm32` gates were touched and that doesn't depend on the currently-broken `os-kernel`:
  `semio-framework-actor`, `semio-framework-machine`, `semio-framework-async`,
  `semio-framework-replication`, `semio-framework-math`, `semio-framework-ui-host`,
  `semio-framework-ui-backend-webgpu`, `semio-framework-ui` — all `cargo check --target
  wasm32-unknown-unknown` clean (warnings only).
  `semio-framework-editor` and `semio-framework-surface` could **not** be checked on either target
  right now — both depend on `os-kernel`, which is red for the same unrelated concurrent reason on
  `wasm32-unknown-unknown` too (verified: identical `FromValue`/`ToValue` errors, not
  wasm-bindgen-related).

## Files touched (all still live, none stubbed)

Manifests: `🎭️actor`, `🔄️machine`, `📡️replication` (net: unchanged after peer's fix), `🗺️surface`,
`🧮️math`, `⏳️async`, `🖱️ui/🖥️host`, `🖱️ui/🖼️render/🎯️targets/🧊️webgpu`, `🖱️ui` (ui itself),
`✍️editor`, `💻️os/🌊️flow`, `💻️os/♾️infinite`, `💻️os` (os-kernel) — all 13 named manifests narrowed.

Rust: `🎭️actor/📦️glue.rs`; `🔄️machine/🦀️.rs`; `🗺️surface/{🏔️terrain,🕸️node-graph,🗺️tiled-map,🎨️paint}/🦀️component.rs`;
`⏳️async/🦀️.rs`; `🖱️ui/🖥️host/🦀️backend_alias.rs`; `🖱️ui/🎯️targets/🧊️webgpu/{📦️glue.rs,🦀️surface_adapter.rs}`;
`🖱️ui/🎯️targets/🧊️wgpu/{cursor,prepared,host,text,gpu}.rs`, `⌨️tui/🦀️component.rs`;
`✍️editor/🦀️component.rs`; `💻️os/🌊️flow/📐️brep-geometry/🦀️component.rs`;
`♾️infinite/{📦️glue.rs,🖼️canvas/🦀️component.rs,🎲️board/…/🕸️dag/🦀️component.rs}`;
`💻️os/📦️glue.rs`; `💻️os/🪪️identity/🦀️component.rs`; `💻️os/📇️directory/🔌️client/🦀️component.rs`;
`💻️os/🏪️store/🔄️sync/🦀️component.rs`.
