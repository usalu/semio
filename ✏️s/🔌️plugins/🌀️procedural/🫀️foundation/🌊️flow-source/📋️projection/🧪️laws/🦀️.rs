//! 🧪️ Projected native fields retain original custody under the finite original caller.
use super::retained_projected_clone::ProjectedFieldCloneWork;
use super::retained_host_scratch_laws::{observed,close,clock};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,close_original_retained_step,receive_original_retained_work,admit_original_construction,OriginalConstructionDemand};
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedOwnedProjection};
#[test]
fn projected_native_fields_preserve_original_receipts_and_prefix_custody(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();
 for case in cases.as_array().unwrap(){for cut in [None,Some(0),Some(1)]{
  let input=case["input"].as_str().unwrap().to_owned();
  let mut admission=RetainedCloneSource::<String,String>::admission_borrowed(input,|value|value);
  let mut source=None;let mut calls=0;
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(policy["operation"].as_u64().unwrap()),Generation(policy["generation"].as_u64().unwrap()),stock,clock,policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());
  let mut decoder=OriginalJobDecodeOwner::empty();
  while source.is_none(){calls+=1;assert!(calls<=200);caller.turn_with_decoder(stock,&mut decoder,|cx|observed(cx,|cx|{
   let turn=if admission.phase()=="ready"{admission.take_output(&mut source,cx.retained_grant())}else{admission.advance(cx.retained_grant())};
   receive_original_retained_work(cx,turn)?;Ok(())
  })).unwrap()}
  assert!(admission.terminal_is_empty());let mut source=source.unwrap();let address=source.try_borrow().unwrap().get().as_ptr();
  let mut projection=None;
  while projection.is_none(){calls+=1;assert!(calls<=200);caller.turn_with_decoder(stock,&mut decoder,|cx|observed(cx,|cx|{
   let d=RetainedOwnedProjection::<String>::constructor_demand();let grant=cx.retained_grant();
   admit_original_construction(cx,&mut projection,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|_|source.retain_original(grant))?;Ok(())
  })).unwrap()}
  let mut work=caller.turn_with_decoder(stock,&mut decoder,|cx|Ok(ProjectedFieldCloneWork::new(projection.take().unwrap(),cx))).unwrap();let mut turns=0;
  while !work.ready(){if cut==Some(turns){break}calls+=1;turns+=1;assert!(calls<=200);caller.turn_with_decoder(stock,&mut decoder,|cx|observed(cx,|cx|work.step(cx))).unwrap();}
  let mut output=None;
  if cut.is_none(){while output.is_none(){calls+=1;assert!(calls<=200);caller.turn_with_decoder(stock,&mut decoder,|cx|observed(cx,|cx|work.take_into(&mut output,cx))).unwrap();}assert_eq!(output.as_deref(),case["expected"].as_str())}
  assert_eq!(source.try_borrow().unwrap().get().as_ptr(),address);
  work.begin_close();while !work.terminal_is_empty(){calls+=1;assert!(calls<=200);caller.turn_with_decoder(stock,&mut decoder,|cx|observed(cx,|cx|cx.with_original_retirement(|cx|{
   for _ in 0..512{if work.terminal_is_empty()||cx.should_yield(){break}let d=work.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|work.close_step(g))?;if step.progress()==Default::default(){break}}Ok(())
  }))).unwrap()}
  if let Some(output)=output{close(output,&mut caller,&mut decoder,stock,&mut calls)}
  while !source.terminal_is_empty(){calls+=1;assert!(calls<=200);caller.turn_with_decoder(stock,&mut decoder,|cx|observed(cx,|cx|cx.with_original_retirement(|cx|{
   for _ in 0..512{if source.terminal_is_empty()||cx.should_yield(){break}let d=source.next_close_demand(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|source.close_step(g))?;if step.progress()==Default::default(){break}}Ok(())
  }))).unwrap()}
  while !decoder.terminal_is_empty(){calls+=1;assert!(calls<=200);caller.turn(stock,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})).unwrap()}
  assert_eq!(caller.stock(),stock);assert!(caller.cumulative_receipt().fits(stock));assert!(policy["fuel"].as_u64().unwrap()-caller.fuel_remaining()<32000);caller.retire(stock);
 }}
}
