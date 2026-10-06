#[path = "../🔏️fingerprint/🦀️.rs"]
mod fingerprint;
use semio_framework_os_kernel::{os_pack::{codec::PackEncodeOptions,PackRefusal},os_spr::io::binary::operation_bytes::{OperationByteOutput,OperationSourceCollection,with_operation_encode_policy}};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind,list::PagedList,paged_text::{TextReadSource,TextEncodingOutput}};
use std::cmp::Ordering;

fn invalid(detail:&'static str)->PackRefusal{ValueError::new(ValueRefusalKind::InvariantViolated,detail).into()}

/// 🪪️ One immutable original child identity and its original operation spans.
pub struct ChildFingerprintSource<'a>{pub identifier:&'a dyn TextReadSource,pub operations:&'a dyn OperationSourceCollection}
/// 🗂️ The accepted caller ledger retains all child sources throughout sorting and hashing.
pub trait ChildFingerprintSources{fn len(&self)->usize;fn child_at(&self,index:usize)->Option<ChildFingerprintSource<'_>>;}

#[derive(Clone,Copy)]
enum EntityDomain{Child,Invocation}
/// 🪪️ A finite ID receipt crosses the hash/output borrow boundary without a String owner.
pub struct EntityId{domain:EntityDomain,hex:[u8;16]}
impl EntityId{
    pub fn encode_into(&self,output:&mut dyn TextEncodingOutput)->Result<(),ValueError>{
        let result=(||{let prefix=match self.domain{EntityDomain::Child=>"child",EntityDomain::Invocation=>"invocation"};let hex=std::str::from_utf8(&self.hex).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"entity digest is not finite ASCII"))?;output.measure_text_fragment(prefix.len(),prefix)?;output.measure_text_fragment(prefix.len()+1,"-")?;output.measure_text_fragment(prefix.len()+17,hex)?;output.declare_exact_byte_len(prefix.len()+17)?;output.write_text(prefix)?;output.write_text("-")?;output.write_text(hex)?;Ok(())})();if result.is_err(){output.refuse_text_producer();}result
    }
}

struct HashOutput{hasher:semio_framework_hash::Hasher}
impl OperationByteOutput for HashOutput{
    fn write_bytes(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
        for span in bytes.chunks(256){control.checkpoint()?;self.hasher.update(span);control.advance(span.len())?;}Ok(())
    }
}
impl HashOutput{
    fn text(&mut self,source:&dyn TextReadSource,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
        let mut bytes=[0;256];for at in(0..source.byte_len()).step_by(bytes.len()){control.checkpoint()?;let count=(source.byte_len()-at).min(bytes.len());for(index,target)in bytes[..count].iter_mut().enumerate(){*target=source.byte_at(at+index).ok_or_else(||invalid("entity identity source octet changed"))?;}self.write_bytes(&bytes[..count],control)?;}Ok(())
    }
    fn finish(&self,domain:EntityDomain)->EntityId{let digest=self.hasher.finalize();let alphabet=b"0123456789abcdef";let mut hex=[0;16];for(index,byte)in digest.as_bytes()[..8].iter().enumerate(){hex[index*2]=alphabet[(byte>>4)as usize];hex[index*2+1]=alphabet[(byte&15)as usize];}EntityId{domain,hex}}
}
fn compare_children(sources:&dyn ChildFingerprintSources,left:usize,right:usize,control:&mut NativeEncodeControl<'_>)->Result<Ordering,PackRefusal>{
    let left_id=sources.child_at(left).ok_or_else(||invalid("left child ordinal changed"))?;let right_id=sources.child_at(right).ok_or_else(||invalid("right child ordinal changed"))?;
    for index in 0..left_id.identifier.byte_len().min(right_id.identifier.byte_len()){
        control.advance(1)?;let a=left_id.identifier.byte_at(index).ok_or_else(||invalid("left child identity octet changed"))?;let b=right_id.identifier.byte_at(index).ok_or_else(||invalid("right child identity octet changed"))?;if a!=b{return Ok(a.cmp(&b))}
    }
    Ok(left_id.identifier.byte_len().cmp(&right_id.identifier.byte_len()).then(left.cmp(&right)))
}
fn sift<const N:usize>(owner:&mut PagedList<usize,N>,sources:&dyn ChildFingerprintSources,start:usize,end:usize,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
    let mut root=start;loop{control.checkpoint()?;let child=root.checked_mul(2).and_then(|value|value.checked_add(1)).ok_or_else(||invalid("child index heap overflow"))?;if child>=end{return Ok(())}let mut selected=child;if child+1<end&&compare_children(sources,owner[child],owner[child+1],control)?.is_lt(){selected=child+1}if !compare_children(sources,owner[root],owner[selected],control)?.is_lt(){return Ok(())}owner.swap(root,selected);root=selected;}
}
fn sorted_indices_into<const N:usize>(sources:&dyn ChildFingerprintSources,owner:&mut PagedList<usize,N>,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
    if !owner.terminal_is_empty(){return Err(invalid("child identifier ordering requires an empty retained index recipient"))}
    for ordinal in 0..sources.len(){
        control.checkpoint()?;while !owner.has_reserved_slot(){let required=owner.next_allocation_bytes().map_err(ValueError::from)?;if required>4096{return Err(invalid("child identifier index page exceeds original grant"))}control.charge(required)?;let step=owner.reserve_one(4096).map_err(|error|ValueError::from(error.refusal()))?;if !step.progressed||step.allocated_bytes!=required{return Err(invalid("child identifier index allocation differs from admission"))}}
        owner.push_reserved(ordinal).map_err(|_|invalid("child identifier index reserved slot changed"))?;control.advance(1)?;
    }
    let length=owner.len();for start in(0..length/2).rev(){sift(owner,sources,start,length,control)?;}
    for end in(1..owner.len()).rev(){control.checkpoint()?;owner.swap(0,end);sift(owner,sources,0,end,control)?;}Ok(())
}

/// 🧬️ Hashes the original child domain without retaining an input or fingerprint copy.
pub fn child_entity_id_from_source(parent:&dyn TextReadSource,slot:&dyn TextReadSource,operations:&dyn OperationSourceCollection,ordinal:u32,options:&PackEncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<EntityId,PackRefusal>{
    with_operation_encode_policy(options,control,|control|control.scoped_stage(|control|{
        control.begin_stage(0)?;let mut hash=HashOutput{hasher:semio_framework_hash::Hasher::new()};hash.write_bytes(b"child\0",control)?;hash.text(parent,control)?;hash.write_bytes(&[0],control)?;hash.text(slot,control)?;hash.write_bytes(&[0],control)?;fingerprint::write_operation_fingerprint_into(operations,&mut hash,control)?;hash.write_bytes(&[0],control)?;hash.write_bytes(&ordinal.to_le_bytes(),control)?;Ok(hash.finish(EntityDomain::Child))
    }))
}

/// 🧾️ Retains a cancellable stable index owner and hashes the complete original invocation domain.
pub fn invocation_entity_id_from_source<const N:usize>(parent:&dyn TextReadSource,parent_operations:&dyn OperationSourceCollection,children:&dyn ChildFingerprintSources,indices:&mut PagedList<usize,N>,options:&PackEncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<EntityId,PackRefusal>{
    with_operation_encode_policy(options,control,|control|control.scoped_stage(|control|{
        control.begin_stage(0)?;sorted_indices_into(children,indices,control)?;let mut hash=HashOutput{hasher:semio_framework_hash::Hasher::new()};hash.write_bytes(b"invocation\0",control)?;hash.text(parent,control)?;hash.write_bytes(&[0],control)?;fingerprint::write_operation_fingerprint_into(parent_operations,&mut hash,control)?;
        for index in indices.iter(){control.checkpoint()?;let child=children.child_at(*index).ok_or_else(||invalid("sorted child source ordinal changed"))?;hash.write_bytes(&[0],control)?;hash.text(child.identifier,control)?;hash.write_bytes(&[0],control)?;fingerprint::write_operation_fingerprint_into(child.operations,&mut hash,control)?;}Ok(hash.finish(EntityDomain::Invocation))
    }))
}
