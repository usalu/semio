//! 🎛️ Norm plugin — the app-surface machinery every one of the fifteen compliance apps shares.
//!
//! 📌️ The fifteen norm apps are structurally identical by construction (one `edit` mode, an
//! inputs/results window pair, the framework document/catalogue/inspection panel trio, the same
//! `model:in`/`report:out` media ports, the same three commands) and differ only in their per-standard
//! `Document` type, ids and labels. Everything that does NOT vary lives here, ONCE; every taxonomy node
//! under each subset's `✏️editor/`/`👁️viewer/` states only what genuinely varies and calls into this module. That is the
//! "shared declarations belong at the shallowest common ancestor" rule taken to its conclusion — the
//! shallowest common ancestor of fifteen sibling apps is the plugin's own `🫀️core`.
//!
//! Nothing here depends on any app or artifact module: every entry point is either a plain constructor
//! or generic over the artifact's `Document`/`NormFamily`, so `🫀️core` stays a leaf of the dependency
//! graph exactly as the artifacts require.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

/// 🧵️ Retained norm command output with empty application config and draft lanes.
pub type NormRetainedCommandResult<M> = Result<Emit<M, NoConfigMutation, semio_framework_plugin::NoDraftMutation>, Fault>;

use semio_s_plugin_norm_compliance::{cached_report_for, document_revision_key, invalidate_cached_report_for, store_cached_report_for, CheckReport, CheckResult, CheckStatus, ClauseId, NormFamily, NormHost, Quantity, QuantityKind, RemedyBound};
#[cfg(test)] use semio_s_plugin_norm_compliance::{LocalizedCopy, Remedy};
use semio_framework::ToolExecutionContract;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind};
use semio_framework_ui_contract as ui;
use semio_framework_plugin::tree_group;
use semio_framework_plugin::tree_item_desc;
use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::tree_window_section_or_placeholder;
use semio_framework_plugin::ui_node_list;
use semio_framework_plugin::ActionFactory;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactKindSpec;
use semio_framework_plugin::ArtifactPresentation;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::ExampleSource;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaPortDirection;
use semio_framework_plugin::MediaPortSpec;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::OsMediaCapability;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::PortMultiplicity;
use semio_framework_plugin::SurfaceKind;
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::Trigger;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiFixedList;
use semio_framework_plugin::UiMapBuilder;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::WindowConfigOwner;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowLayout;
use semio_framework_plugin::WindowLayoutRoot;
use semio_framework_plugin::WindowLayoutStackNode;
use semio_framework_plugin::WindowLayoutWindowNode;
use semio_framework_plugin::WindowOptions;

#[path = "🛡️store-ownership-macros/🦀️.rs"]
mod store_ownership_macros;

#[path = "🏷️ids-chrome-and-labels/🦀️.rs"]
mod ids_chrome_and_labels;
pub use ids_chrome_and_labels::*;

#[path = "🧭️value-path/🦀️.rs"]
mod value_path;
pub use value_path::*;

#[path = "🔖️field-meta/🦀️.rs"]
mod field_meta;
pub use field_meta::*;

#[path = "✏️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::*;

#[path = "🌳️report-tree/🦀️.rs"]
mod report_tree;
pub use report_tree::*;

#[path = "📝️document-editors/🦀️.rs"]
mod document_editors;
pub use document_editors::*;

#[path = "📚️catalogue-and-inspection/🦀️.rs"]
mod catalogue_and_inspection;
pub use catalogue_and_inspection::*;

#[path = "🪟️manifest-and-media/🦀️.rs"]
mod manifest_and_media;
pub use manifest_and_media::*;

#[path = "⌨️command-args-and-dispatch/🦀️.rs"]
mod command_args_and_dispatch;
pub use command_args_and_dispatch::*;

#[path = "🏭️command-and-tool-job-macros/🦀️.rs"]
mod command_and_tool_job_macros;

#[path = "🧵️retained-commands/🦀️.rs"]
mod retained_commands;
pub use retained_commands::*;

#[cfg(any(test, feature = "compliance-testing"))]
//#region 🧵️RetainedDispositionOracle
#[path = "🧪️tests/🔬️retained-disposition-oracle/🦀️.rs"]
pub mod retained_disposition_oracle;
//#endregion 🧵️RetainedDispositionOracle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
