//! 🌊️ Flow artifact — the document entity this plugin's apps edit.
//!
//! The persisted snapshot type is [`FlowSnapshot`] (this plugin). The framework crate
//! `semio-framework-os-flow` still owns a separate `semio_framework_artifact_flow_flow::FlowHostSnapshot` used by `FlowHost` and by
//! other plugins (e.g. procedural) that embed a flow graph; conversions live on `FlowSnapshot`.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`flow→C:flow`, the canonical editor for
//! stdio's `flow` subset): the old inline `widgets`/`synapses`/`layout` fields are replaced by a
//! composed `s.stdio.semio@v1/flow` CHILD slot (`🔖️ContentBridge` below) — this plugin no longer
//! defines its own node-graph content model, it composes stdio's `flow` subset instead. The rich
//! live editing types (`semio_framework_artifact_flow_flow::Widget`/`semio_framework_artifact_flow_flow::SynapseSpec`/`semio_framework_artifact_flow_flow::WidgetLayout`, the framework
//! kernel's own vocabulary `FlowHost` edits) still flow entirely through `FlowSnapshot::to_host_snapshot`/
//! `from_host_snapshot`, which now bridge through the composed child + `🔖️WorkingScene` cache rather than
//! plain struct fields.

extern crate semio_framework_schema as framework_schema;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_flow_demo_tests;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_artifact_flow_flow::{Widget, WidgetLayout};
use {semio_framework_artifact_reference::Dialect, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};

use std::io::Write;
use std::sync::Arc;

pub use semio_s_artifact_flow_flow_schema::{FLOW_DOCUMENT_SCHEMA};

pub use semio_s_artifact_flow_flow_viewer::{FLOW_DIALECT};

pub use semio_s_artifact_flow_flow_schema::{FLOW_CONTENT_ID_DOMAIN, FlowContentChild, FlowContentHashWriter, FlowWorkingScene, cache_flow_content, flow_content_child_from_digest, flow_content_child_handle, flow_content_child_handle_and_cache, flow_content_child_handle_bounded, flow_content_digest, flow_content_node_from_working, flow_content_snapshot_from_working, flow_working_scene_for_handle, widget_from_node, widget_params, working_from_flow_content_snapshot};

pub use semio_s_artifact_flow_flow_schema::{flow_composed_snapshot, flow_genesis_content_pack, flow_working_scene};

pub use semio_s_artifact_flow_flow_editor::{artifact_kind};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.flow.flow")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.flow.flow")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.flow.flow")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.flow.flow.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.flow.flow.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.flow.flow@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.flow.flow@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"s.flow.flow:flow")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "s.flow.flow")?)?
                .claim(ArtifactIdentityClaim::codec_extension("s.flow.flow", "flow")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Flow")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Flow")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Flow")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Flow")?)?)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::EditorApp;
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(schema::flow_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::flow_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<EditorApp<editor::flow::FlowPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_flow_flow_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_flow_flow_diff::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_flow_flow_mutations::standards::v1::subsets::any::schema::mutations;
                    pub use semio_s_artifact_flow_flow_mutations::standards::v1::subsets::any::schema::retirement;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_flow_flow_mutations::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_flow_flow_diff::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_flow_flow_mutations::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_flow_flow_inferences::standards::v1::subsets::any::io::binary::inferences;
    pub use semio_s_artifact_flow_flow_mutations::standards::v1::subsets::any::io::binary::mutations;
    pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::io::binary::snapshot;
}
pub mod sqlite {
    pub use semio_s_artifact_flow_flow_inferences::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::io::sqlite::snapshot;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_flow_flow_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_flow_flow_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_flow_flow_tests::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_flow_flow_tests::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }

}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_flow_flow_tests::snapshot;
pub use crate::standards::v1::subsets::any::schema::diff::FlowDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::FlowMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;

pub use semio_s_artifact_flow_flow_tests::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod flow {
        pub use semio_s_artifact_flow_flow_editor::editor::flow::*;
pub use semio_s_artifact_flow_flow_inferences::editor::flow::demo_session;

        pub use semio_s_artifact_flow_flow_inferences::editor::flow::base;
        pub use semio_s_artifact_flow_flow_mutations::editor::flow::host;

        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_flow_flow_inferences::editor::flow::presence::*;

            pub use semio_s_artifact_flow_flow_inferences::editor::flow::presence::schema;
        }
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::edit_rules;
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::terminology;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::add_widget;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::connect_media_ports;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::context_menu_at;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::delete_selection;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::disconnect;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::duplicate_widget;
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::evaluate;
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::flow_eval_resolve;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::flow_eval_tick;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::focus_selection;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::move_media_node;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::node_graph_edit;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::node_graph_viewport;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::open_spotlight;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::patch_flow_widgets;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::remove_widget;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::rename_flow_widget;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::reorganize;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::replace_image;
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::run_extension_action;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_active_example;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_catalogue_sections;
            pub use semio_s_artifact_flow_flow_inferences::editor::flow::commands::set_graph_parameter;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_grid_factor;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_grid_snap_enabled;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_grid_visible;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_lod_mode;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_preview_off;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::set_proximity_distance;
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::spotlight_commit;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::toggle_extension;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::set_contributions;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_flow_flow_viewer::editor::flow::modes::edit::*;

                pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::edit::tools;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::edit::windows::main::*;

                        pub use semio_s_artifact_flow_flow_config::editor::flow::modes::edit::windows::main::config;
                        pub use semio_s_artifact_flow_flow_tests::editor::flow::modes::edit::windows::main::transient;

                        #[path = "."]
                        pub mod options {
                            pub use semio_s_artifact_flow_flow_mutations::editor::flow::modes::edit::windows::main::options::grid;
                            pub use semio_s_artifact_flow_flow_mutations::editor::flow::modes::edit::windows::main::options::lod;
                            pub use semio_s_artifact_flow_flow_mutations::editor::flow::modes::edit::windows::main::options::proximity;
                        }
                    }

                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::edit::windows::compiled;
                }
            }

            #[path = "."]
            pub mod generate {
                pub use semio_s_artifact_flow_flow_viewer::editor::flow::modes::generate::*;

                #[path = "."]
                pub mod commands {
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::add_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::remove_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::rename_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::select_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::update_generation_values;
                }

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::windows::form;
                    pub use semio_s_artifact_flow_flow_tests::editor::flow::modes::generate::windows::generations;
                    pub use semio_s_artifact_flow_flow_tests::editor::flow::modes::generate::windows::preview;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::panels::catalogue;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::panels::document;
            pub use semio_s_artifact_flow_flow_tests::editor::flow::panels::inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod flow {
        pub use semio_s_artifact_flow_flow_viewer::viewer::flow::*;

        pub use semio_s_artifact_flow_flow_inferences::viewer::flow::presence;
        pub use semio_s_artifact_flow_flow_inferences::viewer::flow::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_flow_flow_viewer::viewer::flow::modes::view::*;

                pub use semio_s_artifact_flow_flow_editor_leaves::viewer::flow::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::io::{FlowBuilderConstruction, FlowParts, FlowAnalyzerAnalysis, FlowBuilderFacets, FlowBuilder, FlowAnalyzer, FlowComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
