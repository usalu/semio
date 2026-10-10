use super::*;
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,shared::SharedControlledRetirement};

#[derive(semio_framework_value::RetireOwned)]
struct FlowHostEffectSources {
 command:FlowCommand,
 snapshot:SharedControlledRetirement<FlowSnapshot>,
 config:SharedControlledRetirement<FlowMainWindowConfig>,
 history:SharedControlledRetirement<semio_framework_plugin::HistoryView>,
 children:SharedControlledRetirement<semio_framework_plugin::ChildContentView>,
 instance_owner:semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
 completion:semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<FlowPlayApp>>,
}

impl RetireOwned for FlowHostEffectPayload {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{command,snapshot,config,history,children,instance_owner,completion}=self;FlowHostEffectSources{command,snapshot:SharedControlledRetirement::lease(snapshot),config:SharedControlledRetirement::lease(config),history:SharedControlledRetirement::lease(history),children:SharedControlledRetirement::lease(children),instance_owner,completion}.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes};sequence_birth_bytes(&[deferred_birth_bytes::<FlowCommand>(),deferred_birth_bytes::<SharedControlledRetirement<FlowSnapshot>>(),deferred_birth_bytes::<SharedControlledRetirement<FlowMainWindowConfig>>(),deferred_birth_bytes::<SharedControlledRetirement<semio_framework_plugin::HistoryView>>(),deferred_birth_bytes::<SharedControlledRetirement<semio_framework_plugin::ChildContentView>>(),deferred_birth_bytes::<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>(),deferred_birth_bytes::<semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<FlowPlayApp>>>()])}
 fn controlled_retirement_supported()->bool{true}
}
