//! 🫳️ Exact canonical Pack output from original immutable ordinal fields.
use super::*;
use semio_framework_dsl_record::{BorrowedShape as B,BorrowedRecordSpec as S};
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource,FieldProjectionView as V};
use super::retained_symbols::{ProjectedSymbolScratch,SourceTextLocator,SourceTextKind};
use semio_framework_value::list::PagedList;
const ORDER_CAPACITY:usize=isize::MAX as usize;

fn mismatch()->PackRefusal{ValueError::new(ValueRefusalKind::InvariantViolated,"borrowed Pack source disagrees with declared field").into()}
fn child(path:&mut[usize;64],depth:usize,index:usize,_maximum:u16)->Result<usize,PackRefusal>{
    if depth>=path.len(){return Err(ValueError::new(ValueRefusalKind::DepthLimit,"borrowed Pack source exceeds declared path depth").into())}
    path[depth]=index;Ok(depth+1)
}
fn level_limit(level:u16,maximum:u16)->Result<(),PackRefusal>{if level>maximum{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"borrowed Pack canonical depth exceeds caller limit").into())}Ok(())}
fn record_ids<T:FieldProjectionSource>(source:&T,path:&[usize])->Result<&'static[u16],PackRefusal>{match source.projection_view(path)?{V::Record(ids)=>Ok(ids),_=>Err(mismatch())}}
fn record(shape:Option<B>,control:&mut NativeEncodeControl<'_>)->Result<Option<S>,PackRefusal>{match shape{Some(B::Record(produce))=>read_record(produce,control).map(Some),_=>Ok(None)}}
fn read_record(produce:fn()->S,control:&mut NativeEncodeControl<'_>)->Result<S,PackRefusal>{control.checkpoint()?;control.step()?;Ok(produce())}
fn edge(shape:Option<B>,role:u8,control:&mut NativeEncodeControl<'_>)->Result<Option<B>,PackRefusal>{let produce=match(shape,role){(Some(B::Tuple(produce,_)|B::List(produce)),0)|(Some(B::Block(produce)),1)|(Some(B::Map(produce)),2)=>Some(produce),_=>None};produce.map(|produce|{control.checkpoint()?;control.step()?;Ok(produce())}).transpose()}
fn inner(shape:Option<B>,control:&mut NativeEncodeControl<'_>)->Result<Option<B>,PackRefusal>{edge(shape,0,control)}
fn table_spec_of(shape:Option<B>)->Option<fn()->S>{match shape{Some(B::Table(produce))=>Some(produce),_=>None}}
fn statements_variants(shape:Option<B>)->Option<&'static[(&'static str,fn()->S)]>{match shape{Some(B::Statements(variants))=>Some(variants),_=>None}}
fn element_tag(shape:B)->u8{match shape{B::Float|B::Quantity(_)|B::Angle(_)=>ELEM_F64,B::Int=>ELEM_INT,B::UInt|B::Count=>ELEM_UINT,B::Enum(_)=>ELEM_ENUM,B::Bool=>ELEM_BOOL,B::Text|B::Ref(_)=>ELEM_STR,_=>ELEM_FALLBACK}}

struct InlineOrder{first:Option<usize>,tail:PagedList<usize,ORDER_CAPACITY>}
impl InlineOrder{
 const fn empty()->Self{Self{first:None,tail:PagedList::empty()}}
 fn len(&self)->usize{usize::from(self.first.is_some())+self.tail.len()}
 fn is_empty(&self)->bool{self.first.is_none()&&self.tail.is_empty()}
 fn has_reserved_slot(&self)->bool{self.first.is_none()||self.tail.has_reserved_slot()}
 fn allocated_bytes(&self)->usize{self.tail.allocated_bytes()}
 fn push_reserved(&mut self,value:usize)->Result<(),usize>{if self.first.is_none(){self.first=Some(value);Ok(())}else{self.tail.push_reserved(value)}}
 fn pop(&mut self)->Option<usize>{self.tail.pop().or_else(||self.first.take())}
 fn swap(&mut self,left:usize,right:usize){let value=self[left];self[left]=self[right];self[right]=value;}
}
impl std::ops::Index<usize> for InlineOrder{type Output=usize;fn index(&self,index:usize)->&usize{if index==0{self.first.as_ref().expect("initialized inline order")}else{&self.tail[index-1]}}}
impl std::ops::IndexMut<usize> for InlineOrder{fn index_mut(&mut self,index:usize)->&mut usize{if index==0{self.first.as_mut().expect("initialized inline order")}else{&mut self.tail[index-1]}}}
#[derive(Clone,Copy,PartialEq,Eq)]
enum InlineSymbolPhase{Discovering,Spilling,Ready,Retiring}
struct InlineSymbols{first:Option<(SourceTextLocator,bool)>,overflow:ProjectedSymbolScratch,spilled:bool,selected:bool,phase:InlineSymbolPhase}
impl InlineSymbols{
 const fn empty()->Self{Self{first:None,overflow:ProjectedSymbolScratch::empty(),spilled:false,selected:false,phase:InlineSymbolPhase::Discovering}}
 fn allocated_bytes(&self)->usize{self.overflow.allocated_bytes()}
 fn note<T:FieldProjectionSource>(&mut self,source:&T,locator:SourceTextLocator,forced:bool,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  if self.phase!=InlineSymbolPhase::Discovering{return Err(inline_invalid())}control.checkpoint()?;let _=locator.resolve(source)?;
  if self.spilled{return self.overflow.note(source,locator,forced,control)}
  let Some((first,first_forced))=self.first else{self.first=Some((locator,forced));return Ok(())};
  self.phase=InlineSymbolPhase::Spilling;self.overflow.note(source,first,first_forced,control)?;self.overflow.note(source,locator,forced,control)?;self.first=None;self.spilled=true;self.phase=InlineSymbolPhase::Discovering;Ok(())
 }
 fn finish<T:FieldProjectionSource>(&mut self,source:&T,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  if self.phase!=InlineSymbolPhase::Discovering{return Err(inline_invalid())}control.checkpoint()?;control.step()?;
  if self.spilled{self.overflow.finish(source,control)?;}else{self.selected=match self.first{Some((locator,forced))=>forced||locator.resolve(source)?.len()<=128,None=>false};}
  self.phase=InlineSymbolPhase::Ready;Ok(())
 }
 fn len(&self)->Result<usize,ValueError>{if self.phase!=InlineSymbolPhase::Ready{return Err(inline_invalid())}if self.spilled{self.overflow.len()}else{Ok(usize::from(self.selected))}}
 fn locator(&self,index:usize)->Result<SourceTextLocator,ValueError>{if index>=self.len()?{return Err(inline_invalid())}if self.spilled{self.overflow.locator(index)}else{self.first.map(|(locator,_)|locator).ok_or_else(inline_invalid)}}
 fn index<T:FieldProjectionSource>(&self,source:&T,text:&str,control:&mut NativeEncodeControl<'_>)->Result<Option<u64>,ValueError>{
  let length=self.len()?;if self.spilled{return self.overflow.index(source,text,control)}if length==0{return Ok(None)}let original=self.locator(0)?.resolve(source)?;controlled_schema::compare_text(original,text,control).map(|order|(order==Ordering::Equal).then_some(0))
 }
 fn retire_one(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<(bool,usize,usize),ValueError>{
  if maximum_items==0||maximum_bytes==0{return Ok((false,0,0))}self.phase=InlineSymbolPhase::Retiring;if self.first.take().is_some(){return Ok((true,1,0))}self.overflow.retire_one(maximum_items,maximum_bytes)
 }
 fn return_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize,maximum_bytes:usize)->Result<(bool,usize),ValueError>{
  if maximum_items==0||maximum_bytes==0{return Ok((false,0))}self.phase=InlineSymbolPhase::Retiring;if self.first.take().is_some(){return Ok((true,0))}self.overflow.return_one(parent,maximum_items,maximum_bytes)
 }
}
fn inline_invalid()->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,"inline Pack scratch has no complete source grant")}

struct Discover<'a>{scratch:&'a mut InlineSymbols}
impl Discover<'_>{
    fn projected<T:FieldProjectionSource>(&mut self,source:&T,shape:Option<B>,path:&mut[usize;64],depth:usize,level:u16,maximum:u16,forced:bool,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
        level_limit(level,maximum.min(64))?;
        control.checkpoint()?;control.step()?;
        match source.projection_view(&path[..depth])?{
            V::Text(text)=>self.scratch.note(source,SourceTextLocator::new(&path[..depth],SourceTextKind::Text)?,forced,control)?,
            V::Record(ids)=>{let spec=record(shape,control)?;self.projected_fields(source,spec.as_ref(),ids,path,depth,level+1,maximum,control)?;},
            V::List(length)|V::Tuple(length)=>{
                let table=table_spec_of(shape).map(|producer|read_record(producer,control)).transpose()?;
                for index in 0..length{let next=child(path,depth,index,maximum)?;if let Some(spec)=&table{let ids=record_ids(source,&path[..next])?;for(field_index,id)in ids.iter().enumerate(){if let Some(field)=spec.fields.iter().find(|field|field.id==*id){let at=child(path,next,field_index,maximum)?;self.projected(source,Some(field.shape),path,at,level+1,maximum,matches!(field.shape,B::Text|B::Ref(_)),control)?;}}}else{self.projected(source,inner(shape,control)?,path,next,level+1,maximum,false,control)?;}}
            },
            V::Block=>{let next=child(path,depth,0,maximum)?;self.projected(source,edge(shape,1,control)?,path,next,level+1,maximum,false,control)?;},
            V::Map(length)=>{for index in 0..length{self.scratch.note(source,SourceTextLocator::new(&path[..depth],SourceTextKind::Key(index))?,false,control)?;let next=child(path,depth,index,maximum)?;self.projected(source,edge(shape,2,control)?,path,next,level+1,maximum,false,control)?;}},
            V::Statements(length)=>{for index in 0..length{let keyword=source.projection_key(&path[..depth],index)?;self.scratch.note(source,SourceTextLocator::new(&path[..depth],SourceTextKind::Key(index))?,true,control)?;let spec=statements_variants(shape).and_then(|variants|variants.iter().find(|(key,_)|*key==keyword)).map(|(_,producer)|read_record(*producer,control)).transpose()?;let next=child(path,depth,index,maximum)?;let ids=record_ids(source,&path[..next])?;self.projected_fields(source,spec.as_ref(),ids,path,next,level+1,maximum,control)?;}},
            V::Wire(_)=>{for index in 0..6{let next=child(path,depth,index,maximum)?;match source.projection_view(&path[..next])?{V::Text(text)=>self.scratch.note(source,SourceTextLocator::new(&path[..next],SourceTextKind::Text)?,false,control)?,V::Absent=>{},_=>return Err(mismatch())}}let next=child(path,depth,8,maximum)?;self.projected_dynamic(source,path,next,level+1,maximum,control)?;},
            V::IntrinsicNull|V::IntrinsicBool(_)|V::IntrinsicNumber(_)|V::IntrinsicText(_)|V::IntrinsicBytes(_)|V::IntrinsicArray(_)|V::IntrinsicObject(_)=>self.projected_dynamic(source,path,depth,level+1,maximum,control)?,
            V::Absent|V::Bool(_)|V::Int(_)|V::UInt(_)|V::Float(_)|V::Enum(_)|V::Bytes(_)=>{},
        }Ok(())
    }
    fn projected_fields<T:FieldProjectionSource>(&mut self,source:&T,spec:Option<&S>,ids:&[u16],path:&mut[usize;64],depth:usize,level:u16,maximum:u16,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
        level_limit(level,maximum.min(64))?;
        for(index,id)in ids.iter().enumerate(){let next=child(path,depth,index,maximum)?;let shape=spec.and_then(|spec|spec.fields.iter().find(|field|field.id==*id)).map(|field|field.shape);self.projected(source,shape,path,next,level+1,maximum,false,control)?;}Ok(())
    }
    fn projected_dynamic<T:FieldProjectionSource>(&mut self,source:&T,path:&mut[usize;64],depth:usize,level:u16,maximum:u16,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
        level_limit(level,maximum)?;control.checkpoint()?;control.step()?;
        match source.projection_view(&path[..depth])?{V::IntrinsicText(text)=>self.scratch.note(source,SourceTextLocator::new(&path[..depth],SourceTextKind::IntrinsicText)?,false,control)?,V::IntrinsicArray(length)|V::IntrinsicObject(length)=>{for index in 0..length{let next=child(path,depth,index,maximum)?;self.projected_dynamic(source,path,next,level+1,maximum,control)?;}},V::IntrinsicNull|V::IntrinsicBool(_)|V::IntrinsicNumber(_)|V::IntrinsicBytes(_)=>{},_=>return Err(mismatch())}Ok(())
    }
}

struct BorrowedEncoder<'a,'c,'d,T:FieldProjectionSource>{source:&'a T,symbols:&'a InlineSymbols,options:&'a EncodeOptions,control:&'c mut NativeEncodeControl<'d>,order:&'c mut InlineOrder}
impl<T:FieldProjectionSource> BorrowedEncoder<'_,'_,'_,T>{
    fn text(&mut self,text:&str,inline:bool,output:&mut Output)->Result<(),PackRefusal>{if !inline{if let Some(index)=self.symbols.index(self.source,text,self.control)?{output.byte(TAG_STR,self.control)?;return output.varint(index,self.control)}}output.byte(TAG_STR_INLINE,self.control)?;output.varint(text.len()as u64,self.control)?;output.bytes(text.as_bytes(),self.control)}
    fn forced(&mut self,text:&str,output:&mut Output)->Result<(),PackRefusal>{let index=self.symbols.index(self.source,text,self.control)?.ok_or_else(mismatch)?;output.varint(index,self.control)}
    fn bytes(&mut self,bytes:&[u8],output:&mut Output)->Result<(),PackRefusal>{output.byte(TAG_BYTES,self.control)?;output.varint(bytes.len()as u64,self.control)?;output.bytes(bytes,self.control)}
    fn symbols(&mut self,output:&mut Output)->Result<(),PackRefusal>{let length=self.symbols.len()?;output.varint(length as u64,self.control)?;for index in 0..length{self.control.checkpoint()?;let text=self.symbols.locator(index)?.resolve(self.source)?;output.varint(text.len()as u64,self.control)?;output.bytes(text.as_bytes(),self.control)?;}Ok(())}
    fn projected_fields(&mut self,source:&T,spec:Option<&S>,path:&mut[usize;64],depth:usize,level:u16,output:&mut Output)->Result<(),PackRefusal>{
        level_limit(level,self.options.limits.max_depth.min(64))?;
        let ids=record_ids(source,&path[..depth])?;let start=self.order.len();
        for(index,id)in ids.iter().enumerate(){self.control.checkpoint()?;if ids[..index].contains(id){return Err(mismatch())}let next=child(path,depth,index,self.options.limits.max_depth.min(64))?;if !matches!(source.projection_view(&path[..next])?,V::Absent)&&(self.options.preserve_unknown||spec.is_some_and(|spec|spec.fields.iter().any(|field|field.id==*id))){order_push(self.order,index,self.control)?;}}
        order_sort(self.order,start,self.control,|left,right,_|Ok((ids[left],left).cmp(&(ids[right],right))))?;let end=self.order.len();output.varint((end-start)as u64,self.control)?;
        self.control.begin_stage(0)?;
        for cursor in start..end{let index=self.order[cursor];let id=ids[index];let next=child(path,depth,index,self.options.limits.max_depth.min(64))?;let shape=spec.and_then(|spec|spec.fields.iter().find(|field|field.id==id)).map(|field|field.shape);output.varint(u64::from(id),self.control)?;self.projected(source,shape,path,next,level+1,output)?;}
        while self.order.len()>start{self.order.pop();}Ok(())
    }
    fn projected(&mut self,source:&T,shape:Option<B>,path:&mut[usize;64],depth:usize,level:u16,output:&mut Output)->Result<(),PackRefusal>{
        level_limit(level,self.options.limits.max_depth.min(64))?;
        self.control.checkpoint()?;
        let view=source.projection_view(&path[..depth])?;
        if matches!(view,V::IntrinsicNull|V::IntrinsicBool(_)|V::IntrinsicNumber(_)|V::IntrinsicText(_)|V::IntrinsicBytes(_)|V::IntrinsicArray(_)|V::IntrinsicObject(_)){output.byte(TAG_VALUE,self.control)?;return self.projected_dynamic(source,path,depth,level+1,output)}
        match view{
            V::Absent=>output.byte(TAG_ABSENT,self.control),V::Bool(value)=>output.byte(if value{TAG_TRUE}else{TAG_FALSE},self.control),
            V::Int(value)=>{output.byte(TAG_INT,self.control)?;output.signed(value,self.control)},V::UInt(value)=>{output.byte(TAG_UINT,self.control)?;output.varint(value,self.control)},V::Float(value)=>{output.byte(TAG_F64,self.control)?;output.bytes(&value.to_le_bytes(),self.control)},V::Enum(value)=>{output.byte(TAG_ENUM,self.control)?;output.varint(u64::from(value),self.control)},
            V::Text(text)=>self.text(text,false,output),V::Bytes(bytes)=>self.bytes(bytes,output),
            V::Record(_)=>{let spec=record(shape,self.control)?;output.byte(TAG_RECORD,self.control)?;self.projected_fields(source,spec.as_ref(),path,depth,level+1,output)},
            V::List(length)|V::Tuple(length)=>{if let Some(producer)=table_spec_of(shape){self.projected_table(source,producer,length,path,depth,level,output)}else{{let shape=inner(shape,self.control)?;self.projected_sequence(source,shape,length,matches!(view,V::Tuple(_)),path,depth,level,output)}}},
            V::Block=>{output.byte(TAG_BLOCK,self.control)?;let next=child(path,depth,0,self.options.limits.max_depth.min(64))?;{let shape=edge(shape,1,self.control)?;self.projected(source,shape,path,next,level+1,output)}},
            V::Map(length)=>{
                let start=self.order.len();
                for index in 0..length{self.control.checkpoint()?;let next=child(path,depth,index,self.options.limits.max_depth.min(64))?;if !matches!(source.projection_view(&path[..next])?,V::Absent){order_push(self.order,index,self.control)?;}}
                order_sort(self.order,start,self.control,|left,right,control|{let order=controlled_schema::compare_text(source.projection_key(&path[..depth],left)?,source.projection_key(&path[..depth],right)?,control)?;Ok(order.then_with(||left.cmp(&right)))})?;
                let end=self.order.len();output.byte(TAG_MAP,self.control)?;output.varint((end-start)as u64,self.control)?;for cursor in start..end{let index=self.order[cursor];let key=source.projection_key(&path[..depth],index)?;self.text(key,false,output)?;let next=child(path,depth,index,self.options.limits.max_depth.min(64))?;let shape=edge(shape,2,self.control)?;self.projected(source,shape,path,next,level+1,output)?;}
                while self.order.len()>start{self.order.pop();}Ok(())
            },
            V::Statements(length)=>{output.byte(TAG_STATEMENTS,self.control)?;output.varint(length as u64,self.control)?;for index in 0..length{self.control.checkpoint()?;let key=source.projection_key(&path[..depth],index)?;self.forced(key,output)?;let spec=statements_variants(shape).and_then(|variants|variants.iter().find(|(keyword,_)|*keyword==key)).map(|(_,producer)|read_record(*producer,self.control)).transpose()?;let next=child(path,depth,index,self.options.limits.max_depth.min(64))?;self.projected_fields(source,spec.as_ref(),path,next,level+1,output)?;}Ok(())},
            V::Wire(directed)=>{output.byte(TAG_WIRE,self.control)?;self.projected_wire(source,directed,path,depth,level,output)},
            _=>Err(mismatch()),
        }
    }
    fn projected_sequence(&mut self,source:&T,shape:Option<B>,length:usize,tuple:bool,path:&mut[usize;64],depth:usize,level:u16,output:&mut Output)->Result<(),PackRefusal>{
        let(mut floats,mut ints,mut enums,mut uints)=(length>0,length>0,length>0,length>0);
        for index in 0..length{self.control.step()?;let next=child(path,depth,index,self.options.limits.max_depth.min(64))?;let value=source.projection_view(&path[..next])?;floats&=matches!(value,V::Float(_));ints&=matches!(value,V::Int(_));enums&=matches!(value,V::Enum(_));uints&=matches!(value,V::UInt(value)if value<=i64::MAX as u64);}
        let kind=if floats{Some(NumKind::F64)}else if ints||enums||uints{Some(NumKind::Varint)}else{None};
        output.byte(match &kind{Some(NumKind::F64)=>TAG_PACKED_F64,Some(NumKind::Varint)=>TAG_PACKED_VARINT,None=>if tuple{TAG_TUPLE}else{TAG_LIST}},self.control)?;output.varint(length as u64,self.control)?;
        for index in 0..length{self.control.checkpoint()?;let next=child(path,depth,index,self.options.limits.max_depth.min(64))?;match(kind.as_ref(),source.projection_view(&path[..next])?){(Some(NumKind::F64),V::Float(value))=>output.bytes(&value.to_le_bytes(),self.control)?,(Some(NumKind::Varint),V::Int(value))=>output.signed(value,self.control)?,(Some(NumKind::Varint),V::UInt(value))if value<=i64::MAX as u64=>output.signed(value as i64,self.control)?,(Some(NumKind::Varint),V::Enum(value))=>output.signed(i64::from(value),self.control)?,(None,_)=>self.projected(source,shape,path,next,level+1,output)?,_=>return Err(mismatch())}}Ok(())
    }
    fn projected_dynamic(&mut self,source:&T,path:&mut[usize;64],depth:usize,level:u16,output:&mut Output)->Result<(),PackRefusal>{
        level_limit(level,self.options.limits.max_depth)?;
        self.control.checkpoint()?;
        match source.projection_view(&path[..depth])?{
            V::IntrinsicNull=>output.byte(TAG_NULL,self.control),V::IntrinsicBool(value)=>output.byte(if value{TAG_TRUE}else{TAG_FALSE},self.control),
            V::IntrinsicNumber(Number::UInt(value))=>{output.byte(TAG_UINT,self.control)?;output.varint(value,self.control)},V::IntrinsicNumber(Number::Int(value))=>{output.byte(TAG_INT,self.control)?;output.signed(value,self.control)},V::IntrinsicNumber(Number::Float(value))=>{output.byte(TAG_F64,self.control)?;output.bytes(&value.to_le_bytes(),self.control)},
            V::IntrinsicText(text)=>self.text(text,false,output),V::IntrinsicBytes(bytes)=>self.bytes(bytes,output),
            V::IntrinsicArray(length)=>{output.byte(TAG_LIST,self.control)?;output.varint(length as u64,self.control)?;for index in 0..length{let next=child(path,depth,index,self.options.limits.max_depth.min(64))?;self.projected_dynamic(source,path,next,level+1,output)?;}Ok(())},
            V::IntrinsicObject(length)=>{output.byte(TAG_MAP,self.control)?;output.varint(length as u64,self.control)?;for index in 0..length{let key=source.projection_key(&path[..depth],index)?;self.text(key,true,output)?;let next=child(path,depth,index,self.options.limits.max_depth.min(64))?;self.projected_dynamic(source,path,next,level+1,output)?;}Ok(())},
            _=>Err(mismatch()),
        }
    }
    fn projected_text<'source>(&mut self,source:&'source T,path:&mut[usize;64],depth:usize,index:usize)->Result<Option<&'source str>,PackRefusal>{
        self.control.checkpoint()?;let next=child(path,depth,index,self.options.limits.max_depth.min(64))?;match source.projection_view(&path[..next])?{V::Absent=>Ok(None),V::Text(text)=>Ok(Some(text)),_=>Err(mismatch())}
    }
    fn projected_node(&mut self,source:&T,path:&mut[usize;64],depth:usize,index:usize,output:&mut Output)->Result<(),PackRefusal>{
        let id=self.projected_text(source,path,depth,index)?.ok_or_else(mismatch)?;let kind=self.projected_text(source,path,depth,index+1)?;let port=self.projected_text(source,path,depth,index+2)?;output.byte(u8::from(kind.is_some())|(u8::from(port.is_some())<<1),self.control)?;self.text(id,false,output)?;if let Some(kind)=kind{self.text(kind,false,output)?;}if let Some(port)=port{self.text(port,false,output)?;}Ok(())
    }
    fn projected_wire(&mut self,source:&T,directed:Option<bool>,path:&mut[usize;64],depth:usize,level:u16,output:&mut Output)->Result<(),PackRefusal>{
        let id=self.projected_text(source,path,depth,6)?;let kind=self.projected_text(source,path,depth,7)?;let label=id.is_some()||kind.is_some();let presence=u8::from(directed.is_some())|(u8::from(directed==Some(true))<<1)|(u8::from(label)<<2);output.byte(presence,self.control)?;self.projected_node(source,path,depth,0,output)?;if directed.is_some(){self.projected_node(source,path,depth,3,output)?;}if label{output.byte(u8::from(id.is_some())|(u8::from(kind.is_some())<<1),self.control)?;if let Some(id)=id{self.text(id,false,output)?;}if let Some(kind)=kind{self.text(kind,false,output)?;}}let next=child(path,depth,8,self.options.limits.max_depth.min(64))?;self.projected_dynamic(source,path,next,level+1,output)
    }
    fn projected_cell(&mut self,source:&T,path:&mut[usize;64],depth:usize,row:usize,id:u16)->Result<Option<usize>,PackRefusal>{
        self.control.checkpoint()?;let next=child(path,depth,row,self.options.limits.max_depth.min(64))?;let ids=record_ids(source,&path[..next])?;let Some(column)=ids.iter().position(|actual|*actual==id)else{return Ok(None)};let cell=child(path,next,column,self.options.limits.max_depth.min(64))?;Ok((!matches!(source.projection_view(&path[..cell])?,V::Absent)).then_some(cell))
    }
    fn projected_table(&mut self,source:&T,producer:fn()->S,length:usize,path:&mut[usize;64],depth:usize,level:u16,output:&mut Output)->Result<(),PackRefusal>{
        let spec=read_record(producer,self.control)?;let start=self.order.len();for index in 0..spec.fields.len(){self.control.checkpoint()?;order_push(self.order,index,self.control)?;}order_sort(self.order,start,self.control,|left,right,_|Ok((spec.fields[left].id,left).cmp(&(spec.fields[right].id,right))))?;let end=self.order.len();output.byte(TAG_TABLE_SOA,self.control)?;output.varint(length as u64,self.control)?;output.varint((end-start)as u64,self.control)?;
        for cursor in start..end{let field=&spec.fields[self.order[cursor]];let mut dense=true;for row in 0..length{dense&=self.projected_cell(source,path,depth,row,field.id)?.is_some();}output.varint(u64::from(field.id),self.control)?;output.byte(u8::from(!dense),self.control)?;
            if !dense{for start in (0..length).step_by(8){let mut byte=0;for(row,bit)in(start..length.min(start+8)).zip(0..8){if self.projected_cell(source,path,depth,row,field.id)?.is_some(){byte|=1<<bit;}}output.byte(byte,self.control)?;}}
            let tag=element_tag(field.shape);output.byte(tag,self.control)?;
            if tag==ELEM_BOOL{for start in(0..length).step_by(8){let mut byte=0;for(row,bit)in(start..length.min(start+8)).zip(0..8){if let Some(cell)=self.projected_cell(source,path,depth,row,field.id)?{if matches!(source.projection_view(&path[..cell])?,V::Bool(true)){byte|=1<<bit;}}}output.byte(byte,self.control)?;}continue}
            for row in 0..length{let Some(cell)=self.projected_cell(source,path,depth,row,field.id)?else{continue};match(tag,source.projection_view(&path[..cell])?){(ELEM_F64,V::Float(value))=>output.bytes(&value.to_le_bytes(),self.control)?,(ELEM_INT,V::Int(value))=>output.signed(value,self.control)?,(ELEM_UINT,V::UInt(value))=>output.varint(value,self.control)?,(ELEM_ENUM,V::Enum(value))=>output.varint(u64::from(value),self.control)?,(ELEM_STR,V::Text(text))=>self.forced(text,output)?,(ELEM_F64|ELEM_INT|ELEM_UINT|ELEM_ENUM|ELEM_STR,_)=>{},_=>self.projected(source,Some(field.shape),path,cell,level+1,output)?,}}
        }while self.order.len()>start{self.order.pop();}Ok(())
    }
    fn projected_body(&mut self,source:&T,spec:&S,output:&mut Output)->Result<(),PackRefusal>{self.symbols(output)?;self.projected_fields(source,Some(spec),&mut[0;64],0,0,output)}
}

fn order_push(owner:&mut InlineOrder,index:usize,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
 while !owner.has_reserved_slot(){let required=owner.tail.next_allocation_bytes().map_err(|error|ValueError::new(ValueRefusalKind::AllocationFailed,error.reason))?;if required>4096{return Err(mismatch())}control.charge(required)?;control.checkpoint()?;let step=owner.tail.reserve_one(required).map_err(|error|ValueError::new(ValueRefusalKind::AllocationFailed,error.reason))?;if !step.progressed||step.allocated_bytes>required{return Err(mismatch())}}
 owner.push_reserved(index).map_err(|_|mismatch())
}
fn order_sort(owner:&mut InlineOrder,start:usize,control:&mut NativeEncodeControl<'_>,mut compare:impl FnMut(usize,usize,&mut NativeEncodeControl<'_>)->Result<Ordering,PackRefusal>)->Result<(),PackRefusal>{
 fn sift(owner:&mut InlineOrder,start:usize,mut root:usize,length:usize,control:&mut NativeEncodeControl<'_>,compare:&mut impl FnMut(usize,usize,&mut NativeEncodeControl<'_>)->Result<Ordering,PackRefusal>)->Result<(),PackRefusal>{loop{control.checkpoint()?;control.step()?;let Some(left)=root.checked_mul(2).and_then(|index|index.checked_add(1)).filter(|index|*index<length)else{return Ok(())};let right=left+1;let largest=if right<length&&compare(owner[start+left],owner[start+right],control)?==Ordering::Less{right}else{left};if compare(owner[start+root],owner[start+largest],control)?!=Ordering::Less{return Ok(())}owner.swap(start+root,start+largest);root=largest;}}
 let length=owner.len()-start;for root in(0..length/2).rev(){sift(owner,start,root,length,control,&mut compare)?;}for end in(1..length).rev(){owner.swap(start,start+end);sift(owner,start,0,end,control,&mut compare)?;}Ok(())
}

/// 🎒️ Binds one immutable source and retains every paid canonical scratch page on all returns.
pub struct BorrowedProjectedPackOperation<T:FieldProjectionSource>{source:T,symbols:InlineSymbols,order:InlineOrder,phase:OperationPhase,spec:S,variant_identity:Option<(&'static str,usize)>}
#[derive(Clone,Copy,PartialEq,Eq)]
enum OperationPhase{Fresh,Admitting,Ready,Failed,Retiring}
impl<T:FieldProjectionSource> BorrowedProjectedPackOperation<T>{
 /// 🌱️ Captures the immutable typed source and declared static schema with no backing allocation.
 const fn new(source:T,spec:S)->Self{Self{source,symbols:InlineSymbols::empty(),order:InlineOrder::empty(),phase:OperationPhase::Fresh,spec,variant_identity:None}}
 /// 📏️ Exposes only physically retained scratch backing, independently of semantic source ownership.
 pub fn allocated_bytes(&self)->usize{self.symbols.allocated_bytes()+self.order.allocated_bytes()}
 /// 📐️ Reports finite caller-embedded locator and scalar scratch without claiming heap allocation.
 pub fn inline_storage_bytes(&self)->usize{std::mem::size_of::<InlineSymbols>()+std::mem::size_of::<InlineOrder>()}
 /// 🖨️ Writes exact canonical bytes under the caller control and retains scratch on success or refusal.
 pub fn write_body(&mut self,options:&EncodeOptions,output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<usize,PackRefusal>{
  if !matches!(self.phase,OperationPhase::Fresh|OperationPhase::Ready){return Err(mismatch())}
  let maximum=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX).min(control.maximum_bytes());
  let result=control.scoped_maximum(maximum,|control|control.scoped_stage(|control|{
   control.begin_stage(0)?;
   if self.phase==OperationPhase::Fresh{self.phase=OperationPhase::Admitting;let ids=record_ids(&self.source,&[])?;Discover{scratch:&mut self.symbols}.projected_fields(&self.source,Some(&self.spec),ids,&mut[0;64],0,0,options.limits.max_depth,control)?;self.symbols.finish(&self.source,control)?;self.phase=OperationPhase::Ready;}
   let mut encoder=BorrowedEncoder{source:&self.source,symbols:&self.symbols,options,control,order:&mut self.order};let mut measured=Output::measure();encoder.projected_body(&self.source,&self.spec,&mut measured)?;
   if measured.length as u64>options.limits.max_file_len{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"borrowed Record exceeds max_file_len").into())}
   let mut emitted=Output{bytes:None,external:Some(output),length:0};encoder.projected_body(&self.source,&self.spec,&mut emitted)?;if emitted.length!=measured.length{return Err(mismatch())}Ok(emitted.length)
  }));if result.is_err(){self.phase=OperationPhase::Failed;}result
 }
 /// ♻️ Revokes complete access and retires one scalar or one actual backing within the caller grant.
 pub fn retire_one(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<(bool,usize,usize),ValueError>{
  if maximum_items==0||maximum_bytes==0{return Ok((false,0,0))}self.phase=OperationPhase::Retiring;
  if !self.order.is_empty(){self.order.pop();return Ok((true,1,0))}
  if self.order.allocated_bytes()>0{let step=self.order.tail.release_empty_page(maximum_bytes).map_err(|error|ValueError::new(ValueRefusalKind::AllocationFailed,error.reason))?;return Ok((step.progressed,usize::from(step.progressed),step.released_allocation_bytes))}
  self.symbols.retire_one(maximum_items,maximum_bytes)
 }
 /// 🏠️ Returns genuine retained backing to the explicitly admitted parent without physical disposal credit.
 pub fn return_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize,maximum_bytes:usize)->Result<(bool,usize),ValueError>{
  if maximum_items==0||maximum_bytes==0{return Ok((false,0))}self.phase=OperationPhase::Retiring;
  if !self.order.is_empty(){self.order.pop();return Ok((true,0))}
  if self.order.allocated_bytes()>0{if self.order.tail.next_release_allocation_bytes().map_err(|error|ValueError::new(ValueRefusalKind::AllocationFailed,error.reason))?>maximum_bytes{return Ok((false,0))}let step=self.order.tail.return_empty_page(parent,1)?;return Ok((step.progressed,step.returned_allocation_bytes))}
  self.symbols.return_one(parent,maximum_items,maximum_bytes)
 }
}

/// 🫳️ Keeps the original source borrowed while the typed capsule owns only its paid scratch.
pub struct BorrowedPackSource<'a,T:FieldProjectionSource>{source:&'a T}
impl<T:FieldProjectionSource> FieldProjectionSource for BorrowedPackSource<'_,T>{
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{self.source.projection_view(path)}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.source.projection_key(path,index)}
}
impl<'a,T:FieldProjectionSource> BorrowedProjectedPackOperation<BorrowedPackSource<'a,T>>{
 /// 🔗️ Binds the exact original immutable record without moving or copying its semantic owner.
 pub const fn from_source(source:&'a T,spec:S)->Self{Self::new(BorrowedPackSource{source},spec)}
}
impl<'a,T:semio_framework_dsl_record::DslVariants+semio_framework_dsl_record::BorrowedDslVariants> BorrowedProjectedPackOperation<semio_framework_dsl_record::native_encoding::VariantProjection<'a,T>>{
 /// 🌿️ Binds the actual original variant and its authored static schema with no metadata factory.
 pub fn from_variant(source:&'a T)->Self{let(keyword,ordinal,spec)=source.projected_borrowed_variant_identity();let mut operation=Self::new(semio_framework_dsl_record::native_encoding::VariantProjection::new(source),spec);operation.variant_identity=Some((keyword,ordinal));operation}
 /// 🏷️ Reads the captured header identity from the same immutable original variant as the body.
 pub fn variant_identity(&self)->Result<(&'static str,usize),PackRefusal>{if !matches!(self.phase,OperationPhase::Fresh|OperationPhase::Ready){return Err(mismatch())}self.variant_identity.ok_or_else(mismatch)}
}
