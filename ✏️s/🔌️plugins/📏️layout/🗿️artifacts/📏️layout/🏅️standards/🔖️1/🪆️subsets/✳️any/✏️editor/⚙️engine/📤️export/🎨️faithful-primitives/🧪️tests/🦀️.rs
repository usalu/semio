//! 🎟️ Real fidelity owners refuse unfunded reservations and preserve geometry under paid single-source turns.
use super::*;
use semio_framework_value::RetainedCloneGrant;
fn grant(row:&serde_json::Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:row["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:row["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:1048576,maximum_depth:row["depth"].as_u64().unwrap()as usize}}
#[test]
fn original_grants_control_fidelity_pages_and_exact_rectangle_geometry(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️funding.json")).unwrap();let r=&fixture["rectangle"];let items=[PdfItem::Rect{x:r["x"].as_f64().unwrap(),y:r["y"].as_f64().unwrap(),width:r["width"].as_f64().unwrap(),height:r["height"].as_f64().unwrap(),rotation:0.0,fill:Some([0.2,0.4,0.8,1.0]),stroke:None}];
 let rows=items;let mut items=crate::PlanItems::new();for row in rows{crate::append_plan_fixture(&mut items,row);}
 for denied in fixture["denied"].as_array().unwrap(){let mut state=FidelityState::new();for _ in 0..16{let mut work=FontWorkLedger::new(grant(denied));state.prepare_one(&items,None,None,&mut work).unwrap();assert_eq!(work.receipt(),Default::default());assert!(state.shape(0).is_none());}assert!(!state.prepared);crate::close_original_fixture(state);}
 let mut state=FidelityState::new();let mut allocated=0;for _ in 0..4096{let mut work=FontWorkLedger::new(grant(&fixture["admitted"]));state.prepare_one(&items,None,None,&mut work).unwrap();let p=work.receipt();assert!(p.copied_items<=1);assert!(p.copied_bytes<=1048576&&p.retained_capacity_bytes<=1048576);allocated+=p.retained_capacity_bytes;if state.prepared{break;}}
 assert!(state.prepared&&allocated>0);let Some(PaintShape::Path(path))=state.shape(0)else{panic!("missing original path")};assert_eq!(path.segments.len(),fixture["expected"]["segments"].as_u64().unwrap()as usize);assert_eq!(path.edges.len(),fixture["expected"]["edges"].as_u64().unwrap()as usize);for(i,want)in fixture["expected"]["bounds"].as_array().unwrap().iter().enumerate(){assert_eq!(path.bounds[i],want.as_f64().unwrap());}crate::close_original_fixture(state);crate::close_plan_fixture(&mut items);
}
