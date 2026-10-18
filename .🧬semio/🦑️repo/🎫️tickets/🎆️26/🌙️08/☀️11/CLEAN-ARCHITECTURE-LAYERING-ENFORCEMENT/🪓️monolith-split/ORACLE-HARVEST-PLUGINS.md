# Oracle harvest, plugin families: ledger

Machine ledger: `ohp-tools/ledger.tsv`. Section 1 lists each real or artifact class with its fix; section 2 gives the proof per family (non-stub rows of the resolution and privacy codes, first run against the last run, `new` = rows that appear only in the last run).

## 1. Rows by class

| family | cfg | row | class | fix |
|---|---|---|---|---|
| trinity | test | E0425 x30 schema::diff::{en,de}code_rewriting_diff_json (10 rewriting test files) | REAL | call sites re-pointed to io::text::diff (functions live in io::text::diff); operations mirror gained pub use of io::text::diff (structural-correspondence test) |
| trinity | test | E0425 jack_envelope_decode_owner_bundle in io::binary::mutations (dev editor test) | REAL | now schema::host::jack_envelope_decode_owner_bundle |
| trinity | test | E0425 commands::preset_query (dev editor test) | REAL | now commands::set_active_example::preset_query |
| trinity | test | E0425 JackResultsWindowTransientOwner x3 (dev editor test) | REAL | explicit import from jack::editor::jack::transient |
| trinity | test | E0425 schema::executor::run (dev editor test) | REAL | now schema::runner::run |
| trinity | test | E0425 cx (dev layout test nakagin_app) | REAL | extra stale cx argument dropped (new_app_with_registry_and_members takes 4 arguments) |
| trinity | lib | E0599 Graph::subgraph_snapshot x2 (jack language-service queryable-graph) + dev content tests x2 | REAL | method restored as inherent Graph::subgraph_snapshot in memory-graph (free fn in executor removed; executor calls the method) |
| trinity | lib | E0603 admit_demand private (jack executor execution) | REAL | host: pub fn admit_demand, pub use snapshot_decode::* |
| trinity | test | E0425 JACK_OWNED_FIELD_BYTES x2 (jack host unit test) | REAL | same host re-export (pub use snapshot_decode::*) |
| trinity | test | E0425 schema::mutations::dispatch_trinity_graph_mutations (jack host unit test) | REAL | now schema::operations::dispatch_trinity_graph_mutations |
| trinity | test | E0624/E0616 JackQueryWork::new/identity/progress/replay_target (jack run-query job unit test hoisted into facade) | REAL | new/identity and the two counters made pub (hoisted test lives in another crate) |
| trinity | test | E0433 semio_s_artifact_stdio_contract::SemioMemberStoreProducer (rewriting child-frame test) | REAL | producer lives in stdio semio base: ::semio_s_artifact_stdio_semio_base::SemioMemberStoreProducer |
| trinity | test | E0433 crate::standards (graph-query tests) x2 | REAL | use crate::flat_position / crate::topology (the crate has no standards tree) |
| trinity | both | E0432/E0425 TrinityJackLabels, TrinityRewritingLabels, RewritingAnalyzer/Builder/Composer, JackAnalyzer/Composer, PublicApps, RewritingArtifactApps, register_*_mutation_descriptors | ARTIFACT | macro-generated (app_labels!, artifact io declaration, plugin enum macro, derive); oracle cannot see |
| trinity | both | E0433 `crate` in paths can only be used in start position; `..crate::` in struct-update; E0432 crate::store | ARTIFACT | oracle qualifier rewriter on nested use lists / range+struct update / extern crate alias |
| trinity | both | E0433 semio_s_artifact_trinity_jack_graph_query in this scope (jack play-app) | ARTIFACT | dependency declared in the editor Cargo.toml; union extern mapping of a standalone workspace crate |
| mathematical | both | E0624 Expr::from_kind_unchecked x9 (equation cas-internals pattern) | REAL | pub (callers are in a sibling crate after the carve) |
| mathematical | test | E0603 reconstruct_pack_record_controlled (equation sqlite snapshot test) | REAL | pub fn |
| mathematical | test | E0425 decode_sqlite_snapshot_native (equation sqlite snapshot test) | REAL | original_owners::with_decoding(/o/ EquationSnapshot::decode_sqlite_snapshot_native(..)) like the sibling calls |
| gis | test | E0425 decode_sqlite_snapshot_native (gismap sqlite snapshot test) | REAL | same fix as equation |
| writer | test | same malformed <X as ArtifactSqliteSnapshotHost>::original_owners::with_decoding(/o/ decode_sqlite_snapshot_native(..)) line 107 | REAL | NOT EDITED: writer r7c running; handed off |
| puzzle | test | same malformed pattern x4 in puzzle 2d sqlite snapshot test (lines 83 89 124 128) | REAL | NOT EDITED: puzzle 2d r8a running; handed off |
| mathematical | both | E0624..., EquationCommand/EquationNodesDelta/EquationEdgesDelta/EquationGraphWindowConfigDiff unresolved; E0433 `polynomial` in `..crate::` | ARTIFACT | macro-generated deltas/commands; `(0..crate::polynomial::` rewriter miss |
| reasoning | test | E0433 snapshot::pack (wires sqlite semantic test, 10 rows) | REAL | wires binary snapshot: pub mod pack; pack::{record, record_controlled, reconstruct_record} pub (test lives in another crate) |
| forms | test | E0603 pack private (forms sqlite snapshot test x2) | REAL | forms binary snapshot: pub mod pack; pack::{record, reconstruct_record_controlled} pub |
| forms | test | E0425 demo_mutation_cases inaccessible (cfg(test) pub(crate) used from tests-3) | REAL | canonical testkit: cfg(any(test, feature="testkit")) pub fn; testkit feature in forms-mutations; tests-3 dev-dependency features=["testkit"] |
| forms | test | E0425 `control` (sqlite preflight law) | REAL | `let mut control=SqliteSnapshotControl::new(&mut callback,limits)` bound before the call that the assertion reads |
| forms | both | E0425 from_snapshot/addressed/register/current (transient_root!/window_transient_owners!) | ARTIFACT | macro generated |
| animate | test | E0616/E0624 PresentationPackSnapshotAuthority::new + field value (host owned law in another crate) | REAL | testkit-gated constructor with_live_value; testkit feature in presentation-schema; law crate dev-dependency features=["testkit"] |
| animate | both | E0432 geometry (extern crate `geometry` vs sibling module name) | ARTIFACT | declared dependency `geometry` |
| animate/architect/equation/note/shooting/flow/gis/sourcing | both | E0433 cannot find module or crate <sibling family crate> (undeclared dependency) | REAL (layering cycle) | NOT FIXED: adding the manifest edge would create a normal-dependency cycle (cyc.py); the `tests`/`examples`/`diff` component crate is a normal dependency of the crate it re-exports from. Owner: crate-rollout slice of the family (r8b animate/note/shooting/forms, r9b architect, r8c equation, r7c sourcing, r11/r3 flow, r6 gis) |
| space | test | E0425 create_artifact_at (operations unit test relied on `use super::*`) | REAL | explicit import added to the test |
| space | test | E0624 advance on Work (7 rows x27) | ARTIFACT | retained work trait method (ArtifactCommandWork) not visible to the stub |
| imperative | test | E0616 ImperativeHost.path (x7) | REAL | tests call the existing pub accessor host.path() |
| flow | test | E0616/E0624 FlowChildGroupWork, FlowContributionsWork, FlowInstanceOperationOwner::new, retained::Retirement methods (companion tests) | REAL | fields/methods widened to pub (widen.py) |
| flow | lib | E0425 map_kernel_error, read_channel_number, list_indices, read_text, read_xyz x587 | ARTIFACT | provided by external glob `use semio_s_spatial_kernel_semio_session::*` (helpers module) |
| lowpoly | both | E0603 LOWPOLY_PLAY_CONTROLLER_ID, catalogue panel definition/render; E0425 LOWPOLY_PLAY_BODY_*, reset_document_effect, lowpoly_window_engagement*, lowpoly_window_measures | REAL | pub; 8 explicit imports (autoimport.py) in play-app |
| process | both | E0603 PROCESS3D_MUTATION_BINARY_FORMAT, process3d_protocol_error x4 | REAL | pub |
| process | both | E0425 kit constants/functions (PROCESS3D_INTERACTION_DOMAIN, PROCESS3D_EXAMPLE_*, installable_contributions, reset_process3d_document_effect, process3d_action_fault, PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES) x22 | REAL | explicit kit imports in the editor root and in the children that name them |
| process | both | E0425 ContributedMachineCatalog / ProcessMachinesTopicPayload in kit; E0616 catalog_id | REAL | kit imports them from the model; trait accessor catalog_id() instead of the private field |
| draw | test | host retained-mutation laws: 7 private host-arena items (DrawingAssetBoundsCursor, DrawingMutationRetirementFactory, DRAWING_MUTATION_ARENA_POOL{,_CAPACITY}, DrawingMutationArenaPoolBootstrap, DrawingMutationArenaBootstrapAdmission, DrawingMutationArenaProcessState) and unimported DrawingCloneWorkTotals/PoolAvailability | REAL | pub + imports in the law root |
| draw | test | DrawingOwnedRetirement, next_initialization_catalog_close_byte_demand, exact_native_layer_backing (removed API) | REAL | laws re-authored: DrawingDecodedFieldRetirement::{try_new,demands,next_grant,step}, initialization_catalog_close_demands/close_initialization_catalog(grant), native_layer_backing(&exact_source.layers); unit-struct aliases built with ::default() |
| draw | lib | E0433 undeclared framework crates (job-registry, ui-locale, 2d, ui-contract, semio-framework in the facade; diagnostic in diff and model) | REAL | manifest dependencies added; job-registry workspace entry added |
| mathematical/gis/cad/fem/remodel/wfc/puzzle3d/block3d/space/stdio* | both | cross-crate private items and lost imports | REAL | autowiden.py/autoimport.py/autotestkit.py edits; counts in section 4 |

## 2. Proof per family (resolution + privacy codes, non-stub)

| family | runs | cfg | before | after | gone | new | all wanted codes before / after |
|---|---|---|---|---|---|---|---|
| animate | r1 to r2 | lib | 85 | 85 | 0 | 0 | 112 / 112 |
| animate | r1 to r2 | test | 133 | 131 | 2 | 0 | 297 / 295 |
| architect | r1 only | lib | 78 | - | - | - | 122 / - |
| architect | r1 only | test | 121 | - | - | - | 408 / - |
| block2d | r1 to r3 | lib | 61 | 38 | 23 | 0 | 69 / 46 |
| block3d | r1 to r3 | lib | 148 | 101 | 47 | 0 | 172 / 126 |
| block3d | r1 to r3 | test | 8 | 1 | 7 | 0 | 128 / 121 |
| block5d | r1 to r3 | lib | 84 | 43 | 41 | 0 | 86 / 45 |
| cad | r1 to r3 | lib | 246 | 190 | 56 | 0 | 267 / 212 |
| cad | r1 to r3 | test | 254 | 196 | 58 | 0 | 360 / 303 |
| dag | r1 only | lib | 11 | - | - | - | 31 / - |
| dag | r1 only | test | 17 | - | - | - | 76 / - |
| demonstrator | r1 only | lib | 27 | - | - | - | 24 / - |
| demonstrator | r1 only | test | 36 | - | - | - | 80 / - |
| dev-misc | r1 only | lib | 0 | - | - | - | 0 / - |
| dev-misc | r1 only | test | 0 | - | - | - | 0 / - |
| draw | d0 to r4 | lib | 200 | 132 | 68 | 0 | 294 / 244 |
| draw | d0 to r4 | test | 164 | 104 | 63 | 3 | 752 / 727 |
| energy | r1 only | lib | 3 | - | - | - | 49 / - |
| energy | r1 only | test | 4 | - | - | - | 94 / - |
| fem | r1 to r2 | lib | 162 | 148 | 14 | 0 | 231 / 217 |
| fem | r1 to r2 | test | 124 | 110 | 14 | 0 | 269 / 255 |
| flow | r1 to r2 | lib | 601 | 601 | 0 | 0 | 637 / 637 |
| flow | r1 to r2 | test | 697 | 676 | 23 | 2 | 744 / 721 |
| forms | r1 to r2 | lib | 32 | 32 | 0 | 0 | 61 / 61 |
| forms | r1 to r2 | test | 45 | 41 | 4 | 0 | 200 / 196 |
| gis | r1 to r2 | lib | 85 | 85 | 0 | 0 | 166 / 166 |
| gis | r1 to r2 | test | 5 | 5 | 0 | 0 | 338 / 338 |
| imperative | r1 to r2 | lib | 16 | 16 | 0 | 0 | 31 / 31 |
| imperative | r1 to r2 | test | 34 | 27 | 7 | 0 | 106 / 99 |
| layout | r1 to r3 | lib | 182 | 120 | 62 | 0 | 231 / 168 |
| lowpoly | r1 to r2 | lib | 278 | 264 | 14 | 0 | 298 / 284 |
| lowpoly | r1 to r2 | test | 284 | 270 | 14 | 0 | 319 / 305 |
| mathematical | r1 to r2 | lib | 34 | 25 | 9 | 0 | 86 / 77 |
| mathematical | r1 to r2 | test | 45 | 34 | 11 | 0 | 245 / 235 |
| note | r1 only | lib | 11 | - | - | - | 33 / - |
| note | r1 only | test | 13 | - | - | - | 186 / - |
| playbook | r1 to r2 | lib | 30 | 29 | 1 | 0 | 54 / 53 |
| playbook | r1 to r2 | test | 44 | 43 | 1 | 0 | 156 / 155 |
| procedural | r1 to r3 | lib | 541 | 440 | 104 | 3 | 596 / 495 |
| procedural | r1 to r3 | test | 784 | 607 | 180 | 3 | 995 / 820 |
| process | r1 to r3 | lib | 161 | 133 | 45 | 17 | 197 / 173 |
| process | r1 to r3 | test | 170 | 190 | 48 | 68 | 232 / 246 |
| puzzle2d | r1 to r3 | lib | 4 | 443 | 4 | 443 | 4 / 531 |
| puzzle2d | r1 to r3 | test | 34 | 660 | 10 | 636 | 23 / 904 |
| puzzle3d | r1 to r4 | lib | 168 | 121 | 47 | 0 | 250 / 203 |
| puzzle3d | r1 to r4 | test | 285 | 165 | 120 | 0 | 371 / 255 |
| puzzle5d | r1 to r2 | lib | 435 | 400 | 35 | 0 | 557 / 522 |
| puzzle5d | r1 to r2 | test | 510 | 470 | 40 | 0 | 720 / 680 |
| raster | r1 only | lib | 88 | - | - | - | 116 / - |
| raster | r1 only | test | 299 | - | - | - | 535 / - |
| reasoning | r1 to r3 | lib | 15 | 15 | 0 | 0 | 37 / 37 |
| reasoning | r1 to r3 | test | 46 | 34 | 12 | 0 | 124 / 114 |
| remodel | r1 to r4 | lib | 229 | 226 | 3 | 0 | 288 / 285 |
| remodel | r1 to r4 | test | 668 | 562 | 106 | 0 | 732 / 626 |
| shooting | r1 only | lib | 81 | - | - | - | 96 / - |
| shooting | r1 only | test | 209 | - | - | - | 508 / - |
| sourcing | r1 only | lib | 27 | - | - | - | 57 / - |
| sourcing | r1 only | test | 0 | - | - | - | 0 / - |
| space | r1 to r2 | lib | 154 | 154 | 0 | 0 | 170 / 170 |
| space | r1 to r2 | test | 271 | 229 | 42 | 0 | 366 / 324 |
| stdio00 | r1 to r2 | lib | 80 | 39 | 41 | 0 | 127 / 86 |
| stdio00 | r1 to r2 | test | 127 | 46 | 81 | 0 | 279 / 204 |
| stdio01 | r1 to r2 | lib | 34 | 53 | 17 | 36 | 31 / 16 |
| stdio01 | r1 to r2 | test | 39 | 58 | 17 | 36 | 79 / 64 |
| stdio02 | r1 to r2 | lib | 38 | 18 | 20 | 0 | 36 / 16 |
| stdio02 | r1 to r2 | test | 48 | 27 | 21 | 0 | 154 / 135 |
| stdio03 | r1 only | lib | 42 | - | - | - | 39 / - |
| stdio03 | r1 only | test | 59 | - | - | - | 162 / - |
| stdio04 | r1 to r2 | lib | 54 | 54 | 0 | 0 | 55 / 55 |
| stdio04 | r1 to r2 | test | 63 | 54 | 9 | 0 | 176 / 167 |
| stdio05 | r1 only | lib | 42 | - | - | - | 42 / - |
| stdio05 | r1 only | test | 45 | - | - | - | 109 / - |
| stdio06 | r1 to r2 | lib | 321 | 321 | 0 | 0 | 299 / 299 |
| stdio06 | r1 to r2 | test | 342 | 340 | 2 | 0 | 384 / 382 |
| stdio07 | r1 only | lib | 31 | - | - | - | 28 / - |
| stdio07 | r1 only | test | 35 | - | - | - | 90 / - |
| stdio08 | r1 to r2 | lib | 263 | 186 | 77 | 0 | 231 / 154 |
| stdio08 | r1 to r2 | test | 269 | 192 | 77 | 0 | 292 / 215 |
| stdio09 | r1 to r2 | lib | 105 | 33 | 72 | 0 | 106 / 34 |
| stdio09 | r1 to r2 | test | 107 | 33 | 74 | 0 | 197 / 123 |
| stdio10 | r1 to r2 | lib | 15 | 15 | 0 | 0 | 8 / 8 |
| stdio10 | r1 to r2 | test | 19 | 16 | 3 | 0 | 70 / 71 |
| stdio11 | r1 to r2 | lib | 29 | 15 | 14 | 0 | 22 / 8 |
| stdio11 | r1 to r2 | test | 29 | 15 | 14 | 0 | 125 / 111 |
| stdio12 | r1 to r2 | lib | 323 | 237 | 86 | 0 | 265 / 179 |
| stdio12 | r1 to r2 | test | 443 | 347 | 96 | 0 | 443 / 347 |
| stdio13 | r1 to r2 | lib | 31 | 15 | 16 | 0 | 28 / 12 |
| stdio13 | r1 to r2 | test | 32 | 15 | 17 | 0 | 134 / 119 |
| stdio14 | r1 to r2 | lib | 117 | 74 | 43 | 0 | 121 / 78 |
| stdio14 | r1 to r2 | test | 118 | 75 | 43 | 0 | 151 / 108 |
| stdio16 | r1 to r3 | lib | 173 | 74 | 99 | 0 | 166 / 67 |
| stdio16 | r1 to r3 | test | 196 | 97 | 99 | 0 | 213 / 114 |
| stdio17 | r1 to r3 | lib | 57 | 19 | 38 | 0 | 58 / 20 |
| stdio17 | r1 to r3 | test | 68 | 19 | 49 | 0 | 164 / 119 |
| stdio18 | r1 to r2 | lib | 67 | 44 | 23 | 0 | 60 / 37 |
| stdio18 | r1 to r2 | test | 96 | 60 | 36 | 0 | 152 / 116 |
| stdio19 | r1 to r2 | lib | 36 | 29 | 7 | 0 | 37 / 30 |
| stdio19 | r1 to r2 | test | 42 | 29 | 13 | 0 | 130 / 117 |
| stdio20 | r1 to r2 | lib | 87 | 87 | 0 | 0 | 77 / 77 |
| stdio20 | r1 to r2 | test | 117 | 109 | 8 | 0 | 248 / 240 |
| stdio21 | r1 only | lib | 38 | - | - | - | 36 / - |
| stdio21 | r1 only | test | 43 | - | - | - | 84 / - |
| stdio22 | r1 to r2 | lib | 100 | 90 | 18 | 8 | 73 / 63 |
| stdio22 | r1 to r2 | test | 101 | 90 | 19 | 8 | 100 / 89 |
| stdio23 | r1 to r2 | lib | 64 | 64 | 0 | 0 | 72 / 72 |
| stdio23 | r1 to r2 | test | 79 | 78 | 1 | 0 | 174 / 175 |
| stdio24 | r1 only | lib | 23 | - | - | - | 22 / - |
| stdio24 | r1 only | test | 23 | - | - | - | 139 / - |
| stdio25 | r1 to r2 | lib | 36 | 36 | 0 | 0 | 36 / 36 |
| stdio25 | r1 to r2 | test | 45 | 41 | 4 | 0 | 162 / 158 |
| stdio26 | r1 to r2 | lib | 1653 | 94 | 1559 | 0 | 1667 / 108 |
| stdio26 | r1 to r2 | test | 1945 | 199 | 1746 | 0 | 1875 / 271 |
| stdio27 | r1 only | lib | 71 | - | - | - | 59 / - |
| stdio27 | r1 only | test | 77 | - | - | - | 149 / - |
| stdio28 | r1 only | lib | 102 | - | - | - | 90 / - |
| stdio28 | r1 only | test | 138 | - | - | - | 224 / - |
| stdio29 | r1 to r2 | lib | 19 | 19 | 0 | 0 | 13 / 13 |
| stdio29 | r1 to r2 | test | 45 | 29 | 16 | 0 | 138 / 122 |
| stdio30 | r1 to r2 | lib | 36 | 36 | 0 | 0 | 35 / 35 |
| stdio30 | r1 to r2 | test | 55 | 43 | 12 | 0 | 190 / 178 |
| stdio31 | r1 only | lib | 279 | - | - | - | 266 / - |
| stdio31 | r1 only | test | 288 | - | - | - | 369 / - |
| stdio32 | r1 to r3 | lib | 87 | 37 | 50 | 0 | 83 / 33 |
| stdio32 | r1 to r3 | test | 106 | 54 | 52 | 0 | 133 / 81 |
| stdio33 | r1 to r2 | lib | 33 | 13 | 20 | 0 | 55 / 35 |
| stdio33 | r1 to r2 | test | 33 | 13 | 20 | 0 | 190 / 170 |
| stdio34 | r1 only | lib | 894 | - | - | - | 1098 / - |
| stdio34 | r1 only | test | 1014 | - | - | - | 2565 / - |
| stdio35 | r1 only | lib | 28 | - | - | - | 24 / - |
| stdio35 | r1 only | test | 28 | - | - | - | 90 / - |
| trinity | r0 to r2 | lib | 39 | 38 | 1 | 0 | 95 / 93 |
| trinity | r0 to r2 | test | 127 | 59 | 69 | 1 | 396 / 318 |
| trinity-dev | r1 only | lib | 0 | - | - | - | 5 / - |
| trinity-dev | r1 only | test | 82 | - | - | - | 173 / - |
| vcs | r1 only | lib | 63 | - | - | - | 88 / - |
| vcs | r1 only | test | 108 | - | - | - | 272 / - |
| wfc | r1 to r2 | lib | 104 | 104 | 0 | 0 | 152 / 152 |
| wfc | r1 to r2 | test | 276 | 270 | 6 | 0 | 397 / 391 |
