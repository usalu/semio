//! ⚡️ Generation3d artifact — OpText/OpBinary codecs + grammar for `Generation3dMutation`.
//!
//! Wire codecs live in `📡️spr` (DSL mirror); this facet keeps grammar + re-exports.

use crate::standards::v1::subsets::any::schema::mutations::{generation_mutation_to_generation3d, inverse_generation3d_mutation, Generation3dMutation};


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

#[allow(unused_imports)]
mod operation_codec {
use crate::standards::v1::subsets::any::schema::mutations::change_generation_value::ChangeGenerationValue;
use crate::standards::v1::subsets::any::schema::mutations::change_schema::ChangeSchema;
use crate::standards::v1::subsets::any::schema::mutations::connect_synapse::ConnectSynapse;
use crate::standards::v1::subsets::any::schema::mutations::create_generation::CreateGeneration;
use crate::standards::v1::subsets::any::schema::mutations::create_widget::CreateWidget;
use crate::standards::v1::subsets::any::schema::mutations::delete_generation::DeleteGeneration;
use crate::standards::v1::subsets::any::schema::mutations::delete_widget::DeleteWidget;
use crate::standards::v1::subsets::any::schema::mutations::delete_widget_position::DeleteWidgetPosition;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_synapse::DisconnectSynapse;
use crate::standards::v1::subsets::any::schema::mutations::move_widget::MoveWidget;
use crate::standards::v1::subsets::any::schema::mutations::rename_generation::RenameGeneration;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::update_camera::UpdateCamera;
use crate::standards::v1::subsets::any::schema::mutations::update_synapse::UpdateSynapse;
use crate::standards::v1::subsets::any::schema::mutations::update_widget::UpdateWidget;
use crate::standards::v1::subsets::any::io::text::snapshot::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl,
    WidgetLayoutDsl,
};
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_slider_value::ChangeSliderValue;
use crate::standards::v1::subsets::any::schema::mutations::drag_transforms::DragTransforms;
use crate::standards::v1::subsets::any::schema::mutations::rotate_transforms::RotateTransforms;
use crate::standards::v1::subsets::any::schema::mutations::scale_transforms::ScaleTransforms;
use crate::standards::v1::subsets::any::schema::mutations::move_nodes::MoveNodes;
use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::{ChangeWidgetInput, WidgetInputValue};
use crate::standards::v1::subsets::any::schema::mutations::change_generation_preview::ChangeGenerationPreview;
use crate::standards::v1::subsets::any::schema::mutations::select_generation::SelectGeneration;
use protocol::OpBinary;
use store::ErasedSnapshotRetirement;
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub(crate) enum Generation3dOperationDsl {
    CreateWidget {
        index: usize,
        #[dsl(statements)]
        widget: Box<WidgetDsl>,
    },
    UpdateWidget {
        #[dsl(statements)]
        widget: Box<WidgetDsl>,
    },
    DeleteWidget {
        id: String,
    },
    ConnectSynapse {
        index: usize,
        #[dsl(block)]
        synapse: SynapseSpecDsl,
    },
    UpdateSynapse {
        #[dsl(block)]
        synapse: SynapseSpecDsl,
    },
    DisconnectSynapse {
        id: String,
    },
    MoveWidget {
        id: String,
        #[dsl(block)]
        layout: WidgetLayoutDsl,
    },
    DeleteWidgetPosition {
        id: String,
    },
    UpdateCamera {
        #[dsl(block)]
        camera: CameraJsonDsl,
    },
    ChangeSchema {
        new_schema: String,
    },
    CreateGeneration {
        #[dsl(block)]
        generation: FormGenerationDsl,
        index: Option<usize>,
    },
    DeleteGeneration {
        id: String,
    },
    RenameGeneration {
        id: String,
        new_name: String,
    },
    ChangeGenerationValue {
        id: String,
        question_id: String,
        new_value: semio_framework_value::DslValue,
    },
    ChangeSliderValue { id: String, value: f64 },
    DragTransforms { targets: Vec<String>, dx: f64, dy: f64, dz: f64 },
    RotateTransforms { targets: Vec<String>, ax: f64, ay: f64, az: f64, angle: f64 },
    ScaleTransforms { targets: Vec<String>, sx: f64, sy: f64, sz: f64 },
    MoveNodes { ids: Vec<String>, dx: f64, dy: f64 },
    ChangeWidgetInput { id: String, channel: String, input: semio_framework_value::DslValue },
    SelectGeneration { generation_id: Option<String> },
    ChangeGenerationPreview { text: Option<String> },
}

/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for Generation3dOperationDsl {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown operation '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

pub(crate) fn generation3d_operation_to_dsl(operation: &Generation3dMutation) -> Generation3dOperationDsl {
    match operation {
        Generation3dMutation::CreateWidget(CreateWidget { index, widget }) => Generation3dOperationDsl::CreateWidget { index: *index, widget: Box::new(widget_to_dsl(widget)) },
        Generation3dMutation::UpdateWidget(UpdateWidget { widget }) => Generation3dOperationDsl::UpdateWidget { widget: Box::new(widget_to_dsl(widget)) },
        Generation3dMutation::DeleteWidget(DeleteWidget { id }) => Generation3dOperationDsl::DeleteWidget { id: id.clone() },
        Generation3dMutation::ConnectSynapse(ConnectSynapse { index, synapse }) => Generation3dOperationDsl::ConnectSynapse { index: *index, synapse: synapse_to_dsl(synapse) },
        Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse }) => Generation3dOperationDsl::UpdateSynapse { synapse: synapse_to_dsl(synapse) },
        Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id }) => Generation3dOperationDsl::DisconnectSynapse { id: id.clone() },
        Generation3dMutation::MoveWidget(MoveWidget { id, layout }) => Generation3dOperationDsl::MoveWidget { id: id.clone(), layout: layout_to_dsl(layout) },
        Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id }) => Generation3dOperationDsl::DeleteWidgetPosition { id: id.clone() },
        Generation3dMutation::UpdateCamera(UpdateCamera { camera }) => Generation3dOperationDsl::UpdateCamera { camera: camera_to_dsl(camera) },
        Generation3dMutation::ChangeSchema(ChangeSchema { new_schema }) => Generation3dOperationDsl::ChangeSchema { new_schema: new_schema.clone() },
        Generation3dMutation::CreateGeneration(CreateGeneration { generation, index }) => Generation3dOperationDsl::CreateGeneration { generation: form_generation_to_dsl(generation), index: *index },
        Generation3dMutation::DeleteGeneration(DeleteGeneration { id }) => Generation3dOperationDsl::DeleteGeneration { id: id.clone() },
        Generation3dMutation::RenameGeneration(RenameGeneration { id, new_name }) => Generation3dOperationDsl::RenameGeneration { id: id.clone(), new_name: new_name.clone() },
        Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue { id, question_id, new_value }) => Generation3dOperationDsl::ChangeGenerationValue { id: id.clone(), question_id: question_id.clone(), new_value: new_value.clone() },
        Generation3dMutation::ChangeSliderValue(ChangeSliderValue { id, value }) => Generation3dOperationDsl::ChangeSliderValue { id: id.clone(), value: *value },
        Generation3dMutation::DragTransforms(DragTransforms { targets, dx, dy, dz }) => Generation3dOperationDsl::DragTransforms { targets: targets.clone(), dx: *dx, dy: *dy, dz: *dz },
        Generation3dMutation::RotateTransforms(RotateTransforms { targets, ax, ay, az, angle }) => Generation3dOperationDsl::RotateTransforms { targets: targets.clone(), ax: *ax, ay: *ay, az: *az, angle: *angle },
        Generation3dMutation::ScaleTransforms(ScaleTransforms { targets, sx, sy, sz }) => Generation3dOperationDsl::ScaleTransforms { targets: targets.clone(), sx: *sx, sy: *sy, sz: *sz },
        Generation3dMutation::MoveNodes(MoveNodes { ids, dx, dy }) => Generation3dOperationDsl::MoveNodes { ids: ids.clone(), dx: *dx, dy: *dy },
        Generation3dMutation::ChangeWidgetInput(ChangeWidgetInput { id, channel, input }) => Generation3dOperationDsl::ChangeWidgetInput { id: id.clone(), channel: channel.clone(), input: semio_framework_value::ToValue::to_value(input) },
        Generation3dMutation::SelectGeneration(SelectGeneration { generation_id }) => Generation3dOperationDsl::SelectGeneration { generation_id: generation_id.clone() },
        Generation3dMutation::ChangeGenerationPreview(ChangeGenerationPreview { text }) => Generation3dOperationDsl::ChangeGenerationPreview { text: text.clone() },
    }
}

pub(crate) fn generation3d_operation_from_dsl(operation: Generation3dOperationDsl) -> Result<Generation3dMutation, semio_framework_diagnostic::TextError> {
    Ok(match operation {
        Generation3dOperationDsl::CreateWidget { index, widget } => Generation3dMutation::CreateWidget(CreateWidget { index, widget: widget_from_dsl(*widget)? }),
        Generation3dOperationDsl::UpdateWidget { widget } => Generation3dMutation::UpdateWidget(UpdateWidget { widget: widget_from_dsl(*widget)? }),
        Generation3dOperationDsl::DeleteWidget { id } => Generation3dMutation::DeleteWidget(DeleteWidget { id }),
        Generation3dOperationDsl::ConnectSynapse { index, synapse } => Generation3dMutation::ConnectSynapse(ConnectSynapse { index, synapse: synapse_from_dsl(synapse) }),
        Generation3dOperationDsl::UpdateSynapse { synapse } => Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: synapse_from_dsl(synapse) }),
        Generation3dOperationDsl::DisconnectSynapse { id } => Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id }),
        Generation3dOperationDsl::MoveWidget { id, layout } => Generation3dMutation::MoveWidget(MoveWidget { id, layout: layout_from_dsl(&layout) }),
        Generation3dOperationDsl::DeleteWidgetPosition { id } => Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id }),
        Generation3dOperationDsl::UpdateCamera { camera } => Generation3dMutation::UpdateCamera(UpdateCamera { camera: camera_from_dsl(&camera) }),
        Generation3dOperationDsl::ChangeSchema { new_schema } => Generation3dMutation::ChangeSchema(ChangeSchema { new_schema }),
        Generation3dOperationDsl::CreateGeneration { generation, index } => Generation3dMutation::CreateGeneration(CreateGeneration { generation: form_generation_from_dsl(generation), index }),
        Generation3dOperationDsl::DeleteGeneration { id } => Generation3dMutation::DeleteGeneration(DeleteGeneration { id }),
        Generation3dOperationDsl::RenameGeneration { id, new_name } => Generation3dMutation::RenameGeneration(RenameGeneration { id, new_name }),
        Generation3dOperationDsl::ChangeGenerationValue { id, question_id, new_value } => Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue { id, question_id, new_value }),
        Generation3dOperationDsl::ChangeSliderValue { id, value } => Generation3dMutation::ChangeSliderValue(ChangeSliderValue { id, value }),
        Generation3dOperationDsl::DragTransforms { targets, dx, dy, dz } => Generation3dMutation::DragTransforms(DragTransforms { targets, dx, dy, dz }),
        Generation3dOperationDsl::RotateTransforms { targets, ax, ay, az, angle } => Generation3dMutation::RotateTransforms(RotateTransforms { targets, ax, ay, az, angle }),
        Generation3dOperationDsl::ScaleTransforms { targets, sx, sy, sz } => Generation3dMutation::ScaleTransforms(ScaleTransforms { targets, sx, sy, sz }),
        Generation3dOperationDsl::MoveNodes { ids, dx, dy } => Generation3dMutation::MoveNodes(MoveNodes { ids, dx, dy }),
        Generation3dOperationDsl::ChangeWidgetInput { id, channel, input } => Generation3dMutation::ChangeWidgetInput(ChangeWidgetInput { id, channel, input: <WidgetInputValue as semio_framework_value::FromValue>::from_value(input).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("change-widget-input input: {error}"),semio_framework_diagnostic::TextSpan::at(1,1)))? }),
        Generation3dOperationDsl::SelectGeneration { generation_id } => Generation3dMutation::SelectGeneration(SelectGeneration { generation_id }),
        Generation3dOperationDsl::ChangeGenerationPreview { text } => Generation3dMutation::ChangeGenerationPreview(ChangeGenerationPreview { text }),
    })
}

/// ⚡️ `Generation3dMutation`'s compact single-line op encoding — derive-engine grammar via
/// `Generation3dOperationDsl`; `parse_op`/`print_op` convert at the boundary.
impl protocol::OpText for Generation3dMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let parsed = <Generation3dOperationDsl as protocol::OpText>::parse_op(line)?;
        generation3d_operation_from_dsl(parsed)
    }

    fn print_op(&self) -> String {
        <Generation3dOperationDsl as protocol::OpText>::print_op(&generation3d_operation_to_dsl(self))
    }
}
}
pub use operation_codec::*;
use crate::central_apply::{apply_generation3d_mutation};

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
