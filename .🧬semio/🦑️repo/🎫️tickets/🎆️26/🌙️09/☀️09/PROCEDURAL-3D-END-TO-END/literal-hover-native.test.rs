extern crate self as semio_plugin;
extern crate self as semio_s_procedural_core_semantic_ui;
extern crate self as semio_s_artifact_procedural_3d_mode_edit;
extern crate semio_ui_contract_builder as semio_ui_contract;

use semio_ui_contract::{ActionId, BuiltNode, BuiltTreeRetirement, Buildable, HasBase, Label, Trigger};

#[derive(Debug)]
pub struct PluginAssemblyError(&'static str);
impl PluginAssemblyError {
    pub fn new(code: &'static str, _: &str) -> Self { Self(code) }
}
type UiAssemblyResult<T> = Result<T, PluginAssemblyError>;
fn ui_assembly_error(code: &'static str) -> PluginAssemblyError { PluginAssemblyError::new(code, code) }
const INTERACTION_HOVER_ACTION_ID: &str = "interactionHover";
const GENERATION_3D_PLAY_APP_ID: &str = "procedural3d-play";
pub struct ActionFactory(&'static str);
impl ActionFactory {
    fn new(scope: &'static str) -> Self { Self(scope) }
    fn action(&self, name: &str, args: Option<UiValue>) -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
        ActionId::try_v1(self.0, name).map(|action| (action, args)).ok_or_else(|| ui_assembly_error("test.action"))
    }
}
fn outline_error(code: &'static str) -> PluginAssemblyError { ui_assembly_error(code) }
pub mod windows {
    pub mod flow {
        pub use crate::bind_graph_hover;
    }
}

#[test]
fn shared_spatial_controls_keep_typed_edits_and_exact_input_hover() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(LITERAL_FIXTURE)).unwrap();
    let _arena = scope::UiArenaFixture::new();
    let binding = |node: &BuiltNode| serde_json::to_value(node.bindings.iter().next().unwrap()).unwrap();
    let cases = fixture["cases"].as_array().unwrap().iter().filter(|row| row["kind"] == "literal");
    for row in cases {
        let key = row["key"].as_str().unwrap();
        let label = row["label"].as_str().unwrap();
        let widget = row["widget"].as_str().unwrap();
        let channel = row["port"].as_str().unwrap();
        let part = row["part"].as_str();
        let index = row["index"].as_u64().map(|index| index as usize);
        let value = DslValue::from(&row["value"]);
        let group = coordinate_input(key, label, widget, channel, part, index, Some(&value)).unwrap();
        let hover = binding(&group);
        assert_eq!(hover["trigger"], fixture["trigger"]);
        assert_eq!(hover["action"]["name"], fixture["action"]);
        assert_eq!(hover["args"]["targetId"], row["target"]["id"]);
        assert_eq!(hover["args"]["granularityId"], row["target"]["granularity"]);
        assert_eq!(group.children.len(), 3);
        for (axis, field) in group.children.iter().enumerate() {
            let edit = binding(&field.children[0]);
            assert_eq!(edit["trigger"], "commit");
            assert_eq!(edit["action"]["name"], "setWidgetInput");
            assert_eq!(edit["args"]["component"], row["components"][axis]);
            assert_eq!(edit["args"]["index"], row["index"]);
        }
        let mut retirement = BuiltTreeRetirement::new(group);
        while !retirement.terminal_is_empty() {
            retirement.close_step(32, 8 * 1024).unwrap();
        }
    }
}
