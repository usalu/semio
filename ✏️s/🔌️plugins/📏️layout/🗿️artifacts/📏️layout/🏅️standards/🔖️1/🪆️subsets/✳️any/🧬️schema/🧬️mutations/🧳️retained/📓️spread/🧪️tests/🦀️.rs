//! 📓️ Neutral spread commands preserve owned payload identity at every cancellation cut.
use crate::{LayoutDiff,LayoutMutation,LayoutSnapshot,protocol};
use super::observe_test_turn as observe;
use crate::retained::spread::output::SpreadOutput;
use semio_framework_value::{RetireOwned,RetirementDemand,RetainedCloneGrant};
use semio_framework_value_clone_source_testing::FixtureSource;

fn grant(d:RetirementDemand)->RetainedCloneGrant{
    RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}
}
fn decode<T:semio_framework_value::FromValue>(v:&serde_json::Value)->T{
    semio_framework_pack_json::from_json_str(&v.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}
fn retire<T:RetireOwned+Sync>(value:T){drop(FixtureSource::from_owner(value));}
fn close(p:&mut SpreadOutput<LayoutMutation>){
    p.begin_close();
    for _ in 0..100000{
        if p.terminal_is_empty(){return;}
        let body=256;
        let d=p.close_demand(body).unwrap();
        assert_eq!(observe(Default::default(),|effect,recover|p.close_step(None,body,effect,recover)).progress(),Default::default());
        observe(grant(d),|effect,recover|p.close_step(None,body,effect,recover));
    }
    panic!("spread original close stalled")
}
#[test]
fn original_affected_spread_strings_keep_physical_custody_and_exact_receipts(){
 let corpus:serde_json::Value=serde_json::from_str(&crate::test_fixture_text("../../🧳️retained/🎚️simple/🧪️tests/🔣️.json")).unwrap();
 let mut checked=0;
 for row in corpus["commands"].as_array().unwrap().iter().filter(|row|row["accepted"]==true&&row["mutation"].get("SetPageSpread").is_some()){
  let event:LayoutMutation=decode(&row["mutation"]);let original:LayoutSnapshot=decode(row.get("before").unwrap_or(&corpus["base"]));
  let LayoutMutation::SetPageSpread(p)=&event else{unreachable!()};
  let at=[original.pages.iter().position(|page|page.id==p.id).unwrap(),original.spreads.iter().position(|spread|spread.id==p.spread_id).unwrap()];
  for cut in[0,1,5,15,40,100,usize::MAX]{
   let authored=FixtureSource::from_owner(event.clone());let base=FixtureSource::from_owner(original.clone());let mut prepared=vec![event.clone()];
   let(id,target)=match &prepared[0]{LayoutMutation::SetPageSpread(p)=>(p.id.as_ptr(),p.spread_id.as_ptr()),_=>unreachable!()};
   let mut diff=LayoutDiff::default();let mut owner=SpreadOutput::<LayoutMutation>::default();
   for turn in 0..cut{
    if owner.ready(){break}assert!(turn<100000);
    let d=owner.demand(authored.try_borrow().unwrap(),base.try_borrow().unwrap(),at,256,1024).unwrap();let g=grant(d);
    for short in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g},RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g},RetainedCloneGrant{maximum_depth:d.depth-1,..g}]{if short==g{continue}assert_eq!(observe(short,|effect,recover|owner.advance(&mut prepared,authored.try_borrow().unwrap(),base.try_borrow().unwrap(),at,&mut diff,256,1024,effect,recover)).progress(),Default::default());}
    observe(g,|effect,recover|owner.advance(&mut prepared,authored.try_borrow().unwrap(),base.try_borrow().unwrap(),at,&mut diff,256,1024,effect,recover));
   }
   if cut==usize::MAX{
    assert!(owner.ready());let pages=&diff.pages.as_ref().unwrap().modified;
    assert_eq!(pages[0].id.as_ptr(),id);assert_eq!(pages[0].patch.spread_id.as_ref().unwrap().as_ptr(),target);
    let expected:LayoutSnapshot=decode(&row["after"]);let output=protocol::apply_diff_cold(&diff,&original).unwrap();assert_eq!(output,expected);retire(output);
   }
   assert_eq!(base.try_borrow().unwrap().get(),&original);assert_eq!(authored.try_borrow().unwrap().get(),&event);
   close(&mut owner);retire(diff);retire(prepared);
  }
  checked+=1;
 }
 assert_eq!(checked,4);
}
