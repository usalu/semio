/// 🧩️ A finite intrinsic semantic cell retains short and partially transitioned text without heap backing.
pub struct InlineTextBuffer{bytes:[u8;128],length:usize,complete:bool}
impl InlineTextBuffer{
    pub const fn empty()->Self{Self{bytes:[0;128],length:0,complete:false}}
    pub fn as_bytes(&self)->&[u8]{&self.bytes[..self.length]}
    pub fn borrow(&self)->Result<&str,ValueError>{if !self.complete{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic text lacks a complete live UTF8 capability"))}std::str::from_utf8(self.as_bytes()).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"complete intrinsic UTF8 changed"))}
    pub fn close_one(&mut self,maximum_items:usize)->bool{if maximum_items==0||self.length==0{return false}self.length=0;self.complete=false;true}
    pub fn terminal_is_empty(&self)->bool{self.length==0}
    pub fn read_from_source(&mut self,source:&impl TextReadSource,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{self.read_controlled(source,control)}
    pub fn read_from_encoding_source(&mut self,source:&impl TextReadSource,control:&mut crate::NativeEncodeControl<'_>)->Result<(),ValueError>{self.read_controlled(source,control)}
    fn read_controlled<C:TextCopyControl>(&mut self,source:&impl TextReadSource,control:&mut C)->Result<(),ValueError>{
        if self.length!=0||self.complete{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic text source construction requires an empty owner"))}
        if source.byte_len()>self.bytes.len(){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"intrinsic text exceeds declared128-byte capacity"))}
        control.scope_copy(|control|{control.begin_copy(source.byte_len())?;for index in 0..source.byte_len(){self.bytes[index]=source.byte_at(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"intrinsic text source changed"))?;self.length+=1;control.advance_copy(1)?;}std::str::from_utf8(self.as_bytes()).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"intrinsic semantic source is not UTF8"))?;Ok::<_,ValueError>(())})?;self.complete=true;Ok(())
    }
}
struct InlineTextOutput<'a,'b,const N:usize>{paged:TextOutput<'a,'b,N>,inline:&'a mut InlineTextBuffer,paged_active:&'a mut bool,logical_bytes:usize,refused:bool}
impl<const N:usize> TextEncodingOutput for InlineTextOutput<'_,'_,N>{
    fn write_text(&mut self,text:&str)->Result<(),ValueError>{
        if self.refused{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic semantic text output previously refused"))}
        let result=(||{
            let length=self.logical_bytes.checked_add(text.len()).filter(|length|*length<=self.paged.maximum_payload_bytes&&*length<=N).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"intrinsic semantic text exceeds logical payload authority"))?;
            if !*self.paged_active&&length<=self.inline.bytes.len(){
                self.paged.control.scoped_stage(|control|{control.begin_stage(text.len())?;for byte in text.as_bytes(){self.inline.bytes[self.inline.length]=*byte;self.inline.length+=1;control.step()?;}Ok::<_,ValueError>(())})?;
            }else{
                if !*self.paged_active{*self.paged_active=true;let prefix=std::str::from_utf8(self.inline.as_bytes()).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"accepted intrinsic semantic prefix changed UTF8"))?;self.paged.write_text(prefix)?;self.inline.close_one(1);}
                self.paged.write_text(text)?;
            }
            self.logical_bytes=length;Ok::<_,ValueError>(())
        })();if result.is_err(){self.refused=true;}result
    }
}
impl<const N:usize> PagedText<N>{
    /// 🧾 One bound producer fills the intrinsic cell or transfers its bounded prefix into genuine pages.
    pub fn encode_with_inline(&mut self,inline:&mut InlineTextBuffer,paged_active:&mut bool,maximum_payload_bytes:usize,control:&mut crate::NativeEncodeControl<'_>,producer:impl FnOnce(&mut dyn TextEncodingOutput)->Result<(),ValueError>)->Result<(),ValueError>{
        if self.bytes.allocated_bytes()!=0||self.complete||self.closing||!inline.terminal_is_empty()||inline.complete||*paged_active{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic semantic text encoding requires an empty composite owner"))}
        if maximum_payload_bytes>N{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"intrinsic semantic text authority exceeds declared capacity"))}
        let paged=TextOutput{bytes:&mut self.bytes,control,maximum_payload_bytes,refused:false};let mut output=InlineTextOutput{paged,inline,paged_active,logical_bytes:0,refused:false};producer(&mut output)?;
        if output.refused||output.paged.refused{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic semantic text producer ignored a refused write"))}
        self.complete=*output.paged_active;output.inline.complete=!*output.paged_active;Ok(())
    }
}
