//! 🔖️ Scalar choice views borrow decoded declaration strings and the already-selected page directly.
use super::{page::{NormInspectorPreparedPage,Control},original::NormInspectorPageOriginal,walk::{self,Kind}};
use semio_framework::ArgSchema;
use semio_framework_plugin_decl_staged_form::{StagedScalarChoice,StagedScalarChoiceSource};
use semio_framework_ui_locale::{Locale,Terminology};
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"norm.inspector.choice-source")}
/// 🍱 A borrowed page keeps its current option resident without copying labels, keys or whole option vectors.
pub struct NormInspectorBorrowedChoices<'page>{original:&'page NormInspectorPageOriginal,control:&'page Control}
impl NormInspectorPreparedPage{
 /// 👓️ The owning page retains every referenced native descriptor for the complete view lifetime.
 pub fn scalar_choices(&self,index:usize)->Result<Option<NormInspectorBorrowedChoices<'_>>,ValueError>{
  let control=self.controls.get(index).and_then(Option::as_ref).ok_or_else(invalid)?;
  if !matches!(control.candidate.kind,Kind::Boolean|Kind::Choice{..}){return Ok(None)}
  let original=self.borrow_original()?.get();Ok(Some(NormInspectorBorrowedChoices{original,control}))
 }
 /// 🎯️ The funded preparation scan already selected this declared key; access never searches the option vector.
 pub fn scalar_selection(&self,index:usize)->Result<Option<&str>,ValueError>{
  let control=self.controls.get(index).and_then(Option::as_ref).ok_or_else(invalid)?;
  let original=self.borrow_original()?.get();
  match control.candidate.kind{
   Kind::Boolean=>Ok(Some(match control.candidate.present.then(||walk::value(original,&control.values[..control.candidate.value_len])).flatten(){Some(DslValue::Bool(true))=>"true",Some(DslValue::Bool(false))=>"false",_=>""})),
   Kind::Choice{..}=>{let(_,schema)=walk::schema(original,control.candidate.node)?;let ArgSchema::String{options,..}=schema else{return Err(invalid())};Ok(Some(match control.selected{Some(index)=>options.get(index).ok_or_else(invalid)?.value.as_str(),None=>""}))},
   _=>Ok(None),
  }
 }
}
impl StagedScalarChoiceSource for NormInspectorBorrowedChoices<'_>{
 fn len(&self)->usize{match self.control.candidate.kind{Kind::Boolean=>2,Kind::Choice{..}=>self.control.choice.map_or(0,|page|page.len()),_=>0}}
 fn choice(&self,index:usize)->Option<StagedScalarChoice<'_>>{
  if matches!(self.control.candidate.kind,Kind::Boolean){return match(index,self.original.request.locale){(0,Locale::En)=>Some(StagedScalarChoice{key:"false",label:"No"}),(0,Locale::De)=>Some(StagedScalarChoice{key:"false",label:"Nein"}),(1,Locale::En)=>Some(StagedScalarChoice{key:"true",label:"Yes"}),(1,Locale::De)=>Some(StagedScalarChoice{key:"true",label:"Ja"}),_=>None}}
  let index=self.control.choice?.index(index)?;let(_,schema)=walk::schema(self.original,self.control.candidate.node).ok()?;
  let ArgSchema::String{options,..}=schema else{return None};let option=options.get(index)?;
  Some(StagedScalarChoice{key:&option.value,label:option.label.resolve(Terminology::Native,self.original.request.locale)})
 }
}
