use semio_framework_value::{ValueError,ValueRefusalKind};
use dsl::{NativeEncodeControl,NativeDecodeControl};

fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn positioned(error:ValueError)->semio_framework_diagnostic::TextError{semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}
fn add_rows(rows:&mut usize,count:usize,maximum:usize)->Result<(),ValueError>{*rows=rows.checked_add(count).filter(|value|*value<=maximum).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"CommonMark domain rows exceed caller ceiling"))?;Ok(())}
fn enqueue<'a,T>(values:&'a[T],pending:&mut std::collections::VecDeque<&'a T>,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{let bytes=values.len().checked_mul(std::mem::size_of::<&T>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"CommonMark borrowed frontier overflow"))?;control.charge(bytes)?;pending.try_reserve(values.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"CommonMark borrowed frontier allocation failed"))?;for value in values{pending.push_back(value);control.step()?;}Ok(())}
fn paid_indices<'a,T>(values:&'a[T],pending:&mut std::collections::VecDeque<&'a T>,next:&mut u64,control:&mut NativeEncodeControl<'_>)->Result<Vec<u64>,ValueError>{control.scoped_stage(|control|{control.begin_stage(values.len())?;let mut output=control.allocate_vec(values.len())?;let bytes=values.len().checked_mul(std::mem::size_of::<&T>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"CommonMark borrowed frontier overflow"))?;control.charge(bytes)?;pending.try_reserve(values.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"CommonMark borrowed frontier allocation failed"))?;for value in values{let id=*next;*next=next.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark logical index overflow"))?;output.push(id);pending.push_back(value);control.step()?;}Ok(output)})}
fn paid_option(value:&Option<String>,control:&mut NativeEncodeControl<'_>)->Result<Option<String>,ValueError>{value.as_deref().map(|text|control.copy_text(text)).transpose()}

fn census(value:&MdSnapshot,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<(usize,usize,usize),ValueError>{
 let mut rows=0;add_rows(&mut rows,1,maximum)?;let mut pending=std::collections::VecDeque::new();let mut inline_pending=std::collections::VecDeque::new();control.begin_stage(0)?;enqueue(&value.blocks,&mut pending,control)?;let(mut blocks,mut inlines)=(0usize,0usize);
 while let Some(block)=pending.pop_front(){add_rows(&mut rows,3,maximum)?;blocks=blocks.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark block count overflow"))?;match block{MdBlock::Heading{inlines,..}|MdBlock::Paragraph{inlines}=>enqueue(inlines,&mut inline_pending,control)?,MdBlock::List{items,..}=>{add_rows(&mut rows,items.len(),maximum)?;control.scoped_stage(|control|{control.begin_stage(items.len())?;for item in items{control.scoped_stage(|control|{control.begin_stage(item.len())?;enqueue(item,&mut pending,control)})?;control.step()?;}Ok::<_,ValueError>(())})?;},MdBlock::BlockQuote{blocks}=>enqueue(blocks,&mut pending,control)?,MdBlock::CodeBlock{..}|MdBlock::ThematicBreak|MdBlock::HtmlBlock{..}=>{}}control.step()?;}
 while let Some(inline)=inline_pending.pop_front(){add_rows(&mut rows,3,maximum)?;inlines=inlines.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark inline count overflow"))?;match inline{MdInline::Emphasis{inlines}|MdInline::Strong{inlines}=>enqueue(inlines,&mut inline_pending,control)?,MdInline::Link{text,..}=>enqueue(text,&mut inline_pending,control)?,MdInline::Text{..}|MdInline::Code{..}|MdInline::Image{..}|MdInline::SoftBreak|MdInline::HardBreak|MdInline::HtmlInline{..}=>{}}control.step()?;}
 Ok((blocks,inlines,rows))
}

fn project_controlled(value:&MdSnapshot,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<Snapshot,ValueError>{
 let(block_count,inline_count,_)=control.scoped_stage(|control|census(value,maximum,control))?;let mut pending=std::collections::VecDeque::new();let mut inline_pending=std::collections::VecDeque::new();let(mut next,mut next_inline)=(0,0);let roots=paid_indices(&value.blocks,&mut pending,&mut next,control)?;let mut blocks=control.allocate_vec(block_count)?;let mut inlines=control.allocate_vec(inline_count)?;control.begin_stage(block_count.checked_add(inline_count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark logical work overflow"))?)?;
 while let Some(value)=pending.pop_front(){blocks.push(control.scoped_stage(|control|block::project_controlled(value,&mut pending,&mut next,&mut inline_pending,&mut next_inline,control))?);control.step()?;}
 while let Some(value)=inline_pending.pop_front(){inlines.push(control.scoped_stage(|control|inline::project_controlled(value,&mut inline_pending,&mut next_inline,control))?);control.step()?;}
 Ok(Snapshot{schema:control.copy_text(&value.schema)?,roots,blocks,inlines})
}

fn record_list(record:&dsl::RecordValue,id:u16)->Result<&[dsl::FieldValue],ValueError>{match record.get(id){Some(dsl::FieldValue::List(values))=>Ok(values),_=>Err(invalid("CommonMark logical list is missing"))}}
fn borrowed_census(record:&dsl::RecordValue,maximum:usize,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 let blocks=record_list(record,2)?;let inlines=record_list(record,3)?;let mut rows=0;add_rows(&mut rows,1,maximum)?;control.begin_stage(blocks.len().checked_add(inlines.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark row scan overflow"))?)?;
 for value in blocks{add_rows(&mut rows,3,maximum)?;let dsl::FieldValue::Record(block)=value else{return Err(invalid("CommonMark block requires its owned record"))};add_rows(&mut rows,record_list(block,6)?.len(),maximum)?;control.step()?;}for _ in inlines{add_rows(&mut rows,3,maximum)?;control.step()?;}Ok(())
}

fn retirement_visit(){#[cfg(test)]retirement_test_visit();}
fn drain_inlines(mut work:Vec<MdInline>){let mut next=work.pop();while let Some(value)=next{retirement_visit();let children=match value{MdInline::Emphasis{inlines}|MdInline::Strong{inlines}=>Some(inlines),MdInline::Link{text,..}=>Some(text),MdInline::Text{..}|MdInline::Code{..}|MdInline::Image{..}|MdInline::SoftBreak|MdInline::HardBreak|MdInline::HtmlInline{..}=>None};if let Some(mut children)=children{if let Some(child)=children.pop(){next=Some(child);if !work.is_empty(){children.push(MdInline::Strong{inlines:work});let last=children.len()-1;children.swap(0,last);}work=children;continue;}}next=work.pop();}}
fn drain_blocks(mut work:Vec<MdBlock>){let mut next=work.pop();while let Some(value)=next{retirement_visit();match value{
 MdBlock::Heading{inlines,..}|MdBlock::Paragraph{inlines}=>drain_inlines(inlines),
 MdBlock::BlockQuote{mut blocks}=>{if let Some(child)=blocks.pop(){next=Some(child);if !work.is_empty(){blocks.push(MdBlock::BlockQuote{blocks:work});let last=blocks.len()-1;blocks.swap(0,last);}work=blocks;continue;}},
 MdBlock::List{ordered,start,tight,mut items}=>{let mut selected=None;while let Some(mut children)=items.pop(){retirement_visit();if let Some(child)=children.pop(){selected=Some((children,child));break;}}if let Some((mut children,child))=selected{if !work.is_empty(){items.push(work);}if !items.is_empty(){children.push(MdBlock::List{ordered,start,tight,items});let last=children.len()-1;children.swap(0,last);}work=children;next=Some(child);continue;}},
 MdBlock::CodeBlock{..}|MdBlock::ThematicBreak|MdBlock::HtmlBlock{..}=>{}}
 next=work.pop();}}
fn retire_parts(blocks:Vec<MdBlock>,inlines:Vec<MdInline>){drain_blocks(blocks);drain_inlines(inlines)}
trait RetireNode{fn retire_nodes(values:Vec<Self>)where Self:Sized;fn retire_node(self);}
impl RetireNode for MdBlock{fn retire_nodes(values:Vec<Self>){retire_parts(values,Vec::new())}fn retire_node(self){match self{MdBlock::Heading{inlines,..}|MdBlock::Paragraph{inlines}=>retire_parts(Vec::new(),inlines),MdBlock::List{items,..}=>{for blocks in items{retire_parts(blocks,Vec::new())}},MdBlock::BlockQuote{blocks}=>retire_parts(blocks,Vec::new()),MdBlock::CodeBlock{..}|MdBlock::ThematicBreak|MdBlock::HtmlBlock{..}=>{}}}}
impl RetireNode for MdInline{fn retire_nodes(values:Vec<Self>){retire_parts(Vec::new(),values)}fn retire_node(self){match self{MdInline::Emphasis{inlines}|MdInline::Strong{inlines}=>retire_parts(Vec::new(),inlines),MdInline::Link{text,..}=>retire_parts(Vec::new(),text),MdInline::Text{..}|MdInline::Code{..}|MdInline::Image{..}|MdInline::SoftBreak|MdInline::HardBreak|MdInline::HtmlInline{..}=>{}}}}
struct OwnedNodes<T:RetireNode>(Vec<T>);
impl<T:RetireNode> Drop for OwnedNodes<T>{fn drop(&mut self){T::retire_nodes(std::mem::take(&mut self.0))}}
struct OwnedSlots<T:RetireNode>(Vec<Option<T>>);
impl<T:RetireNode> Drop for OwnedSlots<T>{fn drop(&mut self){for value in std::mem::take(&mut self.0).into_iter().flatten(){value.retire_node()}}}
struct OwnedItems(Vec<Vec<MdBlock>>);
impl Drop for OwnedItems{fn drop(&mut self){for blocks in std::mem::take(&mut self.0){retire_parts(blocks,Vec::new())}}}
fn paid_children<T:RetireNode>(values:&mut[Option<T>],parent:Option<usize>,keys:Vec<u64>,control:&mut NativeDecodeControl<'_>)->Result<Vec<T>,ValueError>{control.scoped_stage(|control|{control.begin_stage(keys.len())?;let mut output=OwnedNodes(control.allocate_vec(keys.len())?);for key in keys{let key=usize::try_from(key).map_err(|_|invalid("CommonMark child index exceeds native domain"))?;if parent.is_some_and(|parent|key<=parent){return Err(invalid("CommonMark forward child topology differs"))}let value=values.get_mut(key).and_then(Option::take).ok_or_else(||invalid("CommonMark child is unknown or has multiple owners"))?;output.0.push(value);control.step()?;}Ok(std::mem::take(&mut output.0))})}
fn reconstruct_controlled(value:Snapshot,control:&mut NativeDecodeControl<'_>)->Result<MdSnapshot,ValueError>{
 #[cfg(test)]let _stage=super::sqlite_tests::MdReconstructTestScope::enter();
 let Snapshot{schema,roots,blocks,inlines}=value;let mut inlines=inline::reconstruct_controlled(inlines,control)?;let mut blocks=block::reconstruct_controlled(blocks,&mut inlines.0,control)?;let mut roots=OwnedNodes(paid_children(&mut blocks.0,None,roots,control)?);if blocks.0.iter().any(Option::is_some)||inlines.0.iter().any(Option::is_some){return Err(invalid("CommonMark logical record contains unowned entities"))}Ok(MdSnapshot{schema,blocks:std::mem::take(&mut roots.0)})
}

pub(super) fn decode_owned(payload:&store::io_schema::IoPayload,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<MdSnapshot,String>{
 let limits=control.limits();if <MdSnapshot as store::ArtifactSqliteSnapshot>::SQLITE_SCHEMA.len()>limits.max_schema_bytes{return Err("CommonMark authored schema exceeds caller limit".into())}store::decode_sqlite_snapshot_record_native(payload,"stdio.md",Snapshot::__dsl_spec_producer(),|record,native|{native.scoped_stage(|native|borrowed_census(record,limits.max_rows,native)).map_err(positioned)?;let flat=Snapshot::__dsl_from_record_controlled(record,native)?;reconstruct_controlled(flat,native).map_err(positioned)},control)
}
pub(super) fn encode_owned(value:&MdSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{
 let limits=control.limits();if <MdSnapshot as store::ArtifactSqliteSnapshot>::SQLITE_SCHEMA.len()>limits.max_schema_bytes{return Err("CommonMark authored schema exceeds caller limit".into())}store::encode_sqlite_snapshot_record_native(encoding,"stdio.md",Snapshot::__dsl_spec_producer(),|native|{let flat=project_controlled(value,limits.max_rows,native).map_err(positioned)?;flat.__dsl_to_record_controlled(native)},control)
}

pub(super) fn retire_owned(value:MdSnapshot){retire_parts(value.blocks,Vec::new())}
