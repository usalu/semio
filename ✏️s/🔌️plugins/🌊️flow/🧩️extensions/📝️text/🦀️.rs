//! 📝️ Flow text module: operators for text dictionaries.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};

// #region 🔖️Concat
/// 🔗️ Joins two text inputs.
pub struct Concat;

impl Operator for Concat {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("text", text_dictionary(format!("{}{}", read_channel_text(input, "a")?, read_channel_text(input, "b")?))))
    }
}
// #endregion 🔖️Concat

// #region 🔖️Upper
/// 🔠️ Uppercases a text input.
pub struct Upper;

impl Operator for Upper {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("textOut", text_dictionary(read_channel_text(input, "text")?.to_uppercase())))
    }
}
// #endregion 🔖️Upper

// #region 🔖️Helpers
fn text_dictionary(value: String) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value)))
}

fn read_channel_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input.get(key).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn text_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::text_default(id, "", &[operator_id])
}

fn info(id: &str, name: &str, summary: &str, inputs: Vec<ChannelSpec>, output: ChannelSpec) -> OperatorInfo {
    OperatorInfo { id: id.into(), extension: "text".into(), name: name.into(), abbreviation: name.into(), icon: "emoji:📝️".into(), summary: summary.into(), inputs, outputs: vec![output], ..Default::default() }
}

// #endregion 🔖️Helpers

/// 📦️ Registers all text operators.
pub fn register(registry: &mut Registry) {
    registry.register_operator(
        info("text.concat", "Concat", "Joins two text values", vec![text_channel("a", "text.concat"), text_channel("b", "text.concat")], ChannelSpec::named("T", "Txt", "text", "JoinedText")),
        vec![OperatorImpl { schemas: vec!["text".into(), "text".into()], operator: Box::new(Concat) }],
        &["text"],
    );
    registry.register_operator(
        info("text.upper", "Upper", "Uppercases text", vec![text_channel("text", "text.upper")], ChannelSpec::named("T", "Txt", "textOut", "UppercasedText")),
        vec![OperatorImpl { schemas: vec!["text".into()], operator: Box::new(Upper) }],
        &["text"],
    );
    registry.finalize();
}

// #region 🔖️Manifest
/// 📦️ Flow extension manifest JSON contributed to host catalogues.
pub fn extension_manifest_json() -> String {
    use flow_extension_sdk::{build_manifest_json, FlowExtensionCommand};
    build_manifest_json("text", "Text", env!("CARGO_PKG_VERSION"), &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "text.showHelp".into(), title: "Text: Show Help".into() }], vec![])
}

/// 🌊️ Builds an in-process operator registry for this extension.
pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Manifest

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️ExtensionGuest
/// 🧩️ Runtime-installable flow extension bundle for `text`.
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::{extension_manifest_json, module_registry};
    use flow_extension_sdk::{flow_extension_topic_contribution};
    use semio_framework::{Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::{ExecutionMode, ExtensionBundle};

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "text";
    const EXTENSION_LABEL: &str = "Text";

    // 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires
    // a plain sync fn). `.mode`/`.contributes_topic`/`.handler` are still `async fn` in
    // `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's
    // path_scope); bridged via `semio_framework_os_kernel::io::resolve_ready` — see this packet's lease-request.
    // See R9.
    fn bundle() -> ExtensionBundle {
        let manifest_json = extension_manifest_json();
        let flow_topic = flow_extension_topic_contribution(FLOW_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "text", &manifest_json);
        let procedural3d_topic = flow_extension_topic_contribution(PROCEDURAL3D_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "text", &manifest_json);
        let bundle = ExtensionBundle::new("flow-extension-text", EXTENSION_LABEL, env!("CARGO_PKG_VERSION")).extends("flow").depends_on("flow", semio_framework::tree_pin!());
        let bundle = bundle.mode(ExecutionMode::Linked);
        let bundle = bundle.contributes_topic(flow_topic.topic, flow_topic.payload);
        let bundle = bundle.contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload);
        bundle.resource_owner(flow_extension_sdk::ExtensionEvaluationResources::new(module_registry())).owned_handler("evaluate")
    }

    #[cfg(test)]
    include!("🧪️tests/🔬️extension-guest-standalone/🦀️.rs");

    semio_framework_plugin::extension_exports!({ let grant = semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }, bundle);
}
// #endregion 🔖️ExtensionGuest
