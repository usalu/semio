# WP-07 3D viewport window (A7)

Status: code complete, NOT yet compiled or tested. `repo run @semio-s/plugin-norm-app-surface:check` was queued behind about 17 builds (memory pressure) for over 10 minutes at the time of writing; log in `TICKET/🗑️generated/wp07/check.txt`. Expect first-compile fixes (unverified APIs: `pack_json::json!` with `Vec<&Value>`, `Value::as_u64/as_bool`, derive attributes on `Default` enums).

## Day-one spike: custom mesh JSON (against `fem3d_solid_mesh_entries`)
- Mesh entry: `{ "id": str, "data": { "positions": [f64 xyz...], "normals": [f64 xyz...], "colors": [f64 rgb 0..1 per vertex], "indices": [u32...] } }`. Built-ins are references `{id, kind}` (`box` long axis Z, `cone` Y-up) and carry no vertex colours.
- Instance: `{ id, interactionGranularityId, meshId, position, rotation [x,y,z,w], scale, label }` plus host flags `selected, hovered, highlighted, disabled, provisional, interactionId`. There is NO per-instance tint, so colour lives in the mesh: mesh id = `<shape key>@<rrggbb>`; equal shape and colour bucket (1/32 of the domain) share one mesh.
- Correction to architecture §4.1: fem3d instance `id` is NOT the raw entity id but `encode_scoped_identity([granularity, id])` (`semio_framework_os_kernel::wire`); extra parts of one entity add `interactionId` = the entity address. Lowering follows fem3d; `picking::resolve` decodes it (also strips `#n`).
- Only one `World3dScalarField` fits a scene (single `mesh_id`): it is bound to the selected utilisation member, else the worst one. All other entities carry baked ramp vertex colours.

## Delivered (SURFACE = `.../📇️registry/🧬️contract/🖥️app-surface`)
`🪟️visual-windows/🧊️viewport3d/` (no `🪟️visual-windows/🦀️.rs`; modules mount flat like A6's `properties`):
- `🧭️pose` vector/quaternion algebra; `🧮️mesh` ear-clipping triangulation with hole bridging, prisms, cone, cylinder, sphere, grid, flat polygon; `🧱️parts` the `SceneTessellator` trait and `BasicTessellator` (members incl. arcs, offsets, rebar of composites, deformed/mode shapes with ghost; plates with stiffeners; revolved/patch shells; prism/box/cylinder/wedge/mesh/strata/footing/wall solids; bolt/screw/stud/weld/tendon/rebar fasteners with Grid/Row/Along/Cage patterns instanced from shared meshes; supports; all nine load glyphs; surfaces). A3's kernel can replace it behind the trait.
- `🎨️colour` role palette, viridis/inferno/coolwarm/grayscale ramps, status colours; `🎚️config` `Viewport3dWindowConfig` (+ proto/graphql/ts twins in `🎚️config/🧬️schema`), `DeviceClass::profile()` (desktop FULL/20000, tablet REDUCED/8000, mobile COARSE/3000); `📷️camera` iso/plan/4 elevations, fit selection, fit worst; `📏️measure`; `🎯️picking` (entity id + field path, check/loadCase cross-selection); `🏷️overlays` (dimensions in config unit, labels, leaders, clause badges, failure glyphs, fail/near-limit/stale markers, measure result; density capped by device); `♿️description` EN/DE scene text and live-region announcement; `🎞️lowering` `lower()`, section plane, highlight tokens, sub-part pick targets, `instances_delta`; `🖼️window` `world3d_scene_of`, `render_viewport3d`, action id constants.
- `🪟️manifest-and-media`: `visual_window_id`, `viewport_window_definition(variant, ...)` (World3d, interaction `norm.<variant>`, transform utility, 7 actions), `diagram_window_definition`, `visual_layout(variant, device)`.
- `🕹️direct-manipulation/🧭️domain`: `interaction_domain_id`, `viewport_interaction_definition(variant, label, kinds)`.
- Oracle `🔮️oracles/🐍️.py` (closed form, shapely when installed; shapely/numpy absent on this host) -> golden `🧪️tests/🧫️fixtures/🔣️prisms.json`; `🧪️tests/🥒️.feature`; 21 unit tests in `🧪️tests/🔬️unit`.

## Edits outside my tree
- `SURFACE/🦀️.rs`: added the two mount blocks (`pub mod viewport3d`, `mod interaction_domain`) to unblock compilation (A6: keep them). `SURFACE/📦️packages/🦀️rust/Cargo.toml`: path dependency on the scene crate. Mount lines also recorded in `📌️important/mounts.md`.

## Assumptions to review
- Solid `Box`/`Prism`/`Cylinder` frame origin = base centre; wall layers stack laterally centred on the path; pressure zone arrows point against the ring-normal (right-hand rule) times sign of the value; `Cage` pattern draws `count` discs spaced `stirrup_spacing` along the bar; shell thickness, section taper and spline smoothing are not rendered yet.
- `FieldBinding` is A2's stub today; lowering ignores it. Hit areas, gumball arming and handles are WP-10.
