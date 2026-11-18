/** 🪆️ Contract: a derived question that needs a TypeScript program or a derived walk is asked in a child process, one at a time, and never in the process that is already that child; the policy is closed under its schema; a child that cannot answer fails with a named code instead of hanging. */
import{expect,test}from"bun:test";
import{resolve}from"node:path";
import{isRepoErrorV1,renderRepoErrorV1}from"../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json";
import policy from"../🔣️.json";
import{HEAVY_QUESTION_POLICY_V1,askHeavyQuestionV1,isHeavyQuestionV1,preflightToolClosuresInChildV1,primeSharedClosuresInChildV1,refusedToolClosuresV1}from"../🟨️.mjs";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

test("the policy follows its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(schema,policy),JSON.stringify(ajv.errors)).toBe(true);
 expect(HEAVY_QUESTION_POLICY_V1).toEqual(policy as never);
});

test("programs and derived walks are heavy, command walks are not, and the child itself never routes again",()=>{
 const row=(question:unknown)=>({kind:"derived",path:JSON.stringify(question),fact:"x"});
 expect(isHeavyQuestionV1(row({program:1,workspaceRoot:"/w",projectRoot:"p",projects:[]}),{})).toBe(true);
 expect(isHeavyQuestionV1(row({derive:1,workspaceRoot:"/w",recipe:{}}),{})).toBe(true);
 expect(isHeavyQuestionV1(row({walk:1,roots:[],resources:[],selectors:[]}),{})).toBe(false);
 expect(isHeavyQuestionV1({path:"not json"},{})).toBe(false);
 expect(isHeavyQuestionV1(row({program:1}),{[HEAVY_QUESTION_POLICY_V1.environment]:"2560"})).toBe(false);
});

test("a child that cannot answer fails with a bilingual refusal that names the exit status and the error line, and questions are asked one at a time",async()=>{
 const row={kind:"derived",path:JSON.stringify({derive:1,workspaceRoot:"/nonexistent-semio-root",recipe:{fn:"none"}}),fact:"x"};
 const outcomes=await Promise.allSettled([askHeavyQuestionV1(row,{workspaceRoot:"/nonexistent-semio-root"}),askHeavyQuestionV1(row,{workspaceRoot:"/nonexistent-semio-root"})]);
 for(const outcome of outcomes){
  expect(outcome.status).toBe("rejected");
  const reason=(outcome as PromiseRejectedResult).reason;
  expect(isRepoErrorV1(reason)).toBe(true);
  expect(reason.key).toBe("build.graph.heavy-question-failed");
  expect(reason.values.exit).toBe("1");expect(String(reason.values.detail).length).toBeGreaterThan(1);
  expect(renderRepoErrorV1(reason,"en")).toContain("exit 1");expect(renderRepoErrorV1(reason,"de")).toContain("Exit 1");
 }
},120000);

test("a refusal raised in the child reaches the parent with its code, key and values and renders in both languages (the lost message of the 17:11 failure)",async()=>{
 const root=resolve(import.meta.dir,"../".repeat(11)),row={kind:"derived",path:JSON.stringify({derive:1,workspaceRoot:root,recipe:{fn:"no-such-recipe"}}),fact:"x"};
 const reason=await askHeavyQuestionV1(row,{workspaceRoot:root}).then(()=>undefined,(error:unknown)=>error as any);
 expect(isRepoErrorV1(reason)).toBe(true);
 expect(reason.code).toBe("invalid");expect(reason.key).toBe("build.closure.derived-question-unknown");expect(String(reason.values.question)).toContain("no-such-recipe");
 const english=renderRepoErrorV1(reason,"en"),german=renderRepoErrorV1(reason,"de");
 expect(english).toContain("no-such-recipe");expect(german).toContain("no-such-recipe");expect(german).not.toBe(english);
},300000);

test("one refused tool closure is raised as itself and several as one refusal that names the count and every tool with its key and values, in both languages",()=>{
 const row=(name:string,targets:string)=>({name,version:1,code:"invalid",key:"build.closure.provider-generated-import",values:{port:name,targets}});
 const one=refusedToolClosuresV1([row("alpha","p:generate")]);
 expect(one.key).toBe("build.closure.provider-generated-import");expect(one.values).toEqual({port:"alpha",targets:"p:generate"});
 const many=refusedToolClosuresV1([row("alpha","p:generate"),row("beta","q:generate")]);
 expect(many.key).toBe("build.closure.tool-closures-refused");expect(many.values.count).toBe("2");
 for(const language of ["en","de"]){const text=renderRepoErrorV1(many,language);for(const part of ["alpha","beta","p:generate","q:generate","provider-generated-import"])expect(text).toContain(part);}
});

test("the preflight of the capped child accepts the real tree: every declared tool closure and provider port is derivable",async()=>{
 const root=resolve(import.meta.dir,"../".repeat(11)),answer=await preflightToolClosuresInChildV1({workspaceRoot:root});
 expect(answer.checked).toBeGreaterThan(30);
},600000);

const fakeChild=(directory:string,name:string,body:string)=>{const file=join(directory,name);writeFileSync(file,'const[,,root,output]=process.argv;const fs=require("node:fs");const beat=()=>process.stderr.write("\\u0001alive\\n");'+body);return file;};
const FAST={...HEAVY_QUESTION_POLICY_V1,silenceMilliseconds:1500,timeoutMilliseconds:60000};
const QUESTION={kind:"derived",path:"{}",fact:"x"};

test("law: a child that writes nothing is stopped after the silence, with a bilingual refusal, not after the wall limit",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-silent-"));
 try{
  const script=fakeChild(directory,"silent.cjs","setInterval(()=>{},1000);");
  const began=Date.now(),reason=await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,script}).then(()=>undefined,(error:unknown)=>error as any);
  expect(Date.now()-began).toBeLessThan(10000);
  expect(isRepoErrorV1(reason)).toBe(true);expect(reason.key).toBe("build.graph.heavy-question-silent");
  expect(renderRepoErrorV1(reason,"en")).toContain("nothing for 2 s");expect(renderRepoErrorV1(reason,"de")).toContain("nichts geschrieben");
 }finally{rmSync(directory,{recursive:true,force:true});}
},30000);

test("law: a child that keeps its life line outlives the silence, and its answer is taken",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-alive-"));
 try{
  const script=fakeChild(directory,"alive.cjs","const timer=setInterval(beat,300);setTimeout(()=>{clearInterval(timer);fs.writeFileSync(output,JSON.stringify({fact:1,rows:[]}));process.exit(0);},4000);");
  expect(await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,script})).toEqual({fact:1,rows:[]});
 }finally{rmSync(directory,{recursive:true,force:true});}
},30000);

test("law: a long prime does not hold a heavy question back (two lanes), and two primes still run one after the other",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-lanes-"));
 try{
  const slow=fakeChild(directory,"slow.cjs","const timer=setInterval(beat,300);setTimeout(()=>{clearInterval(timer);fs.writeFileSync(output,JSON.stringify({done:Date.now()}));process.exit(0);},3000);");
  const quick=fakeChild(directory,"quick.cjs","fs.writeFileSync(output,JSON.stringify({fact:2,rows:[]}));process.exit(0);");
  const began=Date.now(),prime=primeSharedClosuresInChildV1({workspaceRoot:directory,storeDirectory:directory,policy:FAST,script:slow}),second=primeSharedClosuresInChildV1({workspaceRoot:directory,storeDirectory:directory,policy:FAST,script:slow});
  const question=await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,script:quick});
  expect(question).toEqual({fact:2,rows:[]});expect(Date.now()-began,"the question did not wait for the prime").toBeLessThan(2500);
  const first=await prime,next=await second;expect(next.done-first.done,"the second prime started after the first ended").toBeGreaterThanOrEqual(2500);
 }finally{rmSync(directory,{recursive:true,force:true});}
},40000);

test("law: the prime of a workspace returns after the preflight and runs on beside the caller, a failing prime never fails the caller, a refused preflight still does",async()=>{
 const inference=await import("../../../🟨️.mjs") as any;await inference.libraryBootstrap;
 const calls:string[]=[];let release:(value:unknown)=>void=()=>undefined;
 const slow=new Promise(resolveSlow=>{release=resolveSlow;});
 const ports={preflight:async()=>{calls.push("preflight");},storeOf:()=>"/store",prime:()=>{calls.push("prime");return slow;}};
 const answer=await inference.primeSharedClosuresOfWorkspaceV1({workspaceRoot:"/w",ports});
 let ended=false;answer.run.then(()=>{ended=true;});
 await new Promise(resolveTimer=>setTimeout(resolveTimer,30));
 expect(calls).toEqual(["preflight","prime"]);expect(ended,"the caller did not wait for the prime").toBe(false);
 release({derived:1});await answer.run;expect(ended).toBe(true);
 const failing=await inference.primeSharedClosuresOfWorkspaceV1({workspaceRoot:"/w",ports:{...ports,prime:async()=>{throw Error("child failed");}}});
 expect(await failing.run).toBeUndefined();
 await expect(inference.primeSharedClosuresOfWorkspaceV1({workspaceRoot:"/w",ports:{...ports,preflight:async()=>{throw Error("refused");}}})).rejects.toThrow("refused");
 expect((await inference.primeSharedClosuresOfWorkspaceV1({workspaceRoot:"/w",ports:{...ports,storeOf:()=>undefined}})).run).toBeInstanceOf(Promise);
},120000);
