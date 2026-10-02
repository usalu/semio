//! 🪪️ Authored Forms identities use admitted borrowed slots and bounded literal comparisons.
use dsl::NativeSchemaControl;
use std::cmp::Ordering;
fn compare<C:NativeSchemaControl>(a:(i64,&str),b:(i64,&str),c:&mut C)->Result<Ordering,String>{
 if a.0!=b.0{return Ok(a.0.cmp(&b.0))}let a=a.1.as_bytes();let b=b.1.as_bytes();c.scoped_stage(|c|{let total=a.len().min(b.len());c.begin_stage(total)?;let mut position=0;while position<total{let end=position.saturating_add(65536).min(total);let order=a[position..end].cmp(&b[position..end]);c.advance(end-position)?;if order!=Ordering::Equal{return Ok(order)}position=end;}Ok(a.len().cmp(&b.len()))})
}
pub fn unique<'s,C:NativeSchemaControl>(count:usize,values:impl Iterator<Item=Result<(i64,&'s str),String>>,c:&mut C)->Result<(),String>{
 fn sift<C:NativeSchemaControl>(v:&mut[(i64,&str)],mut root:usize,end:usize,c:&mut C)->Result<(),String>{loop{let Some(child)=root.checked_mul(2).and_then(|n|n.checked_add(1)).filter(|n|*n<end)else{break};c.step()?;let next=if child+1<end&&compare(v[child],v[child+1],c)?==Ordering::Less{child+1}else{child};if compare(v[root],v[next],c)?!=Ordering::Less{break}v.swap(root,next);root=next;}Ok(())}
 c.scoped_stage(|c|{c.begin_stage(count)?;let mut rows=c.allocate_vec(count)?;for value in values{let value=value?;if value.1.is_empty(){return Err("Forms owned identity is empty".into())}rows.push(value);c.step()?;}if rows.len()!=count{return Err("Forms identity frontier count differs".into())}c.begin_stage(0)?;for root in(0..rows.len()/2).rev(){sift(&mut rows,root,count,c)?;}for end in(1..count).rev(){c.step()?;rows.swap(0,end);sift(&mut rows,0,end,c)?;}c.begin_stage(count.saturating_sub(1))?;for i in 1..count{c.step()?;if compare(rows[i-1],rows[i],c)?==Ordering::Equal{return Err("Forms owned identity is duplicate".into())}}Ok(())})
}
pub fn snapshot<C:NativeSchemaControl>(s:&crate::FormsSnapshot,c:&mut C)->Result<(),String>{
 unique(s.definition.steps.len(),s.definition.steps.iter().map(|s|Ok((0,s.id.as_str()))),c)?;
 let count=s.definition.steps.iter().try_fold(0usize,|n,s|n.checked_add(s.blocks.len()).ok_or("Forms identity count overflow"))?;
 unique(count,s.definition.steps.iter().flat_map(|s|s.blocks.iter().map(|q|Ok((0,q.id.as_str())))),c)?;
 for step in &s.definition.steps{for q in &step.blocks{if let Some(v)=&q.options{unique(v.len(),v.iter().map(|v|Ok((0,v.value.as_str()))),c)?;}if let Some(v)=&q.fields{unique(v.len(),v.iter().map(|v|Ok((0,v.key.as_str()))),c)?;}}}
 unique(s.responses.len(),s.responses.iter().map(|r|Ok((0,r.id.as_str()))),c)?;for response in&s.responses{unique(response.answers.len(),response.answers.iter().map(|a|Ok((0,a.question_id.as_str()))),c)?;}Ok(())
}

