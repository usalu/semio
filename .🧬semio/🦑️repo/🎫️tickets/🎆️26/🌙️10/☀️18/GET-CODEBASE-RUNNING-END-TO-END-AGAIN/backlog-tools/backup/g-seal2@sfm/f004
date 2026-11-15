/** 🚦️ Contract of the front door's trampoline with real processes: it starts the front door from the stored snapshot, so a live edit that does not parse (or a live edit at all) does not reach a started command; `live` runs the live door; a store without a snapshot runs the live door; a sealed door that ends before it reported its start (a snapshot that no longer loads) says so in English and German and runs the live door; the exit code and the stop signals pass through. */
import{afterAll,expect,test}from"bun:test";
import{spawn}from"node:child_process";
import{chmodSync,rmSync,statSync,writeFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{findBuildDoorRootV1}from"../../../🚪️door/🟦️.ts";
import doorPolicy from"../../../🚪️door/🔣️.json";
import{findSealedRootV1,implementationModeV1,type ImplementationModeV1,sealedLightCommandV1,readSealedPointerV1,sealedEntryOfV1}from"../../🟦️.ts";
import{sealedDoorRoleV1,sealedKernelsV1}from"../../🚪️entry/🟦️.ts";
import{resolveSealedImplementationV1}from"../../📤️promotion/🟦️.ts";
import policyData from"../../🔣️.json";
import kernelSchema from"../../../⌨️command/🧭️selection/🧬️schema/🔣️.json";
import{fixtureCaptureV1,fixtureEntryV1,fixtureLoaderV1,makeFixtureWorkspaceV1,setFixtureTreeV1,testPolicyV1}from"../🧫️fixtures/🟦️.ts";

const script=fileURLToPath(new URL("../🧫️fixtures/📜️trampoline.ts",import.meta.url)),roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const promote=async(root:string)=>{const entry=sealedEntryOfV1(root,fixtureEntryV1);return resolveSealedImplementationV1(root,entry,{capture:fixtureCaptureV1(root),loader:fixtureLoaderV1,policy:testPolicyV1});};
const run=(root:string,args:string[],environment:Record<string,string|undefined>={},after?:(pid:number)=>void)=>new Promise<{code:number|null;out:string;err:string}>(accept=>{
 const child=spawn(process.execPath,[script,...args],{env:{...process.env,FIXTURE_ROOT:root,SEMIO_LANGUAGE:undefined,LANG:undefined,LANGUAGE:undefined,LC_ALL:undefined,LC_MESSAGES:undefined,SEMIO_REPO_IMPLEMENTATION:undefined,...environment} as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"]});
 let out="",err="";child.stdout.on("data",data=>out+=data);child.stderr.on("data",data=>err+=data);child.on("close",(code,signal)=>accept({code:code??(signal==="SIGTERM"?143:null),out,err}));after?.(child.pid!);
});
const fresh=async()=>{const root=makeFixtureWorkspaceV1("A");roots.push(root);return root;};

test("the sealed door runs the stored snapshot: a live edit that does not parse is reported and not run, a valid edit is promoted before a command starts (a light command or a help request runs the stored snapshot as it is and never waits for a promotion), and the mode switch selects the live door",async()=>{
 const root=await fresh();
 const first=await run(root,["x","y"],{SEMIO_REPO_IMPLEMENTATION:"sealed"});
 expect(first).toMatchObject({code:0,out:"DOOR A x,y\n"});expect(first.err).toContain("Promoted the sealed snapshot");
 expect(await run(root,["x"],{SEMIO_REPO_IMPLEMENTATION:"sealed"})).toMatchObject({code:0,out:"DOOR A x\n",err:""});
 setFixtureTreeV1(root,"S");
 const broken=await run(root,["x"],{SEMIO_REPO_IMPLEMENTATION:"sealed"});
 expect(broken).toMatchObject({code:0,out:"DOOR A x\n"});expect(broken.err).toContain("The live tree is not promoted: tool/helper.ts:1:16");expect(broken.err).toContain("Der Arbeitsbaum wird nicht befoerdert: tool/helper.ts:1:16");
 setFixtureTreeV1(root,"B");
 const light=await run(root,["leases","status"],{SEMIO_REPO_IMPLEMENTATION:"sealed"});
 expect(light).toMatchObject({code:0,out:"DOOR A leases,status\n",err:""});
 expect(await run(root,["x","--help"],{SEMIO_REPO_IMPLEMENTATION:"sealed"})).toMatchObject({code:0,out:"DOOR A x,--help\n",err:""});
 const fixed=await run(root,["x"],{SEMIO_REPO_IMPLEMENTATION:"sealed"});
 expect(fixed).toMatchObject({code:0,out:"DOOR B x\n"});expect(fixed.err).toContain("Promoted the sealed snapshot");
 expect(await run(root,["x"],{SEMIO_REPO_IMPLEMENTATION:"live"})).toMatchObject({code:0,out:"LIVE x\n",err:""});
 const refused=await run(root,["x"],{SEMIO_REPO_IMPLEMENTATION:"fast"});
 expect(refused.code).toBe(2);expect(refused.err).toContain("Expected SEMIO_REPO_IMPLEMENTATION=sealed|live|promote");expect(refused.err).toContain("Erwartet SEMIO_REPO_IMPLEMENTATION=sealed|live|promote");
},120_000);

test("a promoter that fails keeps the last good snapshot running and says so; promote mode promotes and runs nothing; a tree that cannot be promoted and has no snapshot runs the live door after the refusal",async()=>{
 const root=await fresh();await promote(root);setFixtureTreeV1(root,"B");
 const failed=await run(root,["x"],{SEMIO_REPO_IMPLEMENTATION:"sealed",FIXTURE_PROMOTER:"broken"});
 expect(failed).toMatchObject({code:0,out:"DOOR A x\n"});expect(failed.err).toContain("The promotion of the live tree failed (the promoter does not load)");expect(failed.err).toContain("Die Befoerderung des Arbeitsbaums ist fehlgeschlagen");
 const promoted=await run(root,["x"],{SEMIO_REPO_IMPLEMENTATION:"promote"});
 expect(promoted).toMatchObject({code:0,out:""});expect(promoted.err).toContain("Sealed snapshot");
 expect(await run(root,["x"],{SEMIO_REPO_IMPLEMENTATION:"sealed"})).toMatchObject({code:0,out:"DOOR B x\n",err:""});
 setFixtureTreeV1(root,"S");
 const refused=await run(root,["x"],{SEMIO_REPO_IMPLEMENTATION:"promote"});
 expect(refused.code).toBe(1);
 const cold=await fresh();setFixtureTreeV1(cold,"S");
 const none=await run(cold,["a"],{SEMIO_REPO_IMPLEMENTATION:"sealed"});
 expect(none.out).toBe("LIVE a\n");expect(none.err).toContain("No sealed snapshot exists yet");expect(none.err).toContain("Es gibt noch keinen versiegelten Schnappschuss");
},120_000);

test("the exit code of the sealed door is the exit code of the command, and a stop signal reaches the sealed door",async()=>{
 const root=await fresh();await promote(root);
 expect(await run(root,["exit","7"],{SEMIO_REPO_IMPLEMENTATION:"sealed"})).toMatchObject({code:7,out:""});
 const started=Date.now();
 const stopped=await run(root,["sleep"],{SEMIO_REPO_IMPLEMENTATION:"sealed"},pid=>{setTimeout(()=>process.kill(pid,"SIGTERM"),2500);});
 expect(stopped.code).toBe(143);expect(Date.now()-started).toBeLessThan(20_000);
},120_000);

test("a snapshot that no longer loads is reported in both languages and the live door runs",async()=>{
 const root=await fresh();const resolved=await promote(root);const entry=sealedEntryOfV1(root,fixtureEntryV1);
 const file=entry.snapshot(resolved.pointer.digest),bytes=statSync(file).size;
 chmodSync(file,0o644);writeFileSync(file,"x".repeat(bytes));
 const result=await run(root,["a"],{SEMIO_REPO_IMPLEMENTATION:"sealed"});
 expect(result.out).toBe("LIVE a\n");expect(result.err).toContain("The sealed front door did not start");expect(result.err).toContain("Die versiegelte Vordertuer ist nicht gestartet");
 expect(readSealedPointerV1(entry)?.digest).toBe(resolved.pointer.digest);
},120_000);

test("the entry's role, kernel table and root rule are the ones the trampoline and the front door use",async()=>{
 expect(sealedDoorRoleV1).toBe("--semio-role=door");
 expect(Object.keys(sealedKernelsV1).sort()).toEqual(Object.keys(kernelSchema.properties.kernels.const).sort());
 const source=await Bun.file(fileURLToPath(new URL("../../🚪️entry/🟦️.ts",import.meta.url))).text();
 for(const[kind,path] of Object.entries(kernelSchema.properties.kernels.const))expect(source.includes(`"../../⌨️command/${path}"`),kind).toBe(true);
 const trampoline=await Bun.file(fileURLToPath(new URL("../../🚦️trampoline/🟦️.ts",import.meta.url))).text();
 expect(trampoline.includes(`doorRoleV1="${sealedDoorRoleV1}"`)).toBe(true);
 const here=fileURLToPath(new URL(".",import.meta.url));
 expect(findSealedRootV1(here)).toBe(findBuildDoorRootV1(here));
 expect(implementationModeV1({})).toBe(policyData.defaultMode as ImplementationModeV1);
 expect(sealedLightCommandV1(["repo","leases","status"],doorPolicy.light.map(row=>row.operation))).toBe(true);
 expect(sealedLightCommandV1(["repo","run","x"],doorPolicy.light.map(row=>row.operation))).toBe(false);
});
