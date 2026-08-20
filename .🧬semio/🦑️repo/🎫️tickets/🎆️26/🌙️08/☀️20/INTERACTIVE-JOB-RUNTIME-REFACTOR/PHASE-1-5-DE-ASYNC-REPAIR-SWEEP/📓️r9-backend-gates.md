# 📓️ R9 — Cross-Platform Backend Gating (WebGPU, Metal, workspace sweep)

Packet R9 of Phase 1.5. Ownership boundary: the four hand-written `ui_render::GraphicsBackend` crates
under `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/` and the `ui-host` `Cargo.toml` wiring that gates
them as dependencies. Follows R6's established pattern (`📓️r6-d3d12-gates.md`) exactly.

## 1. Pattern applied (identical across all four backends)

Per-crate `📦️glue.rs`:
1. Every `mod`/`pub use` gated behind `#[cfg(target_os = "…")]` or `#[cfg(target_arch = "wasm32")]`,
   matching the crate's own `Cargo.toml` `[target.'cfg(...)'.dependencies]` gate.
2. The top-level unconditional `compile_error!` removed. On the wrong platform the crate now compiles to
   an empty, zero-item lib instead of hard-failing.
3. Header doc comment rewritten to document why (cfg-gated empty lib, not an oversight) and cross-link
   the other three backends' identical treatment.

Intent preservation ("wrong-platform use is an error") still holds one layer up: `🖱️ui/🖥️host/📦️packages/
🦀️rust/Cargo.toml` already pulls each backend in only under its own `[target.'cfg(...)'.dependencies]`
entry, so a consumer on the wrong platform never sees the dependency edge — referencing
`WebGpuBackend`/`MetalBackend`/etc. from such a consumer still fails to compile, as an unresolved
import at the actual misuse site rather than a banner at this crate's own root. No change was needed
there — R3/R6 already put this wiring in place for all four backends.

## 2. Crates changed this packet

### `semio-framework-ui-backend-webgpu`
Gate: `#[cfg(target_arch = "wasm32")]` (not `target_os` — this backend is browser-only, gated on arch
because `Cargo.toml`'s `wgpu`/`web-sys`/`wasm-bindgen` deps are behind
`[target.'cfg(target_arch = "wasm32")'.dependencies]`). R5 confirmed the crate's own code was already
100% correct and made no source edits; R6 left the exact minimal fix as a coordination note in its
report. This packet applied exactly that: gated all 10 `mod` declarations + the `pub use backend::
WebGpuBackend` in `📦️glue.rs`, removed the top-level `compile_error!`.

### `semio-framework-ui-backend-metal`
Gate: `#[cfg(target_os = "macos")]`, matching `Cargo.toml`'s `objc2`/`objc2-metal`/`objc2-quartz-core`/
`objc2-foundation`/`objc2-core-foundation`/`raw-window-handle` deps under
`[target.'cfg(target_os = "macos")'.dependencies]`. Gated all 8 `mod` declarations +
`pub use backend::{MetalBackend, MetalGraphicsError}` in `📦️glue.rs`, removed the top-level
`compile_error!`. Metal is the native backend on this (macOS) host so its `compile_error!` was never
firing here — the fix specifically targets Linux/Windows hosts, verified by cross-compiling below.

Note: both `metal/🦀️backend.rs` and `vulkan/🦀️backend.rs` and `d3d12/🦀️backend.rs` also carry their own
`#[cfg(not(target_os = "…"))] compile_error!(...)` at the top of the file itself, left untouched
(matching R6's precedent of only touching `📦️glue.rs`). These are now dead/unreachable code on the wrong
platform — the `mod backend;` declaration that would pull the file in is itself `#[cfg(target_os =
"…")]`-gated in `📦️glue.rs`, so the file is never parsed on the wrong host and its internal
`compile_error!` never fires. Confirmed harmless by the cross-target verification below (0 errors on the
wrong platform for all three).

## 3. Sweep of the rest of the workspace

- `grep -rl "compile_error!"` across the whole repo (excluding `./compose`, `/target/`) found exactly
  one crate family carrying platform-gate `compile_error!`s: the four render backends (now all fixed —
  metal/vulkan/d3d12 by R6+this packet, webgpu by this packet). The only other hit,
  `🧰️framework/🔨️modules/🔀️dispatch/🦀️component.rs`, is a proc-macro emitting `compile_error!` into
  *generated* code as a deliberate diagnostic for macro misuse — unrelated to platform gating, not
  touched.
- `grep -rl` for `use windows::`, `use ash::`/`ash::vk`, `objc2`/`objc2_metal`/`objc2_foundation`/
  `objc2_quartz_core`/`objc2_core_foundation`, and `web_sys::`/`wasm_bindgen::` across the whole repo:
  every hit inside the four render-backend crates is now covered by the gating above (confirmed by
  re-running the same greps after the edits — all surviving hits are inside `#[cfg(...)]`-gated `mod`s).
  Hits elsewhere (various plugin `component.rs`/`os-kernel`/`renderer` files, `🖱️ui/📦️packages/🦀️rust/
  🎯️targets/🧊️wgpu` and `⌨️tui`, `💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/…/🎯️targets/🧊️wgpu`) belong to a
  **different, unrelated UI/renderer family** — declarative retained-mode UI and a separate os-kernel wasm
  renderer, not the `ui_render::GraphicsBackend` hand-written-backend family this packet owns — and fall
  either inside the concurrent sibling packet's `os-kernel`/`framework-ui` boundary or use ordinary
  feature flags (`wgpu-engine`) rather than platform `compile_error!` gates. None of them appeared in the
  `cargo check --workspace` `could not compile` list (see §5), so none needed touching; flagging by name
  above rather than silently skipping.
- No other workspace member carries a top-level `compile_error!` platform gate or an ungated
  platform-specific import outside the four backends' own tree.

## 4. Per-crate verification actually run

| Crate | Command | Result |
|---|---|---|
| webgpu | `cargo check -p semio-framework-ui-backend-webgpu --all-targets` (macOS, wrong platform) | 0 errors — empty lib, only `Checking`/`Finished` |
| webgpu | `cargo check -p semio-framework-ui-backend-webgpu --all-targets --target wasm32-unknown-unknown` (real target) | 0 errors, pre-existing warnings only (unrelated to this edit — same warnings R5 already catalogued) |
| metal | `cargo check -p semio-framework-ui-backend-metal --all-targets` (macOS, real/native platform) | 0 errors, pre-existing warnings only (`unnecessary qualification` on `std::mem::size_of`, dead-code on `surface_format`/`format` — unrelated to this edit) |
| metal | `cargo check -p semio-framework-ui-backend-metal --all-targets --target x86_64-unknown-linux-gnu` (wrong platform) | 0 errors — empty lib |
| metal | `cargo check -p semio-framework-ui-backend-metal --all-targets --target x86_64-pc-windows-msvc` (wrong platform) | 0 errors — empty lib |
| vulkan | `cargo check -p semio-framework-ui-backend-vulkan --all-targets` (macOS, regression check — untouched this packet) | 0 errors |
| vulkan | `cargo check -p semio-framework-ui-backend-vulkan --all-targets --target x86_64-unknown-linux-gnu` (real target, regression check) | 0 errors |
| d3d12 | `cargo check -p semio-framework-ui-backend-d3d12 --all-targets` (macOS, regression check — untouched this packet) | 0 errors |
| d3d12 | `cargo check -p semio-framework-ui-backend-d3d12 --all-targets --target x86_64-pc-windows-msvc` (real target, regression check) | 0 errors |
| workspace | `cargo check --workspace --all-targets --exclude semio-compose-rs` | see §5 — no render-backend crate in `could not compile` |
| deps | `bun ./📜️script.ts verify dependencies` | clean, 238 = 238, no new third-party deps |
| fmt | `rustfmt --check --config-path ./rustfmt.toml` on the 2 edited files | pre-existing `mod`-ordering drift only (identical category to R6's already-accepted vulkan/d3d12 `📦️glue.rs` — confirmed by running the same check against those untouched files, same diff shape). Left unformatted, consistent with R1/R3/R6 precedent of not taking a wholesale-reformat diff on a live, concurrently-edited tree. |

`rustup target list --installed` confirmed `wasm32-unknown-unknown`, `x86_64-pc-windows-msvc`,
`x86_64-unknown-linux-gnu` already installed on this host — no `rustup target add` needed.

## 5. Workspace-wide `could not compile` after this change

```
error: could not compile `semio-framework-hash` (lib test) due to 29 previous errors
error: could not compile `semio-framework-machine` (lib) due to 3 previous errors
error: could not compile `semio-framework-machine` (lib test) due to 6 previous errors; 1 warning emitted
error: could not compile `semio-framework-ui` (lib test) due to 84 previous errors; 2 warnings emitted
error: could not compile `semio-framework-os-kernel` (lib test) due to 16 previous errors; 20 warnings emitted
error: could not compile `semio-compose-rs` (lib) due to 18 previous errors; 89 warnings emitted
```

**No backend crate appears.** All six remaining failures are out of this packet's ownership boundary:

- `semio-framework-machine` (lib + lib test) — R3's/R6's previously flagged de-async bug class, owned by
  the concurrent sibling packet on `semio-framework-machine`. Not touched.
- `semio-framework-hash` (lib test) — its 29 errors are entirely `E0053`/`E0277`/`E0369`
  `Future`-shaped errors whose reported path resolves into `🔄️machine/📦️packages/🦀️rust/../../
  🦀️component.rs` — i.e. this is `semio-framework-machine`'s own async bug bleeding into a crate that
  depends on it, not an independent defect. Same sibling packet's boundary; not touched.
- `semio-framework-ui` (lib test) — previously flagged by R1 (`Label` gate), owned by the concurrent
  sibling packet on `os-kernel`/`framework-ui` test targets. Not touched.
- `semio-framework-os-kernel` (lib test) — same sibling packet's boundary. Not touched.
- `semio-compose-rs` — explicitly out of scope per the packet brief; `--exclude` does not stop it being
  reached as some other crate's path dependency (R6 already noted and did not investigate further; same
  behavior reconfirmed here, still not investigated, still out of scope).

## 6. Is this gating structure a good foundation for Phase 10?

Yes. Phase 10 replaces wgpu/naga with direct per-platform D3D12/Metal/Vulkan/WebGPU bindings — which is
exactly the shape all four backends already have today: one crate per platform, each with its own
`Cargo.toml` target-cfg'd dependency block, each `📦️glue.rs` gating its whole module tree on the same
`cfg` the dependencies live behind, each compiling to a genuinely empty lib (not a stub with dead code)
everywhere else. There is nothing Phase 10 needs to restructure here — it only needs to keep writing new
per-platform code inside the same four crates (or add a fifth, following the identical template if a
platform split further). The one thing worth calling out for whoever picks up Phase 10: the pattern is
now uniform across `target_os` (macos/linux/windows) *and* `target_arch` (wasm32) gates, so a fifth
backend gated on yet another axis (e.g. a future `target_env` split) can copy either shape directly.

## 7. Files touched

- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/📦️glue.rs` — gated every
  `mod`/`pub use` behind `#[cfg(target_arch = "wasm32")]`; removed the top-level `compile_error!`;
  rewrote header to document the cfg-gated-empty-lib approach, cross-linking the other three backends.
- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/📦️glue.rs` — gated every
  `mod`/`pub use` behind `#[cfg(target_os = "macos")]`; removed the top-level `compile_error!`; rewrote
  header identically.
- No other file touched. `semio-framework-ui-backend-vulkan` and `semio-framework-ui-backend-d3d12` were
  not edited (R6's prior fix, re-verified as a regression check only).

## 8. Cross-boundary findings for the coordinator

Nothing new to flag beyond what R6 already flagged (Metal — now fixed by this packet;
`semio-framework-machine`, `semio-hub`, `semio-framework-ui` lib-test — all previously reported,
reconfirmed still present, still out of this packet's boundary). `semio-hub` did not appear in this run's
`could not compile` list (previously flagged by R6) — plausibly consistent with the documented run-to-run
non-determinism in the packet brief's measurement caveat rather than a fix; not investigated further as
it is out of this packet's ownership.
