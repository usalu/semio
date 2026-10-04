pub(super) fn project_controlled<'a>(value:&'a MdBlock,pending:&mut std::collections::VecDeque<&'a MdBlock>,next:&mut u64,inline_pending:&mut std::collections::VecDeque<&'a MdInline>,next_inline:&mut u64,control:&mut NativeEncodeControl<'_>)->Result<Block,ValueError>{
 let mut row=Block{kind:Kind::ThematicBreak,level:None,inlines:Vec::new(),ordered:None,start:None,tight:None,items:Vec::new(),blocks:Vec::new(),info:None,literal:None,raw:None};match value{
 MdBlock::Heading{level,inlines}=>{row.kind=Kind::Heading;row.level=Some(*level);row.inlines=paid_indices(inlines,inline_pending,next_inline,control)?;},
 MdBlock::Paragraph{inlines}=>{row.kind=Kind::Paragraph;row.inlines=paid_indices(inlines,inline_pending,next_inline,control)?;},
 MdBlock::List{ordered,start,tight,items}=>{row.kind=Kind::List;row.ordered=Some(*ordered);row.start=*start;row.tight=Some(*tight);row.items=control.allocate_vec(items.len())?;control.scoped_stage(|control|{control.begin_stage(items.len())?;for blocks in items{row.items.push(ListItem{blocks:paid_indices(blocks,pending,next,control)?});control.step()?;}Ok::<_,ValueError>(())})?;},
 MdBlock::CodeBlock{info,literal}=>{row.kind=Kind::CodeBlock;row.info=paid_option(info,control)?;row.literal=Some(control.copy_text(literal)?);},
 MdBlock::BlockQuote{blocks}=>{row.kind=Kind::BlockQuote;row.blocks=paid_indices(blocks,pending,next,control)?;},
 MdBlock::ThematicBreak=>{},MdBlock::HtmlBlock{raw}=>{row.kind=Kind::HtmlBlock;row.raw=Some(control.copy_text(raw)?);}}
 Ok(row)
}
pub(super) fn reconstruct_controlled(rows:Vec<Block>,inlines:&mut[Option<MdInline>],control:&mut NativeDecodeControl<'_>)->Result<OwnedSlots<MdBlock>,ValueError>{
 let mut values=OwnedSlots(control.allocate_vec(rows.len())?);values.0.resize_with(rows.len(),||None);control.begin_stage(rows.len())?;
 for(index,row)in rows.into_iter().enumerate().rev(){let Block{kind,mut level,inlines:inline_ids,mut ordered,mut start,mut tight,items,blocks:block_ids,mut info,mut literal,mut raw}=row;let(mut inline_ids,mut items,mut block_ids)=(inline_ids,items,block_ids);
 let fields_allowed=match kind{
 Kind::Heading=>ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&block_ids.is_empty()&&info.is_none()&&literal.is_none()&&raw.is_none(),
 Kind::Paragraph=>level.is_none()&&ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&block_ids.is_empty()&&info.is_none()&&literal.is_none()&&raw.is_none(),
 Kind::List=>level.is_none()&&inline_ids.is_empty()&&block_ids.is_empty()&&info.is_none()&&literal.is_none()&&raw.is_none(),
 Kind::CodeBlock=>level.is_none()&&inline_ids.is_empty()&&ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&block_ids.is_empty()&&raw.is_none(),
 Kind::BlockQuote=>level.is_none()&&inline_ids.is_empty()&&ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&info.is_none()&&literal.is_none()&&raw.is_none(),
 Kind::ThematicBreak=>level.is_none()&&inline_ids.is_empty()&&ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&block_ids.is_empty()&&info.is_none()&&literal.is_none()&&raw.is_none(),
 Kind::HtmlBlock=>level.is_none()&&inline_ids.is_empty()&&ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&block_ids.is_empty()&&info.is_none()&&literal.is_none()};if !fields_allowed{return Err(invalid("CommonMark block contains unrelated variant fields"))}
 let block=match kind{
 Kind::Heading=>{let level=level.take().ok_or_else(||invalid("CommonMark heading level is missing"))?;MdBlock::Heading{level,inlines:paid_children(inlines,None,std::mem::take(&mut inline_ids),control)?}},
 Kind::Paragraph=>MdBlock::Paragraph{inlines:paid_children(inlines,None,std::mem::take(&mut inline_ids),control)?},
 Kind::List=>{let ordered=ordered.take().ok_or_else(||invalid("CommonMark list ordered flag is missing"))?;let tight=tight.take().ok_or_else(||invalid("CommonMark list tight flag is missing"))?;let mut owned=OwnedItems(control.allocate_vec(items.len())?);control.scoped_stage(|control|{control.begin_stage(items.len())?;for item in std::mem::take(&mut items){owned.0.push(paid_children(&mut values.0,Some(index),item.blocks,control)?);control.step()?;}Ok::<_,ValueError>(())})?;MdBlock::List{ordered,start:start.take(),tight,items:std::mem::take(&mut owned.0)}},
 Kind::CodeBlock=>MdBlock::CodeBlock{info:info.take(),literal:literal.take().ok_or_else(||invalid("CommonMark code literal is missing"))?},
 Kind::BlockQuote=>MdBlock::BlockQuote{blocks:paid_children(&mut values.0,Some(index),std::mem::take(&mut block_ids),control)?},
 Kind::ThematicBreak=>MdBlock::ThematicBreak,Kind::HtmlBlock=>MdBlock::HtmlBlock{raw:raw.take().ok_or_else(||invalid("CommonMark HTML block raw text is missing"))?}};
 values.0[index]=Some(block);control.step()?;
 }Ok(values)
}
