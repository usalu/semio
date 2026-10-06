// 🫴️ One pre-admitted recipient owns all decoded child groups and every partial semantic field.
use super::{ChildEmit,PluginCloseStep};
use super::source_codec::{ChildGroupDecodeVisitor,ChildGroupText,visit_groups_span,ChildGroupSources,ChildGroupSource,encode_groups_into};
use semio_framework_os_kernel::os_spr::{self,codec::{ByteSpan,PackDecodeOptions,PackEncodeOptions},operation_bytes::OwnedOperationBytes};
use semio_framework_os_kernel::os_pack::PackRefusal;
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,SnapshotRetirementStep,list::PagedList};
use semio_framework_ui_locale::{LocalizedLabel,Terminology,Locale};
const MAXIMUM_GROUPS:usize=isize::MAX as usize;

impl<const N:usize> ChildGroupSources for PagedList<ChildEmit,N>{
    fn len(&self)->usize{PagedList::len(self)}
    fn group_at(&self,index:usize)->Option<ChildGroupSource<'_>>{self.get(index).map(|child|ChildGroupSource{owner:&child.owner,slot:&child.slot,child_id:&child.child_id,schema:&child.op_schema.0,operations:&child.ops,labels:&child.labels})}
}
fn invariant(detail:&'static str)->PackRefusal{ValueError::new(ValueRefusalKind::InvariantViolated,detail).into()}
fn reserve<T,const N:usize>(owner:&mut PagedList<T,N>,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{
    while !owner.has_reserved_slot(){let bytes=owner.next_allocation_bytes().map_err(ValueError::from)?;if bytes>4096{return Err(invariant("decoded child collection page exceeds its exact production grant"))}control.checkpoint()?;control.charge(bytes)?;let step=owner.reserve_one(4096).map_err(|error|ValueError::from(error.refusal()))?;if !step.progressed{return Err(invariant("decoded child collection admitted no real page"))}}
    Ok(())
}
fn text_into(source:ByteSpan<'_>,target:&mut String,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{
    if !target.is_empty()||target.capacity()!=0{return Err(invariant("decoded child text has already retained an original owner"))}
    control.charge(source.len())?;target.try_reserve_exact(source.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"decoded child semantic text allocation failed"))?;
    let mut position=0;
    while position<source.len(){control.checkpoint()?;let first=source[position];let length=match first{0..=127=>1,194..=223=>2,224..=239=>3,240..=244=>4,_=>return Err(invariant("validated child text lead octet changed"))};let mut bytes=[0;4];for(index,byte)in bytes[..length].iter_mut().enumerate(){*byte=*source.get(position+index).ok_or_else(||invariant("validated child text range changed"))?;}let character=std::str::from_utf8(&bytes[..length]).map_err(|_|invariant("validated child text scalar changed"))?.chars().next().ok_or_else(||invariant("validated child text has no scalar"))?;target.push(character);position+=length;control.advance(length)?;}
    Ok(())
}

/// 🫙️ Keeps the complete accepted group prefix, current child, label and text on every ordinary exit.
struct ChildGroupDecodeOwner{groups:PagedList<ChildEmit,MAXIMUM_GROUPS>,child:Option<ChildEmit>,label:Option<LocalizedLabel>,text:String,operations:usize,labels:usize}
impl ChildGroupDecodeOwner{
    fn new()->Self{Self{groups:PagedList::empty(),child:None,label:None,text:String::new(),operations:0,labels:0}}
    fn child(&mut self)->Result<&mut ChildEmit,PackRefusal>{self.child.as_mut().ok_or_else(||invariant("child decoder has no retained current child"))}
}
impl ChildGroupDecodeVisitor for ChildGroupDecodeOwner{
    fn begin_groups(&mut self,_:usize,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{Ok(())}
    fn begin_group(&mut self,_:usize,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{if self.child.is_some(){return Err(invariant("child decoder retains its previous partial child"))}self.child=Some(ChildEmit::empty());Ok(())}
    fn text(&mut self,field:ChildGroupText,source:ByteSpan<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let child=self.child()?;let target=match field{ChildGroupText::Owner=>&mut child.owner,ChildGroupText::Slot=>&mut child.slot,ChildGroupText::ChildId=>&mut child.child_id,ChildGroupText::Schema=>&mut child.op_schema.0};text_into(source,target,control)}
    fn begin_operations(&mut self,count:usize,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{self.operations=count;Ok(())}
    fn begin_operation(&mut self,_:usize,count:usize,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let child=self.child()?;if child.partial.is_some(){return Err(invariant("child decoder retains its previous partial operation"))}child.partial=Some(OwnedOperationBytes::try_new(count.max(1),control.maximum_bytes()).map_err(|error|ValueError::new(error.kind,error.reason))?);Ok(())}
    fn operation_byte(&mut self,byte:u8,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let source=self.child()?.partial.as_mut().ok_or_else(||invariant("child decoder has no retained partial operation"))?;while !source.has_reserved_slot(){let bytes=source.next_allocation_bytes().map_err(|error|ValueError::new(error.kind,error.reason))?;if bytes>4096{return Err(invariant("decoded child operation page exceeds its production grant"))}control.checkpoint()?;control.charge(bytes)?;if !source.reserve_one(4096).map_err(|error|ValueError::new(error.kind,error.reason))?.progressed{return Err(invariant("decoded child operation admitted no real backing"))}}source.push_reserved(byte).map_err(|error|ValueError::new(error.fault.kind,error.fault.reason).into())}
    fn end_operation(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{let child=self.child()?;reserve(&mut child.ops,control)?;let source=child.partial.take().ok_or_else(||invariant("decoded child operation has no retained source"))?;if let Err(source)=child.ops.push_reserved(source){child.partial=Some(source);return Err(invariant("decoded child operation lost its admitted slot"))}Ok(())}
    fn begin_labels(&mut self,count:usize,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{self.labels=count;Ok(())}
    fn begin_label(&mut self,_:usize,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{if self.label.is_some(){return Err(invariant("child decoder retains its previous partial label"))}self.label=Some(LocalizedLabel::default());Ok(())}
    fn label_text(&mut self,terminology:Terminology,locale:Locale,source:ByteSpan<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{text_into(source,&mut self.text,control)?;let text=std::mem::take(&mut self.text);let label=self.label.as_mut().ok_or_else(||invariant("child decoder has no retained current label"))?;if let Err(text)=label.set_owned_cell(terminology,locale,text){self.text=text;return Err(invariant("decoded locale cell was already occupied"))}Ok(())}
    fn end_label(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{reserve(&mut self.child()?.labels,control)?;let label=self.label.take().ok_or_else(||invariant("child decoder has no completed label"))?;if let Err(label)=self.child()?.labels.push_reserved(label){self.label=Some(label);return Err(invariant("decoded label lost its admitted collection slot"))}Ok(())}
    fn end_group(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{if self.operations!=self.labels{return Err(invariant("decoded child operation and semantic label prefixes differ"))}reserve(&mut self.groups,control)?;let child=self.child.take().ok_or_else(||invariant("child decoder has no completed child"))?;if let Err(child)=self.groups.push_reserved(child){self.child=Some(child);return Err(invariant("decoded child lost its admitted group slot"))}Ok(())}
    fn end_groups(&mut self,_:&mut NativeDecodeControl<'_>)->Result<(),PackRefusal>{if self.child.is_some()||self.label.is_some()||self.text.capacity()!=0{return Err(invariant("completed child parser still owns a partial semantic field"))}Ok(())}
}
impl ErasedSnapshotRetirement for ChildGroupDecodeOwner{
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
        if maximum_items==0||maximum_bytes==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0})}
        if self.text.capacity()!=0{let bytes=self.text.capacity();if bytes>maximum_bytes{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0})}self.text=String::new();return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:bytes})}
        if let Some(label)=self.label.as_mut(){return Ok(match label.close_owned_cell_one(maximum_bytes){Ok(Some(bytes))=>SnapshotRetirementStep::Pending{released_items:1,released_bytes:bytes},Ok(None)=>{self.label.take();SnapshotRetirementStep::Pending{released_items:1,released_bytes:0}},Err(_)=>SnapshotRetirementStep::Pending{released_items:0,released_bytes:0}})}
        let index=self.groups.len().checked_sub(1);let child=match self.child.as_mut(){Some(child)=>Some(child),None=>index.and_then(|index|self.groups.get_mut(index))};
        if let Some(child)=child{return Ok(match child.close_one(1,maximum_bytes){PluginCloseStep::Pending{released_items,released_bytes}=>SnapshotRetirementStep::Pending{released_items,released_bytes},PluginCloseStep::Complete=>{if self.child.is_some(){self.child.take();}else{self.groups.pop();}SnapshotRetirementStep::Pending{released_items:1,released_bytes:0}},_=>SnapshotRetirementStep::Blocked})}
        if !self.groups.terminal_is_empty(){let step=self.groups.release_empty_page(maximum_bytes).map_err(ValueError::from)?;return Ok(SnapshotRetirementStep::Pending{released_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes})}
        Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self)->bool{self.groups.terminal_is_empty()&&self.child.is_none()&&self.label.is_none()&&self.text.capacity()==0}
    fn next_close_byte_demand(&self)->usize{if self.text.capacity()!=0{return self.text.capacity()}if let Some(label)=self.label.as_ref(){return label.next_owned_close_byte_demand().max(1)}if let Some(child)=self.child.as_ref(){return child.next_close_byte_demand().max(1)}if let Some(index)=self.groups.len().checked_sub(1){return self.groups[index].next_close_byte_demand().max(1)}if !self.groups.terminal_is_empty(){return self.groups.next_release_allocation_bytes().unwrap_or(1)}0}
}

/// 🎞️ Transfers success or refusal scaffolding to the explicit caller recipient and compares against the same source.
pub fn decode_groups_span(source:ByteSpan<'_>,options:&PackDecodeOptions,canonical:&PackEncodeOptions,decoding:&mut NativeDecodeControl<'_>,encoding:&mut NativeEncodeControl<'_>)->Result<PagedList<ChildEmit,MAXIMUM_GROUPS>,os_spr::ProtocolError>{
    let maximum=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX);
    decoding.scoped_maximum(maximum,|decoding|decoding.with_retirement_owner(std::mem::size_of::<ChildGroupDecodeOwner>(),|decoding|{
        let mut owner=Box::new(ChildGroupDecodeOwner::new());
        let result=visit_groups_span(source,options,decoding,owner.as_mut()).map_err(os_spr::ProtocolError::from).and_then(|()|{let mut comparison=os_spr::operation_bytes::OperationByteComparison::new(source);encode_groups_into(&owner.groups,canonical,&mut comparison,encoding)?;comparison.finish()?;Ok(std::mem::replace(&mut owner.groups,PagedList::empty()))});
        (result,Some(owner as Box<dyn ErasedSnapshotRetirement>))
    }))
}
