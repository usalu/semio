# Dev Process React End-to-End (2026-09-15/16)

## Objective

Restore **🛠️dev🪚️process🏙️3d⚛️react** (`SEMIO_RENDERER=react`, port `6022`): process wasm component + four extension
components → `activate-process3d-react-dev` → Vite serve → the process3d editor boots in the browser and renders
every requested surface.

Port `6022` was held by a live peer session (ticket 26/09/09/PROCEDURAL-3D-END-TO-END serves generation3d there on
purpose), so this session's browser gate ran on `S_OS_PORT=6222` via the new `process3d-react-lane-attach` launch
entry; the canonical `process3d-react` / `process3d-react-attach` entries (6022) were added to `.claude/launch.json` too.

## Fault 1 — wasm component would not compile

`@semio-tech/process-plugin:component-dev`:

```
error[E0433]: cannot find module or crate `semio_framework_tool_run` in this scope
  --> ✏️s/🔌️plugins/🏭️process/🦀️.rs:9:1   (inside dyn_enum_close! { pub enum ProcessApps: PluginApp { … } })
```

`PluginApp::tool_run_trace_delta` (`🔌️plugin/🦀️.rs`) spelled its cursor parameter as
`semio_framework_tool_run::ToolRunTraceCursor`. `#[dyn_enum]` re-emits that signature verbatim inside every
consumer's `dyn_enum_close!` expansion, so the path had to resolve in the consumer crate — and only 10 of 33 plugins
depend on `semio-framework-tool-run` (cad was patched earlier today by adding the dependency; 23 plugins were still red).

**Fix (framework, not per plugin):** the trait now names bare `ToolRunTraceCursor`, imported into `mod app` from
`semio_framework` (which re-exports it via `manifest::*`) — the same way `Fault`, `ToolOwnerWitness`, … already reach
every consumer through `plugin_app_close_prelude::*` (`pub use semio_framework::*`). All 37 external
`__semio_dispatch_PluginApp` consumers import that prelude, so no plugin needs the tool-run crate for this.

Also: `Process3dDiff.artifact` dropped an `unused_qualifications` warning (`crate::schema::Process3dArtifact` → the
already-imported `Process3dArtifact`).

## Fault 2 — the workshop panel never publishes

Browser (React shell, port 6222):

```
[DEBUG] PluginRuntime: actor process#1 stopped without publishing requested UI surfaces (missing=["1:workshop"], status=idle)
```

The guest's reason never reached the console: the reactor reports a failed dirty-surface render as a
`shell_fault_effect` on the same turn, but `settlePluginTurn` threw on the missing surface before
`retainedUiRefreshEffects` ever decoded that turn's effects. **Fix (host diagnostics):** `settleShellFaultMessages`
in `🔌️PluginRuntime/🟦️.tsx` decodes the settle's `Error` shell frames and appends them as `faults=[…]` to both
settle errors. With it:

```
ui.surface-render: 1:workshop [reconciler g4]: Credits { usage: SurfaceReconcileUsage { nodes: 35, items: 1088, bytes: 8390387 },
  limits: SurfaceReconcileLimits { max_nodes: 128, max_items: 4097, max_bytes: 8388608, max_identifier_bytes: 256 } }
```

35 nodes priced at 8.4 MB: ~240 KiB per node. Measured with a scratch crate against the ui contract:

| type | `size_of` |
|---|---|
| `TreeNode` | 6456 |
| `ActionBinding` | 2072 |
| `RowAction` | 3104 |
| `UiValue` / `UiText` | 520 / 514 |

`UiFixedList::try_push` was "cold": it called `try_reserve()` = `PagedList::reserve_full()`, so ONE binding reserved
all 32 slots — `len=1 capacity=32 allocated=79744` bytes — and the reconcile census prices `capacity()` × `size_of::<T>()`
× 3 semantic copies: ~240 KiB per binding, ~300 KiB per row action. A workshop row (label + activate binding + a
"remove" row action) cost ~0.5 MiB; the demo document's 12 installed machines plus the 4 catalogs' installable rows
blow the 8 MiB surface budget at node 35. puzzle3d hit the same wall and documented it as
`PANEL_RECONCILE_NODE_BUDGET = 16` ("~0.46 MiB per presented node") and virtualised its outliner around it.

**Fix (framework root cause):** `UiFixedList::try_push` (and `Clone`, and `UiFixedMap::try_insert`) now reserve
page-exactly — `reserve_next_slot` loops `next_allocation_bytes` → `try_reserve_one`, the idiom `UiPatchOps::try_push`
already used. For a 2 KiB `T` a page is one slot, so a cold push owns exactly what it holds. The ownership fixture case
`reserved-binding` ("a second binding into reserved backing adds 0 bytes") now reserves that backing explicitly
(`with_reserved_backing`) instead of relying on the cold push having done it.

## Fault 3 — the app aborts at construction: child handles violate the composition invariant

After the rebuild (which also picked up a peer's in-flight framework work — `VcsArtifactApp::new` now runs
`seed_genesis_children()` on the initial snapshot), the actor trapped before its first turn:

```
ArtifactApp::genesis_child_pack members must open cleanly onto a freshly constructed store:
  plugin.internal "initial snapshot child projection failed: child restore projection: InvalidReference"
```

`ChildRestoreProjection::child` (`🏪️store/🦀️.rs`) and the owned-document closure both require
`handle.child_id == handle.target.artifact_id` — the convention cad (`cad_model_child_handle`, and
`cad_child_from_uri` rejects anything else), sourcing (`catalog_child_handle`) and lowpoly follow. process3d's
`brep_child_handle`/`flow_child_handle` minted `child_id = "<slug>-brep-<hash>"` but
`target.artifact_id = "process-<slug>-brep"` / `"process-steps-flow"`, so every process3d document — the timber
default, the plate example, all 40 mutation fixtures — was structurally unloadable; the earlier boot only got as
far as it did because nothing projected the children until the peer's genesis seeding.

**Fix:** both helpers mint `target.artifact_id = child_id`. All assets that embed handles were repaired from the
record itself (the target id is now the handle's own child id, everything else untouched):
`🖼️assets/🎬️demo/🗣️.dsl.semio` (the timber default), `PROCESS_3D_PLATE_EXAMPLE_TEXT`, and the 41
`🧫️fixtures/🧬️mutations/**` before/after/diff JSONs — script kept as
[🐍️repair-process3d-child-target-ids.py](./🐍️repair-process3d-child-target-ids.py).

## Peer churn seen while this ran

- `Cargo.lock` was stale for a peer's new `semio-framework-ui-scene` dependency (`--locked` refused every wasm
  build); `cargo metadata --offline` refreshed it.
- Port `6222` Vite restarted itself on a peer's config edit mid-gate; reload and continue.

## Fault 4 — the workpiece rendered the unit-box fallback, never the process

With the app booting, the workpiece window's `data-meshes-json` carried the ±0.5 unit box
(`PROCESS3D_FALLBACK_MESH_KIND`): `processed_mesh` returned `None` for the default timber document. The old
`render_world_scene_contains_processed_mesh` law only checked the mesh *id*, which the fallback satisfies too, so
this had never been caught. Reproduced natively (new law
`render_world_scene_replays_the_timber_beam_instead_of_the_fallback_box`); stepping the replay showed the stock
alone fine (0.18 m³) and the first cut failing with `exact boolean result failed validation: … non-manifold-edge`.
Three separate causes, each fixed at its root:

### 4a. process3d posed kernel primitives by their corner while every document was authored about centres

`Brep::box_prim` spans `[0,w]×[0,d]×[0,h]` from the origin and `cylinder_prim` runs `z ∈ [0,h]`; process3d's
`solid_for_spec` posed them as-is, and `process3d_step_from_face_drag` even documented that. But the documents
say otherwise: the timber stock rests on the ground at `z = 0.15` (half its 0.3 m height), the plate example
drills at `x = −0.45` of a 1.2 m plate — every pose is a CENTRE. Corner-anchored, the crosscut tool sat flush on
the stock's own faces and the "lap joint" was an internal void. `solid_for_spec` now shifts each primitive by
minus its half extents (`primitive_centre_offset`) before the pose, the face-drag builder centres its tool box on
the dragged region, and the `drill_reduces_volume_below_stock` law states its pocket in centre terms.

### 4b. the exact boolean could not clip a plane/plane line to a face pair

`intcurve_finite_bracket` windowed an unbounded (or huge-domain) intersection LINE to 4× the larger face's
diagonal around the pair's centroid; `clip_intcurve_to_faces` then sampled that window with 64 cells, so a
0.02 m crosscut of a 3 m beam was a fraction of one cell and every pair clipped to nothing — no imprint queued,
non-manifold stitch. The bracket is now the exact parameter run through the two faces' AABB intersection (a
per-axis slab clip, widened by a small margin), applied to finite line domains too.

### 4c. the boolean kernel did not know box-through-box

Four documented or latent gaps in `🗄️stdio`'s brep boolean, each pinned by a new kernel law
(`box_minus_box_through_slab_splits_into_two_boxes`, `…through_pocket_leaves_a_square_hole`,
`…blind_pocket_removes_the_pocket_volume`, `…flush_notch_removes_the_notch_volume`,
`box_minus_cylinder_blind_bore_removes_the_bore_volume`):

| gap | fix |
|---|---|
| a cycle of open segments (a tool side face receiving the stock's whole cross-section as four lines) was refused as "piecewise-closed imprint not supported" | `chain_open_pendings` assembles cycles; new euler operator `split_face_by_interior_chain` imprints them as a hole ring + a new face |
| a planar ring without p-curves has UV area 0, so the hole's winding could not be chosen | planar windings are measured in 3D about the plane normal (`points_signed_area_about`) |
| `loop_volume_moments`' planar fast path took the ring winding at face value, so an opposite-wound hole ADDED its area (`1.16` instead of `0.84`) | the ring is normalised CCW about the natural normal before the tetra sum, as the general `ear_clip` branch already did |
| a tool that merely TOUCHES the target (a notch flush with the top and both sides) was answered as an enclosed void by the trivial fast path — the cut added the tool's volume | `Cut` takes the void fast path only when the tool is STRICTLY inside (`solid_strictly_inside`) |
| a segment along BOTH operands' boundaries (flush faces) was queued as an imprint and failed "midpoint not found inside any active piece"; its clip endpoints snapped onto the coincident boundary instead of the crossing one, leaving 2 µm slivers | both boundaries are subdivided and unified into one edge (`unify_boundary_edges`); `snap_clip_endpoint` only snaps onto TRANSVERSAL boundary curves |
| pieces of A carved coincident with a piece of B (the notch opening) were both kept — edges with three coedges | piece-level coincidence is detected after imprinting and handled like whole-face coincidence (`coincident_b`) |
| `local_point_in_solid` tested ray hits against a periodic face piece with a single canonically-wrapped UV probe, missing the piece above a seam-crossing circle — a blind bore's floor disc classified `Outside` and survived | it uses the period-shift probe (`point_in_face_uv_periodic`) the clip stage already relies on |

The brep suite (`brep::`) is otherwise unchanged: the same 32 pre-existing failures (mutation fixtures, snapshot
round-trips) fail with and without these edits; `offset_sphere_matches_closed_form` times out on HEAD too.

## process3d unit suite

Started at 246/331 green (all red since 2026-09-08). Fixed here: the harness seeded contributions through
`handle_action("setContributions")`, which B1 reserves for framework verbs (→ typed `dispatch`); the roster
counts still said 33 rows after `setActiveUtility`/`setLocale` became framework-injected (→ 31); the app
declared no close-lane disposers for its draft/presence/transient stores (→ `Process3dPresenceRetirementFactory`,
`process3d_presence_store_disposer`, `no_transient_store_disposer`, `NoDraft` owners — every registry-backed
fixture aborted in `Drop`); and all 45 mutation fixtures were re-minted from the crate's own
`apply_mutation`/`Mutation::diff` (float formatting and content-addressed child hashes had drifted with the codec;
scripts [🐍️canonicalize-process3d-fixture-numbers.py](./🐍️canonicalize-process3d-fixture-numbers.py),
[🐍️remint-process3d-fixture-outcomes.py](./🐍️remint-process3d-fixture-outcomes.py)). Now 294/331.

Still red (pre-existing, outside this ticket's runtime path): 12 render laws whose harness serialises a
`ComponentTree` root with plain serde (`BuiltChildren requires retained page transport`); the `🌉️wasm` bridge
laws (`edit history insertion requires its exact mutation retirement factory` after a `LoadDocument` reset,
`typed-operation pending publication rejected`); `export_brep_out_returns_step_text_structured_payload`
(`unknown process export format kind \`step\``); a handful of dispatch laws asserting effects/mutations
(`world_pointer_down_*`, `arg_form_set_stock_*`, `undo_after_*`, `window_measures_surface_the_sun_group`,
`toggle_sun_*`). 

## Verification

```bash
bun nx run @semio-tech/process-plugin:component-dev            # 62 MB wasm-dev component, green
bun nx run-many -t component-dev -p @semio-tech/process-extension-{concrete,metal,robotic,wood}-rust
bun nx run @semio-tech/framework-os-dev:activate-process3d-react-dev   # 5 components activated
S_OS_PORT=6222 SEMIO_RENDERER=react bun nx run @semio-tech/framework-os-dev:serve-process3d-react-dev
```

Browser (React shell, `http://localhost:6222/`, launch entry `process3d-react-lane-attach`; the canonical
`process3d-react` / `process3d-react-attach` entries use 6022):

| check | result |
|---|---|
| plugin + 4 extensions load, actor `process#1` boots, every requested surface publishes | ✅ no `render failed` / `surface-render` / trap in the console |
| Artifact panel | Timber Beam stock, 4 steps with Enabled toggles |
| Workshop panel | 11 installed machines + Concrete / Metal / Robotic catalog sections (the extension wasms) |
| install "Chop Saw" from the Metal catalog (`addWorkshopMachine`) | row moves into MACHINES; op log shows `create-machine index=11 machine { id=chopSaw … catalog-id=metal }` |
| workpiece `data-meshes-json` | the replayed beam: x −1.5…1.5, y ±0.1, z 0…0.3, 94 vertices / 180 indices — crosscut slab, lap-joint notch, dowel |
| disable "Crosscut To Length" (`setStepEnabled`) | mesh re-replays without the slab (70 vertices / 144 indices); re-enable restores 94 / 180 |

Native laws added: `render_world_scene_replays_the_timber_beam_instead_of_the_fallback_box`,
`timber_document_replays_every_step_with_monotone_subtractive_volume`, five box/cylinder boolean laws in the brep kernel.

Peer-ops note: Vite on this lane wedges on every `📇️registry/🤖️generated` regeneration (a restart that never
comes back) — recycle the `vite` pid carrying `S_OS_PORT=6222` and re-run the serve target.
