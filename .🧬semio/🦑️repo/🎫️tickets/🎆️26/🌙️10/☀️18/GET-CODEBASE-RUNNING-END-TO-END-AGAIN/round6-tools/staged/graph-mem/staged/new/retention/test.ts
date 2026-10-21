import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

const api=await import(new URL("../../🟨️.mjs",import.meta.url).href) as typeof import("../../🟨️.mjs");
const memory=await import(new URL("../../../🧠️memory/🟨️.mjs",import.meta.url).href) as typeof import("../../../🧠️memory/🟨️.mjs");
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

/** 🧪️ One isolated workspace holding the fixture file with an optional oversize member, written long enough ago for the first result to be persisted when `age` exceeds the racy window. */
function workspace(age:number,oversize:boolean):{root:string;result:unknown}{
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});
 const root=mkdtempSync(join(realpathSync(parent),"retention-"));roots.push(root);
 const result=oversize?{...fixture.result,payload:"x".repeat(memory.GRAPH_MEMORY_POLICY_V1.retainedImageBytes+fixture.oversize.extraBytes)}:fixture.result;
 writeFileSync(join(root,fixture.file),JSON.stringify(result));
 if(age>0)Bun.sleepSync(age);
 return{root,result};
}

/** 🏃️ Runs the project inference twice in one process and reports the retained image bytes, the computations and both results. */
async function twice(root:string,key:string){
 const path=join(root,fixture.file),cachePath=join(root,"project.json");
 const compute=()=>api.observeProjectInferenceV1({cachePath,key,owner:root},async()=>JSON.parse(await api.observedFs.readFile(path,"utf8")));
 const first=await compute(),imageBytes=api.sourceObservationStatsV1().projectImageBytes,hitsAfterFirst=memory.graphStringInternerV1.stats().hits,second=await compute();
 return{first,second,imageBytes,hitsAfterFirst,computations:api.projectInferenceCountsV1()[root]};
}

test("the fixture is closed under its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
});

test("a retained project image is reused by a repeated round without recomputation and stays byte equal",async()=>{
 const{root,result}=workspace(fixture.persisted.ageMilliseconds,false),run=await twice(root,"persisted");
 expect(JSON.stringify(run.first)).toBe(JSON.stringify(result));expect(JSON.stringify(run.second)).toBe(JSON.stringify(result));
 expect(run.imageBytes).toBeGreaterThan(0);expect(run.computations).toBe(fixture.persisted.computations);
},20000);

test("an unpersisted small result is reused from its retained image",async()=>{
 const{root,result}=workspace(fixture.racy.ageMilliseconds,false),run=await twice(root,"racy");
 expect(JSON.stringify(run.second)).toBe(JSON.stringify(result));expect(run.computations).toBe(fixture.racy.computations);
},20000);

test("a result above the byte bound is never retained: a persisted one is read back from its record, an unpersisted one is recomputed",async()=>{
 const persisted=workspace(fixture.persisted.ageMilliseconds,true),first=await twice(persisted.root,"oversize-persisted");
 expect(first.imageBytes).toBe(0);expect(first.computations).toBe(fixture.oversize.persistedComputations);expect(JSON.stringify(first.second).length).toBe(JSON.stringify(persisted.result).length);
 const racy=workspace(0,true),second=await twice(racy.root,"oversize-racy");
 expect(second.imageBytes).toBe(0);expect(second.computations).toBe(fixture.oversize.racyComputations);expect(JSON.stringify(second.second).length).toBe(JSON.stringify(racy.result).length);
},60000);

test("equal long strings of project results are shared through the process table",async()=>{
 const before=memory.graphStringInternerV1.stats().hits,{root}=workspace(0,false),run=await twice(root,"shared");
 expect(run.hitsAfterFirst).toBeGreaterThan(before);
},20000);
