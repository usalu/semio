impl Text{
    fn read_from_encoding_source(&mut self,source:&impl TextReadSource,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ValueError>{
        if self.complete||self.long||self.length!=0{return Err(invalid("child semantic text requires an empty producer owner"))}
        if source.byte_len()>self.inline.len(){self.long=true;return self.paged.read_from_encoding_source(source,control)}
        control.scoped_stage(|control|{control.begin_stage(source.byte_len())?;for index in 0..source.byte_len(){self.inline[index]=source.byte_at(index).ok_or_else(||invalid("child inline producer source changed"))?;self.length+=1;control.step()?;}std::str::from_utf8(&self.inline[..self.length]).map_err(|_|invalid("child inline producer semantic text is not UTF8"))?;Ok::<_,ValueError>(())})?;self.complete=true;Ok(())
    }
}
fn reserve_encoded<T>(owner:&mut PagedList<T,CAPACITY>,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),PackRefusal>{
    while !owner.has_reserved_slot(){let required=owner.next_allocation_bytes().map_err(ValueError::from)?;if required>4096{return Err(invalid("child producer outer page exceeds physical grant").into())}control.checkpoint()?;control.charge(required)?;let step=owner.reserve_one(4096).map_err(|error|ValueError::from(error.refusal()))?;if !step.progressed||step.allocated_bytes!=required{return Err(invalid("child producer outer page differs from exact admission").into())}}Ok(())
}
impl ChildLabelOwner{
    pub fn read_from_encoding_source(&mut self,terminology:Terminology,locale:Locale,source:&impl TextReadSource,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),PackRefusal>{self.cell_mut(terminology,locale).read_from_encoding_source(source,control).map_err(Into::into)}
}
impl PagedChildOwner{
    pub fn view(&self)->Result<PagedChildGroup<'_>,ValueError>{Ok(PagedChildGroup{owner:self.metadata[0].view()?,slot:self.metadata[1].view()?,child_id:self.metadata[2].view()?,schema:self.metadata[3].view()?,operations:&self.operations,labels:&self.labels})}
    pub fn read_metadata_from_encoding_source(&mut self,field:ChildGroupText,source:&impl TextReadSource,options:&semio_framework_os_kernel::os_pack::codec::PackEncodeOptions,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),PackRefusal>{if source.byte_len()as u64>options.limits.max_segment_len{return Err(invalid("child metadata exceeds full caller segment policy").into())}let index=match field{ChildGroupText::Owner=>0,ChildGroupText::Slot=>1,ChildGroupText::ChildId=>2,ChildGroupText::Schema=>3};semio_framework_os_kernel::os_spr::operation_bytes::with_operation_encode_policy(options,control,|control|self.metadata[index].read_from_encoding_source(source,control).map_err(Into::into))}
    fn begin_owned_operation(&mut self,maximum_payload_bytes:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),PackRefusal>{
        if self.partial_source.is_some()||self.partial_label.is_some()||self.pending_schema.is_some(){return Err(invalid("child producer retains an earlier partial operation, label or schema").into())}
        self.partial_source=Some(OwnedOperationBytes::try_new(maximum_payload_bytes,control.maximum_bytes()).map_err(|fault|ValueError::new(fault.kind,fault.reason))?);self.partial_label=Some(ChildLabelOwner::empty());Ok(())
    }
    pub fn produce_owned_operation<'a>(&'a mut self,maximum_payload_bytes:usize,options:&'a semio_framework_os_kernel::os_pack::codec::PackEncodeOptions,control:&mut semio_framework_value::NativeEncodeControl<'_>,encode:impl FnOnce(&semio_framework_os_kernel::os_pack::codec::PackEncodeOptions,&mut dyn semio_framework_os_kernel::os_spr::operation_bytes::OperationByteOutput,&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),semio_framework_os_kernel::os_spr::ProtocolError>)->Result<ChildOperationSourceReceipt<'a>,semio_framework_os_kernel::os_spr::ProtocolError>{
        semio_framework_os_kernel::os_spr::operation_bytes::with_operation_encode_policy(options,control,|control|{self.begin_owned_operation(maximum_payload_bytes,control)?;let mut limited=semio_framework_os_kernel::os_spr::operation_bytes::OperationByteLimitedOutput::new(self.partial_source.as_mut().unwrap(),options.limits.max_file_len);encode(options,&mut limited,control)})?;Ok(ChildOperationSourceReceipt{owner:self,options})
    }
    fn commit_owned_operation(&mut self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),PackRefusal>{
        if self.partial_source.is_none()||self.partial_label.is_none(){return Err(invalid("child producer lacks its complete operation and semantic label").into())}
        if self.partial_label.as_ref().unwrap().cells.iter().flatten().any(|cell|cell.view().is_err()){return Err(invalid("child producer cannot publish an incomplete locale cell").into())}
        let first=self.operations.is_empty();if first&&self.pending_schema.as_ref().is_none_or(|schema|schema.view().is_err()){return Err(invalid("first child operation lacks its complete pending schema").into())}
        let next=self.operations.len().checked_add(1).ok_or_else(||invalid("child accepted prefix count overflow"))?;reserve_encoded(&mut self.operations,control)?;reserve_encoded(&mut self.labels,control)?;
        let source=self.partial_source.take().unwrap();if let Err(source)=self.operations.push_reserved(source){self.partial_source=Some(source);return Err(invalid("child producer operation lost its preadmitted slot").into())}
        let label=self.partial_label.take().unwrap();if let Err(label)=self.labels.push_reserved(label){self.partial_label=Some(label);self.partial_source=self.operations.pop();return Err(invalid("child producer label lost its preadmitted slot").into())}
        if first{let schema=self.pending_schema.take().unwrap();self.pending_schema=Some(std::mem::replace(&mut self.metadata[3],schema));}self.operation_count=next;self.label_count=next;Ok(())
    }
}
/// 🧾 A linear borrow is issued only after the real direct encoder accepts its entire source.
pub struct ChildOperationSourceReceipt<'a>{owner:&'a mut PagedChildOwner,options:&'a semio_framework_os_kernel::os_pack::codec::PackEncodeOptions}
impl ChildOperationSourceReceipt<'_>{
    pub fn read_first_schema_from_source(&mut self,source:&impl TextReadSource,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),PackRefusal>{if source.byte_len()as u64>self.options.limits.max_segment_len{return Err(invalid("first child schema exceeds full caller segment policy").into())}if !self.owner.operations.is_empty()||self.owner.pending_schema.is_some(){return Err(invalid("child first schema requires the original empty accepted prefix").into())}self.owner.pending_schema=Some(Text::empty());semio_framework_os_kernel::os_spr::operation_bytes::with_operation_encode_policy(self.options,control,|control|self.owner.pending_schema.as_mut().unwrap().read_from_encoding_source(source,control).map_err(Into::into))}
    pub fn read_label_from_source(&mut self,terminology:Terminology,locale:Locale,source:&impl TextReadSource,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),PackRefusal>{if source.byte_len()as u64>self.options.limits.max_segment_len{return Err(invalid("child label exceeds full caller segment policy").into())}semio_framework_os_kernel::os_spr::operation_bytes::with_operation_encode_policy(self.options,control,|control|self.owner.partial_label.as_mut().ok_or_else(||invalid("child producer lacks its actual partial semantic label"))?.read_from_encoding_source(terminology,locale,source,control))}
    pub fn commit(self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),PackRefusal>{semio_framework_os_kernel::os_spr::operation_bytes::with_operation_encode_policy(self.options,control,|control|self.owner.commit_owned_operation(control))}
}
