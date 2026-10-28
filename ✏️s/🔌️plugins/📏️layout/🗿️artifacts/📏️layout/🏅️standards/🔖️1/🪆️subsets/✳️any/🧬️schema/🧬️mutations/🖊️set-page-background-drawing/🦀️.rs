//! 🖊️ Page Drawing commands publish owned child changes through the central sparse page applier.
use semio_s_artifact_layout_layout_diff::{LayoutDiff,LayoutPagesDelta,LayoutPagesModification,PagePatch};
use semio_s_artifact_layout_layout_model::{LayoutDrawingMember,snapshot::LayoutSnapshot};
use protocol::{MutationKind,SemanticDescriptor};
use semio_framework_value_derive::{FromValue,ToValue};
#[derive(Clone,Debug,PartialEq,dsl::MutationLeaf,ToValue,FromValue,semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::RetainedClone)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test,feature="testkit"),derive(serde::Serialize,serde::Deserialize))]
#[cfg_attr(any(test,feature="testkit"),serde(rename_all="camelCase",deny_unknown_fields))]
#[mutation_leaf(contract=::protocol)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct SetPageBackgroundDrawing {pub page_id:String,pub drawing:Option<LayoutDrawingMember>}
impl<Op> MutationKind<LayoutSnapshot,Op> for SetPageBackgroundDrawing where Op:protocol::Mutation<LayoutSnapshot,Diff=LayoutDiff>+From<crate::set_page_background_drawing::SetPageBackgroundDrawing>{
 const SEMANTICS:SemanticDescriptor=SemanticDescriptor{verb:"set",entity:"page-background-drawing",kind:"set-page-background-drawing",record:"SetPageBackgroundDrawing"};
 fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native("Set page Drawing","Seitenzeichnung setzen")}
 fn target(&self)->Vec<String>{vec![self.page_id.clone()]}
}
impl<Op> protocol::ColdMutationKind<LayoutSnapshot,Op> for SetPageBackgroundDrawing where Op:protocol::ColdMutation<LayoutSnapshot,ReferenceDiff=LayoutDiff>+protocol::Mutation<LayoutSnapshot,Diff=LayoutDiff>+From<crate::set_page_background_drawing::SetPageBackgroundDrawing>{
 fn diff(&self,base:&LayoutSnapshot)->protocol::MutationOutcome<LayoutDiff>{diff_set_page_background_drawing(self,base)}
 fn inverse(&self,base:&LayoutSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError>{if diff_set_page_background_drawing(self,base).diff()==&LayoutDiff::default(){return Ok(Vec::new())}Ok(base.pages.iter().find(|page|page.id==self.page_id).map(|page|vec![Op::from(Self{page_id:self.page_id.clone(),drawing:page.background_drawing.clone()})]).unwrap_or_default())}
}
/// 🔺️ Validates only the new child and publishes its precise missing/set/clear event.
pub fn diff_set_page_background_drawing(payload:&SetPageBackgroundDrawing,base:&LayoutSnapshot)->protocol::MutationOutcome<LayoutDiff>{
 let Some(page)=base.pages.iter().find(|page|page.id==payload.page_id)else{return protocol::MutationOutcome::error("mutation.target-missing","The page does not exist.",[payload.page_id.clone()])};
 if let Some(child)=&payload.drawing{if let Err(message)=semio_s_artifact_layout_layout_model::validation::validate_page_drawing_assignment(base,&payload.page_id,child){return protocol::MutationOutcome::fatal("mutation.invariant",message,std::iter::empty::<String>())}}
 if page.background_drawing==payload.drawing{return protocol::MutationOutcome::empty().warning("mutation.no-op","Page Drawing is already set.")}
 protocol::MutationOutcome::new(LayoutDiff{pages:Some(LayoutPagesDelta{modified:vec![LayoutPagesModification{id:payload.page_id.clone(),patch:PagePatch{background_drawing:Some(payload.drawing.clone()),..Default::default()}}],..Default::default()}),..Default::default()})
}
