//! 🚪️ IO-owned value representations of generation3d native semantic records.

use semio_framework_value_derive::value_codec;
use semio_framework_value::DslValue;
use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, SynapseSpec, Widget, WidgetLayout};
use semio_framework_artifact_playbook_playbook::{FormGeneration, GenerationPlayRoot};
use std::collections::BTreeMap;
#[cfg(test)] use semio_s_artifact_procedural_3d_mutations::{create_widget, update_widget, delete_widget, connect_synapse, update_synapse, disconnect_synapse, move_widget, delete_widget_position, update_camera, change_schema, create_generation, delete_generation, rename_generation, change_generation_value, change_slider_value, drag_transforms, rotate_transforms, scale_transforms, move_nodes, change_widget_input, select_generation, change_generation_preview};

fn is_false(value: &bool) -> bool { !value }

use semio_s_artifact_procedural_3d_model::Generation3dArtifact;
use semio_s_artifact_procedural_3d_model::Generation3dPreviewCamera;

#[path = "🧊️snapshot-and-catalogue-values/🦀️.rs"]
mod snapshot_and_catalogue_values;
pub use snapshot_and_catalogue_values::*;

#[path = "🧬️mutation-delta-and-canonical-values/🦀️.rs"]
mod mutation_delta_and_canonical_values;
pub use mutation_delta_and_canonical_values::*;

use semio_s_artifact_procedural_3d_model::catalogue::Localized;
use semio_s_artifact_procedural_3d_model::catalogue::Quality;
use semio_s_artifact_procedural_3d_model::catalogue::PortType;
use semio_s_artifact_procedural_3d_model::catalogue::ShapeKind;
use semio_s_artifact_procedural_3d_model::catalogue::SelectionComponent;
use semio_s_artifact_procedural_3d_model::catalogue::Selection;
use semio_s_artifact_procedural_3d_model::catalogue::EnumOption;
use semio_s_artifact_procedural_3d_model::catalogue::Port;
use semio_s_artifact_procedural_3d_model::catalogue::PickComponent;
use semio_s_artifact_procedural_3d_model::catalogue::Pick;
use semio_s_artifact_procedural_3d_model::catalogue::GumballMotion;
use semio_s_artifact_procedural_3d_model::catalogue::Along;
use semio_s_artifact_procedural_3d_model::catalogue::Gumball;
use semio_s_artifact_procedural_3d_model::catalogue::Interaction;
use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use semio_s_artifact_procedural_3d_model::catalogue::Category;
use semio_s_artifact_procedural_3d_model::catalogue::CategoryFile;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetsDelta;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSynapsesDelta;
use semio_s_artifact_procedural_3d_model::diff::Generation3dGenerationsDelta;
use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetPatch;
use semio_s_artifact_procedural_3d_model::diff::Generation3dLayoutRow;
use semio_s_artifact_procedural_3d_model::diff::Generation3dLayoutDelta;
use semio_s_artifact_procedural_3d_model::diff::Generation3dGenerationPatch;
use semio_s_artifact_procedural_3d_model::diff::Generation3dValueRow;
use semio_s_artifact_procedural_3d_model::diff::Generation3dValuesDelta;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSelectionChange;
use semio_s_artifact_procedural_3d_model::diff::Generation3dPreviewChange;
use semio_s_artifact_procedural_3d_model::diff::Generation3dStringList;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSynapsePatch;

use semio_s_artifact_procedural_3d_inferences::Generation3dInference;

use semio_s_artifact_procedural_3d_inferences::topology::Generation3dTopology;

use semio_s_artifact_procedural_3d_geometry::GeometryWire;
use semio_s_artifact_procedural_3d_geometry::GeometryDependencyLiteral;
use semio_s_artifact_procedural_3d_geometry::GeometryDependency;
use semio_s_artifact_procedural_3d_geometry::Generation3dFaultRecord;
use semio_s_artifact_procedural_3d_geometry::Generation3dOutputRecord;
use semio_s_artifact_procedural_3d_geometry::Generation3dWidgetRecord;
use semio_s_artifact_procedural_3d_geometry::Generation3dGeometryRecord;

use semio_s_artifact_procedural_3d_geometry::service::GeometryCursor;
use semio_s_artifact_procedural_3d_geometry::service::GeometryProgress;
use semio_s_artifact_procedural_3d_geometry::service::GeometryWidgetResult;
use semio_s_artifact_procedural_3d_geometry::service::GeometryResult;

use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;

use semio_s_artifact_procedural_3d_mut_widgets::create_widget::CreateWidget;

use semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget;

use semio_s_artifact_procedural_3d_mut_widgets::delete_widget::DeleteWidget;

use semio_s_artifact_procedural_3d_mut_synapses::connect_synapse::ConnectSynapse;

use semio_s_artifact_procedural_3d_mut_synapses::update_synapse::UpdateSynapse;

use semio_s_artifact_procedural_3d_mut_synapses::disconnect_synapse::DisconnectSynapse;

use semio_s_artifact_procedural_3d_mut_widgets::move_widget::MoveWidget;

use semio_s_artifact_procedural_3d_mut_widgets::delete_widget_position::DeleteWidgetPosition;

use semio_s_artifact_procedural_3d_mut_document::update_camera::UpdateCamera;

use semio_s_artifact_procedural_3d_mut_document::change_schema::ChangeSchema;

use semio_s_artifact_procedural_3d_mut_generations::create_generation::CreateGeneration;

use semio_s_artifact_procedural_3d_mut_generations::delete_generation::DeleteGeneration;

use semio_s_artifact_procedural_3d_mut_generations::rename_generation::RenameGeneration;

use semio_s_artifact_procedural_3d_mut_generations::change_generation_value::ChangeGenerationValue;

use semio_s_artifact_procedural_3d_mut_widgets::change_slider_value::ChangeSliderValue;

use semio_s_artifact_procedural_3d_mut_transforms::drag_transforms::DragTransforms;

use semio_s_artifact_procedural_3d_mut_transforms::rotate_transforms::RotateTransforms;

use semio_s_artifact_procedural_3d_mut_transforms::scale_transforms::ScaleTransforms;

use semio_s_artifact_procedural_3d_mut_widgets::move_nodes::MoveNodes;

use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputPlane;
use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputValue;
use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::ChangeWidgetInput;

use semio_s_artifact_procedural_3d_mut_generations::select_generation::SelectGeneration;

use semio_s_artifact_procedural_3d_mut_generations::change_generation_preview::ChangeGenerationPreview;

use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetRemoval;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetInsertion;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetRelocation;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetModification;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSynapseRemoval;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSynapseInsertion;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSynapseRelocation;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSynapseModification;
use semio_s_artifact_procedural_3d_model::diff::Generation3dGenerationRemoval;
use semio_s_artifact_procedural_3d_model::diff::Generation3dGenerationInsertion;
use semio_s_artifact_procedural_3d_model::diff::Generation3dGenerationRelocation;
use semio_s_artifact_procedural_3d_model::diff::Generation3dGenerationModification;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

use semio_framework_pack_json::ArtifactCanonicalJsonTree;
