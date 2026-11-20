//! 🧪️ Portable retained laws compare bitmap fragments with the independent JSON oracle through the central owner.
use super::*;
#[path="../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📽️projection/👁️post/🧪️testing/🦀️.rs"]
mod post_fixture;
use post_fixture::PostFixture;
use protocol::{RetainedDiffOwner, RetainedDiffPreflightStep};
use protocol::{RetainedDone as Done, RetainedReply as Reply};
use semio_framework_value::{OriginalRetirementFailure, RetainedWorkPoll, retirement::{RetireOwned, controlled::ControlledRetirement}};
use semio_framework_value_clone_source_testing::{FixtureSource, drive::{drive, drive_reply}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

const UNITS:u64=1<<20;
const BODY:usize=4096;
type Owner=RetainedDiffOwner<BitmapSnapshot,BitmapDiff>;

fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn denied(g:RetainedCloneGrant,d:RetirementDemand)->Vec<RetainedCloneGrant>{let mut grants=vec![RetainedCloneGrant{maximum_items:0,..g}];if d.copy_bytes>0{grants.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g});}if d.capacity_bytes>0{grants.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g});}if d.release_bytes>0{grants.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g});}if d.depth>0{grants.push(RetainedCloneGrant{maximum_depth:d.depth-1,..g});}grants}
fn source<T:RetireOwned+Sync+'static>(value:T)->FixtureSource<T>{FixtureSource::from_owner(value)}
fn drain<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).map_err(|(e,_)|e).unwrap();while !owner.terminal_is_empty(){let d=owner.next_demand(BODY).unwrap();let units=owner.next_computational_work(BODY).unwrap();let step=drive(grant(d),units,|effect|owner.step(BODY,effect)).result.unwrap_or_else(|_|panic!("original retirement turn refused"));assert!(step.progress().fits(grant(d)));}}
fn pins(value:&serde_json::Value)->Vec<BitmapPinnedPixel>{value.as_array().unwrap().iter().map(|v|BitmapPinnedPixel{x:v[0].as_u64().unwrap() as u32,y:v[1].as_u64().unwrap() as u32,color:v[2].as_u64().unwrap() as u32}).collect()}
fn palette(colors:&[(u32,u32,u32)])->Vec<BitmapColor>{colors.iter().map(|c|BitmapColor::opaque(c.0,c.1,c.2)).collect()}
fn two_by_one(pinned:Vec<BitmapPinnedPixel>)->BitmapSnapshot{BitmapSnapshot{input:BitmapInput{width:2,height:1,palette:palette(&[(0,0,0),(255,255,255)]),pixels:vec![0,1]},output:BitmapOutputSpec{width:2,height:1,periodic:false},pinned,..Default::default()}}

fn preflight(owner:&mut Owner,diff:RetainedCloneRef<'_,BitmapDiff>,original:RetainedCloneRef<'_,BitmapSnapshot>,post:RetainedCloneRef<'_,BitmapSnapshot>,g:RetainedCloneGrant,receiving:&mut NativeDecodeControl<'_>)->RetainedDiffPreflightStep{
    drive_reply::<RetainedDiffPreflightStep,OriginalRetirementFailure>(g,UNITS,|answer|match answer{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()},|effect,reply:Reply<'_,RetainedDiffPreflightStep>,recover|->Result<Done<'_,RetainedDiffPreflightStep>,ValueError>{if recover{protocol::recover_preflight_retained_diff_into(owner,diff,original,post,BODY,UNITS,effect,&mut *receiving,reply)}else{protocol::preflight_retained_diff_into(owner,diff,original,post,BODY,UNITS,effect,&mut *receiving,reply)}}).result.unwrap_or_else(|_|panic!("original central preflight refused"))
}
fn commit(owner:&mut Owner,diff:RetainedCloneRef<'_,BitmapDiff>,original:RetainedCloneRef<'_,BitmapSnapshot>,post:&mut PostFixture<BitmapSnapshot>,g:RetainedCloneGrant)->RetainedCloneStep{
    drive(g,UNITS,|effect|RetainedWorkPoll::Complete(protocol::commit_retained_diff(owner,diff,original,&mut *post,effect))).result.unwrap_or_else(|_|panic!("original central commit refused"))
}
fn close_once(owner:&mut Owner,g:RetainedCloneGrant)->RetainedCloneStep{
    drive_reply::<RetainedCloneStep,OriginalRetirementFailure>(g,UNITS,|answer|match answer{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()},|effect,reply:Reply<'_>,recover|->Result<Done<'_>,ValueError>{if recover{owner.recover_close_into(BODY,effect,reply)}else{owner.close_into(BODY,effect,reply)}}).result.unwrap_or_else(|_|panic!("original central close refused"))
}
fn close_owner(owner:&mut Owner){
    owner.begin_close();
    while !owner.terminal_is_empty(){let d=owner.close_demand(BODY).unwrap();for denied in denied(grant(d),d){assert_eq!(close_once(owner,denied).progress(),Default::default());}assert!(close_once(owner,grant(d)).progress().fits(grant(d)));}
}
fn new_owner()->Owner{protocol::begin_retained_diff::<BitmapSnapshot,BitmapDiff>(grant(protocol::retained_diff_birth_demand::<BitmapSnapshot,BitmapDiff>())).unwrap().0}

/// 🪢️ One drive of the whole preflight; `stop` cancels after that many funded turns and returns whether the diff was admitted.
fn run_preflight(owner:&mut Owner,diff:&FixtureSource<BitmapDiff>,original:&FixtureSource<BitmapSnapshot>,post:&PostFixture<BitmapSnapshot>,base:&BitmapSnapshot,stop:Option<usize>)->(Option<bool>,u64){
    let mut accept=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut receiving=NativeDecodeControl::new(1<<28,&mut accept);
    let mut paid=0u64;let mut turns=0;
    while stop.is_none_or(|stop|turns<stop){
        let d=owner.preflight_demand(diff.borrow(),original.borrow(),post.borrow(),BODY,UNITS).unwrap();
        for denied in denied(grant(d),d){assert_eq!(preflight(owner,diff.borrow(),original.borrow(),post.borrow(),denied,&mut receiving).progress(),Default::default());assert_eq!(owner.preflight_demand(diff.borrow(),original.borrow(),post.borrow(),BODY,UNITS).unwrap(),d);}
        let step=preflight(owner,diff.borrow(),original.borrow(),post.borrow(),grant(d),&mut receiving);assert!(step.progress().fits(grant(d)));assert_eq!(post.borrow().get(),base);paid+=step.progress().retained_capacity_bytes as u64;turns+=1;assert!(turns<100000);
        match step{RetainedDiffPreflightStep::Admitted(_,rows)=>{drain(rows);return (Some(true),paid)},RetainedDiffPreflightStep::Refused(_,rows)=>{drain(rows);return (Some(false),paid)},_=>{}}
    }
    (None,paid)
}
/// ✍️ Commits an admitted preflight, handing the mutable post back through its fixture.
fn run_commit(owner:&mut Owner,diff:&FixtureSource<BitmapDiff>,original:&FixtureSource<BitmapSnapshot>,post:&mut PostFixture<BitmapSnapshot>){
    for _ in 0..100000{let d=owner.commit_demand(diff.borrow(),original.borrow(),post.mutable_read(),BODY).unwrap();for denied in denied(grant(d),d){assert_eq!(commit(owner,diff.borrow(),original.borrow(),post,denied).progress(),Default::default());}if matches!(commit(owner,diff.borrow(),original.borrow(),post,grant(d)),RetainedCloneStep::Complete(_)){return}}
    panic!("central commit stalled");
}

#[test]
fn original_bitmap_key_scan_refuses_oversized_text_before_search(){assert_eq!(key_coordinates("999:999"),Some((999,999)));assert_eq!(key_coordinates("000:1"),None);let oversized="9".repeat(100_000);assert_eq!(key_coordinates(&oversized),None);}

#[test]
fn retained_bitmap_preflight_is_atomic_cancellable_and_original_bound(){
    let law:serde_json::Value=serde_json::from_str(include_str!("retained.json")).unwrap();
    for case in law["cases"].as_array().unwrap(){
        let base=two_by_one(pins(&case["basePins"]));
        let target=pins(&case["afterPins"]);
        let diff=BitmapDiff{pinned:BitmapPinnedDelta{removed:base.pinned.iter().enumerate().map(|(index,p)|BitmapPinnedRemoval{id:pin_key(p.x,p.y),index}).collect(),inserted:target.iter().enumerate().map(|(index,p)|BitmapPinnedInsertion{index,row:*p}).collect(),..Default::default()},input:BitmapInputPatch{size:None,writes:case.get("writes").and_then(serde_json::Value::as_array).into_iter().flatten().map(|r|BitmapInputWrite::Region{region:BitmapPixelRegion{x:r["x"].as_u64().unwrap() as u32,y:r["y"].as_u64().unwrap() as u32,width:r["width"].as_u64().unwrap() as u32,height:r["height"].as_u64().unwrap() as u32,pixels:r["pixels"].as_array().unwrap().iter().map(|x|x.as_u64().unwrap() as u8).collect()}}).collect()},..Default::default()};
        let original=source(base.clone());let mut post=PostFixture::new(base.clone());let diff=source(diff);let mut owner=new_owner();
        let (admitted,_)=run_preflight(&mut owner,&diff,&original,&post,&base,None);
        assert_eq!(admitted,case["admitted"].as_bool(),"{}",case["id"]);
        if admitted==Some(true){
            run_commit(&mut owner,&diff,&original,&mut post);
            assert_eq!(post.pinned,target);
            assert_eq!(serde_json::json!(post.input.pixels),case["expectedPixels"]);
        }
        close_owner(&mut owner);
    }
}

#[test]
fn cancellation_returns_every_staged_frontier_without_changing_original_post(){
    let base=BitmapSnapshot{output:BitmapOutputSpec{width:2,height:1,periodic:false},..Default::default()};
    let diff=source(BitmapDiff{pinned:BitmapPinnedDelta::insertion(0,BitmapPinnedPixel{x:1,y:0,color:0}),..Default::default()});let original=source(base.clone());
    let mut completed=false;
    for stop in 0..1000{
        let post=PostFixture::new(base.clone());let mut owner=new_owner();
        let (admitted,_)=run_preflight(&mut owner,&diff,&original,&post,&base,Some(stop));
        assert_eq!(post.borrow().get(),&base);close_owner(&mut owner);
        if admitted==Some(true){completed=true;break;}
    }
    assert!(completed);
}

#[test]
fn unchanged_owned_fields_preserve_the_original_projection_allocation_budget(){
    let law:serde_json::Value=serde_json::from_str(include_str!("retained.json")).unwrap();
    let base=two_by_one(vec![BitmapPinnedPixel{x:1,y:0,color:1}]);
    let original=source(base.clone());
    let mut baseline=None;
    for case in law["allocationCases"].as_array().unwrap(){
        let diff=BitmapDiff{seed:case.get("seed").and_then(serde_json::Value::as_u64),input:BitmapInputPatch{size:None,writes:case.get("pixels").map(|v|vec![BitmapInputWrite::Region{region:BitmapPixelRegion{x:0,y:0,width:2,height:1,pixels:v.as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u8).collect()}}]).unwrap_or_default()},palette:case.get("paletteR").map(|v|BitmapPaletteDelta::recoloring(0,BitmapColor::opaque(v.as_u64().unwrap() as u32,0,0))).unwrap_or_default(),..Default::default()};
        let diff=source(diff);let mut post=PostFixture::new(base.clone());let mut owner=new_owner();
        let (admitted,allocated)=run_preflight(&mut owner,&diff,&original,&post,&base,None);assert_eq!(admitted,Some(true),"{}",case["id"]);
        let baseline=*baseline.get_or_insert(allocated);
        assert_eq!(allocated,baseline+case["extraCapacityBytes"].as_u64().unwrap(),"{}",case["id"]);
        run_commit(&mut owner,&diff,&original,&mut post);
        assert_eq!(serde_json::json!(post.input.pixels),case["expectedPixels"]);assert_eq!(serde_json::json!(post.input.palette.iter().map(|c|c.r).collect::<Vec<_>>()),case["expectedPaletteR"]);assert_eq!(post.seed,case["expectedSeed"].as_u64().unwrap());assert_eq!(post.pinned,base.pinned);
        close_owner(&mut owner);
    }
}

#[test]
fn malformed_original_extent_is_refused_before_platform_sized_multiplication(){
    let law:serde_json::Value=serde_json::from_str(include_str!("retained.json")).unwrap();
    let base=BitmapSnapshot{input:BitmapInput{width:law["malformedInputSize"][0].as_u64().unwrap() as u32,height:law["malformedInputSize"][1].as_u64().unwrap() as u32,..Default::default()},..Default::default()};
    let original=source(base.clone());let post=PostFixture::new(base.clone());let diff=source(BitmapDiff{input:BitmapInputPatch{size:Some(BitmapSize{width:1,height:1}),writes:Vec::new()},..Default::default()});let mut owner=new_owner();
    let (admitted,_)=run_preflight(&mut owner,&diff,&original,&post,&base,None);
    assert_eq!(admitted,Some(false));close_owner(&mut owner);
}
