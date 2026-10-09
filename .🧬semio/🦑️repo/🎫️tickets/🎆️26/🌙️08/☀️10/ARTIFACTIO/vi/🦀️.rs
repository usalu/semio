#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs"]
mod entity_identity;
mod value {pub use semio_framework_value::*;}
#[path="../../../../../../../../🧰️framework/🔨️modules/📡️replication/🚪️io/💾️binary/📑️operation-sequence/🦀️.rs"]
mod operation_sequence;
#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/📥️input/🦀️.rs"]
mod operation_authority;
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

#[test]
fn streamed_authoring_preimages_and_native_operation_framing_match_independent_oracles(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["stream"];
 let payload:Vec<u8>=(0..law["payloadBytes"].as_u64().unwrap()as usize).map(|n|(n%251)as u8).collect();let children:Vec<(String,Vec<u8>)>=law["children"].as_array().unwrap().iter().map(|child|(child[0].as_str().unwrap().to_owned(),bytes(&child[1]))).collect();
 let parent=law["parent"].as_str().unwrap();let slot=law["slot"].as_str().unwrap();let message=law["message"].as_str().unwrap();let ordinal=law["ordinal"].as_u64().unwrap()as u32;
 let ops:Vec<Vec<u8>>=law["operationLengths"].as_array().unwrap().iter().map(|n|payload[..n.as_u64().unwrap()as usize].to_vec()).collect();let ids:Vec<String>=children.iter().map(|child|child.0.clone()).collect();
 let mut fingerprint=Vec::new();for op in &ops{fingerprint.extend_from_slice(&(op.len()as u64).to_le_bytes());fingerprint.extend_from_slice(op);}
 let mut preimages=Vec::new();let mut child=b"child\0".to_vec();for part in[parent.as_bytes(),&[0],slot.as_bytes(),&[0],&payload,&[0],&ordinal.to_le_bytes()]{child.extend_from_slice(part);}preimages.push(("child",child));
 let mut invocation=b"invocation\0".to_vec();invocation.extend_from_slice(parent.as_bytes());invocation.push(0);invocation.extend_from_slice(&payload);let mut ordered=children.clone();ordered.sort_by(|left,right|left.0.cmp(&right.0));for(id,body)in ordered{invocation.push(0);invocation.extend_from_slice(id.as_bytes());invocation.push(0);invocation.extend_from_slice(&body);}preimages.push(("invocation",invocation));
 let mut checkpoint=b"space-checkpoint\0".to_vec();checkpoint.extend_from_slice(message.as_bytes());checkpoint.push(0);checkpoint.extend_from_slice(&payload);preimages.push(("space-checkpoint",checkpoint));
 let mut alternative=b"space-alternative\0".to_vec();alternative.extend_from_slice(message.as_bytes());alternative.push(0);alternative.extend_from_slice(ids.join("\0").as_bytes());preimages.push(("space-alternative",alternative));
 let run=|kind:usize,control:&mut NativeEncodeControl<'_>|match kind{0=>mint_child_id(parent,slot,&payload,ordinal,control),1=>mint_invocation_id(parent,&payload,&children,control),2=>mint_space_checkpoint_id(message,&payload,control),_=>mint_space_alternative_id(message,&ids,control)};
 let mut cancellation_boundaries=0;for(kind,(prefix,preimage))in preimages.iter().enumerate(){let mut events=0;let mut observer=|_:NativeEncodeProgress|{events+=1;true};let mut control=NativeEncodeControl::new(4096,&mut observer);let actual=run(kind,&mut control).unwrap();drop(control);drop(observer);let digest=blake3::hash(preimage).to_hex().to_string();assert_eq!(actual,format!("{prefix}-{}",&digest[..16]));for boundary in 0..events{let mut observed=0;let mut observer=|_:NativeEncodeProgress|{let allow=observed!=boundary;observed+=1;allow};let mut control=NativeEncodeControl::new(4096,&mut observer);assert_eq!(run(kind,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);cancellation_boundaries+=1;}}
 let mut observer=|_:NativeEncodeProgress|true;let mut control=NativeEncodeControl::new(fingerprint.len(),&mut observer);assert_eq!(operation_list_fingerprint(&ops,&mut control).unwrap(),fingerprint);assert_eq!(control.owned_bytes(),fingerprint.len());
 fn varint(mut n:usize,out:&mut Vec<u8>){loop{let byte=(n&127)as u8;n>>=7;out.push(byte|if n!=0{128}else{0});if n==0{return}}}
 let mut framed=Vec::new();varint(ops.len(),&mut framed);for op in &ops{varint(op.len(),&mut framed);framed.extend_from_slice(op);}let mut library_framed=Vec::new();leb128::write::unsigned(&mut library_framed,ops.len()as u64).unwrap();for op in &ops{leb128::write::unsigned(&mut library_framed,op.len()as u64).unwrap();library_framed.extend_from_slice(op);}assert_eq!(framed,library_framed);let mut observer=|_:NativeEncodeProgress|true;let mut control=NativeEncodeControl::new(framed.len(),&mut observer);assert_eq!(super::operation_sequence::encode(&ops,&mut control).unwrap(),framed);assert_eq!(control.owned_bytes(),framed.len());
 let mut observer=|_:NativeEncodeProgress|true;let mut control=NativeEncodeControl::new(0,&mut observer);assert_eq!(super::operation_sequence::encode(&ops,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
 let stamp=(u64::MAX,17,23);let mut observer=|_:NativeEncodeProgress|true;let mut control=NativeEncodeControl::new(25,&mut observer);let mut stream=NativeIdentityPreimage::new("mutation",&mut control).unwrap();for span in payload.chunks(64){stream.write(span,&mut control).unwrap();}for n in[stamp.0,stamp.1,stamp.2]{stream.write(&n.to_le_bytes(),&mut control).unwrap();}let actual=stream.finish(&mut control).unwrap();let mut observer=|_:NativeEncodeProgress|true;let mut control=NativeEncodeControl::new(25,&mut observer);assert_eq!(actual,mint_mutation_id(&payload,stamp,&mut control).unwrap());
 println!("[DEBUG] Native receiving preimages: streamedMutation=true compositionKinds=4 stableDuplicateChildren=true canceledBoundaries={cancellation_boundaries} operationFramingVarints=true exactOwnership=true independentBlake3=true independentLeb128=true");
}
#[test]
fn empty_duplicate_child_ordering_observes_comparison_work_and_original_authority(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();let count=fixture["stream"]["emptyChildCount"].as_u64().unwrap()as usize;let children:Vec<(String,Vec<u8>)>=(0..count).map(|n|(String::new(),vec![(n%251)as u8])).collect();let pointer=children.as_ptr();let owned=count*std::mem::size_of::<(usize,&(String,Vec<u8>))>()+"invocation".len()+17;
 let mut events=Vec::new();let mut observer=|progress:NativeEncodeProgress|{events.push(progress);true};let mut control=NativeEncodeControl::new(owned,&mut observer);let actual=mint_invocation_id("parent",b"fingerprint",&children,&mut control).unwrap();assert_eq!(control.owned_bytes(),owned);drop(control);drop(observer);
 let mut preimage=b"invocation\0parent\0fingerprint".to_vec();for(_,body)in &children{preimage.extend_from_slice(&[0,0]);preimage.extend_from_slice(body);}let digest=blake3::hash(&preimage).to_hex().to_string();assert_eq!(actual,format!("invocation-{}",&digest[..16]));let comparisons=events.iter().filter(|progress|progress.total==0&&progress.completed>count).count();assert!(comparisons>2);
 for canceled in 0..events.len(){let mut observed=0;let mut observer=|_:NativeEncodeProgress|{let allowed=observed!=canceled;observed+=1;allowed};let mut control=NativeEncodeControl::new(owned,&mut observer);assert_eq!(mint_invocation_id("parent",b"fingerprint",&children,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(children.as_ptr(),pointer);for(index,(_,body))in children.iter().enumerate(){assert_eq!(body,&vec![(index%251)as u8]);}}
 println!("[DEBUG] Empty duplicate child sorting: children={count} comparisonCheckpoints={comparisons} canceledBoundaries={} stableOriginalOrder=true exactScratchOwnership=true independentBlake3=true",events.len());
}

#[test]
fn refused_stage_preserves_the_original_authority_and_owned_receipt(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();
 let law=&fixture["control"];let stage=&law["stageRefusal"];let prior=law["priorOwnedBytes"].as_u64().unwrap()as usize;let ceiling=law["priorCeilingBytes"].as_u64().unwrap()as usize;let charge=stage["charge"].as_u64().unwrap()as usize;
 let mut progress=NativeEncodeProgress{completed:0,total:0,owned_bytes:0};let mut observer=|next|{progress=next;true};
 let mut control=NativeEncodeControl::new(ceiling,&mut observer);control.charge(prior).unwrap();let receipt=control.pause().unwrap();
 let mut authority=control::EntityIdentityAuthority::resume(receipt,&mut observer);
 let result:Result<(),ValueError>=authority.encode(|control|{control.charge(charge)?;control.begin_stage(stage["declared"].as_u64().unwrap()as usize)?;control.advance(stage["attempted"].as_u64().unwrap()as usize)});
 assert_eq!(result.unwrap_err().kind,ValueRefusalKind::WorkLimit);
 let receipt=authority.pause().expect("refused work retains a valid continuation");
 let mut control=NativeEncodeControl::resume(receipt,&mut observer).unwrap();assert_eq!(control.owned_bytes(),prior+charge);
 let expected=format!("x-{}",&blake3::hash(b"x\0payload").to_hex()[..16]);let actual=content_addressed_entity_id("x",b"payload",&mut control).unwrap();assert_eq!(actual,expected);assert_eq!(control.owned_bytes(),prior+charge+18);
 println!("[DEBUG] Refused identity stage retains prior={} newCharge={} exactNext=18 independentBlake3=true",prior,charge);
}
#[test]
fn draft_metadata_identity_matches_original_preimage_with_bounded_admission(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();let row=&fixture["stream"]["draft"];
 let kind=row["kind"].as_str().unwrap();let schema=row["schema"].as_str().unwrap();let name=row["name"].as_str().unwrap();let now:u64=row["now"].as_str().unwrap().parse().unwrap();let sequence:u64=row["sequence"].as_str().unwrap().parse().unwrap();
 let preimage=format!("draft\0{kind}\0{schema}\0{name}\0{now}\0{sequence}");let expected=format!("draft-{}",&blake3::hash(preimage.as_bytes()).to_hex()[..16]);let mut callbacks=0;let mut observe=|_:NativeEncodeProgress|{callbacks+=1;true};let mut control=NativeEncodeControl::new(22,&mut observe);assert_eq!(mint_draft_id(kind,schema,name,now,sequence,&mut control).unwrap(),expected);assert_eq!(control.owned_bytes(),22);drop(control);drop(observe);
 for stop in 1..=callbacks{let mut seen=0;let mut cancel=|_:NativeEncodeProgress|{seen+=1;seen!=stop};let mut control=NativeEncodeControl::new(22,&mut cancel);assert_eq!(mint_draft_id(kind,schema,name,now,sequence,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);}
 let mut allow=|_:NativeEncodeProgress|true;let mut control=NativeEncodeControl::new(0,&mut allow);assert_eq!(mint_draft_id(kind,schema,name,now,sequence,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.owned_bytes(),0);
 println!("[DEBUG] Draft identity: boundedDecimalU64=true originalPreimage=true exactOutputBytes=22 canceledBoundaries={} independentBlake3=true",callbacks);
}
#[test]
fn forwarded_entity_authority_preserves_original_preallocation_port_through_hops(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["foreign"];let n=|key:&str|law[key].as_u64().unwrap()as usize;
 let canceled=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));let flag=canceled.clone();let mut host_observer=move|_:NativeEncodeProgress|!flag.load(std::sync::atomic::Ordering::Relaxed);
 let mut host_control=NativeEncodeControl::new(n("maximumBytes"),&mut host_observer);host_control.charge(n("priorOwnedBytes")).unwrap();let receipt=host_control.pause().unwrap();let host=std::sync::Mutex::new(control::EntityIdentityAuthority::resume(receipt,&mut host_observer));
 let mut observer=|next:NativeEncodeProgress|host.lock().unwrap().encode(|control|{control.begin_stage(next.total)?;control.advance(next.completed)}).is_ok();
 let mut calls=0;let mut allocate=|request:semio_framework_value::native_encoding::NativeEncodeAllocation|{calls+=1;host.lock().unwrap().encode(|control|control.charge(request.bytes))};
 let mut guest=control::EntityIdentityAuthority::new_forwarded(n("guestMaximumBytes"),&mut observer,&mut allocate).unwrap();
 fn requires_send<T:Send>(_:&T){}requires_send(&guest);
 let first=guest.encode(|control|mint_draft_id("s.space","schema","draft",17,23,control)).unwrap();let digest=blake3::hash(b"draft\0s.space\0schema\0draft\017\023").to_hex().to_string();assert_eq!(first,format!("draft-{}",&digest[..16]));
 let refused:Result<(),ValueError>=guest.encode(|control|{control.charge(n("failedStageCharge"))?;control.begin_stage(n("failedStageDeclared"))?;control.advance(n("failedStageAttempted"))});assert_eq!(refused.unwrap_err().kind,ValueRefusalKind::WorkLimit);
 canceled.store(true,std::sync::atomic::Ordering::Relaxed);let refusal=guest.encode(|control|content_addressed_entity_id("x",b"payload",control));assert_eq!(refusal.unwrap_err().kind,ValueRefusalKind::Canceled);
 canceled.store(false,std::sync::atomic::Ordering::Relaxed);let expected=format!("x-{}",&blake3::hash(b"x\0payload").to_hex()[..16]);assert_eq!(guest.encode(|control|content_addressed_entity_id("x",b"payload",control)).unwrap(),expected);
 let guest_receipt=guest.pause_forwarded().unwrap();assert_eq!(guest_receipt.owned_bytes(),n("finalGuestOwnedBytes"));drop(guest_receipt);drop(observer);drop(allocate);assert_eq!(calls,3);
 let host_receipt=host.into_inner().unwrap().pause().unwrap();let mut observer=|_:NativeEncodeProgress|true;let host_control=NativeEncodeControl::resume(host_receipt,&mut observer).unwrap();assert_eq!(host_control.owned_bytes(),n("finalHostOwnedBytes"));
 println!("[DEBUG] Forwarded entity authority: originalPort=true originalObserver=true nativeSend=true stageFailureReceiptPreserved=true originalCanceledHash=true guestOwned=43 hostPrior=7 hostOwned=50 independentBlake3=true");
}
#[test]
fn original_receiver_rejects_forged_ledgers_before_original_admission(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();let n=|key:&str|fixture["foreign"][key].as_u64().unwrap()as usize;
 let mut observer=|_:NativeEncodeProgress|true;let mut host=control::EntityIdentityAuthority::new(n("maximumBytes"),&mut observer).unwrap();host.encode(|c|c.charge(n("priorOwnedBytes"))).unwrap();
 let mut receiver=control::OriginalOperationReceiver::new(&mut host);assert_eq!(receiver.receive_output(n("nextIdentityBytes")).unwrap_err().kind,ValueRefusalKind::InvariantViolated);assert_eq!(receiver.begin().unwrap(),n("guestMaximumBytes"));assert_eq!(receiver.begin().unwrap_err().kind,ValueRefusalKind::InvariantViolated);
 let request=semio_framework_value::native_encoding::NativeEncodeAllocation{bytes:n("firstIdentityBytes"),owned_bytes:0,next_owned_bytes:n("firstIdentityBytes"),maximum_bytes:n("firstIdentityBytes")};
 assert_eq!(receiver.allocation(semio_framework_value::native_encoding::NativeEncodeAllocation{owned_bytes:1,..request}).unwrap_err().kind,ValueRefusalKind::InvariantViolated);receiver.allocation(request).unwrap();
 assert_eq!(receiver.finish(0).unwrap_err().kind,ValueRefusalKind::InvariantViolated);receiver.finish(n("firstIdentityBytes")).unwrap();assert_eq!(receiver.allocation(request).unwrap_err().kind,ValueRefusalKind::InvariantViolated);receiver.receive_output(n("nextIdentityBytes")).unwrap();drop(receiver);
 assert_eq!(host.encode(|c|Ok::<_,ValueError>(c.owned_bytes())).unwrap(),n("priorOwnedBytes")+n("firstIdentityBytes")+n("nextIdentityBytes"));
 println!("[DEBUG] Original receiving ledger: duplicateBeginRefused=true forgedPreviousRefused=true exactFinishRequired=true postFinishRefused=true admittedOwnershipPreserved=true");
}
#[test]
fn original_receiver_serial_phases_share_original_cancellation_and_cumulative_ownership(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();let n=|key:&str|fixture["foreign"][key].as_u64().unwrap()as usize;
 let canceled=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));let flag=canceled.clone();let mut observer=move|_:NativeEncodeProgress|!flag.load(std::sync::atomic::Ordering::Relaxed);let mut host=control::EntityIdentityAuthority::new(n("maximumBytes"),&mut observer).unwrap();host.encode(|c|c.charge(n("priorOwnedBytes"))).unwrap();
 let mut receiver=control::OriginalOperationReceiver::new(&mut host);assert_eq!(receiver.begin().unwrap(),n("guestMaximumBytes"));receiver.allocation(semio_framework_value::native_encoding::NativeEncodeAllocation{bytes:n("firstIdentityBytes"),owned_bytes:0,next_owned_bytes:n("firstIdentityBytes"),maximum_bytes:n("guestMaximumBytes")}).unwrap();receiver.finish(n("firstIdentityBytes")).unwrap();
 let remaining=n("guestMaximumBytes")-n("firstIdentityBytes");assert_eq!(receiver.begin().unwrap(),remaining);assert_eq!(receiver.progress(NativeEncodeProgress{completed:0,total:1,owned_bytes:n("firstIdentityBytes")}).unwrap_err().kind,ValueRefusalKind::InvariantViolated);receiver.progress(NativeEncodeProgress{completed:1,total:1,owned_bytes:0}).unwrap();
 canceled.store(true,std::sync::atomic::Ordering::Relaxed);assert_eq!(receiver.allocation(semio_framework_value::native_encoding::NativeEncodeAllocation{bytes:n("failedStageCharge"),owned_bytes:0,next_owned_bytes:n("failedStageCharge"),maximum_bytes:remaining}).unwrap_err().kind,ValueRefusalKind::Canceled);canceled.store(false,std::sync::atomic::Ordering::Relaxed);
 receiver.allocation(semio_framework_value::native_encoding::NativeEncodeAllocation{bytes:n("failedStageCharge"),owned_bytes:0,next_owned_bytes:n("failedStageCharge"),maximum_bytes:remaining}).unwrap();receiver.finish(n("failedStageCharge")).unwrap();drop(receiver);assert_eq!(host.encode(|c|Ok::<_,ValueError>(c.owned_bytes())).unwrap(),n("priorOwnedBytes")+n("firstIdentityBytes")+n("failedStageCharge"));
 println!("[DEBUG] Original receiving phases: guestLedgers=22,3 hostPrior=7 cumulativeHost=32 originalCanceledPreallocation=true forgedProgressRefused=true noReplacementCeiling=true");
}
#[test]
fn owned_input_json_is_admitted_before_reservation_and_matches_independent_serde(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();let input=&fixture["inputAdmission"]["value"];let expected=serde_json::to_vec(input).unwrap();let mut callbacks=0;let mut observer=|_:NativeEncodeProgress|{callbacks+=1;true};let mut control=NativeEncodeControl::new(expected.len(),&mut observer);let actual=super::operation_authority::encode_json_input(input,&mut control).unwrap();assert_eq!(actual,expected);assert_eq!(control.owned_bytes(),expected.len());drop(control);drop(observer);
 for stop in 1..=callbacks{let mut count=0;let mut observer=|_:NativeEncodeProgress|{count+=1;count!=stop};let mut control=NativeEncodeControl::new(expected.len(),&mut observer);assert_eq!(super::operation_authority::encode_json_input(input,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);}
 let mut observer=|_:NativeEncodeProgress|true;let mut control=NativeEncodeControl::new(0,&mut observer);assert_eq!(super::operation_authority::encode_json_input(input,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.owned_bytes(),0);
 println!("[DEBUG] Owned input admission: borrowedSerde=true measuredBeforeAllocation=true exactReservation=true zeroCeilingOwned=0 canceledBoundaries={} independentSerde=true",callbacks);
}
#[test]
fn original_receiving_scalar_hop_retains_guest_ledger_and_pre_return_reservation(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();let n=|key:&str|fixture["foreign"][key].as_u64().unwrap()as usize;let mut observer=|_:NativeEncodeProgress|true;let mut host=control::EntityIdentityAuthority::new(n("maximumBytes"),&mut observer).unwrap();host.encode(|c|c.charge(n("priorOwnedBytes"))).unwrap();
 let r=|key:&str|fixture["returnReservation"][key].as_u64().unwrap()as usize;let owned=r("guestOwnedBytes");let returned=r("hostReturnBytes");let mut receiver=control::OriginalOperationReceiver::new(&mut host);receiver.ensure_finished().unwrap();receiver.begin().unwrap();receiver.allocation(semio_framework_value::native_encoding::NativeEncodeAllocation{bytes:owned,owned_bytes:0,next_owned_bytes:owned,maximum_bytes:n("guestMaximumBytes")}).unwrap();assert_eq!(receiver.ensure_finished().unwrap_err().kind,ValueRefusalKind::InvariantViolated);let receipt=receiver.pause();
 let mut receiver=control::OriginalOperationReceiver::resume(receipt,&mut host).unwrap_or_else(|(error,_)|panic!("{error}"));receiver.progress(NativeEncodeProgress{completed:1,total:1,owned_bytes:owned}).unwrap();receiver.reserve_return(returned).unwrap();receiver.finish(owned).unwrap();receiver.ensure_finished().unwrap();assert_eq!(receiver.reserved_return_bytes(),returned);receiver.verify_reserved_return(returned).unwrap();assert_eq!(receiver.verify_reserved_return(r("wrongHostReturnBytes")).unwrap_err().kind,ValueRefusalKind::InvariantViolated);drop(receiver);assert_eq!(host.encode(|c|Ok::<_,ValueError>(c.owned_bytes())).unwrap(),r("finalHostOwnedBytes"));
 println!("[DEBUG] Original receiving scalar hop: sameOriginalHost=true guestLedgerPreserved=3 preReturnReserved=11 hostOwned=21 unfinishedRefused=true noPhaseAuthoringAllowed=true exactReservedReturn=true");
}

#[test]
fn original_receiving_restored_scalar_refuses_a_different_host_ledger(){
 let mut observer=|_:NativeEncodeProgress|true;let mut host=control::EntityIdentityAuthority::new(64,&mut observer).unwrap();host.encode(|c|c.charge(7)).unwrap();let mut receiver=control::OriginalOperationReceiver::new(&mut host);receiver.begin().unwrap();receiver.allocation(semio_framework_value::native_encoding::NativeEncodeAllocation{bytes:3,owned_bytes:0,next_owned_bytes:3,maximum_bytes:57}).unwrap();let receipt=receiver.pause();let encoded=serde_json::to_vec(&receipt).unwrap();let receipt:control::OriginalOperationReceipt=serde_json::from_slice(&encoded).unwrap();host.encode(|c|c.charge(1)).unwrap();let (error,receipt)=match control::OriginalOperationReceiver::resume(receipt,&mut host){Ok(_)=>panic!("stale scalar accepted different original host ledger"),Err(refusal)=>refusal};assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);assert_eq!(serde_json::to_vec(&receipt).unwrap(),encoded);assert_eq!(host.encode(|c|Ok::<_,ValueError>(c.owned_bytes())).unwrap(),11);println!("[DEBUG] Original scalar restoration: staleHostLedgerRefused=true actualReceiptRetained=true noAuthorityMint=true");
}

#[test]
fn original_scalar_contract_matches_independent_serde_and_firstparty_restoration(){
 use semio_framework_value::{ToValue,FromValue,DslValue};let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();let value=|case:&Value|DslValue::object(case.as_object().unwrap().iter().map(|(name,value)|(name.clone(),match value{Value::Null=>DslValue::Null,Value::Bool(value)=>value.to_value(),Value::Number(value)=>value.as_u64().unwrap().to_value(),_=>panic!("unexpected scalar contract field")})));for case in fixture["scalarReceipts"]["accepted"].as_array().unwrap(){let receipt:control::OriginalOperationReceipt=serde_json::from_value(case.clone()).unwrap();let typed=control::OriginalOperationReceipt::from_value(value(case)).unwrap();assert_eq!(serde_json::to_value(&receipt).unwrap(),*case);assert_eq!(serde_json::to_value(&typed).unwrap(),*case);let restored=control::OriginalOperationReceipt::from_value(receipt.to_value()).unwrap();assert_eq!(serde_json::to_value(&restored).unwrap(),*case);}for case in fixture["scalarReceipts"]["semanticRefused"].as_array().unwrap(){assert!(serde_json::from_value::<control::OriginalOperationReceipt>(case.clone()).is_err());assert!(control::OriginalOperationReceipt::from_value(value(case)).is_err());}assert!(serde_json::from_value::<control::OriginalOperationReceipt>(fixture["scalarReceipts"]["missingMaximum"].clone()).is_err());assert!(control::OriginalOperationReceipt::from_value(value(&fixture["scalarReceipts"]["missingMaximum"])).is_err());println!("[DEBUG] Original scalar domain: firstpartyRestoration=true independentSerde=true closedSchema=true accepted=5 semanticRefused=3");
}

}

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🧬️schema/⚠️diagnostic/🦀️.rs"]
mod schema_diagnostic;
#[test]
fn original_close_diagnostic_retains_real_progress_and_kind(){let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🧬️schema/⚠️diagnostic/🧫️fixtures/🔣️.json")).unwrap();let r=&fixture["retainedProgress"];let progress=semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:r["copiedItems"].as_u64().unwrap()as usize,copied_bytes:r["copiedBytes"].as_u64().unwrap()as usize,retained_capacity_bytes:r["retainedCapacityBytes"].as_u64().unwrap()as usize,released_bytes:r["releasedBytes"].as_u64().unwrap()as usize};let actual=semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original physical turn refused").with_retained_progress(progress);let diagnostic=schema_diagnostic::SchemaDecodeDiagnostic::before("artifact-envelope.target-retirement",fixture["path"].as_str().unwrap()).with_native(actual);assert_eq!(diagnostic.retained_progress,progress);assert_eq!(diagnostic.refusal_kind.as_str(),fixture["refusalKind"].as_str().unwrap());let independently=serde_json::json!({"copiedItems":diagnostic.retained_progress.copied_items,"copiedBytes":diagnostic.retained_progress.copied_bytes,"retainedCapacityBytes":diagnostic.retained_progress.retained_capacity_bytes,"releasedBytes":diagnostic.retained_progress.released_bytes});assert_eq!(independently,*r);println!("[DEBUG] Original schema close diagnostic: physicalProgressPreserved=true typedRefusalPreserved=true independentSerde=true");}

#[test]
fn entity_identity_retains_native_original_retirement_receipt_and_observer(){
 use serde_json::Value;use crate::entity_identity::control;
 use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retirement::controlled::ControlledRetirement,native_encoding::NativeEncodeRetirementRecipient};
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["retainedIdentity"];let grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();let text=law["text"].as_str().unwrap();let initial=law["initialBytes"].as_u64().unwrap()as usize;let maximum=law["maximumBytes"].as_u64().unwrap()as usize;let mut recipient=NativeEncodeRetirementRecipient::new();let canceled=std::cell::Cell::new(false);let calls=std::cell::Cell::new(0);let mut original=|_|{calls.set(calls.get()+1);!canceled.get()};let mut native=NativeEncodeControl::new(maximum,&mut original);native.charge(initial).unwrap();let receipt=native.pause().unwrap().with_retirement_recipient(&mut recipient).unwrap_or_else(|_|panic!("original recipient refused"));let mut identity=control::EntityIdentityAuthority::resume_retirement(receipt,&mut original);let wrapper=std::mem::size_of::<ControlledRetirement<String>>();
 let error=identity.encode(|native|native.with_retirement_owner(wrapper,|native|match native.copy_text(text){Err(error)=>(Err::<(),_>(error),None),Ok(owned)=>{assert_eq!(serde_json::to_value(&owned).unwrap(),serde_json::to_value(text).unwrap());(Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original retained refusal")),Some(Box::new(ControlledRetirement::new(owned).ok().unwrap())as Box<dyn ErasedSnapshotRetirement>))}})).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvalidValue);canceled.set(true);assert_eq!(identity.encode(|native|native.close_retirement_recipient(grant)).unwrap_err().kind,ValueRefusalKind::Canceled);canceled.set(false);identity.encode(|native|{while native.has_retirement_owner(){native.close_retirement_recipient(grant)?;}Ok::<_,ValueError>(())}).unwrap();let receipt=identity.pause_retirement().unwrap();assert_eq!(receipt.maximum_bytes(),maximum);assert!(receipt.owned_bytes()>=initial+wrapper+text.len());drop(receipt);assert!(!recipient.has_owner());assert!(calls.get()>1);println!("[DEBUG] Entity original native identity: sameObserver=true consumingReceipt=true originalRecipient=true refusalOwnerDrained=true independentSerde=true");
}
