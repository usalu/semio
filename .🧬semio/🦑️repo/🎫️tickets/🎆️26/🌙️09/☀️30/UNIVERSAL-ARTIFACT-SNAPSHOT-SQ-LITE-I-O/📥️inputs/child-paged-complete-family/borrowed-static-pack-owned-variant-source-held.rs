/// 🧬️ Retains the genuinely decoded variant alongside its canonical encoding scratch.
pub struct OwnedVariantPackSource<T:semio_framework_dsl_record::DslVariants>{source:T}
impl<T:semio_framework_dsl_record::DslVariants> FieldProjectionSource for OwnedVariantPackSource<T>{
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{self.source.projected_variant_view(path)}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.source.projected_variant_key(path,index)}
}
impl<T:semio_framework_dsl_record::DslVariants+semio_framework_dsl_record::BorrowedDslVariants> BorrowedProjectedPackOperation<OwnedVariantPackSource<T>>{
 /// 📥️ Takes the actual decoded semantic owner before any canonical reencoding demand.
 pub fn from_owned_variant(source:T)->Self{let(keyword,ordinal,spec)=source.projected_borrowed_variant_identity();let mut operation=Self::new(OwnedVariantPackSource{source},spec);operation.variant_identity=Some((keyword,ordinal));operation}
 /// 🏷️ Reads the captured header identity from the same retained decoded variant as the body.
 pub fn variant_identity(&self)->Result<(&'static str,usize),PackRefusal>{if !matches!(self.phase,OperationPhase::Fresh|OperationPhase::Ready){return Err(mismatch())}self.variant_identity.ok_or_else(mismatch)}
 /// 🔎️ Borrows the original semantic owner without releasing canonical scratch.
 pub fn retained_source(&self)->&T{&self.source.source}
 /// 📤️ Transfers the original semantic owner only after all scratch scalars and backing settle.
 pub fn take_settled_source(self)->Result<T,Self>{if self.phase!=OperationPhase::Retiring||!self.order.is_empty()||self.allocated_bytes()!=0||self.symbols.first.is_some(){return Err(self)}Ok(self.source.source)}
}
