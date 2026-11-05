//! 🪪️ One immutable original owns paid schema sources, entered values and genuine semantic admission.
use semio_framework::{ActionArgDef,ActionArgShapeSeed,ArgSchema};
use semio_framework::action_staging::{StagedFormRequest,StagedFormStatus};
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind,retained_clone::RetainedCloneSource,shared_issuance::SealedShared};
pub type NormInspectorDescriptorSource=super::static_source::NormInspectorPreparedStaticSource<ActionArgDef>;
pub type NormInspectorSeedSource=super::static_source::NormInspectorPreparedStaticSource<ActionArgShapeSeed>;
/// 🛤️ Static descriptor indices identify one original schema without cloning its tree.
#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub struct NormInspectorSchemaPath{pub indices:[usize;64],pub len:usize}
impl NormInspectorSchemaPath{
 pub fn new(path:&[usize])->Result<Self,ValueError>{if path.is_empty()||path.len()>64{return Err(invalid())}let mut indices=[0;64];indices[..path.len()].copy_from_slice(path);Ok(Self{indices,len:path.len()})}
 pub fn as_slice(&self)->&[usize]{&self.indices[..self.len]}
}
/// 🎯️ Draft arguments retain their original insertion or complete-replacement boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum NormInspectorDraftKey{Value,Replacement}
impl NormInspectorDraftKey{pub fn as_str(self)->&'static str{match self{Self::Value=>"value",Self::Replacement=>"replacement"}}}
/// 🔀️ The paid semantic producer supplies one complete dynamic parent replacement for an exact entered path and variant.
#[derive(semio_framework_value::RetireOwned)]
pub struct NormInspectorPreparedVariant{pub option_index:usize,pub value:DslValue}
/// 🧩️ One encountered selector has one directly indexed replacement per declared option.
#[derive(semio_framework_value::RetireOwned)]
pub struct NormInspectorPreparedVariantGroup{pub schema_row:usize,pub value_path:Vec<usize>,pub options:Vec<NormInspectorPreparedVariant>}
/// 🧭️ Static shape seeds are separate from document-dependent complete-parent replacements.
#[derive(semio_framework_value::RetireOwned)]
pub struct NormInspectorPreparedSemantics{pub initial:Option<DslValue>,pub variants:Vec<NormInspectorPreparedVariantGroup>}
/// 📚️ Ownership transfer carries already-paid values; page preparation never invokes the cold plan or admission oracle.
#[derive(semio_framework_value::RetireOwned)]
pub struct NormInspectorPageOriginal{
 pub descriptors:NormInspectorDescriptorSource,pub seeds:NormInspectorSeedSource,
 pub request:StagedFormRequest,pub status:StagedFormStatus,pub schema_path:NormInspectorSchemaPath,
 pub draft_key:NormInspectorDraftKey,pub controller:String,pub semantics:NormInspectorPreparedSemantics,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"norm.inspector.source")}
impl NormInspectorPageOriginal{
 pub fn schema(&self,path:NormInspectorSchemaPath)->Result<(&ActionArgDef,&ArgSchema),ValueError>{
  if path.len==0||path.len>64{return Err(invalid())}
  let inputs=self.descriptors.borrow();let mut input=inputs.get().get(path.indices[0]).ok_or_else(invalid)?;let mut schema=&input.schema;
  for index in &path.indices[1..path.len]{match schema{ArgSchema::Object{fields}=>{input=fields.get(*index).ok_or_else(invalid)?;schema=&input.schema},ArgSchema::Array{items,..}if *index==0=>schema=items,_=>return Err(invalid())}}
  Ok((input,schema))
 }
 pub fn validate(&self)->Result<(),ValueError>{
  if !self.descriptors.same_compiled_source(&self.seeds){return Err(invalid())}
  if !self.request.token.admitted()||self.status.token!=self.request.token||!self.status.admitted()||self.controller.is_empty()||self.request.args.as_object().is_none()||self.request.page.as_ref().is_some_and(|page|!page.admitted()){return Err(invalid())}
  self.schema(self.schema_path)?;Ok(())
 }
}


/// 🪢️ The page owner keeps this actual original source until every lent native projection returns.
pub type NormInspectorOriginalSource=RetainedCloneSource<NormInspectorPageOriginal>;
/// 🌱️ Transport can quote retain_owned on this exact seed borrow before publishing a scoped reference.
pub fn norm_inspector_seed_source<'a>(original:semio_framework_value::retained_clone::RetainedCloneRef<'a,NormInspectorPageOriginal>,index:usize)->Result<semio_framework_value::retained_clone::RetainedCloneRef<'a,DslValue>,ValueError>{
 original.project(0,|original|&original.seeds).try_project(index,|source|source.borrow().get().get(index).map(|seed|&seed.value).ok_or_else(invalid))
}
/// 🚦️ Only genuine prepared commit arguments can be lent; an unready status supplies no command source.
pub fn norm_inspector_commit_source(original:semio_framework_value::retained_clone::RetainedCloneRef<'_,NormInspectorPageOriginal>)->Result<semio_framework_value::retained_clone::RetainedCloneRef<'_,DslValue>,ValueError>{
 original.project(1,|original|&original.status).try_project(0,|status|if status.ready{status.commit_args.as_ref().ok_or_else(invalid)}else{Err(invalid())})
}
/// 🔀️ Complete document-dependent replacements retain their exact prepared original owner too.
pub fn norm_inspector_variant_source<'a>(original:semio_framework_value::retained_clone::RetainedCloneRef<'a,NormInspectorPageOriginal>,group:usize,option:usize)->Result<semio_framework_value::retained_clone::RetainedCloneRef<'a,DslValue>,ValueError>{
 original.project(2,|original|&original.semantics.variants).try_project(group,|groups|groups.get(group).ok_or_else(invalid))?.try_project(option,|group|group.options.get(option).filter(|value|value.option_index==option).map(|value|&value.value).ok_or_else(invalid))
}
/// 🪄️ A complete dynamic initial replacement remains a separate original projection from static shape seeds.
pub fn norm_inspector_initial_source(original:semio_framework_value::retained_clone::RetainedCloneRef<'_,NormInspectorPageOriginal>)->Result<semio_framework_value::retained_clone::RetainedCloneRef<'_,DslValue>,ValueError>{
 original.try_project(3,|original|original.semantics.initial.as_ref().ok_or_else(invalid))
}

/// 🧺️ Missing-array Add lends the actual compiled one-item array, separately from the parent default and item seed.
pub fn norm_inspector_singleton_source<'a>(original:semio_framework_value::retained_clone::RetainedCloneRef<'a,NormInspectorPageOriginal>,index:usize)->Result<semio_framework_value::retained_clone::RetainedCloneRef<'a,DslValue>,ValueError>{
 original.project(0,|original|&original.seeds).try_project(index,|source|source.borrow().get().get(index).ok_or_else(invalid))?.try_project(1,|row|row.singleton.as_ref().ok_or_else(invalid))
}
