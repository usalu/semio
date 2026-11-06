import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),f="🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/🦀️.rs",s=readFileSync(f,"utf8"),a=s.indexOf("pub fn close_owned<");if(a<0)throw Error("closehelper");const n=s.slice(0,a)+`pub fn close_owned<T:RetireOwned>(value:T,wallet:&mut Wallet,body:usize,limit:usize){
 let(mut owner,h)=semio_framework_value_testing_allocator::observe_retirement_allocations(||crate::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|_|panic!("native fixture supports paid original closure")));assert_eq!(h,(0,0));let mut actual=None;
 for _ in 0..limit{
  if actual.is_some(){let d=RetirementDemand{copy_bytes:std::mem::size_of::<Option<usize>>(),depth:1,..Default::default()};let g=wallet.loan(1,body);assert!(retained_clone::retained_clone_grant_admits(g,d));assert!(wallet.work_remaining()>=1);actual=None;wallet.post(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()},1,g,(0,0));continue}
  if owner.terminal_is_empty(){return}
  let d=owner.next_demand(body).unwrap();let quoted=owner.next_computational_work(body).unwrap();assert!(quoted<=wallet.work_remaining());assert!(d.copy_bytes<=body);let g=wallet.loan(1,body);
  let(result,h)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.step(g,&mut actual));let work=owner.last_turn_work();assert!(work<=quoted);if let Some(bytes)=actual{assert_eq!(bytes,h.0)}else{assert_eq!(h.0,0)}
  post_step(wallet,crate::RetainedWorkOutcome{result,work},g,h,crate::OriginalRetirementFailure::retained_progress).unwrap();
 }
 panic!("native fixture did not close in its original turn bound")
}
`;const q=join(t,"🗑️generated/testing-close-marker.rs");writeFileSync(q,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent helper");renameSync(q,f);console.log("Testing close retains, validates and separately clears the actual allocator marker");
