# Gumball rotate F2 (slab vanishes, `Analysis failed`)

## Root cause
Turning `sol1` (`10..12 x 0..2`, `meshSize 1`) about its centroid by roughly 44-46 degrees (and the same angle +90/180/270, 12 of 361 whole degrees) makes `semio_s_fem_mesh::triangulate` fail with `could not recover boundary edge`. The interior refinement lattice is axis-aligned; on a diagonal boundary it strands a point an ulp-sliver from a boundary edge. A 60 px drag of the ring is about 0.78 rad = 45 degrees, an 18 px drag is about 0.25 rad and meshes fine.
`mesh_solids` is all-or-nothing, so one failing solid:
- Model view: `fem3d_solid_mesh_entries` swallows the `Err` (`let Ok(..) else { return Ok((meshes, instances)) }`) and draws no solid at all (the slab vanishes).
- Results: `build_model`/`resolve_geometry` returns `MeshFailed`, so `Analysis failed`.
Ctrl+Z restores the unrotated outline, which meshes.

Verified by baseline run of the new law with the fix disabled: 12 of 361 turns fail (`44,45,46,134,135,136,224,225,...` degrees).

## Secondary defects found in the rotate/scale path (also fixed)
1. `fem3d_gumball_then` composed two rotation ticks only if their axes matched within `1e-9` per component. The host derives the axis from a quaternion delta every tick, so noise (and the sign flip when the pointer reverses) made `continue_stream` drop the tick silently (`unwrap_or(stream)`): the drag lost motion. Now rotations compose as a signed angle for (anti-)parallel axes within `1e-6` and as a quaternion product otherwise; no tick is ever dropped.
2. `MoveSelection::map_solid` required the footprint plane to be kept within an absolute `1e-9`; an axis tilted by sensor noise (`1e-7`) made the solid "unable to follow" and it silently did not move. Tolerance is now `1e-6` relative to the solid's reach. A really tipping axis (X/Y ring on a Z slab) still refuses cleanly (`mutation.partial`, solid unchanged) and the model keeps building and solving.
3. Scale: ticks with zero, negative or non-finite factors are filtered by `fem3d_gumball_moves` and the leaf breach (`positive scale factors`), so the scale handle cannot destroy a solid; covered by a law.

## Fix
- `🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🔷️triangulation/🦀️.rs`: `triangulate` retries `owned_triangulate` with lattice spacings x1.0, 1.07, 0.93, 1.19, 0.81, 1.41 and finally boundary-only (`LATTICE_RETRIES`). Shared by 3D and 2D (regions use the same call), so 2D is fixed by the same change.
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/.../✏️editor/🕹️interaction/🧭️gumball/🦀️.rs`: `fem3d_turn_then` (rotation composition).
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/.../🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🦀️.rs`: relative planarity tolerance in `map_solid`.

## Laws
- 3D engine `a_turned_slab_always_meshes_with_its_area_and_volume` (`⚙️engine/🧊️3d/🧪️tests/🔬️unit/🦀️.rs`): 361 turns, tet volume equals area x height.
- 2D engine `a_turned_region_always_meshes` (`⚙️engine/◻️2d/🧪️tests/🔬️unit/🦀️.rs`).
- Interaction `a_sixty_pixel_ring_drag_turns_the_slab_into_a_valid_solid`, `degenerate_scale_ticks_never_reach_the_slab`, extended `ticks_compose_into_one_net_leaf` (`✏️editor/🕹️interaction/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs`).
- Editor `a_ring_turn_of_a_solid_keeps_it_and_its_analysis_alive` (replaces the `p3_probe` print probe in `✏️editor/🎮️commands/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs`): Z/noisy/negative axes follow, X/Y refuse, model builds and solves in all cases.

## Status
- 3D engine `a_turned_slab_always_meshes_with_its_area_and_volume`: FAIL without fix (12 of 361, `🗑️generated/gb/eng3.log`), PASS with fix (`eng5.log`).
- 2D engine `a_turned_region_always_meshes`, interaction laws and editor law `a_ring_turn_of_a_solid_keeps_it_and_its_analysis_alive`: written, NOT yet run. Native runs were blocked by the shared build-lane file lock and by concurrent breakage in other agents' framework crates (`semio-os-kernel`, `semio-plugin-instance-commands`, dwg crates). Detached chains `/c/t/gb-run.sh` and `/c/t/gb-run2.sh` (logs `/c/t/gb-run.log`, `/c/t/gb-run2.log`, per-run logs in `🗑️generated/gb/`) are still queued; scripts `gb-law.sh`, `gb-engine.sh`, `gb-engine2d.sh` in this folder. The new interaction/editor code is therefore uncompiled; the law `ticks_compose_into_one_net_leaf` was updated to the new composition semantics.
- Dirs `C:\t\g*` are still in use by the queued runs and must be deleted afterwards.

## Remaining hardening (not done, out of scope)
`fem3d_solid_mesh_entries` still hides ALL solids when any one fails to mesh; a per-solid fallback or a visible error would make such failures diagnosable.
