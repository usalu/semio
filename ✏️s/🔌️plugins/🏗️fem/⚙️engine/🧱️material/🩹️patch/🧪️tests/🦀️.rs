use super::{BODY,MAXIMUM,close_source,denied,drain,driver,exact,paid,source,with_bank};
use super::super::FemMaterial;
use super::super::NATIVE_MATERIAL_DIMENSION;
use super::super::NATIVE_MATERIAL_COUNT;
use crate::patch::NativeMaterialPatch;
use crate::patch::NativeMaterialPatchCursor;
use semio_framework_value::{DslValue, RetainedWorkOutcome, OriginalRetirementFailure};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeReceive};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
#[derive(semio_framework_value::RetireOwned)]
struct Patch{name:Option<String>,numbers:[Option<f64>;NATIVE_MATERIAL_COUNT]}
impl NativeMaterialPatch<NATIVE_MATERIAL_COUNT> for Patch{fn name(&self)->Option<&String>{self.name.as_ref()}fn numbers(&self)->[Option<f64>;NATIVE_MATERIAL_COUNT]{self.numbers}}
fn row()->FemMaterial{let f:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let row=&f["accepted"].as_array().unwrap().iter().find(|c|c["dimension"].as_u64()==Some(NATIVE_MATERIAL_DIMENSION as u64)).unwrap()["row"];let original=source(DslValue::from(row));let mut receiver=<FemMaterial as NativeReceive<DslValue>>::receiver();let material=with_bank(|bank|driver::receive_cut(&mut receiver,original.borrow(),bank,BODY,None)).unwrap().expect("uncut receive publishes");with_bank(|bank|driver::close(&mut receiver,bank,BODY));drop(receiver);close_source(original);material}
#[test]
fn native_material_sparse_patch_preserves_original_field_authorities_and_all_cancelled_frontiers(){
 type Cursor=NativeMaterialPatchCursor<Patch,FemMaterial,NATIVE_MATERIAL_COUNT>;let f:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for case in f["cases"].as_array().unwrap(){let mut normal=0;for cut in 0..192{
  let make=||Patch{name:case["name"].as_str().map(str::to_owned),numbers:std::array::from_fn(|i|if i==0{case["e"].as_f64()}else{None})};
  let original=source(row());let patch=source(make());let foreign=source(make());let d=Cursor::birth_demand();for g in super::denied(d){let(result,h)=observe(||Cursor::begin(g));assert!(result.is_err());assert_eq!(h,(0,0));}let(mut cursor,_)=Cursor::begin(exact(d)).unwrap();let id=std::ptr::from_ref(original.borrow().get().id.get());let name=std::ptr::from_ref(original.borrow().get().name.get());let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut turns=0;
  while !cursor.ready()&&turns<cut{let d=cursor.demand(original.borrow(),patch.borrow(),BODY,MAXIMUM).unwrap();if turns>1{assert!(cursor.demand(original.borrow(),foreign.borrow(),BODY,MAXIMUM).is_err());}
   let mut advance=|g|with_bank(|bank|{let(outcome,_,heap)=driver::funded(bank,g,MAXIMUM,|effect|{let before=effect.performed_work();let result=cursor.advance(original.borrow(),patch.borrow(),&mut control,BODY,effect);RetainedWorkOutcome{result,work:effect.performed_work()-before}});(outcome.result.unwrap(),heap)});
   for g in denied(d){let(s,heap)=advance(g);assert_eq!(s.progress(),Default::default());assert_eq!(heap,(0,0));}
   let g=exact(d);let(s,heap)=advance(g);assert!(s.progress().fits(g));assert_eq!(heap,(s.progress().retained_capacity_bytes,s.progress().released_bytes));turns+=1;}
  if cursor.ready(){normal=turns;let d=cursor.take_demand().unwrap();for g in denied(d){assert!(cursor.take(g).unwrap().is_none());}let(output,p)=cursor.take(exact(d)).unwrap().unwrap();assert!(p.fits(exact(d)));assert_eq!(std::ptr::from_ref(output.get().id.get()),id);if case["name"].is_null(){assert_eq!(std::ptr::from_ref(output.get().name.get()),name);}else{assert_ne!(std::ptr::from_ref(output.get().name.get()),name);assert_eq!(output.get().name.as_str(),case["name"].as_str().unwrap());}assert_eq!(output.get().e,case["e"].as_f64().unwrap_or(original.borrow().get().e));assert_eq!(output.get().nu,original.borrow().get().nu);assert_eq!(output.get().rho,original.borrow().get().rho);drain(output);}
  cursor.begin_close();for _ in 0..10000{if cursor.terminal_is_empty(){break;}let d=cursor.close_demand(BODY).unwrap();let w=MAXIMUM;paid::<OriginalRetirementFailure>(d,w,&mut|effect,reply,recover|if recover{cursor.recover_close_step(BODY,effect,reply)}else{cursor.close_step(BODY,effect,reply)});}assert!(cursor.terminal_is_empty());let(_,h)=observe(||drop(cursor));assert_eq!(h,(0,0));close_source(foreign);close_source(patch);close_source(original);if normal>0&&cut>normal{break;}
 }assert!(normal>0);}
}
