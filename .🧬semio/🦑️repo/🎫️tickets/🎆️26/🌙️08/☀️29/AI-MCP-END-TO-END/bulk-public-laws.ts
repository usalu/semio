import {readFileSync,writeFileSync,renameSync} from 'node:fs';
import {join,dirname} from 'node:path';
const root='🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/',file=root+'🚚️bulk-vec/🧪️tests/🦀️.rs',old=readFileSync(file,'utf8');
function at(s:string){const i=old.indexOf(s);if(i<0)throw Error('anchor '+s);return i}
const start=at('#[test]\nfn bulk_vector_work_quote'),capacity=at('#[test]\nfn bulk_vector_clone_capacity'),cancel=at('#[test]\nfn bulk_vector_clone_cancels'),array=at('#[test]\nfn bulk_array_zero');
const helperStart=at('fn turn_grant'),helperEnd=at('fn advance_one');
let prelude=old.slice(0,start).replace('use super::*;','use semio_framework_value_retained_clone::*;\nuse semio_framework_value_retained_clone::retirement::RetireOwned;\nuse semio_framework_value_clone_source_testing::{FixtureSource,FixtureSourceBirth};\nuse std::mem::size_of;').replace('use crate::observe_retirement_allocations as observe;','use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;').replaceAll('crate::RetainedWorkOutcome','RetainedWorkOutcome');
prelude=prelude.replace(old.slice(helperStart,helperEnd),`fn turn_grant<T:RetainedClone>(cursor:&VecCursor<T>,source:&Held<T>,_planned:usize,copy:usize)->RetainedCloneGrant{let d=cursor.advance_demands(source.borrow(),copy,4096).unwrap();RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:copy.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
`);
let publicText=prelude+old.slice(start,capacity)+old.slice(cancel,array);
publicText=publicText.replaceAll('cursor.advance_work_demand(source.borrow(),copy)','cursor.advance_work_demand(source.borrow(),copy,4096)').replaceAll('cursor.advance(source.borrow(), copy, grant)','cursor.advance(source.borrow(), copy,4096, grant)').replaceAll('cursor.advance_demands(original.borrow(),4096)','cursor.advance_demands(original.borrow(),4096,4096)').replaceAll('cursor.advance_work_demand(foreign.borrow(),4096)','cursor.advance_work_demand(foreign.borrow(),4096,4096)');
const publicFile=root+'🚚️bulk-vec/🧪️tests/public.rs';writeFileSync(publicFile,publicText);
const next=old.slice(0,start)+old.slice(capacity,cancel)+old.slice(array),tmp=join(dirname(import.meta.path),'🗑️generated/bulk-laws-'+Date.now()+'.rs');writeFileSync(tmp,next);if(readFileSync(file,'utf8')!==old)throw Error('concurrent');renameSync(tmp,file);
const manifest=root+'🧬️contract/📦️packages/🦀️rust/Cargo.toml',before=readFileSync(manifest,'utf8');writeFileSync(tmp,before+'\n[[test]]\nname = "original-bulk-vector-public"\npath = "../../../🚚️bulk-vec/🧪️tests/public.rs"\n');if(readFileSync(manifest,'utf8')!==before)throw Error('concurrent manifest');renameSync(tmp,manifest);
console.log(JSON.stringify({removedFromInline:Buffer.byteLength(old)-Buffer.byteLength(next),publicTests:(publicText.match(/#\[test\]/g)||[]).length}));
