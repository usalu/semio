use super::*;
use semio_framework_value::retirement::{RetireOwned,arc_custody::ArcLease};

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
#[doc(hidden)]
pub struct FlowHostEffectSources {
 command:FlowCommand,
 snapshot:ArcLease<FlowSnapshot>,
 config:ArcLease<FlowMainWindowConfig>,
 history:ArcLease<semio_plugin::HistoryView>,
 children:ArcLease<semio_plugin::ChildContentView>,
 instance_owner:semio_plugin::ArtifactInstanceOperationOwnerHandle,
 completion:semio_plugin::ArtifactToolCompletion<semio_plugin::EditorApp<FlowPlayApp>>,
}

impl RetireOwned for FlowHostEffectPayload {
 type RetirementCursor=<FlowHostEffectSources as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{let Self{command,snapshot,config,history,children,instance_owner,completion}=self;FlowHostEffectSources{command,snapshot:ArcLease::new(snapshot),config:ArcLease::new(config),history:ArcLease::new(history),children:ArcLease::new(children),instance_owner,completion}.retirement_seed()}
 fn controlled_retirement_supported()->bool{true}
}
