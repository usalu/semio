from pathlib import Path
import os
p=Path('🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🪙️escrow/🦀️.rs')
s=p.read_text(encoding='utf8'); original=s
def r(a,b):
 global s
 assert s.count(a)==1,(a[:60],s.count(a))
 s=s.replace(a,b)
r('const MAX_METADATA:usize=384;', 'const MAX_METADATA:usize=1024;')
r('+std::mem::size_of::<Option<OriginalEscrowHeaderReturned>>()}', '+std::mem::size_of::<Option<OriginalEscrowHeaderReturned>>() +3*std::mem::size_of::<Option<ValueError>>() }')
r('header_ok:bool,header_state:u8}', 'header_ok:bool,header_state:u8,header_codec_failure:Option<ValueError>,header_retained_failure:Option<ValueError>,header_work_failure:Option<ValueError>}')
r('header_ok:false,header_state:0}}', 'header_ok:false,header_state:0,header_codec_failure:None,header_retained_failure:None,header_work_failure:None}}')
r('items:18,copy_bytes:MAX+header+MAX_METADATA,work:129+88*32', 'items:21,copy_bytes:MAX+header+MAX_METADATA,work:132+88*32')
r('self.borrowed.is_none()&&self.header_state==0}', 'self.borrowed.is_none()&&self.header_state==0&&self.header_codec_failure.is_none()&&self.header_retained_failure.is_none()&&self.header_work_failure.is_none()}')
r('items:5,copy_bytes:length+std::mem::size_of::<usize>()+std::mem::size_of::<u64>()+std::mem::size_of::<bool>()+std::mem::size_of::<u8>(),work:HEADER_WORK+4', 'items:8,copy_bytes:length+std::mem::size_of::<usize>()+std::mem::size_of::<u64>()+std::mem::size_of::<bool>()+std::mem::size_of::<u8>()+3*std::mem::size_of::<Option<ValueError>>(),work:HEADER_WORK+7')
r("pub fn with_preparing_header<E>(&mut self,write:impl for<'loan> FnOnce(&'loan[u8],&mut OriginalEscrowHeaderCursor<'loan>)->Result<(),E>)->Result<Result<(),E>,ValueError>{if self.borrowed.is_none()||self.header_state!=1{return Err(invalid())}let mut cursor=OriginalEscrowHeaderCursor{bytes:&mut self.header[..self.header_length],written:0,work:0};let result=write(&self.bytes[..self.length],&mut cursor);self.header_written=cursor.written;self.header_work=cursor.work;self.header_ok=result.is_ok();self.header_state=2;Ok(result)}",
"pub fn with_preparing_header(&mut self,write:impl for<'loan> FnOnce(&'loan[u8],&mut OriginalEscrowHeaderCursor<'loan>)->Result<(),ValueError>)->Result<bool,ValueError>{if self.borrowed.is_none()||self.header_state!=1||self.header_codec_failure.is_some()||self.header_retained_failure.is_some()||self.header_work_failure.is_some(){return Err(invalid())}let mut cursor=OriginalEscrowHeaderCursor{bytes:&mut self.header[..self.header_length],written:0,work:0};let result=write(&self.bytes[..self.length],&mut cursor);self.header_written=cursor.written;self.header_work=cursor.work;self.header_ok=result.is_ok();self.header_codec_failure=result.err();self.header_state=2;Ok(self.header_ok)}")
r('items:5,copy_bytes:self.header_written+std::mem::size_of::<usize>()+std::mem::size_of::<u64>()+std::mem::size_of::<bool>()+std::mem::size_of::<u8>(),work:self.header_work+4', 'items:5+usize::from(self.header_codec_failure.is_some()),copy_bytes:self.header_written+std::mem::size_of::<usize>()+std::mem::size_of::<u64>()+std::mem::size_of::<bool>()+std::mem::size_of::<u8>()+if self.header_codec_failure.is_some(){std::mem::size_of::<Option<ValueError>>()}else{0},work:self.header_work+4+u64::from(self.header_codec_failure.is_some())')
needle=' /// 🧊️ The loan can originate only from this Source-owned header and capsule.'
insert=""" /// 👁️ The original codec and independent posting failures remain in Source-owned cells.
 pub fn header_outcome(&self)->Result<(usize,u64,Option<&ValueError>,Option<&ValueError>,Option<&ValueError>),ValueError>{if self.header_state!=2{return Err(invalid())}Ok((self.header_written,self.header_work,self.header_codec_failure.as_ref(),self.header_retained_failure.as_ref(),self.header_work_failure.as_ref()))}
 /// 📏️ Each independent posting failure placement is a separate Source effect.
 pub fn header_post_failure_demand()->OriginalEscrowPublicationDemand{OriginalEscrowPublicationDemand{items:1,copy_bytes:std::mem::size_of::<Option<ValueError>>(),work:1}}
 /// 📥️ The caller funds each actual placement and fences a failure to post that effect.
 pub fn retain_header_post_failure(&mut self,retained:bool,error:ValueError)->Result<(),ValueError>{let slot=if retained{&mut self.header_retained_failure}else{&mut self.header_work_failure};if self.header_state!=2||slot.is_some(){return Err(error)}*slot=Some(error);Ok(())}
"""
assert s.count(needle)==1;s=s.replace(needle,insert+needle)
r('self.header_state!=2||!self.header_ok||self.header_written!=self.header_length', 'self.header_state!=2||!self.header_ok||self.header_retained_failure.is_some()||self.header_work_failure.is_some()||self.header_written!=self.header_length')
r('if !matches!(self.header_state,1|2){return Err(invalid())}let d=Self::header_begin_demand()', 'if !matches!(self.header_state,1|2)||self.header_codec_failure.is_some()||self.header_retained_failure.is_some()||self.header_work_failure.is_some(){return Err(invalid())}let d=Self::header_begin_demand()')
assert p.read_text(encoding='utf8')==original
tmp=p.with_name(p.name+'.terra.tmp');tmp.write_text(s,encoding='utf8',newline='')
assert p.read_text(encoding='utf8')==original;os.replace(tmp,p)
