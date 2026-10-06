impl ChildOperationSourceReceipt<'_>{
    /// 🏷️ The authored producer writes into the retained locale cell under the same full caller policy.
    pub fn produce_label(&mut self,terminology:Terminology,locale:Locale,control:&mut semio_framework_value::NativeEncodeControl<'_>,producer:impl FnOnce(&mut dyn semio_framework_value::paged_text::TextEncodingOutput)->Result<(),ValueError>)->Result<(),PackRefusal>{
        let maximum=self.options.limits.max_segment_len.min(usize::MAX as u64)as usize;
        semio_framework_os_kernel::os_spr::operation_bytes::with_operation_encode_policy(self.options,control,|control|self.owner.partial_label.as_mut().ok_or_else(||invalid("child label producer lacks its retained cell owner"))?.cell_mut(terminology,locale).encode_label_with(maximum,control,producer).map_err(Into::into))
    }
}
