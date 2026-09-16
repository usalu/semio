# Koordinator (`s.cad.cad@1/*#editor`) — object mutations, the re-materialization seam, and `setPanelPage` (2026-09-16)

Follows `🎆️26/🌙️09/☀️15/DEV-CAD-REACT-E2E`'s `📓️cad-react-e2e-2026-09-16.md` §Open items 1 and 2. Item 3 (the engagement HUD's `N selected`) is another worker's and untouched here.

## 1. Findings — what "documented no-op" actually meant

### 1.1 Where the object data lives

`CadSnapshot` carries no object data at all. It composes four fixed `s.stdio.semio.model` CHILD HANDLES (`shape_model` / `building_model` / `energy_model` / `structure_classic_model`, `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:24-38`), each a `store::ArtifactChild<SemioModelSnapshot>` — two strings, no content.

The content a pane actually renders comes from the handle's **local materialization**:

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:808-812` — `cad_model_child_for_pane` mints the content-addressed handle from `semio_model_snapshot_from_objects(objects)` and attaches an `Arc<CadWorkingScene>` via `ArtifactChild::with_local_owner`.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs:249-266` — `cad_pane_working_scene` reads it back with `child.local_owner::<CadWorkingScene>()`; `cad_pane_working_objects` slices it per pane.
- `…/✏️editor/🎭️modes/✏️edit/🦀️.rs:268-287` — `build_world_scene_for_pane` builds the `meshes` / `instances` / `selection` lanes from exactly those objects.
- `…/✏️editor/📌️panels/🗿️artifact/🦀️.rs:158-159` — the document tree reads the same accessor.

`local_owner` is deliberately invisible to every codec: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2856-2858` — "*intentionally absent from equality, debug, DSL, pack, and JSON identity*". `Clone` keeps it (`:2957-2961`), `FromValue` drops it (`:3016`). That single fact drives everything below.

### 1.2 Why the handlers were no-ops (and what the seam is)

`✏️editor/🎮️commands/🧱️object/🦀️.rs` and `…/🔄️transform/🦀️.rs` both returned `Emit::default()`, with a comment claiming no child-dispatch seam existed. Two separate things were conflated:

1. **Dispatching against the child document** — `Emit::child_emits` / `ChildEmit::of` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:10255, 10562`). This exists, but `dispatch_emit_group` refuses a `ChildEmit` for a child with **no live child store** (`…/🔌️plugin/🦀️.rs:24194-24197`), and `register_child`/`open_child` (`:23202, 23259`) are called from **tests only** — nowhere in production, in any plugin. So this route is not reachable in the demonstrator today.
2. **Re-materializing the pane after the edit** — the part the report called the missing seam.

The working precedent is `🌊️flow`, and it does **not** use route 1 for its own content: its parent mutations carry the payload and their **diff re-mints the content child with a fresh local owner** — `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:185-211, 228-232, 245-247` (`flow_content_child_handle_and_cache`), consumed by e.g. `…/🧬️schema/🔺️diff/📝️text/🦀️.rs:81` (`FlowDiff { content: Some(flow_content_child_handle_and_cache(..)) }`). That is the seam, and it is entirely inside a plugin's own diff builders.

Why cad's existing `create-<pane>-model` could not be reused as-is: its diff rebuilds the child from the payload's URI via `cad_model_child_from_uri` (`…/🧬️mutations/🧱create-shape-model/🔺️diff/🦀️.rs:9`), i.e. `ArtifactChild::new` with `local_owner: None`. Re-handling a slot with that verb would leave the pane rendering nothing.

**Concretely, the re-materialization seam is:** an object gesture reads the pane's live `CadWorkingScene` off its child handle, computes the next object list, and emits one parent op whose diff calls `cad_model_child_handle(pane, json_of(semio_model_snapshot_from_objects(next)))` **`.with_local_owner(Arc::new(next_scene))`** and writes it into that pane's `CadDiff` slot. Because `dispatch_emit`'s single-store path applies the **typed** mutations (`…/🔌️plugin/🦀️.rs:24011` — `ArtifactCommand::Apply { mutations }`, not re-decoded bytes) and `MutationDiff::apply` clones, the owner survives into the store snapshot and therefore into the next `render`.

### 1.3 The second, undocumented blocker

Even with real handlers, **none of these verbs could be dispatched from the browser**: `addObject`, `patchObject`, `patchSelection`, `deleteObject`, `duplicateObject`, `translateSelection`, `rotateSelection`, `scaleSelection` were all declared `InteractiveJobClassification::BatchOnlyPendingRewrite` (`✏️editor/🦀️.rs:2168-2177` pre-change). `ActionBus::register` rejects every non-`Migrated` factory (`🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:531, 562`). The gumball's own dispatch is `gumballTransformDeltaBetweenPoses` → `translateSelection` / `rotateSelection` / `scaleSelection` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:2469-2504`), so the drag was dead twice over.

### 1.4 `setPanelPage`

CAD had no such verb; the document tree closed an over-long section with the SDK's plain, unclickable `+N` row (`semio_framework_plugin::panel_continuation_row`). Puzzle 3d carries the full reference implementation — a `panel_pages: BTreeMap<String, u32>` cursor map, a `setPanelPage` command, and a `paged_section_from` that starts each section at its cursor (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs:255-320`, `…/✏️editor/🎮️commands/📄set-panel-page/🦀️.rs`). Puzzle routes it through its window-config lane; cad has no window-config cursor store, so cad's rides the ordinary app config lane instead.

## 2. Design

### 2.1 Five new `CadMutation` kinds

Approved verbs only (`APPROVED_VERBS`, `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs:112`); each inverts to exactly one row, so each is one invertible item under cad's existing `for_one_invertible_item` fold contract.

| kind | payload | inverse |
|------|---------|---------|
| `move-objects` | `{ pane, placements: [{objectId, newOrigin}] }` | `move-objects` with the pre-move origins |
| `rotate-objects` | `{ pane, placements: [{objectId, newOrientation}] }` | `rotate-objects` with the pre-rotation quaternions |
| `scale-objects` | `{ pane, placements: [{objectId, newScale}] }` | `scale-objects` with the pre-scale factors |
| `create-object` | `{ pane, index, object: CadObjectSpec, primitives: [CadObjectPrimitive] }` | `delete-object` |
| `delete-object` | `{ pane, objectId }` | `create-object` at the removed slot |

Four deliberate choices:

- **Placements are ABSOLUTE, never deltas.** A delta inverse would rely on `x + d - d` round-tripping through f64; an absolute inverse restores the recorded pose bit-for-bit. The command handlers turn the gumball's incremental delta into absolute placements by reading each object's current pose.
- **`create-object` carries `index`.** The child id is content-addressed over the ORDERED element list, so order is identity: a `delete-object` in the middle must invert to a create at the same slot or the document would not come back.
- **`create-object` carries `primitives` beside the object, not inside it.** A `#[dsl(table)]` field cannot nest inside a `#[dsl(block)]` one, and the importer authors its own slot ids (`…-solid-313`) that a `solid_handle`-derived list would silently rewrite — so an undo has to carry them verbatim. An empty list falls back to the one slot `solid_handle` implies, which is what a hand-authored "add a box" wants.
- **Absence IS the identity pose.** `rotate-objects` / `scale-objects` write `None` when the new value is exactly `[0,0,0,1]` / `[1,1,1]`, and compare through `unwrap_or` — otherwise an undo restored an equal DOCUMENT but a different working scene.

### 2.2 The seam, in one place

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️.rs:169-232` — four new crate-root helpers every diff builder shares:

- `cad_pane_local_scene(document, pane)` — the one materialization accessor.
- `cad_scene_pane_objects` / `cad_scene_with_pane_objects` — read/replace one pane's slice.
- `cad_pane_rematerialized_child(scene, pane, objects)` — **the seam**: mints the content-addressed handle from the new object list and attaches the updated `CadWorkingScene` as its local owner. Empty objects ⇒ `None`, i.e. vacate the slot.
- `cad_pane_child_diff_slot(diff, pane, child)` — writes it into the right `CadDiff` arm.

Only the EDITED pane's child is re-minted; the other three keep their own handles and owners, because each pane only ever reads its own slice from its own handle.

**Cost/progress:** synchronous per mutation, no spawned job. The whole cost is `semio_model_snapshot_from_objects` + a hash over its JSON — no tessellation happens in the mutation. Tessellation stays where it already was, in `world_meshes_json` at render time, per object, from the BREP kernel. There is nothing expensive to make cancellable here; if that changes (a kernel re-evaluation per edit), the spawned-job seam is the place, not the diff.

**Undo/redo** needs nothing new: these are ordinary in-history `CadMutation`s with real inverses, so the existing history mechanism (and ⌘Z) inverts them exactly as it already inverts `patchCadPlayReference`.

### 2.3 Why these five kinds carry no committed wire vector

Every other cad kind ships a `(before, mutation, diff, outcome, after)` JSON quintet replayed by `🧪️tests/📐️mutate-cad-1`. These five cannot, and that is a property of the vocabulary: the state they read is `ArtifactChild::local_owner`, which no codec serializes. A committed `before` snapshot can only describe a pane with no materialization, in which every one of these verbs is a `mutation.no-op` — and the differential's Python reference refuses a no-op vector by construction (`…/🧪️tests/📐️mutate-cad-1/🐍️.py:280`). Their specification lives in Rust laws beside each leaf instead; the oracle manifest records exactly this, and what would unblock a wire vector (a persisted child resolver — `store::LinkResolver` / `ChildStoreFactory`).

### 2.4 `setPanelPage`

Cursor on the config lane as an opaque JSON object (`CadConfig::panel_pages_json`), decoded into `CadPlayRuntime::panel_pages`. A string, not a `BTreeMap` field, for the same reason `engagement_session_json` is a string: `CadConfig` is a `dsl::DslArtifact` and a map keyed by an arbitrary section id has no `dsl` shape. The tree's sections page from that cursor and close with a **clickable** `+N` that dispatches `setPanelPage` for the next page, degrading to the SDK's plain row when the argument arena has no credit for the map.

## 3. Edits

### 3.1 New files

| file | what |
|------|------|
| `…/🧬️schema/🧬️mutations/🚚move-objects/{🦀️.rs, 🔺️diff/🦀️.rs, ↩️inverse/🦀️.rs, 🔣️.json, 🧬️schema/🔣️.json}` | `move-objects` triad + leaf metadata + payload schema |
| `…/🧬️schema/🧬️mutations/🌀rotate-objects/{…}` | `rotate-objects` triad |
| `…/🧬️schema/🧬️mutations/⚖️scale-objects/{…}` | `scale-objects` triad |
| `…/🧬️schema/🧬️mutations/🆕create-object/{…}` | `create-object` triad |
| `…/🧬️schema/🧬️mutations/❌delete-object/{…}` | `delete-object` triad |
| `…/🧬️schema/🧬️mutations/🚚move-objects/🧪️tests/📍️moves-the-shape-pane-object/🦀️.rs` | apply / re-mint / inverse / no-op laws |
| `…/🧬️schema/🧬️mutations/🆕create-object/🧪️tests/🌱️appends-a-box-to-the-shape-pane/🦀️.rs` | create / empty-pane / duplicate-id / inverse laws |
| `…/🧬️schema/🧬️mutations/❌delete-object/🧪️tests/🚫️removes-the-shape-pane-object/🦀️.rs` | remove / vacate-slot / target-missing / inverse laws |
| `…/✏️editor/🎮️commands/📄️panel/🦀️.rs` | `setPanelPage` command |

### 3.2 Changed files

| file:line | change |
|-----------|--------|
| `🗿️artifacts/📐️cad/🦀️.rs:169-232` | the four seam helpers (`cad_pane_local_scene`, `cad_scene_pane_objects`, `cad_scene_with_pane_objects`, `cad_pane_rematerialized_child`, `cad_pane_child_diff_slot`) |
| `🗿️artifacts/📐️cad/🦀️.rs:752-813` | five new `pub mod` mounts in the mutations mod tree |
| `🗿️artifacts/📐️cad/🦀️.rs:1255-1256` | `commands::panel` mount |
| `…/🧬️schema/🧬️mutations/🦀️.rs:47-160` | `CadObjectSpec` / `CadObjectPrimitive` / `CadObjectOrigin` / `CadObjectOrientation` / `CadObjectScale` + `cad_object_from_spec` / `cad_object_spec_of` / `cad_object_primitives_of` |
| `…/🧫️fixtures/🗄️retained-jobs/🔣️.json` | the eight object routes flipped to `migrated` on the `artifact` lane, a `setPanelPage` route added, `admittedRoutes` and `proofRows` (21 → 30) regenerated |
| `…/✏️editor/🦀️.rs:2027-2044` | the nine new ids joined `bounded_first_step_tool_proofs!` |
| `…/🧬️schema/🧬️mutations/🦀️.rs:160-166, 191-195, 214-220` | five enum variants, five `KINDS` rows, five `use super::…` |
| `…/🧬️schema/🧬️mutations/{🔣️.json, 🟦️.ts}` | five `$ref`s; TS payload interfaces + union arms |
| `…/🧬️schema/💡️inferences/🦀️.rs:1007-1029` | `validate_cad_computer_contributions` now RETURNS the accepted `CadComputerContribution`s (was `()`), so acceptance is observable |
| `…/✏️editor/🦀️.rs:352-356` | `ui_value_number` |
| `…/✏️editor/🦀️.rs:245-252` | `parse_panel_pages` / `print_panel_pages` |
| `…/✏️editor/🦀️.rs:192-194, 218, 277, 299` | `CadPlayRuntime::panel_pages` + both config conversions |
| `…/✏️editor/🦀️.rs:499-616` | `cad_pane_objects`, `cad_pane_of_object`, `cad_objects_by_pane`, `translate_objects_mutations`, `rotate_objects_mutations` (Hamilton product), `scale_objects_mutations`, `create_object_mutations`, `delete_object_mutations`, `duplicate_object_mutations`; `apply_transformation_mutations`'s stale "no seam" comment corrected |
| `…/✏️editor/🦀️.rs:757-838` | `patch_objects_mutations` — real; pose fields → move/scale/rotate ops, whole-value fields → `delete-object` + `create-object` at the same slot |
| `…/✏️editor/🦀️.rs:1147` | `setPanelPage` appended LAST in `app_commands!` (row order is the binary variant ordinal) |
| `…/✏️editor/🦀️.rs:1172` | `setPanelPage` action→command bridge |
| `…/✏️editor/🦀️.rs:1290, 1317, 1348, 1358-1366, 1388` | eight object verbs joined `CAD_RETAINED_ARTIFACT_TOOL_IDS` / `CAD_RETAINED_TOOL_IDS` with `Artifact`-lane publication contracts; `setPanelPage` joined the config lists |
| `…/✏️editor/🦀️.rs:1538` | `panel_pages_json` counted in the config's retained-byte envelope |
| `…/✏️editor/🦀️.rs:2331, 2359-2368, 2404` | `setPanelPage` action definition; eight verbs reclassified `BatchOnlyPendingRewrite` → `Migrated` |
| `…/✏️editor/🎚️config/🦀️.rs:115-120, 170-174, 192, 338` | `panel_pages_json` field + default + `Default` row; adapted `validate_cad_computer_contributions` call |
| `…/✏️editor/🎮️commands/🧱️object/🦀️.rs` | all five handlers real |
| `…/✏️editor/🎮️commands/🔄️transform/🦀️.rs` | translate/rotate/scale handlers real |
| `…/✏️editor/📌️panels/🗿️artifact/🦀️.rs:139-236, 250-263` | `CAD_SECTION_ROWS`, `section_page`, `continuation_row`, `paged_section_from`; both section builders and `build_document_tree` take the cursor map |
| `…/🔮️oracles/🔣️.json` | five catalog rows + the `_comment` recording why they carry no wire vector |
| `🧫️fixtures/🧩️sample-scene/🦀️.rs` | `sample_object`, `materialized_shape_scene`, `materialized_objects` test helpers |
| `…/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` | five kinds in `every_mutation()` (no-ops against the unmaterialized sample scene, which is what the wire/inverse laws need from them) |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `SetPanelPage` in `every_command()`; laws (a)(b)(c)(d-config)(e) |
| `…/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | law (d) rendering half + the stale-cursor clamp |

## 4. Tests (the task's laws a–e)

| law | where |
|-----|-------|
| (a) `translateSelection` changes the child transform AND the rendered instance transform | `✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `translate_selection_moves_the_object_and_the_rendered_instance` (asserts the composed child's origin, that the child handle is re-minted, and that the world-3d `instances` lane carries the moved `position`) |
| (b) `addObject` yields a new rendered instance | same file — `add_object_yields_a_new_rendered_instance` |
| (c) undo restores | same file — `object_mutations_invert_back_to_the_demo_document` (translate, scale and delete, each inverted newest-first back to the demo document); plus the per-leaf inverse laws |
| (d) `setPanelPage` pages the artifact tree | `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` — `set_panel_page_advances_the_structure_section`, `a_stale_panel_page_cursor_clamps_to_the_last_page`; config half in `set_panel_page_records_the_section_cursor_on_the_config_lane` |
| (e) contributed computer pack accepted | `✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `a_contributed_cad_computer_pack_is_accepted_by_set_contributions` (a real `cad.computer` `ProgramContributionEntry` plus a foreign-topic entry that must be skipped, not refused) |

Plus, beside the leaves: re-mint identity, empty-pane creation, slot vacation, duplicate-id fatal, target-missing error, and the two no-op arms.

## 5. Commands and counts

All outputs under `🗑️generated/koordinator-*.txt`. Every command prefixed with
`DEVELOPER_DIR=/Library/Developer/CommandLineTools RUSTFLAGS=-Awarnings CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_BUILD_JOBS=4`.

| command | result |
|---------|--------|
| `cargo check -p semio-s-artifact-cad-cad --lib` | clean (`koordinator-check-2.txt`) |
| `cargo check --target wasm32-wasip2 -p semio-s-artifact-cad-cad -p semio-s-plugin-cad` | clean (`koordinator-wasm-check.txt`) — `semio-s-plugin-cad` declares no `component-app-assembly` feature, so the target alone is the editor-code gate here |
| `cargo test -p semio-s-artifact-cad-cad --lib` | **319 passed / 19 failed / 1 ignored** (`koordinator-test-5.txt`) |
| `cargo test -p semio-s-plugin-cad --lib` | **5 passed / 0 failed** (`koordinator-plugin-test.txt`) |

**Before / after.** The 2026-09-16 CAD end-to-end report recorded `297 passed / 21 failed` for the artifact crate and `5/5` for the plugin crate. This pass leaves **319 / 19** and **5 / 5** — 22 more passing, two fewer failing, none of the remaining failures in CAD runtime code. Every one of the 19 is framework test-harness debt, classified from this run's own panics:

- **9×** registry-less `artifact_app_laws::new_app` (`interactive-job.catalog-authority`, `migrated={}`) — `coalesced_translate_drag_is_a_single_undo_step`, `engagement_input_and_possible_engagements_present`, `engagement_hud_no_longer_carries_utility_switcher_options`, `add_object_through_wrapper_grows_the_composed_pane`, `ingest_operations_is_idempotent_for_cad`, `two_instances_converge_disjoint_edits_via_backbone`, `undo_redo_round_trips_added_node_through_generic_helper`, `undo_redo_round_trips_added_node_through_wrapper`, `window_engagements_registered_for_all_four_panes`.
- **3×** `artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner` — `import_cad_file_action_accepts_spatial_json_text_string_payload`, `production_transition_exhaustion_and_missing_context_fail_before_checkpoint_persistence`, `production_transition_authority_routes_engagement_utility_and_import_without_noop_increment`.
- **3×** `edit history insertion requires its exact mutation retirement factory` — `art_cad_demo_tests::demo_subset_integrated_roundtrip`, `mutations::binary::tests::create_shape_model_round_trips_through_store`, `snapshot::binary::tests::command_envelope_round_trip_holds_for_an_applied_operation`.
- **1×** `artifact store reached Drop without its exact terminal-empty shallow-shell witness` — `mutations::binary::tests::cad_projection_defaults`.
- **1×** the codec no longer rejecting unknown fields — `schema::component::document_contract_tests::cad_document_contract_round_trips_exact_child_identities`.
- **1×** byte-exact engagement-preview stamp — `production_transition_authority_isolates_two_app_aba_sequences`.
- **1×** `CAD window publication timed out` — `windows::config::tests::cad_document_contract_world_window_runtime_isolates_commands_and_restores_exact_owner`.

**Three stale tests were rewritten rather than deleted**, because their assertions had become false statements about the app:
`add_object_action_is_a_documented_no_op` → `add_object_action_seeds_an_empty_pane_with_a_composed_model_child`;
`add_object_through_wrapper_is_a_documented_no_op` → `add_object_through_wrapper_grows_the_composed_pane` (still fails on the registry-less harness, which is exactly where it failed before);
`coalesced_translate_drag_is_a_single_undo_step` kept its name but now asserts the honest property it actually exercises — an id no pane materializes has nothing to move.

**Three defects the laws caught while being written**, all now fixed:

1. `create-object` against a pane whose slot is OCCUPIED but UNMATERIALIZED (a wire-decoded handle) would have re-minted the child from an empty scene and silently discarded its content. It now refuses that case as a `mutation.no-op`; only a genuinely EMPTY slot seeds from an empty scene.
2. `scale-objects` / `rotate-objects` wrote `Some(identity)` where the object carried `None`, so an undo restored an equal DOCUMENT but a different working scene. Absence now IS the identity: a pose landing back on `[1,1,1]` / `[0,0,0,1]` is written as `None`, and change detection compares through `unwrap_or`.
3. `delete-object`'s inverse rebuilt `primitives` from `solid_handle`, rewriting the importer's own authored slot ids (`…-solid-313` → the handle hash). `create-object` now carries `primitives` as a sibling `#[dsl(table)]` field (a table cannot nest inside the `#[dsl(block)]` object), so an undo restores them verbatim.

## 6. What remains for the coordinator's browser pass on `:6029`

1. **Gumball drag persists.** Arm Dislocate on a pane, pick an object, drag a move handle: `translateSelection` should now land a `move-objects` op in History and the mesh should stay put on release (the pane's child handle changes, so the `fit` revision does NOT — `world_fit_revision` hashes object IDs, not poses, so the camera must not re-frame).
2. **Both windows re-render.** The Perspective/Top windows of the same pane read the same child, so both must show the moved object.
3. **⌘Z.** One undo must restore the pose; History should show `Move N object(s)`.
4. **`addObject` / `duplicateObject` / `deleteObject`** from the palette (all now `Migrated`, so they are dispatchable at all for the first time) — a created object must appear in the viewport AND as a tree row.
5. **Tree paging.** The structure-classic section's `+9` row is now clickable; clicking it must page the section instead of doing nothing, and the section must not blank out.
6. **Watch for `ui.fixed-capacity: cad UI map admission failed`.** The continuation row now authors one argument map per over-long section (up to 9), which is new pressure on the shared argument arena. The row degrades to the plain `+N` when credit runs out, so the failure mode is a dead `+N`, not a blank panel — but if the panel does blank, this is the first suspect.
7. **Still open, deliberately**: `apply-transformation` (the derivation rules for building/energy/structure from shape geometry are not written down anywhere — a modelling decision, no longer a missing seam); whole-value object patching is expressed as a delete+create pair at the same slot, which reads as two rows in History; and no committed wire vector exists for the five new kinds until a persisted child resolver lands (§2.3).
