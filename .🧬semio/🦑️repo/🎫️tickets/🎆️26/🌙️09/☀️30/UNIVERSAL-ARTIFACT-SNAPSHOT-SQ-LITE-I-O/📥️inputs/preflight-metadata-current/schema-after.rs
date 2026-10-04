//! 🕸️ Canonical schema quotient borrows immutable field metadata and bounded indexed work slots.
use super::*;
use std::cmp::Ordering;
const N:usize=64;
struct Graph{records:[R;N],addresses:[usize;N],slots:[Option<(usize,usize)>;128],length:usize}
fn order(spec:R)->Result<([usize;256],usize),ValueError>{if spec.fields.len()>256{return Err(error(K::WorkLimit,"borrowed schema exceeds inline field-order frontier"))}let mut indices=[0;256];for(index,slot)in indices[..spec.fields.len()].iter_mut().enumerate(){*slot=index;}indices[..spec.fields.len()].sort_unstable_by_key(|index|(spec.fields[*index].id,*index));Ok((indices,spec.fields.len()))}
fn edges(shape:H,depth:usize,visit:&mut impl FnMut(fn()->R,&mut NativeEncodeControl<'_>)->Result<(),ValueError>,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if depth>64{return Err(error(K::DepthLimit,"borrowed schema shape exceeds depth"))}control.step()?;match shape{H::Record(make)|H::Table(make)=>visit(make,control)?,H::Tuple(make,_)|H::List(make)|H::Block(make)|H::Map(make)=>edges(make(),depth+1,visit,control)?,H::Statements(variants)=>{let(indices,length)=variant_order(variants,control)?;for index in indices[..length].iter().copied(){visit(variants[index].1,control)?;}},_=>{}}Ok(())}
fn variant_order(variants:&[(&str,fn()->R)],control:&mut NativeEncodeControl<'_>)->Result<([usize;256],usize),ValueError>{if variants.len()>256{return Err(error(K::WorkLimit,"borrowed schema exceeds inline variant-order frontier"))}let mut indices=[0;256];for(index,slot)in indices[..variants.len()].iter_mut().enumerate(){*slot=index;}controlled_schema::sort(&mut indices[..variants.len()],|a,b,control|{let order=controlled_schema::compare_text(variants[*a].0,variants[*b].0,control)?;Ok(if order==Ordering::Equal{a.cmp(b)}else{order})},control)?;Ok((indices,variants.len()))}
impl Graph{
 fn new(root:R,control:&mut NativeEncodeControl<'_>)->Result<Self,ValueError>{
  let mut graph=Self{records:[root;N],addresses:[usize::MAX;N],slots:[None;128],length:1};let mut cursor=0;
  while cursor<graph.length{control.step()?;let(indices,length)=order(graph.records[cursor])?;for index in indices[..length].iter().copied(){edges(graph.records[cursor].fields[index].shape,0,&mut|make,control|{control.step()?;let address=make as usize;let mut slot=(((address as u64).wrapping_mul(0x9e3779b97f4a7c15)>>7) as usize)&127;loop{control.step()?;match graph.slots[slot]{Some((key,_))if key==address=>return Ok(()),Some(_)=>slot=(slot+1)&127,None=>{if graph.length==N{return Err(error(K::WorkLimit,"borrowed schema exceeds inline record frontier"))}graph.slots[slot]=Some((address,graph.length));graph.addresses[graph.length]=address;graph.records[graph.length]=make();graph.length+=1;return Ok(())}}}},control)?;}cursor+=1;}Ok(graph)
 }
 fn target(&self,make:fn()->R)->Result<usize,ValueError>{let address=make as usize;let mut slot=(((address as u64).wrapping_mul(0x9e3779b97f4a7c15)>>7) as usize)&127;loop{match self.slots[slot]{Some((key,index))if key==address=>return Ok(index),Some(_)=>slot=(slot+1)&127,None=>return Err(error(K::InvariantViolated,"borrowed schema edge was not discovered"))}}}
 fn hash(&self,control:&mut NativeEncodeControl<'_>)->Result<[u8;32],ValueError>{
  let mut classes=[0usize;N];let mut count=0;
  let mut digests=[[0u8;32];N];for node in 0..self.length{let mut hash=HashSink::new();let mut edge=0;self.write_fields(node,&mut hash,&mut|_|{let index=edge;edge+=1;Ok(index)},control)?;digests[node]=hash.finish();}
  classify(self,&digests,None,&mut classes,&mut count,control)?;
  loop{let old=classes;let previous=count;for node in 0..self.length{let mut hash=HashSink::new();hash.varint(old[node]as u64,control)?;let mut edge_count=0;for field in self.records[node].fields{edges(field.shape,0,&mut |_,_|{edge_count+=1;Ok(())},control)?;}hash.varint(edge_count,control)?;let(indices,length)=order(self.records[node])?;for index in indices[..length].iter().copied(){edges(self.records[node].fields[index].shape,0,&mut|make,control|hash.varint(old[self.target(make)?]as u64,control),control)?;}digests[node]=hash.finish();}count=0;classify(self,&digests,Some(&old),&mut classes,&mut count,control)?;if count==previous{break}}
  let mut ordinals=[usize::MAX;N];let mut queue=[0usize;N];let(mut cursor,mut length)=(0,1);ordinals[classes[0]]=0;
  while cursor<length{control.step()?;let node=queue[cursor];cursor+=1;let(indices,count)=order(self.records[node])?;for index in indices[..count].iter().copied(){edges(self.records[node].fields[index].shape,0,&mut|make,_|{let target=self.target(make)?;let class=classes[target];if ordinals[class]==usize::MAX{ordinals[class]=length;queue[length]=target;length+=1;}Ok(())},control)?;}}
  let mut hash=HashSink::new();hash.varint(length as u64,control)?;for node in queue[..length].iter().copied(){self.write_fields(node,&mut hash,&mut|make|Ok(ordinals[classes[self.target(make)?]]),control)?;}Ok(hash.finish())
 }
 fn write_fields(&self,node:usize,sink:&mut impl Sink,map:&mut impl FnMut(fn()->R)->Result<usize,ValueError>,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{let spec=self.records[node];let(indices,length)=order(spec)?;sink.varint(length as u64,control)?;for index in indices[..length].iter().copied(){let field=spec.fields[index];sink.varint(field.id as u64,control)?;write_text(field.key,sink,control)?;sink.byte(u8::from(field.optional)|(u8::from(field.flatten)<<1),control)?;write_shape(field.shape,sink,map,0,control)?;}Ok(())}
}
fn classify(graph:&Graph,digests:&[[u8;32];N],old:Option<&[usize;N]>,classes:&mut[usize;N],count:&mut usize,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
 let mut slots=[None;128];let mut next=[None;N];for node in 0..graph.length{control.step()?;let digest=digests[node];let mut slot=u64::from_le_bytes(digest[..8].try_into().unwrap())as usize&127;loop{control.step()?;match slots[slot]{Some((key,first))if key==digest=>{let mut candidate=Some(first);let mut found=None;while let Some(index)=candidate{control.step()?;if equal_record(graph,node,index,old,control)?{found=Some(classes[index]);break}candidate=next[index];}if let Some(class)=found{classes[node]=class;}else{classes[node]=*count;*count+=1;next[node]=Some(first);slots[slot]=Some((digest,node));}break},Some(_)=>slot=(slot+1)&127,None=>{classes[node]=*count;*count+=1;slots[slot]=Some((digest,node));break}}}}Ok(())
}
fn equal_record(graph:&Graph,left:usize,right:usize,old:Option<&[usize;N]>,control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{
 if old.is_some_and(|classes|classes[left]!=classes[right]){return Ok(false)}let a=graph.records[left];let b=graph.records[right];if a.fields.len()!=b.fields.len(){return Ok(false)}let(ai,length)=order(a)?;let(bi,_)=order(b)?;for index in 0..length{control.step()?;let a=a.fields[ai[index]];let b=b.fields[bi[index]];if a.id!=b.id||controlled_schema::compare_text(a.key,b.key,control)?!=Ordering::Equal||a.optional!=b.optional||a.flatten!=b.flatten||!equal_shape(graph,a.shape,b.shape,old,0,control)?{return Ok(false)}}Ok(true)
}
fn equal_shape(graph:&Graph,a:H,b:H,classes:Option<&[usize;N]>,depth:usize,control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{
 if depth>64{return Err(error(K::DepthLimit,"borrowed schema comparison exceeds depth"))}control.step()?;Ok(match(a,b){
 (H::Bool,H::Bool)|(H::Int,H::Int)|(H::UInt,H::UInt)|(H::Float,H::Float)|(H::Text,H::Text)|(H::Bytes64,H::Bytes64)|(H::Value,H::Value)|(H::Wire,H::Wire)|(H::Dir,H::Dir)|(H::Range,H::Range)|(H::Count,H::Count)|(H::Expr,H::Expr)=>true,
 (H::Enum(a),H::Enum(b))=>{let(ai,n)=enum_order(a,control)?;let(bi,m)=enum_order(b,control)?;let mut same=n==m;if same{for(i,j)in ai[..n].iter().zip(&bi[..m]){control.step()?;if a[*i].1!=b[*j].1||controlled_schema::compare_text(a[*i].0,b[*j].0,control)?!=Ordering::Equal{same=false;break}}}same},
 (H::Tuple(a,n),H::Tuple(b,m))=>n==m&&equal_shape(graph,a(),b(),classes,depth+1,control)?,
 (H::List(a),H::List(b))|(H::Block(a),H::Block(b))|(H::Map(a),H::Map(b))=>equal_shape(graph,a(),b(),classes,depth+1,control)?,
 (H::Record(a),H::Record(b))|(H::Table(a),H::Table(b))=>if let Some(classes)=classes{classes[graph.target(a)?]==classes[graph.target(b)?]}else{true},
 (H::Statements(a),H::Statements(b))=>{let(ai,n)=variant_order(a,control)?;let(bi,m)=variant_order(b,control)?;let mut same=n==m;if same{for(i,j)in ai[..n].iter().zip(&bi[..m]){control.step()?;if controlled_schema::compare_text(a[*i].0,b[*j].0,control)?!=Ordering::Equal||if let Some(classes)=classes{classes[graph.target(a[*i].1)?]!=classes[graph.target(b[*j].1)?]}else{false}{same=false;break}}}same},
 (H::Quantity(a),H::Quantity(b))|(H::Angle(a),H::Angle(b))=>controlled_schema::compare_text(a.symbol,b.symbol,control)?==Ordering::Equal,
 (H::Ref(a),H::Ref(b))|(H::Embed(a),H::Embed(b))|(H::EmbedFrom(a),H::EmbedFrom(b))=>controlled_schema::compare_text(a,b,control)?==Ordering::Equal,(H::Coord(a),H::Coord(b))|(H::Dim(a),H::Dim(b))=>a==b,_=>false})
}
fn enum_order(variants:&[(&str,u32)],control:&mut NativeEncodeControl<'_>)->Result<([usize;256],usize),ValueError>{if variants.len()>256{return Err(error(K::WorkLimit,"borrowed enum exceeds inline order frontier"))}let mut indices=[0;256];for(index,slot)in indices[..variants.len()].iter_mut().enumerate(){*slot=index;}controlled_schema::sort(&mut indices[..variants.len()],|a,b,control|{let order=variants[*a].1.cmp(&variants[*b].1);if order==Ordering::Equal{controlled_schema::compare_text(variants[*a].0,variants[*b].0,control)}else{Ok(order)}},control)?;Ok((indices,variants.len()))}
fn write_text(text:&str,sink:&mut impl Sink,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{sink.varint(text.len()as u64,control)?;sink.bytes(text.as_bytes(),control)}
fn write_shape(shape:H,sink:&mut impl Sink,map:&mut impl FnMut(fn()->R)->Result<usize,ValueError>,depth:usize,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
 if depth>64{return Err(error(K::DepthLimit,"borrowed schema hashing exceeds depth"))}control.step()?;match shape{
 H::Bool=>sink.byte(1,control),H::Int=>sink.byte(2,control),H::UInt=>sink.byte(3,control),H::Float=>sink.byte(4,control),H::Text=>sink.byte(5,control),H::Bytes64=>sink.byte(6,control),
 H::Enum(labels)=>{sink.byte(7,control)?;let(indices,length)=enum_order(labels,control)?;sink.varint(length as u64,control)?;for index in indices[..length].iter().copied(){sink.varint(labels[index].1 as u64,control)?;write_text(labels[index].0,sink,control)?;}Ok(())},
 H::Tuple(make,count)=>{sink.byte(8,control)?;write_shape(make(),sink,map,depth+1,control)?;if let Some(count)=count{sink.byte(1,control)?;sink.varint(count as u64,control)}else{sink.byte(0,control)}},
 H::List(make)|H::Block(make)|H::Map(make)=>{sink.byte(match shape{H::List(_)=>9,H::Block(_)=>11,_=>13},control)?;write_shape(make(),sink,map,depth+1,control)},
 H::Record(make)|H::Table(make)=>{sink.byte(if matches!(shape,H::Record(_)){10}else{15},control)?;sink.varint(map(make)?as u64,control)},
 H::Statements(variants)=>{sink.byte(12,control)?;let(indices,length)=variant_order(variants,control)?;sink.varint(length as u64,control)?;for index in indices[..length].iter().copied(){write_text(variants[index].0,sink,control)?;sink.varint(map(variants[index].1)?as u64,control)?;}Ok(())},
 H::Value=>sink.byte(14,control),H::Wire=>sink.byte(16,control),H::Quantity(unit)|H::Angle(unit)=>{sink.byte(if matches!(shape,H::Quantity(_)){17}else{18},control)?;write_text(unit.symbol,sink,control)},
 H::Ref(text)|H::Embed(text)|H::EmbedFrom(text)=>{sink.byte(match shape{H::Ref(_)=>19,H::Embed(_)=>26,_=>27},control)?;write_text(text,sink,control)},H::Coord(n)=>sink.bytes(&[20,n],control),H::Dir=>sink.byte(21,control),H::Dim(n)=>sink.bytes(&[22,n],control),H::Range=>sink.byte(23,control),H::Count=>sink.byte(24,control),H::Expr=>sink.byte(25,control)
 }
}
struct HashSink{hasher:semio_framework_hash::Hasher}
impl HashSink{fn new()->Self{Self{hasher:semio_framework_hash::Hasher::new()}}fn finish(self)->[u8;32]{*self.hasher.finalize().as_bytes()}}
impl Sink for HashSink{fn bytes(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{control.scoped_stage(|control|{control.begin_stage(bytes.len())?;for part in bytes.chunks(65536){self.hasher.update(part);control.advance(part.len())?;}Ok(())})}}
pub(super) fn hash(root:R,control:&mut NativeEncodeControl<'_>)->Result<[u8;32],ValueError>{Graph::new(root,control)?.hash(control)}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
