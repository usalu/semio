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

- Scene `an_unmeshable_solid_is_marked_and_the_others_still_draw` (`🎬️scene/🧪️tests/🔬️unit/🦀️.rs`): a solid with a degenerate outline next to `sol1`: `sol1` keeps its mesh and label, the failing one gets an error-tinted outline prism mesh under its own id and a label in the requested locale only: `Solid <id> cannot be meshed: <reason>` for En, `Körper <id> lässt sich nicht vernetzen: <reason>` for De; the law asserts the other language and any ` | ` joiner are absent.

## Scene gap (closed)
`mesh_solids` was all-or-nothing and `fem3d_solid_mesh_entries` swallowed its error, so one bad solid hid all solids. Now `meshing::mesh_solids_partial` and `mesh_preview::fem3d_mesh_preview_partial` return every meshable solid plus `(solid id, reason)` per failure (`mesh_solids` keeps its strict contract on top of it). The scene draws each failure as a red prism marker (pickable under its own id) labelled with `fem3d_unmeshable_solid_label(id, reason, locale)`: one language per view. The active `semio_plugin::Locale` is threaded into `fem3d_scene_parts`, `fem3d_solid_mesh_entries` and `model_scene` (the same route the model/results captions and `surface_name` use); the model window, results window and viewer pass the locale they already hold.

## Status (all native, all green)
- 3D engine turned-slab law: FAIL without fix (12 of 361), PASS with fix.
- 2D engine `a_turned_region_always_meshes`: PASS.
- Interaction (`ticks_compose_into_one_net_leaf`, `a_sixty_pixel_ring_drag...`, `degenerate_scale_ticks...`): 3 passed.
- Editor `a_ring_turn_of_a_solid_keeps_it_and_its_analysis_alive`: PASS (18 s).
- Scene unmeshable-solid law (EN-only and DE-only assertions): PASS. Editor `renders_fem3d_model_scene` (new `locale` argument): PASS.
- Not recompiled after the locale change: the identity test call sites in `🕹️interaction/🧪️tests/👥️identity/🦀️.rs` (only a `Locale::En` argument was appended), because other agents' `semio-plugin-instance-stores` build was broken and the disk filled up during my last runs.
- Fixed along the way: my law had a type-inference compile error (`step.abs()` on an untyped float).
- `C:\t\g`, `gb`, `gbt`, `gc`, `gcb`, `gct`, `gd`, `gdb` deleted; generated logs deleted. Scripts `gb-law.sh`, `gb-engine.sh`, `gb-engine2d.sh` kept.
