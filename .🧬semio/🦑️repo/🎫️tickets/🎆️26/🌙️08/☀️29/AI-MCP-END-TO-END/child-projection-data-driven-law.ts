import{readFileSync,writeFileSync,renameSync,appendFileSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),f="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🧪️tests/📌️retain/🦀️.rs",s=readFileSync(f,"utf8"),a=s.indexOf("#[test]\nfn fallible_child_retention");if(a<0)throw Error("new native child law");
const n=s.slice(0,a)+`#[test]
fn fallible_child_retention_is_funded_before_accessor_and_preserves_original_alias(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in law["childCases"].as_array().unwrap(){
  let rows:Vec<String>=serde_json::from_value(law["rows"].clone()).unwrap();let rank=row["selected"].as_u64().unwrap()as usize;
  let mut bank=Bank::new();let mut root=source(rows,&mut bank);let g=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_depth:row["depth"].as_u64().unwrap()as usize,..grant(RetainedOwnedProjection::<String>::constructor_demand(),&bank)};let calls=std::cell::Cell::new(0);
  if row["retained"].as_bool().unwrap(){
   let(result,heap)=bank.run(g,||root.borrow().try_project_owned(rank,|rows|{calls.set(calls.get()+1);rows.get(rank).ok_or_else(||crate::ValueError::invariant("absent original child"))},g),|(_,p)|*p);
   let(mut retained,p)=result.unwrap();assert_eq!(heap,(0,0));assert!(p.fits(g));assert_eq!(retained.borrow().unwrap().get().as_str(),law["rows"][rank].as_str().unwrap());assert_eq!(retained.borrow().unwrap().get().as_ptr(),root.borrow().get()[rank].as_ptr());assert_eq!(retained.borrow().unwrap().source_identity(),root.borrow().source_identity());
   for _ in 0..10000{if retained.terminal_is_empty(){break}let g=grant(retained.close_demand(4096).unwrap(),&bank);let(result,_)=bank.run(g,||retained.close_step(g,&mut None),|s|s.progress());result.unwrap();}
   assert!(retained.terminal_is_empty());
  }else{
   let(result,heap)=observe_retirement_allocations(||root.borrow().try_project_owned(rank,|rows|{calls.set(calls.get()+1);rows.get(rank).ok_or_else(||crate::ValueError::invariant("absent original child"))},g));assert!(result.result.is_err());assert_eq!(result.work,0);assert_eq!(heap,(0,0));
  }
  assert_eq!(calls.get(),row["accessorCalls"].as_u64().unwrap()as usize);close_source(&mut root,&mut bank);
 }
}
`;
const q=join(t,"🗑️generated/child-data-driven-law.rs");writeFileSync(q,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent retain tests");renameSync(q,f);appendFileSync(join(t,"native-work-view-2026-11-06.md"),"\nChild projection neutral cases now explicitly include zero items, zero depth, successful row retention and an absent row. The native law consumes the same rows and expected accessor counts; the portable strict-schema/SQLite batch passes 2 tests /10 assertions. Root fixed the captured TestingAdmission E0631 by preserving OriginalRetirementFailure in the close receipt mapper. Latest expanded Root parser gate has no syntax errors; retirement cfg(test) is29 files/251882bytes/2340lines after real work-law factoring. These are authoring/parser/portable checks; no native test pass is claimed.\n");console.log("Native child law consumes exact neutral refusal cases");
