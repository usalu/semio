# Remaining Artifact Extraction Preparation

Read-only source inventory for the 57 non-stdio artifacts, excluding nested generator/probe/bridge packages. Crate-root symbols are lexical references and include tests/comments; use mounted-source inspection to determine final Cargo edges.

| Artifact | Rust Files | Lines | Crate-Root Symbols |
| --- | ---: | ---: | --- |
| ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer | 111 | 10680 | artifacts, editor, plugin, viewer |
| ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation | 120 | 19739 | MathematicalApps, algebra, artifacts, cas, editor, number, polynomial, viewer |
| ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d | 132 | 14582 | artifacts, editor, generation_form, generation_tree, scene_surface, ui_label, ui_node_list, ui_value_map, ui_value_text, viewer |
| ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d | 161 | 17880 | artifacts, editor, examples, generation_form, generation_tree, scene_surface, ui_label, ui_node_list, ui_text, ui_value_list, ui_value_map, ui_value_text, viewer |
| ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly | 89 | 18414 | artifacts, editor, sampling, viewer, wfc_engine |
| ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow | 141 | 15023 | artifacts, editor, examples, playbook, plugin, viewer |
| ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain | 71 | 5468 | artifacts, editor, modules, viewer |
| ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap | 124 | 12255 | artifacts, editor, plugin, viewer |
| ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs | 82 | 6217 | VcsApps, artifacts, editor, viewer |
| ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation | 120 | 18470 | apps, artifacts, editor, examples, viewer |
| ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting | 213 | 14016 | artifacts, editor, viewer |
| ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground | 39 | 2667 | artifacts, editor, viewer |
| ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence | 99 | 11790 | artifacts, editor, plugin, viewer |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d | 249 | 29607 | FemApps, analyses, app_surface, artifacts, core, editor, elements2d, fem2d_engine, mesh, model, sparse, viewer |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d | 248 | 29642 | FemApps, analyses, app_surface, artifacts, editor, elements3d, fem3d_engine, mesh, model, sparse, viewer |
| ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program | 1131 | 72955 | apps, artifacts, editor, viewer |
| ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d | 134 | 21454 | __semio_dispatch_MachineCatalog, artifacts, editor, viewer |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly | 149 | 16028 | apps, artifacts, editor, viewer |
| ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires | 104 | 8037 | artifacts, editor, plugin, viewer |
| ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms | 126 | 13270 | FormsApps, artifacts, editor, playbook, viewer |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout | 133 | 19423 | artifacts, editor, viewer |
| ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad | 154 | 19365 | artifacts, editor, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990 | 74 | 6062 | app_surface, artifacts, config, document, editor, examples, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599 | 86 | 6641 | app_surface, artifacts, config, document, editor, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997 | 122 | 7807 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798 | 282 | 18558 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991 | 162 | 9810 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992 | 174 | 11037 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805 | 110 | 9153 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757 | 118 | 9509 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993 | 102 | 9523 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994 | 122 | 7378 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108 | 122 | 8869 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996 | 122 | 7836 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, impl_norm_set_snapshot_ops, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995 | 114 | 7405 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999 | 138 | 8432 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998 | 230 | 14064 | app_surface, artifacts, config, document, editor, impl_norm_artifact_record, norm_owned_tool_job_factory, viewer |
| ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook | 93 | 6855 | PlaybookApps, artifacts, editor, mutation, playbook, viewer |
| ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure | 87 | 7035 | artifacts, editor, extensions, viewer |
| ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling | 373 | 61103 | RemodelApps, algebra, artifacts, editor, examples, graph_matching, lie, optimize, signal, spatial, viewer |
| ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model | 1452 | 85811 | EnergyQualityTier, EnergyWireIdentity, Model, Results, air_exchange, artifacts, bestest, calendar, editor, energy_simulation_session, examples, model, schedule, viewer |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting | 116 | 8226 | TrinityApps, artifacts, ast, editor, executor, language_service, lexer, viewer |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack | 140 | 13209 | TrinityApps, artifacts, ast, core, editor, executor, language_service, lexer, viewer |
| ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag | 125 | 8373 | DagApps, artifacts, editor, viewer |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | 144 | 24911 | BitSet, Command, DrawApps, Host, Model, NoMigrations, NodeId, Status, TimerId, artifacts, editor, explore, init, macrostep, restore, statechart, timer_elapsed, viewer |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster | 125 | 16371 | artifacts, editor, viewer |
| ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note | 244 | 19354 | NoteApps, artifacts, document, editor, markup, viewer |
| ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d | 254 | 28741 | PuzzleApps, artifacts, editor, examples, retained_command, viewer |
| ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d | 226 | 24273 | PuzzleApps, artifacts, editor, examples, retained_command, viewer |
| ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d | 270 | 39150 | PuzzleApps, artifacts, editor, examples, retained_command, viewer |
| ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d | 161 | 10066 | BlockApps, BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, artifacts, editor, examples, register_block_exports, viewer |
| ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d | 220 | 13081 | BlockApps, BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation, artifacts, editor, examples, viewer |
| ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d | 225 | 14011 | BlockApps, BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation, artifacts, editor, examples, viewer |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home | 67 | 5333 | DirectorySpaceRole, HomeSpaceRow, HomeTableLabels, X, artifacts, catalog_port, create_and_register_ephemeral_studio, draft_backbone_port, editor, ephemeral_draft_catalog, home_space_rows, list_all_space_catalog_entries, register_studio_port, resolve_studio_document, space_retained_store_preparation, sync_os_space_document_helper, temp_catalog_port, viewer |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space | 57 | 4315 | artifacts, editor, space_retained_store_preparation, viewer |
| ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation | 85 | 7782 | SourcingApps, apps, artifacts, editor, viewer |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run | 6 | 198 |  |

## Parallel Work Boundaries

- Norm has 15 standards sharing the compliance, config, and app-surface modules. Extract shared domain contracts first; keep every standard artifact as its own leaf package.
- Most other plugins have one artifact. Separate artifact schema/codecs and artifact-owned editor/viewer definitions from plugin installation. Their existing crate-root module references identify shared owner code that must move to an acyclic lower package, not depend back on the plugin.
- Puzzle/block/procedural/GIS/space/trinity/FEM own multiple artifacts and need explicit cross-artifact dependencies.
- The framework workflow/run artifact is a separate owner and requires its own package/launch integration.
- Source types outside artifact directories can remain in their current domain taxonomy; package declarations must point out to those sources rather than copying implementations.

## Language Declaration Readiness

There are 57 remaining roots, 3 with a root TypeScript implementation and 0 with the same artifact definition JSON path as stdio.

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d`: TypeScript root=True; common definition path=False.
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d`: TypeScript root=True; common definition path=False.
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d`: TypeScript root=True; common definition path=False.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space`: TypeScript root=False; common definition path=False.
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation`: TypeScript root=False; common definition path=False.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run`: TypeScript root=False; common definition path=False.

## Composable App Declarations

The framework already supports `ArtifactDeclaration<PA: PluginApp>` and `editor_surface<E, PA>` / `viewer_surface<V, PA>` with `From<VcsArtifactApp<EditorApp<E>>>` and the viewer equivalent. Current artifact/standard/subset functions refer to plugin-wide app enums such as `FormsApps`. Make those declaration factories generic over a per-artifact bound satisfied by the composer enum; the package then exposes real editor/viewer implementations without importing its owning plugin. A local trait can collect repeated bounds. Keep shared lower packages independent of every artifact and avoid an all-sibling app enum in shared contracts.

Forms package facade groups artifact/schema, editor, viewer, examples and plugin sections. The first four belong to the forms artifact crate; the plugin installation entry stays in the composition crate. Single-artifact root helpers referring to plugin() may be test-only; inspect each mounted source before moving.
