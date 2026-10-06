//! 🫳️ Original PDF 1.4 fields decode directly from immutable source with an explicit refusal recipient.
use super::{PdfMutation,ProtocolError,PackRefusal,insert_page,remove_page,move_page,resize_page,replace_page_text,set_snapshot,patch_snapshot};
use protocol::{codec::{ByteSpan,ByteReader,PackDecodeOptions,PackEncodeOptions},operation_bytes::{OperationByteComparison,OperationByteLimitedOutput,OperationByteOutput}};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,SnapshotRetirementStep};
use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc,PdfSnapshot};
use semio_s_artifact_stdio_contract::editing::patch::SnapshotPatchReadCursor;

fn refusal(kind:ValueRefusalKind,detail:&'static str)->ProtocolError{PackRefusal::RetainedMalformed{kind,what:"PDF 1.4 operation source",offset:0,detail}.into()}
fn ordinal(reader:&mut ByteReader<'_>)->Result<usize,ProtocolError>{usize::try_from(reader.read_u64_le()?).map_err(|_|refusal(ValueRefusalKind::OwnershipLimit,"index is outside native extent"))}
fn finite(reader:&mut ByteReader<'_>)->Result<f64,ProtocolError>{let value=reader.read_f64_le()?;if value.is_finite(){Ok(value)}else{Err(refusal(ValueRefusalKind::InvalidValue,"non-finite geometry"))}}
fn scalar(source:ByteSpan<'_>,position:&mut usize)->Result<char,ProtocolError>{
    let first=*source.get(*position).ok_or_else(||refusal(ValueRefusalKind::InvalidValue,"truncated UTF8"))?;
    let length=match first{0..=0x7f=>1,0xc2..=0xdf=>2,0xe0..=0xef=>3,0xf0..=0xf4=>4,_=>return Err(refusal(ValueRefusalKind::InvalidValue,"invalid UTF8"))};
    let mut bytes=[0;4];for(index,byte)in bytes[..length].iter_mut().enumerate(){*byte=*source.get(*position+index).ok_or_else(||refusal(ValueRefusalKind::InvalidValue,"truncated UTF8"))?;}
    let value=std::str::from_utf8(&bytes[..length]).map_err(|_|refusal(ValueRefusalKind::InvalidValue,"invalid UTF8"))?.chars().next().unwrap();*position+=length;Ok(value)
}
fn text(reader:&mut ByteReader<'_>,output:&mut String,options:&PackDecodeOptions,control:&mut NativeDecodeControl<'_>)->Result<(),ProtocolError>{
    let length=ordinal(reader)?;if length as u64>options.limits.max_segment_len{return Err(refusal(ValueRefusalKind::OwnershipLimit,"text segment exceeds caller limit"));}
    let source=reader.read_span(length)?;
    control.scoped_stage(|control|{control.begin_stage(length).map_err(PackRefusal::from)?;let mut position=0;while position<length{let before=position;scalar(source,&mut position)?;control.advance(position-before).map_err(PackRefusal::from)?;}Ok::<_,ProtocolError>(())})?;
    control.charge(length).map_err(PackRefusal::from)?;
    output.try_reserve_exact(length).map_err(|_|refusal(ValueRefusalKind::AllocationFailed,"text allocation failed"))?;
    control.scoped_stage(|control|{control.begin_stage(length).map_err(PackRefusal::from)?;let mut position=0;while position<length{let before=position;output.push(scalar(source,&mut position)?);control.advance(position-before).map_err(PackRefusal::from)?;}Ok::<_,ProtocolError>(())})
}
fn compare(operation:&PdfMutation,source:ByteSpan<'_>,options:&PackEncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<(),ProtocolError>{let mut comparison=OperationByteComparison::new(source);operation.encode_op_into(options,&mut comparison,control)?;comparison.finish()?;Ok(())}

struct DirectReadRetirement{operation:Option<PdfMutation>}
fn release_text(value:&mut String,maximum:usize)->Option<usize>{let bytes=value.capacity();if bytes==0{return None;}if bytes>maximum{return Some(0);}*value=String::new();Some(bytes)}
impl ErasedSnapshotRetirement for DirectReadRetirement{
    fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
        if items==0||bytes==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
        let pending=|released|SnapshotRetirementStep::Pending{released_items:usize::from(released!=0),released_bytes:released};
        let Some(operation)=self.operation.as_mut()else{return Ok(SnapshotRetirementStep::Complete)};
        match operation{
            PdfMutation::InsertPage(payload)=>if let Some(released)=release_text(&mut payload.page.text,bytes){return Ok(pending(released));},
            PdfMutation::ReplacePageText(payload)=>if let Some(released)=release_text(&mut payload.text,bytes){return Ok(pending(released));},
            PdfMutation::SetSnapshot(payload)=>{
                if let Some(page)=payload.snapshot.pages.last_mut(){if let Some(released)=release_text(&mut page.text,bytes){return Ok(pending(released));}payload.snapshot.pages.pop();return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0});}
                let allocated=payload.snapshot.pages.capacity()*std::mem::size_of::<PageDoc>();if allocated!=0{if allocated>bytes{return Ok(pending(0));}payload.snapshot.pages=Vec::new();return Ok(pending(allocated));}
                if let Some(released)=release_text(&mut payload.snapshot.schema,bytes){return Ok(pending(released));}
            },
            PdfMutation::PatchSnapshot(_)=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"patch owner belongs to its retained parser")),
            _=>{}
        }
        self.operation.take();Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0})
    }
    fn terminal_is_empty(&self)->bool{self.operation.is_none()}
    fn next_close_byte_demand(&self)->usize{match self.operation.as_ref(){Some(PdfMutation::InsertPage(payload))=>payload.page.text.capacity().max(1),Some(PdfMutation::ReplacePageText(payload))=>payload.text.capacity().max(1),Some(PdfMutation::SetSnapshot(payload))=>payload.snapshot.pages.last().map(|page|page.text.capacity().max(1)).unwrap_or_else(||(payload.snapshot.pages.capacity()*std::mem::size_of::<PageDoc>()).max(payload.snapshot.schema.capacity()).max(1)),Some(_)=>1,None=>0}}
}
impl Drop for DirectReadRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"PDF direct decoder retains actual candidate allocations");}}

fn direct_candidate(tag:u8)->Result<PdfMutation,ProtocolError>{Ok(match tag{
    0=>PdfMutation::InsertPage(insert_page::InsertPage{index:0,page:PageDoc{width:0.0,height:0.0,text:String::new()}}),
    1=>PdfMutation::RemovePage(remove_page::RemovePage{index:0}),
    2=>PdfMutation::MovePage(move_page::MovePage{from:0,to:0}),
    3=>PdfMutation::ResizePage(resize_page::ResizePage{index:0,width:0.0,height:0.0}),
    4=>PdfMutation::ReplacePageText(replace_page_text::ReplacePageText{index:0,text:String::new()}),
    5=>PdfMutation::SetSnapshot(set_snapshot::SetSnapshot{snapshot:PdfSnapshot{schema:String::new(),pages:Vec::new()}}),
    _=>return Err(refusal(ValueRefusalKind::InvalidValue,"unknown original operation tag"))
})}
fn fill_direct(candidate:&mut PdfMutation,reader:&mut ByteReader<'_>,options:&PackDecodeOptions,control:&mut NativeDecodeControl<'_>)->Result<(),ProtocolError>{
    match candidate{
        PdfMutation::InsertPage(payload)=>{payload.index=ordinal(reader)?;payload.page.width=finite(reader)?;payload.page.height=finite(reader)?;text(reader,&mut payload.page.text,options,control)?;},
        PdfMutation::RemovePage(payload)=>payload.index=ordinal(reader)?,
        PdfMutation::MovePage(payload)=>{payload.from=ordinal(reader)?;payload.to=ordinal(reader)?;},
        PdfMutation::ResizePage(payload)=>{payload.index=ordinal(reader)?;payload.width=finite(reader)?;payload.height=finite(reader)?;},
        PdfMutation::ReplacePageText(payload)=>{payload.index=ordinal(reader)?;text(reader,&mut payload.text,options,control)?;},
        PdfMutation::SetSnapshot(payload)=>{
            if options.limits.max_depth==0{return Err(refusal(ValueRefusalKind::DepthLimit,"snapshot depth exceeds caller limit"));}
            text(reader,&mut payload.snapshot.schema,options,control)?;
            let count=ordinal(reader)?;if count as u64>options.limits.max_items{return Err(refusal(ValueRefusalKind::WorkLimit,"snapshot page count exceeds caller limit"));}
            let allocation=count.checked_mul(std::mem::size_of::<PageDoc>()).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"snapshot page layout overflow"))?;control.charge(allocation).map_err(PackRefusal::from)?;payload.snapshot.pages.try_reserve_exact(count).map_err(|_|refusal(ValueRefusalKind::AllocationFailed,"page collection allocation failed"))?;
            for _ in 0..count{control.checkpoint().map_err(PackRefusal::from)?;let width=reader.read_f64_le()?;let height=reader.read_f64_le()?;payload.snapshot.pages.push(PageDoc{width,height,text:String::new()});text(reader,&mut payload.snapshot.pages.last_mut().unwrap().text,options,control)?;}
        },
        _=>unreachable!()
    }
    if reader.remaining()!=0{return Err(refusal(ValueRefusalKind::InvalidValue,"trailing operation octets"));}Ok(())
}
impl PdfMutation{
    /// 📏️ Declares the exact immediate wrapper allocation required before each owned source recipe.
    pub fn decode_retirement_allocation_bytes(tag:u8)->usize{match tag{0|4|5=>std::mem::size_of::<DirectReadRetirement>(),6=>SnapshotPatchReadCursor::retirement_allocation_bytes(),_=>0}}
    /// 🫳️ Decodes the exact original seven-frame source and returns every partial/scaffold owner to its caller.
    pub fn decode_op_span(source:ByteSpan<'_>,options:&PackDecodeOptions,canonical_options:&PackEncodeOptions,decoding:&mut NativeDecodeControl<'_>,encoding:&mut NativeEncodeControl<'_>)->Result<Self,ProtocolError>{
        decoding.checkpoint().map_err(PackRefusal::from)?;
        if source.len()as u64>options.limits.max_file_len{return Err(refusal(ValueRefusalKind::OwnershipLimit,"complete source exceeds caller limit"));}
        let mut reader=ByteReader::from_span(source);if reader.read_u8()?!=store::pack_rt::OP_BINARY_FORMAT{return Err(refusal(ValueRefusalKind::InvalidValue,"operation format mismatch"));}let tag=reader.read_u8()?;
        let maximum=options.limits.max_total_alloc.min(usize::MAX as u64)as usize;
        decoding.scoped_maximum(maximum,|decoding|Ok::<_,ValueError>((||{
            if tag==6{
                return decoding.with_retirement_owner(Self::decode_retirement_allocation_bytes(tag),|decoding|{
                    let body=reader.read_span(reader.remaining()).unwrap();
                    let mut cursor=match SnapshotPatchReadCursor::new(body,options){Ok(cursor)=>cursor,Err(error)=>return(Ok(Err(ProtocolError::from(PackRefusal::from(error)))),None)};
                    let result=(||{
                        while !cursor.step(256,decoding).map_err(PackRefusal::from)?{}
                        cursor.admit_patch(decoding).map_err(PackRefusal::from)?;
                        let mut comparison=OperationByteComparison::new(source);
                        protocol::operation_bytes::with_operation_encode_policy(canonical_options,encoding,|encoding|{
                            let mut limited=OperationByteLimitedOutput::new(&mut comparison,canonical_options.limits.max_file_len);limited.write_bytes(&[1,6],encoding)?;
                            cursor.patch_ref().unwrap().encode_op_into(canonical_options,&mut limited,encoding)
                        })?;comparison.finish()?;
                        Ok::<_,ProtocolError>(PdfMutation::PatchSnapshot(patch_snapshot::PatchSnapshot{patch:cursor.take_patch().unwrap()}))
                    })();
                    (Ok(result),Some(cursor.into_retirement()))
                }).map_err(PackRefusal::from)?;
            }
            let mut candidate=Some(direct_candidate(tag)?);
            if matches!(tag,1|2|3){fill_direct(candidate.as_mut().unwrap(),&mut reader,options,decoding)?;compare(candidate.as_ref().unwrap(),source,canonical_options,encoding)?;return Ok(candidate.take().unwrap());}
            decoding.with_retirement_owner(Self::decode_retirement_allocation_bytes(tag),|decoding|{
                let result=fill_direct(candidate.as_mut().unwrap(),&mut reader,options,decoding).and_then(|_|compare(candidate.as_ref().unwrap(),source,canonical_options,encoding)).map(|_|candidate.take().unwrap());
                (Ok::<_,ValueError>(result),Some(Box::new(DirectReadRetirement{operation:candidate})as Box<dyn ErasedSnapshotRetirement>))
            }).map_err(PackRefusal::from)?
        })())).map_err(PackRefusal::from)?
    }
}
