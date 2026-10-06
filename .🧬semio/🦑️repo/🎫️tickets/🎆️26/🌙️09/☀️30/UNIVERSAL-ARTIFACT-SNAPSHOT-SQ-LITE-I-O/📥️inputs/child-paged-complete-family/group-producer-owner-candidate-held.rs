// 🫙️ Child semantic text and encoded source owners retain their independent physical lifetimes.
use super::paged_encoder::{ChildTextView,PagedChildGroup,PagedChildGroups,PagedChildLabels};
use super::reader::{ChildGroupDecodeVisitor,ChildGroupText};
use semio_framework_os_kernel::{os_pack::{PackRefusal,codec::ByteSpan},os_spr::operation_bytes::OwnedOperationBytes};
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind,paged_text::{PagedText,TextReadSource},list::PagedList,ErasedSnapshotRetirement,SnapshotRetirementStep};
use semio_framework_ui_locale::{Terminology,Locale};
const CAPACITY:usize=isize::MAX as usize;
/// 🔤️ Short semantic cells stay in their intrinsic wrapper; long cells retain genuine payload pages.
struct Text{inline:[u8;128],length:usize,complete:bool,paged:PagedText<CAPACITY>,long:bool}
impl Text{
    fn empty()->Self{Self{inline:[0;128],length:0,complete:false,paged:PagedText::empty(),long:false}}
    fn allocated_bytes(&self)->usize{self.paged.allocated_bytes()}
    fn view(&self)->Result<ChildTextView<'_>,ValueError>{if self.long{return Ok(ChildTextView::Paged(&self.paged))}if !self.complete{return Err(invalid("child inline text lacks complete UTF8 authority"))}Ok(ChildTextView::Literal(std::str::from_utf8(&self.inline[..self.length]).map_err(|_|invalid("validated child inline UTF8 changed"))?))}
    fn read_from_source(&mut self,source:&Source<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
        if self.complete||self.long||self.length!=0{return Err(invalid("child semantic text requires an empty owner"))}
        if source.byte_len()>self.inline.len(){self.long=true;return self.paged.read_from_source(source,control)}
        control.scoped_stage(|control|{control.begin_stage(source.byte_len())?;for index in 0..source.byte_len(){self.inline[index]=source.byte_at(index).ok_or_else(||invalid("child inline text source changed"))?;self.length+=1;control.step()?;}std::str::from_utf8(&self.inline[..self.length]).map_err(|_|invalid("child inline semantic text is not UTF8"))?;Ok::<_,ValueError>(())})?;self.complete=true;Ok(())
    }
}
impl ErasedSnapshotRetirement for Text{
    fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,ValueError>{if self.long{return self.paged.close_step(items,bytes)}if self.length==0{return Ok(SnapshotRetirementStep::Complete)}if items==0||bytes==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0})}self.length=0;self.complete=false;Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0})}
    fn terminal_is_empty(&self)->bool{self.paged.terminal_is_empty()&&self.length==0}
    fn next_close_byte_demand(&self)->usize{if self.long{self.paged.next_close_byte_demand()}else{usize::from(self.length!=0)}}
}
fn invalid(detail:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,detail)}
struct Source<'a>(ByteSpan<'a>);
impl TextReadSource for Source<'_>{fn byte_len(&self)->usize{self.0.len()}fn byte_at(&self,index:usize)->Option<u8>{self.0.get(index).copied()}}
fn reserve<T>(owner:&mut PagedList<T,CAPACITY>,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{while !owner.has_reserved_slot(){let required=owner.next_allocation_bytes().map_err(ValueError::from)?;if required>4096{return Err(invalid("child outer page exceeds physical grant").into())}control.checkpoint()?;control.charge(required)?;let step=owner.reserve_one(4096).map_err(|error|ValueError::from(error.refusal()))?;if !step.progressed||step.allocated_bytes!=required{return Err(invalid("child outer page differs from its exact admission").into())}}Ok(())}

/// 🏷️ The complete locale matrix owns each typed semantic cell in separate physical pages.
pub struct ChildLabelOwner{cells:[[Text;Locale::COUNT];Terminology::COUNT]}
impl ChildLabelOwner{
    pub fn empty()->Self{Self{cells:std::array::from_fn(|_|std::array::from_fn(|_|Text::empty()))}}
    fn cell(&self,terminology:Terminology,locale:Locale)->&Text{&self.cells[terminology as usize][locale as usize]}
    fn cell_mut(&mut self,terminology:Terminology,locale:Locale)->&mut Text{&mut self.cells[terminology as usize][locale as usize]}
    fn read(&mut self,terminology:Terminology,locale:Locale,source:ByteSpan<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{self.cell_mut(terminology,locale).read_from_source(&Source(source),control).map_err(Into::into)}
}
impl ErasedSnapshotRetirement for ChildLabelOwner{
    fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,ValueError>{for row in &mut self.cells{for cell in row{if !cell.terminal_is_empty(){return cell.close_step(items,bytes)}}}Ok(SnapshotRetirementStep::Complete)}
    fn terminal_is_empty(&self)->bool{self.cells.iter().flatten().all(ErasedSnapshotRetirement::terminal_is_empty)}
    fn next_close_byte_demand(&self)->usize{self.cells.iter().flatten().find(|cell|!cell.terminal_is_empty()).map(ErasedSnapshotRetirement::next_close_byte_demand).unwrap_or(0)}
}
impl PagedChildLabels for PagedList<ChildLabelOwner,CAPACITY>{fn len(&self)->usize{PagedList::len(self)}fn text(&self,label:usize,terminology:Terminology,locale:Locale)->Option<ChildTextView<'_>>{self.get(label).and_then(|label|label.cell(terminology,locale).view().ok())}}

/// 🧩️ Metadata, full operation sources and labels remain real independent owners on success or refusal.
pub struct PagedChildOwner{metadata:[Text;4],pub operations:PagedList<OwnedOperationBytes,CAPACITY>,labels:PagedList<ChildLabelOwner,CAPACITY>,partial_source:Option<OwnedOperationBytes>,partial_label:Option<ChildLabelOwner>,pending_schema:Option<Text>,operation_count:usize,label_count:usize}
impl PagedChildOwner{pub fn empty()->Self{Self{metadata:std::array::from_fn(|_|Text::empty()),operations:PagedList::empty(),labels:PagedList::empty(),partial_source:None,partial_label:None,pending_schema:None,operation_count:0,label_count:0}}
    pub fn allocated_bytes(&self)->usize{self.pending_schema.as_ref().map(Text::allocated_bytes).unwrap_or(0)+self.metadata.iter().map(Text::allocated_bytes).sum::<usize>()+self.operations.allocated_bytes()+self.operations.iter().map(OwnedOperationBytes::allocated_bytes).sum::<usize>()+self.labels.allocated_bytes()+self.labels.iter().flat_map(|label|label.cells.iter().flatten()).map(Text::allocated_bytes).sum::<usize>()+self.partial_source.as_ref().map(OwnedOperationBytes::allocated_bytes).unwrap_or(0)+self.partial_label.as_ref().map(|label|label.cells.iter().flatten().map(Text::allocated_bytes).sum::<usize>()).unwrap_or(0)}
}
impl ErasedSnapshotRetirement for PagedChildOwner{
    fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
        if items==0||bytes==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0})}
        if let Some(schema)=self.pending_schema.as_mut(){if !schema.terminal_is_empty(){return schema.close_step(1,bytes)}self.pending_schema.take();return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0})}
        let index=self.operations.len().checked_sub(1);let source=match self.partial_source.as_mut(){Some(source)=>Some(source),None=>index.and_then(|index|self.operations.get_mut(index))};
        if let Some(source)=source{return Ok(match source.close_one(1,bytes).map_err(|fault|ValueError::new(fault.kind,fault.reason))?{semio_framework_os_kernel::os_spr::operation_bytes::OperationByteCloseStep::Complete=>{if self.partial_source.is_some(){self.partial_source.take();}else{self.operations.pop();}SnapshotRetirementStep::Pending{released_items:1,released_bytes:0}},semio_framework_os_kernel::os_spr::operation_bytes::OperationByteCloseStep::Pending{released_items,released_bytes}=>SnapshotRetirementStep::Pending{released_items,released_bytes}})}
        if !self.operations.terminal_is_empty(){let step=self.operations.release_empty_page(bytes).map_err(ValueError::from)?;return Ok(SnapshotRetirementStep::Pending{released_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes})}
        let index=self.labels.len().checked_sub(1);let label=match self.partial_label.as_mut(){Some(label)=>Some(label),None=>index.and_then(|index|self.labels.get_mut(index))};
        if let Some(label)=label{return match label.close_step(1,bytes)?{SnapshotRetirementStep::Complete=>{if self.partial_label.is_some(){self.partial_label.take();}else{self.labels.pop();}Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0})},step=>Ok(step)}}
        if !self.labels.terminal_is_empty(){let step=self.labels.release_empty_page(bytes).map_err(ValueError::from)?;return Ok(SnapshotRetirementStep::Pending{released_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes})}
        for text in &mut self.metadata{if !text.terminal_is_empty(){return text.close_step(1,bytes)}}Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self)->bool{self.metadata.iter().all(ErasedSnapshotRetirement::terminal_is_empty)&&self.operations.terminal_is_empty()&&self.labels.terminal_is_empty()&&self.partial_source.is_none()&&self.partial_label.is_none()&&self.pending_schema.is_none()}
    fn next_close_byte_demand(&self)->usize{if let Some(schema)=self.pending_schema.as_ref(){return schema.next_close_byte_demand().max(1)}if let Some(source)=self.partial_source.as_ref().or_else(||self.operations.len().checked_sub(1).and_then(|index|self.operations.get(index))){return source.next_close_byte_demand().unwrap_or(1)}if !self.operations.terminal_is_empty(){return self.operations.next_release_allocation_bytes().unwrap_or(1)}if let Some(label)=self.partial_label.as_ref().or_else(||self.labels.len().checked_sub(1).and_then(|index|self.labels.get(index))){return label.next_close_byte_demand().max(1)}if !self.labels.terminal_is_empty(){return self.labels.next_release_allocation_bytes().unwrap_or(1)}self.metadata.iter().find(|text|!text.terminal_is_empty()).map(ErasedSnapshotRetirement::next_close_byte_demand).unwrap_or(0)}
}
impl PagedChildGroups for PagedList<PagedChildOwner,CAPACITY>{fn len(&self)->usize{PagedList::len(self)}fn group(&self,index:usize)->Option<PagedChildGroup<'_>>{self.get(index).and_then(|group|Some(PagedChildGroup{owner:group.metadata[0].view().ok()?,slot:group.metadata[1].view().ok()?,child_id:group.metadata[2].view().ok()?,schema:group.metadata[3].view().ok()?,operations:&group.operations,labels:&group.labels}))}}

/// 🛬️ A preinstalled caller owner retains complete groups plus the actual partial child on every ordinary exit.
pub struct PagedChildDecodeOwner{pub groups:PagedList<PagedChildOwner,CAPACITY>,pub child:Option<PagedChildOwner>,pub symbols:super::paged_encoder::ChildSymbolOwner,started:bool}
impl PagedChildDecodeOwner{pub const fn empty()->Self{Self{groups:PagedList::empty(),child:None,symbols:super::paged_encoder::ChildSymbolOwner::empty(),started:false}}fn child(&mut self)->Result<&mut PagedChildOwner,PackRefusal>{self.child.as_mut().ok_or_else(||invalid("child decoder lacks its retained owner").into())}}
impl ChildGroupDecodeVisitor for PagedChildDecodeOwner{
    fn begin_groups(&mut self,_:usize,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{if self.started||!self.terminal_is_empty(){return Err(invalid("child decode requires one fresh wholly empty retained owner").into())}self.started=true;Ok(())}
    fn begin_group(&mut self,_:usize,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{if self.child.is_some(){return Err(invalid("child decoder retained its prior partial group").into())}self.child=Some(PagedChildOwner::empty());Ok(())}
    fn text(&mut self,field:ChildGroupText,source:ByteSpan<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let index=match field{ChildGroupText::Owner=>0,ChildGroupText::Slot=>1,ChildGroupText::ChildId=>2,ChildGroupText::Schema=>3};self.child()?.metadata[index].read_from_source(&Source(source),control).map_err(Into::into)}
    fn begin_operations(&mut self,count:usize,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{self.child()?.operation_count=count;Ok(())}
    fn begin_operation(&mut self,_:usize,count:usize,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let child=self.child()?;if child.partial_source.is_some(){return Err(invalid("child decoder retained its prior partial operation").into())}child.partial_source=Some(OwnedOperationBytes::try_new(count.max(1),control.maximum_bytes()).map_err(|fault|ValueError::new(fault.kind,fault.reason))?);Ok(())}
    fn operation_byte(&mut self,byte:u8,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let source=self.child()?.partial_source.as_mut().ok_or_else(||invalid("child decoder lacks partial encoded source"))?;while !source.has_reserved_slot(){let required=source.next_allocation_bytes().map_err(|fault|ValueError::new(fault.kind,fault.reason))?;if required>4096{return Err(invalid("child encoded source page exceeds physical grant").into())}control.checkpoint()?;control.charge(required)?;let step=source.reserve_one(4096).map_err(|fault|ValueError::new(fault.kind,fault.reason))?;if !step.progressed||step.allocated_bytes!=required{return Err(invalid("child encoded source admission changed").into())}}source.push_reserved(byte).map_err(|error|ValueError::new(error.fault.kind,error.fault.reason).into())}
    fn end_operation(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let child=self.child()?;reserve(&mut child.operations,control)?;let source=child.partial_source.take().ok_or_else(||invalid("child decoded operation lacks its source"))?;if let Err(source)=child.operations.push_reserved(source){child.partial_source=Some(source);return Err(invalid("child operation lost admitted slot").into())}Ok(())}
    fn begin_labels(&mut self,count:usize,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{self.child()?.label_count=count;Ok(())}
    fn begin_label(&mut self,_:usize,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let child=self.child()?;if child.partial_label.is_some(){return Err(invalid("child decoder retained prior partial label").into())}child.partial_label=Some(ChildLabelOwner::empty());Ok(())}
    fn label_text(&mut self,terminology:Terminology,locale:Locale,source:ByteSpan<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{self.child()?.partial_label.as_mut().ok_or_else(||invalid("child decoder lacks partial semantic label"))?.read(terminology,locale,source,control)}
    fn end_label(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let child=self.child()?;reserve(&mut child.labels,control)?;let label=child.partial_label.take().ok_or_else(||invalid("child decoded label lacks its semantic owner"))?;if let Err(label)=child.labels.push_reserved(label){child.partial_label=Some(label);return Err(invalid("child label lost admitted slot").into())}Ok(())}
    fn end_group(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let child=self.child()?;if child.operation_count!=child.label_count{return Err(invalid("child operation and label prefixes differ").into())}reserve(&mut self.groups,control)?;let child=self.child.take().ok_or_else(||invalid("child decoded group lacks original owner"))?;if let Err(child)=self.groups.push_reserved(child){self.child=Some(child);return Err(invalid("child group lost admitted slot").into())}Ok(())}
    fn end_groups(&mut self,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{if self.child.is_some(){return Err(invalid("complete child decode retains an unfinished group").into())}Ok(())}
}
impl ErasedSnapshotRetirement for PagedChildDecodeOwner{
    fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,ValueError>{if items==0||bytes==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0})}if !self.symbols.terminal_is_empty(){return self.symbols.close_step(1,bytes)}let index=self.groups.len().checked_sub(1);let child=match self.child.as_mut(){Some(child)=>Some(child),None=>index.and_then(|index|self.groups.get_mut(index))};if let Some(child)=child{return match child.close_step(1,bytes)?{SnapshotRetirementStep::Complete=>{if self.child.is_some(){self.child.take();}else{self.groups.pop();}Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0})},step=>Ok(step)}}if self.groups.terminal_is_empty(){return Ok(SnapshotRetirementStep::Complete)}let step=self.groups.release_empty_page(bytes).map_err(ValueError::from)?;Ok(SnapshotRetirementStep::Pending{released_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes})}
    fn terminal_is_empty(&self)->bool{self.child.is_none()&&self.groups.terminal_is_empty()&&self.symbols.terminal_is_empty()}
    fn next_close_byte_demand(&self)->usize{if !self.symbols.terminal_is_empty(){return self.symbols.next_close_byte_demand()}if let Some(child)=self.child.as_ref().or_else(||self.groups.len().checked_sub(1).and_then(|index|self.groups.get(index))){child.next_close_byte_demand().max(1)}else{self.groups.next_release_allocation_bytes().unwrap_or(0)}}
}

/// 🫴️ The caller recipient retains parse and canonical scaffolds on both success and refusal.
pub fn decode_paged_groups_span(source:ByteSpan<'_>,options:&semio_framework_os_kernel::os_pack::codec::PackDecodeOptions,canonical:&semio_framework_os_kernel::os_pack::codec::PackEncodeOptions,decoding:&mut NativeDecodeControl<'_>,encoding:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<PagedList<PagedChildOwner,CAPACITY>,semio_framework_os_kernel::os_spr::ProtocolError>{
    decoding.scoped_maximum(usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX),|decoding|decoding.with_retirement_owner(std::mem::size_of::<PagedChildDecodeOwner>(),|decoding|{
        let mut owner=Box::new(PagedChildDecodeOwner::empty());
        let result=super::reader::visit_groups_span(source,options,decoding,owner.as_mut()).and_then(|()|{let mut comparison=semio_framework_os_kernel::os_spr::operation_bytes::OperationByteComparison::new(source);super::paged_encoder::encode_paged_groups_into(&owner.groups,canonical,&mut comparison,&mut owner.symbols,encoding)?;comparison.finish()?;Ok(std::mem::replace(&mut owner.groups,PagedList::empty()))});
        (result,Some(owner as Box<dyn ErasedSnapshotRetirement>))
    })).map_err(Into::into)
}

impl Text{
    fn return_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize)->Result<semio_framework_value::list::PagedListReturnProgress,ValueError>{
        if self.long{return self.paged.return_one(parent,maximum_items)}
        if maximum_items==0||self.length==0{return Ok(semio_framework_value::list::PagedListReturnProgress::default())}
        self.length=0;self.complete=false;Ok(semio_framework_value::list::PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})
    }
}
impl ChildLabelOwner{
    pub fn return_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize)->Result<semio_framework_value::list::PagedListReturnProgress,ValueError>{
        for row in &mut self.cells{for text in row{if !text.terminal_is_empty(){return text.return_one(parent,maximum_items)}}}Ok(semio_framework_value::list::PagedListReturnProgress::default())
    }
}
impl PagedChildOwner{
    pub fn return_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize,maximum_bytes:usize)->Result<semio_framework_value::list::PagedListReturnProgress,ValueError>{
        use semio_framework_os_kernel::os_spr::operation_bytes::OperationByteReturnStep;
        use semio_framework_value::list::PagedListReturnProgress;
        if maximum_items==0||maximum_bytes==0{return Ok(PagedListReturnProgress::default())}
        if let Some(schema)=self.pending_schema.as_mut(){if !schema.terminal_is_empty(){return schema.return_one(parent,1)}self.pending_schema.take();return Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})}
        let index=self.operations.len().checked_sub(1);let source=match self.partial_source.as_mut(){Some(source)=>Some(source),None=>index.and_then(|index|self.operations.get_mut(index))};
        if let Some(source)=source{return Ok(match source.return_one(parent,1,maximum_bytes)?{OperationByteReturnStep::Pending{returned_items,returned_bytes}=>PagedListReturnProgress{progressed:returned_items!=0,returned_allocation_bytes:returned_bytes},OperationByteReturnStep::Complete=>{if self.partial_source.is_some(){self.partial_source.take();}else{self.operations.pop();}PagedListReturnProgress{progressed:true,returned_allocation_bytes:0}}})}
        if !self.operations.terminal_is_empty(){return self.operations.return_empty_page(parent,1)}
        let index=self.labels.len().checked_sub(1);let label=match self.partial_label.as_mut(){Some(label)=>Some(label),None=>index.and_then(|index|self.labels.get_mut(index))};
        if let Some(label)=label{if !label.terminal_is_empty(){return label.return_one(parent,1)}if self.partial_label.is_some(){self.partial_label.take();}else{self.labels.pop();}return Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})}
        if !self.labels.terminal_is_empty(){return self.labels.return_empty_page(parent,1)}
        for text in &mut self.metadata{if !text.terminal_is_empty(){return text.return_one(parent,1)}}Ok(PagedListReturnProgress::default())
    }
}

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
