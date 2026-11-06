import{readFileSync,writeFileSync,renameSync,mkdirSync,existsSync,appendFileSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),b="🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🪑️inline/";
function put(f:string,s:string){if(existsSync(f))throw Error("exists "+f);mkdirSync(dirname(f),{recursive:true});writeFileSync(f,s)}
const law={contract:"semio.original-inline-retirement/v1",text:"original β🙂",numbers:[3,5,8],kinds:["scalar","utf8","vector","tuple","array","struct"],maximumTurns:4096,stock:{items:10000,copyBytes:1048576,capacityBytes:1048576,releaseBytes:1048576,depth:128,work:100000},expected:{values:3,utf8Bytes:15}};
put(b+"📐️.json",JSON.stringify(law)+"\n");
put(b+"🔣️.json",JSON.stringify({$schema:"https://json-schema.org/draft/2020-12/schema",type:"object",required:Object.keys(law),additionalProperties:false,properties:{contract:{const:law.contract},text:{type:"string"},numbers:{type:"array",items:{type:"integer"}},kinds:{type:"array",items:{enum:law.kinds},minItems:6,maxItems:6,uniqueItems:true},maximumTurns:{type:"integer",minimum:1},stock:{type:"object",required:Object.keys(law.stock),additionalProperties:false,properties:Object.fromEntries(Object.keys(law.stock).map(k=>[k,{type:"integer",minimum:1}]))},expected:{type:"object",required:["values","utf8Bytes"],additionalProperties:false,properties:{values:{type:"integer",minimum:0},utf8Bytes:{type:"integer",minimum:0}}}}})+"\n");
put(b+"🟦️.ts",`import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv/dist/2020";
import schema from"./🔣️.json";
import law from"./📐️.json";
test("inline retirement inputs agree with the independent SQLite value and UTF8 oracle",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{const row=db.query("SELECT json_array_length(?) AS values_count,length(CAST(? AS BLOB)) AS bytes").get(JSON.stringify(law.numbers),law.text)as{values_count:number,bytes:number};expect(row.values_count).toBe(law.expected.values);expect(row.bytes).toBe(law.expected.utf8Bytes);}finally{db.close();}
});
`);
put(b+"🦀️.rs",`//! 🪑️ Concrete primitive cursors preserve the standard allocator's original release conservation.
use semio_framework_value_retirement::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
use semio_framework_value_retirement::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
struct InlineFields{text:String,numbers:Vec<u64>}
semio_framework_value_retirement::artifact_retire_struct!(InlineFields{text,numbers});
struct Bank{grant:RetainedCloneGrant,work:u64,turns:usize,maximum:usize,born:usize,released:usize}
impl Bank{
 fn close(&mut self,cursor:&mut dyn RetirementCursor){
  while !cursor.terminal_is_empty(){
   assert!(self.turns<self.maximum);self.turns+=1;
   let d=cursor.next_demand(4096).unwrap();let q=cursor.next_computational_work(4096).unwrap();assert!(q<=self.work);let g=self.grant;assert!(d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth);
   let mut actual=None;let(step,heap)=observe_retirement_allocations(||cursor.close_step(RetainedCloneGrant{maximum_items:1,..g},&mut actual));assert_eq!(actual.unwrap_or(0),heap.0);
   let work=step.work();assert!(work<=q);self.work-=work;self.born+=heap.0;self.released+=heap.1;
   let(progress,child)=match step{RetirementStep::Progress(p,_)|RetirementStep::ChildProgress(_,p,_) if false=>(p,None),RetirementStep::Progress(p,_)=>(p,None),RetirementStep::ChildProgress(child,p,_)=>(p,Some(child)),RetirementStep::Child(child)=>(Default::default(),Some(child)),RetirementStep::Bytes(n)=>(RetainedCloneProgress{copied_items:1,released_bytes:n,..Default::default()},None),RetirementStep::ProcessedBytes(n)=>(RetainedCloneProgress{copied_items:1,copied_bytes:n,..Default::default()},None),RetirementStep::Advanced=>(RetainedCloneProgress{copied_items:1,..Default::default()},None),RetirementStep::Complete(_)=>(Default::default(),None),RetirementStep::Failure(error,_)=>panic!("original inline retirement refused: {error:?}"),RetirementStep::BudgetExhausted=>panic!("quoted original inline retirement did not advance")};
   assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);self.post(progress);
   if let Some(mut child)=child{self.close(child.as_mut());let release=child.terminal_release_bytes().unwrap();assert!(release<=self.grant.maximum_release_bytes);assert!(self.work>0);self.work-=1;let(_,heap)=observe_retirement_allocations(||drop(child));assert_eq!(heap,(0,release));self.released+=release;self.post(RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()});}
  }
 }
 fn post(&mut self,p:RetainedCloneProgress){assert!(p.fits(self.grant));self.grant.maximum_items-=p.copied_items;self.grant.maximum_copy_bytes-=p.copied_bytes;self.grant.maximum_capacity_bytes-=p.retained_capacity_bytes;self.grant.maximum_release_bytes-=p.released_bytes;}
}
fn check<T:RetireOwned>(factory:impl Fn()->T,law:&serde_json::Value){
 let oracle=factory();let(_,expected)=observe_retirement_allocations(||drop(oracle));assert_eq!(expected.0,0);
 let original=factory();let(mut cursor,heap)=observe_retirement_allocations(||original.retirement());assert_eq!(heap,(0,0));
 let stock=&law["stock"];let n=|k|stock[k].as_u64().unwrap()as usize;let mut bank=Bank{grant:RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copyBytes"),maximum_capacity_bytes:n("capacityBytes"),maximum_release_bytes:n("releaseBytes"),maximum_depth:n("depth")},work:n("work")as u64,turns:0,maximum:law["maximumTurns"].as_u64().unwrap()as usize,born:0,released:0};
 bank.close(&mut cursor);assert_eq!(bank.released.checked_sub(bank.born).unwrap(),expected.1);assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));
}
#[test]fn primitive_inline_cursors_allocate_only_on_funded_original_child_turns(){
 let law:serde_json::Value=serde_json::from_str(include_str!("📐️.json")).unwrap();assert_eq!(law["text"].as_str().unwrap().len(),law["expected"]["utf8Bytes"].as_u64().unwrap()as usize);
 for kind in law["kinds"].as_array().unwrap(){match kind.as_str().unwrap(){"scalar"=>check(||73_u64,&law),"utf8"=>check(||law["text"].as_str().unwrap().to_owned(),&law),"vector"=>check(||serde_json::from_value::<Vec<u64>>(law["numbers"].clone()).unwrap(),&law),"tuple"=>check(||(law["text"].as_str().unwrap().to_owned(),73_u64),&law),"array"=>check(||[3_u64,5,8],&law),"struct"=>check(||InlineFields{text:law["text"].as_str().unwrap().to_owned(),numbers:serde_json::from_value(law["numbers"].clone()).unwrap()},&law),_=>unreachable!()}}
}
`.replace('RetirementStep::Progress(p,_)|RetirementStep::ChildProgress(_,p,_) if false=>(p,None),','').replace(",RetirementDemand",""));
const f="🧰️framework/🔨️modules/🌱️value/♻️retirement/📦️packages/🦀️rust/Cargo.toml",s=readFileSync(f,"utf8"),n=s+'\n[[test]]\nname = "original-inline-retirement"\npath = "../../🧪️tests/🪑️inline/🦀️.rs"\n',q=join(t,"🗑️generated/inline-law-manifest.toml");writeFileSync(q,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent manifest");renameSync(q,f);
appendFileSync(join(t,"native-work-view-2026-11-06.md"),"\nA native integration law original-inline-retirement now compares six concrete cursor families (scalar, UTF8, vector, tuple, array, struct) against real standard-drop allocator releases. It asserts constructor allocation zero, finite actual work and physical budgets, marker/heap equality on each child turn, funded child cursor release and net original allocation conservation. The shared neutral fixture has an independent SQLite input oracle. This law is authored; no native run is claimed.\n");console.log("Six-family native actual allocator law and neutral fixture authored");
