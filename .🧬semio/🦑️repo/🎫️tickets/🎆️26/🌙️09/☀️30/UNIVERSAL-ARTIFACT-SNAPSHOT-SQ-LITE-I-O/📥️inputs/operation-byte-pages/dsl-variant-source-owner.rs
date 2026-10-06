
/// 🫳️ Borrows a declared original operation variant without owning a Record payload mirror.
pub struct VariantProjection<'a,T:DslVariants>{source:&'a T}
impl<'a,T:DslVariants> VariantProjection<'a,T>{
    /// 🌿️ Keeps the exact original variant immutable through canonical emission.
    pub fn new(source:&'a T)->Self{Self{source}}
}
impl<T:DslVariants> FieldProjectionSource for VariantProjection<'_,T>{
    fn projection_view(&self,path:&[usize])->Result<FieldProjectionView<'_>,ValueError>{self.source.projected_variant_view(path)}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.source.projected_variant_key(path,index)}
}
