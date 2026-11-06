import fs from"node:fs";const root="🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress";const p=root+"/🧪️tests/🔬️unit/🦀️.rs",old=fs.readFileSync(p,"utf8");const law=`
#[test]
fn command_page_sources_return_real_backing_only_under_funded_retirement(){
 use semio_framework_value::{RetainedCloneGrant,retirement::controlled::ControlledRetirement};
 for count in 0..=2{
  let mut pages=CommandPageSet::try_new(2).unwrap();
  for _ in 0..count{pages.try_push(FixedCommandPage::try_copy_from(&[19]).unwrap()).unwrap();}
  let existing=pages.pages.capacity()*size_of::<FixedCommandPage>()+count*COMMAND_PAGE_MAXIMUM_BYTES;
  let mut owner=ControlledRetirement::retain(pages);let mut born=0;let mut released=0;
  for at in 0..10000{
   if owner.terminal_is_empty(){break}
   let d=owner.next_demand(4096).unwrap();
   let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)};
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(g));let p=result.unwrap().progress();
   assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));born+=p.retained_capacity_bytes;released+=p.released_bytes;assert!(at<9999);
  }
  assert!(owner.terminal_is_empty());assert_eq!(released,existing+born);
 }
}
`;if(fs.readFileSync(p,"utf8")!==old)throw Error("stale");fs.writeFileSync(import.meta.dir+"/🗑️generated/command-retirement-law.rs",old+law);fs.renameSync(import.meta.dir+"/🗑️generated/command-retirement-law.rs",p);
const file=root+"/🦀️.rs",before=fs.readFileSync(file,"utf8");let next=before;for(const type of["CommandPageSet","PagedCommand","PagedCommandReader"]){let needle="pub struct "+type+" {";if(next.split(needle).length!==2)throw Error("type seam "+type);next=next.replace(needle,"#[derive(semio_framework_value::RetireOwned)]\n#[value(crate=\"::semio_framework_value\")]\n"+needle);}next+='\n#[cfg(test)]\n#[global_allocator]\nstatic COMMAND_PAGE_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;\n';if(fs.readFileSync(file,"utf8")!==before)throw Error("stale root");fs.writeFileSync(import.meta.dir+"/🗑️generated/command-retirement.rs",next);fs.renameSync(import.meta.dir+"/🗑️generated/command-retirement.rs",file);
fs.appendFileSync(import.meta.dir+"/command-input-custody-2026-11-05.md","\nCommandPageSet, PagedCommand and PagedCommandReader now expose controlled owned retirement through the existing first-party derive. A new native allocator law measures real page backing (4096 bytes per page), actual deque capacity and every retirement scaffold against paid progress. The old ordinary page reader still has unfunded release methods and is not wired into a native Source owner; this new owned-retirement path is a required foundation, not proof of the final command path. Native laws remain uncompiled.\n");console.log("Added controlled command backing retirement and actual heap law.");
