//! 🏈️ Staged `publication` members of the `instance-typed-ops` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 📬️ Typed operation publication: the mounted publication units, the latest-wins ladder and the stall witness.
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, plugin_retirement_fault};
use semio_framework::{AppRole, Fault, FaultCode, FaultFrom, FaultOrigin};
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_operation::{ArtifactDocumentAuthority, ToolCancellationLease, ToolOperationKey};
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactToolCompletionValue, ArtifactToolPublicationLane, TYPED_OPERATION_FAULT_PAGE_BYTES, TypedOperationResultLane};
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep;
use semio_framework_plugin_composition_publication::{ChildPublicationResultV1, ComposedGestureResult, MountedTypedCommandFullOperationStage, PendingArtifactStorePublication, PendingChildGroupPublication, PendingChildGroupPublicationPhase};
use semio_framework_plugin_job::{DownloadResultPayload, INTERACTIVE_TURN_WORKER_WALL_US, TYPED_OPERATION_PUBLICATION_PUMPS, TYPED_OPERATION_STALL_FAULT_CEILING, TYPED_OPERATION_STALL_PROGRESS_MIX, TYPED_OPERATION_STALL_PUBLISHING_STAGE, TYPED_OPERATION_STALL_WITNESS_FLOOR, TransactionProposalDraft, TypedOperationCompletionWitness, TypedOperationLeftover, TypedOperationResultPage, TypedOperationUnitKind, record_typed_operation_unit};
use semio_framework_value::{DslValue, ValueError};
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework_plugin_decl_emission::{Emit, TransactionPhase};
use std::future::Future;
use std::collections::HashSet;
use store::os_store::store_schema::HistoryLane;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use semio_framework_plugin_instance_docs::docs_components::MediaTransfers;
use store::{MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_retained_command::typed_operation::{MountedTypedCommandFullOperation, PendingPublicationOutcome};
use protocol::OpText;
use semio_framework_plugin_composition_publication::group::private_publication_group::owner::PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use crate::typed_bundle::TypedCtx;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_interaction::interaction_topology::take_inline_interaction_verbs;
use semio_framework_ui_contract::*;
use semio_framework_plugin_instance_docs::docs_instance_root::tool_transaction_shape_fault;
use semio_framework_plugin_instance_docs::docs_instance_root::typed_operation_document_is_fresh;
use semio_framework_plugin_instance_docs::docs_instance_root::viewer_read_only_fault;
use crate::typed_ops_instance_root::store_publication_fault;
use crate::typed_ops_plugin_app_impl::PUBLICATION_RETURNED_ROOT_ALLOWANCE;

#[path = "🔁️stall-witness-and-unit-advance/🦀️.rs"]
mod stall_witness_and_unit_advance;
pub(crate) use stall_witness_and_unit_advance::*;

#[path = "🏃️mounted-run-and-pump/🦀️.rs"]
mod mounted_run_and_pump;

#[path = "🖱️inline-interaction-and-latest-wins/🦀️.rs"]
mod inline_interaction_and_latest_wins;

#[path = "👶️child-operation-unit/🦀️.rs"]
mod child_operation_unit;

#[path = "📬️typed-operation-unit/🦀️.rs"]
mod typed_operation_unit;
