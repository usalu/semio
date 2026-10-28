//! 🧪️ Shared PDF raster plane and small-grant laws.
use super::*;
use crate::output::OutputWork;
use crate::test_oracle::{layout_export_fixture_caller,LAYOUT_EXPORT_FIXTURE_TURN,LAYOUT_EXPORT_FIXTURE_STOCK};
fn operation()->semio_framework_job::Operation{semio_framework_job::Operation::new(semio_framework_job::OperationId(1965),semio_framework_job::RevisionId(1),semio_framework_job::Generation(3),7)}

#[test]
fn pdf_raster_portable_planes_are_bounded_and_lossless() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️fixtures/📕️pdf/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let width = case["width"].as_u64().unwrap() as u32; let height = case["height"].as_u64().unwrap() as u32;
        let rgba: Vec<_> = case["rgba"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
        for (plane, key) in [(RasterPlane::Rgb, "rgb"), (RasterPlane::Alpha, "alpha")] {
            let expected: Vec<_> = case[key].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
            for limit in [1,2,3,4096]{let operation=operation();let mut caller=layout_export_fixture_caller(operation,operation.generation);let mut result=Vec::new();let mut target=[0u8;4096];loop{let count=caller.turn(LAYOUT_EXPORT_FIXTURE_TURN,|cx|{let mut work=OutputWork::new(cx.retained_grant());let(count,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||raster_chunk(width,height,&rgba,plane,result.len(),&mut target[..limit],&mut work));let count=count.map_err(crate::Fault::from)?;assert_eq!(physical.requested_bytes,0);assert_eq!(physical.released_bytes,0);assert!(!physical.overflowed);assert_eq!(work.receipt().copied_bytes,if count==0{0}else{count+std::mem::size_of::<usize>()*3});cx.consume_retained(work.receipt())?;Ok(count)}).unwrap();if count==0{break;}assert!(count<=limit);result.extend_from_slice(&target[..count]);}assert_eq!(result,expected);caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);}
        }
    }
    for case in corpus["refusals"].as_array().unwrap() { let rgba: Vec<_> = case["rgba"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect(); assert!(raster_chunk(case["width"].as_u64().unwrap() as u32, case["height"].as_u64().unwrap() as u32, &rgba, RasterPlane::Rgb,0,&mut[0u8;1],&mut OutputWork::new(LAYOUT_EXPORT_FIXTURE_TURN)).is_err()); }
}
#[test]fn original_raster_undergrants_preserve_caller_scratch_and_source(){let rgba=[1u8,2,3,4];let mut target=[77u8;1];for grant in [semio_framework_value::RetainedCloneGrant{maximum_items:0,..LAYOUT_EXPORT_FIXTURE_TURN},semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:std::mem::size_of::<usize>()*3,..LAYOUT_EXPORT_FIXTURE_TURN},semio_framework_value::RetainedCloneGrant{maximum_depth:0,..LAYOUT_EXPORT_FIXTURE_TURN}]{let mut work=OutputWork::new(grant);assert!(raster_chunk(1,1,&rgba,RasterPlane::Rgb,0,&mut target,&mut work).is_err());assert_eq!(target,[77]);assert_eq!(rgba,[1,2,3,4]);assert_eq!(work.receipt(),Default::default());}}
