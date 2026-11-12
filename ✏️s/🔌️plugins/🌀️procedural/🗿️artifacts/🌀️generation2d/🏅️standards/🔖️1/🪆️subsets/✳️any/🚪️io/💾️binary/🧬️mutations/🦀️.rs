//! ⚖️ Generation2d artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;


#[cfg(test)]


use semio_s_artifact_procedural_2d_model::dsl_mirror::camera_from_dsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::camera_to_dsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::form_generation_from_dsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::form_generation_to_dsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::layout_from_dsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::layout_to_dsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::synapse_from_dsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::synapse_to_dsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::widget_from_dsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::widget_to_dsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::CameraJsonDsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::FormGenerationDsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::SynapseSpecDsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::WidgetDsl;
use semio_s_artifact_procedural_2d_model::dsl_mirror::WidgetLayoutDsl;
#[cfg(test)] use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::OpBinary;

//#region 🔖️OpTextMirror

//#region 🔖️HandcraftedOpCodecs


impl OpBinary for Generation2dOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(COMPONENT_PROTOCOL_SEMIO, self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(COMPONENT_PROTOCOL_SEMIO, bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs







/// ⚡️ Binary mirror of the `OpText` bridge above — `Generation2dOperationDsl` already implements
/// `OpBinary`, so this is a pure to/from-dsl forward.
impl OpBinary for Generation2dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(store::os_pack::pack_rt::encode_pack_value(&semio_framework_value::ToValue::to_value(self)))
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let malformed = |detail: String| protocol::ProtocolError::Malformed { what: "generation2d mutation", offset: 0, detail };
        let value = store::os_pack::pack_rt::decode_pack_value(bytes).map_err(|error| malformed(error.to_string()))?;
        semio_framework_value::FromValue::from_value(store::os_pack::pack_rt::renormalize_whole_number_floats(value)).map_err(|error| malformed(error.to_string()))
    }
}

dsl::original_pack_value_operation_receiving!(Generation2dMutation);
dsl::artifact_persisted_binary_operation!(Generation2dMutation);
//#endregion 🔖️OpTextMirror

/// 📦️ Encodes a `Generation2dMutation` to its binary state-patch form.
pub fn encode_op(operation: &Generation2dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Generation2dMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<Generation2dMutation, protocol::ProtocolError> {
    Generation2dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#region 🔖️RetainedMountedIngress



























































pub const GENERATION2D_RETAINED_SCHEMA_DISCRIMINATOR: [u8; 4] = *b"P2D2";
pub const GENERATION2D_FORBIDDEN_3D_DISCRIMINATOR: [u8; 4] = *b"P3D3";
















//#endregion 🔖️RetainedMountedIngress

//#region 🔖️TypedOwnedEnvelopeCatalog


































































































//#endregion 🔖️TypedOwnedEnvelopeCatalog

//#region 🔖️RetainedStoreInitialization



































//#endregion 🔖️RetainedStoreInitialization





#[cfg(any(test, feature = "testkit"))]
pub fn generation2d_all_retained_mutation_fixtures_for_test() -> Vec<Generation2dMutation> {
    use semio_s_artifact_procedural_2d_mutations::*;
    let synapse = semio_framework_artifact_flow_flow::SynapseSpec { id: "retained-synapse".into(), from: "retained-a".into(), to: "retained-b".into(), from_port: "out".into(), to_port: "in".into() };
    let mut values = semio_framework_artifact_playbook_playbook::PlaybookValues::new();
    values.insert("nested".into(), semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"array": [true, null, 3.5], "text": "retained"})));
    let params = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("integer", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(7))).insert(
        "nested",
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(
            semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("text", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String("retained".into()))),
        ),
    );
    vec![
        create_widget(0, semio_framework_artifact_flow_flow::Widget::Neuron { id: "retained-a".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true }),
        replace_widget(semio_framework_artifact_flow_flow::Widget::Cluster { id: "retained-a".into(), name: "Replaced".into(), tree: Default::default(), flow: Default::default() }),
        delete_widget("retained-a".into()),
        connect_synapse(0, generation2d_copy_synapse(&synapse).expect("P2 synapse fixture copy")),
        replace_synapse(semio_framework_artifact_flow_flow::SynapseSpec { to_port: "alternate".into(), ..synapse }),
        disconnect_synapse("retained-synapse".into()),
        move_widget("retained-a".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 11.0, y: -7.0 }),
        clear_widget_layout("retained-a".into()),
        update_camera(semio_framework_artifact_flow_flow::CameraJson { x: 3.0, y: 4.0, zoom: 1.5 }),
        change_schema("flow.host_snapshot.retained".into()),
        Generation2dMutation::CreateGeneration(semio_s_artifact_procedural_2d_mut_generations::create_generation::CreateGeneration { generation: semio_framework_artifact_playbook_playbook::FormGeneration { id: "retained-generation".into(), name: "Retained Generation".into(), values }, index: Some(0) }),
        delete_generation("retained-generation".into()),
        rename_generation("retained-generation".into(), "Renamed Generation".into()),
        change_generation_value("retained-generation".into(), "deep-answer".into(), semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"object": {"array": [1.0, false, "value"]}}))),
        change_slider_value("retained-slider", 7.25),
        move_nodes(vec!["retained-a".into(), "retained-b".into()], 12.5, -4.0),
        semio_s_artifact_procedural_2d_mut_generations::select_generation("retained-generation".to_string().into()),
    ]
}





use crate::text::Generation2dOperationDsl;
use crate::text::generation2d_operation_to_dsl;
use crate::text::generation2d_operation_from_dsl;
#[cfg(any(test, feature = "testkit"))] use semio_s_artifact_procedural_2d_mutations::central_apply::{GENERATION2D_MAXIMUM_DOMAIN_ITEMS, GENERATION2D_OWNER_BYTES, GENERATION2D_RETAINED_STACK_CAPACITY, generation2d_apply_initialization_mutation, generation2d_copy_generation, generation2d_copy_string, generation2d_copy_synapse, generation2d_copy_widget};
#[cfg(any(test, feature = "testkit"))]
use semio_s_artifact_procedural_2d_mutations::central_apply::generation2d_apply_retained_mutations_for_test;
#[cfg(any(test, feature = "testkit"))]
use semio_s_artifact_procedural_2d_mutations::central_apply::generation2d_retire_mutations_cold;
