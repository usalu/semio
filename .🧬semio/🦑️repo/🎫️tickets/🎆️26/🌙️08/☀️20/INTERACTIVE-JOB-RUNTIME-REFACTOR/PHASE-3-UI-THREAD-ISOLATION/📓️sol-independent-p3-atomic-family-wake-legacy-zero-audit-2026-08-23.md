# Sol Independent Audit: Phase 3 Atomic Mesh-Family, Wake, and Legacy-Zero Packet — 2026-08-23

## Audit Admission

The coordinator requested an independent Terra/high child audit. The scheduler rejected child admission because the shared agent-thread limit was exhausted, so this is the explicitly authorized independent Sol High fallback. This audit changed no production source and did not run Cargo, Nx, Wasm, a browser, network work, or root lint.

## Verdict

**REJECT — source packet.** The old `Mesh3d` type/constructor census, paged-oracle containment, face complete-generation visibility, terrain complete-family visibility, and realm scalar close are source-coherent. The live wake, however, is not a retained generation-keyed authority: it is created afresh inside every frame, reduced to a boolean before leaving World rendering, and therefore cannot preserve or validate the claimed ABA-distinguishable generation through native or browser scheduling. The current fixtures and source guards do not exercise or reject that live defect.

Phase 3 remains **RED** independently of this packet verdict.

## Independent Findings

| Requirement | Result | Evidence |
| --- | --- | --- |
| Exact legacy `Mesh3d` census | PASS | `rg -n '\bMesh3d\b|Mesh3d::from_buffers|-> Mesh3d\b' 🧰️framework --glob '*.rs'` returned no matches (exit 1). The public Vec-backed type, constructor, conversion, reexport, and test spelling are absent. |
| `LegacyMeshOracleData` containment | PASS | All 17 occurrences are in `ui-scene/math.rs`'s `#[cfg(test)] mod tests` or the canonical World's individually `#[cfg(test)]` oracle/helper plus its test module. Both oracle paths publish through `mesh3d_begin` / page allocation / typed writers / `mesh3d_seal` before a shared `Mesh3dLease` is observed. No production oracle type is compiled. |
| Face family visibility and last-valid preservation | PASS by source inspection | Category leases are published under `component-face-overlay:{surface}:{generation}:{index}` staging keys. `face_overlay_generation` and the fixed color table change only on `WorldFaceOverlayMeshStep::Complete`. A partial or stale superseding cursor leaves the prior visible generation unchanged. Publication rejection restores the exact returned lease and retry key into the cursor; cursor fault/close retains token or lease until its page-stepped terminal witness. Old and partial generation-qualified registry leases remain retained until later retirement and are recorded below as an accepted RED residual, not as lost ownership. |
| Terrain ten-band visibility | PASS by source inspection | `TERRAIN_COLOR_BANDS` is 10. `WorldTerrainMeshCursor` advances every band and reaches `Complete(tile)` only after `NextBand` reaches ten and the retained source is retired. Only then does `step_world_terrain_mesh` insert the tile into `terrain_built_tiles`; `sync_terrain_state` skips every draw while `terrain_family_visible` is false. Partial band publication is therefore hidden. Normal style replacement still removes old visibility before the replacement completes. |
| Fixed generation/armed wake | **FAIL** | `WorldCursorWake` itself has fixed `generation` and `armed` scalars, but its only production owner is `World3dBuildContext`. `AppRuntime::frame_before_input` constructs `World3dBuildContext::default()` anew every frame. `take_cursor_wake` discards `WorldCursorWakeToken` and returns `bool`; every subsequent frame/preparation/presentation field is also `bool`. Consequently a resumed production frame restarts at generation 0 and emits token 1 again. There is no retained generation to compare, acknowledge, or reject after the frame boundary. |
| Native/browser one-shot handoff | PARTIAL | The boolean survives `AppFrameAfterChrome -> AppFrameBuild -> AppFramePreparation -> AppFramePresentation -> AppPresentStep`. Native completion sets `RESOURCE_READY`, whose scheduler dirty bit coalesces. Browser output consumes `host.cursor_wake_requested` with `mem::take`. Those boolean handoffs are one-shot, but they do not repair the missing retained generation/ABA contract. |
| Realm scalar retirement | PASS | `step_world3d_dynamic_retirement` closes an active face cursor first, clears at most one of the three color options per grant, then clears the visible and retired generation options on separate grants. `world3d_dynamic_retirement_terminal_is_empty` includes every cursor, color, and generation witness before the mesh registry terminal witness can succeed. |
| Fixtures and adversarial guards | **FAIL for the wake claim** | `cursor_wake_coalesces_duplicates_and_rearms_after_exact_take` proves two requests/takes on one local `WorldCursorWake`; it never recreates the live per-frame `World3dBuildContext`, carries a generation through the frame structs, or checks stale-generation rejection. `face_overlay_family_becomes_visible_only_after_every_bucket_is_published` proves a staged/complete boundary and stale last-valid preservation, but constructs only one nonempty category rather than a mixed three-category family. No permanent verifier mutation was found that rejects per-frame wake-authority recreation, `WorldCursorWakeToken -> bool` erasure, missing generation propagation, or a single-category-only atomic fixture. |
| Scoped formatting/parser | PASS | Edition-2024 `rustfmt --check --config skip_children=true` passed separately for ui-scene math, canonical World, `winit_app.rs`, `frame_job.rs`, `os_host.rs`, and `browser_worker.rs`. Renderer glue parsed successfully through edition-2024 `rustfmt --emit stdout`. |
| Diff checks | PASS | Scoped P3 and whole-tree working, staged, and `HEAD` `git diff --check` passed. The actual P3 cohort delta in `HEAD^..HEAD` also passed `git diff --check`. Current P3 production paths have no working or staged delta; this report is the audit's only Phase 3 write. |

## Exact Blocking Repair

The wake authority must have a durable owner that survives frame construction. Its generation token must remain typed through frame build, preparation, presentation, and the native/browser scheduling edge. Taking a wake must atomically disarm one exact generation; stale acknowledgements must not consume a newly rearmed generation; rearming after the exact take must advance the retained generation. A boolean may be derived only at the final platform edge after the generation-qualified take/ack has completed.

The source evidence must then include:

- a live two-frame fixture proving the second wake has a distinct generation rather than another fresh token 1;
- duplicate-request and wake-storm coalescing before one take;
- stale token/ABA refusal after rearm;
- exact one-shot native `RESOURCE_READY` and browser `request_frame` consumption from the same retained token;
- a mutation that recreates the authority per frame or erases the token to `bool` and fails for the intended wake rule;
- a genuinely mixed-category face fixture that stages multiple buckets, proves no partial generation is visible, and proves stale interruption returns/retains every staged owner while the previous family remains visible.

## Commands Run

```text
rg exact legacy/type/oracle/wake/face/terrain/realm source censuses
rustfmt --edition 2024 --check --config skip_children=true <six scoped Rust sources>
rustfmt --edition 2024 --emit stdout --config skip_children=true <renderer glue>
git diff --check <scoped and whole working tree>
git diff --cached --check <scoped and whole index>
git diff HEAD --check <scoped and whole HEAD delta>
git diff HEAD^ HEAD --check <scoped P3 cohort>
```

No build or runtime result is claimed. Rust fixtures were inspected but not executed.

## Remaining Phase 3 RED Residuals

- Presenter-witnessed normal retirement of old face generations and last-valid terrain style replacement.
- Retained release of old/partial generation-qualified registry leases outside realm close.
- Typed, retained terrain input; current JSON/serde flat payload materialization remains indivisible.
- Dynamic collection and frame-owner retirement, including pending packets.
- GPU table, upload, atlas, raster, cache, and replacement retirement beyond the already retained mesh-upload slice.
- Opaque render/submit/present timing and complete presenter/GPU close.
- Full realm terminal-empty proof across every renderer/runtime owner.
- Real native timing plus Wasm/browser Worker scheduling and close evidence.

Accordingly this report does not accept Phase 3, Phase 5, or the broader runtime matrix.
