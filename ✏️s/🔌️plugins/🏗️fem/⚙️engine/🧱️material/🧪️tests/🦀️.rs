use super::FemMaterial;
use super::FemMaterialValue;
use super::NATIVE_MATERIAL_DIMENSION;
#[path="../📋️table/🧪️tests/🦀️.rs"]mod table;
#[path="../🩹️patch/🧪️tests/🦀️.rs"]mod patch;
use semio_framework_value::{DslValue, FromValue, ToValue, RetirementDemand, retirement::{RetireOwned, controlled::ControlledRetirement}, retained_clone::{RetainedCloneSource, RetainedCloneGrant}};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeReceive, native_decoding::owned::NativeRecordSource, native_decoding::owned::NativeReceivingOriginal};
fn funded(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
#[derive(Default)]
struct Recipient;
impl semio_framework_value_native_owned::native_decoding::NativeDecodeConstructionRecipient for Recipient{
 fn grant(&mut self,_:RetirementDemand)->Result<RetainedCloneGrant,semio_framework_value::ValueError>{Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64})}
 fn accept(&mut self,g:RetainedCloneGrant,p:semio_framework_value::retained_clone::RetainedCloneProgress)->Result<(),semio_framework_value::ValueError>{assert!(p.fits(g));Ok(())}
}
fn drain<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("material typed original retirement"));for _ in 0..10000{if owner.terminal_is_empty(){return;}let g=funded(owner.next_demand(4096).unwrap());assert!(owner.step(g).unwrap().progress().fits(g));}panic!("material typed original close stalled");}
fn receive<I:NativeRecordSource+RetireOwned>(input:impl Fn()->I,expected:&serde_json::Value){
 type Material<I>=NativeReceivingOriginal<I,FemMaterial>;let mut finished=0;
 for cut in 0..192{let allocation=RetainedCloneSource::<I>::owned_constructor_demand::<()>();let mut source=Some(RetainedCloneSource::admit_owned(input(),(),funded(RetirementDemand{copy_bytes:RetainedCloneSource::<I>::constructor_copy_bytes(),capacity_bytes:allocation.capacity_bytes,depth:allocation.depth,..Default::default()})).unwrap_or_else(|_|panic!("material original input admission")).0);let(mut owner,_)=Material::<I>::begin_shell(funded(Material::<I>::shell_demand())).unwrap();owner.admit_original(&mut source,funded(owner.original_capture_demand().unwrap())).unwrap();owner.begin_receiver(funded(owner.receiver_birth_demand().unwrap())).unwrap();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut recipient=Recipient;let mut turns=0;
  while turns<cut&&!owner.output_ready(){let d=owner.demands(4096).unwrap();let g=funded(d);let mut denied=g;denied.maximum_items=0;assert_eq!(control.with_construction_recipient(&mut recipient,|control|owner.advance(control,4096,denied)).unwrap().progress(),Default::default());let step=control.with_construction_recipient(&mut recipient,|control|owner.advance(control,4096,g)).unwrap();assert!(step.progress().fits(g));turns+=1;}
  if owner.output_ready(){finished=turns;let g=funded(owner.take_demand().unwrap());let(material,p)=owner.take(g).unwrap().unwrap();assert!(p.fits(g));assert_eq!(material.id.as_str(),expected["id"].as_str().unwrap());assert_eq!(material.name.as_str(),expected["name"].as_str().unwrap());let value=material.to_value();for key in ["e","g","nu","rho"]{if expected.get(key).is_some(){assert_eq!(value.get(key).unwrap().as_f64().unwrap().to_bits(),expected[key].as_f64().unwrap().to_bits());}}drain(value);drain(material);}
  drain(owner);if finished>0&&cut>finished{break;}
 }assert!(finished>0);eprintln!("[DEBUG] native material dimension={NATIVE_MATERIAL_DIMENSION} input={} every-prefix normalTurns={finished}",std::any::type_name::<I>());
}
#[test]
fn native_material_receives_canonical_and_authored_original_values(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let row=&law["accepted"].as_array().unwrap().iter().find(|case|case["dimension"].as_u64()==Some(NATIVE_MATERIAL_DIMENSION as u64)).unwrap()["row"];
 receive(||DslValue::from(row),row);receive(||FemMaterialValue::from_value(DslValue::from(row)).unwrap(),row);
}
#[test]
fn native_material_receives_actual_declared_text_ordinals(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let row=&law["accepted"].as_array().unwrap().iter().find(|case|case["dimension"].as_u64()==Some(NATIVE_MATERIAL_DIMENSION as u64)).unwrap()["row"];
 receive(||{let mut record=semio_framework_dsl_record::RecordValue::default();let keys: &[&str]=if NATIVE_MATERIAL_DIMENSION==2{&["id","name","e","nu","rho"]}else{&["id","name","e","g","nu","rho"]};for(ordinal,key)in keys.iter().enumerate(){record.fields.insert(u16::try_from(ordinal).unwrap(),if let Some(text)=row[key].as_str(){semio_framework_dsl_record::FieldValue::Text(text.to_owned())}else{semio_framework_dsl_record::FieldValue::Float(row[key].as_f64().unwrap())});}record},row);
}
#[test]
fn native_material_refuses_invalid_parts_without_dropping_partial_fields(){
 type Owner=NativeReceivingOriginal<DslValue,FemMaterial>;let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let mut refused=0;
 for case in law["refused"].as_array().unwrap().iter().filter(|case|case["dimension"].as_u64()==Some(NATIVE_MATERIAL_DIMENSION as u64)){
  let a=RetainedCloneSource::<DslValue>::owned_constructor_demand::<()>();let mut source=Some(RetainedCloneSource::admit_owned(DslValue::from(&case["row"]),(),funded(RetirementDemand{copy_bytes:RetainedCloneSource::<DslValue>::constructor_copy_bytes(),capacity_bytes:a.capacity_bytes,depth:a.depth,..Default::default()})).unwrap_or_else(|_|panic!("material refused original input")).0);let(mut owner,_)=Owner::begin_shell(funded(Owner::shell_demand())).unwrap();owner.admit_original(&mut source,funded(owner.original_capture_demand().unwrap())).unwrap();owner.begin_receiver(funded(owner.receiver_birth_demand().unwrap())).unwrap();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut recipient=Recipient;let mut found=false;
  for _ in 0..192{let g=funded(owner.demands(4096).unwrap());match control.with_construction_recipient(&mut recipient,|control|owner.advance(control,4096,g)){Ok(step)=>assert!(step.progress().fits(g)),Err(error)=>{assert!(error.retained_progress().fits(g));found=true;break;}}assert!(!owner.output_ready());}assert!(found);assert!(owner.borrow_input().is_ok());drain(owner);refused+=1;
 }assert!(refused>0);eprintln!("[DEBUG] native material dimension={NATIVE_MATERIAL_DIMENSION} invalid field vectors={refused}, original and partial owned fields retained through refusal");
}
