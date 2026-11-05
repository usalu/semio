//! 🧪️ Portable retained laws compare bitmap fragments with the independent JSON oracle.
use super::*;
use semio_framework_value::{retirement::RetireOwned, retained_clone::{RetainedCloneSource,RetainedCloneSourceTake}};
use semio_framework_value_snapshot_projection::{SnapshotProjection,SnapshotProjectionCursor};
use semio_framework_value_testing_allocator::observe_retirement_allocations;

fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn generous()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:4096}}
fn source<T:RetireOwned+Sync+'static>(value:T)->RetainedCloneSource<T,T>{
    let mut admission=RetainedCloneSource::admission_owned(value,());
    for _ in 0..1000{if admission.phase()=="ready"{break;}let g=grant(admission.next_demand().unwrap());let p=admission.advance(g).result.unwrap().progress();assert!(p.fits(g));}
    let mut output=None;admission.take_output(&mut output,generous()).result.unwrap();assert!(admission.terminal_is_empty());output.unwrap()
}
fn close_source<T:RetireOwned+Sync+'static>(source:&mut RetainedCloneSource<T,T>){for _ in 0..100000{if source.terminal_is_empty(){return;}let d=source.next_close_demand(4096).unwrap();let g=grant(d);let p=source.close_step(g).result.unwrap().progress();assert!(p.fits(g));}panic!("source close stalled");}
fn pins(value:&serde_json::Value)->Vec<BitmapPinnedPixel>{value.as_array().unwrap().iter().map(|v|BitmapPinnedPixel{x:v[0].as_u64().unwrap() as u32,y:v[1].as_u64().unwrap() as u32,color:v[2].as_u64().unwrap() as u32}).collect()}
fn close_plan(plan:&mut BitmapApply){plan.begin_close();for _ in 0..100000{if plan.terminal_is_empty(){return;}let g=grant(plan.close_demand(4096).unwrap());assert_eq!(plan.close_step(RetainedCloneGrant{maximum_items:0,..g}).unwrap().progress(),Default::default());let p=plan.close_step(g).unwrap().progress();assert!(p.fits(g));}panic!("plan close stalled");}

#[test]
fn retained_bitmap_preflight_is_atomic_cancellable_and_original_bound(){
    let law:serde_json::Value=serde_json::from_str(include_str!("retained.json")).unwrap();
    for case in law["cases"].as_array().unwrap(){
        let base=BitmapSnapshot{input:BitmapInput{width:2,height:1,palette:vec![BitmapColor::opaque(0,0,0),BitmapColor::opaque(255,255,255)],pixels:vec![0,1]},output:BitmapOutputSpec{width:2,height:1,periodic:false},pinned:pins(&case["basePins"]),..Default::default()};
        let target=pins(&case["afterPins"]);
        let diff=BitmapDiff{pinned:BitmapPinnedDelta{removed:base.pinned.iter().enumerate().map(|(index,p)|BitmapPinnedRemoval{id:pin_key(p.x,p.y),index}).collect(),inserted:target.iter().enumerate().map(|(index,p)|BitmapPinnedInsertion{index,row:*p}).collect(),..Default::default()},input:BitmapInputPatch{size:None,writes:case.get("writes").and_then(serde_json::Value::as_array).into_iter().flatten().map(|r|BitmapInputWrite::Region{region:BitmapPixelRegion{x:r["x"].as_u64().unwrap() as u32,y:r["y"].as_u64().unwrap() as u32,width:r["width"].as_u64().unwrap() as u32,height:r["height"].as_u64().unwrap() as u32,pixels:r["pixels"].as_array().unwrap().iter().map(|x|x.as_u64().unwrap() as u8).collect()}}).collect()},..Default::default()};
        let mut original=source(base.clone());let mut post=source(base.clone());let mut diff_source=source(diff);
        let mut accept=|_|true;let mut receiving=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1<<28,&mut accept);
        let (mut plan,_)=protocol::begin_retained_diff::<BitmapSnapshot,BitmapDiff>(generous()).unwrap();let mut admitted=None;
        for _ in 0..law["maximumTurns"].as_u64().unwrap(){
            let d=plan.preflight_demand(diff_source.borrow(),original.borrow(),post.borrow(),4096).unwrap();let g=grant(d);
            let denied=protocol::preflight_retained_diff(&mut plan,diff_source.borrow(),original.borrow(),post.borrow(),4096,RetainedCloneGrant{maximum_items:0,..g},&mut receiving).unwrap();assert_eq!(denied.progress(),Default::default());
            let step=protocol::preflight_retained_diff(&mut plan,diff_source.borrow(),original.borrow(),post.borrow(),4096,g,&mut receiving).unwrap();assert!(step.progress().fits(g));assert_eq!(post.borrow().get(),&base);
            match step{protocol::RetainedDiffPreflightStep::Admitted(_,_)=>{admitted=Some(true);break;},protocol::RetainedDiffPreflightStep::Refused(_,_)=>{admitted=Some(false);break;},_=>{}}
        }
        assert_eq!(admitted,case["admitted"].as_bool());
        if admitted==Some(true){
            let mut actual=None;for _ in 0..1000{match post.take_authority(generous()).result.unwrap(){RetainedCloneSourceTake::Pending(_)=>{},RetainedCloneSourceTake::Ready(value,_)=>{actual=Some(value);break;}}}
            let mut actual=actual.expect("original post alias is closed before mutable handback");
            let g=grant(plan.commit_demand(diff_source.borrow(),original.borrow(),&actual,4096).unwrap());
            assert_eq!(protocol::commit_retained_diff(&mut plan,diff_source.borrow(),original.borrow(),&mut actual,RetainedCloneGrant{maximum_items:0,..g}).unwrap().progress(),Default::default());
            assert!(matches!(protocol::commit_retained_diff(&mut plan,diff_source.borrow(),original.borrow(),&mut actual,g).unwrap(),RetainedCloneStep::Complete(_)));
            assert_eq!(actual.pinned,target);
            assert_eq!(serde_json::json!(actual.input.pixels),case["expectedPixels"]);
        }
        plan.begin_close();for _ in 0..100000{if plan.terminal_is_empty(){break;}let g=grant(plan.close_demand(4096).unwrap());assert!(plan.close_step(g).unwrap().progress().fits(g));}assert!(plan.terminal_is_empty());close_source(&mut diff_source);close_source(&mut original);close_source(&mut post);
    }
}

#[test]
fn snapshot_projection_uses_paid_owned_fragments_and_can_cancel_at_every_frontier(){
    let mut original=source(BitmapSnapshot::reference_scene());
    let (mut cursor,p)=BitmapSnapshot::begin_projection(grant(BitmapSnapshot::projection_birth_demand())).result.unwrap();assert!(p.copied_items>0);
    for _ in 0..100000{if cursor.ready(){break;}let g=grant(cursor.demands(original.borrow(),4096).unwrap());assert_eq!(cursor.advance(original.borrow(),4096,RetainedCloneGrant{maximum_items:0,..g}).result.unwrap().progress(),Default::default());let p=cursor.advance(original.borrow(),4096,g).result.unwrap().progress();assert!(p.fits(g));}
    assert!(cursor.ready());let g=grant(cursor.take_demand().unwrap());let (value,p)=cursor.take(g).result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(&value,original.borrow().get());
    cursor.begin_close();for _ in 0..100000{if cursor.terminal_is_empty(){break;}let g=grant(cursor.close_demands(4096).unwrap());assert!(cursor.close_step(g).result.unwrap().progress().fits(g));}assert!(cursor.terminal_is_empty());close_source(&mut original);
}

#[test]
fn cancellation_returns_every_staged_frontier_without_changing_original_post(){
    let base=BitmapSnapshot{output:BitmapOutputSpec{width:2,height:1,periodic:false},..Default::default()};
    let mut original=source(base.clone());let mut post=source(base.clone());let mut diff=source(BitmapDiff{pinned:BitmapPinnedDelta::insertion(0,BitmapPinnedPixel{x:1,y:0,color:0}),..Default::default()});
    let mut accept=|_|true;let mut receiving=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1<<28,&mut accept);
    let mut completed=false;
    for stop in 0..1000{
        let mut plan=BitmapApply::new();
        for _ in 0..stop{
            let d=plan.preflight_demand(diff.borrow(),original.borrow(),post.borrow(),4096).unwrap();let g=grant(d);
            for denied in [(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().flatten(){assert_eq!(plan.preflight(diff.borrow(),original.borrow(),post.borrow(),4096,denied,&mut receiving).unwrap().progress(),Default::default());assert_eq!(plan.preflight_demand(diff.borrow(),original.borrow(),post.borrow(),4096).unwrap(),d);}
            let step=plan.preflight(diff.borrow(),original.borrow(),post.borrow(),4096,g,&mut receiving).unwrap();assert!(step.progress().fits(g));
            if matches!(step,RetainedDiffPreflightAdvance::Ready(_,true)){completed=true;break;}
        }
        assert_eq!(post.borrow().get(),&base);close_plan(&mut plan);if completed{break;}
    }
    assert!(completed);close_source(&mut diff);close_source(&mut original);close_source(&mut post);
}

fn observed_copy_capacity(post:RetainedCloneRef<'_,BitmapSnapshot>)->usize{
    let mut field=RetainedFieldPreparation::<BitmapSnapshot>::default();let mut allocated=0;
    for _ in 0..100000{if field.value().is_some(){break;}let g=grant(field.demand(post,4096).unwrap());let (step,heap)=observe_retirement_allocations(||field.advance(post,4096,g));let p=step.unwrap().progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));allocated+=heap.0;}
    let g=grant(field.exchange_demand().unwrap());let (step,heap)=observe_retirement_allocations(||field.take(g));let(value,p)=step.unwrap();assert!(value.is_some());assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));allocated+=heap.0;
    field.begin_close();for _ in 0..100000{if field.terminal_is_empty(){break;}let g=grant(field.close_demand(4096).unwrap());let(step,heap)=observe_retirement_allocations(||field.close_step(g));let p=step.unwrap().progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));allocated+=heap.0;}assert!(field.terminal_is_empty());allocated
}

#[test]
fn unchanged_owned_fields_preserve_the_original_projection_allocation_budget(){
    let law:serde_json::Value=serde_json::from_str(include_str!("retained.json")).unwrap();
    let base=BitmapSnapshot{input:BitmapInput{width:2,height:1,palette:vec![BitmapColor::opaque(0,0,0),BitmapColor::opaque(255,255,255)],pixels:vec![0,1]},output:BitmapOutputSpec{width:2,height:1,periodic:false},pinned:vec![BitmapPinnedPixel{x:1,y:0,color:1}],..Default::default()};
    let mut original=source(base.clone());let mut post=source(base.clone());let baseline=observed_copy_capacity(post.borrow());
    for case in law["allocationCases"].as_array().unwrap(){
        let diff=BitmapDiff{seed:case.get("seed").and_then(serde_json::Value::as_u64),input:BitmapInputPatch{size:None,writes:case.get("pixels").map(|v|vec![BitmapInputWrite::Region{region:BitmapPixelRegion{x:0,y:0,width:2,height:1,pixels:v.as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u8).collect()}}]).unwrap_or_default()},palette:case.get("paletteR").map(|v|BitmapPaletteDelta::recoloring(0,BitmapColor::opaque(v.as_u64().unwrap() as u32,0,0))).unwrap_or_default(),..Default::default()};
        let mut diff=source(diff);let mut plan=BitmapApply::new();let mut allocated=0;let mut completed=false;
        let mut accept=|_|true;let mut receiving=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1<<28,&mut accept);
        for _ in 0..100000{
            let g=grant(plan.preflight_demand(diff.borrow(),original.borrow(),post.borrow(),4096).unwrap());
            let(denied,heap)=observe_retirement_allocations(||plan.preflight(diff.borrow(),original.borrow(),post.borrow(),4096,RetainedCloneGrant{maximum_items:0,..g},&mut receiving));assert_eq!(denied.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));
            if g.maximum_capacity_bytes>0{let(denied,heap)=observe_retirement_allocations(||plan.preflight(diff.borrow(),original.borrow(),post.borrow(),4096,RetainedCloneGrant{maximum_capacity_bytes:g.maximum_capacity_bytes-1,..g},&mut receiving));assert_eq!(denied.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));}
            let(step,heap)=observe_retirement_allocations(||plan.preflight(diff.borrow(),original.borrow(),post.borrow(),4096,g,&mut receiving));let step=step.unwrap();let p=step.progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));allocated+=heap.0;
            if matches!(step,RetainedDiffPreflightAdvance::Ready(_,true)){completed=true;break;}
        }
        assert!(completed);assert_eq!(allocated,baseline+case["extraCapacityBytes"].as_u64().unwrap() as usize,"{}",case["id"]);
        let result=plan.candidate.as_ref().unwrap();assert_eq!(serde_json::json!(result.input.pixels),case["expectedPixels"]);assert_eq!(serde_json::json!(result.input.palette.iter().map(|c|c.r).collect::<Vec<_>>()),case["expectedPaletteR"]);assert_eq!(result.seed,case["expectedSeed"].as_u64().unwrap());assert_eq!(result.pinned,base.pinned);assert_eq!(post.borrow().get(),&base);
        close_plan(&mut plan);close_source(&mut diff);
    }
    close_source(&mut original);close_source(&mut post);
}

#[test]
fn malformed_original_extent_is_refused_before_platform_sized_multiplication(){
    let law:serde_json::Value=serde_json::from_str(include_str!("retained.json")).unwrap();
    let base=BitmapSnapshot{input:BitmapInput{width:law["malformedInputSize"][0].as_u64().unwrap() as u32,height:law["malformedInputSize"][1].as_u64().unwrap() as u32,..Default::default()},..Default::default()};
    let mut original=source(base.clone());let mut post=source(base.clone());let mut diff=source(BitmapDiff{input:BitmapInputPatch{size:Some(BitmapSize{width:1,height:1}),writes:Vec::new()},..Default::default()});let mut plan=BitmapApply::new();let mut refused=false;
    let mut accept=|_|true;let mut receiving=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1<<28,&mut accept);
    for _ in 0..100000{let g=grant(plan.preflight_demand(diff.borrow(),original.borrow(),post.borrow(),4096).unwrap());let step=plan.preflight(diff.borrow(),original.borrow(),post.borrow(),4096,g,&mut receiving).unwrap();assert!(step.progress().fits(g));if matches!(step,RetainedDiffPreflightAdvance::Ready(_,false)){refused=true;break;}}
    assert!(refused);assert_eq!(post.borrow().get(),&base);close_plan(&mut plan);close_source(&mut diff);close_source(&mut original);close_source(&mut post);
}
