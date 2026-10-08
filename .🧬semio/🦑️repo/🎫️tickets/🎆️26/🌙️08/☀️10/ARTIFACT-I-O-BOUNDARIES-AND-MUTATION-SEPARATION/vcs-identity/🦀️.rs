#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs"]
mod entity_identity;
#[cfg(test)]
mod tests {
use super::entity_identity::*;
use serde_json::Value;
use semio_framework_value::native_encoding::NativeEncodeProgress;
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};
fn bytes(value:&Value)->Vec<u8>{value.as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect()}
fn strings(value:&Value)->Vec<String>{value.as_array().unwrap().iter().map(|text|text.as_str().unwrap().to_owned()).collect()}
#[test]
fn actual_entity_io_matches_neutral_preimages_and_independent_blake3() {
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap() {
  let input=&case["input"];let prefix=case["prefix"].as_str().unwrap();let original=input.clone();
  let mut observer=|_:NativeEncodeProgress|true; let mut control=NativeEncodeControl::new(prefix.len()+17,&mut observer);
  let actual=match case["kind"].as_str().unwrap() {
   "entity"=>content_addressed_entity_id(prefix,&bytes(&input["payload"]),&mut control),
   "scoped"=>edit_scoped_id(input["editId"].as_str().unwrap(),input["ordinal"].as_u64().unwrap()as u32,&mut control),
   "edit"=>mint_edit_id(input["replica"].as_str().unwrap().parse().unwrap(),input["sequence"].as_i64().unwrap()as i32,&bytes(&input["fingerprint"]),&mut control),
   "change"=>mint_change_id(&strings(&input["editIds"]),input["description"].as_str(),&mut control),
   "alternative"=>mint_alternative_id(input["name"].as_str().unwrap(),&strings(&input["checkpointIds"]),&mut control),
   "mutation"=>{let stamp=&input["stamp"];mint_mutation_id(&bytes(&input["operation"]),(stamp[0].as_str().unwrap().parse().unwrap(),stamp[1].as_str().unwrap().parse().unwrap(),stamp[2].as_str().unwrap().parse().unwrap()),&mut control)},
   _=>panic!("unknown neutral identity kind"),
  }.unwrap();
  assert_eq!(control.owned_bytes(),prefix.len()+17);
  let hex=case["preimageHex"].as_str().unwrap();let preimage:Vec<u8>=hex.as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect();
  let digest=blake3::hash(&preimage).to_hex().to_string();let expected=format!("{prefix}-{}",&digest[..16]);assert_eq!(actual,expected,"{}",case["kind"]);assert_eq!(*input,original);
 }
 println!("[DEBUG] Actual VCS entity IO: functions=6 neutralCases={} independentBlake3=true semanticInputsPreserved=true",fixture["cases"].as_array().unwrap().len());
}

fn controlled_case(payload:&[u8],ids:&[String],control:&mut NativeEncodeControl<'_>,kind:usize)->Result<String,ValueError>{match kind{0=>content_addressed_entity_id("entity",payload,control),1=>edit_scoped_id(&ids[0],u32::MAX,control),2=>mint_edit_id(u64::MAX,i32::MIN,payload,control),3=>mint_change_id(ids,Some("description🧬️"),control),4=>mint_alternative_id("name🧬️",ids,control),_=>mint_mutation_id(payload,(u64::MAX,17,23),control)}}
#[test]
fn caller_control_cancels_each_phase_and_preserves_borrowed_inputs(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();
 let payload:Vec<u8>=(0..fixture["control"]["payloadBytes"].as_u64().unwrap()as usize).map(|n|(n%251)as u8).collect();
 let ids:Vec<String>=(0..fixture["control"]["idCount"].as_u64().unwrap()).map(|n|format!("edit-{n}-🧬️")).collect();let pointers=(payload.as_ptr(),ids.as_ptr(),ids[0].as_ptr());let original=(payload.clone(),ids.clone());
 let prefixes=["entity","scoped","edit","change","alternative","mutation"];let mut total=0;
 for(kind,prefix)in prefixes.iter().enumerate(){
  let mut events=Vec::new();let mut callback=|event:NativeEncodeProgress|{events.push(event);true};let mut control=NativeEncodeControl::new(prefix.len()+17,&mut callback);let actual=controlled_case(&payload,&ids,&mut control,kind).unwrap();assert_eq!(control.owned_bytes(),prefix.len()+17);drop(control);drop(callback);
  let mut preimage=Vec::new();match kind{0=>{preimage.extend_from_slice(b"entity\0");preimage.extend_from_slice(&payload)},1=>preimage.extend_from_slice(format!("{}:{}",ids[0],u32::MAX).as_bytes()),2=>{preimage.extend_from_slice(b"edit\0");preimage.extend_from_slice(&u64::MAX.to_le_bytes());preimage.push(0);preimage.extend_from_slice(&i32::MIN.to_le_bytes());preimage.push(0);preimage.extend_from_slice(&payload)},3=>{preimage.extend_from_slice(b"change\0");preimage.extend_from_slice(ids.join("\0").as_bytes());preimage.push(0);preimage.extend_from_slice("description🧬️".as_bytes())},4=>{preimage.extend_from_slice(b"alternative\0name");preimage.extend_from_slice("🧬️".as_bytes());preimage.push(0);preimage.extend_from_slice(ids.join("\0").as_bytes())},_=>{preimage.extend_from_slice(b"mutation\0");preimage.extend_from_slice(&payload);for n in [u64::MAX,17,23]{preimage.extend_from_slice(&n.to_le_bytes())}}};let digest=blake3::hash(&preimage).to_hex().to_string();assert_eq!(actual,format!("{prefix}-{}",&digest[..16]));
  for canceled in 0..events.len(){let mut observed=0;let mut callback=|_:NativeEncodeProgress|{let allow=observed!=canceled;observed+=1;allow};let mut control=NativeEncodeControl::new(prefix.len()+17,&mut callback);assert_eq!(controlled_case(&payload,&ids,&mut control,kind).unwrap_err().kind,ValueRefusalKind::Canceled);assert!(control.owned_bytes()<=prefix.len()+17);total+=1;}
  let mut callback=|_:NativeEncodeProgress|true;let mut control=NativeEncodeControl::new(0,&mut callback);assert_eq!(controlled_case(&payload,&ids,&mut control,kind).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.owned_bytes(),0);assert_eq!((payload.as_ptr(),ids.as_ptr(),ids[0].as_ptr()),pointers);assert_eq!((&payload,&ids),(&original.0,&original.1));
 }
 println!("[DEBUG] Actual identity control: encoders=6 cancellationBoundaries={total} payloadSpans=true idListSpans=true zeroCeilingPreservesPointers=true independentBlake3=true");
}

#[test]
fn async_authority_preserves_prior_ownership_and_refusal_receipt(){
 use super::entity_identity::control::EntityIdentityAuthority;
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["control"];let prior=law["priorOwnedBytes"].as_u64().unwrap()as usize;let ceiling=law["priorCeilingBytes"].as_u64().unwrap()as usize;let prefix=law["priorIdentityPrefix"].as_str().unwrap();
 let mut events=Vec::new();let mut observer=|event:NativeEncodeProgress|{events.push(event);true};
 let mut initial=NativeEncodeControl::new(ceiling,&mut observer);initial.charge(prior).unwrap();let receipt=initial.pause().unwrap();
 let mut authority=EntityIdentityAuthority::resume(receipt,&mut observer);let id=authority.encode(|control|content_addressed_entity_id(prefix,b"payload",control)).unwrap();assert!(id.starts_with("x-"));
 let refused:Result<String,ValueError>=authority.encode(|control|{control.charge(65)?;content_addressed_entity_id("x",b"never",control)});assert_eq!(refused.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
 let receipt=authority.pause().unwrap();let mut next_observer=|_:NativeEncodeProgress|true;let control=NativeEncodeControl::resume(receipt,&mut next_observer).unwrap();assert_eq!(control.owned_bytes(),prior+prefix.len()+17);assert_eq!(control.maximum_bytes(),ceiling);
 #[cfg(not(target_arch="wasm32"))]fn native_send<T:Send>(){}
 #[cfg(not(target_arch="wasm32"))]native_send::<EntityIdentityAuthority<'static>>();
 println!("[DEBUG] Native authoring authority: send=true priorOwnedBytes=7 exactOutputBytes=18 refusalPreservesReceipt=true");
}
}

