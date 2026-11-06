import fs from"node:fs";const p="🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🧪️tests/🔬️unit/🦀️.rs";const old=fs.readFileSync(p,"utf8");let next=old;const marker='assert_eq!(pages.as_ref().unwrap().pages.back().unwrap().as_slice().as_ptr(),pointer);\n}';if(next.split(marker).length!==2)throw Error("cleanup seam");next=next.replace(marker,`assert_eq!(pages.as_ref().unwrap().pages.back().unwrap().as_slice().as_ptr(),pointer);
 let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::retain(pages.take().unwrap());
 for at in 0..10000{
  if close.terminal_is_empty(){break}
  let d=close.next_demand(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)};
  let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close.step(g));let p=turn.unwrap().progress();assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(at<9999);
 }
 assert!(close.terminal_is_empty());
}`);if(fs.readFileSync(p,"utf8")!==old)throw Error("stale");fs.writeFileSync(import.meta.dir+"/🗑️generated/command-source-cleanup-law.rs",next);fs.renameSync(import.meta.dir+"/🗑️generated/command-source-cleanup-law.rs",p);console.log("Native Source page law now fully retires actual backing under grants.");