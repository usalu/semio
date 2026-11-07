import sys,os
if 'go' not in sys.argv[1:]: raise SystemExit(0)
D="🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔗️link/🧬️schema"
os.makedirs(D+"/📥️receiving",exist_ok=True)
open(D+"/📥️receiving/🦀️.rs","w",encoding="utf-8").write('''//! 📥️ An artifact link moves its target, pin and role strings out of their record without copying.
use super::{ArtifactLink,LinkPin};
use semio_framework_dsl_record::{receiving::owned::invalid,DslField,FieldValue};
use semio_framework_value::ValueError;
use semio_os_store_blob_identity::BlobRef;

fn pin_is_well_formed(pin:&FieldValue)->bool{
    let FieldValue::Record(record)=pin else{return false};
    if super::validate_link_pin_record(record).is_err(){return false}
    match record.get(0){
        Some(FieldValue::Enum(0))=>true,
        Some(FieldValue::Enum(1))=>matches!(record.get(1),Some(FieldValue::Text(_))),
        Some(FieldValue::Enum(2))=>matches!((record.get(2),record.get(3),record.get(4)),(Some(FieldValue::Text(_)),Some(FieldValue::UInt(_)),Some(FieldValue::Text(_)))),
        _=>false,
    }
}
fn text(record:&mut semio_framework_dsl_record::RecordValue,id:u16)->String{match record.fields.get_mut(&id){Some(FieldValue::Text(value))=>std::mem::take(value),_=>String::new()}}
/// 🚚️ Shape validation precedes every move, so a refusal leaves the source record intact.
pub fn take_artifact_link(source:&mut Option<FieldValue>)->Result<ArtifactLink,ValueError>{
    let Some(FieldValue::Record(record))=source.as_mut() else{return Err(invalid())};
    if record.fields.len()!=3||!matches!(record.fields.get(&0),Some(FieldValue::Record(_)))||!record.fields.get(&1).is_some_and(pin_is_well_formed)||!matches!(record.fields.get(&2),Some(FieldValue::Text(_))){return Err(invalid())}
    let target=semio_framework_artifact_reference::take_artifact_ref(record.fields.get_mut(&0).ok_or_else(invalid)?)?;
    let pin=match record.fields.get_mut(&1){
        Some(FieldValue::Record(pin))=>match pin.get(0){
            Some(FieldValue::Enum(0))=>LinkPin::Head,
            Some(FieldValue::Enum(1))=>LinkPin::Checkpoint{id:text(pin,1)},
            Some(FieldValue::Enum(2))=>{let size=match pin.get(3){Some(FieldValue::UInt(size))=>*size,_=>return Err(invalid())};LinkPin::Snapshot{blob:BlobRef{hash:text(pin,2),size,media_type:text(pin,4)}}},
            _=>return Err(invalid()),
        },
        _=>return Err(invalid()),
    };
    let role=text(record,2);
    Ok(ArtifactLink{target,pin,role})
}
fn payload(value:&FieldValue)->usize{
    fn texts(record:&semio_framework_dsl_record::RecordValue)->usize{record.fields.values().take(5).map(|field|match field{FieldValue::Text(text)=>text.len(),_=>0}).sum()}
    match value{FieldValue::Record(record)=>record.fields.values().take(3).map(|field|match field{FieldValue::Text(text)=>text.len(),FieldValue::Record(inner)=>texts(inner),_=>0}).sum(),_=>0}
}
fn convert(value:&FieldValue)->Result<ArtifactLink,ValueError>{<ArtifactLink as DslField>::from_value(value).map_err(|_|invalid())}
fn take(source:&mut Option<FieldValue>)->Result<ArtifactLink,ValueError>{take_artifact_link(source)}
semio_framework_dsl_record::dsl_owned_scalar_receiving!(move ArtifactLink,payload,convert,take);
''')
p=D+"/🦀️.rs";s=open(p,encoding="utf-8").read()
if "📥️receiving" not in s:
    s=s.rstrip("\n")+'\n#[path="📥️receiving/🦀️.rs"]\nmod receiving;\npub use receiving::take_artifact_link;\n'
    open(p,"w",encoding="utf-8").write(s)
