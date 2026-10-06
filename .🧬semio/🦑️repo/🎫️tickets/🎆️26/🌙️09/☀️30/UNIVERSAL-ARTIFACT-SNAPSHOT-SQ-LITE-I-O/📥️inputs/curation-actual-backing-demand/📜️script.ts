import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
const root="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot",pairs:{path:string;before:string|null;after:string}[]=[];
const fixture=join(root,"🧫️fixtures/🪶️sqlite/📏️semantic-cells/🔣️.json"),before=readFileSync(fixture,"utf8"),law=JSON.parse(before);law.nativeBacking={maximumBytes:1048576,repeatedOwnerships:2,tinyAllocationBytes:[0,1],requestIncludesReallocFullSize:true};pairs.push({path:fixture,before,after:JSON.stringify(law,null,2)+"\n"});
const schema=join(fixture,"../🧬️schema/🔣️.json"),schemaBefore=readFileSync(schema,"utf8"),value=JSON.parse(schemaBefore);value.const=law;pairs.push({path:schema,before:schemaBefore,after:JSON.stringify(value,null,2)+"\n"});
const allocator=String.raw`//! 🔬️ Curation test requests are independently delegated to the system allocator.
use std::{alloc::{GlobalAlloc,Layout,System},cell::Cell};
std::thread_local!{static ACTIVE:Cell<bool>=const{Cell::new(false)};static REQUESTED:Cell<usize>=const{Cell::new(0)};}
fn request(bytes:usize){let _=ACTIVE.try_with(|active|{if active.get(){let _=REQUESTED.try_with(|requested|requested.set(requested.get().checked_add(bytes).expect("bounded Curation test requests")));}});}
struct Allocator;
unsafe impl GlobalAlloc for Allocator{
 unsafe fn alloc(&self,layout:Layout)->*mut u8{request(layout.size());unsafe{System.alloc(layout)}}
 unsafe fn alloc_zeroed(&self,layout:Layout)->*mut u8{request(layout.size());unsafe{System.alloc_zeroed(layout)}}
 unsafe fn realloc(&self,pointer:*mut u8,layout:Layout,size:usize)->*mut u8{request(size);unsafe{System.realloc(pointer,layout,size)}}
 unsafe fn dealloc(&self,pointer:*mut u8,layout:Layout){unsafe{System.dealloc(pointer,layout)}}
}
#[global_allocator]static SYSTEM_REQUESTS:Allocator=Allocator;
struct Restore;
impl Drop for Restore{fn drop(&mut self){ACTIVE.with(|active|active.set(false));}}
pub(super) fn observe<T>(operation:impl FnOnce()->T)->(T,usize){ACTIVE.with(|active|assert!(!active.get(),"non-nested Curation allocation observation"));REQUESTED.with(|requested|requested.set(0));ACTIVE.with(|active|active.set(true));let restore=Restore;let result=operation();drop(restore);(result,REQUESTED.with(Cell::get))}
`;
const allocatorPath=join(root,"🧪️tests/🪶️sqlite/💰️backing/🦀️.rs");let allocatorBefore:string|null=null;try{allocatorBefore=readFileSync(allocatorPath,"utf8");}catch{}assert.equal(allocatorBefore,null);pairs.push({path:allocatorPath,before:null,after:allocator});
const native=join(root,"🧪️tests/🪶️sqlite/🦀️.rs"),nativeBefore=readFileSync(native,"utf8");const nativeLaw=String.raw`
#[path="💰️backing/🦀️.rs"]mod allocation;
#[test]
fn sqlite_snapshot_curation_reconstruction_pays_actual_system_requests_cumulatively(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📏️semantic-cells/🔣️.json")).unwrap();let backing=&law["nativeBacking"];assert_eq!(backing["requestIncludesReallocFullSize"],true);let maximum=usize::try_from(backing["maximumBytes"].as_u64().unwrap()).unwrap();let expected=fixture();let database=database(&expected);let defaults=SqliteDatabaseLimits{max_allocation_bytes:maximum,..SqliteDatabaseLimits::default()};let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,defaults);
 let(result,requested)=allocation::observe(||CurationSnapshot::from_sqlite_database(&database,&mut control));assert_eq!(result.unwrap(),expected);let admitted=maximum-control.allocation_remaining_bytes();println!("[DEBUG] Curation actual system backing requests={} caller_admitted={} semantic_cells_separate=true",requested,admitted);assert!(admitted>0,"real Curation indexes and typed allocations must settle in the caller ledger");assert_eq!(admitted,requested,"every complete source/typed backing request must be admitted");
 for allowance in[admitted,admitted-1,0,1]{let limits=SqliteDatabaseLimits{max_allocation_bytes:allowance,..defaults};let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);let(result,requested)=allocation::observe(||CurationSnapshot::from_sqlite_database(&database,&mut control));if allowance==admitted{assert_eq!(result.unwrap(),expected);assert_eq!(requested,admitted);assert_eq!(control.allocation_remaining_bytes(),0);}else{assert_eq!(result.unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert!(control.allocation_remaining_bytes()<=allowance);}}
 let repetitions=usize::try_from(backing["repeatedOwnerships"].as_u64().unwrap()).unwrap();assert_eq!(repetitions,2);let total=admitted.checked_mul(repetitions).unwrap();let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:total,..defaults});for _ in 0..repetitions{let(result,requested)=allocation::observe(||CurationSnapshot::from_sqlite_database(&database,&mut control));assert_eq!(result.unwrap(),expected);assert_eq!(requested,admitted);}assert_eq!(control.allocation_remaining_bytes(),0);assert_eq!(CurationSnapshot::from_sqlite_database(&database,&mut control).unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
}
`;
pairs.push({path:native,before:nativeBefore,after:nativeBefore+nativeLaw});writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify(pairs,null,2)+"\n");for(const pair of pairs){let current:string|null=null;try{current=readFileSync(pair.path,"utf8");}catch{}assert.equal(current,pair.before,"Concurrent Curation demand owner change");}for(const pair of pairs){mkdirSync(join(pair.path,".."),{recursive:true});writeFileSync(pair.path,pair.after);}console.log("[DEBUG] Curation actual system allocation and cumulative/exact/short/retired ownership Native law mounted paths="+pairs.length+" production_mutations=0");
