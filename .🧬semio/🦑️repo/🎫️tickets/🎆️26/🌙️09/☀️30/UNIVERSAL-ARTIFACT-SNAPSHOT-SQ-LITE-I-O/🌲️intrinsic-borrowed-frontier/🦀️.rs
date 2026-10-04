enum DynamicItems<'a>{Array(std::slice::Iter<'a,DslValue>),Object(std::slice::Iter<'a,(String,DslValue)>)}
struct DynamicFrame<'a>{items:DynamicItems<'a>,depth:u16}
impl<'a> DynamicFrame<'a>{
    fn next(&mut self)->Option<(Option<&'a str>,&'a DslValue)>{match &mut self.items{DynamicItems::Array(items)=>items.next().map(|value|(None,value)),DynamicItems::Object(items)=>items.next().map(|(key,value)|(Some(key.as_str()),value))}}
}
fn dynamic_child_depth(depth:u16,maximum:u16)->Result<u16,OutputError>{depth.checked_add(1).filter(|next|*next<=maximum).ok_or_else(||ValueError::new(ValueRefusalKind::DepthLimit,"Pack dynamic child exceeds declared depth").into())}

impl<'a> Symbols<'a>{
    fn dynamic(&mut self,value:&'a DslValue,depth:u16,maximum:u16,control:&mut NativeEncodeControl<'_>)->Result<(),OutputError>{
        control.scoped_stage(|control|{
            control.begin_stage(0)?;let mut frames=Vec::new();let(mut value,mut depth)=(value,depth);
            'visit:loop{
                if depth>maximum{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Pack dynamic symbol depth exceeds declared limit").into())}control.checkpoint()?;control.step()?;
                match value{
                    DslValue::String(text)=>self.note(text,false,control)?,
                    DslValue::Array(items) if !items.is_empty()=>{let depth=dynamic_child_depth(depth,maximum)?;push(&mut frames,DynamicFrame{items:DynamicItems::Array(items.iter()),depth},control)?;},
                    DslValue::Object(items) if !items.is_empty()=>{let depth=dynamic_child_depth(depth,maximum)?;push(&mut frames,DynamicFrame{items:DynamicItems::Object(items.iter()),depth},control)?;},
                    DslValue::Null|DslValue::Bool(_)|DslValue::Number(_)|DslValue::Bytes(_)|DslValue::Array(_)|DslValue::Object(_)=>{},
                }
                loop{let Some(frame)=frames.last_mut()else{return Ok(())};if let Some((_,child))=frame.next(){value=child;depth=frame.depth;continue 'visit;}frames.pop();}
            }
        })
    }
}

impl Encoder<'_,'_,'_,'_>{
    fn dynamic(&mut self,value:&DslValue,depth:u16,output:&mut Output)->Result<(),OutputError>{
        let mut frames=Vec::new();let(mut value,mut depth)=(value,depth);
        'visit:loop{
            if depth>self.options.limits.max_depth{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Pack dynamic output depth exceeds declared limit").into())}self.control.checkpoint()?;
            match value{
                DslValue::Null=>output.byte(TAG_NULL,self.control)?,DslValue::Bool(value)=>output.byte(if *value{TAG_TRUE}else{TAG_FALSE},self.control)?,
                DslValue::Number(Number::UInt(value))=>{output.byte(TAG_UINT,self.control)?;output.varint(*value,self.control)?;},
                DslValue::Number(Number::Int(value))=>{output.byte(TAG_INT,self.control)?;output.signed(*value,self.control)?;},
                DslValue::Number(Number::Float(value))=>{output.byte(TAG_F64,self.control)?;output.bytes(&value.to_le_bytes(),self.control)?;},
                DslValue::String(text)=>self.text(text,false,output)?,DslValue::Bytes(bytes)=>self.bytes(bytes,output)?,
                DslValue::Array(items)=>{output.byte(TAG_LIST,self.control)?;output.varint(items.len()as u64,self.control)?;if !items.is_empty(){let depth=dynamic_child_depth(depth,self.options.limits.max_depth)?;push(&mut frames,DynamicFrame{items:DynamicItems::Array(items.iter()),depth},self.control)?;}},
                DslValue::Object(items)=>{output.byte(TAG_MAP,self.control)?;output.varint(items.len()as u64,self.control)?;if !items.is_empty(){let depth=dynamic_child_depth(depth,self.options.limits.max_depth)?;push(&mut frames,DynamicFrame{items:DynamicItems::Object(items.iter()),depth},self.control)?;}},
            }
            loop{let Some(frame)=frames.last_mut()else{return Ok(())};if let Some((key,child))=frame.next(){value=child;depth=frame.depth;if let Some(key)=key{self.text(key,true,output)?;}continue 'visit;}frames.pop();}
        }
    }
}
