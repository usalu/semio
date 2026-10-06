    /// 🏷️ Reads the current authored variant and its literal ordinal without allocating labels.
    fn projected_variant_identity(&self)->(&'static str,usize,RecordSpecProducer);
    /// 🫳️ Borrows one original variant field by its declared ordinal path.
    fn projected_variant_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>;
    /// 🔑️ Borrows a key from the original variant's ranked field source.
    fn projected_variant_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>;
