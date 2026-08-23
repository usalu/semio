# Terra Audit: Phase 3 Paged Mesh3d/Terrain/Face-Overlay Checkpoint — 2026-08-23

## Verdict

**REJECT — source packet, one isolated blocker.** The required scoped format gate is red for the cohort-touched `os_host.rs`: canonical rustfmt orders `default_intent_exchange` before `AppKernelSeam`, while the current staged source has the inverse. No production source was changed by this audit. The retained Mesh3d source evidence otherwise passes the requested static inspection; Phase 3 remains independently RED.

## Independent Source Findings

| Check | Result | Evidence |
| --- | --- | --- |
| Mounted root and legacy fallback | PASS | `♾️infinite/🦀️component.rs` consists solely of `pub use crate::world::*;`. Its production `Mesh3d`, `GpuContext`, and `Vec<` census is exactly 0: no second mounted World, direct field, or compatibility Vec fallback is compiled through that root. |
| Fixed Mesh3d authority | PASS by source inspection | `Mesh3dWriteToken`/`Mesh3dLease` use 256 slots, 16 KiB pages, 1,024 pages (16 MiB) per owner, and 4,096 process pages. Schema validation occurs before slot insertion; tokens carry slot epoch/generation/revision; stale, page/item/byte, order, incomplete, abort, one-page close, and terminal-empty paths are explicit. |
| Canonical producers and atomic per-lease publication | PASS | GLB materialization, analytic placeholders, terrain output, and face overlay construct a write token, allocate page-by-page, write one typed item per step, seal, and transfer a lease through the observed mesh/version/interaction-slot transaction. Rejected publication returns the original `Mesh3dLease`; replacement goes through one retained close owner. |
| Face overlay | PASS by source inspection | The scan advances draw → instance → triangle and compares preview, hovered, then selected state. Fixed `[u32; 3]` counts/order preserve first-seen category order; geometry writes positions/normals one typed item at a time; indices preserve `[0,1,2,0,2,1]`; stale generation/revision, saturation retry, abort, and close retain exact owner/key state. The face-route negative scan has 0 matches for `Mesh3d::from_buffers`, `FaceOverlayBucket`, `HashSet<String>`, and `Vec<f32>`. |
| Terrain output | PASS within the bounded single-band scope | `WorldTerrainMeshCursor` retains tile identity/revisions, the ten band phases, one token/lease, and source ownership. Count, allocation, position/normal/index write, seal, publication, next band, and source retirement are each persistent phases; stale style/visibility, malformed values/indices, saturation, interruption, and close enter the same cursor close path with terminal witness. The three source `Vec` payloads remain the explicitly unaccepted JSON-to-flat terrain input boundary. |
| GPU upload / close order | PASS by source inspection | `MeshGpuUploadCursor` writes one vertex or one index per `ensure_mesh_step`; no active bulk mesh loop was found. `OsHostRetirement::close_step` drains the active mesh upload and verifies its terminal witness before `runtime.close_world3d_dynamic_step()`. The renderer's in-source negative assertions cover removal of the former all-upload/full-schema loop shapes. |
| Source fixtures and negative assertions | PRESENT, unexecuted | Fixtures inspect placeholder differential geometry plus interrupted close, terrain band differential/malformed/interrupted paths, face winding/offset/stale close, capacity and ABA. `include_str!` source assertions reject direct dynamic-owner and face-route fallback patterns. Cargo execution was prohibited, so these are not runtime/compile results. |
| Exact legacy census | PASS / residual recorded | World has 12 `Mesh3d::from_buffers` occurrences: two are individually `#[cfg(test)]` legacy helpers and ten are in the test module. The obsolete Vec-backed public `Mesh3d` declaration, constructor impl, and slice conversion remain in `ui-scene/math.rs`; its test fixture remains too. |
| Scoped formatting | **FAIL** | Individual checks passed for mounted root, canonical World, ui-scene math, GPU draw, GPU context, and renderer glue (edition 2024). `os_host.rs` fails edition-2021 rustfmt at its kernel-seam import ordering. |
| Diff whitespace | PASS | Scoped and whole working/staged/HEAD `git diff --check` were clean. |

## Commands Run

```text
rustfmt --edition 2021 --check --config skip_children=true <root, World, math, draw, gpu, os_host>
# five first paths PASS; os_host FAIL

rustfmt --edition 2024 --check --config skip_children=true <renderer glue>
# PASS

rg / source-slice structural scans
# root census 0; face-route forbidden census 0; 16 KiB/16 MiB/256/4,096 authority verified

git diff --check; git diff --cached --check; git diff HEAD --check
# all clean (whole and scoped)
```

## Exact Repair

Format only `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️os_host.rs` so its kernel-seam import is canonical, then rerun the individual scoped rustfmt checks and the source/diff census above. No behavioral repair is indicated by this audit.

## Non-Accepted Residuals

Even after that formatting repair, this is not Phase 3 acceptance:

- Ten terrain bands are published atomically **per lease**, but the source publishes them sequentially; all-ten-band visibility is not one aggregate atomic transaction.
- Placeholder progress depends on render invalidation rather than a proven wake/scheduling signal.
- The public legacy Vec-backed `Mesh3d` type and cfg(test) differential oracles remain; deletion/type validation is open.
- GPU cache eviction uses dynamic `HashMap` traversal; raster/atlas uploads remain contiguous; presenter render/submit is still an opaque step; pending packet/GPU-table/full realm close are not fully cursor-witnessed.
- Cargo compilation, Rust fixture execution, browser/Wasm/render/submit behavior, cache/raster/GPU timing, and terminal realm close were not run by instruction.

Accordingly Phase 3 remains **RED** regardless of the narrow format repair.
