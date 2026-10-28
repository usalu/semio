use super::*;
use semio_framework_value::{RetireOwned,retained_clone::RetainedCloneSource};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn decode<T:semio_framework_value::FromValue>(v:&serde_json::Value)->T{semio_framework_pack_json::from_json_str(&v.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn source<T:RetireOwned+Sync>(v:T)->RetainedCloneSource<T>{let d=RetainedCloneSource::<T>::owned_constructor_demand::<()>();let g=grant(RetirementDemand{copy_bytes:RetainedCloneSource::<T>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()});RetainedCloneSource::admit_owned(v,(),g).unwrap_or_else(|(e,_,_)|panic!("{e}")).0}
fn close_source<T:RetireOwned+Sync>(s:&mut RetainedCloneSource<T>){while !s.terminal_is_empty(){let d=s.next_close_demand(256).unwrap();s.close_step(grant(d)).unwrap();}}
fn retire<T:RetireOwned>(v:T){let mut v=Some(v);let mut r=RetainedCloneClose::default();while v.is_some()||!r.is_empty(){let d=r.next_owner_demands::<T>(v.is_some(),&None,256).unwrap();if !r.is_empty(){r.step_granted(grant(d)).unwrap();}else{r.begin_granted(&mut v,grant(d)).unwrap();}}}
fn observe(g:RetainedCloneGrant,f:impl FnOnce()->RetainedCloneStep)->RetainedCloneStep{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);assert!(!heap.overflowed);assert_eq!(heap.requested_bytes,step.progress().retained_capacity_bytes);assert_eq!(heap.released_bytes,step.progress().released_bytes);assert!(step.progress().fits(g));step}
fn close(p:&mut SpreadOutput){p.begin_close();for _ in 0..100000{if p.terminal_is_empty(){return}let d=p.close_demand(256).unwrap();assert_eq!(observe(Default::default(),||p.close_step(Default::default()).unwrap()).progress(),Default::default());observe(grant(d),||p.close_step(grant(d)).unwrap());}panic!("spread original close stalled")}
#[test]
fn original_affected_spread_strings_keep_physical_custody_and_exact_receipts(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🎚️simple/🧪️tests/🔣️.json")).unwrap();
 let mut checked=0;
 for row in corpus["commands"].as_array().unwrap().iter().filter(|row|row["accepted"]==true&&row["mutation"].get("SetPageSpread").is_some()){
  let event:LayoutMutation=decode(&row["mutation"]);let original:LayoutSnapshot=decode(row.get("before").unwrap_or(&corpus["base"]));
  let LayoutMutation::SetPageSpread(p)=&event else{unreachable!()};
  let at=[original.pages.iter().position(|page|page.id==p.id).unwrap(),original.spreads.iter().position(|spread|spread.id==p.spread_id).unwrap()];
  for cut in[0,1,5,15,40,100,usize::MAX]{
   let mut authored=source(event.clone());let mut base=source(original.clone());let mut prepared=vec![event.clone()];
   let(id,target)=match &prepared[0]{LayoutMutation::SetPageSpread(p)=>(p.id.as_ptr(),p.spread_id.as_ptr()),_=>unreachable!()};
   let mut diff=LayoutDiff::default();let mut owner=SpreadOutput::default();let mut pointers=None;
   for turn in 0..cut{
    if owner.ready(){break}assert!(turn<100000);
    if owner.phase==11{pointers=Some((owner.held[0].as_ref().unwrap().iter().map(|s|s.as_ptr()).collect::<Vec<_>>(),owner.strings[2].as_ref().unwrap().as_ptr()));}
    let d=owner.demand(authored.borrow(),base.borrow(),at,256).unwrap();let g=grant(d);
    for short in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g},RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g},RetainedCloneGrant{maximum_depth:d.depth-1,..g}]{if short==g{continue}assert_eq!(observe(short,||owner.advance(&mut prepared,authored.borrow(),base.borrow(),at,&mut diff,256,short).unwrap()).progress(),Default::default());}
    observe(g,||owner.advance(&mut prepared,authored.borrow(),base.borrow(),at,&mut diff,256,g).unwrap());
   }
   if cut==usize::MAX{
    assert!(owner.ready());let pages=&diff.pages.as_ref().unwrap().modified;let spreads=&diff.spreads.as_ref().unwrap().modified;
    assert_eq!(pages[0].id.as_ptr(),id);assert_eq!(pages[0].patch.spread_id.as_ref().unwrap().as_ptr(),target);
    let expected:LayoutSnapshot=decode(&row["after"]);let output=protocol::apply_diff_cold(&diff,&original).unwrap();assert_eq!(output,expected);retire(output);
    let(old,inserted)=pointers.unwrap();let ids=spreads[0].patch.page_ids.as_ref().unwrap();for i in 0..ids.len(){let expected=if i==owner.destination{inserted}else{let mut original=i-usize::from(i>owner.destination);if owner.positions[0].is_some_and(|position|original>=position){original+=1}old[original]};assert_eq!(ids[i].as_ptr(),expected)}
   }
   assert_eq!(base.borrow().get(),&original);assert_eq!(authored.borrow().get(),&event);
   close(&mut owner);retire(diff);retire(prepared);close_source(&mut authored);close_source(&mut base);
  }
  checked+=1;
 }
 assert_eq!(checked,4);
}
