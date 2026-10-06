//! 🫳️ Exact canonical Pack demand from immutable typed fields and static schema views.
use super::*;
use semio_framework_dsl_record::{BorrowedRecordSpec as R,BorrowedShape as H,native_encoding::{FieldProjectionSource,FieldProjectionView as V}};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind as K};
#[path="🏭️schema/🦀️.rs"]
mod schema;
fn error(kind:K,message:&str)->ValueError{ValueError::new(kind,message)}
trait Sink{
 fn bytes(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>;
 fn byte(&mut self,byte:u8,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.bytes(&[byte],control)}
 fn varint(&mut self,mut value:u64,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{let mut bytes=[0;10];let mut length=0;loop{bytes[length]=(value&127)as u8;value>>=7;if value!=0{bytes[length]|=128;}length+=1;if value==0{break;}}self.bytes(&bytes[..length],control)}
 fn signed(&mut self,value:i64,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.varint(((value as u64)<<1)^((value>>63)as u64),control)}
}
#[derive(Clone,Copy)]
struct Symbol<'a>{text:&'a str,repeated:bool}
struct Symbols<'a>{inline:[Symbol<'a>;64],heap:Vec<Symbol<'a>>,length:usize}
impl<'a> Symbols<'a>{
 fn new()->Self{Self{inline:[Symbol{text:"",repeated:false};64],heap:Vec::new(),length:0}}
 fn entries(&self)->&[Symbol<'a>]{if self.heap.capacity()==0{&self.inline[..self.length]}else{&self.heap}}
 fn entries_mut(&mut self)->&mut[Symbol<'a>]{if self.heap.capacity()==0{&mut self.inline[..self.length]}else{&mut self.heap}}
 fn note(&mut self,text:&'a str,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  control.scoped_stage(|control|{control.begin_stage(text.len())?;for part in text.as_bytes().chunks(256){control.advance(part.len())?;}Ok::<_,ValueError>(())})?;
  let(mut low,mut high)=(0,self.length);while low<high{control.step()?;let middle=low+(high-low)/2;match controlled_schema::compare_text(self.entries()[middle].text,text,control)?{std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>{self.entries_mut()[middle].repeated=true;return Ok(())}}}
  let symbol=Symbol{text,repeated:false};if self.length<64&&self.heap.capacity()==0{self.inline[self.length]=symbol;}else{if self.heap.len()==self.heap.capacity(){let capacity=if self.heap.capacity()==0{128}else{self.heap.capacity().checked_mul(2).ok_or_else(||error(K::WorkLimit,"borrowed symbol capacity overflow"))?};let mut next=control.allocate_vec(capacity)?;next.extend_from_slice(self.entries());self.heap=next;control.checkpoint()?;}self.heap.push(symbol);}self.length+=1;for index in (low+1..self.length).rev(){control.step()?;self.entries_mut()[index]=self.entries()[index-1];}self.entries_mut()[low]=symbol;Ok(())
 }
 fn finish(&mut self,maximum:u64,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  let mut written=0;for index in 0..self.length{control.step()?;let symbol=self.entries()[index];if symbol.text.len()<=128||symbol.repeated{self.entries_mut()[written]=symbol;written+=1;}}self.length=written;if self.heap.capacity()>0{self.heap.truncate(written);}if written as u64>maximum{return Err(error(K::WorkLimit,"borrowed symbol count exceeds declared limit"))}Ok(())
 }
 fn index(&self,text:&str,control:&mut NativeEncodeControl<'_>)->Result<Option<u64>,ValueError>{let(mut low,mut high)=(0,self.length);while low<high{control.step()?;let middle=low+(high-low)/2;match controlled_schema::compare_text(self.entries()[middle].text,text,control)?{std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>return Ok(Some(middle as u64))}}Ok(None)}
}
fn fields(spec:R)->Result<([usize;256],usize),ValueError>{if spec.fields.len()>256{return Err(error(K::WorkLimit,"borrowed record exceeds inline field-order frontier"))}let mut ids=[0;256];for(index,slot)in ids[..spec.fields.len()].iter_mut().enumerate(){*slot=index;}ids[..spec.fields.len()].sort_unstable_by_key(|index|(spec.fields[*index].id,*index));Ok((ids,spec.fields.len()))}
fn check(depth:usize,maximum:u16)->Result<(),ValueError>{if depth>=64||depth>usize::from(maximum){Err(error(K::DepthLimit,"borrowed Pack field exceeds declared depth"))}else{Ok(())}}
fn collect<'a,T:FieldProjectionSource>(source:&'a T,shape:H,path:&mut[usize;64],depth:usize,symbols:&mut Symbols<'a>,maximum:u16,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
 check(depth,maximum)?;control.step()?;if matches!(shape,H::Value){return collect_intrinsic(source,path,depth,symbols,maximum,control)}match(source.projection_view(&path[..depth])?,shape){
  (V::Text(text),_)=>symbols.note(text,control),
  (V::Record(ids),H::Record(make))=>{let spec=make();if ids.len()!=spec.fields.len()||ids.iter().zip(spec.fields).any(|(id,f)|*id!=f.id){return Err(error(K::InvalidValue,"borrowed record identities differ from schema"))}let(order,length)=fields(spec)?;for index in order[..length].iter().copied(){path[depth]=index;collect(source,spec.fields[index].shape,path,depth+1,symbols,maximum,control)?;}Ok(())},
  (V::List(length),H::List(make))|(V::Tuple(length),H::Tuple(make,_))=>{for index in 0..length{path[depth]=index;collect(source,make(),path,depth+1,symbols,maximum,control)?;}Ok(())},
  (V::Block,H::Block(make))=>{path[depth]=0;collect(source,make(),path,depth+1,symbols,maximum,control)},
  (V::Absent,_)|(V::Bool(_),_)|(V::Int(_),_)|(V::UInt(_),_)|(V::Float(_),_)|(V::Enum(_),_)|(V::Bytes(_),_)=>Ok(()),
  _=>Err(error(K::UnsupportedOwner,"borrowed Pack symbol shape has no immutable visitor"))
 }
}
fn collect_intrinsic<'a,T:FieldProjectionSource>(source:&'a T,path:&mut[usize;64],depth:usize,symbols:&mut Symbols<'a>,maximum:u16,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
 check(depth+1,maximum)?;control.step()?;match source.projection_view(&path[..depth])?{
  V::IntrinsicText(text)=>symbols.note(text,control),
  V::IntrinsicArray(length)|V::IntrinsicObject(length)=>{for index in 0..length{path[depth]=index;collect_intrinsic(source,path,depth+1,symbols,maximum,control)?;}Ok(())},
  V::IntrinsicNull|V::IntrinsicBool(_)|V::IntrinsicNumber(_)|V::IntrinsicBytes(_)=>Ok(()),
  _=>Err(error(K::InvalidValue,"borrowed intrinsic symbol visitor disagrees with Value owner"))
 }
}
fn collect_record<'a,T:FieldProjectionSource>(source:&'a T,spec:R,symbols:&mut Symbols<'a>,maximum:u16,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{let V::Record(ids)=source.projection_view(&[])?else{return Err(error(K::InvalidValue,"borrowed root is not a record"))};if ids.len()!=spec.fields.len()||ids.iter().zip(spec.fields).any(|(id,f)|*id!=f.id){return Err(error(K::InvalidValue,"borrowed root field identities differ"))}let(order,length)=fields(spec)?;let mut path=[0;64];for index in order[..length].iter().copied(){path[0]=index;collect(source,spec.fields[index].shape,&mut path,1,symbols,maximum,control)?;}Ok(())}
struct Segment<'a>{cursor:&'a mut crate::codec::DeflateMeasure,codec:u8,raw:usize}
impl Segment<'_>{
 fn finish(&mut self,control:&mut NativeEncodeControl<'_>)->Result<usize,ValueError>{let stored=if self.codec==0{self.raw}else{self.cursor.finish(&mut||control.step())?};segment_size(self.raw,stored,self.codec)}
}
impl Sink for Segment<'_>{fn bytes(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.raw=self.raw.checked_add(bytes.len()).ok_or_else(||error(K::WorkLimit,"borrowed Pack segment length overflow"))?;for byte in bytes{if self.codec!=0{self.cursor.push(*byte,&mut||control.step())?;}control.step()?;}Ok(())}}
fn varint_size(mut value:usize)->usize{let mut size=1;while value>=128{size+=1;value>>=7;}size}
fn segment_size(raw:usize,stored:usize,codec:u8)->Result<usize,ValueError>{2usize.checked_add(varint_size(stored)).and_then(|n|n.checked_add(if codec!=0{varint_size(raw)}else{0})).and_then(|n|n.checked_add(stored)).and_then(|n|n.checked_add(4)).ok_or_else(||error(K::WorkLimit,"borrowed Pack framed size overflow"))}
struct Document<'a>{segment:Segment<'a>,frame:usize,total:usize,raw:usize,frames:usize}
impl Document<'_>{fn flush(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if self.segment.raw!=0{self.total=self.total.checked_add(self.segment.finish(control)?).ok_or_else(||error(K::WorkLimit,"borrowed document span overflow"))?;self.frames+=1;self.segment.raw=0;self.segment.cursor.reset();}Ok(())}}
impl Sink for Document<'_>{fn bytes(&mut self,mut bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.raw=self.raw.checked_add(bytes.len()).ok_or_else(||error(K::WorkLimit,"borrowed document raw length overflow"))?;while !bytes.is_empty(){let length=bytes.len().min(self.frame-self.segment.raw);self.segment.bytes(&bytes[..length],control)?;bytes=&bytes[length..];if self.segment.raw==self.frame{self.flush(control)?;}}Ok(())}}
fn emit_record<T:FieldProjectionSource>(source:&T,spec:R,path:&mut[usize;64],depth:usize,symbols:&Symbols<'_>,options:&EncodeOptions,sink:&mut impl Sink,control:&mut NativeEncodeControl<'_>)->Result<usize,ValueError>{
 check(depth,options.limits.max_depth)?;let V::Record(ids)=source.projection_view(&path[..depth])?else{return Err(error(K::InvalidValue,"borrowed field is not Record"))};if ids.len()!=spec.fields.len()||ids.iter().zip(spec.fields).any(|(id,f)|*id!=f.id){return Err(error(K::InvalidValue,"borrowed field identities differ from declared schema"))}let(order,length)=fields(spec)?;let mut present=0;for index in order[..length].iter().copied(){path[depth]=index;if !matches!(source.projection_view(&path[..depth+1])?,V::Absent){present+=1;}}
 sink.varint(present as u64,control)?;for index in order[..length].iter().copied(){path[depth]=index;if matches!(source.projection_view(&path[..depth+1])?,V::Absent){continue}sink.varint(spec.fields[index].id as u64,control)?;emit_value(source,spec.fields[index].shape,path,depth+1,symbols,options,sink,control)?;}Ok(present)
}
fn emit_value<T:FieldProjectionSource>(source:&T,shape:H,path:&mut[usize;64],depth:usize,symbols:&Symbols<'_>,options:&EncodeOptions,sink:&mut impl Sink,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
 check(depth,options.limits.max_depth)?;control.step()?;if matches!(shape,H::Value){sink.byte(TAG_VALUE,control)?;return emit_intrinsic(source,path,depth,symbols,options,sink,control)}match(source.projection_view(&path[..depth])?,shape){
  (V::Absent,_)=>sink.byte(TAG_ABSENT,control),(V::Bool(value),_)=>sink.byte(if value{TAG_TRUE}else{TAG_FALSE},control),
  (V::Int(value),_)=>{sink.byte(TAG_INT,control)?;sink.signed(value,control)},(V::UInt(value),_)=>{sink.byte(TAG_UINT,control)?;sink.varint(value,control)},(V::Float(value),_)=>{sink.byte(TAG_F64,control)?;sink.bytes(&value.to_le_bytes(),control)},(V::Enum(value),_)=>{sink.byte(TAG_ENUM,control)?;sink.varint(value as u64,control)},
  (V::Text(text),_)=>if let Some(index)=symbols.index(text,control)?{sink.byte(TAG_STR,control)?;sink.varint(index,control)}else{sink.byte(TAG_STR_INLINE,control)?;sink.varint(text.len()as u64,control)?;sink.bytes(text.as_bytes(),control)},
  (V::Bytes(bytes),_)=>{if bytes.len()as u64>=options.chunk_threshold{return Err(error(K::UnsupportedOwner,"borrowed chunked bytes require retained chunk-table measurement"))}sink.byte(TAG_BYTES,control)?;sink.varint(bytes.len()as u64,control)?;sink.bytes(bytes,control)},
  (V::Record(_),H::Record(make))=>{sink.byte(TAG_RECORD,control)?;emit_record(source,make(),path,depth,symbols,options,sink,control)?;Ok(())},
  (V::List(length),H::List(make))|(V::Tuple(length),H::Tuple(make,_))=>{
   let mut floats=length>0;let mut ints=length>0;let mut enums=length>0;let mut uints=length>0;for index in 0..length{control.step()?;path[depth]=index;let view=source.projection_view(&path[..depth+1])?;floats&=matches!(view,V::Float(_));ints&=matches!(view,V::Int(_));enums&=matches!(view,V::Enum(_));uints&=matches!(view,V::UInt(value)if value<=i64::MAX as u64);}
   let signed=ints||enums||uints;sink.byte(if floats{TAG_PACKED_F64}else if signed{TAG_PACKED_VARINT}else if matches!(shape,H::Tuple(_,_)){TAG_TUPLE}else{TAG_LIST},control)?;sink.varint(length as u64,control)?;
   for index in 0..length{path[depth]=index;if floats{let V::Float(value)=source.projection_view(&path[..depth+1])?else{return Err(error(K::InvariantViolated,"borrowed packed float changed"))};sink.bytes(&value.to_le_bytes(),control)?;}else if signed{let value=match source.projection_view(&path[..depth+1])?{V::Int(value)=>value,V::UInt(value)=>value as i64,V::Enum(value)=>value as i64,_=>return Err(error(K::InvariantViolated,"borrowed packed integer changed"))};sink.signed(value,control)?;}else{emit_value(source,make(),path,depth+1,symbols,options,sink,control)?;}}Ok(())
  },
  (V::Block,H::Block(make))=>{sink.byte(TAG_BLOCK,control)?;path[depth]=0;emit_value(source,make(),path,depth+1,symbols,options,sink,control)},
  _=>Err(error(K::UnsupportedOwner,"borrowed Pack shape has no immutable byte emitter"))
 }
}

fn emit_intrinsic<T:FieldProjectionSource>(source:&T,path:&mut[usize;64],depth:usize,symbols:&Symbols<'_>,options:&EncodeOptions,sink:&mut impl Sink,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
 check(depth+1,options.limits.max_depth)?;control.step()?;match source.projection_view(&path[..depth])?{
  V::IntrinsicNull=>sink.byte(TAG_NULL,control),V::IntrinsicBool(value)=>sink.byte(if value{TAG_TRUE}else{TAG_FALSE},control),
  V::IntrinsicNumber(Number::UInt(value))=>{sink.byte(TAG_UINT,control)?;sink.varint(value,control)},
  V::IntrinsicNumber(Number::Int(value))=>{sink.byte(TAG_INT,control)?;sink.signed(value,control)},
  V::IntrinsicNumber(Number::Float(value))=>{sink.byte(TAG_F64,control)?;sink.bytes(&value.to_le_bytes(),control)},
  V::IntrinsicText(text)=>if let Some(index)=symbols.index(text,control)?{sink.byte(TAG_STR,control)?;sink.varint(index,control)}else{sink.byte(TAG_STR_INLINE,control)?;sink.varint(text.len()as u64,control)?;sink.bytes(text.as_bytes(),control)},
  V::IntrinsicBytes(bytes)=>{if bytes.len()as u64>=options.chunk_threshold{return Err(error(K::UnsupportedOwner,"borrowed intrinsic chunked bytes require retained chunk-table measurement"))}sink.byte(TAG_BYTES,control)?;sink.varint(bytes.len()as u64,control)?;sink.bytes(bytes,control)},
  V::IntrinsicArray(length)|V::IntrinsicObject(length)=>{let object=matches!(source.projection_view(&path[..depth])?,V::IntrinsicObject(_));sink.byte(if object{TAG_MAP}else{TAG_LIST},control)?;sink.varint(length as u64,control)?;for index in 0..length{control.step()?;if object{let key=source.projection_key(&path[..depth],index)?;sink.byte(TAG_STR_INLINE,control)?;sink.varint(key.len()as u64,control)?;sink.bytes(key.as_bytes(),control)?;}path[depth]=index;emit_intrinsic(source,path,depth+1,symbols,options,sink,control)?;}Ok(())},
  _=>Err(error(K::InvalidValue,"borrowed intrinsic byte visitor disagrees with Value owner"))
 }
}
pub(super) fn borrowed_schema_hash(spec:R,control:&mut NativeEncodeControl<'_>)->Result<[u8;32],ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;schema::hash(spec,control)})}

/// 📏️ Measures the complete canonical producer without constructing its output or source metadata.
pub fn measure_document_borrowed<T:FieldProjectionSource>(source:&T,spec:&R,options:&EncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<usize,ValueError>{
 if options.codec.0>1{return Err(error(K::UnsupportedOwner,"borrowed Pack codec is not authored"))}
 control.scoped_stage(|control|{control.begin_stage(0)?;control.checkpoint()?;let mut symbols=Symbols::new();collect_record(source,*spec,&mut symbols,options.limits.max_depth,control)?;symbols.finish(options.limits.max_symbols as u64,control)?;
 let hash=schema::hash(*spec,control)?;let mut cursor=crate::codec::DeflateMeasure::new();let mut symbol_segment=Segment{cursor:&mut cursor,codec:options.codec.0,raw:0};symbol_segment.varint(symbols.length as u64,control)?;for symbol in symbols.entries(){symbol_segment.varint(symbol.text.len()as u64,control)?;symbol_segment.bytes(symbol.text.as_bytes(),control)?;}let symbol_size=symbol_segment.finish(control)?;cursor.reset();
 let doc_start=crate::format::HEADER_SIZE.checked_add(symbol_size).ok_or_else(||error(K::WorkLimit,"borrowed Pack document offset overflow"))?;
 let frame=usize::try_from(options.frame_size.max(1)).map_err(|_|error(K::WorkLimit,"borrowed Pack frame exceeds address space"))?;let mut document=Document{segment:Segment{cursor:&mut cursor,codec:options.codec.0,raw:0},frame,total:0,raw:0,frames:0};let field_count=emit_record(source,*spec,&mut[0;64],0,&symbols,options,&mut document,control)?;document.flush(control)?;let(doc_span,raw,frames)=(document.total,document.raw,document.frames);cursor.reset();
 let mut manifest=Segment{cursor:&mut cursor,codec:options.codec.0,raw:0};manifest.varint(0,control)?;manifest.bytes(&hash,control)?;for value in [doc_start,doc_span,frames,crate::format::HEADER_SIZE,symbol_size,0,0,0,0,raw,field_count,0,symbols.length]{manifest.varint(value as u64,control)?;}let manifest_size=manifest.finish(control)?;
 let total=doc_start.checked_add(doc_span).and_then(|n|n.checked_add(manifest_size)).and_then(|n|n.checked_add(7+crate::format::FOOTER_SIZE)).ok_or_else(||error(K::WorkLimit,"borrowed Pack file size overflow"))?;if total as u64>options.limits.max_file_len{return Err(error(K::WorkLimit,"borrowed Pack file exceeds declared exact allowance"))}control.checkpoint()?;Ok(total)
 })
}
