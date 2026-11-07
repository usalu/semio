import sys as _s
if 'go' not in _s.argv[1:]:
    raise SystemExit(0)
REF="🧰️framework/🔨️modules/🚪️io/📝️text/🗿️artifact-reference/🪆️binding/🦀️.rs"
CHILD="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🦀️.rs"
s=open(REF,encoding="utf-8").read()
if "dsl_owned_scalar_receiving" not in s:
    marker="#[cfg(test)]\n#[path=\"🧪️tests/🚦️refusals/🦀️.rs\"]\nmod refusal_tests;"
    assert s.count(marker)==1
    code='''
/// 🚚️ The four literal identity strings are moved out of their record; the emptied record shell stays with its paid owner.
pub fn take_artifact_ref(source:&mut FieldValue)->Result<ArtifactRef,ValueError>{
    let FieldValue::Record(record)=source else{return Err(semio_framework_dsl_record::receiving::owned::invalid())};
    if record.fields.len()!=4||!(0..4u16).all(|id|matches!(record.fields.get(&id),Some(FieldValue::Text(_)))){return Err(semio_framework_dsl_record::receiving::owned::invalid())}
    let mut text=|id:u16|match record.fields.get_mut(&id){Some(FieldValue::Text(value))=>std::mem::take(value),_=>String::new()};
    Ok(ArtifactRef{artifact_id:text(0),dialect:ArtifactDialect{artifact_kind:text(1),standard:text(2),subset:text(3)}})
}
fn artifact_ref_payload(value:&FieldValue)->usize{match value{FieldValue::Record(record)=>record.fields.values().take(4).map(|field|match field{FieldValue::Text(text)=>text.len(),_=>0}).sum(),_=>0}}
fn artifact_ref_convert(value:&FieldValue)->Result<ArtifactRef,ValueError>{<ArtifactRef as DslField>::from_value(value).map_err(|_|semio_framework_dsl_record::receiving::owned::invalid())}
fn artifact_ref_take(source:&mut Option<FieldValue>)->Result<ArtifactRef,ValueError>{take_artifact_ref(source.as_mut().ok_or_else(semio_framework_dsl_record::receiving::owned::invalid)?)}
semio_framework_dsl_record::dsl_owned_scalar_receiving!(move ArtifactRef,artifact_ref_payload,artifact_ref_convert,artifact_ref_take);

'''
    s=s.replace(marker,code+marker,1)
    open(REF,"w",encoding="utf-8").write(s)
import sys
c=open(CHILD,encoding="utf-8").read()
if "child" in sys.argv[1:] and "take_artifact_ref" not in c:
    anchor="pub fn artifact_child_spec() -> semio_framework_dsl_record::RecordSpec {"
    assert c.count(anchor)==1
    code='''/// 🚚️ The child id and its target identity move out of the record; the emptied shell stays with its paid owner.
pub fn take_artifact_child<S>(source:&mut Option<semio_framework_dsl_record::FieldValue>)->Result<ArtifactChild<S>,ValueError>{
    use semio_framework_dsl_record::{receiving::owned::invalid,FieldValue};
    let Some(FieldValue::Record(record))=source.as_mut() else{return Err(invalid())};
    if record.fields.len()!=2||!matches!(record.fields.get(&0),Some(FieldValue::Text(_)))||!matches!(record.fields.get(&1),Some(FieldValue::Record(_))){return Err(invalid())}
    let target=semio_framework_artifact_reference::take_artifact_ref(record.fields.get_mut(&1).ok_or_else(invalid)?)?;
    let child_id=match record.fields.get_mut(&0){Some(FieldValue::Text(value))=>std::mem::take(value),_=>return Err(invalid())};
    Ok(ArtifactChild::new(child_id,target))
}
impl<S> semio_framework_dsl_record::receiving::owned::scalar_sealed::Sealed for ArtifactChild<S>{}
impl<S:Send+'static> semio_framework_dsl_record::receiving::owned::OwnedScalar for ArtifactChild<S>{
    fn creation_bytes()->usize{0}
    fn take_source(source:&mut Option<semio_framework_dsl_record::FieldValue>)->Result<Self,ValueError>{take_artifact_child(source)}
}
impl<S:Send+'static> semio_framework_dsl_record::receiving::owned::DslOwnedReceivingField for ArtifactChild<S>{type Cursor=semio_framework_dsl_record::receiving::owned::ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{Default::default()}}
impl<S:Send+'static> semio_framework_dsl_record::DslReceivingField for ArtifactChild<S>{
    fn receive_value(value:&semio_framework_dsl_record::FieldValue,owner:&mut semio_framework_dsl_record::receiving::NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_value::RetainedWorkOutcome<Self,semio_framework_dsl_record::receiving::NativeSnapshotFailure>{
        let payload=match value{semio_framework_dsl_record::FieldValue::Record(record)=>record.fields.values().take(2).map(|field|match field{semio_framework_dsl_record::FieldValue::Text(text)=>text.len(),semio_framework_dsl_record::FieldValue::Record(target)=>target.fields.values().take(4).map(|part|match part{semio_framework_dsl_record::FieldValue::Text(text)=>text.len(),_=>0}).sum(),_=>0}).sum(),_=>0};
        semio_framework_dsl_record::receiving::receiving_turn(|meter|{meter.admit(owner.native())?;semio_framework_dsl_record::receiving::work(owner,payload.max(std::mem::size_of::<Self>()),1)?;let mut copy=Some(value.clone());let output=take_artifact_child(&mut copy)?;Ok(output)})
    }
}
'''
    c=c.replace(anchor,code+anchor,1)
    open(CHILD,"w",encoding="utf-8").write(c)

LIB="🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/🦀️.rs"
l=open(LIB,encoding="utf-8").read()
if "pub use reference_binding::take_artifact_ref" not in l:
    m='mod reference_binding;'
    assert l.count(m)==1
    l=l.replace(m,m+'\npub use reference_binding::take_artifact_ref;',1)
    open(LIB,"w",encoding="utf-8").write(l)
