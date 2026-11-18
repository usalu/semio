/** 🔭️ Law: the take pass of a read computes nothing in its parent. A look is a job of a farm child (the assembly authority, the verdict on the stored record and the closure references of a batch of owners); the parent asks the look queue before it derives anything, takes native, pipeline and head from the answer, uses the verdict of the look for the record it holds, starts the queue with the lanes of the take pass and runs every look inside the look scope that counts what the parent computes. A child answers an owner it cannot look at as data without failing the others.
 * @see ../../🔭️look-queue/🧪️tests/🟦️.ts */
import{expect,test}from"bun:test";
import fs from"node:fs";const readFileSync=(path:string,encoding:"utf8")=>fs.readFileSync(path,encoding);
import{join}from"node:path";
import{lookOwnersV1}from"../../🟨️.mjs";

const source=readFileSync(join(import.meta.dir,"..","..","🟨️.mjs"),"utf8"),service=readFileSync(join(import.meta.dir,"..","..","🏊️project-farm","👷️service","🟨️.mjs"),"utf8");

test("the parent asks the look queue before it derives anything and takes native, pipeline and head from the look",()=>{
 const body=source.slice(source.indexOf("const inferProject=async("),source.indexOf("const loop=async end=>"));
 expect(body.indexOf("lookQueue.ask(name)")).toBeGreaterThan(0);
 expect(body.indexOf("lookQueue.ask(name)")).toBeLessThan(body.indexOf("projectAssemblyAuthorityV1("));
 expect(body.indexOf("lookQueue.ask(name)")).toBeLessThan(body.indexOf('existsSync(join(workspaceRoot,nativeRoot,"Cargo.toml"))'));
 expect(body).toContain("nativeCandidate=looked!==undefined?looked.native:existsSync(");
 expect(body).toContain("pipeline=looked!==undefined?looked.pipeline:");
 expect(body).toContain("assemblyAuthority=looked!==undefined?undefined:native||pipeline?");
 expect(body).toContain("head=looked!==undefined?looked.head:[");
 expect(body).toContain("STAGE_LOOK_V1");
});

test("the verdict of the look is the verdict of the record when it judged the record the parent holds, else a child is asked again",()=>{
 expect(source).toContain("if(looked?.verdict!==undefined&&looked.headerDigest===headerDigest)");
 expect(source).toContain("return Promise.resolve(looked);}stageV1(name,STAGE_VERDICT_V1);");
});

test("the take pass starts the queue with its lanes, every look runs in the look scope, the queue follows the children and a read says what its parent computed",()=>{
 expect(source).toContain("lookQueue?.start([...lanes,split.heavy])");
 expect(source).toContain("withTakeLookScopeV1(()=>runClosureTakeV1(()=>lookProject(job,takeAway)))");
 expect(source).toContain("farm?.onReady(()=>lookQueue?.kick())");
 expect(source).toContain("farm===undefined||options?.ownerAdmission?undefined:createLookQueueV1(");
 expect(source).toContain("readParentObservedCountsV1()");
 expect(service).toContain('if (job.look === true) return stagedWithinV1(join(staging, "look"), () => lookOwnersV1(environment, job));');
});

test("a child answers an owner it cannot look at as data, the others of the batch are looked at, and the reply is light",async()=>{
 const reply=await lookOwnersV1({} as never,{owners:[{name:"@semio-tech/a"},{name:"@semio-tech/b"}],globals:{}} as never);
 expect(reply.names).toEqual([]);expect(reply.looks.map((look:any)=>look.name)).toEqual(["@semio-tech/a","@semio-tech/b"]);
 for(const look of reply.looks)expect(typeof look.failed).toBe("string");
 expect(reply.weight).toBeCloseTo(0.02);expect(reply.cpuMilliseconds).toBeGreaterThanOrEqual(0);
});
