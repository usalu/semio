# W3 — fem3d descriptors, dispatch migration, boot & live_visual

Implementer W3 (relaunch), ticket `26/09/06/FEM-PLUGIN-END-TO-END`, 2026-09-06.

> **NOTHING WAS COMPILED.** No `cargo`, `bun`, `nx`, or build of any kind was run (host swap exhausted per
> the coordinator's instruction). Every claim below is from reading the source and the framework contract.
> The only tool executed against the edits is `rustfmt --emit stdout` on scratch copies — a **parser**, not
> a compiler — which confirms every edited `.rs` file parses. Type-checking, trait-resolution and test
> outcomes are UNVERIFIED and are the coordinator's to establish.

---

## 1. Files changed

All paths relative to `/Users/ueli/Documents/semio`, all under
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/` unless noted.

| File | Change |
|---|---|
| `✏️editor/🎚️config/🦀️.rs` | **A** — added `Mutation::DESCRIPTORS` (3 leaves) + `descriptor()` to `Fem3dConfigMutation` |
| `✏️editor/👥️presence/🦀️.rs` | **A** — added `Mutation::DESCRIPTORS` (1 leaf) + `descriptor()` to `Fem3dPresenceMutation` |
| `✏️editor/🦀️.rs` | **B/C** — rewrote `🧵️RetainedCommands` (18 tool ids, 18 publication contracts, one contract fn, one reduce, snapshot-bounded extent); new `📬️ArtifactStorePreparation` region; `build_artifact_store_one_item_preparation_factory`; proofs macro → 18 tools + shared contract; manifest → 18 × `Migrated`; `initial_snapshot` → boot example + `[DEBUG]`; testkit `fem3d_empty_app()`; 4 new law tests + strengthened fixture test |
| `✏️editor/🧪️fixtures/🚧️retained-command-limits/🔣️.json` | route table → 18 × `Migrated` with lanes; limits updated |
| `✏️editor/🧪️fixtures/🚧️retained-command-limits/🧬️.schema.json` | regenerated to pin the new fixture exactly (18 `prefixItems`, new limit consts) |
| `✏️editor/🎮️commands/📚️set-active-example/🦀️.rs` | config reset `Snapshot` → 2 granular rows; `[DEBUG]`; new law test |
| `✏️editor/🧵️session/🦀️.rs` (`live_visual`) | `[DEBUG]` at the reconcile spawn point (instance/revision/generation/shell/job + mesh item & draw-instance credit) |
| `✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs` | `[DEBUG]` lease/snapshot line + doc comment explaining the `"[]"` base scene |
| `✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs` | `[DEBUG]` lease/snapshot line; one test switched to the empty-document app |
| `👁️viewer/🦀️.rs` | `initial_snapshot` → shared boot fn + `[DEBUG]` |
| `👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs` | `[DEBUG]` lease/snapshot line |
| `🧬️schema/📸️snapshot/📝️text/🦀️.rs` | new `fem3d_boot_snapshot()` (shared editor/viewer boot document) |
| `✏️editor/🎮️commands/⚪️add-node/🦀️.rs` | 5 tests → `fem3d_empty_app()` |
| `✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 2 tests → `fem3d_empty_app()` |
| `✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs` | 2 tests → `fem3d_empty_app()` |
| `✏️editor/🎮️commands/🧮️set-analysis-settings/🦀️.rs` | 1 test → `fem3d_empty_app()` |

Nothing outside fem3d was touched. No file was deleted or renamed. `🗑️generated/`, `🎫️ticket.json`,
`AGENTS.md` untouched. No git state-modifying command was run.

---

## 2. Deliverable A — compile blocker (`DESCRIPTORS` / `descriptor()`)

`protocol::Mutation<P>` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:145-152`) declares
`const DESCRIPTORS: &'static [MutationLeafDescriptor]` and `fn descriptor(&self)` with **no defaults**.
Verified directly against the trait, not taken from the fem2d report.

Both fem3d hand-written impls omitted them → E0046 (same class as block2d/block3d):

- `✏️editor/🎚️config/🦀️.rs:171` `impl Mutation<Fem3dConfig> for Fem3dConfigMutation` — had only `type Diff`, `diff`, `inverse`.
- `✏️editor/👥️presence/🦀️.rs:51` `impl Mutation<Fem3dPresence> for Fem3dPresenceMutation` — same.

Fixed following block3d's shape exactly
(`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/…/✏️editor/🎚️config/🦀️.rs:237-271`, `👥️presence/🦀️.rs:95-103`):
one `MutationLeafDescriptor` per variant in declaration order, `descriptor()` a `match` returning
`&Self::DESCRIPTORS[i]`.

- `Fem3dConfigMutation`: `Snapshot` → `set-snapshot`, `SetResultDisplay` → `set-result-display`,
  `SetCamera` → `set-camera`.
- `Fem3dPresenceMutation`: `Noop` → `presence-noop`.

`owner` paths are **provisional** (they name `…/🎚️config/<slug>` and `…/👥️presence/<slug>` directories
that do not exist on disk) — identical to block3d's and puzzle3d's own provisional config/presence
descriptors, and flagged as such in the doc comments. `validate_mutation_leaf_descriptor` would reject
these owners (it requires a `/🧬️mutations/` segment), but it is never invoked in a const context on these
rosters, exactly as for block3d/puzzle3d. Fixing that class properly means authoring real config/presence
mutation-leaf directories across every plugin at once — out of this ticket's scope, and deliberately not
diverged from the sibling plugins.

**Scan for other missing impls in fem3d:** `grep -rn "impl protocol::Mutation<\|impl Mutation<"` over
`✏️s/🔌️plugins/🏗️fem/` returns exactly 4 hits — the 2 fem3d ones above (now fixed) and the 2 fem2d ones
(W2's scope). Every other `Mutation` impl in fem3d comes from `#[derive(dsl::Mutations)]` on
`Fem3dMutation`, which emits both items.

---

## 3. Deliverable B — dispatch

### 3.1 Per-action classification, before → after

| # | Action | Before | After | Publication lane | Owned reducer |
|---|---|---|---|---|---|
| 1 | `addNode` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_node::handle` |
| 2 | `addBar` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_bar::handle` |
| 3 | `addFrame` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_frame::handle` |
| 4 | `addMaterial` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_material::handle` |
| 5 | `addSection` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_section::handle` |
| 6 | `addSupport` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_support::handle` |
| 7 | `addNodalLoad` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_nodal_load::handle` |
| 8 | `addMemberUdl` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_member_udl::handle` |
| 9 | `addAreaLoad` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_area_load::handle` |
| 10 | `addSolid` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_solid::handle` |
| 11 | `addLoadCase` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_load_case::handle` |
| 12 | `addCombination` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::add_combination::handle` |
| 13 | `setSelfWeight` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::set_self_weight::handle` |
| 14 | `setAnalysisSettings` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::set_analysis_settings::handle` |
| 15 | `removeSelection` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `commands::remove_selection::handle` |
| 16 | `setActiveExample` | BatchOnlyPendingRewrite | **Migrated** | Config | `commands::set_active_example::handle` |
| 17 | `setCamera` | Migrated | **Migrated** | Config | `commands::set_camera::handle` |
| 18 | `setResultDisplay` | Migrated | **Migrated** | Config | `commands::set_result_display::handle` |

**Zero actions remain `BatchOnlyPendingRewrite` in fem3d.** `grep -n BatchOnlyPendingRewrite`
over the whole `🧊️3d` tree now returns exactly one hit — a doc-comment sentence in `✏️editor/🦀️.rs:78`
explaining what the invariant protects against. No justified exceptions; every action is retained.

Lane assignment was derived by reading each handler's `Emit`:
15 handlers return `Emit::mutations(...)` only (Artifact); `setCamera`/`setResultDisplay` return
`Emit::config(...)` only (Config); `setActiveExample` returns `{ effects: [LoadDocument], config_mutations }`
— effects do not travel on a publication lane (`🔌️plugin/🦀️.rs:23039` pushes them to the
`typed_effect_outbox` after all store lanes drain), so its declared lane is Config alone.

### 3.2 Factory shape

`Fem3dRetainedCommandJobFactory` (unchanged type, rewritten inputs):

```
FEM3D_RETAINED_TOOL_IDS          18 ids, Fem3dCommand declaration order (= binary ordinal order)
FEM3D_RETAINED_PAYLOAD_SCHEMA    "fem.3d.tool-command.v1"   (unchanged)
FEM3D_RETAINED_RAW_BYTES         8_192   -> 65_536
FEM3D_RETAINED_DECODED_ITEMS     (new)      4_096
FEM3D_RETAINED_OUTPUT_BYTES      (new)    262_144
FEM3D_RETAINED_STEP_MICROS       (new)      7_500
FEM3D_RETAINED_WORK_ITEMS        1       -> 4_096
FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES (new) 65_536
FEM3D_CONFIG_{VALUE,BASE,STEP}_BYTES     512 / 512 / 4_096   (unchanged)
```

- `fem3d_retained_contract()` is now the **single source of truth**: both
  `ToolJobFactory::execution_contract` and the `bounded_first_step_tool_proofs!` `contract:` argument
  call it.
- `fem3d_retained_extent(command, snapshot, _)` — rejects any id not in `FEM3D_RETAINED_TOOL_IDS`,
  rejects a config-lane payload over `FEM3D_CONFIG_VALUE_BYTES` (preserving the old `setCamera`
  byte gate), then bounds the semantic work by the sum of the document's 8 id-keyed collections
  (`removeSelection` walks all of them) against `FEM3D_RETAINED_WORK_ITEMS`. Mirrors
  `block3d_retained_extent`.
- `fem3d_retained_reduce(...)` — now a single `command.dispatch(&ArtifactView::with_operation(...),
  &ConfigView { .. })`, i.e. **the `🎮️commands/*` handlers stay the reducers**; the retained job reuses
  the exact same owned reducers the batch path used. No duplicated command bodies (this replaced a
  two-arm `match` that hard-coded `set_camera::handle`/`set_result_display::handle`).
- `build_tool_job` is unchanged in shape; its `FEM3D_RETAINED_TOOL_IDS` gate now admits all 18.

### 3.3 Pre-existing wiring bug found and fixed (would have killed even `setCamera` at runtime)

`AppActionCatalog::validate_tool_job_rows` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12262`)
requires `registration.contract == row.contract` for a proof row to count as `exact_registered`; a row
that is neither exact nor generic faults the app with `interactive-job.catalog-authority`.

fem3d shipped a **mismatch**: `fem3d_retained_contract()` returned
`ToolExecutionContract::resumable(8_192, 64, 1, 65_536, 7_500, 1, 1)` while the proofs macro declared
`ToolExecutionContract::bounded_first_step(8_192, 64, 1, 65_536, 7_500)`. Those differ in the `shape`
field (`Resumable` vs `BoundedFirstStep`, `🎯️action-bus/🦀️.rs:259` / `:274`), so `==` is false. By reading
alone this means the fem3d editor faulted at construction even for its two "already migrated" actions.
Both now build from one fn, so they cannot drift. Shape is `BoundedFirstStep` (block3d's choice; puzzle3d
uses `resumable` in both places — either works, `shape` is only pattern-matched permissively at
`🔌️plugin/🦀️.rs:23076`).

### 3.4 Artifact publication lane — new one-item preparation factory (required, was missing)

`VcsArtifactApp` rejects a tool whose declared lane has no one-item preparation factory with
`interactive-job.publication-authority-missing` (`🔌️plugin/🦀️.rs:19566-19570`). fem3d implemented
`build_config_store_one_item_preparation_factory` only. Migrating 15 document tools onto the Artifact
lane therefore required a new `Fem3dArtifactPreparationFactory` / `Fem3dArtifactPreparation` pair
(new `📬️ArtifactStorePreparation` region), modelled line-for-line on
`Block3dArtifactStorePreparationFactory`: `preflight` admits only `HistoryLane::Document` and bounds the
mutation by its own `OpBinary::encode_op` length against `FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES`; `advance`
computes `inverse` + `post` and seals one `protocol::Edit<Fem3dMutation>`; `close_step` retires
prepared → mutation → description → base → authority. `build_artifact_store_one_item_preparation_factory`
now returns it.

### 3.5 `setActiveExample` config reset had to become granular

`Fem3dConfigPreparationFactory::preflight` explicitly **rejects** `Fem3dConfigMutation::Snapshot`
("FEM3d config preparation rejects whole-snapshot publication", `✏️editor/🦀️.rs`, config region). The
`setActiveExample` handler emitted exactly that row, so retaining the tool would have faulted it at
publication. Replaced with the two granular rows that together cover every `Fem3dConfig` field
(`SetResultDisplay { None, "static", 0 }` + `SetCamera { FemCamera::default() }`), which is
value-identical to `Snapshot { Fem3dConfig::default() }` and stays inside the bounded config envelope.
A new test in the handler asserts both (no `Snapshot` row; applying both rows to a dirtied config yields
`Fem3dConfig::default()`).

### 3.6 Fixture + fixture schema

`🧪️fixtures/🚧️retained-command-limits/🔣️.json` route table rewritten: 18 rows, all `Migrated`, with
per-row `lanes`, in `FEM3D_RETAINED_TOOL_IDS` order (the existing test asserts
`migrated == FEM3D_RETAINED_TOOL_IDS` positionally). `limits` gained `artifactStoreBytes` and
`decodedItems` and updated `rawBytes`/`commandStepBytes`/`workItems`.
`🧬️.schema.json` was **regenerated from the fixture** so its 18 `prefixItems` `const`s and its `limits`
`const`s match exactly; a scripted comparison confirms fixture ↔ schema agreement field by field.

### 3.7 New/strengthened tests (written, NOT run)

In `✏️editor/🦀️.rs`:
- `retained_command_fixture_matches_exact_routes_and_value_codec_boundaries` — now also asserts every
  fixture row's `id`/`lanes` equals the corresponding `FEM3D_RETAINED_PUBLICATION_CONTRACTS` entry, and
  pins 5 more limit constants.
- `retained_route_dispositions_are_exact_and_exhaustive` (new) — 18 tool ids / 18 proofs / 18
  publication contracts, contract shape is `BoundedFirstStep`, the registered factory's
  `execution_contract()` equals `fem3d_retained_contract()`, ids unique, `TOOL_IDS` bijection, every
  `every_command()` row owned + non-empty lanes, and every id `Migrated` on the manifest's model window.
- `both_declared_publication_lanes_have_a_preparation_factory` (new).
- `every_boot_document_mutation_is_admissible_on_the_artifact_lane` (new) — every `CreateSolid`/
  `CreateNode` built from the boot fixture passes `Fem3dArtifactPreparationFactory::preflight`, and a
  non-`Document` lane is rejected.
- `initial_snapshot_is_the_bundled_example_not_empty` (new).

In `📚️set-active-example/🦀️.rs`: `set_active_example_resets_config_without_a_whole_snapshot_row` (new).

---

## 4. Deliverable C — boot snapshot, example switching, `live_visual`

### 4.1 Boot

`Fem3dPlayApp::initial_snapshot()` returned `empty_fem3d_snapshot()` — a brand-new editor document had
zero nodes/elements/solids, so the `World3d` Model window had nothing to mesh at first paint.

Added `crate::artifacts::fem3d::dsl::fem3d_boot_snapshot()` in
`🧬️schema/📸️snapshot/📝️text/🦀️.rs` (mirrors block3d's `block3d_boot_snapshot`): parses
`FEM3D_EXAMPLE_TEXT`, falling back to the empty snapshot if the fixture ever stops parsing. It lives in
the schema text module, beside the fixture text, because the **viewer must not import through the editor
module** (`policyViewerPurityBreaches`) and both surfaces now boot the same document. Editor and viewer
`initial_snapshot()` both call it, each emitting a `[DEBUG]` line with the parsed counts.

The bundled example carries 16 nodes, 16 frame elements, 2 materials, 1 section, 1 solid, 8 supports,
2 load cases, 1 combination — comfortably inside `live_visual`'s `MAXIMUM_NODES=128`,
`MAXIMUM_ELEMENTS=128`, `MAXIMUM_REGIONS=32`, `MAXIMUM_SUPPORTS=64` preflight ceilings
(`✏️editor/🧵️session/🦀️.rs:26-29`), so the boot document is admissible to the reconcile job.

### 4.2 Example switching

`setActiveExample` is now `Migrated` and owned by the factory (§3.1/§3.5), so it is reachable in live
dispatch. `"default"` loads `FEM3D_EXAMPLE_TEXT`; any other id resets to the empty document (unchanged
semantics; this is what the new testkit `fem3d_empty_app()` exploits).

### 4.3 `live_visual` — traced, already action-agnostic

Traced `with_live_visual` / `Fem3dPageVisualLease` / `reconcile()`:

- `Fem3dPlayApp::pending_effects(doc, cfg)` → `live_visual::reconcile(doc)` and
  `mounted_job_prepare_snapshot_read` → `live_visual::prepare_snapshot_read`. Both are
  `ArtifactEditor` hooks the framework calls generically per render/operation — **they are not wired
  per action**, so there was no "only wired for the two view actions" defect to fix.
- `reconcile` keys purely on `AppRenderOperationContext { app_instance_id, base_revision, generation,
  canonical_base_revision }` (`🧵️session/🦀️.rs:3525-3599`). Any mutation advances the document
  revision/generation, so the next render produces a fresh `PendingSnapshot`, and once its
  `SnapshotPreflight` completes, `reconcile` cancels the previous job and spawns a new
  `semio.fem3d.mounted-live-visual` job. That path is identical for all 18 actions.
- `with_live_visual` (`:3601-3614`) returns `build(None)` when there is no render context, no mounted
  shell for the instance, or an identity mismatch — and both Model windows build their base scene as
  `world3d_scene(camera, "[]", "[]", …)`, overwriting only `scene.snapshot`. So **a `None` lease renders
  a visually empty world with no error**. This is the sharpest edge in the surface and is exactly what
  the new `[DEBUG]` lines make observable.

### 4.4 `[DEBUG]` instrumentation added (crate idiom: `eprintln!("[DEBUG] …")`, as in trinity's editor)

| Site | Line |
|---|---|
| `✏️editor/🦀️.rs` `initial_snapshot` | `[DEBUG] fem3d editor boot snapshot: nodes=… elements=… solids=… materials=… loadCases=…` |
| `👁️viewer/🦀️.rs` `initial_snapshot` | `[DEBUG] fem3d viewer boot snapshot: nodes=… elements=… solids=…` |
| `🧵️session/🦀️.rs` `reconcile` (spawn point) | `[DEBUG] fem3d live_visual reconcile spawn: instance=… revision=… generation=… shell=… job=0x… meshItems=… drawInstances=… cancelled=…` |
| `🧱️model/🦀️.rs` `render_with_progress` | `[DEBUG] fem3d model window render: liveVisualLease=… sceneSnapshot=…` |
| `📊️results/🦀️.rs` `render_with_progress` | `[DEBUG] fem3d results window render: liveVisualLease=… sceneSnapshot=…` |
| `👁️viewer/…/🧱️model/🦀️.rs` `render` | `[DEBUG] fem3d viewer model window render: liveVisualLease=… sceneSnapshot=…` |
| `📚️set-active-example/🦀️.rs` `handle` | `[DEBUG] fem3d setActiveExample id=… nodes=… elements=… solids=…` |

`meshItems`/`drawInstances` come from `SnapshotPreflight::credit()`'s `Fem3dPageCredit { item_count,
draw_count }` — the exact per-page item and draw-instance counts the reconcile job reserves for the
`World3d` snapshot. **How the coordinator proves non-empty rendering from console logs:** a boot line
with `nodes>0`, followed by a `reconcile spawn` line with `meshItems>0 drawInstances>0`, followed by a
`model window render` line with `liveVisualLease=true sceneSnapshot=true`. A render line with
`liveVisualLease=false` after a spawn line means an identity/generation mismatch, not a stub.

---

## 5. Test fallout from the boot change (and how it was handled)

Booting the example makes `fem3d_app()` non-empty, which breaks every test that indexed `[0]` into a
collection or asserted `is_empty()` on a fresh app. Rather than weaken those assertions, a testkit helper
was added:

`testkit::fem3d_empty_app()` — `fem3d_app()` then `dispatch(SetActiveExample { example_id: "empty" })`,
which the existing testkit `dispatch` applies through `load_document_pack` exactly as the real host
applies `Effect::LoadDocument`.

Switched to it (10 tests): all 5 in `⚪️add-node`, `app_with_load_case` + `add_nodal_load_with_no_existing_case_creates_one`
in `📍️add-nodal-load`, both in `🗂️remove-selection`, the one in `🧮️set-analysis-settings`,
`results_window_surfaces_solver_error_without_panicking_3d` in `📊️results`, and
`import_media_geometry_in_adds_a_new_solid_3d` in `✏️editor/🦀️.rs`.

Left on `fem3d_app()` (boot-agnostic or example-dependent by design): `undo_restores_document_after_add_node`
(delta-based), `an_unknown_body_key_renders_a_diagnostic_instead_of_panicking`,
`export_media_results_out_returns_solved_json_for_every_case_3d` (dispatches `setActiveExample "default"`),
both `🧱️model` window tests, `🎥️set-camera`, `👁️set-result-display`, and the `📊️results` tests that go
through `app_with_example()`.

---

## 6. Verification performed (no compiler)

1. **Parse** — every edited `.rs` file was copied to the scratchpad and run through
   `rustfmt --edition 2021 --emit stdout`; all 15 parse cleanly (brace/paren/string balance proven by the
   parser, not by eye).
2. **No dead classifications** — `grep -n BatchOnlyPendingRewrite` over `🧊️3d/**` → 1 hit, a doc comment.
3. **Symbol existence** — every newly introduced symbol has exactly one definition and ≥2 references
   (`fem3d_boot_snapshot`, `Fem3dArtifactPreparationFactory`, `Fem3dArtifactPreparation`,
   `fem3d_artifact_edit`, `admit_fem3d_artifact_mutation`, `fem3d_artifact_mutation_retained_bytes`,
   `fem3d_retained_config_value_bytes`, `FEM3D_RETAINED_DECODED_ITEMS`, `FEM3D_RETAINED_OUTPUT_BYTES`,
   `FEM3D_RETAINED_STEP_MICROS`, `FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES`, `fem3d_empty_app`).
4. **Framework symbols confirmed at source** — `Mutation::DESCRIPTORS`/`descriptor`
   (`📡️replication/🎮️mutation/🦀️.rs:145-152`); `MutationLeafDescriptor`'s 14 fields (`:320-335`) and every
   enum variant used (`:196`, `:223`, `:248`, `:275`, `:290`);
   `ArtifactEditor::build_artifact_store_one_item_preparation_factory` (`🔌️plugin/🦀️.rs:26717`);
   `ArtifactEditor::bounded_first_step_tool_proofs` (`:26650`); `validate_tool_job_rows`'s contract
   equality (`:12262`) and `interactive-job.catalog-incomplete` set equality (`:12295`); publication-lane
   authority check (`:19566`); emit-lane check + drain order (`:22945-23044`);
   `store::ArtifactStoreOneItemFootprint` (`🏪️store/🦀️.rs:13029`),
   `…PreparationRequest` (`:13217`), `ARTIFACT_STORE_ONE_ITEM_ID_BYTES` (`:13023`),
   `HistoryLane { Document, Interaction }` (`:2300`).
   Two mistakes were caught this way and corrected before finishing: `ArtifactBoundedFirstStepProof.contract`
   is a private field (assertion rewritten to compare the factory's own `execution_contract()`), and
   `HistoryLane::Presence` does not exist (`Interaction` used instead).
5. **Fixture ↔ schema** — scripted field-by-field comparison of `🔣️.json` against the regenerated
   `🧬️.schema.json`: 18/18 routes and 10/10 limits agree.
6. **Privacy/module-tree check** — the crate entry mounts `✏️editor/🦀️.rs` as a private `mod component`
   re-exported by `pub use component::*` (`📦️packages/🦀️rust/🦀️.rs:1449-1453`), so
   `commands::*` are **siblings**, not descendants, of that module; an assertion in the
   `set-active-example` test that reached for the private `Fem3dConfigPreparationFactory` was removed
   for that reason.

---

## 7. Open risks (for the coordinator's first compile / boot)

1. **Nothing is compiled.** Expect ordinary type-level fallout, most likely in the new
   `Fem3dArtifactPreparation` (trait method signatures were copied from block3d's twin, but fem's
   `store` re-export surface may differ in small ways) and in the new tests.
2. **`Fem3dConfigPreparationFactory::preflight` returns `work_items: 3, retained_bytes: 4096`** and is
   now reached by three tools instead of two. If the store's grant loop ever bounds a Config publication
   by the tool contract's `max_work_units_per_step` (=1), `setActiveExample`/`setCamera`/`setResultDisplay`
   could stall. Unchanged from before for the two view actions, so it is not a regression — but it is
   untested at runtime.
3. **Contract shape change** (`Resumable` → `BoundedFirstStep`). Verified that `shape` is only
   pattern-matched permissively (`🔌️plugin/🦀️.rs:23076`) and that block3d ships `BoundedFirstStep` with
   the same `ArtifactRetainedCommandJob`, but this is a runtime behaviour change on the two previously
   working actions.
4. **Provisional descriptor `owner` paths** (§2) would fail `validate_mutation_leaf_descriptor` if any
   gate ever calls it. Repo-wide issue shared with block3d/puzzle3d; deliberately not diverged.
5. **`removeSelection` emits N mutations**; the publication loop drains them one per turn
   (`🔌️plugin/🦀️.rs:22955`). Extent returns `1`, and the contract's `max_work_units_per_step` is `1`.
   If the framework counts drained mutations against that budget rather than against the job's own step
   budget, a large multi-select could stall. Not readable with confidence without running it.
6. **`FEM3D_RETAINED_RAW_BYTES` 8 KiB → 64 KiB** widens the accepted wire envelope. Intentional
   (`setActiveExample` replays a ~3 KB fixture), but it is a real relaxation.
7. **Owner-root descriptor is stale** — `✏️s/🔌️plugins/🏗️fem/🔣️.json` (mtime 2026-09-04) already claims
   all 152 actions are `"migrated"`, which was *wrong* before this change and is *right* now, by
   accident. It still needs `bun nx run @semio-tech/fem-plugin:describe` (ticket DoD item 3).
8. **Peer overlap** — W1 (test-case name literals), W2 (`◻️2d/**`), W4 (`🚪️io/`), W6 (`🧊️3d/…/📈️analysis/🧪️tests/`)
   have uncommitted edits in the same tree. None of my files overlap theirs; nothing of theirs was
   reverted.
