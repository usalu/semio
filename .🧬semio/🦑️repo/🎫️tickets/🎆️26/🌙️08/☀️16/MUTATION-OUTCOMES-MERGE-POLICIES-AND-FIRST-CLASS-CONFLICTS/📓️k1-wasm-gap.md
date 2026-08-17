# Lane K1 — Closing the wasm verification gap

## Method

Every `.rs` file under `✏️s/🔌️plugins/**` and `🧰️framework/**` containing `#[cfg(target_arch = "wasm32")]`
was found via grep (98 files; `#[cfg(feature = "wasm")]` does not exist anywhere in this repo's Rust
source, only as a Cargo feature *name* in `Cargo.toml`s, which is irrelevant here). A brace-matching
script (`🧪️k1-find-wasm-spans.py`) extracted the exact line span each such `#[cfg(...)]` attribute
governs (224 spans total), then a second script (`🧪️k1-scan-spans.py`) grepped those spans — and only
those spans — for the ticket's breaking API shapes: `Store::new(`, `.diff(`, `fn diff(`, `fn validate(`,
bare `Hint`, the deleted CRDT vocabulary, `.apply(`, and `ArtifactEnvelope { .. }` literals. Full file
list: `🧪️k1-wasm-gated-files-list.txt`; final clean scan: `🧪️k1-scan-results-final.txt`.

## Findings

**13 wasm-bindgen VCS-bridge constructors exist** (one `#[wasm_bindgen(constructor)]` per plugin/module
that wraps an `ArtifactStore<_,_>` in a `RefCell` for browser JS): `dag`, trinity `jack`, trinity
`rewrite`/world, `raster`, `process3d`, `cad`, `writer`, `gismap`, `shooting`, `fem3d`, `fem2d`, `draw`,
framework `flow`/vcs.

**12 of 13 were broken** by `ArtifactStore::new` becoming `-> Result<Self, VcsError>` (contract C6) —
every one *except* `cad`, which a prior lane had already fixed correctly and served as the reference
shape (`Store::new(envelope).map_err(|e| JsValue::from_str(&e.to_string()))?`). `dag` was the
originally-reported instance (J2); the other 11 are the same class, previously undetected because no
lane's `cargo check`/`cargo test` ever compiles `target_arch = "wasm32"` code.

**2 more sites were broken** by a second, independent break in the same API surface, found only by
actually running a wasm build (not by static grep, since the call itself is unrelated string text):
`ArtifactStore::dispatch_text`/`dispatch_binary` now return `Result<CommandReceipt, VcsError>` instead
of `Result<(), VcsError>`. Trinity's `rewrite`/world bridge and `draw`'s bridge called these without the
`.map(|_| ())` every other bridge already had, so `Result<(), JsValue>` failed to unify with
`Result<CommandReceipt, JsValue>` (`error[E0308]`, discovered mid-build, not by the pre-scan).

**Total: 14 of 15 wasm-gated call-site breakages found and fixed, across 12 files, 16 edits.**

## Fixes applied (region-local `Edit`, CAD's convention copied verbatim)

| File | Fix |
|---|---|
| `🔱️trinity/…/🔌️jack/…/✏️editor/🌉️wasm/🦀️component.rs` | `TrinityGraphStore::new(..)` × 2 → `.map_err(..)?` |
| `🔱️trinity/…/♻️rewrite/…/✏️editor/🌍️world/🦀️component.rs` | `TrinityGraphStore::new(..)` × 2 → `.map_err(..)?`; `dispatch_text`/`dispatch_binary` → `.map(\|_\| ())` added |
| `🖨️raster/…/✏️editor/🌉️wasm/🦀️component.rs` | `RasterStore::new(envelope)` → `.map_err(..)?` |
| `🏭️process/…/process3d/…/✏️editor/🌉️wasm/🦀️component.rs` | `Process3dStore::new(..)` × 2 → `.map_err(..)?` |
| `✒️writer/…/✏️editor/🌉️wasm/🦀️component.rs` | `WriterStore::new(envelope)` → `.map_err(..)?` |
| `🌍️gis/…/gismap/…/✏️editor/🌉️wasm/🦀️component.rs` | `GisMapStore::new(..)` × 2 → `.map_err(..)?` |
| `🎥️shooting/…/✏️editor/🌉️wasm/🦀️component.rs` | `ShootingStore::new(..)` × 2 → `.map_err(..)?` |
| `🏗️fem/…/🧊️3d/…/✏️editor/🌉️wasm/🦀️component.rs` | `Fem3dStore::new(..)` × 2 → `.map_err(..)?` |
| `🏗️fem/…/◻2d/…/✏️editor/🌉️wasm/🦀️component.rs` | `Fem2dStore::new(..)` × 2 → `.map_err(..)?` |
| `🖍️draw/…/✏️editor/🦀️component.rs` | `DrawStore::new(..)` × 2 → `.map_err(..)?`; `dispatch_text`/`dispatch_binary` → `.map(\|_\| ())` added |
| `🧰️framework/…/🌊️flow/🌿️vcs/🦀️component.rs` | `FlowStore::new(..)` × 2 → `.map_err(..)?` |
| `🧰️framework/…/♾️infinite/🎲️board/…/🕸️dag/🦀️component.rs` | `DagStore::new(..)` × 2 → `.map_err(..)?` (the original J2 finding) |

Every fix follows the exact convention already established at
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`:
inside a wasm bridge, a constructor failure surfaces to the host as `Err(JsValue)`, never an `unwrap`/
`expect` abort.

## Found but NOT broken / NOT fixed (verified, left alone)

- **`🏪️store/🔄️sync/🦀️component.rs`'s `mod wasm_actor` (framework, in-lease) still references
  `SpaceConflict`** (contract C10 marks it for deletion). It is **not currently broken** — `SpaceConflict`
  still exists at `🏪️store/🦀️component.rs:6041` — so the deleting lane simply hasn't reached this file
  yet. Left untouched; flagging for whichever lane does the C10 `SpaceConflict` deletion sweep.
- **`ProgramBridge/🧊️component.rs:375`'s `.apply(&JsValue::NULL, &args)`** is `js_sys::Function::apply`
  (a JS host call, wasm32-gated only because it's the browser backend arm), unrelated to
  `MutationDiff::apply`. False positive, no action.
- No `.diff(` calls, `fn diff(` leaves, `fn validate(` overrides, bare `Hint`, or `ArtifactEnvelope { .. }`
  struct literals exist anywhere inside a wasm32-gated span repo-wide — the mutation-diff/outcome
  machinery itself is target-agnostic and only reached indirectly through `store.dispatch_text/binary`,
  whose signature these bridges already treat opaquely (once the `.map(|_| ())` gap above is closed).

## Build verification (real, not simulated)

Built via `bun 🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts plugin <id>`
(`wasm32-wasip2` target — `target_arch = "wasm32"` covers this triple too, so it compiles the exact
code this lane's lease covers; confirmed with `PLUGIN_WASM_TARGET = "wasm32-wasip2"` in that script).

| Plugin | Result |
|---|---|
| **`dag`** (+ `stdio`) | **clean, 0 errors** — `[DEBUG] built program dag (wasm32-wasip2, dev)` |
| **`cad`** (+ `stdio`) | **clean, 0 errors** |
| **`process3d`** (+ 4 extensions + `stdio`) | **clean, 0 errors** |
| **`trinity` + `writer`** (+ `stdio`) | broke on the `dispatch_text`/`dispatch_binary` bug above on first attempt; **clean, 0 errors** after the fix |
| `raster` (+ `stdio`) | blocked by 4 **unrelated, pre-existing** errors in `raster`'s own non-wasm-gated code: `DwgSnapshot.bytes` field access (matches FULL-STDIO's charter, file last touched 2026-08-13), an unresolved `super::composite` path, and a `base64::GeneralPurpose::decode/encode` API mismatch (file touched today 2026-08-16 by a concurrent lane). My wasm-bridge fix itself compiles clean — none of the 4 errors are in `🌉️wasm/component.rs`. |
| `draw` (+ `stdio`) | blocked by 1 **unrelated, pre-existing** error: `use ui_wgpu::wgpu::SurfaceKind` unresolved in `🖼️canvas/component.rs` (file touched today 2026-08-16 — matches the brief's `UiNode`/`ui_wgpu`/`InteractionView` exclusion). My wasm-bridge fix itself compiles clean. |

**5 plugins built end-to-end with zero errors** (dag, cad, process3d + 4 extensions, trinity, writer),
satisfying "at least 4 including dag." 2 more (raster, draw) had my specific fix verified clean but hit
unrelated concurrent-lane breakage elsewhere in the same crate — attributed via `git log --date=iso`,
not fixed (outside this lane's lease).

**Framework wasm target**: attempted `bun 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts wasm`
(the `@semio-tech/framework-os-rs` wasm-bindgen target, `wasm32-unknown-unknown`). It fails on an
unrelated, pre-existing Cargo feature-flag gap: `getrandom` needs its `wasm_js` backend feature enabled
for this target (`error: The "wasm_js" backend requires the wasm_js feature for getrandom`) — a
`Cargo.toml` dependency-feature problem, not a `#[cfg(target_arch = "wasm32")]` *code* problem, so
outside this lane's lease. Full log: `🧪️k1-wasm-build-os-host.txt`.

## Native regression checks

```
cargo check -p semio-s-plugin-dag        → clean, exit 0 (🧪️k1-cargo-check-dag.txt)
cargo test -p semio-framework-os-kernel --lib
  → test result: ok. 987 passed; 0 failed; 0 ignored   (🧪️k1-cargo-test-os-kernel.txt — matches the pre-lane baseline exactly, no regression)
bun ./📜️script.ts verify mutation-outcome-law
  → [verify mutation-outcome-law] passed.               (🧪️k1-verify-mutation-outcome-law.txt — 0 breaches)
```

## Files touched

- Edited (12): trinity jack wasm bridge, trinity rewrite world bridge, raster wasm bridge, process3d
  wasm bridge, writer wasm bridge, gismap wasm bridge, shooting wasm bridge, fem3d wasm bridge, fem2d
  wasm bridge, draw editor component, framework flow/vcs component, framework dag component — all listed
  with full paths in the table above.
- Logs/scripts added (this folder, all `.txt`/`.py`, none deleted): `🧪️k1-find-wasm-spans.py`,
  `🧪️k1-scan-spans.py`, `🧪️k1-wasm-gated-files-list.txt`, `🧪️k1-scan-results-final.txt`,
  `🧪️k1-wasm-build-{dag,cad,process3d,raster,writer,writer2,draw,os-host}.txt`,
  `🧪️k1-cargo-check-dag.txt`, `🧪️k1-cargo-test-os-kernel.txt`, `🧪️k1-verify-mutation-outcome-law.txt`.
- Not touched (found, attributed, left for the owning lane/ticket): `🏪️store/🔄️sync/🦀️component.rs`
  (`SpaceConflict`, not yet broken), `raster`'s DwgSnapshot/composite/base64 sites (FULL-STDIO +
  unrelated), `draw`'s canvas `ui_wgpu` site (concurrent UI-reshaping lane), `framework/os/host`'s
  `getrandom` wasm feature-flag gap (Cargo.toml, not lease-covered code).

Ticket not closed (lane instruction: never close a shared ticket).
