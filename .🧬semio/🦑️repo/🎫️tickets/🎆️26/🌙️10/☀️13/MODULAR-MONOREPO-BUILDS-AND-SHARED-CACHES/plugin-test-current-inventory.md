# Current Plugin Test Item Inventory

| Type | Name | Start | End | Bytes | Assertions |
|---|---|---:|---:|---:|---:|
| line_comment | //! 🧪️ The plugin contract's own unit test: a `TestApp | 3 | 3 | 96 | 0 |
| line_comment | //! surface (B1), wrapped in `VcsArtifactApp`, exercisi | 4 | 4 | 101 | 0 |
| line_comment | //! operations that emit no document operations, histor | 5 | 5 | 98 | 0 |
| line_comment | //! idempotency. `TestCommand` is `TestApp`'s typed `Se | 6 | 6 | 93 | 0 |
| line_comment | //! (history/clipboard/revert/filter/noteShellCommand)  | 7 | 7 | 100 | 0 |
| line_comment | //! `handle_command` — everything app-specific dispatch | 8 | 8 | 81 | 0 |
| use_declaration | use crate::__semio_dispatch_PluginApp; | 9 | 9 | 38 | 0 |
| use_declaration | use crate::interaction_topology::INLINE_INTERACTION_VER | 10 | 10 | 73 | 0 |
| use_declaration | use crate::typed_operation::TypedCommandFullOperationJo | 11 | 11 | 64 | 0 |
| use_declaration | use crate::tool_jobs::{test_artifact_owned_tool_job_con | 12 | 12 | 156 | 0 |
| use_declaration | use crate::artifact_instance::{artifact_handle_of,MAINT | 13 | 13 | 99 | 0 |
| use_declaration | use crate::typed_operation::test_retained_document_canc | 14 | 14 | 64 | 0 |
| use_declaration | use crate::typed_operation::test_retained_latest_wins_s | 15 | 15 | 84 | 0 |
| use_declaration | use crate::typed_operation::test_retained_cancellation_ | 16 | 16 | 78 | 0 |
| use_declaration | use crate::typed_operation::test_typed_operation_lands_ | 17 | 17 | 90 | 0 |
| use_declaration | use crate::typed_operation::test_typed_operation_comple | 18 | 18 | 89 | 0 |
| use_declaration | use crate::typed_operation::test_typed_operation_never_ | 19 | 19 | 98 | 0 |
| use_declaration | use crate::typed_operation::test_typed_operation_slot_r | 20 | 20 | 79 | 0 |
| use_declaration | use crate::typed_operation::test_typed_operation_slot_r | 21 | 21 | 77 | 0 |
| use_declaration | use crate::typed_operation::test_typed_operation_slot_p | 22 | 22 | 67 | 0 |
| use_declaration | use crate::typed_operation::test_retained_keyed_dispatc | 23 | 23 | 57 | 0 |
| use_declaration | use semio_framework_value::DslValue; | 24 | 24 | 36 | 0 |
| use_declaration | use semio_framework_value_derive::{FromValue, ToValue}; | 25 | 25 | 55 | 0 |
| use_declaration | use semio_framework_ui_locale::Locale; | 26 | 26 | 38 | 0 |
| use_declaration | use semio_framework_ui_locale::LocalizedLabel; | 27 | 27 | 46 | 0 |
| use_declaration | use semio_framework_ui_locale::Terminology; | 28 | 28 | 43 | 0 |
| line_comment | /// 🌐️ One locale cell of a history row's label carrie | 30 | 30 | 100 | 0 |
| line_comment | /// it reads, so a row that carries only English cannot | 31 | 31 | 91 | 0 |
| line_comment | /// locale axis is asserted by `a_shell_noted_row_carri | 32 | 32 | 89 | 0 |
| function_item | test_child_emit | 33 | 37 | 331 | 0 |
| line_comment | /// 📨️ Funds the actual original child preparation quo | 39 | 39 | 109 | 0 |
| function_item | prepare_original_fixture_child | 40 | 45 | 677 | 1 |
| function_item | label_in | 46 | 48 | 118 | 0 |
| line_comment | /// 🌉️ Test-only convenience: builds a `serde_json::js | 50 | 50 | 95 | 0 |
| line_comment | /// `DslValue` `handle_action`/`dispatch_action`/`comma | 51 | 51 | 89 | 0 |
| line_comment | /// boundary — the fixtures below stay readable as JSON | 52 | 52 | 84 | 0 |
| line_comment | /// `serde_json::Value` alive past this one conversion. | 53 | 53 | 55 | 0 |
| function_item | dv | 54 | 56 | 93 | 0 |
| line_comment | /// 🎟️ Validates a physical demand within the caller-a | 58 | 58 | 89 | 0 |
| function_item | fixture_close_grant | 59 | 63 | 514 | 1 |
| line_comment | /// 🎟️ Checks the next physical demand against the fix | 65 | 65 | 88 | 0 |
| function_item | job_close_grant | 66 | 71 | 921 | 1 |
| line_comment | /// 🧹️ Drives one interactive job to its terminal-empt | 73 | 73 | 136 | 0 |
| function_item | drive_job_close | 74 | 81 | 621 | 2 |
| line_comment | /// 🎟️ Spends exactly the quoted close turn of a hoste | 83 | 83 | 65 | 0 |
| function_item | app_close_turn | 84 | 92 | 1080 | 2 |
| line_comment | /// 🎟️ Spends exactly the quoted maintenance turn of a | 94 | 94 | 71 | 0 |
| function_item | app_maintenance_turn | 95 | 102 | 1074 | 2 |
| line_comment | /// ✍️ Authors one `Apply` of document mutations under  | 104 | 104 | 104 | 0 |
| function_item | apply_test_mutations | 105 | 107 | 467 | 0 |
| use_declaration | use super::ContextMenuWireRequest; | 109 | 109 | 34 | 0 |
| use_declaration | use crate::app_emission::{ActionMeta, AsyncTask, ChildE | 110 | 110 | 91 | 0 |
| use_declaration | use crate::app_builder::App; | 111 | 111 | 28 | 0 |
| use_declaration | use crate::app_contract::{ArtifactApp, EphemeralSnapsho | 112 | 112 | 58 | 0 |
| use_declaration | use crate::plugin_app::PluginApp; | 113 | 113 | 33 | 0 |
| use_declaration | use crate::window_kits::AppActionRegistry; | 114 | 114 | 42 | 0 |
| use_declaration | use crate::media_contract::ArtifactDownloadOutput; | 115 | 115 | 50 | 0 |
| use_declaration | use crate::media_contract::ArtifactMediaExportHandle; | 116 | 116 | 53 | 0 |
| use_declaration | use crate::media_contract::ArtifactOutputChunks; | 117 | 117 | 48 | 0 |
| use_declaration | use crate::app_views::{ArtifactView, ConfigView, DraftV | 118 | 118 | 158 | 0 |
| use_declaration | use crate::command_history::ui_history_panel; | 119 | 119 | 45 | 0 |
| use_declaration | use crate::history_view::CommandView; | 120 | 120 | 37 | 0 |
| use_declaration | use crate::history_view::HistoryCommandFilter; | 121 | 121 | 46 | 0 |
| use_declaration | use crate::history_view::HistoryView; | 122 | 122 | 37 | 0 |
| use_declaration | use crate::window_kits::Menu; | 123 | 123 | 29 | 0 |
| use_declaration | use crate::app_views::PeerPresence; | 124 | 124 | 35 | 0 |
| use_declaration | use crate::artifact_instance::VcsArtifactApp; | 125 | 125 | 45 | 0 |
| use_declaration | use crate::artifact_io::{ArtifactDeserializer, Artifact | 126 | 126 | 111 | 0 |
| use_declaration | use semio_framework_artifact_reference::{Dialect, Stand | 127 | 127 | 72 | 0 |
| use_declaration | use semio_framework_os_kernel::io::{ErasedComposeSource | 128 | 128 | 68 | 0 |
| use_declaration | use crate::publication_fixture::{ChangePublicationPrese | 129 | 129 | 190 | 0 |
| use_declaration | use semio_framework_diagnostic::FaultFrom; | 130 | 130 | 42 | 0 |
| use_declaration | use crate::{IconName, MediaClass, MediaType, ViewModel, | 131 | 131 | 80 | 0 |
| use_declaration | use protocol::Mutation; | 132 | 132 | 23 | 0 |
| use_declaration | use semio_framework::Fault; | 133 | 133 | 27 | 0 |
| use_declaration | use semio_framework::kernel::ArtifactHandle; | 134 | 134 | 44 | 0 |
| use_declaration | use semio_framework::kernel::{AppEvent, ClipboardError, | 135 | 135 | 126 | 0 |
| use_declaration | use semio_framework::{ActionArgDef, ActionDefinition, A | 136 | 136 | 210 | 0 |
| use_declaration | use semio_framework_job::InteractiveJob as _; | 137 | 137 | 45 | 0 |
| mod_item | local_interaction_dispatch | 138 | 140 | 190 | 0 |
| line_comment | /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): this  | 141 | 141 | 88 | 0 |
| line_comment | /// explicitly (no `use super::*;`), so the `🕹️IntentD | 142 | 142 | 94 | 0 |
| line_comment | /// import too, rather than relying on `mod app`'s oute | 143 | 143 | 62 | 0 |
| use_declaration | use crate::tree_kit::TREE_WINDOW_DEFAULT_ROWS; | 144 | 144 | 46 | 0 |
| use_declaration | use semio_framework::TreeWindowRequest; | 145 | 145 | 39 | 0 |
| use_declaration | use semio_framework_ui_contract::{ActionId, BuiltNode,  | 146 | 146 | 162 | 0 |
| use_declaration | use serde::{Deserialize, Serialize}; | 147 | 147 | 36 | 0 |
| use_declaration | use serde_json::json; | 148 | 148 | 21 | 0 |
| use_declaration | use std::collections::BTreeMap; | 149 | 149 | 31 | 0 |
| use_declaration | use {semio_framework_artifact_reference::ArtifactRef}; | 150 | 150 | 54 | 0 |
| use_declaration | use semio_framework_2d::compute::EngineHandles; | 151 | 151 | 47 | 0 |
| use_declaration | use pack::snapshot::ArtifactPack; | 152 | 152 | 33 | 0 |
| use_declaration | use store::{os_store::backbone::Backbone, os_store::bac | 153 | 153 | 115 | 0 |
| use_declaration | use store::{MemberFactory, SpaceMember}; | 154 | 154 | 40 | 0 |
| use_declaration | use semio_framework_ui_layout::layout::FRAMEWORK_HISTOR | 155 | 155 | 66 | 0 |
| use_declaration | use semio_framework_ui_layout::layout::{ContextMenuItem | 156 | 156 | 92 | 0 |
| line_comment | /// 🪪️ `TestApp`'s one dialect coordinate (contract §1 | 158 | 158 | 102 | 0 |
| line_comment | /// (`TestApp::APP_ID`, the `App::builder` ids below, e | 159 | 159 | 93 | 0 |
| line_comment | /// `ClipboardFragment.source_app` literal) derives fro | 160 | 160 | 88 | 0 |
| line_comment | /// pre-migration `"synthetic-play"` string, so `test_a | 161 | 161 | 95 | 0 |
| line_comment | /// never independently drift (guarded by `test_app_id_ | 162 | 162 | 87 | 0 |
| const_item | TEST_APP_DIALECT | 163 | 163 | 130 | 0 |
| line_comment | /// 🪟️ The one window body key `interaction_registry`' | 165 | 165 | 109 | 0 |
| line_comment | /// `TestApp::interaction_scope` names, so the fixture  | 166 | 166 | 95 | 0 |
| const_item | TEST_APP_WINDOW_BODY_KEY | 167 | 167 | 56 | 0 |
| macro_invocation | std::thread_local! {         static RENDER_CONTEXT_PROB | 169 | 171 | 154 | 0 |
| line_comment | /// 🏷️ The recorded digest is an oracle for ONE digest | 173 | 173 | 92 | 0 |
| line_comment | /// (`ARC-CONTEXT-3` → `-4` …) exactly when the identit | 174 | 174 | 95 | 0 |
| line_comment | /// invalidates the literal — so the tag is joined here | 175 | 175 | 95 | 0 |
| line_comment | /// that names its cause instead of on a bare number. | 176 | 176 | 53 | 0 |
| attribute_item | #[test] | 177 | 177 | 7 | 0 |
| function_item | app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift | 178 | 202 | 2949 | 10 |
| line_comment | /// 🪪️ Live `surface_app_id` call over `TEST_APP_DIALE | 204 | 204 | 104 | 0 |
| line_comment | /// in this module passes, per contract §1 (`AppBuilder | 205 | 205 | 103 | 0 |
| function_item | test_app_surface_id | 206 | 208 | 114 | 0 |
| line_comment | //#region 🧬️TestDocumentMutationFixture | 210 | 210 | 44 | 0 |
| use_declaration | use crate::test_app_mutation_fixture::TestSnapshot; | 211 | 211 | 51 | 0 |
| mod_item | child_emission_refusal | 212 | 212 | 146 | 0 |
| use_declaration | use crate::test_app_mutation_fixture::document::{MAXIMU | 213 | 213 | 123 | 0 |
| line_comment | /// 🧪️ Trivial dummy `ArtifactSerializer`/`ArtifactDes | 215 | 215 | 90 | 0 |
| line_comment | /// `TestSnapshot` to itself, for `serializer_entry_of` | 216 | 216 | 92 | 0 |
| struct_item | DummySerializer | 217 | 217 | 23 | 0 |
| impl_item | impl ArtifactSerializer for DummySerializer {         t | 218 | 226 | 500 | 0 |
| struct_item | DummyDeserializer | 228 | 228 | 25 | 0 |
| impl_item | impl ArtifactDeserializer for DummyDeserializer {       | 229 | 237 | 506 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 239 | 239 | 43 | 0 |
| function_item | serializer_entry_of_and_deserializer_entry_of_erase_correctly | 240 | 268 | 2063 | 8 |
| line_comment | //#region 🧬️TestDocumentMutationLeaves | 270 | 270 | 43 | 0 |
| use_declaration | use crate::test_app_mutation_fixture::{SetSlotChildren, | 271 | 271 | 90 | 0 |
| line_comment | //#endregion 🧬️TestDocumentMutationLeaves | 272 | 272 | 46 | 0 |
| line_comment | //#endregion 🧬️TestDocumentMutationFixture | 274 | 274 | 47 | 0 |
| use_declaration | use crate::test_app_mutation_fixture::{ChangeTestConfig | 276 | 276 | 98 | 0 |
| line_comment | /// 🧪️ B1: `TestApp`'s typed command enum — the sole d | 278 | 278 | 98 | 0 |
| attribute_item | #[derive(Clone, Debug, PartialEq, Serialize, ToValue, D | 279 | 279 | 122 | 0 |
| attribute_item | #[derive(semio_framework_value::RetireOwned)] | 280 | 280 | 45 | 0 |
| enum_item | TestCommand | 281 | 346 | 3603 | 0 |
| impl_item | impl ::protocol::OpText for TestCommand {         fn pa | 348 | 368 | 1754 | 0 |
| impl_item | impl ::protocol::OpBinary for TestCommand {         con | 370 | 379 | 427 | 0 |
| macro_invocation | thread_local! {         /// 📨️ Every host event `TestA | 381 | 384 | 237 | 0 |
| line_comment | /// 📨️ Takes the host events recorded on this test thr | 386 | 386 | 63 | 0 |
| function_item | take_test_host_events | 387 | 389 | 134 | 0 |
| line_comment | /// 🪟️ A view of the two-window roster `main-1`/`main- | 391 | 391 | 97 | 0 |
| function_item | host_event_view | 392 | 396 | 548 | 0 |
| attribute_item | #[derive(Debug,semio_framework_value::RetireOwned)] | 398 | 398 | 51 | 0 |
| struct_item | SpawnCountTask | 399 | 399 | 22 | 0 |
| impl_item | impl crate::app_emission::AsyncTaskSource<TestMutation, | 400 | 409 | 781 | 0 |
| line_comment | /// 🧪️ Records the actual typed commands received by t | 410 | 410 | 77 | 0 |
| attribute_item | #[derive(Default)] | 411 | 411 | 18 | 0 |
| struct_item | TestApp | 412 | 414 | 150 | 0 |
| macro_invocation | include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪 | 416 | 416 | 96 | 0 |
| empty_statement | ; | 416 | 416 | 1 | 0 |
| line_comment | //#region 🧪️TestClipboardReservedJob | 418 | 418 | 41 | 0 |
| line_comment | //#region 📋️ContractClipboardFixture | 419 | 419 | 41 | 0 |
| line_comment | /// 📋️ The exact reserved clipboard producer of the co | 420 | 420 | 238 | 0 |
| struct_item | TestClipboardJob | 421 | 432 | 593 | 0 |
| impl_item | impl<const RETAINED: bool, const TOOLS: u8> TestClipboa | 434 | 477 | 3578 | 0 |
| impl_item | impl<const RETAINED: bool, const TOOLS: u8> semio_frame | 479 | 528 | 4513 | 0 |
| impl_item | impl<const RETAINED: bool, const TOOLS: u8> crate::tool | 530 | 530 | 126 | 0 |
| line_comment | //#endregion 📋️ContractClipboardFixture | 531 | 531 | 44 | 0 |
| line_comment | //#region 🧵️RetainedCommandReplayFixture | 533 | 533 | 45 | 0 |
| const_item | TEST_RETAINED_COMMAND_CONTROLLER | 534 | 534 | 77 | 0 |
| const_item | TEST_RETAINED_COMMAND_TOOL | 535 | 535 | 52 | 0 |
| const_item | TEST_RETAINED_COMMAND_SCHEMA | 536 | 536 | 76 | 0 |
| const_item | TEST_RETAINED_COMMAND_RAW_BYTES | 537 | 537 | 53 | 0 |
| static_item | TEST_RETAINED_COMMAND_STEP_CALLS | 538 | 538 | 113 | 0 |
| struct_item | TestRetainedCommandWork | 540 | 542 | 58 | 0 |
| struct_item | TestRetainedChildCommandWork | 544 | 546 | 122 | 0 |
| impl_item | impl crate::retained_command::ArtifactCommandWork<TestA | 548 | 571 | 2164 | 4 |
| impl_item | impl crate::retained_command::ArtifactCommandWork<TestA | 573 | 604 | 1942 | 0 |
| struct_item | TestRetainedCommandFactory | 606 | 608 | 76 | 0 |
| impl_item | impl TestRetainedCommandFactory {         fn new() -> S | 610 | 614 | 191 | 0 |
| struct_item | OtherTestRetainedCommandFactory | 616 | 616 | 67 | 0 |
| impl_item | impl ToolJobFactory for OtherTestRetainedCommandFactory | 618 | 636 | 895 | 0 |
| impl_item | impl ArtifactOwnedToolJobFactory for OtherTestRetainedC | 638 | 643 | 401 | 0 |
| impl_item | impl ArtifactOwnedToolJobFactory for TestRetainedComman | 645 | 650 | 470 | 0 |
| impl_item | impl ToolJobFactory for TestRetainedCommandFactory {    | 652 | 691 | 2049 | 0 |
| function_item | test_retained_command_id | 693 | 698 | 212 | 0 |
| const_item | TEST_COMMAND_RETAINED_GRANT | 700 | 700 | 259 | 0 |
| function_item | test_retained_command_payload | 702 | 720 | 1260 | 0 |
| function_item | test_retained_child_command_payload | 722 | 740 | 1341 | 0 |
| function_item | test_retained_wire_input | 742 | 749 | 670 | 0 |
| line_comment | //#endregion 🧵️RetainedCommandReplayFixture | 751 | 751 | 48 | 0 |
| line_comment | //#endregion 🧪️TestClipboardReservedJob | 753 | 753 | 44 | 0 |
| attribute_item | #[derive(semio_framework_value::FactoryPayloadRetiremen | 755 | 755 | 58 | 0 |
| struct_item | PublicationPresenceRetirementFactory | 756 | 756 | 44 | 0 |
| impl_item | impl store::SnapshotRetirementFactory<PublicationPresen | 758 | 763 | 718 | 0 |
| line_comment | /// 🌊️ The one tool transaction every `streamLabel` ti | 766 | 766 | 124 | 0 |
| const_item | TEST_STREAM_LABEL_TRANSACTION | 767 | 767 | 66 | 0 |
| line_comment | /// 🪪️ `TestApp`'s verb for one command variant, as a  | 769 | 769 | 85 | 0 |
| line_comment | /// command payload (`ArtifactRetainedCommandPayload::c | 770 | 770 | 80 | 0 |
| line_comment | /// `ArtifactApp::command_id` answer the same id from O | 771 | 771 | 64 | 0 |
| line_comment | /// | 772 | 772 | 3 | 0 |
| line_comment | /// 🔀️ Rides the keyed fixture's ONE generated tool id | 773 | 773 | 96 | 0 |
| line_comment | /// a pick and a composite edit are two commands of the | 774 | 774 | 72 | 0 |
| function_item | test_command_id | 775 | 799 | 1384 | 0 |
| line_comment | /// 🧮️ `TestApp`'s reducer body, shared verbatim by `A | 801 | 801 | 95 | 0 |
| line_comment | /// `test_app_command_reduce` (the app-owned retained t | 802 | 802 | 88 | 0 |
| line_comment | /// and a direct one can never diverge. | 803 | 803 | 39 | 0 |
| line_comment | /// | 804 | 804 | 3 | 0 |
| line_comment | /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: no mut | 805 | 805 | 84 | 0 |
| line_comment | /// task's eventual `TaskResolution::Command` follow-up | 806 | 806 | 75 | 0 |
| line_comment | /// document, on a LATER dispatch (see `ApplyCountFromT | 807 | 807 | 67 | 0 |
| function_item | test_app_reduce | 808 | 849 | 4141 | 0 |
| function_item | test_prepared_operation_wire_source | 851 | 859 | 719 | 0 |
| function_item | test_document_preparation_factory | 861 | 866 | 493 | 0 |
| impl_item | impl<const RETAINED: bool, const TOOLS: u8> ArtifactApp | 868 | 1141 | 18185 | 0 |
| attribute_item | #[test] | 1143 | 1143 | 7 | 0 |
| function_item | declared_child_projection_default_preserves_empty_and_full_literal_references | 1144 | 1166 | 2346 | 9 |
| line_comment | //#region 🗝️RegisteredKeyedDispatchFixture | 1168 | 1168 | 47 | 0 |
| attribute_item | #[derive(Default)] | 1169 | 1169 | 18 | 0 |
| struct_item | KeyedTestApp | 1170 | 1170 | 20 | 0 |
| line_comment | /// 🪟️ The keyed fixture's one window body — the carri | 1172 | 1172 | 101 | 0 |
| const_item | KEYED_WINDOW_BODY_KEY | 1173 | 1173 | 53 | 0 |
| line_comment | /// 🔀️ Exactly what `canvas_gesture::pointer_up` (fem2 | 1175 | 1175 | 85 | 0 |
| line_comment | /// `interaction_select_effect` (`Effect::ReplayShellCo | 1176 | 1176 | 95 | 0 |
| line_comment | /// window refresh, and NO store lane — the emit whose  | 1177 | 1177 | 99 | 0 |
| function_item | keyed_pick_emit | 1178 | 1184 | 339 | 0 |
| function_item | keyed_pick_carrier_scope | 1186 | 1188 | 247 | 0 |
| line_comment | /// 🗝️ `keyed_test_registry` plus the declared `items` | 1190 | 1190 | 98 | 0 |
| line_comment | /// names — the registry the typed-ladder fold laws dis | 1191 | 1191 | 92 | 0 |
| line_comment | /// fixture's one generated tool (`compositeEdit`, see  | 1192 | 1192 | 103 | 0 |
| function_item | keyed_pick_test_registry | 1193 | 1226 | 1840 | 0 |
| struct_item | KeyedTestCommandDisposer | 1228 | 1232 | 164 | 0 |
| impl_item | impl KeyedTestCommandDisposer {         fn new() -> Sel | 1234 | 1236 | 117 | 0 |
| impl_item | impl ArtifactOwnedDisposer<TestCommand> for KeyedTestCo | 1238 | 1261 | 2910 | 0 |
| struct_item | KeyedTestJobSources | 1263 | 1277 | 837 | 0 |
| impl_item | impl KeyedTestJobSources {         /// 🧹️ The next ori | 1279 | 1299 | 2522 | 0 |
| struct_item | KeyedTestJob | 1301 | 1312 | 496 | 0 |
| impl_item | impl KeyedTestJob {         fn close_demands(&self, bod | 1314 | 1324 | 1258 | 0 |
| impl_item | impl semio_framework_job::InteractiveJob for KeyedTestJ | 1326 | 1402 | 6257 | 1 |
| struct_item | KeyedTestFactory | 1404 | 1406 | 66 | 0 |
| impl_item | impl ToolJobFactory for KeyedTestFactory {         type | 1408 | 1437 | 1295 | 1 |
| impl_item | impl ArtifactOwnedToolJobFactory for KeyedTestFactory { | 1439 | 1453 | 879 | 0 |
| line_comment | //#region 🧹️KeyedNoStateFixtureOwners | 1455 | 1455 | 42 | 0 |
| line_comment | //#endregion 🧹️KeyedNoStateFixtureOwners | 1456 | 1456 | 45 | 0 |
| impl_item | impl ArtifactApp for KeyedTestApp {         const DIALE | 1458 | 1534 | 5545 | 1 |
| attribute_item | #[test] | 1536 | 1536 | 7 | 0 |
| function_item | keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners | 1537 | 1637 | 8591 | 23 |
| function_item | keyed_test_registry | 1639 | 1655 | 822 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 1657 | 1657 | 43 | 0 |
| function_item | retained_latest_wins_registered_dispatch_rebases_worker_and_publishes_real_document | 1658 | 1677 | 1638 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 1679 | 1679 | 43 | 0 |
| function_item | typed_operation_ingress_pre_admits_the_exact_slot_before_it_mints_an_operation_id | 1680 | 1690 | 1474 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 1692 | 1692 | 43 | 0 |
| function_item | every_admitted_typed_operation_slot_is_released_by_the_host_continuation_that_reports_it_runnable | 1693 | 1703 | 1500 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 1705 | 1705 | 43 | 0 |
| function_item | a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot | 1706 | 1716 | 1486 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 1718 | 1718 | 43 | 0 |
| function_item | a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work | 1719 | 1729 | 1498 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 1731 | 1731 | 43 | 0 |
| function_item | a_status_only_host_call_finishes_every_typed_operation_it_admitted | 1732 | 1742 | 1481 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 1744 | 1744 | 43 | 0 |
| function_item | an_admitting_host_call_hands_back_the_terminal_lane_it_earned | 1745 | 1755 | 1477 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 1757 | 1757 | 43 | 0 |
| function_item | microsecond_registered_factory_dispatch_preserves_exact_half_ms_fake_clock | 1758 | 1780 | 1706 | 0 |
| line_comment | /// ⏱️ Continuation turns a staged operation may spend  | 1782 | 1782 | 105 | 0 |
| line_comment | /// the terminating turn on which the scan finally answ | 1783 | 1783 | 78 | 0 |
| const_item | TYPED_OPERATION_CONTINUATION_SLACK | 1784 | 1784 | 50 | 0 |
| line_comment | /// ⛽️ The per-turn ceiling a browser turn actually car | 1786 | 1786 | 105 | 0 |
| line_comment | /// `🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️ | 1787 | 1787 | 131 | 0 |
| line_comment | /// drives the continuation reads the exact grant produ | 1788 | 1788 | 91 | 0 |
| function_item | reactor_native_budget | 1789 | 1791 | 299 | 0 |
| line_comment | /// 📄️ LAW: k live operations each holding a presentab | 1793 | 1793 | 102 | 0 |
| line_comment | /// | 1794 | 1794 | 3 | 0 |
| line_comment | /// 🐛️ `advance_typed_operation_output` took exactly o | 1795 | 1795 | 105 | 0 |
| line_comment | /// the turn the instant it existed, so every acknowled | 1796 | 1796 | 92 | 0 |
| line_comment | /// operation cost its own worker round trip. Served me | 1797 | 1797 | 105 | 0 |
| line_comment | /// (`📓️shard-message-crossings-2026-09-15.md` §2): 20 | 1798 | 1798 | 95 | 0 |
| line_comment | /// operations, **every one of them exactly one ack wid | 1799 | 1799 | 95 | 0 |
| line_comment | /// crossings. A page's ACK is per-page by protocol; th | 1800 | 1800 | 103 | 0 |
| line_comment | /// `Event::Message` is a list and the reactor's poll a | 1801 | 1801 | 85 | 0 |
| line_comment | /// | 1802 | 1802 | 3 | 0 |
| line_comment | /// The law is about CARDINALITY, so it admits whatever | 1803 | 1803 | 97 | 0 |
| line_comment | /// edits of one immutable document root make three of  | 1804 | 1804 | 93 | 0 |
| line_comment | /// published as its own acknowledgeable `Fault` page o | 1805 | 1805 | 92 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 1806 | 1806 | 43 | 0 |
| function_item | concurrent_typed_operations_hand_every_presentable_page_to_one_turn | 1807 | 1868 | 5213 | 5 |
| line_comment | /// 🧮️ A many-mutation edit publishes every one of its | 1870 | 1870 | 100 | 0 |
| line_comment | /// spending many publication units under the stall gua | 1871 | 1871 | 94 | 0 |
| line_comment | /// | 1872 | 1872 | 3 | 0 |
| line_comment | /// 🩺️ The guard (`fault_stalled_typed_operation_publi | 1873 | 1873 | 93 | 0 |
| line_comment | /// operation once 4 096 consecutive units leave its st | 1874 | 1874 | 93 | 0 |
| line_comment | /// had no term for the ladder's own progress. This law | 1875 | 1875 | 86 | 0 |
| line_comment | /// publication the store admits — `ARTIFACT_STORE_ONE_ | 1876 | 1876 | 93 | 0 |
| line_comment | /// unit spends up to `TYPED_OPERATION_RESULT_PAGE_BYTE | 1877 | 1877 | 96 | 0 |
| line_comment | /// on the fold being COMPLETE and fault-free rather th | 1878 | 1878 | 91 | 0 |
| line_comment | /// document publication cannot reach 4 096 units at al | 1879 | 1879 | 96 | 0 |
| line_comment | /// 150 000 are refused as "batched preparation footpri | 1880 | 1880 | 88 | 0 |
| line_comment | /// capacity"). | 1881 | 1881 | 15 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 1882 | 1882 | 43 | 0 |
| function_item | a_many_mutation_publication_folds_every_mutation_and_is_never_terminated_as_stalled | 1883 | 1939 | 4858 | 6 |
| attribute_item | #[semio_framework_async_macros::async_test] | 1941 | 1941 | 43 | 0 |
| function_item | a_host_crossing_outranks_every_background_hold_on_the_same_app | 1942 | 1992 | 4272 | 7 |
| line_comment | /// ⚖️ LAW: a staged operation costs the host ONE turn  | 1994 | 1994 | 99 | 0 |
| line_comment | /// per publication unit. The host has to come back for | 1995 | 1995 | 94 | 0 |
| line_comment | /// released by its own ACK; every other unit is intern | 1996 | 1996 | 96 | 0 |
| line_comment | /// after each of them cost this fixture 48 turns befor | 1997 | 1997 | 82 | 0 |
| line_comment | /// 🧮️ A turn the operation was not even schedulable o | 1998 | 1998 | 100 | 0 |
| line_comment | /// shared worker pool busy with somebody else — is the | 1999 | 1999 | 99 | 0 |
| line_comment | /// and is not counted: on a loaded machine it drew one | 2000 | 2000 | 92 | 0 |
| line_comment | /// (measured 2026-09-21, trace `[(false, true, true),  | 2001 | 2001 | 94 | 0 |
| line_comment | /// runnable turn still carried its exact receipt. The  | 2002 | 2002 | 85 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 2003 | 2003 | 43 | 0 |
| function_item | retained_operation_continues_after_command_admission_until_publication_and_retirement | 2004 | 2092 | 6731 | 9 |
| attribute_item | #[semio_framework_async_macros::async_test] | 2094 | 2094 | 43 | 0 |
| function_item | retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires | 2095 | 2244 | 13717 | 24 |
| line_comment | //#endregion 🗝️RegisteredKeyedDispatchFixture | 2245 | 2245 | 50 | 0 |
| use_declaration | use crate::plugin_app_close_prelude::*; | 2247 | 2247 | 39 | 0 |
| macro_invocation | semio_framework_dispatch_macros::dyn_enum_close! {      | 2248 | 2252 | 151 | 0 |
| line_comment | /// 🧬️ A hostile structural copy of Draw's public cont | 2254 | 2254 | 92 | 0 |
| line_comment | /// concrete Rust type is deliberately not the audited  | 2255 | 2255 | 71 | 0 |
| attribute_item | #[derive(Default)] | 2256 | 2256 | 18 | 0 |
| struct_item | CopyDrawApp | 2257 | 2257 | 19 | 0 |
| impl_item | impl ArtifactApp for CopyDrawApp {         const DIALEC | 2259 | 2297 | 1719 | 0 |
| line_comment | // 🚫️async: E1 pure constructor, called pervasively as | 2299 | 2299 | 81 | 0 |
| function_item | meta | 2300 | 2302 | 110 | 0 |
| function_item | settle_reserved | 2304 | 2306 | 266 | 0 |
| function_item | reserved_action | 2308 | 2311 | 410 | 0 |
| line_comment | /// 🎡️ The maintenance rotation is fair: one call runs | 2313 | 2313 | 104 | 0 |
| line_comment | /// and reports a blocked stage only once nothing else  | 2314 | 2314 | 97 | 0 |
| line_comment | /// EVENTUAL answer and never the answer of the one cal | 2315 | 2315 | 95 | 0 |
| line_comment | /// authority that was LOST instead of blocked never re | 2316 | 2316 | 94 | 0 |
| line_comment | /// weakens nothing — it is the same assertion, made wh | 2317 | 2317 | 97 | 0 |
| function_item | assert_maintenance_reports_block | 2318 | 2328 | 670 | 0 |
| line_comment | /// 🧹️ Drains every maintenance stage, then closes. A  | 2330 | 2330 | 103 | 0 |
| line_comment | /// child member or an app-typed presence peer under ex | 2331 | 2331 | 99 | 0 |
| line_comment | /// store's terminal-empty Drop witness by `close_step` | 2332 | 2332 | 96 | 0 |
| line_comment | /// `maintenance_step` pumps, one bounded unit per stag | 2333 | 2333 | 66 | 0 |
| function_item | drain_and_close_fixture | 2334 | 2343 | 940 | 0 |
| line_comment | /// 🧹️ [`drain_and_close_fixture`] for a composed fixt | 2345 | 2345 | 81 | 0 |
| function_item | drain_and_close_composed_fixture | 2346 | 2355 | 962 | 0 |
| function_item | close_reserved_app | 2357 | 2359 | 277 | 0 |
| function_item | synthetic_play_app | 2361 | 2370 | 553 | 0 |
| line_comment | /// 🧪️ A registry-backed app declaring the contract-en | 2372 | 2372 | 103 | 0 |
| line_comment | /// the context-menu label lookup, a declared-but-empty | 2373 | 2373 | 100 | 0 |
| line_comment | /// utility (which auto-injects the `setActiveUtility`  | 2374 | 2374 | 93 | 0 |
| line_comment | /// required/default-arg materialization tests were del | 2375 | 2375 | 101 | 0 |
| line_comment | /// (`AppActionRegistry`/`materialize_args`) and has no | 2376 | 2376 | 99 | 0 |
| line_comment | /// Rust caller constructs directly (a "missing require | 2377 | 2377 | 97 | 0 |
| line_comment | /// one). `setLabelRequired` stays declared here purely | 2378 | 2378 | 98 | 0 |
| line_comment | /// label-resolution test below. These synthetic reduce | 2379 | 2379 | 93 | 0 |
| line_comment | /// only an exact authority test may promote its own se | 2380 | 2380 | 78 | 0 |
| line_comment | /// | 2381 | 2381 | 3 | 0 |
| line_comment | /// 🧪️ `Mutation`-kind by declaration, but `TestApp` e | 2382 | 2382 | 94 | 0 |
| line_comment | /// "declared Mutation action that happened to produce  | 2383 | 2383 | 72 | 0 |
| function_item | contract_registry | 2384 | 2404 | 1747 | 0 |
| line_comment | /// 🪪️ Every verb `TestApp::command_id` can answer, de | 2406 | 2406 | 101 | 0 |
| line_comment | /// emits — `View` for the config/scope-only verbs, `Sh | 2407 | 2407 | 97 | 0 |
| line_comment | /// for the document ones. `admit_command_wire_with_pro | 2408 | 2408 | 92 | 0 |
| line_comment | /// typed dispatch, so a verb missing from this roster  | 2409 | 2409 | 93 | 0 |
| line_comment | /// verb declared non-`Migrated` is `interactive-job.no | 2410 | 2410 | 66 | 0 |
| line_comment | /// 🪪️ Every verb `TestApp::command_id` can answer, de | 2411 | 2411 | 101 | 0 |
| line_comment | /// emits — `View` for the config/scope-only verbs, `Sh | 2412 | 2412 | 97 | 0 |
| line_comment | /// for the document ones. `admit_command_wire_with_pro | 2413 | 2413 | 92 | 0 |
| line_comment | /// typed dispatch, so a verb missing from this roster  | 2414 | 2414 | 93 | 0 |
| line_comment | /// verb declared non-`Migrated` is `interactive-job.no | 2415 | 2415 | 66 | 0 |
| line_comment | /// | 2416 | 2416 | 3 | 0 |
| line_comment | /// 🧵️ Synchronous on purpose: `AppBuilder`'s chain mo | 2417 | 2417 | 94 | 0 |
| line_comment | /// `.await`, and an unoptimized build gives each await | 2418 | 2418 | 96 | 0 |
| line_comment | /// slot — twelve more of them overflowed the 2 MiB bou | 2419 | 2419 | 79 | 0 |
| line_comment | /// `one_framework_reserved_route_fits_a_bounded_thread | 2420 | 2420 | 92 | 0 |
| line_comment | /// statement drops each temporary at its own semicolon | 2421 | 2421 | 64 | 0 |
| function_item | declare_test_app_verbs | 2422 | 2435 | 1674 | 0 |
| line_comment | /// 🧰️ The `Migrated` twin of [`contract_registry`]. ` | 2437 | 2437 | 94 | 0 |
| line_comment | /// `BatchOnlyPendingRewrite`: it is the only reason th | 2438 | 2438 | 79 | 0 |
| line_comment | /// (`unproved_command_fails_before_an_overrun_reducer_ | 2439 | 2439 | 66 | 0 |
| line_comment | /// `activated_tool_factory_keys_are_an_exact_bijection | 2440 | 2440 | 84 | 0 |
| line_comment | /// `ui_dispatch_backstop_rejects_every_non_migrated_ac | 2441 | 2441 | 98 | 0 |
| line_comment | /// all. Every behavioural law dispatches against THIS  | 2442 | 2442 | 67 | 0 |
| function_item | migrated_contract_registry | 2443 | 2473 | 1860 | 0 |
| line_comment | /// 🧪️ A mounted `TestApp` fixture whose caller explic | 2475 | 2475 | 133 | 0 |
| line_comment | /// instead of in every one of the 70 call sites that u | 2476 | 2476 | 94 | 0 |
| line_comment | /// registry-backed (so a typed verb has a manifest dec | 2477 | 2477 | 90 | 0 |
| line_comment | /// instance (`dispatch_typed_command_inner` refuses `i | 2478 | 2478 | 96 | 0 |
| line_comment | /// and it retires its stores before `ArtifactStore::dr | 2479 | 2479 | 95 | 0 |
| struct_item | ContractApp | 2480 | 2480 | 73 | 0 |
| impl_item | impl std::ops::Deref for ContractApp {         type Tar | 2482 | 2487 | 164 | 0 |
| impl_item | impl std::ops::DerefMut for ContractApp {         fn de | 2489 | 2493 | 136 | 0 |
| impl_item | impl Drop for ContractApp {         fn drop(&mut self)  | 2495 | 2499 | 229 | 1 |
| impl_item | impl ContractApp {         /// 🔁️ Shadows `VcsArtifact | 2501 | 2566 | 5311 | 0 |
| line_comment | /// 🎯️ The intent twin of {@link ContractApp::dispatch | 2568 | 2568 | 106 | 0 |
| line_comment | /// same admission receipt every migrated dispatch answ | 2569 | 2569 | 100 | 0 |
| line_comment | /// after it observed the previous revision. Settles th | 2570 | 2570 | 91 | 0 |
| line_comment | /// mutations/inverse through the SAME `result_from_las | 2571 | 2571 | 85 | 0 |
| function_item | handle_intent_settled | 2572 | 2592 | 1485 | 0 |
| line_comment | /// ⏳️ Drives the admission a manifest-addressed dispat | 2594 | 2594 | 102 | 0 |
| line_comment | /// {@link ContractApp::dispatch_typed} does for the di | 2595 | 2595 | 97 | 0 |
| line_comment | /// hands its reducer to a worker too, so a law that re | 2596 | 2596 | 97 | 0 |
| line_comment | /// pre-dispatch revision. | 2597 | 2597 | 26 | 0 |
| function_item | settle_contract_app | 2598 | 2600 | 457 | 0 |
| function_item | contract_app_under_test | 2602 | 2604 | 311 | 0 |
| line_comment | /// 🧪️ [`contract_app_under_test`] over an explicit re | 2606 | 2606 | 99 | 0 |
| line_comment | /// fixtures declare their own manifests but need the s | 2607 | 2607 | 109 | 0 |
| function_item | contract_app_with | 2608 | 2613 | 622 | 0 |
| line_comment | /// 🧪️ The fixture every law that used to build a bare | 2615 | 2615 | 100 | 0 |
| line_comment | /// uses now. A registry-less wrapper can no longer dis | 2616 | 2616 | 96 | 0 |
| line_comment | /// declaration for the verb), so "no registry" was nev | 2617 | 2617 | 87 | 0 |
| function_item | contract_app | 2618 | 2620 | 300 | 0 |
| line_comment | /// 🧪️ [`contract_app`] without the self-closing wrapp | 2622 | 2622 | 103 | 0 |
| line_comment | /// `PluginRuntime` cell or a `dyn_enum_close!` arm, or | 2623 | 2623 | 91 | 0 |
| line_comment | /// generic surface (a generic bound never applies a `D | 2624 | 2624 | 96 | 0 |
| line_comment | /// themselves, exactly as they did before. | 2625 | 2625 | 43 | 0 |
| function_item | contract_app_raw | 2626 | 2631 | 601 | 0 |
| line_comment | /// 🧩️ [`contract_app_raw`] over the composed member r | 2633 | 2633 | 65 | 0 |
| function_item | contract_composed_app_raw | 2634 | 2639 | 647 | 0 |
| line_comment | /// 🚫️ The UNPROVED twin: `contract_registry`'s `Batch | 2641 | 2641 | 102 | 0 |
| line_comment | /// app that owns no tool factory at all, for the laws  | 2642 | 2642 | 90 | 0 |
| line_comment | /// 🚫️ A `TestApp` that registers no owned factory at  | 2643 | 2643 | 101 | 0 |
| line_comment | /// `BatchOnlyPendingRewrite`. The migrated registry is | 2644 | 2644 | 101 | 0 |
| line_comment | /// verbs and supplying no proof row cannot be CONSTRUC | 2645 | 2645 | 105 | 0 |
| line_comment | /// which is the same fail-closed join from the other s | 2646 | 2646 | 59 | 0 |
| function_item | unproved_contract_app | 2647 | 2649 | 454 | 0 |
| line_comment | /// 🧩️ [`ContractApp`] over the composed member roster | 2651 | 2651 | 96 | 0 |
| struct_item | ContractComposedApp | 2652 | 2652 | 94 | 0 |
| impl_item | impl std::ops::Deref for ContractComposedApp {          | 2654 | 2659 | 185 | 0 |
| impl_item | impl std::ops::DerefMut for ContractComposedApp {       | 2661 | 2665 | 144 | 0 |
| impl_item | impl Drop for ContractComposedApp {         fn drop(&mu | 2667 | 2671 | 237 | 1 |
| impl_item | impl ContractComposedApp {         /// 🧩️ A COMPOSED g | 2673 | 2743 | 5362 | 0 |
| function_item | contract_composed_app | 2745 | 2750 | 663 | 0 |
| line_comment | /// 🧵️ Source text of the mounted artifact instance: i | 2752 | 2752 | 212 | 0 |
| function_item | instance_source | 2753 | 2755 | 240 | 0 |
| line_comment | /// 🧭️ The bijection has exactly two named residues, a | 2757 | 2757 | 98 | 0 |
| line_comment | /// drift in either direction fails on the clause that  | 2758 | 2758 | 71 | 0 |
| line_comment | /// | 2759 | 2759 | 3 | 0 |
| line_comment | /// 1. Framework-reserved SURFACE verbs the framework r | 2760 | 2760 | 93 | 0 |
| line_comment | ///    (`register_framework_reserved_tool_factories`) w | 2761 | 2761 | 92 | 0 |
| line_comment | ///    none of them — it has no interaction topology an | 2762 | 2762 | 93 | 0 |
| line_comment | ///    here is registered but undeclared, so `require_u | 2763 | 2763 | 93 | 0 |
| line_comment | ///    before its factory is ever reached. | 2764 | 2764 | 42 | 0 |
| line_comment | /// 2. Migrated verbs the framework serves without a to | 2765 | 2765 | 92 | 0 |
| line_comment | ///    is not the dead-action defect it would be for an | 2766 | 2766 | 96 | 0 |
| line_comment | ///    (injected whenever the app declares utilities; ` | 2767 | 2767 | 90 | 0 |
| line_comment | ///    DIRECTLY to `dispatch_emit`) and `cancelTypedOpe | 2768 | 2768 | 94 | 0 |
| line_comment | ///    cancel; the head of `dispatch_action` routes it  | 2769 | 2769 | 102 | 0 |
| line_comment | /// | 2770 | 2770 | 3 | 0 |
| line_comment | /// 🚦️ `not-ui-safe`, not `missing-factory`: `contract | 2771 | 2771 | 89 | 0 |
| line_comment | /// `BatchOnlyPendingRewrite`, and the UI-safety backst | 2772 | 2772 | 95 | 0 |
| line_comment | /// at the HEAD of `admit_command_wire`, before any pro | 2773 | 2773 | 94 | 0 |
| line_comment | /// no factory precisely BECAUSE it is not migrated, so | 2774 | 2774 | 94 | 0 |
| line_comment | /// not the cause — the clause above still proves there | 2775 | 2775 | 80 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 2776 | 2776 | 43 | 0 |
| function_item | activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations | 2777 | 2841 | 6753 | 13 |
| line_comment | //#region 🧪️SharedFrameworkActionRouteTests | 2843 | 2843 | 48 | 0 |
| line_comment | /// 🪪️ Every framework-reserved factory is parameteriz | 2844 | 2844 | 92 | 0 |
| line_comment | /// identity this law joins must name the exact owner t | 2845 | 2845 | 95 | 0 |
| line_comment | /// `TestApp` is `TestApp<false, TEST_APP_TOOLS_FULL>`, | 2846 | 2846 | 97 | 0 |
| line_comment | /// | 2847 | 2847 | 3 | 0 |
| line_comment | /// 📏️ The last column is `ToolExecutionContract::max_ | 2848 | 2848 | 101 | 0 |
| line_comment | /// this verb's own `framework_reserved_job!(..)` line  | 2849 | 2849 | 97 | 0 |
| line_comment | /// an 8 KiB raw wire and a 1 MiB OUTPUT; recording the | 2850 | 2850 | 93 | 0 |
| line_comment | /// 1 MiB wire admission that the bus has never granted | 2851 | 2851 | 61 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 2852 | 2852 | 43 | 0 |
| function_item | shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity | 2853 | 2913 | 6838 | 9 |
| attribute_item | #[test] | 2915 | 2915 | 7 | 0 |
| function_item | shared_framework_job_is_cancellable_resumable_and_boundedly_closeable | 2916 | 2949 | 3852 | 11 |
| line_comment | //#endregion 🧪️SharedFrameworkActionRouteTests | 2950 | 2950 | 51 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 2952 | 2952 | 43 | 0 |
| function_item | retained_factory_proof_requires_the_exact_registered_runtime_authority | 2953 | 2955 | 340 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 2957 | 2957 | 43 | 0 |
| function_item | retained_latest_wins_cancellation_guards_real_store_publication_and_preserves_committed_ack | 2958 | 2960 | 221 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 2962 | 2962 | 43 | 0 |
| function_item | retained_latest_wins_reserved_slots_and_ready_publisher_are_fair | 2963 | 2965 | 200 | 0 |
| attribute_item | #[test] | 2967 | 2967 | 7 | 0 |
| function_item | retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement | 2968 | 2970 | 154 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 2972 | 2972 | 43 | 0 |
| function_item | retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close | 2973 | 2975 | 348 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 2977 | 2977 | 43 | 0 |
| function_item | copied_app_type_cannot_inherit_same_controller_schema_and_id_proof | 2978 | 2990 | 1078 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 2992 | 2992 | 43 | 0 |
| function_item | tool_cancellation_isolated_by_live_instance_with_same_controller_and_document | 2993 | 3012 | 1289 | 5 |
| attribute_item | #[semio_framework_async_macros::async_test] | 3014 | 3014 | 43 | 0 |
| function_item | cancellation_supersession_and_saturated_app_close_are_parent_scope_constant_time | 3015 | 3057 | 2330 | 8 |
| attribute_item | #[test] | 3059 | 3059 | 7 | 0 |
| function_item | cancellation_numeric_authority_rejects_collision_capacity_and_contention_without_blocking | 3060 | 3080 | 1287 | 2 |
| attribute_item | #[test] | 3082 | 3082 | 7 | 0 |
| function_item | poisoned_cancellation_authority_fails_closed_without_recovery_or_waiting | 3083 | 3098 | 863 | 1 |
| attribute_item | #[test] | 3100 | 3100 | 7 | 0 |
| function_item | completed_media_result_uses_explicit_mime_instead_of_schema | 3101 | 3111 | 698 | 7 |
| attribute_item | #[semio_framework_async_macros::async_test] | 3113 | 3113 | 43 | 0 |
| function_item | segmented_download_remains_addressable_until_terminal_none_is_observed | 3114 | 3139 | 2659 | 8 |
| attribute_item | #[semio_framework_async_macros::async_test] | 3141 | 3141 | 43 | 0 |
| function_item | completed_media_cancel_moves_exact_output_to_bounded_cleanup | 3142 | 3188 | 4842 | 14 |
| attribute_item | #[semio_framework_async_macros::async_test] | 3190 | 3190 | 43 | 0 |
| function_item | app_maintenance_reclaims_late_envelope_field_returns_before_close_terminal | 3191 | 3285 | 6157 | 6 |
| line_comment | /// 🔢️ Swept over every declared stage instead of a fr | 3287 | 3287 | 99 | 0 |
| line_comment | /// completed-record lane is an implementation detail t | 3288 | 3288 | 97 | 0 |
| line_comment | /// the law — ordinary maintenance never retires an unc | 3289 | 3289 | 93 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 3290 | 3290 | 43 | 0 |
| function_item | app_maintenance_and_close_retain_completed_envelope_results_until_terminal_empty | 3291 | 3394 | 6849 | 8 |
| line_comment | /// 🪜️ Park the ladder exactly ON the segment stage wi | 3396 | 3396 | 99 | 0 |
| line_comment | /// is bookkeeping that releases no bytes, and the segm | 3397 | 3397 | 96 | 0 |
| line_comment | /// chunk (`maximum_bytes < ARTIFACT_OUTPUT_CHUNK_BYTES | 3398 | 3398 | 97 | 0 |
| line_comment | /// to that stage and then stands still there — which i | 3399 | 3399 | 92 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 3400 | 3400 | 43 | 0 |
| function_item | app_close_step_drains_at_most_one_segment_and_one_chunk_budget | 3401 | 3447 | 5118 | 14 |
| attribute_item | #[semio_framework_async_macros::async_test] | 3449 | 3449 | 43 | 0 |
| function_item | artifact_close_final_destructor_is_constant_after_every_owned_field_is_drained | 3450 | 3476 | 2477 | 6 |
| attribute_item | #[test] | 3478 | 3478 | 7 | 0 |
| function_item | retained_field_maximum_and_maximum_plus_one_are_language_neutral | 3479 | 3497 | 2059 | 8 |
| attribute_item | #[semio_framework_async_macros::async_test] | 3499 | 3499 | 43 | 0 |
| function_item | retained_command_download_seam_keeps_original_chunks_and_bounded_cancel_owner | 3500 | 3551 | 8830 | 17 |
| attribute_item | #[semio_framework_async_macros::async_test] | 3553 | 3553 | 43 | 0 |
| function_item | shared_retained_command_checkpoint_resumes_exact_cursor_and_cancels_with_bounded_close | 3554 | 3648 | 7538 | 9 |
| attribute_item | #[semio_framework_async_macros::async_test] | 3650 | 3650 | 43 | 0 |
| function_item | retained_command_child_emit_prepublication_close_and_rejected_handoff_are_bounded | 3651 | 3762 | 8465 | 16 |
| attribute_item | #[semio_framework_async_macros::async_test] | 3764 | 3764 | 43 | 0 |
| function_item | unproved_command_fails_before_an_overrun_reducer_can_start | 3765 | 3785 | 2426 | 3 |
| line_comment | /// 🧪️ A registry declaring one `HierarchyProvider::To | 3787 | 3787 | 99 | 0 |
| line_comment | /// `TestApp::interaction_topology`) — the `🕹️Interact | 3788 | 3788 | 103 | 0 |
| line_comment | /// framework interaction actions via `interaction_acti | 3789 | 3789 | 83 | 0 |
| line_comment | /// `build_definition_carries_window_interactions_and_i | 3790 | 3790 | 88 | 0 |
| function_item | interaction_registry | 3791 | 3793 | 140 | 0 |
| line_comment | /// 🕹️ The synthetic app the interaction laws dispatch | 3795 | 3795 | 98 | 0 |
| line_comment | /// domain, and the `window_kind_interactions` ref that | 3796 | 3796 | 102 | 0 |
| line_comment | /// `interaction_declared_refresh_scope` derives a verb | 3797 | 3797 | 77 | 0 |
| function_item | interaction_app_definition | 3798 | 3825 | 1645 | 0 |
| function_item | interaction_app_under_test | 3827 | 3829 | 308 | 0 |
| line_comment | /// 🕹️ [`interaction_app_under_test`] without the self | 3831 | 3831 | 105 | 0 |
| line_comment | /// laws that move the app into a `PluginRuntime` cell. | 3832 | 3832 | 55 | 0 |
| function_item | interaction_app_raw | 3833 | 3838 | 598 | 0 |
| line_comment | /// 🧪️ Builds the JSON `args` an `interactionSelect`/` | 3840 | 3840 | 101 | 0 |
| line_comment | /// `targets` arg is itself a JSON-encoded string (an ` | 3841 | 3841 | 84 | 0 |
| line_comment | /// `interaction_action_definitions`'s declaration), no | 3842 | 3842 | 77 | 0 |
| function_item | interaction_target_args | 3843 | 3851 | 550 | 0 |
| function_item | interaction_empty_target_args | 3853 | 3860 | 405 | 0 |
| line_comment | /// 🧪️ Same wire shape as {@link interaction_target_ar | 3862 | 3862 | 101 | 0 |
| line_comment | /// marquee release (or a pick that resolved several re | 3863 | 3863 | 81 | 0 |
| function_item | interaction_targets_args | 3864 | 3872 | 617 | 0 |
| macro_invocation | include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../⚛️ | 3874 | 3874 | 124 | 0 |
| empty_statement | ; | 3874 | 3874 | 1 | 0 |
| macro_invocation | include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../⚛️ | 3876 | 3876 | 124 | 0 |
| empty_statement | ; | 3876 | 3876 | 1 | 0 |
| line_comment | // 📄️ What ONE command costs the guest's command-ingre | 3878 | 3878 | 97 | 0 |
| line_comment | // authority does instead of trapping. | 3879 | 3879 | 38 | 0 |
| line_comment | // | 3880 | 3880 | 2 | 0 |
| line_comment | // Build #29 of ticket 26/09/02/PUZZLE-3D-END-TO-END ra | 3881 | 3881 | 94 | 0 |
| line_comment | // (~44 s at 4 Hz) and then answered every single one w | 3882 | 3882 | 58 | 0 |
| line_comment | // `plugin.command-page-allocation: fixed command page  | 3883 | 3883 | 95 | 0 |
| line_comment | // slots`, thirty-two times in a row, before the guest  | 3884 | 3884 | 94 | 0 |
| line_comment | // `CommandPageSet::try_new`'s `try_reserve_exact` refu | 3885 | 3885 | 94 | 0 |
| line_comment | // saturation refusal (`plugin.command-page-count`) — s | 3886 | 3886 | 102 | 0 |
| line_comment | // could no longer hand out the block the prologue aske | 3887 | 3887 | 61 | 0 |
| line_comment | // | 3888 | 3888 | 2 | 0 |
| line_comment | // The block was 262 272 B: `COMMAND_MAXIMUM_PAGES * si | 3889 | 3889 | 97 | 0 |
| line_comment | // EVERY command regardless of how many pages that comm | 3890 | 3890 | 93 | 0 |
| line_comment | // commands that are one page of a few hundred bytes. T | 3891 | 3891 | 100 | 0 |
| line_comment | // units and never shrinks it, so a quarter-megabyte co | 3892 | 3892 | 92 | 0 |
| line_comment | // fragmented or nearly-full guest refuses — which is e | 3893 | 3893 | 95 | 0 |
| line_comment | // | 3894 | 3894 | 2 | 0 |
| line_comment | // The laws below pin both halves: the reservation is t | 3895 | 3895 | 98 | 0 |
| line_comment | // refusal on this path is a `Fault` the host can displ | 3896 | 3896 | 78 | 0 |
| line_comment | /// 🔁️ Commands driven through the real ingress — over | 3898 | 3898 | 100 | 0 |
| line_comment | /// prologue started refusing, so a per-command owner t | 3899 | 3899 | 99 | 0 |
| const_item | INGRESS_COMMANDS | 3900 | 3900 | 34 | 0 |
| line_comment | /// 🧮️ Retained bytes one command may leave behind. Th | 3902 | 3902 | 104 | 0 |
| line_comment | /// pages are released as the command is assembled and  | 3903 | 3903 | 87 | 0 |
| const_item | INGRESS_COMMAND_RETENTION_CEILING_BYTES | 3904 | 3904 | 59 | 0 |
| line_comment | /// ⏱️ Turns one command may take to reach its terminal | 3906 | 3906 | 105 | 0 |
| const_item | INGRESS_TURNS_PER_COMMAND | 3907 | 3907 | 45 | 0 |
| function_item | command_page_authority_budget | 3909 | 3911 | 230 | 0 |
| line_comment | /// 📤️ Encodes one `AppCommand` into the exact page an | 3913 | 3913 | 108 | 0 |
| line_comment | /// the header come from `encode_app_command` and `Comm | 3914 | 3914 | 99 | 0 |
| line_comment | /// hand-written wire literal. | 3915 | 3915 | 30 | 0 |
| function_item | command_page_authority_page | 3916 | 3925 | 1114 | 1 |
| line_comment | /// 🚚️ Drives one command to its terminal ingress stat | 3927 | 3927 | 107 | 0 |
| line_comment | /// first turn, then plain turns while the guest steps  | 3928 | 3928 | 94 | 0 |
| line_comment | /// `CommandComplete` or `Fault`. Returns whether it co | 3929 | 3929 | 85 | 0 |
| function_item | drive_one_command | 3930 | 3943 | 1059 | 0 |
| line_comment | /// 🧱️ Page counts the page-set law drives. The host s | 3945 | 3945 | 103 | 0 |
| line_comment | /// 8-page command is the shape a `registerBrushMesh`-s | 3946 | 3946 | 93 | 0 |
| line_comment | /// 272 089-char scoped contributions pack the procedur | 3947 | 3947 | 94 | 0 |
| line_comment | /// fixed 64-page ceiling refused outright (`command in | 3948 | 3948 | 98 | 0 |
| line_comment | /// by a margin no constant explains. | 3949 | 3949 | 37 | 0 |
| const_item | INGRESS_PAGE_SETS | 3950 | 3950 | 59 | 0 |
| line_comment | /// ⏱️ Turns a page set may spend beyond one per page.  | 3952 | 3952 | 103 | 0 |
| line_comment | /// turn, so `pages` is the protocol's own floor; the s | 3953 | 3953 | 94 | 0 |
| line_comment | /// contention retry (`plugin_exchange` hands the owner | 3954 | 3954 | 90 | 0 |
| const_item | INGRESS_TURN_SLACK | 3955 | 3955 | 36 | 0 |
| line_comment | /// 📤️ The production host owner for one multi-page co | 3957 | 3957 | 107 | 0 |
| line_comment | /// per command, which hands out its next page only onc | 3958 | 3958 | 94 | 0 |
| function_item | command_page_authority_driver | 3959 | 3965 | 780 | 0 |
| line_comment | /// 🚚️ Drives a whole page set to its terminal ingress | 3967 | 3967 | 108 | 0 |
| line_comment | /// next page (the wire carries at most one per turn),  | 3968 | 3968 | 102 | 0 |
| line_comment | /// whether it completed, the turns it spent and the pa | 3969 | 3969 | 78 | 0 |
| function_item | drive_one_page_set | 3970 | 3989 | 1686 | 0 |
| line_comment | /// ⚖️ LAW: a command's page set reaches its terminal i | 3991 | 3991 | 109 | 0 |
| line_comment | /// the wire's own page count, never the number of inte | 3992 | 3992 | 90 | 0 |
| line_comment | /// | 3993 | 3993 | 3 | 0 |
| line_comment | /// 🐢️ Before ticket 26/09/02 wave B24 `plugin_exchang | 3994 | 3994 | 100 | 0 |
| line_comment | /// `PluginCommandIngress::step` — `Encoded → Decoding` | 3995 | 3995 | 105 | 0 |
| line_comment | /// `Decoded → Ready` — and each `Pending` costs the ho | 3996 | 3996 | 97 | 0 |
| line_comment | /// `CommandText` measured 4.006 turns over 320 command | 3997 | 3997 | 103 | 0 |
| line_comment | /// cost was `pages + k` with `k` set by the command's  | 3998 | 3998 | 101 | 0 |
| line_comment | /// fails for every page count under that pacing, which | 3999 | 3999 | 88 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4000 | 4000 | 43 | 0 |
| function_item | a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page | 4001 | 4028 | 2665 | 3 |
| line_comment | /// ⚖️ LAW: the prologue reserves EXACTLY the pages its | 4030 | 4030 | 104 | 0 |
| line_comment | /// inside one guest-allocator growth unit. | 4031 | 4031 | 43 | 0 |
| line_comment | /// | 4032 | 4032 | 3 | 0 |
| line_comment | /// 🧊️ The measurement is the heap witness's own peak  | 4033 | 4033 | 104 | 0 |
| line_comment | /// the block the allocator was actually asked for. Bef | 4034 | 4034 | 100 | 0 |
| line_comment | /// — four times the ceiling the guest can be relied on | 4035 | 4035 | 104 | 0 |
| line_comment | /// | 4036 | 4036 | 3 | 0 |
| line_comment | /// 🧵️ This thread's own allocator scope — a process-w | 4037 | 4037 | 99 | 0 |
| line_comment | /// reads every concurrent law's allocations and frees  | 4038 | 4038 | 93 | 0 |
| line_comment | /// the reading drift below the block that was really r | 4039 | 4039 | 94 | 0 |
| attribute_item | #[test] | 4040 | 4040 | 7 | 0 |
| function_item | a_command_page_authority_reserves_only_the_pages_its_command_declares | 4041 | 4069 | 2288 | 6 |
| line_comment | /// ⚖️ LAW: a page authority refuses every over-declara | 4071 | 4071 | 102 | 0 |
| line_comment | /// host can display, and never panics. Exhaustion on t | 4072 | 4072 | 100 | 0 |
| line_comment | /// not as a guest that stops answering. | 4073 | 4073 | 40 | 0 |
| attribute_item | #[test] | 4074 | 4074 | 7 | 0 |
| function_item | a_saturated_command_page_authority_refuses_without_panicking | 4075 | 4105 | 2651 | 11 |
| line_comment | /// 📥️ The cross-language paged command-ingress contra | 4107 | 4107 | 106 | 0 |
| line_comment | /// and the TypeScript host writer's twin read. | 4108 | 4108 | 47 | 0 |
| attribute_item | #[derive(serde::Deserialize)] | 4109 | 4109 | 29 | 0 |
| struct_item | CommandIngressDisorderFixture | 4110 | 4112 | 94 | 0 |
| attribute_item | #[derive(serde::Deserialize)] | 4114 | 4114 | 29 | 0 |
| struct_item | CommandIngressDisorderRow | 4115 | 4121 | 193 | 0 |
| line_comment | /// 📤️ Every page of one command with the cursor the h | 4123 | 4123 | 108 | 0 |
| line_comment | /// `encode_app_command`, cut at the page extent exactl | 4124 | 4124 | 100 | 0 |
| line_comment | /// may deliver them in an order no production owner wo | 4125 | 4125 | 59 | 0 |
| function_item | command_ingress_pages_of | 4126 | 4154 | 1493 | 0 |
| line_comment | /// ⚖️ LAW: the guest reassembles a multi-page command  | 4156 | 4156 | 106 | 0 |
| line_comment | /// not the next one — a duplicate, a skip and a swap a | 4157 | 4157 | 95 | 0 |
| line_comment | /// | 4158 | 4158 | 3 | 0 |
| line_comment | /// 🧨️ A stream with no page ceiling is only safe if t | 4159 | 4159 | 105 | 0 |
| line_comment | /// page per turn over an unbounded number of turns, so | 4160 | 4160 | 100 | 0 |
| line_comment | /// to splice a command together out of the wrong bytes | 4161 | 4161 | 56 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4162 | 4162 | 43 | 0 |
| function_item | a_command_page_that_is_not_the_next_one_is_refused_by_its_order | 4163 | 4198 | 3541 | 3 |
| line_comment | /// ⚖️ LAW: an ingress owner the host has stopped drivi | 4200 | 4200 | 97 | 0 |
| line_comment | /// driving, and never pins the ingress lane against it | 4201 | 4201 | 56 | 0 |
| line_comment | /// | 4202 | 4202 | 3 | 0 |
| line_comment | /// 🧭️ Nothing on this wire tells the guest that a com | 4203 | 4203 | 92 | 0 |
| line_comment | /// `CommandBatchDriver` simply stops existing when its | 4204 | 4204 | 97 | 0 |
| line_comment | /// out or its own cleanup tears the command down. The  | 4205 | 4205 | 98 | 0 |
| line_comment | /// a page naming a DIFFERENT command on the same insta | 4206 | 4206 | 95 | 0 |
| line_comment | /// per instance at a time. So that page must supersede | 4207 | 4207 | 94 | 0 |
| line_comment | /// every status the new command's own driver observes  | 4208 | 4208 | 99 | 0 |
| line_comment | /// on the turns the host carries no page at all, which | 4209 | 4209 | 93 | 0 |
| line_comment | /// speak. | 4210 | 4210 | 10 | 0 |
| line_comment | /// | 4211 | 4211 | 3 | 0 |
| line_comment | /// 🐛️ Before the retiring lane the superseded owner s | 4212 | 4212 | 103 | 0 |
| line_comment | /// page of the new command answered `Backpressure` for | 4213 | 4213 | 94 | 0 |
| line_comment | /// `CommandPending` with the STALE cursor, which `vali | 4214 | 4214 | 88 | 0 |
| line_comment | /// `plugin.command-cursor-mismatch: … owner is 1, the  | 4215 | 4215 | 100 | 0 |
| line_comment | /// held every semio-MCP mutation verb (ticket 26/09/18 | 4216 | 4216 | 56 | 0 |
| line_comment | /// `📓️a2-mcp-plugin-host-instance-open.md` §6.1, `📓️ | 4217 | 4217 | 103 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4218 | 4218 | 43 | 0 |
| function_item | an_abandoned_ingress_owner_never_answers_the_command_the_host_is_driving | 4219 | 4271 | 4617 | 4 |
| line_comment | /// ⚖️ LAW: a long command stream leaves the guest's re | 4273 | 4273 | 104 | 0 |
| line_comment | /// is dispatched and closed within its own turns; noth | 4274 | 4274 | 99 | 0 |
| line_comment | /// behind, and the run retains no per-command heap. | 4275 | 4275 | 52 | 0 |
| line_comment | /// | 4276 | 4276 | 3 | 0 |
| line_comment | /// 🚚️ The commands go through the real `poll_kernel`  | 4277 | 4277 | 106 | 0 |
| line_comment | /// by the real `CommandBatchDriver` — not through a te | 4278 | 4278 | 87 | 0 |
| line_comment | /// | 4279 | 4279 | 3 | 0 |
| line_comment | /// 🧵️ This thread's own allocator scope — see `retain | 4280 | 4280 | 93 | 0 |
| line_comment | /// process-wide difference weighs every concurrent law | 4281 | 4281 | 92 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4282 | 4282 | 43 | 0 |
| function_item | a_long_command_stream_never_pins_the_retained_ingress_authority | 4283 | 4328 | 3470 | 3 |
| function_item | __semio_plugin_bundle | 4330 | 4332 | 320 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4334 | 4334 | 43 | 0 |
| function_item | plugin_builder_builds_bundle_from_fluent_spec | 4335 | 4341 | 418 | 4 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4343 | 4343 | 43 | 0 |
| function_item | test_app_id_matches_its_own_dialect | 4344 | 4346 | 209 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4348 | 4348 | 43 | 0 |
| function_item | plugin_builder_wires_app_factory_for_create_app | 4349 | 4378 | 3448 | 8 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4380 | 4380 | 43 | 0 |
| function_item | merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads | 4381 | 4451 | 6533 | 15 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4453 | 4453 | 43 | 0 |
| function_item | operation_action_emits_kernel_op_with_true_inverse | 4454 | 4477 | 2417 | 5 |
| line_comment | //#region 🔖️EphemeralLaneTests | 4479 | 4479 | 35 | 0 |
| line_comment | /// 🧾️ Neither ephemeral lane may appear in history: t | 4480 | 4480 | 93 | 0 |
| line_comment | /// command-log rows of their own — the document's sing | 4481 | 4481 | 92 | 0 |
| line_comment | /// | 4482 | 4482 | 3 | 0 |
| line_comment | /// ↩️ Undo rolls back the DOCUMENT; the ephemeral lane | 4483 | 4483 | 91 | 0 |
| line_comment | /// were never part of the undoable gesture in the firs | 4484 | 4484 | 63 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4485 | 4485 | 43 | 0 |
| function_item | a_command_reaches_both_ephemeral_lanes_without_touching_history | 4486 | 4532 | 4800 | 15 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4534 | 4534 | 43 | 0 |
| function_item | a_command_that_emits_nothing_ephemeral_leaves_both_lanes_untouched | 4535 | 4555 | 2076 | 2 |
| line_comment | /// 👥️ Contract-freeze §C7.6, "zero app-side code": an | 4557 | 4557 | 100 | 0 |
| line_comment | /// domain with `broadcast: true` (nothing app-specific | 4558 | 4558 | 95 | 0 |
| line_comment | /// `ephemeral_snapshot().interaction` — assembled pure | 4559 | 4559 | 98 | 0 |
| line_comment | /// the framework-owned `interaction_store`/`interactio | 4560 | 4560 | 70 | 0 |
| line_comment | /// | 4561 | 4561 | 3 | 0 |
| line_comment | /// 🧪️ `interaction_topology`'s fixture only knows "it | 4562 | 4562 | 93 | 0 |
| line_comment | /// non-empty (see that fn's own doc comment) — without | 4563 | 4563 | 92 | 0 |
| line_comment | /// the pick as an unknown id and the domain never show | 4564 | 4564 | 85 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4565 | 4565 | 43 | 0 |
| function_item | ephemeral_snapshot_carries_encoded_interaction_from_declared_broadcast_specs | 4566 | 4594 | 2748 | 5 |
| line_comment | /// 📏️ The unoptimized-build thread stack the bounded- | 4596 | 4596 | 120 | 0 |
| line_comment | /// 8 MiB (`.cargo/config.toml` `-zstack-size`) and onl | 4597 | 4597 | 121 | 0 |
| line_comment | /// async state machines are several times larger. Meas | 4598 | 4598 | 117 | 0 |
| line_comment | /// between 4 and 8 MiB, the select dispatch on top bet | 4599 | 4599 | 118 | 0 |
| line_comment | /// that still fails on a runaway recursion or a re-inl | 4600 | 4600 | 70 | 0 |
| const_item | BOUNDED_THREAD_STACK_BYTES | 4601 | 4601 | 59 | 0 |
| line_comment | /// 🧵️ Every world/graph pick is one framework-reserve | 4603 | 4603 | 106 | 0 |
| line_comment | /// a bounded thread stack. `dispatch_framework_reserve | 4604 | 4604 | 100 | 0 |
| line_comment | /// routes as separate `run_framework_reserved_job` awa | 4605 | 4605 | 101 | 0 |
| line_comment | /// gives each such await its own non-overlapping slot, | 4606 | 4606 | 101 | 0 |
| line_comment | /// and every pick aborted the process with `fatal runt | 4607 | 4607 | 102 | 0 |
| line_comment | /// plain OS thread gets (ticket 26/09/09/PROCEDURAL-3D | 4608 | 4608 | 67 | 0 |
| line_comment | /// `📓️selection-overflow-2026-09-09.md`). 2 MiB is ex | 4609 | 4609 | 95 | 0 |
| line_comment | /// construct-select-close lifecycle measures ~1.75 MiB | 4610 | 4610 | 97 | 0 |
| line_comment | /// before the split. | 4611 | 4611 | 21 | 0 |
| attribute_item | #[cfg(not(target_arch = "wasm32"))] | 4612 | 4612 | 35 | 0 |
| attribute_item | #[test] | 4613 | 4613 | 7 | 0 |
| function_item | one_framework_reserved_route_fits_a_bounded_thread_stack | 4614 | 4640 | 2438 | 0 |
| line_comment | //#endregion 🔖️EphemeralLaneTests | 4641 | 4641 | 38 | 0 |
| line_comment | //#region 🔖️AdoptPresenceTests | 4643 | 4643 | 35 | 0 |
| function_item | sample_presence_peer | 4644 | 4664 | 689 | 0 |
| function_item | publish_presence_roster | 4666 | 4689 | 1717 | 0 |
| line_comment | /// 👥️ Contract-freeze §C7.6: `adopt_presence` (1) ado | 4691 | 4691 | 97 | 0 |
| line_comment | /// `presence_store` ONLY when one is present, (2) unco | 4692 | 4692 | 93 | 0 |
| line_comment | /// `interaction` into `peer_presence` for every peer i | 4693 | 4693 | 94 | 0 |
| line_comment | /// as the single source of truth — a peer absent from  | 4694 | 4694 | 96 | 0 |
| line_comment | /// | 4695 | 4695 | 3 | 0 |
| line_comment | /// 👋 bob leaves the roster — a second call carrying o | 4696 | 4696 | 98 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4697 | 4697 | 43 | 0 |
| function_item | retained_presence_fills_presence_store_and_peer_marks_and_drops_left_peers | 4698 | 4733 | 3236 | 12 |
| line_comment | /// 🧮️ The drive bound was a hand-picked 16 calls; the | 4735 | 4735 | 101 | 0 |
| line_comment | /// then the peer's own bytes one at a time). The count | 4736 | 4736 | 98 | 0 |
| line_comment | /// of the product, so it is replaced by the stronger s | 4737 | 4737 | 93 | 0 |
| line_comment | /// bounded call must release something until the autho | 4738 | 4738 | 98 | 0 |
| line_comment | /// fails at the exact call it happened on instead of h | 4739 | 4739 | 83 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4740 | 4740 | 43 | 0 |
| function_item | peer_presence_capture_is_one_arc_and_retirement_waits_for_then_drains_the_exact_root | 4741 | 4792 | 4391 | 7 |
| line_comment | /// 🎡️ One maintenance call now runs every stage that  | 4794 | 4794 | 100 | 0 |
| line_comment | /// call's `released_items` is not this stage's answer. | 4795 | 4795 | 98 | 0 |
| line_comment | /// asserted exactly: a grant one byte short of the ret | 4796 | 4796 | 97 | 0 |
| line_comment | /// stays mounted (next line). | 4797 | 4797 | 30 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4798 | 4798 | 43 | 0 |
| function_item | peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority | 4799 | 4883 | 6897 | 11 |
| line_comment | //#endregion 🔖️AdoptPresenceTests | 4884 | 4884 | 38 | 0 |
| line_comment | //#region 🔖️InteractionViewPeersTests | 4886 | 4886 | 42 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 4887 | 4887 | 43 | 0 |
| function_item | interaction_view_peers_selecting_returns_actor_and_color | 4888 | 4916 | 1951 | 9 |
| line_comment | //#endregion 🔖️InteractionViewPeersTests | 4917 | 4917 | 45 | 0 |
| line_comment | //#region 🔖️CompositionTests | 4919 | 4919 | 33 | 0 |
| macro_invocation | store::space_members! {     native_producer:&::semio_fr | 4920 | 4927 | 779 | 0 |
| line_comment | /// 📇️ The APP's half of a composition: declares `chil | 4929 | 4929 | 121 | 0 |
| line_comment | /// it already declares. The runtime admits a member on | 4930 | 4930 | 107 | 0 |
| line_comment | /// (`declared_child_reference`), so a law that registe | 4931 | 4931 | 114 | 0 |
| line_comment | /// parent's own leaf does before its member is opened. | 4932 | 4932 | 55 | 0 |
| function_item | declare_test_child | 4933 | 4939 | 644 | 0 |
| line_comment | /// 🤝️ [`declare_test_child`], then the RUNTIME's half | 4941 | 4941 | 123 | 0 |
| function_item | register_test_child | 4942 | 4945 | 389 | 0 |
| line_comment | /// 🧪️ A live child `ArtifactStore<TestSnapshot, TestM | 4947 | 4947 | 98 | 0 |
| line_comment | /// the shape `VcsArtifactApp::register_child`/`open_ch | 4948 | 4948 | 86 | 0 |
| line_comment | /// runtime `ChildStoreFactory`/`MemberFactory` registr | 4949 | 4949 | 91 | 0 |
| line_comment | /// dispatches by kind at compile time) — the SAME `Tes | 4950 | 4950 | 87 | 0 |
| line_comment | /// `TestApp` itself uses, so a "child" here is just a  | 4951 | 4951 | 91 | 0 |
| line_comment | /// of the identical document shape. | 4952 | 4952 | 36 | 0 |
| function_item | new_test_child | 4953 | 4959 | 677 | 0 |
| line_comment | /// 🫙️ The owner-less twin of {@link new_test_child},  | 4961 | 4961 | 96 | 0 |
| line_comment | /// member that has NOT yet adopted its bounded retirem | 4962 | 4962 | 96 | 0 |
| line_comment | /// member through `create_member_store`/`open_member_s | 4963 | 4963 | 96 | 0 |
| line_comment | /// shape exists only to drive the refusal those laws a | 4964 | 4964 | 61 | 0 |
| function_item | new_bare_test_child | 4965 | 4969 | 528 | 0 |
| struct_item | TestSnapshotRetirement | 4971 | 4974 | 136 | 0 |
| impl_item | impl store::ErasedSnapshotRetirement for TestSnapshotRe | 4976 | 4992 | 1433 | 0 |
| impl_item | impl TestSnapshotRetirement {         fn demands(&self, | 4994 | 4999 | 451 | 0 |
| line_comment | /// 🎭️ Spends one invalid terminal witness while retai | 5001 | 5001 | 123 | 0 |
| attribute_item | #[derive(semio_framework_value::FactoryPayloadRetiremen | 5002 | 5002 | 58 | 0 |
| struct_item | TestLyingOwnedValueRetirementFactory | 5003 | 5006 | 135 | 0 |
| impl_item | impl store::ArtifactOwnedValueRetirementFactory<TestSna | 5008 | 5023 | 1527 | 0 |
| impl_item | impl store::os_store::document_store_owners::MemberStor | 5025 | 5052 | 3782 | 0 |
| line_comment | /// 🔐️ Adopts the exact member owner catalog a product | 5054 | 5054 | 78 | 0 |
| line_comment | /// `create_member_store`, for a law that first drove t | 5055 | 5055 | 95 | 0 |
| line_comment | /// then has to close the app it registered that member | 5056 | 5056 | 61 | 0 |
| function_item | install_test_member_owners | 5057 | 5060 | 422 | 0 |
| line_comment | /// 🎭️ Installs the member owner catalog whose OWNED-V | 5062 | 5062 | 91 | 0 |
| line_comment | /// the one `retire_snapshot_read_erased` draws its uni | 5063 | 5063 | 97 | 0 |
| line_comment | /// place a child-content retirement can meet a lying t | 5064 | 5064 | 95 | 0 |
| line_comment | /// factory stays honest on purpose: it retires the sna | 5065 | 5065 | 95 | 0 |
| line_comment | /// would pin that alias for ever, so the captured root | 5066 | 5066 | 94 | 0 |
| line_comment | /// owner and `Arc::into_inner` would never reach the o | 5067 | 5067 | 74 | 0 |
| function_item | install_test_snapshot_retirement | 5068 | 5079 | 1588 | 0 |
| function_item | test_child_dialect | 5081 | 5083 | 168 | 0 |
| function_item | close_member_admission_fixture | 5085 | 5098 | 986 | 3 |
| function_item | close_member_admission_app | 5100 | 5102 | 326 | 0 |
| macro_invocation | include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪 | 5104 | 5104 | 98 | 0 |
| empty_statement | ; | 5104 | 5104 | 1 | 0 |
| macro_invocation | include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪 | 5106 | 5106 | 113 | 0 |
| empty_statement | ; | 5106 | 5106 | 1 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5108 | 5108 | 43 | 0 |
| function_item | member_factory_closed_dialect_open_failure_retains_pin_and_drains_exact_member | 5109 | 5155 | 5298 | 14 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5157 | 5157 | 43 | 0 |
| function_item | member_factory_closed_dialect_register_rejects_pin_without_mutating_member | 5158 | 5192 | 3495 | 7 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5194 | 5194 | 43 | 0 |
| function_item | member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner | 5195 | 5224 | 3641 | 7 |
| line_comment | /// 🧾️ One `KernelMutation` for the parent's own op, o | 5226 | 5226 | 95 | 0 |
| line_comment | /// its OWN document handle, never the parent's, for th | 5227 | 5227 | 84 | 0 |
| line_comment | /// target" requirement). | 5228 | 5228 | 25 | 0 |
| line_comment | /// | 5229 | 5229 | 3 | 0 |
| line_comment | /// 🧾️ ONE `UndoGroup` names BOTH documents via `membe | 5230 | 5230 | 68 | 0 |
| line_comment | /// | 5231 | 5231 | 3 | 0 |
| line_comment | /// The child store actually applied its own op. | 5232 | 5232 | 48 | 0 |
| line_comment | /// | 5233 | 5233 | 3 | 0 |
| line_comment | /// And the command log recorded the child's edit id in | 5234 | 5234 | 73 | 0 |
| line_comment | /// | 5235 | 5235 | 3 | 0 |
| line_comment | /// 🧾️ The migrated route logs the admission row (no e | 5236 | 5236 | 94 | 0 |
| line_comment | /// publication logs the real one, so the subject is th | 5237 | 5237 | 94 | 0 |
| line_comment | /// one composite row names the child's edit, and it is | 5238 | 5238 | 90 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5239 | 5239 | 43 | 0 |
| function_item | composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles | 5240 | 5292 | 4579 | 12 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5294 | 5294 | 43 | 0 |
| function_item | an_agent_transaction_carries_owned_child_op_groups_and_commits_undoes_and_redoes_them_as_one_group | 5295 | 5354 | 5878 | 18 |
| line_comment | /// 📥️ Drives one stepped document archive load on `ap | 5356 | 5356 | 105 | 0 |
| function_item | drive_test_archive_load | 5357 | 5367 | 784 | 0 |
| line_comment | /// 🧒️ Registering a member is the RUNTIME's job; decl | 5369 | 5369 | 97 | 0 |
| line_comment | /// APP's: the runtime admits a member only under the i | 5370 | 5370 | 94 | 0 |
| line_comment | /// | 5371 | 5371 | 3 | 0 |
| line_comment | /// 📤️ Persist exactly what the host does: the documen | 5372 | 5372 | 100 | 0 |
| line_comment | /// parent pack and `.spr` plus one owner-stamped entry | 5373 | 5373 | 91 | 0 |
| line_comment | /// recursive archive is the single carrier). | 5374 | 5374 | 45 | 0 |
| line_comment | /// | 5375 | 5375 | 3 | 0 |
| line_comment | /// 📥️ Reload into a FRESH app through the stepped arc | 5376 | 5376 | 97 | 0 |
| line_comment | /// child comes back as its OWN live store, at the valu | 5377 | 5377 | 95 | 0 |
| line_comment | /// reloaded document. | 5378 | 5378 | 22 | 0 |
| line_comment | /// | 5379 | 5379 | 3 | 0 |
| line_comment | /// 🚫️ The members travel WITH the parent: the same ar | 5380 | 5380 | 95 | 0 |
| line_comment | /// ownership closure (`closure-rejected`), so a compos | 5381 | 5381 | 97 | 0 |
| line_comment | /// "parent first, children later" route. | 5382 | 5382 | 41 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5383 | 5383 | 43 | 0 |
| function_item | a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames | 5384 | 5432 | 4957 | 9 |
| line_comment | /// 🪆️ LAW (design §20.15, W-b): `child_head_packs` —  | 5434 | 5434 | 134 | 0 |
| line_comment | /// the `child:<slot>/<childId>` dependencies of an inf | 5435 | 5435 | 128 | 0 |
| line_comment | /// its slot, id and dialect: a reader that composes on | 5436 | 5436 | 117 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5437 | 5437 | 43 | 0 |
| function_item | a_composed_documents_child_heads_answer_each_owned_childs_current_head_snapshot | 5438 | 5464 | 3017 | 3 |
| line_comment | /// 🎞️ LAW (W2A-6 P5, `📓️api-stepped-document-load.md | 5466 | 5466 | 137 | 0 |
| line_comment | /// inside the call — `consume_media` answers the carri | 5467 | 5467 | 135 | 0 |
| line_comment | /// only the stepped archive load (admit → polls → ackn | 5468 | 5468 | 138 | 0 |
| line_comment | /// a text-only host edge carries it losslessly. A fore | 5469 | 5469 | 121 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5470 | 5470 | 43 | 0 |
| function_item | a_whole_document_media_import_is_a_stepped_archive_load_and_never_folds_inside_the_call | 5471 | 5524 | 6184 | 9 |
| line_comment | /// 🪆️ LAW (design §20.15, S4-LOAD wave 4): a COMPOSED | 5526 | 5526 | 144 | 0 |
| line_comment | /// recursive archive (parent pack + `.spr` + members)  | 5527 | 5527 | 133 | 0 |
| line_comment | /// adopts the members: the child comes back as its own | 5528 | 5528 | 101 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5529 | 5529 | 43 | 0 |
| function_item | a_composed_document_crosses_a_media_edge_with_its_children | 5530 | 5570 | 5335 | 4 |
| line_comment | /// 🧪️ Runs the reserved spawn-job the host would for  | 5572 | 5572 | 95 | 0 |
| line_comment | /// `JobCompleted` carries. | 5573 | 5573 | 27 | 0 |
| function_item | finish_reserved_spawn_job | 5574 | 5584 | 686 | 0 |
| line_comment | /// 🧪️ One reactor-turn continuation unit, polled exac | 5586 | 5586 | 103 | 0 |
| line_comment | /// a no-op waker. A unit that suspends is the `resolve | 5587 | 5587 | 98 | 0 |
| line_comment | /// trap that killed the gis actor on its first auto ch | 5588 | 5588 | 62 | 0 |
| function_item | original_publication_turn | 5589 | 5595 | 817 | 0 |
| line_comment | /// 🧪️ Polls one publication under the same once-paid  | 5597 | 5597 | 93 | 0 |
| function_item | continuation_unit_is_ready_on_first_poll | 5598 | 5601 | 408 | 0 |
| line_comment | /// ⚖️ A composed `commitCheckpoint` is a job driven ac | 5603 | 5603 | 101 | 0 |
| line_comment | /// queues the commit, every continuation unit complete | 5604 | 5604 | 96 | 0 |
| line_comment | /// unit per composed child plus the parent's own, repo | 5605 | 5605 | 91 | 0 |
| line_comment | /// publishes its result — pins included — only after t | 5606 | 5606 | 72 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5607 | 5607 | 43 | 0 |
| function_item | a_composed_checkpoint_commit_is_driven_one_ready_unit_per_turn_with_progress | 5608 | 5656 | 4455 | 9 |
| line_comment | /// ⚖️ Cancelling the operation between units stops a c | 5658 | 5658 | 95 | 0 |
| line_comment | /// commits: the outcome is `interactive-job.cancelled` | 5659 | 5659 | 97 | 0 |
| line_comment | /// still closes to terminal emptiness. | 5660 | 5660 | 39 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5661 | 5661 | 43 | 0 |
| function_item | a_composed_checkpoint_commit_cancels_between_units | 5662 | 5697 | 3703 | 6 |
| line_comment | /// 📌️ Checkpoint the parent: the cascade must commit  | 5699 | 5699 | 94 | 0 |
| line_comment | /// child checkpoint that commit produced. | 5700 | 5700 | 42 | 0 |
| line_comment | /// | 5701 | 5701 | 3 | 0 |
| line_comment | /// ⏭️ Move both forward, past the pin. | 5702 | 5702 | 43 | 0 |
| line_comment | /// | 5703 | 5703 | 3 | 0 |
| line_comment | /// ⏮️ Checking the parent out to the pinned checkpoint | 5704 | 5704 | 96 | 0 |
| line_comment | /// otherwise a restored composition silently mixes an  | 5705 | 5705 | 83 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5706 | 5706 | 43 | 0 |
| function_item | a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them | 5707 | 5739 | 3283 | 4 |
| line_comment | /// 🧪️ The child's current `count`, read through the s | 5741 | 5741 | 99 | 0 |
| function_item | reads_child_count | 5742 | 5745 | 279 | 0 |
| line_comment | /// 🧾️ The command capture this law holds IS the alias | 5747 | 5747 | 101 | 0 |
| line_comment | /// a real captured root is released when its operation | 5748 | 5748 | 94 | 0 |
| line_comment | /// already run against it. | 5749 | 5749 | 27 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5750 | 5750 | 43 | 0 |
| function_item | child_content_publication_path_copies_fixed_pages_and_command_capture_retains_one_root | 5751 | 5780 | 3094 | 6 |
| line_comment | /// 🎡️ The rotation is fair: one call runs every stage | 5782 | 5782 | 96 | 0 |
| line_comment | /// reports a blocked stage only once nothing else can  | 5783 | 5783 | 95 | 0 |
| line_comment | /// EVENTUAL answer rather than the answer of the one c | 5784 | 5784 | 88 | 0 |
| line_comment | /// authority never reports a block at all, so nothing  | 5785 | 5785 | 72 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5786 | 5786 | 43 | 0 |
| function_item | child_snapshot_retirement_rejection_preserves_exact_erased_owner | 5787 | 5811 | 2837 | 1 |
| line_comment | /// 🎡️ The property is RECLAMATION, not the step word. | 5813 | 5813 | 96 | 0 |
| line_comment | /// walks every stage that still has something to give, | 5814 | 5814 | 98 | 0 |
| line_comment | /// `Complete` and that is what the call returns — the  | 5815 | 5815 | 99 | 0 |
| line_comment | /// LYING owner reaching a terminal step and failed on  | 5816 | 5816 | 96 | 0 |
| line_comment | /// happen is the lying owner's authority leaving the r | 5817 | 5817 | 98 | 0 |
| line_comment | /// is asserted to leave `child_content_retirements` po | 5818 | 5818 | 94 | 0 |
| line_comment | /// clause was a proxy for, and the fault must still ar | 5819 | 5819 | 87 | 0 |
| line_comment | /// | 5820 | 5820 | 3 | 0 |
| line_comment | /// 🪢️ The registered root captured the member's CURRE | 5821 | 5821 | 97 | 0 |
| line_comment | /// holds that exact `Arc` the returned-read retirement | 5822 | 5822 | 91 | 0 |
| line_comment | /// answers `None`, the alias is released truthfully an | 5823 | 5823 | 93 | 0 |
| line_comment | /// — where the lie lives — is never consulted. Advanci | 5824 | 5824 | 95 | 0 |
| line_comment | /// draining the alias its displacement retired makes t | 5825 | 5825 | 91 | 0 |
| line_comment | /// owner, which is the only state in which a child-con | 5826 | 5826 | 93 | 0 |
| line_comment | /// at all. | 5827 | 5827 | 11 | 0 |
| line_comment | /// | 5828 | 5828 | 3 | 0 |
| line_comment | /// 🪪️ Either named guard is an acceptable answer — th | 5829 | 5829 | 99 | 0 |
| line_comment | /// refusal, or the nested disposer refusal it now attr | 5830 | 5830 | 87 | 0 |
| line_comment | /// anonymous `plugin.internal` — but an unnamed intern | 5831 | 5831 | 73 | 0 |
| line_comment | /// | 5832 | 5832 | 3 | 0 |
| line_comment | /// 🧹️ The lie is spent (see `TestSnapshotRetirement:: | 5833 | 5833 | 98 | 0 |
| line_comment | /// was caught now answers honestly and the fixture clo | 5834 | 5834 | 94 | 0 |
| line_comment | /// law proves the refusal, not a leak. | 5835 | 5835 | 39 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5836 | 5836 | 43 | 0 |
| function_item | child_root_maintenance_requires_terminal_empty_before_reclaim | 5837 | 5894 | 4770 | 5 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5896 | 5896 | 43 | 0 |
| function_item | child_root_maintenance_reclaims_completed_owner_for_later_publication | 5897 | 5927 | 3117 | 1 |
| line_comment | /// ⌨️ LAW (coordinator P1, ticket 26/09/23 C12): child | 5929 | 5929 | 119 | 0 |
| line_comment | /// maintenance turn between them — never fault. Every  | 5930 | 5930 | 123 | 0 |
| line_comment | /// reclaim) instead of answering `interactive-job.chil | 5931 | 5931 | 117 | 0 |
| line_comment | /// order (exactly the next generation), the registry n | 5932 | 5932 | 119 | 0 |
| line_comment | /// reports maintenance pressure until maintenance retu | 5933 | 5933 | 97 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5934 | 5934 | 43 | 0 |
| function_item | child_publications_at_the_maximum_rate_wait_on_retirement_instead_of_faulting | 5935 | 5963 | 3300 | 4 |
| line_comment | /// ⏱️ This clause used to be an 8 ms wall clock. The c | 5965 | 5965 | 100 | 0 |
| line_comment | /// maximum child's bytes never cross the public dispat | 5966 | 5966 | 93 | 0 |
| line_comment | /// continuation instead — and on a loaded machine the  | 5967 | 5967 | 98 | 0 |
| line_comment | /// property itself is asserted directly and determinis | 5968 | 5968 | 89 | 0 |
| line_comment | /// continuation, its own answer carries no child bytes | 5969 | 5969 | 93 | 0 |
| line_comment | /// below stay at zero. Nothing the clock could catch i | 5970 | 5970 | 89 | 0 |
| line_comment | /// 4 MiB snapshot would have to clone it or pack-encod | 5971 | 5971 | 82 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 5972 | 5972 | 43 | 0 |
| function_item | maximum_child_public_dispatch_reaches_first_continuation_without_clone_or_encode | 5973 | 6005 | 3720 | 5 |
| line_comment | /// ↩️ Store-level undo bypasses `ArtifactApp::handle`  | 6007 | 6007 | 97 | 0 |
| line_comment | /// `thread_local!` child caches this view replaces use | 6008 | 6008 | 69 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6009 | 6009 | 43 | 0 |
| function_item | the_child_content_view_never_goes_stale_across_undo_and_redo | 6010 | 6035 | 2497 | 3 |
| line_comment | /// 🪢️ `child-b` is registered but NEVER targeted by t | 6037 | 6037 | 93 | 0 |
| line_comment | /// `tail_group_id()` stays `None`, the textbook "forei | 6038 | 6038 | 86 | 0 |
| line_comment | /// skip rather than abort the whole group over. | 6039 | 6039 | 48 | 0 |
| line_comment | /// | 6040 | 6040 | 3 | 0 |
| line_comment | /// The parent reverted... | 6041 | 6041 | 26 | 0 |
| line_comment | /// | 6042 | 6042 | 3 | 0 |
| line_comment | /// ...child-a (the real group member) reverted too... | 6043 | 6043 | 54 | 0 |
| line_comment | /// | 6044 | 6044 | 3 | 0 |
| line_comment | /// ...and child-b — a genuine foreign tail, never touc | 6045 | 6045 | 93 | 0 |
| line_comment | /// SKIPPED, not silently dropped nor allowed to abort  | 6046 | 6046 | 77 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6047 | 6047 | 43 | 0 |
| function_item | group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child | 6048 | 6080 | 2959 | 5 |
| line_comment | /// 🧒️ The parent declares the created child first: ab | 6082 | 6082 | 94 | 0 |
| line_comment | /// identity the published parent declares, like every  | 6083 | 6083 | 71 | 0 |
| line_comment | /// | 6084 | 6084 | 3 | 0 |
| line_comment | /// 🌱️ Proves `VcsArtifactApp::absorb_created_children | 6085 | 6085 | 80 | 0 |
| line_comment | /// `ChildGenesis`-authoring `Emit` constructor (a late | 6086 | 6086 | 85 | 0 |
| line_comment | /// freshly-minted child reachable at all; per B2's own | 6087 | 6087 | 88 | 0 |
| line_comment | /// doc comment, skipping this step would make `ChildGe | 6088 | 6088 | 72 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6089 | 6089 | 43 | 0 |
| function_item | created_children_survive_absorb_into_the_child_store_map | 6090 | 6116 | 2787 | 2 |
| line_comment | //#endregion 🔖️CompositionTests | 6117 | 6117 | 36 | 0 |
| line_comment | /// 👁️ A view command never advances the document. | 6119 | 6119 | 55 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6120 | 6120 | 43 | 0 |
| function_item | view_action_emits_no_operations | 6121 | 6142 | 2097 | 3 |
| line_comment | /// ⏪️ A row follows its parent document lane, else its | 6144 | 6144 | 122 | 0 |
| line_comment | /// of a row). 💠️lowpoly `addPrimitive` and 🎥️shootin | 6145 | 6145 | 125 | 0 |
| line_comment | /// config lane answered for them (ticket 26/09/18 S11  | 6146 | 6146 | 119 | 0 |
| attribute_item | #[test] | 6147 | 6147 | 7 | 0 |
| function_item | a_row_follows_its_parent_document_lane_and_else_its_children | 6148 | 6156 | 849 | 6 |
| line_comment | /// ⚖️ LAW (design §20.13, L4): a config-lane edit is n | 6158 | 6158 | 123 | 0 |
| line_comment | /// lists the document row alone (every row, no `edit_i | 6159 | 6159 | 121 | 0 |
| line_comment | /// config the select wrote stays. | 6160 | 6160 | 34 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6161 | 6161 | 43 | 0 |
| function_item | a_config_lane_edit_is_never_a_history_row_and_undo_never_steps_over_it | 6162 | 6185 | 2629 | 4 |
| line_comment | /// ⏪️ Revert-to-command walks the DOCUMENT lane alone  | 6188 | 6188 | 127 | 0 |
| line_comment | /// after it" — reverting to the first increment undoes | 6189 | 6189 | 121 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6190 | 6190 | 43 | 0 |
| function_item | revert_to_command_walks_the_document_lane_and_leaves_config_alone | 6191 | 6222 | 3231 | 6 |
| line_comment | /// 🫧️ The plugin cannot touch shell-owned state itsel | 6224 | 6224 | 104 | 0 |
| line_comment | /// instead of replaying anything locally, and does NOT | 6225 | 6225 | 90 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6226 | 6226 | 43 | 0 |
| function_item | shell_action_with_inverse_bubbles_a_replay_effect_instead_of_replaying_locally | 6227 | 6255 | 2843 | 4 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6257 | 6257 | 43 | 0 |
| function_item | shell_action_emits_host_effect_without_operations | 6258 | 6278 | 2057 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6280 | 6280 | 43 | 0 |
| function_item | copy_emits_clipboard_write_effect_with_no_operations | 6281 | 6305 | 2393 | 4 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6307 | 6307 | 43 | 0 |
| function_item | copy_on_empty_selection_is_a_benign_no_operation | 6308 | 6328 | 1976 | 2 |
| line_comment | /// ✂️ One undo restores the cut label — cut is a singl | 6330 | 6330 | 87 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6331 | 6331 | 43 | 0 |
| function_item | cut_removes_label_and_emits_clipboard_write_as_one_undo_unit | 6332 | 6356 | 2400 | 4 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6358 | 6358 | 43 | 0 |
| function_item | paste_materializes_fragment_at_original_anchor | 6359 | 6387 | 2364 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6389 | 6389 | 43 | 0 |
| function_item | paste_with_non_original_anchor_reaches_the_app_placement | 6390 | 6418 | 2411 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6420 | 6420 | 43 | 0 |
| function_item | paste_with_no_fragment_arg_is_a_benign_no_operation | 6421 | 6441 | 1990 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6443 | 6443 | 43 | 0 |
| function_item | copy_cut_paste_are_registered_as_clipboard_kind_actions | 6444 | 6455 | 572 | 1 |
| line_comment | /// 🪡️ A streamed tool transaction is ONE edit (design | 6457 | 6457 | 125 | 0 |
| line_comment | /// and one undo reverts the whole gesture back to the  | 6458 | 6458 | 67 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6459 | 6459 | 43 | 0 |
| function_item | streamed_ticks_and_their_commit_are_one_undo_step | 6460 | 6484 | 2351 | 2 |
| line_comment | /// 🧱️ Outside a tool transaction every emission is it | 6486 | 6486 | 123 | 0 |
| line_comment | /// reverts exactly the last dispatch. | 6487 | 6487 | 38 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6488 | 6488 | 43 | 0 |
| function_item | plain_emissions_never_coalesce | 6489 | 6511 | 2094 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6513 | 6513 | 43 | 0 |
| function_item | history_actions_round_trip_through_the_store | 6514 | 6546 | 2695 | 7 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6548 | 6548 | 43 | 0 |
| function_item | reserved_undo_invocation_does_not_require_window_ownership | 6549 | 6580 | 2819 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6582 | 6582 | 43 | 0 |
| function_item | reserved_undo_actor_ingress_admits_undeclared_window_kind | 6583 | 6634 | 4542 | 8 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6636 | 6636 | 43 | 0 |
| function_item | reserved_undo_first_turn_admits_spawn_job_and_drive_commits_history_route | 6637 | 6666 | 3094 | 6 |
| line_comment | /// 🧪 Browser #37 chrome: [Set Active Example, Resize  | 6668 | 6668 | 87 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6669 | 6669 | 43 | 0 |
| function_item | reserved_undo_pops_chrome_top_shell_then_publishes_history_patch | 6670 | 6758 | 6183 | 24 |
| line_comment | /// 🧪 #38 host-drive contract: spawn-admit undo reache | 6760 | 6760 | 108 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6761 | 6761 | 43 | 0 |
| function_item | reserved_undo_reaches_done_within_host_drive_contract | 6762 | 6811 | 3549 | 4 |
| line_comment | /// 🧪 Browser #37 mixed stack: document example under  | 6813 | 6813 | 82 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6814 | 6814 | 43 | 0 |
| function_item | reserved_undo_pops_shell_then_falls_through_to_document_store | 6815 | 6867 | 4017 | 10 |
| line_comment | /// 🧪️ The addressed borrowed receiver retains one ori | 6869 | 6869 | 113 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6870 | 6870 | 43 | 0 |
| function_item | reserved_undo_addressed_receiver_admits_isolated_spawn_job | 6871 | 6928 | 5683 | 2 |
| line_comment | /// 🧪 Browser-shaped `noteShellCommand`: a note with N | 6931 | 6931 | 101 | 0 |
| line_comment | /// undo target, so chrome-order undo steps over `shell | 6932 | 6932 | 96 | 0 |
| line_comment | /// and pops the one row that declared a real inverse.  | 6933 | 6933 | 97 | 0 |
| line_comment | /// its own `detail`) is an identity, not an inverse —  | 6934 | 6934 | 100 | 0 |
| line_comment | /// is what the window-kind gate refused `undeclared-ac | 6935 | 6935 | 99 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6936 | 6936 | 43 | 0 |
| function_item | reserved_undo_browser_note_without_inverse_is_not_an_undo_target | 6937 | 6987 | 4878 | 12 |
| line_comment | /// 🧪 §3.2's user-visible half: clicking a window betw | 6989 | 6989 | 101 | 0 |
| line_comment | /// which the chrome branch then popped instead of the  | 6990 | 6990 | 98 | 0 |
| line_comment | /// nothing and the host replayed the chrome id into th | 6991 | 6991 | 98 | 0 |
| line_comment | /// not an undo target, so the DOCUMENT edit is what `u | 6992 | 6992 | 96 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 6993 | 6993 | 43 | 0 |
| function_item | reserved_undo_steps_over_undeclared_chrome_and_pops_the_document_edit | 6994 | 7033 | 3497 | 6 |
| line_comment | /// 🧪 `ReplayShellCommand` survives the leftover `pack | 7035 | 7035 | 95 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7036 | 7036 | 43 | 0 |
| function_item | reserved_undo_replay_shell_command_survives_wire_roundtrip | 7037 | 7054 | 1278 | 1 |
| line_comment | //#region 🔖️CommandLogTests | 7056 | 7056 | 32 | 0 |
| line_comment | /// 🏷️ `increment` IS declared by `migrated_contract_r | 7057 | 7057 | 106 | 0 |
| line_comment | /// so the row carries its DECLARED label — still `Loca | 7058 | 7058 | 101 | 0 |
| line_comment | /// rather than untranslated English, which is the prop | 7059 | 7059 | 83 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7060 | 7060 | 43 | 0 |
| function_item | an_operation_action_appends_one_command_log_entry_linked_to_its_edit | 7061 | 7089 | 2568 | 8 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7091 | 7091 | 43 | 0 |
| function_item | a_streamed_gesture_appends_exactly_one_command_log_entry | 7092 | 7115 | 2349 | 1 |
| line_comment | /// 📜️ LAW (ticket 26/09/23 C12): a streamed gesture's | 7117 | 7117 | 103 | 0 |
| line_comment | /// `HISTORY_ROW_OPERATION_PREVIEW` operations — the ne | 7118 | 7118 | 128 | 0 |
| line_comment | /// costs the same per tick at the end of a long gestur | 7119 | 7119 | 73 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7120 | 7120 | 43 | 0 |
| function_item | a_long_streamed_gesture_previews_its_newest_operations_only | 7121 | 7152 | 3075 | 4 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7154 | 7154 | 43 | 0 |
| function_item | undo_and_redo_append_entries_and_never_shrink_the_log | 7155 | 7182 | 2607 | 5 |
| line_comment | /// 🧾️ `commands` is newest-first — take the MINIMUM s | 7184 | 7184 | 115 | 0 |
| line_comment | /// | 7185 | 7185 | 3 | 0 |
| line_comment | /// 🧾️ `commands` is newest-first — the just-appended  | 7186 | 7186 | 109 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7187 | 7187 | 43 | 0 |
| function_item | revert_to_command_restores_the_snapshot_and_appends_one_entry | 7188 | 7217 | 3012 | 5 |
| line_comment | /// 🧾️ The receiver never dispatched anything itself — | 7219 | 7219 | 107 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7220 | 7220 | 43 | 0 |
| function_item | ingested_remote_edits_are_backfilled_into_the_command_log | 7221 | 7258 | 3115 | 3 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7260 | 7260 | 43 | 0 |
| function_item | set_history_command_filter_emits_no_operations_and_updates_the_view | 7261 | 7280 | 2128 | 2 |
| line_comment | /// 📌️ Six since the explicit check-in row landed (`#s | 7282 | 7282 | 77 | 0 |
| line_comment | /// `framework.history.checkin`) between Commit Checkpo | 7283 | 7283 | 82 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7284 | 7284 | 43 | 0 |
| function_item | ui_history_panel_filters_rows_and_gates_the_backwards_action | 7285 | 7360 | 5211 | 14 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7362 | 7362 | 43 | 0 |
| function_item | ui_history_panel_clips_an_oversized_operation_description | 7363 | 7396 | 1757 | 2 |
| line_comment | /// 🏷️ A command label is authored copy that can carry | 7398 | 7398 | 104 | 0 |
| line_comment | /// row appends ` xN` on top. Both must clip on a char  | 7399 | 7399 | 99 | 0 |
| line_comment | /// every panel in the refresh (seen 2026-09-10: `dupli | 7400 | 7400 | 98 | 0 |
| line_comment | /// window failed `refreshUi` with `history-panel.comma | 7401 | 7401 | 66 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7402 | 7402 | 43 | 0 |
| function_item | ui_history_panel_clips_an_oversized_command_label_and_its_folded_count | 7403 | 7433 | 1841 | 3 |
| function_item | history_window_fixture | 7435 | 7437 | 160 | 0 |
| line_comment | /// 🪙️ A revertible row materialises a `UiValue::Map`; | 7439 | 7439 | 100 | 0 |
| line_comment | /// arena. The window laws below use the zero-cost shap | 7440 | 7440 | 93 | 0 |
| line_comment | /// arithmetic alone, never the process-global arena an | 7441 | 7441 | 92 | 0 |
| function_item | history_window_log | 7442 | 7463 | 972 | 0 |
| line_comment | /// 🧾️ The body as the host reads it — `BuiltChildren` | 7465 | 7465 | 102 | 0 |
| line_comment | /// direct serde walk, so the tree is projected node by | 7466 | 7466 | 61 | 0 |
| function_item | history_body_json | 7467 | 7473 | 298 | 0 |
| function_item | history_commands_window | 7475 | 7478 | 223 | 0 |
| line_comment | /// 🪟️ ARTIFACT-TREE-VIRTUALISED-STREAMING: the Comman | 7480 | 7480 | 103 | 0 |
| line_comment | /// are the live entries themselves (no `UI_BUILT_CHILD | 7481 | 7481 | 100 | 0 |
| line_comment | /// and its rows), it publishes the FULL filtered row c | 7482 | 7482 | 99 | 0 |
| line_comment | /// it materialises, a cold paint draws `TREE_WINDOW_DE | 7483 | 7483 | 96 | 0 |
| line_comment | /// ceiling, a host request draws exactly its slice key | 7484 | 7484 | 90 | 0 |
| line_comment | /// continuation row exists at any point. | 7485 | 7485 | 41 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7486 | 7486 | 43 | 0 |
| function_item | ui_history_panel_windows_command_rows_over_the_live_count | 7487 | 7524 | 2954 | 11 |
| line_comment | /// 🪙️ Every revertible row the window materialises ke | 7526 | 7526 | 100 | 0 |
| line_comment | /// never out-spend the process-wide argument arena: a  | 7527 | 7527 | 98 | 0 |
| line_comment | /// which the contract prices at `UI_VALUE_PAGE_ROWS` i | 7528 | 7528 | 98 | 0 |
| line_comment | /// pressure shortens the window — it never faults the  | 7529 | 7529 | 95 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7530 | 7530 | 43 | 0 |
| function_item | ui_history_panel_keeps_every_materialised_revert_inside_the_arena_page | 7531 | 7552 | 1483 | 4 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7554 | 7554 | 43 | 0 |
| function_item | a_config_only_view_action_is_not_a_history_row | 7555 | 7575 | 2183 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7577 | 7577 | 43 | 0 |
| function_item | consecutive_config_only_view_dispatches_add_no_history_rows | 7578 | 7600 | 2195 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7602 | 7602 | 43 | 0 |
| function_item | interleaved_config_only_views_leave_only_the_document_row | 7603 | 7626 | 2488 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7628 | 7628 | 43 | 0 |
| function_item | note_shell_command_is_intercepted_before_the_app_and_records_each_repeat | 7629 | 7660 | 2851 | 8 |
| line_comment | /// 🌐️ A shell-noted row's text is the shell's own alr | 7662 | 7662 | 102 | 0 |
| line_comment | /// locale-INVARIANT data (`LocalizedLabel::data`) — ev | 7663 | 7663 | 99 | 0 |
| line_comment | /// therefore never claims to be English, and the Histo | 7664 | 7664 | 96 | 0 |
| line_comment | /// shell instead of leaking a locale it does not have. | 7665 | 7665 | 55 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7666 | 7666 | 43 | 0 |
| function_item | a_shell_noted_row_carries_the_same_text_in_every_locale | 7667 | 7691 | 2290 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7693 | 7693 | 43 | 0 |
| function_item | history_delivery_does_not_widen_a_none_ui_scope | 7694 | 7713 | 1971 | 1 |
| line_comment | /// 📨️ LAW: a utility switch a dispatch's view shows i | 7715 | 7715 | 126 | 0 |
| line_comment | /// dispatch; the first view of a window, an unchanged  | 7716 | 7716 | 119 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7717 | 7717 | 43 | 0 |
| function_item | a_utility_switch_in_a_dispatch_view_delivers_one_host_event_to_its_window | 7718 | 7744 | 2953 | 3 |
| line_comment | /// 📨️ LAW: a forwarded `hostEvent{windowId, kind}` re | 7746 | 7746 | 124 | 0 |
| line_comment | /// answer dispatches without a history row of its own, | 7747 | 7747 | 87 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7748 | 7748 | 43 | 0 |
| function_item | a_forwarded_host_event_reaches_the_app_and_its_answer_is_never_a_history_row | 7749 | 7780 | 3038 | 3 |
| line_comment | /// 🙈️ A `View` verb that edited no store and carries  | 7782 | 7782 | 120 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7783 | 7783 | 43 | 0 |
| function_item | a_pure_view_verb_is_no_history_row | 7784 | 7807 | 2358 | 1 |
| line_comment | /// 🙈️ The row predicate: interactions never, views on | 7809 | 7809 | 110 | 0 |
| attribute_item | #[test] | 7810 | 7810 | 7 | 0 |
| function_item | history_rows_exclude_interactions_and_pure_views | 7811 | 7824 | 696 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7826 | 7826 | 43 | 0 |
| function_item | history_delivery_preserves_partial_ui_scope | 7827 | 7848 | 2163 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7850 | 7850 | 43 | 0 |
| function_item | scope_upgrade_full_stays_full | 7851 | 7870 | 1968 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7872 | 7872 | 43 | 0 |
| function_item | benign_undo_with_nothing_to_undo_stays_unlogged_with_scope_none | 7873 | 7892 | 2094 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7894 | 7894 | 43 | 0 |
| function_item | set_history_command_filter_is_never_logged | 7895 | 7914 | 2159 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7916 | 7916 | 43 | 0 |
| function_item | rendering_the_history_body_reflects_a_log_only_change_with_no_store_generation_bump | 7917 | 7939 | 2614 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7941 | 7941 | 43 | 0 |
| function_item | an_operation_kind_action_with_zero_operations_still_logs_one_entry | 7942 | 7967 | 2344 | 5 |
| line_comment | //#endregion 🔖️CommandLogTests | 7968 | 7968 | 35 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7970 | 7970 | 43 | 0 |
| function_item | undo_on_empty_history_is_a_benign_no_operation | 7971 | 7989 | 1880 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 7991 | 7991 | 43 | 0 |
| function_item | document_round_trips_through_serialization | 7992 | 8018 | 2540 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8020 | 8020 | 43 | 0 |
| function_item | ingest_operations_is_idempotent | 8021 | 8057 | 3192 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8059 | 8059 | 43 | 0 |
| function_item | attach_detach_reattach_resumes_backbone_convergence | 8060 | 8096 | 3369 | 8 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8098 | 8098 | 43 | 0 |
| function_item | selection_count_phrase_formats_mixed_selection | 8099 | 8103 | 566 | 3 |
| line_comment | /// 🖱️ `PluginApp::context_menu` end-to-end through `V | 8105 | 8105 | 98 | 0 |
| line_comment | /// "selection guard" (`Menu::when`) drops the gated co | 8106 | 8106 | 97 | 0 |
| line_comment | /// "something is selected"), both rows resolve label/i | 8107 | 8107 | 94 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8108 | 8108 | 43 | 0 |
| function_item | context_menu_resolves_labels_from_the_registry_and_respects_guards | 8109 | 8141 | 3325 | 4 |
| line_comment | //#region 🗂️GroupedContextMenu | 8143 | 8143 | 35 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8144 | 8144 | 43 | 0 |
| function_item | action_definition_with_category_sets_the_ribbon_taxonomy_field | 8145 | 8148 | 275 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8150 | 8150 | 43 | 0 |
| function_item | menu_group_produces_a_group_row_keyed_by_category | 8151 | 8159 | 748 | 4 |
| line_comment | /// 🧪️ A registry declaring `setLabelRequired` plus te | 8161 | 8161 | 101 | 0 |
| line_comment | /// `RIBBON_PARENT_CATEGORIES` category via `with_categ | 8162 | 8162 | 96 | 0 |
| line_comment | /// `"flat-menu-test"` branch below. | 8163 | 8163 | 36 | 0 |
| function_item | flat_menu_registry | 8164 | 8198 | 2247 | 0 |
| line_comment | /// 🖱️ End to end through `VcsArtifactApp::context_men | 8200 | 8200 | 104 | 0 |
| line_comment | /// back as 5 primaries + 3 taxonomy-sorted `menu.group | 8201 | 8201 | 103 | 0 |
| line_comment | /// `organize_context_menu` to every emitter, not just  | 8202 | 8202 | 95 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8203 | 8203 | 43 | 0 |
| function_item | context_menu_funnel_organizes_a_synthetic_apps_flat_overflow_menu | 8204 | 8235 | 3160 | 3 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8237 | 8237 | 43 | 0 |
| function_item | context_menu_wire_requires_and_forwards_canonical_view_state | 8238 | 8254 | 1534 | 7 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8256 | 8256 | 43 | 0 |
| function_item | surface_context_reaches_real_app_render_and_rejects_hidden_surfaces | 8257 | 8290 | 3555 | 7 |
| line_comment | //#endregion 🗂️GroupedContextMenu | 8291 | 8291 | 38 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8293 | 8293 | 43 | 0 |
| function_item | surface_context_refresh_projects_panels_from_focused_window_state | 8294 | 8316 | 2784 | 3 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8318 | 8318 | 43 | 0 |
| function_item | reserved_section_surfaces_render_accessor_maps_instead_of_app_bodies | 8319 | 8361 | 3944 | 8 |
| line_comment | /// 🧮️ Sized FROM the two ceilings rather than against | 8363 | 8363 | 104 | 0 |
| line_comment | /// a payload one whole node of children cannot hold, s | 8364 | 8364 | 96 | 0 |
| line_comment | /// `UI_TEXT_MAX_BYTES`/`UI_BUILT_CHILDREN_MAX` current | 8365 | 8365 | 99 | 0 |
| line_comment | /// JSON (`"window-0000":"measure-0000",`), so this man | 8366 | 8366 | 91 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8367 | 8367 | 43 | 0 |
| function_item | reserved_section_carrier_pages_a_payload_past_one_node_of_children | 8368 | 8405 | 2560 | 8 |
| line_comment | /// 🧩️ Pins the pack this producer writes against the  | 8407 | 8407 | 98 | 0 |
| line_comment | /// reader (`sectionValueFromBuiltNode` in `📺️renderer | 8408 | 8408 | 119 | 0 |
| line_comment | /// a reader that read only `value` handed `JSON.parse` | 8409 | 8409 | 92 | 0 |
| line_comment | /// 26/09/09/PROCEDURAL-3D-END-TO-END `📓️json-512-trun | 8410 | 8410 | 81 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8411 | 8411 | 43 | 0 |
| function_item | packed_section_carrier_matches_the_neutral_fixture_slices | 8412 | 8433 | 1913 | 11 |
| line_comment | //#region 🚚️World3dSceneLaneCarriers | 8435 | 8435 | 41 | 0 |
| line_comment | /// 🧱 Presented-node census of a projected builder tre | 8437 | 8437 | 88 | 0 |
| function_item | count_projected_nodes | 8438 | 8440 | 173 | 0 |
| function_item | count_projected_text_leaves | 8442 | 8445 | 257 | 0 |
| line_comment | /// 🚚️ Projects one built surface node and returns `(p | 8447 | 8447 | 100 | 0 |
| function_item | project_scene_surface | 8448 | 8457 | 540 | 0 |
| line_comment | /// 🚚️ Asserts one lane carrier obeys the contract's o | 8459 | 8459 | 102 | 0 |
| function_item | assert_lane_carrier | 8460 | 8477 | 835 | 3 |
| line_comment | /// 🌍️ A world-3d scene whose payload is far past the  | 8479 | 8479 | 93 | 0 |
| function_item | oversized_world_scene | 8480 | 8493 | 987 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8495 | 8495 | 43 | 0 |
| function_item | world3d_scene_surface_pages_every_lane_beside_a_spine_that_fits_the_fixed_doc | 8496 | 8525 | 2116 | 7 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8527 | 8527 | 43 | 0 |
| function_item | world3d_scene_surface_republishes_only_the_lanes_that_changed | 8528 | 8540 | 1410 | 3 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8542 | 8542 | 43 | 0 |
| function_item | world3d_scene_surface_pages_one_oversized_lane_instead_of_faulting | 8543 | 8552 | 1185 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8554 | 8554 | 43 | 0 |
| function_item | a_scene_that_declares_no_lanes_still_publishes_one_childless_surface | 8555 | 8561 | 684 | 3 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8563 | 8563 | 43 | 0 |
| function_item | nakagin_scale_world3d_surface_fits_document_node_cap | 8564 | 8570 | 599 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8572 | 8572 | 43 | 0 |
| function_item | a_100kib_measures_section_fits_document_node_cap | 8573 | 8580 | 798 | 2 |
| line_comment | //#endregion 🚚️World3dSceneLaneCarriers | 8582 | 8582 | 44 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8584 | 8584 | 43 | 0 |
| function_item | surface_context_presence_targets_each_concrete_surface | 8585 | 8619 | 3568 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8621 | 8621 | 43 | 0 |
| function_item | view_action_emitting_ops_is_rejected | 8622 | 8642 | 2268 | 2 |
| line_comment | //#region 🧪️IntentDispatchTests | 8644 | 8644 | 36 | 0 |
| line_comment | /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`) accept | 8645 | 8645 | 97 | 0 |
| line_comment | /// produces the mutation's document change AND a comma | 8646 | 8646 | 83 | 0 |
| line_comment | /// `handle_intent_frame` call — the reactor's own `pol | 8647 | 8647 | 96 | 0 |
| line_comment | /// since `plugin_dispatch_intents` (which calls this)  | 8648 | 8648 | 82 | 0 |
| line_comment | /// `dirty_render` pass. | 8649 | 8649 | 24 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8650 | 8650 | 43 | 0 |
| function_item | activate_intent_dispatches_through_the_typed_command_path_same_turn | 8651 | 8684 | 2897 | 3 |
| line_comment | /// 🎯️ M1 acceptance: a `View`-kind action arriving as | 8686 | 8686 | 96 | 0 |
| line_comment | /// still hard-faults — kind discipline survives the ne | 8687 | 8687 | 89 | 0 |
| line_comment | /// `handle_intent_frame` routes through the SAME `disp | 8688 | 8688 | 92 | 0 |
| line_comment | /// command path uses. | 8689 | 8689 | 22 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8690 | 8690 | 43 | 0 |
| function_item | view_kind_intent_returning_operations_hard_faults | 8691 | 8722 | 2788 | 2 |
| line_comment | /// 🎯️ M1 decision: `ActionId.version` mismatch return | 8724 | 8724 | 99 | 0 |
| line_comment | /// stale contract — the default `command_from_intent`  | 8725 | 8725 | 88 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8726 | 8726 | 43 | 0 |
| function_item | command_from_intent_rejects_a_non_v1_action_version | 8727 | 8741 | 855 | 1 |
| line_comment | //#endregion 🧪️IntentDispatchTests | 8742 | 8742 | 39 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8744 | 8744 | 43 | 0 |
| function_item | set_active_utility_carries_its_value_directly_and_emits_no_operations | 8745 | 8766 | 2329 | 2 |
| line_comment | /// 🧷️ A streamed gesture is one undo step on the type | 8768 | 8768 | 125 | 0 |
| line_comment | /// only reverts the last one. | 8769 | 8769 | 30 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8770 | 8770 | 43 | 0 |
| function_item | a_streamed_gesture_is_one_undo_step_while_each_commit_is_its_own | 8771 | 8801 | 2632 | 4 |
| line_comment | /// 🪢️ Regression guard for `result_from_last_edit`'s  | 8803 | 8803 | 97 | 0 |
| line_comment | /// transaction's open edit accumulates every tick's op | 8804 | 8804 | 100 | 0 |
| line_comment | /// `InvocationResult` must report only the operation I | 8805 | 8805 | 102 | 0 |
| line_comment | /// growing edit into every `KernelMutation`/`UndoGroup | 8806 | 8806 | 82 | 0 |
| line_comment | /// | 8807 | 8807 | 3 | 0 |
| line_comment | /// The narrowed per-dispatch reporting must not affect | 8808 | 8808 | 103 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8809 | 8809 | 43 | 0 |
| function_item | a_streamed_tick_reports_only_its_own_new_operation | 8810 | 8843 | 3373 | 7 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8845 | 8845 | 43 | 0 |
| function_item | operation_command_emits_kernel_op_with_true_inverse | 8846 | 8868 | 2388 | 4 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8870 | 8870 | 43 | 0 |
| function_item | manifest_command_dispatch_validates_structural_app_ownership | 8871 | 8896 | 2798 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8898 | 8898 | 43 | 0 |
| function_item | ui_dispatch_backstop_rejects_every_non_migrated_action_and_command | 8899 | 8939 | 4465 | 6 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8941 | 8941 | 43 | 0 |
| function_item | manifest_mode_command_requires_the_active_structural_owner | 8942 | 8969 | 2723 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8971 | 8971 | 43 | 0 |
| function_item | addressed_window_action_injects_the_exact_window_instance_into_the_typed_handler | 8972 | 8997 | 2767 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 8999 | 8999 | 43 | 0 |
| function_item | plugin_command_handler_is_program_owned_across_app_instances | 9000 | 9029 | 1937 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9031 | 9031 | 43 | 0 |
| function_item | command_op_records_history_exactly_like_an_operation_action | 9032 | 9054 | 2229 | 2 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9056 | 9056 | 43 | 0 |
| function_item | registry_less_construction_rejects_before_the_reducer | 9057 | 9073 | 2167 | 2 |
| line_comment | //#region 🔖️InteractionDispatchTests | 9075 | 9075 | 41 | 0 |
| line_comment | /// 🕹️ `interaction_topology` requires a non-empty `la | 9076 | 9076 | 86 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9077 | 9077 | 43 | 0 |
| function_item | interaction_select_replace_persists_through_the_interaction_store | 9078 | 9100 | 2385 | 2 |
| line_comment | /// 🐁️ Empty targets clears the channel (see `next_hov | 9102 | 9102 | 91 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9103 | 9103 | 43 | 0 |
| function_item | interaction_hover_is_ephemeral_and_never_touches_the_persisted_interaction_store | 9104 | 9132 | 3044 | 3 |
| line_comment | /// 🕰️ The framework-injected "undo" action only ever  | 9134 | 9134 | 95 | 0 |
| line_comment | /// DOCUMENT store) — with only the label-seed edit on  | 9135 | 9135 | 97 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9136 | 9136 | 43 | 0 |
| function_item | a_pick_is_never_undoable_the_default_undo_only_ever_walks_the_document_store | 9137 | 9161 | 2729 | 3 |
| line_comment | /// 🛂️ An undeclared granularity is rejected, not sile | 9163 | 9163 | 104 | 0 |
| line_comment | /// a framework-reserved verb (`FrameworkSetInteraction | 9164 | 9164 | 97 | 0 |
| line_comment | /// reserved job's commit, exactly where `reserved_acti | 9165 | 9165 | 90 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9166 | 9166 | 43 | 0 |
| function_item | set_selection_mode_and_set_interaction_granularity_persist_immediately | 9167 | 9193 | 2847 | 3 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9195 | 9195 | 43 | 0 |
| function_item | clear_selection_and_select_all_apply_across_every_declared_domain | 9196 | 9220 | 2553 | 2 |
| line_comment | /// 🧹️ `TestApp::interaction_topology` reports NO ids  | 9222 | 9222 | 100 | 0 |
| line_comment | /// "item-1 was deleted from the document" — task 4: re | 9223 | 9223 | 97 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9224 | 9224 | 43 | 0 |
| function_item | validate_state_prunes_a_stale_selection_id_after_the_document_deletes_it | 9225 | 9249 | 2633 | 2 |
| line_comment | /// 🙈️ Interaction verbs (pick, hover, clear selection | 9251 | 9251 | 111 | 0 |
| line_comment | /// only the seeding edit stays in the log. | 9252 | 9252 | 43 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9253 | 9253 | 43 | 0 |
| function_item | interaction_verbs_never_become_history_rows | 9254 | 9278 | 2696 | 1 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9280 | 9280 | 43 | 0 |
| function_item | build_definition_rejects_transitive_flat_interaction | 9281 | 9301 | 1397 | 1 |
| line_comment | /// 👥️ Contract-freeze §C7.6 peer setup — M2 (ticket 2 | 9303 | 9303 | 96 | 0 |
| line_comment | /// this the real presence-derivation fixture the prior | 9304 | 9304 | 90 | 0 |
| line_comment | /// | 9305 | 9305 | 3 | 0 |
| line_comment | /// 👥️ M2: `stamp_and_cache_interaction_ui` no longer  | 9306 | 9306 | 95 | 0 |
| line_comment | /// tree itself (`TreeNode`/`Component::TreeItem` still | 9307 | 9307 | 92 | 0 |
| line_comment | /// method's own doc comment for why "the framework win | 9308 | 9308 | 90 | 0 |
| line_comment | /// it derives `ui_contract::PresenceUpdate`s into `sel | 9309 | 9309 | 92 | 0 |
| line_comment | /// below. | 9310 | 9310 | 10 | 0 |
| line_comment | /// | 9311 | 9311 | 3 | 0 |
| line_comment | /// 👥️ M2 acceptance: selecting item-1 (own) with alic | 9312 | 9312 | 94 | 0 |
| line_comment | /// one `PresenceUpdate` for that node — own.selected t | 9313 | 9313 | 86 | 0 |
| line_comment | /// threaded through, with the app-local body key befor | 9314 | 9314 | 81 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9315 | 9315 | 43 | 0 |
| function_item | ui_tree_stamping_caches_interaction_topology_from_a_domain_bound_tree | 9316 | 9399 | 5567 | 14 |
| line_comment | /// 🧹️ A selection a later render DROPS must be retire | 9401 | 9401 | 104 | 0 |
| line_comment | /// reports only marked nodes and `PresenceHub::record_ | 9402 | 9402 | 90 | 0 |
| line_comment | /// overwritten, so without `retire_dropped_presence` t | 9403 | 9403 | 99 | 0 |
| line_comment | /// for the life of the session on BOTH renderers — the | 9404 | 9404 | 103 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9405 | 9405 | 43 | 0 |
| function_item | a_dropped_selection_publishes_a_cleared_presence_update | 9406 | 9473 | 5429 | 4 |
| line_comment | /// 🧪 W-G3 §8.21 — `interactionSelect` leftover `Invoc | 9475 | 9475 | 118 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9476 | 9476 | 43 | 0 |
| function_item | interaction_select_job_completion_publishes_interaction_view_on_leftover | 9477 | 9501 | 2884 | 4 |
| line_comment | /// 🧪 W-G3 §8.30 — leftover `interactionSelect` of an  | 9503 | 9503 | 130 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9504 | 9504 | 43 | 0 |
| function_item | leftover_interaction_select_object_id_on_vortex_domain_lands_in_selected_ids | 9505 | 9529 | 2345 | 1 |
| line_comment | /// 🧪 W-G3 §8.21 — `interactionHover` leftover publish | 9531 | 9531 | 108 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9532 | 9532 | 43 | 0 |
| function_item | interaction_hover_job_completion_publishes_hover_target_on_leftover | 9533 | 9553 | 2308 | 2 |
| line_comment | /// 🐁️ Wave B34 — the plugin-host half of the app-decl | 9555 | 9555 | 106 | 0 |
| line_comment | /// dirties ONLY the hover lane of its own window. The  | 9556 | 9556 | 100 | 0 |
| line_comment | /// for all six `INTERACTION_ACTION_IDS`, hover include | 9557 | 9557 | 95 | 0 |
| line_comment | /// window body, every panel body, the utilities/tools/ | 9558 | 9558 | 101 | 0 |
| line_comment | /// — measured at 24 interaction ingresses and 3 whole- | 9559 | 9559 | 96 | 0 |
| line_comment | /// (ticket 26/09/02 `📓️2026-09-12-wave-B32-world-lane | 9560 | 9560 | 88 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9561 | 9561 | 43 | 0 |
| function_item | interaction_hover_dirties_only_the_hovering_windows_own_lane | 9562 | 9584 | 2529 | 3 |
| line_comment | /// 🖱️ Wave B34 — the same hop for a PICK: the declari | 9586 | 9586 | 105 | 0 |
| line_comment | /// panel bodies included, so an app can widen a select | 9587 | 9587 | 98 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9588 | 9588 | 43 | 0 |
| function_item | interaction_select_carries_the_apps_declared_selection_lane | 9589 | 9618 | 2449 | 1 |
| line_comment | /// 🈳️ The framework DEFAULT: a verb the app declares  | 9620 | 9620 | 98 | 0 |
| line_comment | /// DERIVED from the app's own surface declarations (`i | 9621 | 9621 | 94 | 0 |
| line_comment | /// `WindowKindDefinition.interactions`) — the window t | 9622 | 9622 | 99 | 0 |
| line_comment | /// and no panel, because `setInteractionGranularity` m | 9623 | 9623 | 92 | 0 |
| line_comment | /// nothing for this verb precisely so this arm has a f | 9624 | 9624 | 94 | 0 |
| line_comment | /// reached only when not even a window declares the to | 9625 | 9625 | 92 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9626 | 9626 | 43 | 0 |
| function_item | an_undeclared_interaction_verb_falls_through_to_the_declared_surface_scope | 9627 | 9648 | 2306 | 1 |
| line_comment | /// 🌀️ The widest, always-correct answer is what is le | 9650 | 9650 | 102 | 0 |
| line_comment | /// window kind declares the touched domain, so the fra | 9651 | 9651 | 97 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9652 | 9652 | 43 | 0 |
| function_item | a_domain_no_window_declares_keeps_the_framework_full_scope | 9653 | 9662 | 871 | 3 |
| line_comment | /// 🎮️ Wave B34 — the verb type the declaration is key | 9664 | 9664 | 89 | 0 |
| line_comment | /// `INTERACTION_ACTION_IDS` intercepts, in both direct | 9665 | 9665 | 99 | 0 |
| line_comment | /// without an author deciding what it dirties. | 9666 | 9666 | 47 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9667 | 9667 | 43 | 0 |
| function_item | every_intercepted_interaction_id_round_trips_through_its_verb | 9668 | 9677 | 594 | 4 |
| line_comment | /// 🧪 Empty-target `interactionSelect` with `replace`  | 9679 | 9679 | 128 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9680 | 9680 | 43 | 0 |
| function_item | emitted_empty_replace_write_clears_selection_while_hover_remains | 9681 | 9704 | 2889 | 3 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9706 | 9706 | 43 | 0 |
| function_item | empty_target_interaction_select_clears_selection_while_hover_remains | 9707 | 9730 | 2822 | 2 |
| line_comment | /// 🔀️ The exact effect every app's `interaction_selec | 9732 | 9732 | 103 | 0 |
| line_comment | /// to replay one framework interaction verb. | 9733 | 9733 | 45 | 0 |
| function_item | inline_interaction_select | 9734 | 9736 | 166 | 0 |
| line_comment | /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — a | 9738 | 9738 | 107 | 0 |
| line_comment | /// in the SAME turn as the app command that carried it | 9739 | 9739 | 95 | 0 |
| line_comment | /// selection snapshot already moved when the carrier's | 9740 | 9740 | 98 | 0 |
| line_comment | /// widened by the verb's app-declared interaction refr | 9741 | 9741 | 100 | 0 |
| line_comment | /// rides the carrier's own `output` (the lane a host-d | 9742 | 9742 | 96 | 0 |
| line_comment | /// effect still reaches the host untouched, in order.  | 9743 | 9743 | 97 | 0 |
| line_comment | /// host and the pick landed one guest round trip later | 9744 | 9744 | 99 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9745 | 9745 | 43 | 0 |
| function_item | guest_emitted_interaction_select_is_folded_into_the_carrying_turn | 9746 | 9778 | 3713 | 6 |
| line_comment | /// 🔀️ A second carrier whose folded `interactionSelec | 9780 | 9780 | 103 | 0 |
| line_comment | /// selection (the background-click deselect), and its  | 9781 | 9781 | 94 | 0 |
| line_comment | /// partial scope and the verb's declared partial scope | 9782 | 9782 | 97 | 0 |
| line_comment | /// refresh is dropped. | 9783 | 9783 | 23 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9784 | 9784 | 43 | 0 |
| function_item | guest_emitted_empty_replace_select_clears_and_unions_the_carrier_scope | 9785 | 9816 | 4067 | 8 |
| line_comment | /// 🔀️ A folded verb that cannot be applied (an undecl | 9818 | 9818 | 98 | 0 |
| line_comment | /// command that carried it — the pick already happened | 9819 | 9819 | 98 | 0 |
| line_comment | /// carrier's result under `INLINE_INTERACTION_VERB_DIA | 9820 | 9820 | 89 | 0 |
| line_comment | /// consumed (the host is never asked to retry a verb t | 9821 | 9821 | 96 | 0 |
| line_comment | /// untouched. A `Full` carrier scope stays `Full`. | 9822 | 9822 | 51 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9823 | 9823 | 43 | 0 |
| function_item | guest_emitted_interaction_select_on_an_undeclared_domain_is_a_diagnostic_not_a_fault | 9824 | 9848 | 2968 | 6 |
| line_comment | /// 🔀️ The fold is scoped to the six interaction verbs | 9850 | 9850 | 101 | 0 |
| line_comment | /// shell relay riding the same `ReplayShellCommand` va | 9851 | 9851 | 95 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9852 | 9852 | 43 | 0 |
| function_item | non_interaction_replay_shell_commands_still_reach_the_host | 9853 | 9873 | 2365 | 3 |
| line_comment | /// 🔀️ The second emission shape for the same intent — | 9875 | 9875 | 98 | 0 |
| line_comment | /// INTERACTION_ACTION_IDS }`, what `📏️layout`'s `layo | 9876 | 9876 | 101 | 0 |
| line_comment | /// `wires_select_effect` build — folds identically to  | 9877 | 9877 | 92 | 0 |
| line_comment | /// effect never reaches the host, the selection snapsh | 9878 | 9878 | 93 | 0 |
| line_comment | /// carrier's scope widens to the verb's declared inter | 9879 | 9879 | 89 | 0 |
| line_comment | /// `InteractionView` rides the carrier's `output`, and | 9880 | 9880 | 97 | 0 |
| line_comment | /// The placeholder `req` an app literal carries is awa | 9881 | 9881 | 97 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9882 | 9882 | 43 | 0 |
| function_item | guest_emitted_dispatch_action_interaction_select_is_folded_into_the_carrying_turn | 9883 | 9912 | 3818 | 6 |
| line_comment | /// 🔀️ The `DispatchAction` fold is scoped to the six  | 9914 | 9914 | 93 | 0 |
| line_comment | /// `ReplayShellCommand` one: a self-dispatch chain (`p | 9915 | 9915 | 88 | 0 |
| line_comment | /// staged-work tick) riding the same variant keeps rea | 9916 | 9916 | 90 | 0 |
| line_comment | /// `delay_ms` intact. | 9917 | 9917 | 22 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9918 | 9918 | 43 | 0 |
| function_item | non_interaction_dispatch_actions_still_reach_the_host | 9919 | 9941 | 2649 | 4 |
| line_comment | /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — t | 9943 | 9943 | 100 | 0 |
| line_comment | /// ladder, observed the way the browser host observes  | 9944 | 9944 | 91 | 0 |
| line_comment | /// drains one effect per turn from `typed_effect_outbo | 9945 | 9945 | 94 | 0 |
| line_comment | /// on NO turn of a typed app command does a guest-emit | 9946 | 9946 | 96 | 0 |
| line_comment | /// an effect. The emit is fem2d's `canvasPointerUp` sh | 9947 | 9947 | 89 | 0 |
| line_comment | /// `interactionSelect`, a partial window refresh — bec | 9948 | 9948 | 92 | 0 |
| line_comment | /// `publish_mounted_typed_operation_unit` installed th | 9949 | 9949 | 95 | 0 |
| line_comment | /// through to `emit.effects.pop()` → `typed_effect_out | 9950 | 9950 | 82 | 0 |
| line_comment | /// (`mounted_typed_inline_interaction_verbs_are_next`) | 9951 | 9951 | 93 | 0 |
| line_comment | /// and answered false on a still-`None` publication. M | 9952 | 9952 | 95 | 0 |
| line_comment | /// {"actionId":"interactionSelect"}` host lines per ba | 9953 | 9953 | 85 | 0 |
| line_comment | /// | 9954 | 9954 | 3 | 0 |
| line_comment | /// Laws asserted per turn and at the end: the verb is  | 9955 | 9955 | 95 | 0 |
| line_comment | /// result page is ever published; the selection snapsh | 9956 | 9956 | 94 | 0 |
| line_comment | /// progress scope the host receives is already WIDER t | 9957 | 9957 | 95 | 0 |
| line_comment | /// fold ran before the `Ui` page was minted); the fold | 9958 | 9958 | 95 | 0 |
| line_comment | /// same progress unit; the operation retires through ` | 9959 | 9959 | 91 | 0 |
| line_comment | /// Fails-before: the first turn after admission yielde | 9960 | 9960 | 82 | 0 |
| line_comment | /// | 9961 | 9961 | 3 | 0 |
| line_comment | /// 📡️ EXACTLY the host's per-turn effect drain — ever | 9962 | 9962 | 90 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 9963 | 9963 | 43 | 0 |
| function_item | typed_ladder_never_hands_a_guest_interaction_verb_to_the_host_on_any_turn | 9964 | 10072 | 9068 | 0 |
| line_comment | /// 🕹️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — t | 10074 | 10074 | 105 | 0 |
| line_comment | /// owes the host rides the `Invocation` reply of the c | 10075 | 10075 | 89 | 0 |
| line_comment | /// (`merge_typed_operation_leftover_into_reply`, run b | 10076 | 10076 | 97 | 0 |
| line_comment | /// the reply's `{ operationId, generation }` witness k | 10077 | 10077 | 98 | 0 |
| line_comment | /// the envelope `interactionViewFromLeftoverOutput` pe | 10078 | 10078 | 95 | 0 |
| line_comment | /// answers another operation, an unsolicited (`in_repl | 10079 | 10079 | 93 | 0 |
| line_comment | /// frame are left byte-identical. Fails-before: the ty | 10080 | 10080 | 97 | 0 |
| line_comment | /// host's leftover overlay / Inspection tab never move | 10081 | 10081 | 75 | 0 |
| attribute_item | #[test] | 10082 | 10082 | 7 | 0 |
| function_item | typed_operation_leftover_merges_into_exactly_the_carrying_reply | 10083 | 10102 | 2747 | 6 |
| line_comment | /// 🧪 Ticket 26/09/09/PROCEDURAL-3D-END-TO-END (`📓️se | 10103 | 10103 | 105 | 0 |
| line_comment | /// `selectedIds` publication is a SET of topology ids  | 10104 | 10104 | 94 | 0 |
| line_comment | /// idempotent per id whatever the merge, and however m | 10105 | 10105 | 96 | 0 |
| line_comment | /// walks. Fixture-driven off `🧫️fixtures/🕹️selection | 10106 | 10106 | 109 | 0 |
| line_comment | /// Node twin (`interactionSelectionSetOracle`) re-deri | 10107 | 10107 | 92 | 0 |
| line_comment | /// Fails-before: one plain pick published the id twice | 10108 | 10108 | 83 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 10109 | 10109 | 43 | 0 |
| function_item | leftover_interaction_view_selected_ids_are_a_set_for_every_merge | 10110 | 10152 | 4420 | 5 |
| line_comment | /// 🎯️ Ticket 26/09/09/PROCEDURAL-3D-END-TO-END (`📓️s | 10153 | 10153 | 109 | 0 |
| line_comment | /// DOMAIN path — the reserved `interactionSelect` job  | 10154 | 10154 | 103 | 0 |
| line_comment | /// accepts exactly the five words of `🕹️interaction`' | 10155 | 10155 | 101 | 0 |
| line_comment | /// else. Fixture-driven off `🕹️interaction/🧫️fixture | 10156 | 10156 | 113 | 0 |
| line_comment | /// (`selectionMergeVocabularyOracle`) and the reducer  | 10157 | 10157 | 79 | 0 |
| line_comment | /// `next_selection_obeys_the_merge_vocabulary_fixture_ | 10158 | 10158 | 81 | 0 |
| line_comment | /// | 10159 | 10159 | 3 | 0 |
| line_comment | /// Fails-before: the renderer host translated the reso | 10160 | 10160 | 99 | 0 |
| line_comment | /// shift-click / ctrl-click on the generation3d previe | 10161 | 10161 | 98 | 0 |
| line_comment | /// 'add'` and only the unmodified `replace` pick worke | 10162 | 10162 | 57 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 10163 | 10163 | 43 | 0 |
| function_item | interaction_select_speaks_exactly_the_schema_merge_vocabulary | 10164 | 10195 | 3744 | 3 |
| line_comment | //#endregion 🔖️InteractionDispatchTests | 10197 | 10197 | 44 | 0 |
| line_comment | //#region 🔖️AsyncTaskTests | 10199 | 10199 | 31 | 0 |
| line_comment | // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design- | 10200 | 10200 | 100 | 0 |
| line_comment | // event routing (`Event::Completed`, `run_until_idle`, | 10201 | 10201 | 80 | 0 |
| line_comment | // wasm32-wasip2-only (`wit_bridge`) and cannot run und | 10202 | 10202 | 92 | 0 |
| line_comment | // exercise the SAME underlying primitives `poll` itsel | 10203 | 10203 | 96 | 0 |
| line_comment | // `⚛️reactor::reactor_driver::{run_until_idle, resolve | 10204 | 10204 | 96 | 0 |
| line_comment | // `crate::plugin_runtime::plugin_resume_task` — direct | 10205 | 10205 | 90 | 0 |
| line_comment | // glue: `reactor_driver::resolve_request` IS what `Eve | 10206 | 10206 | 79 | 0 |
| line_comment | // (`RequestRegistry::resolve`), and `pop_task_resume`/ | 10207 | 10207 | 88 | 0 |
| line_comment | // `drain_task_resumes` minus the frame-routing tail (` | 10208 | 10208 | 92 | 0 |
| line_comment | // this packet). An honest, named gap: the wasm-only ta | 10209 | 10209 | 84 | 0 |
| line_comment | /// 🌱️ A command spawns a task; the task awaits a REAL | 10211 | 10211 | 91 | 0 |
| line_comment | /// (genuinely parked on `RequestRegistry`, not a synch | 10212 | 10212 | 90 | 0 |
| line_comment | /// completion (`reactor_driver::resolve_request`, stan | 10213 | 10213 | 86 | 0 |
| line_comment | /// resolves it; the follow-up `TaskResolution::Command | 10214 | 10214 | 88 | 0 |
| line_comment | /// the task's CLONED originating `ActionMeta`. | 10215 | 10215 | 47 | 0 |
| line_comment | /// | 10216 | 10216 | 3 | 0 |
| line_comment | /// 🪪️ This law dispatches under its OWN instance, not | 10217 | 10217 | 98 | 0 |
| line_comment | /// is refused with `interactive-job.live-instance` unl | 10218 | 10218 | 94 | 0 |
| line_comment | /// is exactly `meta.instance_id`, and `spawn_task` is  | 10219 | 10219 | 78 | 0 |
| line_comment | /// | 10220 | 10220 | 3 | 0 |
| line_comment | /// 🪜️ A migrated verb's `dispatch_typed` only ADMITS: | 10221 | 10221 | 92 | 0 |
| line_comment | /// operation, and every lane of it — the task lane inc | 10222 | 10222 | 90 | 0 |
| line_comment | /// publication ladder, one lane per unit. Asserting th | 10223 | 10223 | 91 | 0 |
| line_comment | /// that a migrated command never spawns. | 10224 | 10224 | 41 | 0 |
| line_comment | /// | 10225 | 10225 | 3 | 0 |
| line_comment | /// 🪪️ `spawn_task` is keyed off `meta.instance_id` (t | 10226 | 10226 | 95 | 0 |
| line_comment | /// `mounted.meta`, i.e. `spawn_meta` above, which shar | 10227 | 10227 | 94 | 0 |
| line_comment | /// | 10228 | 10228 | 3 | 0 |
| line_comment | /// ▶️ First poll: the task runs up to its `.await` on  | 10229 | 10229 | 96 | 0 |
| line_comment | /// genuinely pending, not synchronously resolved. | 10230 | 10230 | 50 | 0 |
| line_comment | /// | 10231 | 10231 | 3 | 0 |
| line_comment | /// ✅️ Inject the "completion" (`⚛️reactor::poll`'s `Ev | 10232 | 10232 | 98 | 0 |
| line_comment | /// same `RequestRegistry::resolve`) — request id 1 is  | 10233 | 10233 | 89 | 0 |
| line_comment | /// only) request this test has allocated. | 10234 | 10234 | 42 | 0 |
| line_comment | /// | 10235 | 10235 | 3 | 0 |
| line_comment | /// 🔀️ Re-enter through the SAME typed-command path a  | 10236 | 10236 | 89 | 0 |
| line_comment | /// `plugin_runtime::plugin_resume_task`'s `TaskResumeI | 10237 | 10237 | 84 | 0 |
| line_comment | /// directly rather than through `crate::reactor::drain | 10238 | 10238 | 82 | 0 |
| line_comment | /// | 10239 | 10239 | 3 | 0 |
| line_comment | /// 🪜️ The follow-up re-enters the APPLYING lane (`Plu | 10240 | 10240 | 91 | 0 |
| line_comment | /// `dispatch_typed`), so like every migrated dispatch  | 10241 | 10241 | 86 | 0 |
| line_comment | /// advances when its publication ladder walks. Settle  | 10242 | 10242 | 91 | 0 |
| line_comment | /// host's continuation drives, then read what it actua | 10243 | 10243 | 63 | 0 |
| line_comment | /// | 10244 | 10244 | 3 | 0 |
| line_comment | /// 🧾️ And it landed as the app's own verb in the comm | 10245 | 10245 | 89 | 0 |
| line_comment | /// (`handle_command_frame`, the agent prepare lane thi | 10246 | 10246 | 85 | 0 |
| line_comment | /// nothing at all and applies nothing. | 10247 | 10247 | 39 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 10248 | 10248 | 43 | 0 |
| function_item | a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta | 10249 | 10319 | 7773 | 18 |
| line_comment | /// 🧵️ Defers the original task factory until the acto | 10321 | 10321 | 87 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 10322 | 10322 | 43 | 0 |
| function_item | task_admission_defers_the_user_factory_until_actor_execution | 10323 | 10341 | 1621 | 4 |
| line_comment | /// 🧵️ Test-only typed task source: the closure is the | 10343 | 10343 | 105 | 0 |
| struct_item | TestTaskSource | 10344 | 10344 | 28 | 0 |
| struct_item | TestTaskDrop | 10346 | 10346 | 58 | 0 |
| impl_item | impl<F: Send + 'static> semio_framework_value::retireme | 10348 | 10363 | 1219 | 0 |
| impl_item | impl<F: Send + 'static> semio_framework_value::retireme | 10365 | 10369 | 431 | 0 |
| impl_item | impl<F, Fut> AsyncTaskSource<TestMutation, TestConfigMu | 10371 | 10378 | 405 | 0 |
| function_item | test_task | 10380 | 10386 | 537 | 0 |
| line_comment | /// 🚫️ The (quota+1)th task on one instance is refused | 10388 | 10388 | 92 | 0 |
| line_comment | /// silent drop — while earlier tasks and OTHER instanc | 10389 | 10389 | 75 | 0 |
| line_comment | /// | 10390 | 10390 | 3 | 0 |
| line_comment | /// 🔓️ A different instance has its OWN quota accounti | 10391 | 10391 | 94 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 10392 | 10392 | 43 | 0 |
| function_item | spawn_task_quota_gate_faults_the_n_plus_1th_task_and_never_silently_drops_it | 10393 | 10415 | 1754 | 4 |
| line_comment | /// 🔑️ Spawning a second task under the SAME `(instanc | 10417 | 10417 | 93 | 0 |
| line_comment | /// future is dropped (never runs to completion, never  | 10418 | 10418 | 90 | 0 |
| line_comment | /// index tracks only the NEW task afterward. | 10419 | 10419 | 45 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 10420 | 10420 | 43 | 0 |
| function_item | key_dedupe_cancels_the_previously_live_task_under_the_same_key | 10421 | 10449 | 1958 | 6 |
| line_comment | /// 🚫️ `Event::InstanceClose` cancellation: every task | 10451 | 10451 | 96 | 0 |
| line_comment | /// executor (never runs to completion) and its pending | 10452 | 10452 | 90 | 0 |
| line_comment | /// — no leaked slot — while a DIFFERENT instance's liv | 10453 | 10453 | 87 | 0 |
| line_comment | /// | 10454 | 10454 | 3 | 0 |
| line_comment | /// parks forever in this test | 10455 | 10455 | 30 | 0 |
| line_comment | /// | 10456 | 10456 | 3 | 0 |
| line_comment | /// 🚫️ `cancel_instance_tasks` alone drops the future  | 10457 | 10457 | 90 | 0 |
| line_comment | /// with it); `RequestRegistry::cancel_instance` is the | 10458 | 10458 | 87 | 0 |
| line_comment | /// `Event::InstanceClose` arm runs right after — exerc | 10459 | 10459 | 86 | 0 |
| line_comment | /// together leave no slot behind, matching that call s | 10460 | 10460 | 67 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 10461 | 10461 | 43 | 0 |
| function_item | instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot | 10462 | 10498 | 2557 | 8 |
| line_comment | /// 📸️ A task's `restart` command survives into the ch | 10500 | 10500 | 97 | 0 |
| line_comment | /// design-abi.md §4) and `restore_now` queues it as an | 10501 | 10501 | 93 | 0 |
| line_comment | /// `plugin_resume_task` on the very next turn — exerci | 10502 | 10502 | 87 | 0 |
| line_comment | /// `TestApp` instance, exactly like a live task's own  | 10503 | 10503 | 62 | 0 |
| line_comment | /// | 10504 | 10504 | 3 | 0 |
| line_comment | /// 🚫️ The in-flight task (and its parked request) bel | 10505 | 10505 | 95 | 0 |
| line_comment | /// never resumed as though the host round-trip were st | 10506 | 10506 | 84 | 0 |
| line_comment | /// `restore_now` below is what re-arms it, as a fresh  | 10507 | 10507 | 85 | 0 |
| line_comment | /// The order is the close ladder's own and the complet | 10508 | 10508 | 67 | 0 |
| line_comment | /// `await-request-retirement-before-observation`: the  | 10509 | 10509 | 87 | 0 |
| line_comment | /// (`ReactorCloseState::requests_complete` precedes `t | 10510 | 10510 | 92 | 0 |
| line_comment | /// at that moment can only ever observe its request as | 10511 | 10511 | 94 | 0 |
| line_comment | /// response — and `cancel_instance_tasks` then DROPS w | 10512 | 10512 | 92 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 10513 | 10513 | 43 | 0 |
| function_item | checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume | 10514 | 10559 | 4417 | 12 |
| attribute_item | #[test] | 10561 | 10561 | 7 | 0 |
| function_item | checkpoint_restart_mode_requires_its_exact_concrete_factory_owner | 10562 | 10570 | 1165 | 6 |
| line_comment | //#endregion 🔖️AsyncTaskTests | 10571 | 10571 | 34 | 0 |
| line_comment | // 🪟️ Included rather than a sibling module: the surfa | 10573 | 10573 | 98 | 0 |
| line_comment | // `plugin_mount_surface`/`plugin_render_surface`, and  | 10574 | 10574 | 89 | 0 |
| line_comment | // `RENDER_CONTEXT_PROBE`, `reserved_action`, `interact | 10575 | 10575 | 91 | 0 |
| macro_invocation | include!("../🔬️surface-view-state-routing/🦀️.rs") | 10576 | 10576 | 59 | 0 |
| empty_statement | ; | 10576 | 10576 | 1 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 10577 | 10577 | 43 | 0 |
| function_item | mounted_command_history_original_pages_retain_exact_app_close_authority | 10578 | 10590 | 1365 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 10591 | 10591 | 43 | 0 |
| function_item | mounted_command_history_document_replacement_keeps_original_held_rows | 10592 | 10605 | 1641 | 0 |
| attribute_item | #[semio_framework_async_macros::async_test] | 10606 | 10606 | 43 | 0 |
| function_item | mounted_private_child_frame_retains_exact_admission_and_close_authority | 10607 | 10624 | 1976 | 0 |
| mod_item | mounted_owned_child_laws | 10625 | 10625 | 83 | 0 |
| mod_item | owned_child_dispatch_laws | 10626 | 10626 | 85 | 0 |
| line_comment | /// 👷️ Original reserved route returns physical backin | 10628 | 10628 | 98 | 0 |
| attribute_item | #[test] | 10629 | 10629 | 7 | 0 |
| function_item | original_reserved_job_currency_law_retains_spare_backing_until_exact_release | 10630 | 10649 | 3977 | 26 |
