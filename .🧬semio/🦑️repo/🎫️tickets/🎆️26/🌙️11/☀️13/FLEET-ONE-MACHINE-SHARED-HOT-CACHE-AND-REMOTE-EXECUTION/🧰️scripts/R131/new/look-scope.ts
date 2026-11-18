/** 🔭️ Law: inside a look of the take pass the parent of a read with a farm runs no observed computation. The assembly authority refuses with the kind `observe`; every memoized observed computation and every observed record that runs there is counted, and nothing is counted outside a look, in an unmarked process or once the farm is gone; the count is what the read reports and the law test demands to stay zero.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{assertNotReadParentV1,markReadParentV1,readParentObservedCountsV1,withTakeLookScopeV1}from"../../🟨️.mjs";
import{memoizedObservedComputationV1,observedFs}from"../../../🟨️.mjs";
import{projectAssemblyAuthorityV1}from"../../../../🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs";
import{createLookQueueV1}from"../../../../🕸️graph/📥️inference/📦️project-nodes/🔭️look-queue/🟨️.mjs";

const delta=(work:()=>unknown)=>{const before=readParentObservedCountsV1();work();const after=readParentObservedCountsV1();return{computations:after.computations-before.computations,records:after.records-before.records};};
const visit=(key:string)=>{memoizedObservedComputationV1(new Map(),key,()=>1);observedFs.existsSync("/");};

test("the assembly authority is refused in the parent of a read with a farm and computes where no farm is",()=>{
 expect(()=>projectAssemblyAuthorityV1("/w",undefined,new Map(),null)).not.toThrow();
 let farm=true;const unmark=markReadParentV1(()=>farm);
 let caught:any;try{projectAssemblyAuthorityV1("/w","crates/x",new Map(),null);}catch(error){caught=error;}
 expect(caught?.code).toBe("graph.read-parent-computes");expect(caught.kind).toBe("observe");expect(caught.what).toContain("crates/x");
 expect(()=>assertNotReadParentV1("observe","x")).toThrow();
 farm=false;expect(()=>projectAssemblyAuthorityV1("/w",undefined,new Map(),null)).not.toThrow();unmark();
});

test("observed computations and records are counted inside a look of a marked parent and nowhere else",()=>{
 expect(delta(()=>visit("a")),"unmarked").toEqual({computations:0,records:0});
 let farm=true;const unmark=markReadParentV1(()=>farm);
 expect(delta(()=>visit("b")),"marked, outside a look").toEqual({computations:0,records:0});
 const inside=delta(()=>withTakeLookScopeV1(()=>visit("c")));
 expect(inside.computations).toBe(1);expect(inside.records).toBeGreaterThanOrEqual(1);
 farm=false;expect(delta(()=>withTakeLookScopeV1(()=>visit("d"))),"the farm is gone").toEqual({computations:0,records:0});
 unmark();
});

test("LAW: the looks of 200 owners over 4 children leave the observed count of the parent at zero, while the old look in the parent does not",async()=>{
 const unmark=markReadParentV1(()=>true),names=Array.from({length:200},(_,at)=>"o"+at);
 const old=delta(()=>{for(const name of names.slice(0,5))withTakeLookScopeV1(()=>visit(name));});
 expect(old.computations,"the old look computed in the parent").toBe(5);
 const farm={width:()=>4,observe:async(job:any)=>{await Bun.sleep(5);return{looks:job.owners.map((owner:any)=>({name:owner.name,native:true,pipeline:false,head:[owner.name],verdict:"holds",headerDigest:"d"}))};}};
 const queue=createLookQueueV1({farm:farm as never,jobOf:name=>({name}),refuse:(name,reason)=>Error(name+reason)});
 const seen=await (async()=>{const before=readParentObservedCountsV1();queue.start([names]);await Promise.all(names.map(name=>withTakeLookScopeV1(()=>queue.ask(name))));const after=readParentObservedCountsV1();return{computations:after.computations-before.computations,records:after.records-before.records};})();
 expect(seen).toEqual({computations:0,records:0});unmark();
});
