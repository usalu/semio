# Wave G1a — `semio-framework-os-flow` + `semio-s-plugin-flow-extension-brep`

Boundary: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/` (crate `semio-framework-os-flow`) and `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/` (crate `semio-s-plugin-flow-extension-brep`). Nothing outside those two was edited.

## Headline

**Job 1 (delete the framework `brep-geometry` module) could NOT be safely executed and was NOT performed.** The mission's premise — "the plugin already has a near-duplicate, reconcile to one" — is factually wrong for the current tree: the plugin has **zero** local copies of this content; it reaches the framework module through a glob import, and so do **two plugin crates entirely outside this ticket's boundary**. Deleting the module would have broken those crates, which the mission's own binding rule for Job 2 ("non-zero call sites outside your boundary → do not touch, report, leave the function") requires me not to do. Full evidence below.

**Job 2 (delete the DWG bridge functions) was executed for the one function with zero call sites.** The other three have real, live external consumers and were left in place with patches reported below.

## Job 1 — investigation and why the module stays

### What was actually read
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️component.rs` (563 LOC, in full).
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs` (1928 LOC, in full).

### Duplication analysis — there is none
The plugin's `component.rs` opens with:
```rust
use flow_extension_sdk::brep_geometry::*;
```
`flow_extension_sdk` is a Cargo alias, declared in the plugin's own manifest:
```toml
flow_extension_sdk = { path = "...🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
```
i.e. `flow_extension_sdk::brep_geometry` **is** `semio-framework-os-flow`'s `brep_geometry` module, glob-imported. The plugin's 1928 lines are almost entirely `geo_operation!`/`num_operation!`/… macro-generated `Operator` impls and a ~700-line `reg_geo(...)` registration table (`register()`); they *call* `with_kernel`, `with_kernel_read`, `geometry_dict`, `read_channel_number`, etc. — they never *redefine* them. There is no second copy of `KERNEL`, `MESH_CACHE`, `with_kernel`, `tessellate_geometry`, `export_solid_json`, `import_solid_json`, `dispose_geometry`, or `retain_geometry_handles` anywhere in the plugin. **Nothing was genuinely duplicated; everything is genuinely unique to the framework file.** The "near-duplicate" framing in the mission does not match the tree.

### Why deletion breaks things outside the boundary
`🌊️flow/📦️packages/🦀️rust/📦️glue.rs` mounts the module and re-exports 5 of its ~40 public items at the crate root:
```rust
#[path = "../../📐️brep-geometry/🦀️component.rs"]
pub mod brep_geometry;
pub use brep_geometry::{
    dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry,
};
```
Census of all 5 re-exported names, repo-wide (`grep -rl`, `.rs`/`.ts`/`.tsx`, excluding `target`):

| function | real external (outside-boundary) callers |
|---|---|
| `tessellate_geometry` | `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs:670` — `flow::tessellate_geometry(handle, tolerance)` |
| `export_solid_json` | `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs:526` |
| `import_solid_json` | `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs:537` |
| `dispose_geometry` | none found outside this boundary |
| `retain_geometry_handles` | none found outside this boundary |

`playbook/🧩️extensions/🌀️procedural/🦀️component.rs:4` imports all three of `export_solid_json`, `import_solid_json`, `tessellate_geometry` from `flow::{...}` (same `extern crate self as flow;` alias) and calls them at lines 448, 526, 537. (One initial hit, `✏️s/🔌️plugins/📐️cad/…/🗿️subsets/✳️any/🚪️io/🗺️geometry-import/🦀️component.rs`, was a false positive on the substring `tessellate_geometry` — it defines its own unrelated `tessellate_geometry_handle(kernel: &mut dyn BrepKernel, …)` and never touches this crate's kernel.)

So the ambient `KERNEL`/`MESH_CACHE` in `brep-geometry/component.rs` is not "brep operator state accidentally left in the framework" — it is **shared ambient state that the brep plugin, the procedural-3d app, and the playbook-procedural extension all read/write through the same process-global instance**, so a solid handle produced by one is tessellatable/exportable by the others. Moving it wholesale into the brep plugin crate would either (a) break those two out-of-boundary crates' compiles (`flow::tessellate_geometry` etc. would vanish from the framework), or (b) silently desync geometry handles at runtime if the plugin got its own copy of the statics instead (a handle minted in the plugin's kernel would not resolve in the framework's, and vice versa) — the second failure mode is invisible to `cargo check` and was the reason I did not attempt a "give the plugin its own local statics" workaround.

Two more in-boundary consumers were found and would also break, both inside `semio-framework-os-flow` itself, reached only via `crate::…` (the glue.rs re-export), not via the (dead) `use crate::brep_geometry::{…}` import lines:
- `🖥️host/🦀️component.rs:887,1837` — production eval-loop calls `crate::retain_geometry_handles(...)` after every flow-graph tick (session GC hook), plus a `#[cfg(test)]` integration test at `:4041` that tessellates a real `Extrude` result to assert non-empty mesh output.
- `🌉️wasm/🦀️component.rs:614,665` — `#[cfg(target_arch="wasm32")]` `#[wasm_bindgen]` exports `tessellate(handle, tolerance)` / `dispose(handle)`, the JS-facing 3D-preview API.

Of the 8 `use crate::brep_geometry::{dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry};` import lines (mission estimated 9; actual count is 8 — `📄️artifact`, `📚️catalogue`, `🌉️bridge`, `🖥️host`, `🌉️wasm`, `🌿️vcs`, `🖍️drawing`, `📔️registry`), 6 are dead (the imported names are never referenced unqualified in those files — `host.rs`/`wasm.rs` reach the functions via `crate::` instead). I left these alone too: removing 6 unused imports produces zero functional benefit while the module they point at stays mounted, and touching 6 files for a cosmetic win wasn't worth the incremental risk given the real work (Job 1's actual goal) is blocked.

**Net effect: `🌊️flow/📐️brep-geometry/🦀️component.rs`, `🌊️flow/📦️packages/🦀️rust/📦️glue.rs`, and the 8 sibling import lines are all untouched.** Properly dissolving this module needs a coordinated wave that also touches `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs` and `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs` (both outside this ticket's boundary) plus `🖥️host/🦀️component.rs`'s production retention hook and `🌉️wasm/🦀️component.rs`'s wasm-bindgen preview API (in-boundary, but non-trivial: the mission's own text notes `semio-framework-os-flow` is *not* in the forbidden stdio-closure, suggesting the real fix is for this crate to depend on a real stdio-side geometry artifact instead of hosting an ad-hoc `OnceLock` kernel — that redesign is out of scope for a bounded wave).

## Job 2 — DWG bridge functions

Census (`grep -rln`, `.rs`/`.ts`/`.tsx`, excluding `target`) of the 4 named functions:

| function | file | call sites found | action |
|---|---|---|---|
| `dwg_decode_mesh_json` | `🌉️wasm/🦀️component.rs` | **zero** (only its own definition) | **deleted** |
| `dwg_encode_mesh_json` | `🌉️wasm/🦀️component.rs` | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts:2725-2733` — dynamically imports the compiled `flow_core.js` wasm and calls it | kept, see sharedFileRequests |
| `export_dwg_sync` | `🖍️drawing/🦀️component.rs:478` | called by `DrawingKernel::export_dwg` (`:962`, same file) → called by `export_dwg_json` (`:1073`) → called by `🌉️wasm/🦀️component.rs:641` (in-boundary, fine) **and** `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs:643,701` (outside boundary) | kept, see sharedFileRequests |
| `import_dwg_sync` | `🖍️drawing/🦀️component.rs:513` | same chain via `import_dwg` (`:966`) → `import_dwg_json` (`:1088`) → `🌉️wasm/🦀️component.rs:647` **and** `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs:643,705,982` | kept, see sharedFileRequests |

Only `dwg_decode_mesh_json` had zero call sites anywhere. It was removed from `🌉️wasm/🦀️component.rs` (the whole `#[cfg(target_arch = "wasm32")] #[wasm_bindgen] pub fn dwg_decode_mesh_json(...)` block, ~15 lines including its doc comment). `dwg_encode_mesh_json` remains untouched in the same file (still needed by `dwg_from_bytes`/`dwg_to_bytes` etc. from `semio_framework`, unaffected by the removal).

## sharedFileRequests

1. **`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts`** (lines 2725-2733) — once the DWG codec has actually moved into stdio's `🖊️dwg` artifact, this file's dynamic `import("...flow_core.js")` + `flowCore.dwg_encode_mesh_json(meshJson)` call needs to switch to whatever the new stdio DWG artifact's JS/WASM entry point is. Exact patch: replace the `flow_core.js` import and the `dwg_encode_mesh_json` call with the equivalent call on the new artifact; then `dwg_encode_mesh_json` can be deleted from `🌉️wasm/🦀️component.rs`.

2. **`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs`** (lines 643, 701, 705, 982) — imports and calls `export_dwg_json`/`import_dwg_json` (which bottom out in `export_dwg_sync`/`import_dwg_sync`). Once the DWG codec moves to stdio's `🖊️dwg` artifact, this file's imports need to point at the new artifact's export/import surface instead of `flow_extension_sdk::{export_dwg_json, import_dwg_json}`; then `export_dwg_sync`/`import_dwg_sync`/`export_dwg_json`/`import_dwg_json` can be deleted from `🖍️drawing/🦀️component.rs`, and `🌉️wasm/🦀️component.rs:634-648`'s `export_drawing_dwg`/`import_drawing_dwg` wasm-bindgen exports updated to call the new artifact directly.

3. **`✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs:670`** and **`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs:4,448,526,537`** — both call `flow::tessellate_geometry`/`export_solid_json`/`import_solid_json` directly against the framework's ambient brep kernel (see Job 1 above). Any future attempt to actually dissolve `🌊️flow/📐️brep-geometry/🦀️component.rs` must update both of these files (and `🖥️host/🦀️component.rs`'s retention hooks, in-boundary) in the same change — it cannot be done crate-by-crate without a transition period where handles desync.

## Verification (real output, not claimed)

Baseline was captured before any edit (per the mandated recipe: `RUSTC_WRAPPER=""`, `--all-targets`, `CARGO_TARGET_DIR` pinned to the ticket's `🎯️target`). Contrary to the mission's claim of ">100 pre-existing errors in `host.rs`/`vcs.rs`/`playbook.rs`", the actual baseline for `semio-framework-os-flow --all-targets` is **6 errors, all in an unrelated upstream dependency** (`🧰️framework/🔨️modules/🖱️ui/…/🎯️targets/🧊️wgpu/…` → `🧊️3d/🎬️scene/🦀️component.rs`, `E0433`/`E0689`/`E0432` — missing `semio_framework_math` crate + ambiguous float types + missing `wgpu::Vec3`). None of them are in `host.rs`, `vcs.rs`, or `playbook.rs`; the compile evidently halts at this shared upstream dependency before those files are ever type-checked, which is presumably why the mission's stale ">100 errors" figure isn't visible in the current tree (another session may have fixed it since, or the earlier count was from a different failure mode). I did not touch `🖱️ui` or `🧊️3d/🎬️scene` — outside my boundary — and report this discrepancy rather than silently trusting the mission's number.

```
$ TD=".../DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS/🎯️target"
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-framework-os-flow --all-targets --message-format=short 2>&1 | grep -E ": error" | sort
🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/../../../../../🧊️3d/🎬️scene/🦀️component.rs:3:9: error[E0433]: cannot find module or crate `semio_framework_math` in this scope: use of unresolved module or unlinked crate `semio_framework_math`
🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/../../../../../🧊️3d/🎬️scene/🦀️component.rs:842:28: error[E0689]: can't call method `abs` on ambiguous numeric type `{float}`
🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/../../../../../🧊️3d/🎬️scene/🦀️component.rs:861:41: error[E0689]: can't call method `sqrt` on ambiguous numeric type `{float}`
🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/../../../../../🧊️3d/🎬️scene/🦀️component.rs:864:41: error[E0689]: can't call method `sqrt` on ambiguous numeric type `{float}`
🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/../../../../../🧊️3d/🎬️scene/🦀️component.rs:867:41: error[E0689]: can't call method `sqrt` on ambiguous numeric type `{float}`
🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️widgets.rs:594:45: error[E0432]: unresolved import `crate::wgpu::Vec3`: no `Vec3` in `wgpu`
```
After my one edit (deleting `dwg_decode_mesh_json` from `🌉️wasm/🦀️component.rs`), the same command produced the **identical 6 lines**, and `diff baseline after` was empty. Full output saved at `scratch-g1a-baseline-errors.txt` / `scratch-g1a-after-errors.txt` in this ticket folder.

The plugin crate (mission expected zero) also baselines at the same 6 lines (same shared upstream dependency, `scratch-g1a-plugin-baseline-errors.txt`), not zero — again contradicting the mission's assumption. After my edit (which didn't touch the plugin crate at all), `scratch-g1a-plugin-after-errors.txt` is identical; `diff` empty.

Both gates: **no new error lines, none removed** (the removed function had no compile footprint of its own — it wasn't erroring before either).

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs` — deleted `dwg_decode_mesh_json` (dead, zero call sites).

## Files investigated but NOT modified (with reason)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️component.rs` — not deleted; real external consumers (Job 1 above).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` — mount/pub-use left in place for the same reason.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/{📄️artifact,📚️catalogue,🌉️bridge,🖥️host,🌿️vcs,📔️registry}/🦀️component.rs` and `🖍️drawing/🦀️component.rs` — the 8 `use crate::brep_geometry::{...}` lines left in place (6 are dead imports with zero functional effect either way; `host.rs` and `drawing.rs` also have the DWG/tessellate call sites documented above).
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs` — read in full; no duplication found to reconcile, so no edit was made.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs` — `dwg_encode_mesh_json`, `tessellate`, `dispose` left in place (real consumers / dependency on the still-present `brep_geometry` module).

## Honest remainders

- Job 1 is functionally **not done** — the anti-pattern (`static KERNEL: OnceLock<...>` in the framework) still exists. It cannot be removed within this ticket's two-crate boundary without breaking `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs` and `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs`. Recommend a follow-up ticket scoped to all four consumer crates (+ `host.rs`'s retention hook) simultaneously, ideally landing the "artifact-backed geometry kernel" replacement the mission's own text hints at (`semio-framework-os-flow` is explicitly not in the forbidden stdio-closure).
- Job 2 is 1-of-4 done; the other 3 DWG functions are documented with exact patch targets in sharedFileRequests above and were deliberately left alone per the mission's own "non-zero call sites outside your boundary → leave it" rule.
- The mission's baseline assumption (">100 pre-existing errors in host.rs/vcs.rs/playbook.rs", "plugin crate gates at zero") did not match the measured tree; I did not silently substitute my own gate — I ran the mandated recipe verbatim and reported the real numbers.
