/// 🪪️ Orders a paid borrowed identity frontier without materializing native Strings.
pub(crate)fn doc_frontier<'a>(ids:impl Iterator<Item=&'a str>,count:usize,out:&mut RowWriter<'_,'_>)->Result<Vec<(&'a str,i64)>,ValueError>{
 let phase=out.phase();let mut names=out.allocate_frontier(count)?;for(ordinal,id)in ids.enumerate(){out.checkpoint()?;names.push((id,number(ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"document identity extent overflow"))?)?));}
 out.sort_frontier(&mut names,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,control))?;
 for pair in names.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate document named identity"))}}Ok(names)
}
/// 🔍️ Resolves an authored reference with bounded cancellable UTF8 comparisons.
pub(crate)fn doc_identity(names:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=names.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(names[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid, std::cmp::Ordering::Equal=>return Ok(names[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling document named reference"))
}
/// 🏷️ Preserves external literal references or resolves the caller's real named partition.
fn doc_reference<'a>(native:Option<&'a str>,names:Option<&[(&str,i64)]>,out:&mut RowWriter<'_,'_>)->Result<Cell<'a>,ValueError>{
 match native{None=>Ok(Cell::Null),Some(name)=>match names{Some(names)=>Ok(Cell::Integer(doc_identity(names,name,out)?)),None=>Ok(Cell::Text(name))}}
}
/// ♻️ Checks style ancestry using paid fixed-width parent marks and cancellable walks.
fn doc_style_cycles(parents:&mut[(Option<usize>,u8)],out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 for root in 0..parents.len(){let mut current=Some(root);while let Some(index)=current{out.checkpoint()?;match parents[index].1{2=>break,1=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"cyclic document style inheritance")),_=>{parents[index].1=1;current=parents[index].0;}}}
  let mut current=Some(root);while let Some(index)=current{out.checkpoint()?;if parents[index].1!=1{break}parents[index].1=2;current=parents[index].0;}
 }Ok(())
}
/// 🫳️ Visits the complete real document corpus using the actual shared RowWriter.
fn visit_rows(snapshot:&SemioDocumentSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let styles=doc_frontier(snapshot.styles.iter().map(|style|style.id.as_str()),snapshot.styles.len(),out)?;let images=doc_frontier(snapshot.images.iter().map(|image|image.id.as_str()),snapshot.images.len(),out)?;let mut parents=out.allocate_frontier(snapshot.styles.len())?;
 for style in &snapshot.styles{out.checkpoint()?;let parent=style.based_on.as_deref().map(|id|doc_identity(&styles,id,out).and_then(|row|usize::try_from(row-1).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string())))).transpose()?;parents.push((parent,0u8));}doc_style_cycles(&mut parents,out)?;
 for(ordinal,style)in snapshot.styles.iter().enumerate(){let parent=parents[ordinal].0.map(|index|number(index+1).map(Cell::Integer)).transpose()?.unwrap_or(Cell::Null);out.insert_key("semio_document_style",number(ordinal+1)?,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&style.id),Cell::Text(&style.name),parent])?;}
 for(ordinal,image)in snapshot.images.iter().enumerate(){out.insert("semio_document_image",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&image.id),Cell::Text(&image.mime),Cell::Blob(&image.bytes)])?;}
 let root=project_block_collection(&snapshot.blocks,DOCUMENT_TABLES,Some(&styles),Some(&images),out)?;out.insert_key("semio_document_document",1,&[Cell::Text(&snapshot.schema),Cell::Integer(root)])?;Ok(())
}
/// 🧱️ Visits the shared block tree with a paid iterative collection frontier.
pub fn project_block_collection(blocks:&[DocBlock],t:DocSqliteTables,styles:Option<&[(&str,i64)]>,images:Option<&[(&str,i64)]>,p:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
let root=p.insert_float(t.collection,&[],float_columns(t.collection))?;let mut pending=p.allocate_frontier(1)?;pending.push((root,blocks));while let Some((collection,blocks))=pending.pop(){p.check_rows(blocks.len().checked_add(pending.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"document traversal count overflow"))?)?;for(ordinal,block)in blocks.iter().enumerate(){let kind=match block{DocBlock::Paragraph{..}=>"paragraph",DocBlock::Heading{..}=>"heading",DocBlock::List{..}=>"list",DocBlock::Table{..}=>"table",DocBlock::Code{..}=>"code",DocBlock::Quote{..}=>"quote",DocBlock::Image{..}=>"image",DocBlock::PageBreak=>"pageBreak"};let id=p.insert_float(t.block,&[Cell::Text(kind)],float_columns(t.block))?;p.insert_float(t.member,&[Cell::Integer(collection),Cell::Integer(number(ordinal)?),Cell::Integer(id)],float_columns(t.member))?;
match block{
DocBlock::Paragraph{style_id,..}=>{let style=doc_reference(style_id.as_deref(),styles,p)?;p.insert_key_float(t.paragraph,id,&[style],float_columns(t.paragraph))?;},
DocBlock::Heading{level,style_id,..}=>{let style=doc_reference(style_id.as_deref(),styles,p)?;p.insert_key_float(t.heading,id,&[Cell::Integer(i64::from(*level)),style],float_columns(t.heading))?;},
DocBlock::List{ordered,items}=>{p.insert_key_float(t.list,id,&[Cell::Integer(i64::from(*ordered))],float_columns(t.list))?;p.check_rows(pending.len().checked_add(items.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"document list traversal overflow"))?)?;for(ordinal,item)in items.iter().enumerate(){let child=p.insert_float(t.collection,&[],float_columns(t.collection))?;p.insert_float(t.list_item,&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Integer(child)],float_columns(t.list_item))?;p.push_frontier(&mut pending,(child,item.blocks.as_slice()))?;}},
DocBlock::Table{rows}=>{p.insert_key_float(t.table,id,&[],float_columns(t.table))?;for(ordinal,row)in rows.iter().enumerate(){let row_id=p.insert_float(t.table_row,&[Cell::Integer(id),Cell::Integer(number(ordinal)?)],float_columns(t.table_row))?;p.check_rows(pending.len().checked_add(row.cells.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"document table traversal overflow"))?)?;for(ordinal,cell)in row.cells.iter().enumerate(){let child=p.insert_float(t.collection,&[],float_columns(t.collection))?;p.insert_float(t.table_cell,&[Cell::Integer(row_id),Cell::Integer(number(ordinal)?),Cell::Integer(child)],float_columns(t.table_cell))?;p.push_frontier(&mut pending,(child,cell.blocks.as_slice()))?;}}},
DocBlock::Code{language,text}=>p.insert_key_float(t.code,id,&[language.as_deref().map(Cell::Text).unwrap_or(Cell::Null),Cell::Text(text)],float_columns(t.code))?,
DocBlock::Quote{blocks}=>{let child=p.insert_float(t.collection,&[],float_columns(t.collection))?;p.insert_key_float(t.quote,id,&[Cell::Integer(child)],float_columns(t.quote))?;p.push_frontier(&mut pending,(child,blocks))?;},
DocBlock::Image{image_id,alt,width,height}=>{let image=doc_reference(Some(image_id),images,p)?;p.insert_key_float(t.image_block,id,&[image,Cell::Text(alt),width.map(Cell::Real).unwrap_or(Cell::Null),height.map(Cell::Real).unwrap_or(Cell::Null)],float_columns(t.image_block))?;}
DocBlock::PageBreak=>p.insert_key_float(t.page_break,id,&[],float_columns(t.page_break))?}
if let DocBlock::Paragraph{runs,..}|DocBlock::Heading{runs,..}=block{for(ordinal,run)in runs.iter().enumerate(){let s=&run.style;p.insert_float(t.run,&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&run.text),Cell::Integer(i64::from(s.bold)),Cell::Integer(i64::from(s.italic)),Cell::Integer(i64::from(s.underline)),s.size.map(Cell::Real).unwrap_or(Cell::Null),s.font.as_deref().map(Cell::Text).unwrap_or(Cell::Null),s.color.as_deref().map(Cell::Text).unwrap_or(Cell::Null),s.link.as_deref().map(Cell::Text).unwrap_or(Cell::Null)],float_columns(t.run))?;}}p.checkpoint()?;}}
Ok(root)}