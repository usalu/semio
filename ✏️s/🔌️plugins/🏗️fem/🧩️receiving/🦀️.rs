//! 🫴️ Retained generic FEM row and collection receiving with caller-owned publication slots.
use semio_framework_dsl_record::{FieldValue,RecordValue,receiving::{DslReceivingField,NativeSnapshotDecodeOwner,NativeSnapshotFailure,NativeSnapshotEffectProgress,NativeSnapshotDecodeEffectQuote,NativeSnapshotDecodeWorkQuote,ReceivingWork,receiving_turn}};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind,retirement::RetireOwned};
use std::mem::size_of;

/// 🎯️ Performs one exact retained metadata transfer and posts its actual physical and work receipts.
fn transfer(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,bytes:usize,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork,mut effect:impl FnMut())->Result<(),NativeSnapshotFailure>{
 let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};
 let turn=owner.effect(demand,progress,unwind,&mut |performed,physical|{effect();*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});
 meter.take(turn)?;progress.reset_completed()?;Ok(())
}
/// 📝️ Moves an admitted string into an empty retained field.
pub fn move_string(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,target:&mut String,source:&mut Option<String>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 if target.capacity()!=0||source.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"FEM string field requires an empty destination and retained source").into())}
 transfer(owner,0,progress,unwind,meter,||*target=source.take().unwrap())
}
/// 👁️ Runs one bounded, non-mutating inspection under the original work reservation.
pub fn inspect<T:Copy>(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork,mut effect:impl FnMut()->Result<T,ValueError>)->Result<T,NativeSnapshotFailure>{
 let mut output=None;let quote=NativeSnapshotDecodeWorkQuote::reserve(owner.native())?;let turn=quote.run(&mut progress.performed,unwind,&mut |performed|{*performed=true;output=Some(effect()?);Ok(())});meter.take(turn)?;progress.reset_completed()?;output.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"FEM inspection did not retain its result").into())
}
/// 🔢️ Validates one numeric field using its original work reservation.
pub fn finite(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,value:f64,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 let quote=NativeSnapshotDecodeWorkQuote::reserve(owner.native())?;let turn=quote.run(&mut progress.performed,unwind,&mut |performed|{let valid=value.is_finite();*performed=true;if valid{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::InvalidValue,"FEM numeric field must be finite"))}});
 meter.take(turn)?;progress.reset_completed()?;Ok(())
}
/// 🪙️ Moves one copy scalar into its retained destination.
pub fn move_scalar<T:Copy>(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,target:&mut T,source:&mut Option<T>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 if source.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"FEM scalar field requires a retained source").into())}
 transfer(owner,size_of::<T>(),progress,unwind,meter,||*target=source.take().unwrap())
}
/// 📍️ Publishes one copy scalar into an empty retained slot.
pub fn publish_scalar<T:Copy>(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<T>,value:T,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"FEM scalar publication slot must be empty").into())}
 transfer(owner,size_of::<T>(),progress,unwind,meter,||*output=Some(value))
}
/// 🧺️ Moves an admitted vector into an empty retained field.
pub fn move_vec<T:RetireOwned>(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,target:&mut Vec<T>,source:&mut Option<Vec<T>>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 if target.capacity()!=0||source.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"FEM vector field requires an empty destination and retained source").into())}
 transfer(owner,0,progress,unwind,meter,||*target=source.take().unwrap())
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"FEM exchange record differs from its declared analytical shape")}
/// 🔎️ Borrows a declared field without creating a replacement owner.
pub fn field(record:&RecordValue,id:u16)->Result<&FieldValue,ValueError>{record.get(id).ok_or_else(invalid)}
/// 📏️ Checks each declared field under its original work reservation.
pub fn declared(record:&RecordValue,count:usize,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 let quote=NativeSnapshotDecodeWorkQuote::reserve(owner.native())?;let turn=quote.run(&mut progress.performed,unwind,&mut |performed|{let valid=record.fields.len()==count;*performed=true;if valid{Ok(())}else{Err(invalid())}});meter.take(turn)?;progress.reset_completed()?;
 for(i,(id,_))in record.fields.iter().enumerate(){let quote=NativeSnapshotDecodeWorkQuote::reserve(owner.native())?;let turn=quote.run(&mut progress.performed,unwind,&mut |performed|{let valid=*id==i as u16;*performed=true;if valid{Ok(())}else{Err(invalid())}});meter.take(turn)?;progress.reset_completed()?;}Ok(())
}
/// 📝️ Receives a string into retained scratch before its exact field handoff.
pub fn text(record:&RecordValue,id:u16,target:&mut String,scratch:&mut Option<String>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 meter.take(String::receive_value(field(record,id)?,scratch,owner))?;move_string(owner,target,scratch,progress,unwind,meter)
}
/// 🔢️ Receives, validates and publishes a finite numeric field.
pub fn number(record:&RecordValue,id:u16,target:&mut f64,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 let mut value=None;meter.take(f64::receive_value(field(record,id)?,&mut value,owner))?;finite(owner,*value.as_ref().ok_or_else(invalid)?,progress,unwind,meter)?;move_scalar(owner,target,&mut value,progress,unwind,meter)
}
/// 🧳️ Keeps a partially built row and its typed publication scratch together.
#[derive(semio_framework_value::RetireOwned)]
pub struct RowFrame<T:RetireOwned,S:RetireOwned>{pub value:Option<T>,pub scratch:S,pub progress:NativeSnapshotEffectProgress,pub unwind:Option<NativeSnapshotFailure>}
/// 📑️ Receives one row into an output slot that its caller retains after refusal.
pub fn row<T:RetireOwned,S:RetireOwned>(value:&RecordValue,output:&mut Option<T>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,mut initial:impl FnMut()->(T,S),mut fill:impl FnMut(&mut T,&mut S,&mut NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&RecordValue,&mut NativeSnapshotDecodeOwner<'_,'_>,&mut ReceivingWork)->Result<(),NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 meter.take(owner.receive_nested::<RowFrame<T,S>,T,NativeSnapshotFailure>(output,&mut |slot,output,birth,birth_unwind,original|receiving_turn(|meter|{
  let bytes=size_of::<RowFrame<T,S>>();let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};let turn=original.effect(demand,birth,birth_unwind,&mut |performed,physical|{let(row,scratch)=initial();*slot=Some(RowFrame{value:Some(row),scratch,progress:Default::default(),unwind:None});*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});meter.take(turn)?;birth.reset_completed()?;
  let frame=slot.as_mut().unwrap();fill(frame.value.as_mut().unwrap(),&mut frame.scratch,&mut frame.progress,&mut frame.unwind,value,original,meter)?;
  let turn=original.move_existing(&mut frame.value,output,&mut frame.progress,&mut frame.unwind);meter.take(turn)?;frame.progress.reset_completed()?;Ok(())
 })))
}
/// 🧺️ Retains backing and an unfinished child while walking one collection.
#[derive(semio_framework_value::RetireOwned)]
pub struct Rows<T:RetireOwned>{pub values:Option<Vec<T>>,pub child:Option<T>,pub progress:NativeSnapshotEffectProgress,pub unwind:Option<NativeSnapshotFailure>}
/// 📋️ Receives list elements through caller-owned child and collection slots.
pub fn collect<T:RetireOwned,V>(values:&[V],output:&mut Option<Vec<T>>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,mut receive:impl FnMut(&V,&mut Option<T>,&mut NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut NativeSnapshotDecodeOwner<'_,'_>,&mut ReceivingWork)->Result<(),NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 meter.take(owner.receive_nested::<Rows<T>,Vec<T>,NativeSnapshotFailure>(output,&mut |slot,output,birth,birth_unwind,original|receiving_turn(|meter|{
  let bytes=size_of::<Rows<T>>();let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};let turn=original.effect(demand,birth,birth_unwind,&mut |performed,physical|{*slot=Some(Rows{values:None,child:None,progress:Default::default(),unwind:None});*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});meter.take(turn)?;birth.reset_completed()?;let frame=slot.as_mut().unwrap();
  {let Rows{values:backing,progress,unwind,..}=frame;meter.take(original.receive::<Vec<T>,Vec<T>,NativeSnapshotFailure>(backing,&mut |slot,output,_birth,_birth_unwind,native,body|receiving_turn(|meter|{
   let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_depth:1,..Default::default()};
   let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,demand)?;let turn=quote.run(progress,unwind,&mut |performed,physical|{*slot=Some(Vec::new());*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<T>>(),..Default::default()};Ok(())});meter.take(turn)?;progress.reset_completed()?;
   let capacity=values.len().checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"FEM list capacity overflow"))?;
   body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1})?;meter.admit(native)?;body.allocate_vec_into(native,values.len(),slot.as_mut().unwrap(),1)?;
   let handoff=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,handoff)?;let turn=quote.run(progress,unwind,&mut |performed,physical|{*output=slot.take();*performed=true;*physical=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(())});meter.take(turn)?;progress.reset_completed()?;Ok(())
  })))?;}

  for value in values{receive(value,&mut frame.child,&mut frame.progress,&mut frame.unwind,original,meter)?;transfer(original,0,&mut frame.progress,&mut frame.unwind,meter,||frame.values.as_mut().unwrap().push(frame.child.take().unwrap()))?;}
  let turn=original.move_existing(&mut frame.values,output,&mut frame.progress,&mut frame.unwind);meter.take(turn)?;frame.progress.reset_completed()?;Ok(())
 })))
}

/// 🎚️ Receives a finite, duplicate-free enum list into its original parent slot.
pub fn enum_list<T:Copy+PartialEq+RetireOwned>(value:&FieldValue,output:&mut Option<Vec<T>>,choices:&[T],progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 let FieldValue::List(values)=value else{return Err(invalid().into())};
 let quote=NativeSnapshotDecodeWorkQuote::reserve(owner.native())?;let turn=quote.run(&mut progress.performed,unwind,&mut |performed|{let valid=values.len()<=choices.len();*performed=true;if valid{Ok(())}else{Err(invalid())}});meter.take(turn)?;progress.reset_completed()?;
 meter.take(owner.receive::<Vec<T>,Vec<T>,NativeSnapshotFailure>(output,&mut |slot,output,_birth,_birth_unwind,native,body|receiving_turn(|meter|{
  let header=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_depth:1,..Default::default()};
  let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,header)?;let turn=quote.run(progress,unwind,&mut |performed,physical|{*slot=Some(Vec::new());*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<T>>(),..Default::default()};Ok(())});meter.take(turn)?;progress.reset_completed()?;
  let capacity=values.len().checked_mul(size_of::<T>()).ok_or_else(invalid)?;body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1})?;meter.admit(native)?;body.allocate_vec_into(native,values.len(),slot.as_mut().unwrap(),1)?;
  for value in values{
   let mut candidate=None;let quote=NativeSnapshotDecodeWorkQuote::reserve(native)?;let turn=quote.run(&mut progress.performed,unwind,&mut |performed|{*performed=true;let FieldValue::Enum(index)=value else{return Err(invalid())};let item=*choices.get(*index as usize).ok_or_else(invalid)?;if slot.as_ref().unwrap().contains(&item){return Err(invalid())}candidate=Some(item);Ok(())});meter.take(turn)?;progress.reset_completed()?;
   let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<T>(),maximum_depth:1,..Default::default()};let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,demand)?;let turn=quote.run(progress,unwind,&mut |performed,physical|{slot.as_mut().unwrap().push(candidate.take().unwrap());*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()};Ok(())});meter.take(turn)?;progress.reset_completed()?;
  }
  let handoff=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,handoff)?;let turn=quote.run(progress,unwind,&mut |performed,physical|{*output=slot.take();*performed=true;*physical=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(())});meter.take(turn)?;progress.reset_completed()?;Ok(())
 })))
}

/// 🔍️ Compares bounded text chunks under their original work reservations.
pub fn compare_text(a:&str,b:&str,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<std::cmp::Ordering,NativeSnapshotFailure>{
 let(a,b)=(a.as_bytes(),b.as_bytes());for offset in (0..a.len().min(b.len())).step_by(256){let end=(offset+256).min(a.len().min(b.len()));let order=inspect(owner,progress,unwind,meter,||Ok(a[offset..end].cmp(&b[offset..end])))?;if !order.is_eq(){return Ok(order)}}inspect(owner,progress,unwind,meter,||Ok(a.len().cmp(&b.len())))
}
/// 🧷️ Pushes one Copy locator without growing its already-admitted vector.
pub fn push_copy<T:Copy>(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,values:&mut Vec<T>,value:T,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 if values.len()==values.capacity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"FEM locator frontier lacks admitted capacity").into())}transfer(owner,size_of::<T>(),progress,unwind,meter,||values.push(value))
}
/// 🔀️ Swaps two Copy locators under one actual two-item effect.
pub fn swap_copy<T:Copy>(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,values:&mut [T],a:usize,b:usize,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<(),NativeSnapshotFailure>{
 if a>=values.len()||b>=values.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"FEM locator swap is outside its retained frontier").into())}let bytes=size_of::<T>().checked_mul(2).ok_or_else(invalid)?;let demand=RetainedCloneGrant{maximum_items:2,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};
 let turn=owner.effect(demand,progress,unwind,&mut |performed,physical|{values.swap(a,b);*performed=true;*physical=RetainedCloneProgress{copied_items:2,copied_bytes:bytes,..Default::default()};Ok(())});meter.take(turn)?;progress.reset_completed()?;Ok(())
}
/// 🧮️ Sorts admitted Copy locators without allocating a comparison frontier.
pub fn sort<T:Copy>(rows:&mut [T],owner:&mut NativeSnapshotDecodeOwner<'_,'_>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork,compare:&mut impl FnMut(T,T,&mut NativeSnapshotDecodeOwner<'_,'_>,&mut NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut ReceivingWork)->Result<std::cmp::Ordering,NativeSnapshotFailure>)->Result<(),NativeSnapshotFailure>{
 fn sift<T:Copy>(rows:&mut [T],mut root:usize,end:usize,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,progress:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork,compare:&mut impl FnMut(T,T,&mut NativeSnapshotDecodeOwner<'_,'_>,&mut NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut ReceivingWork)->Result<std::cmp::Ordering,NativeSnapshotFailure>)->Result<(),NativeSnapshotFailure>{
  loop{let left=root*2+1;if left>=end{return Ok(())}let mut child=left;if left+1<end{let(a,b)=inspect(owner,progress,unwind,meter,||Ok((rows[left],rows[left+1])))?;if compare(a,b,owner,progress,unwind,meter)?.is_lt(){child=left+1}}let(a,b)=inspect(owner,progress,unwind,meter,||Ok((rows[root],rows[child])))?;if !compare(a,b,owner,progress,unwind,meter)?.is_lt(){return Ok(())}swap_copy(owner,rows,root,child,progress,unwind,meter)?;root=child;}
 }
 let length=rows.len();for root in (0..length/2).rev(){sift(rows,root,length,owner,progress,unwind,meter,compare)?;}for end in (1..length).rev(){swap_copy(owner,rows,0,end,progress,unwind,meter)?;sift(rows,0,end,owner,progress,unwind,meter,compare)?;}Ok(())
}
