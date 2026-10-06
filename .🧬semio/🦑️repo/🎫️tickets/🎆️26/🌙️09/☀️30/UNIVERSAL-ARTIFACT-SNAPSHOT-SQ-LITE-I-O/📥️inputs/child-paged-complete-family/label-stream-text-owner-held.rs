/// 🔤️ A finite intrinsic semantic cell and genuine long pages have distinct complete borrows.
struct Text{inline:semio_framework_value::paged_text::InlineTextBuffer,complete:bool,paged:PagedText<CAPACITY>,long:bool}
impl Text{
    fn empty()->Self{Self{inline:semio_framework_value::paged_text::InlineTextBuffer::empty(),complete:false,paged:PagedText::empty(),long:false}}
    fn allocated_bytes(&self)->usize{self.paged.allocated_bytes()}
    fn view(&self)->Result<ChildTextView<'_>,ValueError>{if self.long{self.paged.borrow()?;return Ok(ChildTextView::Paged(&self.paged))}if !self.complete{return Err(invalid("child intrinsic text lacks complete UTF8 authority"))}Ok(ChildTextView::Literal(self.inline.borrow()?))}
    fn read_from_source(&mut self,source:&Source<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
        if self.complete||self.long||!self.inline.terminal_is_empty(){return Err(invalid("child semantic text requires an empty owner"))}
        if source.byte_len()>128{self.long=true;return self.paged.read_from_source(source,control)}
        self.inline.read_from_source(source,control)?;self.complete=true;Ok(())
    }
    fn encode_label_with(&mut self,maximum_payload_bytes:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>,producer:impl FnOnce(&mut dyn semio_framework_value::paged_text::TextEncodingOutput)->Result<(),ValueError>)->Result<(),ValueError>{
        if self.complete||self.long||!self.inline.terminal_is_empty(){return Err(invalid("child semantic label requires an empty owner"))}
        self.paged.encode_with_inline(&mut self.inline,&mut self.long,maximum_payload_bytes,control,producer)?;self.complete=true;Ok(())
    }
}
impl ErasedSnapshotRetirement for Text{
    fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,ValueError>{if items==0||bytes==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0})}if self.inline.close_one(1){self.complete=false;return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0})}if self.long{return self.paged.close_step(1,bytes)}Ok(SnapshotRetirementStep::Complete)}
    fn terminal_is_empty(&self)->bool{self.inline.terminal_is_empty()&&self.paged.terminal_is_empty()}
    fn next_close_byte_demand(&self)->usize{if !self.inline.terminal_is_empty(){1}else if self.long{self.paged.next_close_byte_demand()}else{0}}
}
