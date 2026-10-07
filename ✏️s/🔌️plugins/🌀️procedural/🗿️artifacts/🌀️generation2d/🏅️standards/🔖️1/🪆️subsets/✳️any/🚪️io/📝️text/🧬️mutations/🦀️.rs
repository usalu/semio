//! ⚡️ Generation2d artifact — OpText/OpBinary codecs + grammar for `Generation2dMutation`.
//!
//! Wire codecs live in `📡️spr` (DSL mirror); this facet keeps grammar + re-exports.

use crate::standards::v1::subsets::any::schema::mutations::{apply_generation2d_mutation,generation2d_host_snapshot_operations,generation_mutation_to_generation2d,inverse_generation2d_mutation,replace_widget,Generation2dMutation};


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

use crate::standards::v1::subsets::any::io::text::snapshot::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl,
    WidgetLayoutDsl,
};
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub(crate) enum Generation2dOperationDsl {
    CreateWidget {
        index: usize,
        #[dsl(statements)]
        widget: Box<WidgetDsl>,
    },
    ReplaceWidget {
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
    ReplaceSynapse {
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
    ClearWidgetLayout {
        id: String,
    },
    UpdateCamera {
        #[dsl(block)]
        camera: CameraJsonDsl,
    },
    ChangeSchema {
        schema: String,
    },
    CreateGeneration {
        #[dsl(block)]
        generation: FormGenerationDsl,
    },
    DeleteGeneration {
        id: String,
    },
    RenameGeneration {
        id: String,
        name: String,
    },
    ChangeGenerationValue {
        id: String,
        question_id: String,
        value: semio_framework_value::DslValue,
    },
    ChangeSliderValue {
        id: String,
        value: f64,
    },
    MoveNodes {
        ids: Vec<String>,
        dx: f64,
        dy: f64,
    },
}

impl protocol::OpText for Generation2dOperationDsl {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

pub(crate) fn generation2d_operation_to_dsl(operation: &Generation2dMutation) -> Generation2dOperationDsl {
    match operation {
        Generation2dMutation::CreateWidget(payload) => Generation2dOperationDsl::CreateWidget { index: payload.index, widget: Box::new(widget_to_dsl(&payload.widget)) },
        Generation2dMutation::ReplaceWidget(payload) => Generation2dOperationDsl::ReplaceWidget { widget: Box::new(widget_to_dsl(&payload.widget)) },
        Generation2dMutation::DeleteWidget(payload) => Generation2dOperationDsl::DeleteWidget { id: payload.id.clone() },
        Generation2dMutation::ConnectSynapse(payload) => Generation2dOperationDsl::ConnectSynapse { index: payload.index, synapse: synapse_to_dsl(&payload.synapse) },
        Generation2dMutation::ReplaceSynapse(payload) => Generation2dOperationDsl::ReplaceSynapse { synapse: synapse_to_dsl(&payload.synapse) },
        Generation2dMutation::DisconnectSynapse(payload) => Generation2dOperationDsl::DisconnectSynapse { id: payload.id.clone() },
        Generation2dMutation::MoveWidget(payload) => Generation2dOperationDsl::MoveWidget { id: payload.id.clone(), layout: layout_to_dsl(&payload.layout) },
        Generation2dMutation::ClearWidgetLayout(payload) => Generation2dOperationDsl::ClearWidgetLayout { id: payload.id.clone() },
        Generation2dMutation::UpdateCamera(payload) => Generation2dOperationDsl::UpdateCamera { camera: camera_to_dsl(&payload.camera) },
        Generation2dMutation::ChangeSchema(payload) => Generation2dOperationDsl::ChangeSchema { schema: payload.schema.clone() },
        Generation2dMutation::CreateGeneration(payload) => Generation2dOperationDsl::CreateGeneration { generation: form_generation_to_dsl(&payload.generation) },
        Generation2dMutation::DeleteGeneration(payload) => Generation2dOperationDsl::DeleteGeneration { id: payload.id.clone() },
        Generation2dMutation::RenameGeneration(payload) => Generation2dOperationDsl::RenameGeneration { id: payload.id.clone(), name: payload.name.clone() },
        Generation2dMutation::ChangeGenerationValue(payload) => Generation2dOperationDsl::ChangeGenerationValue { id: payload.id.clone(), question_id: payload.question_id.clone(), value: payload.value.clone() },
        Generation2dMutation::ChangeSliderValue(payload) => Generation2dOperationDsl::ChangeSliderValue { id: payload.id.clone(), value: payload.value },
        Generation2dMutation::MoveNodes(payload) => Generation2dOperationDsl::MoveNodes { ids: payload.ids.clone(), dx: payload.dx, dy: payload.dy },
    }
}

pub(crate) fn generation2d_operation_from_dsl(operation: Generation2dOperationDsl) -> Result<Generation2dMutation, semio_framework_diagnostic::TextError> {
    use crate::standards::v1::subsets::any::schema::mutations::{change_generation_value,change_schema,change_slider_value,clear_widget_layout,connect_synapse,create_generation,create_widget,delete_generation,delete_widget,disconnect_synapse,move_nodes,move_widget,rename_generation,replace_synapse,replace_widget,update_camera};

    Ok(match operation {
        Generation2dOperationDsl::CreateWidget { index, widget } => create_widget(index, widget_from_dsl(*widget)?),
        Generation2dOperationDsl::ReplaceWidget { widget } => replace_widget(widget_from_dsl(*widget)?),
        Generation2dOperationDsl::DeleteWidget { id } => delete_widget(id),
        Generation2dOperationDsl::ConnectSynapse { index, synapse } => connect_synapse(index, synapse_from_dsl(synapse)),
        Generation2dOperationDsl::ReplaceSynapse { synapse } => replace_synapse(synapse_from_dsl(synapse)),
        Generation2dOperationDsl::DisconnectSynapse { id } => disconnect_synapse(id),
        Generation2dOperationDsl::MoveWidget { id, layout } => move_widget(id, layout_from_dsl(&layout)),
        Generation2dOperationDsl::ClearWidgetLayout { id } => clear_widget_layout(id),
        Generation2dOperationDsl::UpdateCamera { camera } => update_camera(camera_from_dsl(&camera)),
        Generation2dOperationDsl::ChangeSchema { schema } => change_schema(schema),
        Generation2dOperationDsl::CreateGeneration { generation } => create_generation(form_generation_from_dsl(generation)),
        Generation2dOperationDsl::DeleteGeneration { id } => delete_generation(id),
        Generation2dOperationDsl::RenameGeneration { id, name } => rename_generation(id, name),
        Generation2dOperationDsl::ChangeGenerationValue { id, question_id, value } => change_generation_value(id, question_id, value),
        Generation2dOperationDsl::ChangeSliderValue { id, value } => change_slider_value(id, value),
        Generation2dOperationDsl::MoveNodes { ids, dx, dy } => move_nodes(ids, dx, dy),
    })
}

impl protocol::OpText for Generation2dMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let parsed = <Generation2dOperationDsl as protocol::OpText>::parse_op(line)?;
        generation2d_operation_from_dsl(parsed)
    }

    fn print_op(&self) -> String {
        <Generation2dOperationDsl as protocol::OpText>::print_op(&generation2d_operation_to_dsl(self))
    }
}
