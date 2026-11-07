//! ✏️ Original typed procedural mutation fields move once into their retained domain producer.
use super::retained_native_provisional_fold::{NativeProvisionalFold,NativeProvisionalFolds};
use super::retained_host_scratch::{HostScratchEdit,HostScratchWork,HostScratchDomain};
use super::retained_generation_fold::{GenerationFoldInput,GenerationFoldWork};
use super::Generation3dMutation;
use semio_framework_job::OriginalSourceBinding;
use super::retained_provisional_transform::{ProvisionalTransformInput,ProvisionalTransformWork,TransformKind};
use super::retained_typed_input::{P3ProvisionalFold,ProvisionalTypedInputWork};
/// 🧵️ Routes every original 3d mutation into its retained domain producer.
pub struct ProvisionalFolds;
impl NativeProvisionalFolds<Generation3dMutation> for ProvisionalFolds{
 type Additional=super::retained_typed_input::P3ProvisionalFold;
 fn into_fold_original(mutation:Generation3dMutation,original:OriginalSourceBinding)->Result<NativeProvisionalFold<Self::Additional>,Generation3dMutation>{
  let host=|edit|Ok(NativeProvisionalFold::Host(HostScratchWork::new_original(edit,HostScratchDomain::Generation3d,original)));
  let generation=|input|Ok(NativeProvisionalFold::Generation(GenerationFoldWork::new_original(input,original)));
  match mutation{
   Generation3dMutation::CreateWidget(v)=>host(HostScratchEdit::InsertWidget{index:v.index,widget:v.widget}),
   Generation3dMutation::UpdateWidget(v)=>host(HostScratchEdit::ReplaceWidget{widget:v.widget}),
   Generation3dMutation::DeleteWidget(v)=>host(HostScratchEdit::RemoveWidget{id:v.id}),
   Generation3dMutation::ConnectSynapse(v)=>host(HostScratchEdit::InsertSynapse{index:v.index,synapse:v.synapse}),
   Generation3dMutation::UpdateSynapse(v)=>host(HostScratchEdit::ReplaceSynapse{synapse:v.synapse}),
   Generation3dMutation::DisconnectSynapse(v)=>host(HostScratchEdit::RemoveSynapse{id:v.id}),
   Generation3dMutation::MoveWidget(v)=>host(HostScratchEdit::SetLayout{id:v.id,layout:v.layout}),
   Generation3dMutation::DeleteWidgetPosition(v)=>host(HostScratchEdit::RemoveLayout{id:v.id}),
   Generation3dMutation::UpdateCamera(v)=>host(HostScratchEdit::SetCamera{camera:v.camera}),
   Generation3dMutation::ChangeSchema(v)=>host(HostScratchEdit::SetSchema{schema:v.new_schema}),
   Generation3dMutation::ChangeSliderValue(v)=>host(HostScratchEdit::SetSlider{id:v.id,value:v.value}),
   Generation3dMutation::MoveNodes(v)=>host(HostScratchEdit::MoveNodes{ids:v.ids,dx:v.dx,dy:v.dy}),
   Generation3dMutation::CreateGeneration(v)=>generation(GenerationFoldInput::Add{generation:v.generation,index:v.index}),
   Generation3dMutation::DeleteGeneration(v)=>generation(GenerationFoldInput::Remove{id:v.id}),
   Generation3dMutation::RenameGeneration(v)=>generation(GenerationFoldInput::Rename{id:v.id,name:v.new_name}),
   Generation3dMutation::ChangeGenerationValue(v)=>generation(GenerationFoldInput::SetValue{id:v.id,key:v.question_id,value:v.new_value}),
   Generation3dMutation::SelectGeneration(v)=>generation(GenerationFoldInput::Select{id:v.generation_id}),
   Generation3dMutation::ChangeGenerationPreview(v)=>generation(GenerationFoldInput::Preview{text:v.text}),
   Generation3dMutation::ChangeWidgetInput(v)=>Ok(NativeProvisionalFold::Additional(P3ProvisionalFold::Input(ProvisionalTypedInputWork::new_original(v,original)))),
   Generation3dMutation::DragTransforms(v)=>Ok(NativeProvisionalFold::Additional(P3ProvisionalFold::Transform(ProvisionalTransformWork::new_original(ProvisionalTransformInput{targets:v.targets,kind:TransformKind::Drag,vector:[v.dx,v.dy,v.dz],angle:0.0},original)))),
   Generation3dMutation::RotateTransforms(v)=>Ok(NativeProvisionalFold::Additional(P3ProvisionalFold::Transform(ProvisionalTransformWork::new_original(ProvisionalTransformInput{targets:v.targets,kind:TransformKind::Rotate,vector:[v.ax,v.ay,v.az],angle:v.angle},original)))),
   Generation3dMutation::ScaleTransforms(v)=>Ok(NativeProvisionalFold::Additional(P3ProvisionalFold::Transform(ProvisionalTransformWork::new_original(ProvisionalTransformInput{targets:v.targets,kind:TransformKind::Scale,vector:[v.sx,v.sy,v.sz],angle:0.0},original)))),

  }
 }
}
