//! ✏️ Original typed procedural mutation fields move once into their retained domain producer.
use super::retained_native_provisional_fold::{NativeProvisionalFold,NativeProvisionalFolds};
use super::retained_host_scratch::{HostScratchEdit,HostScratchWork,HostScratchDomain};
use super::retained_generation_fold::{GenerationFoldInput,GenerationFoldWork};
use super::Generation2dMutation;
use semio_framework_job::OriginalSourceBinding;
/// 🧵️ Routes every original 2d mutation into its retained domain producer.
pub struct ProvisionalFolds;
impl NativeProvisionalFolds<Generation2dMutation> for ProvisionalFolds{
 type Additional=super::retained_native_provisional_fold::NeverAdditional;
 fn into_fold_original(mutation:Generation2dMutation,original:OriginalSourceBinding)->Result<NativeProvisionalFold<Self::Additional>,Generation2dMutation>{
  let host=|edit|Ok(NativeProvisionalFold::Host(HostScratchWork::new_original(edit,HostScratchDomain::Generation2d,original)));
  let generation=|input|Ok(NativeProvisionalFold::Generation(GenerationFoldWork::new_original(input,original)));
  match mutation{
   Generation2dMutation::CreateWidget(v)=>host(HostScratchEdit::InsertWidget{index:v.index,widget:v.widget}),
   Generation2dMutation::ReplaceWidget(v)=>host(HostScratchEdit::ReplaceWidget{widget:v.widget}),
   Generation2dMutation::DeleteWidget(v)=>host(HostScratchEdit::RemoveWidget{id:v.id}),
   Generation2dMutation::ConnectSynapse(v)=>host(HostScratchEdit::InsertSynapse{index:v.index,synapse:v.synapse}),
   Generation2dMutation::ReplaceSynapse(v)=>host(HostScratchEdit::ReplaceSynapse{synapse:v.synapse}),
   Generation2dMutation::DisconnectSynapse(v)=>host(HostScratchEdit::RemoveSynapse{id:v.id}),
   Generation2dMutation::MoveWidget(v)=>host(HostScratchEdit::SetLayout{id:v.id,layout:v.layout}),
   Generation2dMutation::ClearWidgetLayout(v)=>host(HostScratchEdit::RemoveLayout{id:v.id}),
   Generation2dMutation::UpdateCamera(v)=>host(HostScratchEdit::SetCamera{camera:v.camera}),
   Generation2dMutation::ChangeSchema(v)=>host(HostScratchEdit::SetSchema{schema:v.schema}),
   Generation2dMutation::ChangeSliderValue(v)=>host(HostScratchEdit::SetSlider{id:v.id,value:v.value}),
   Generation2dMutation::MoveNodes(v)=>host(HostScratchEdit::MoveNodes{ids:v.ids,dx:v.dx,dy:v.dy}),
   Generation2dMutation::CreateGeneration(v)=>generation(GenerationFoldInput::Add{generation:v.generation,index:v.index}),
   Generation2dMutation::DeleteGeneration(v)=>generation(GenerationFoldInput::Remove{id:v.id}),
   Generation2dMutation::RenameGeneration(v)=>generation(GenerationFoldInput::Rename{id:v.id,name:v.name}),
   Generation2dMutation::ChangeGenerationValue(v)=>generation(GenerationFoldInput::SetValue{id:v.id,key:v.question_id,value:v.value}),
   Generation2dMutation::SelectGeneration(v)=>generation(GenerationFoldInput::Select{id:v.generation_id}),
  }
 }
}
