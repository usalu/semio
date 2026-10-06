//! 🧵️ Original length-prefixed fingerprint octets flow directly from the retained source collection.
use semio_framework_os_kernel::{os_spr::operation_bytes::{OperationByteOutput,OperationSourceCollection},os_pack::PackRefusal};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};

fn invalid(detail:&'static str)->PackRefusal{ValueError::new(ValueRefusalKind::InvariantViolated,detail).into()}

/// 🔢️ Preserves each original u64 little-endian length and every borrowed source octet.
pub fn write_operation_fingerprint_into(sources:&dyn OperationSourceCollection,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
    control.checkpoint()?;
    for ordinal in 0..sources.len(){
        control.checkpoint()?;
        let source=sources.source_at(ordinal).ok_or_else(||invalid("fingerprint source ordinal changed"))?;
        let length=u64::try_from(source.len()).map_err(|_|invalid("fingerprint source length exceeds original u64 authority"))?;
        output.write_bytes(&length.to_le_bytes(),control)?;
        let mut bytes=[0;256];
        for at in (0..source.len()).step_by(bytes.len()){
            control.checkpoint()?;
            let count=(source.len()-at).min(bytes.len());
            for(index,target)in bytes[..count].iter_mut().enumerate(){*target=*source.get(at+index).ok_or_else(||invalid("fingerprint source octet changed"))?;}
            output.write_bytes(&bytes[..count],control)?;
        }
    }
    control.checkpoint()?;Ok(())
}
