impl semio_framework_value::retirement::RetireOwned for SetLabelSource{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.value)}
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.value)}
    fn controlled_retirement_supported()->bool{true}
}
impl SetLabel{
    pub(crate) fn source_custody_capacity_bytes()->usize{semio_framework_value::retained_clone::RetainedCloneSource::<SetLabelSource>::owned_constructor_capacity_bytes::<()>()}
    /// 🎟️ Admits this original semantic owner under the same cumulative producer authority before custody allocates.
    pub(crate) fn admit_source_owned(owner:SetLabelSource,control:&mut semio_framework_value::NativeEncodeControl<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(semio_framework_value::retained_clone::RetainedCloneSource<SetLabelSource>,semio_framework_value::retained_clone::RetainedCloneProgress),(semio_framework_value::ValueError,SetLabelSource)>{
        type Source=semio_framework_value::retained_clone::RetainedCloneSource<SetLabelSource>;
        let demand=Source::owned_constructor_demand::<()>();if let Err(error)=demand.admit(grant){return Err((error,owner))}if let Err(error)=control.charge(demand.capacity_bytes){return Err((error,owner))}
        Source::admit_owned(owner,(),grant).map_err(|(error,owner,_)|(error,owner))
    }
}
