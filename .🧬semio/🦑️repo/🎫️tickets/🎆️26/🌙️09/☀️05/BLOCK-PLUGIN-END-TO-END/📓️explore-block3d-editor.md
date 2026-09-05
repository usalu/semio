# block3d editor — exploration

App: `s.block.block3d@1/*#editor` (`Block3dPlayApp`), plugin `✏️s/🔌️plugins/🧱️block`.
Subset root: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`.
Precedent read first: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️03/PROCEDURAL-3D-END-TO-END/📓️status.md` ("Root cause" —
gen3d's bare `factory: "BoundedFirstStepCommandJobFactory"` without `factory_type:` makes every action
dispatch-dead one layer *below* the UI-classification gate). **block3d fails one layer *above* that: it
never reaches the tool-proof/factory stage at all, because none of its actions are classified `Migrated`.**

## 1. Modes / windows / SurfaceKind / non-empty-render requirement

Editor (`✏️editor`): exactly **one mode**, `edit` (`BLOCK3D_PLAY_MODE_EDIT`,
editor `🎭️modes/✏️edit/🦀️.rs:6-18`), exactly **one window kind**, `block3d-world`
(`BLOCK3D_WINDOW_WORLD`, editor `🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️.rs:13,21-38`):
`SurfaceKind::World3d` (`:26`), `body_key = "block3d.play.world"`.

Render (`world::render`, `🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️.rs:53-84`) builds a `world3d_scene_extended`
from `world_meshes_json`/`world_instances_json`/`world_vortices_json` (compute in editor `🌍️world/🦀️.rs`).
Non-empty content requires:
- `Block3dSnapshot.representations` non-empty (`visible_representations`, `🌍️world/🦀️.rs:19-24` — returns
  ALL representations when the window's `representation_ids` view is empty, which it is by default), **and**
- each representation's `mesh_url: Option<String>` set (`world_meshes_json` silently drops any
  representation whose `mesh_url` is `None`, `🌍️world/🦀️.rs:51-60`; `world_instances_json` still emits an
  instance record for it, `🌍️world/🦀️.rs:62-82`, so a doc with representations but no mesh URLs renders
  instances with no backing mesh).
- Vortices need `Block3dSnapshot.vortices` non-empty to render rim markers (`world_vortices_json`).

Viewer (`👁️viewer`): one mode `view`, one window kind — the **shared** `MeshWindowKit::window_kind()`
(`framework.window.mesh`, also `SurfaceKind::World3d`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26186-26198`). Render
(`👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs:33-46`) is a pure, read-only
`Block3dSnapshot -> UiNode`; same mesh_url-gated-mesh / always-present-instance behavior, zero
arrangement offset (viewer has `Config = NoConfig`, no per-session window view). Viewer's sole command
is an inert `Block3dViewCommand::Noop` (`👁️viewer/🦀️.rs:18-34`) — structurally never mutates.

## 2. Document/snapshot state and default boot document

`Block3dSnapshot` (persisted document) fields declared in `Block3dArtifact`
(`🧬️schema/🦀️.rs:16-62`, `#[state(artifact)]` rows): `schema`, `object_kind`, `representations`,
`catalog` (child `SemioKitSnapshot`), `vortex_kind_extra`, `vortices`, `compatibility`, `attributes`,
`authors`, `camera3d`, `meta`, plus `brush_preview`/`hovered_vortex_full_id` (also `#[state(artifact)]`,
despite being view-ish). Presence-lane (`selected_ids`, `active_representation_id`, `wanted_tags`) and
config-lane (`locale`, `windows`, `brush_*`, `camera`) fields live on the same struct only for the
combined schema descriptor; the real runtime split is `Block3dSnapshot` (doc) vs. `Block3dConfig`
(editor `🎚️config/🦀️.rs:23-51`, session-only-but-undoable) vs. `Block3dPresence` (editor
`👥️presence/🦀️.rs:17`, now genuinely empty — selection/hover moved to the framework's `vortex`
interaction domain per ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).

**Default boot document is fully empty**: `ArtifactEditor::initial_snapshot` (editor `🦀️.rs:226-228`)
calls `crate::artifacts::block3d::schema::empty_block3d_snapshot()`
(`🧬️schema/🦀️.rs:287-289`), which is literally `Block3dSnapshot::default()` — zero representations,
zero vortices, empty object kind. **No example is parsed at boot** (unlike the sibling gen3d app, whose
status doc records a `hexagonal-mushroom-column` default). The world window therefore renders visually
empty until a client explicitly dispatches `setActiveExample`.

**`mesh-collection` `/mesh` route** (Cargo.toml `[[package.metadata.semio.assets]]`,
`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml:44-48`, `route = "/mesh"`,
`catalog = "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json"`, scoped to
`app = "s.block.block3d@1/*#editor"`): parsed by the registry generator
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:335-366,444-463,1042-1049`) into
the playground registry as a dev-server asset-serving spec (validated: route must be `/mesh`, catalog
file must exist). Consumption at runtime: `representation.mesh_url` values in the document (e.g.
`"/mesh/🧊️capsule_J.glb"`) are resolved by
`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🟦️.ts` — `MESH_DELIVERY_CATALOG` parses
`🥽️mesh/📇️catalog.json` (3 direct entries: two `hexagonal-cut-concrete-forest-{left,right}.glb` +
`placeholder.glb`) plus one nested `collections` catalog
(`🌱️metabolism/🎨️representation/📇️catalog.json`, hundreds of mesh entries incl. `capsule_J.glb`).
`resolveMeshAsset`/`meshAssetTransportUrl` (same file, :70-84) are called from
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:1535,1624`
(the `useLoader(GLTFLoader, meshAssetTransportUrl(url))` call) at scene render time — i.e. per mesh
in the rendered scene, not at document-load time.

## 3. Every editor command — id, declaration, classification, handler

All 23 rows come from the `Block3dCommand` enum (`app_commands!`, editor `🦀️.rs:169-200`); every
row has a real handler module under `✏️editor/🎮️commands/*` (all 23 `#[path]` mounts in the crate
entry resolve to files that exist — verified programmatically, see §6). **Manifest declaration is via
`.mutation(id, …)` / `.view_action(id, …)` in `create_block3d_app` (editor `🦀️.rs:424-444`)**, and
**block3d's manifest never calls `.action_interactive_job(id, InteractiveJobClassification::Migrated)`
for any of them** (zero occurrences anywhere in the file — confirmed by grep). `.mutation()`/`.view_action()`
build their `ActionDefinition` via `ActionDefinition::bounded_catalog` →`new_catalog`
(`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:1010-1019`), which — per its own doc comment — "builds a
catalog row **without granting UI execution authority**"; `interactive_job` stays at its type default,
`InteractiveJobClassification::Unclassified`.

| # | command id | wire key | declared as | classification | handler |
|---|---|---|---|---|---|
| 1 | patchObjectKind | patchObjectKind | `.mutation` | **Unclassified** | `commands/🏷️patch-object-kind` |
| 2 | addRepresentation | addRepresentation | `.mutation` | **Unclassified** | `commands/🧱️add-representation` |
| 3 | removeRepresentation | removeRepresentation | `.mutation` | **Unclassified** | `commands/🚮️remove-representation` |
| 4 | addVortexKind | addVortexKind | `.mutation` | **Unclassified** | `commands/🔘️add-vortex-kind` |
| 5 | removeVortexKind | removeVortexKind | `.mutation` | **Unclassified** | `commands/🗑️remove-vortex-kind` |
| 6 | addVortex | addVortex | `.mutation` | **Unclassified** | `commands/🌀️add-vortex` |
| 7 | removeVortex | removeVortex | `.mutation` | **Unclassified** | `commands/➖️remove-vortex` |
| 8 | setActiveExample | setActiveExample | `.mutation` | **Unclassified** | `commands/🎬️set-active-example` |
| 9 | edit | edit | `.mutation` | **Unclassified** | `commands/🎨️edit` |
| 10 | setActiveRepresentation | setActiveRepresentation | `.view_action` | **Unclassified** | `commands/🪟️set-active-representation` |
| 11 | setWindowRepresentations | setWindowRepresentations | `.view_action` | **Unclassified** | `commands/🖼️set-window-representations` |
| 12 | toggleWindowRepresentation | toggleWindowRepresentation | `.view_action` | **Unclassified** | `commands/🔀️toggle-window-representation` |
| 13 | setWindowArrangement | setWindowArrangement | `.view_action` | **Unclassified** | `commands/↔️set-window-arrangement` |
| 14 | setWindowSpacing | setWindowSpacing | `.view_action` | **Unclassified** | `commands/📐️set-window-spacing` |
| 15 | setActiveUtility | setActiveUtility | **not declared by block3d** — matched by the framework's auto-injected `setActiveUtility` action (id collision is intentional/shared) | **Migrated** (framework auto-injection, since `.utility(...)` is declared — `try_build_definition`, `🔌️plugin/🦀️.rs:5408-5410`, builds it via `ActionDefinition::resumable_framework_catalog`, which *does* set `Migrated`, `🛂️manifest/🦀️.rs:1023-1027`) | `commands/🪛️set-active-utility` |
| 16 | setBrushVortexKind | setBrushVortexKind | `.view_action` | **Unclassified** | `commands/🧬️set-brush-vortex-kind` |
| 17 | setBrushRadius | setBrushRadius | `.view_action` | **Unclassified** | `commands/📏️set-brush-radius` |
| 18 | setBrushFlip | setBrushFlip | `.view_action` | **Unclassified** | `commands/🔄️set-brush-flip` |
| 19 | HoverSurface | hoverSurface (action id `worldSurfaceHover`) | `.view_action` | **Unclassified** | `commands/🖌️hover-surface` |
| 20 | LeaveSurface | leaveSurface (action id `worldSurfaceLeave`) | `.view_action` | **Unclassified** | `commands/👋️leave-surface` |
| 21 | PlaceVortex | placeVortex (action id `worldSurfacePlace`) | `.mutation` | **Unclassified** | `commands/📍️place-vortex` |
| 22 | setCamera | setCamera | **not declared at all** (no `.mutation`/`.view_action`/window action; editor `🦀️.rs:286-291` documents this as a known pre-migration gap: "the `Block3dCommand::SetCamera` variant was only reachable via direct `dispatch_typed`/binary `OpBinary`") | n/a — no action-registry entry to classify | `commands/🎥️set-camera` |
| 23 | patchRepresentation | patchRepresentation | `.mutation` | **Unclassified** | `commands/🩹️patch-representation` |

**Effect**: `validate_ui_dispatch_classification` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12033-12039`)
accepts only `InteractiveJobClassification::Migrated`, and is invoked unconditionally as the *first* gate
in both real UI dispatch entry points — `dispatch_action` (`:22354-22360`, called from `PluginApp::handle_action`,
`:24850-24854`, the trait method the host calls for every `{action,args}` UI event) and `dispatch_command`
(`:22389-22407`). Since 20 of 23 block3d commands are `Unclassified` and 1 (`setCamera`) is not registered
as an action at all, **every block3d editor action except `setActiveUtility` is dispatch-dead from the real
UI path**: it fails immediately with `Fault("interactive-job.not-ui-safe", …)` (or, for `setCamera`,
`Fault("interactive-job.unknown-key", …)`) before command construction, before `admit_command_json`, before
any tool-proof/factory logic ever runs. This includes `setActiveExample` — **the example loader is itself
dispatch-dead**, so the empty default document (§2) can never be replaced with real content through the
normal UI dispatch path either. (Tests bypass all of this: the testkit's `dispatch()`
(editor `🦀️.rs:486-488`) calls `app.dispatch_typed(command, …)` directly on the typed enum, which never
goes through `dispatch_action`/`validate_ui_dispatch_classification` — this is why every `#[test]` in the
file passes despite the runtime dead end.)

## 4. Tool proofs / factory / RETAINED_TOOL_IDS

**block3d has none of this wiring at all** — zero occurrences anywhere under
`✏️editor/🦀️.rs` (grepped for `tool_proofs`, `factory_type`, `BoundedCommandJobFactory`,
`register_tool_job_factories`, `build_tool_job`, `RETAINED_TOOL_IDS`,
`build_artifact_store_one_item_preparation_factory`, `action_interactive_job`). No
`bounded_first_step_tool_proofs!` macro invocation, no app-owned `*BoundedCommandJobFactory`, no
`BLOCK3D_RETAINED_TOOL_IDS` constant. (Since no action ever reaches `Migrated`, this second layer is
never even exercised at runtime for block3d — the classification gate above rejects first.)

Compare to the sibling apps in the same plugin:
- **block5d**: HAS all of it — `BLOCK5D_RETAINED_TOOL_IDS: &[&str; 7]`
  (`🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:165`), a
  `.action_interactive_job(id, InteractiveJobClassification::Migrated)` call for every one of its 7
  actions (`:597-603`), `ToolExecutionContract::bounded_first_step`/`bounded_first_step_tool_proofs!`
  with `factory_type:`, `build_artifact_store_one_item_preparation_factory` override, etc. This is the
  ONLY app in the whole `block` plugin with a `🧪️publication-authority` fixture
  (`✏️s/🔌️plugins/🧱️block/🧪️publication-authority/🔣️.json`, `"owner": "Block5dPlayApp"`) whose oracle
  (`📦️packages/🟦️typescript/📜️script.ts:9-16`) actually verifies the `RETAINED_TOOL_IDS`
  set / `action_interactive_job(Migrated)` set / publication-lane contract set are all mutually exact —
  this is the plugin's ONLY enforced regression gate for this fault pattern, and it only covers block5d.
- **block2d**: has `.action_interactive_job(id, InteractiveJobClassification::BatchOnlyPendingRewrite)`
  for its 8 own-authored actions (`🗿️artifacts/◻️2d/…/✏️editor/🦀️.rs:336-344`) — explicitly and
  correctly marked NOT UI-safe (batch-only), a real (if incomplete) classification, unlike block3d's
  silent default.
- **block3d**: neither — its 20 own-authored actions were simply never classified either way. There is
  no test anywhere in the repo (not even a block3d-scoped `🧪️publication-authority` fixture — none
  exists) that would catch this; `command_from_action_covers_every_declared_action_and_rejects_unknown_ones`
  (editor `🦀️.rs:569-573`) only checks the action→command *bridging* function, never classification.

## 5. Examples

Two real, wired examples, both id-consistent end to end:
- `nakagin-capsule` (`BLOCK3D_EXAMPLE_CAPSULE`, `commands/🎬️set-active-example/🦀️.rs:4`)
- `hexagonal-cut-concrete-forest-left` (`BLOCK3D_EXAMPLE_FOREST_LEFT`, `:5`)

`setActiveExample`'s handler (`commands/🎬️set-active-example/🦀️.rs:207-217`) matches the id, parses the
matching DSL text via `crate::artifacts::block3d::dsl::parse_dsl(…_EXAMPLE_TEXT)`, and emits the minimal
ordered mutation batch that carries the current document to the parsed one
(`replace_document_operations`, `:9-197` — a real semantic diff, never a banned whole-document-replace
op). The `..._EXAMPLE_TEXT` constants are `include_str!`'d at compile time from the SAME
`.dsl.semio` fixture files the artifact-level `ExampleSource` wrappers use
(`🧬️schema/📸️snapshot/📝️text/🦀️.rs:13,15` →
`📚️examples/🏢️nakagin-capsule/🖼️assets/🧪️nakagin-capsule/🗣️.dsl.semio` and
`📚️examples/🎬️hexagonal-cut-concrete-forest-left/🖼️assets/🧪️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio`).
Parsing itself happens **at runtime**, once per `setActiveExample` dispatch (not compile-time, not
test-only) — `PRIMARY_TEXT`/`..._EXAMPLE_TEXT` are only the *source bytes*, embedded at compile time via
`include_str!`; `dsl::parse_dsl(...)` runs when the command executes. Real registry wiring:
`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:19-22` builds
`SubsetDeclaration.examples` from `crate::examples::art_3d_hexagonal_cut_concrete_forest_left::source()` /
`art_3d_nakagin_capsule::source()` (the artifact-level `📚️examples/*/🦀️.rs` modules, crate-mounted at
`📦️packages/🦀️rust/🦀️.rs:2690-2696`) — this is the live subset-level example catalog, separate from (and
unaffected by) the app-builder `.example()` gap noted next.

**But**: because `setActiveExample` is `Unclassified` (§3), it is dispatch-dead through the real UI path
regardless of how correctly the example machinery itself is wired — examples can only be loaded through
`dispatch_typed` (tests / non-UI internal callers), never through `handle_action`.

There is also a THIRD, editor-scoped `demo-session` example
(`✏️editor/📚️examples/🎬️demo-session/🦀️.rs`, `ExampleSource::new("demo-session", …, "🖼️assets/🎮️.cmd.semio", "play")`),
mounted into the crate as `examples::app_3d_demo_session` (`📦️packages/🦀️rust/🦀️.rs:2679-2680`) but
**its `source()` function is never called anywhere** — confirmed by grep across the whole plugin. This
matches the manifest's own comment (editor `🦀️.rs:446-453`): the current `EditorBuilder` has no
`.example(...)`/`.workflow(...)` method at all, so this module is dead weight, compiled but unreachable.

## 6. Other defects found

- **(Primary, §3)** All 20 block3d-authored editor actions are `Unclassified` → rejected by
  `validate_ui_dispatch_classification` at the very first gate of real UI dispatch. This is a strictly
  earlier-stage failure than the gen3d precedent (which had `Migrated` actions dying later, at the
  tool-proof/factory stage) — block3d never even reaches that stage. No test in the repo catches this
  (no block3d publication-authority fixture exists; block5d's is the plugin's only one).
- **`setCamera` has no manifest action entry at all** (not `Unclassified` — simply absent from the
  registry), so it fails even earlier, with `interactive-job.unknown-key`. Self-documented as a known
  gap in a comment (editor `🦀️.rs:286-291`), but that comment describes it as fixed by adding the
  `command_from_action` arm — it does NOT mention that the action is still never manifest-declared, so
  the arm is unreachable from the UI regardless.
- **Broken mesh asset reference**: `📚️examples/🏢️nakagin-capsule/🖼️assets/🧪️nakagin-capsule/🗣️.dsl.semio:15`
  (and the matching Rust literal, `🧬️schema/📸️snapshot/📝️text/🦀️.rs:52`) set representation `r1`
  (`"1:500"`) `mesh_url` to `"/mesh/capsule_J.1to500.glb"`. This exact string is **not present in any mesh
  catalog** in the repo (`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json` nor the nested
  `🌱️metabolism/🎨️representation/📇️catalog.json` — confirmed by repo-wide grep for `1to500`, only hits in
  this fixture, its Rust mirror, and unrelated historical ticket files). `resolveMeshAsset`
  (`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🟦️.ts:69-78`) throws `Unknown mesh asset: …` for any URL not in
  the catalog; `World3dHost.tsx:1535/1624` calls this synchronously before `useLoader(GLTFLoader, …)`.
  Because the default window view has empty `representation_ids` (`visible_representations` then shows
  ALL representations, `🌍️world/🦀️.rs:19-24`, and `Block3dWindowView::for_window`'s default is
  `representation_ids: Vec::new()`, `🗿️artifacts/🧊️3d/🦀️.rs:178-208`), loading the `nakagin-capsule`
  example renders BOTH representations by default, so this throws immediately on load — no user action
  needed beyond `setActiveExample("nakagin-capsule")` (itself currently unreachable per the above, so
  this second bug is presently masked by the first, but would surface immediately once classification is
  fixed).
- **`#[path]` mounts**: all 244 block3d-related `#[path = "…"]` attributes in the crate entry
  (`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/🦀️.rs`) were checked programmatically against the
  filesystem — **none dangle**; every mounted file/dir exists. No stale/renamed-file references found for
  block3d specifically.
- No `todo!()`/`unimplemented!()`/stray `panic!` in the block3d editor tree; the only `NotImplemented`
  return (editor `🦀️.rs:366`) is a deliberate, documented default-port fallthrough in `export_media`
  (only `"catalog:out"`/`"document:out"` are real ports).

## Appendix — files read

- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (main editor/manifest)
- `…/✏️editor/🎚️config/🦀️.rs`, `…/✏️editor/🌍️world/🦀️.rs`
- `…/✏️editor/🎭️modes/✏️edit/🦀️.rs`, `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️.rs`
- `…/👁️viewer/🦀️.rs`, `…/👁️viewer/🎭️modes/👁️view/🦀️.rs`, `…/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs`
- `…/🧬️schema/🦀️.rs`, `…/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs`, `…/✏️editor/👥️presence/🦀️.rs`
- `…/✏️editor/📌️panels/🗿️artifact/🦀️.rs`, `…/✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `…/✏️editor/📚️examples/🎬️demo-session/🦀️.rs`, `…/📚️examples/🏢️nakagin-capsule/🦀️.rs`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/🦀️.rs` (crate `#[path]` wiring), `…/Cargo.toml`
- `✏️s/🔌️plugins/🧱️block/🧪️publication-authority/🔣️.json`, `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (block5d comparison)
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (block2d comparison)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (builder/registry/dispatch internals:
  `action_interactive_job`, `validate_ui_dispatch_classification`, `AppActionRegistry::from_definition`,
  `try_build_definition`, `dispatch_action`, `handle_action`, `MeshWindowKit`)
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (`ActionDefinition::{new_catalog,bounded_catalog,resumable_framework_catalog}`,
  `set_active_utility_action_definition`)
- `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🟦️.ts`, `…/📇️catalog.json`, `🌱️metabolism/🎨️representation/📇️catalog.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` (mesh URL usage)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` (mesh-collection asset parsing)
