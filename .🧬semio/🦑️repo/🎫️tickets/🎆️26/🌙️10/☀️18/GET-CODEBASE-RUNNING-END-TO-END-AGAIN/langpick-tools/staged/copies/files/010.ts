/** 🔁️ Contract: a step that reads inputs other processes edit takes a new snapshot and runs again, bounded by the schema, with EN/DE progress; only an edit that persists through every attempt of the widest layer fails, naming the path. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {CurrentPhysicalOwnerV1} from "../../📁️current/🟦️.ts";
import type {FileObservationControlV1,FileObservationProgressV1} from "../../🟦️.ts";
import {testPythonV1} from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {ConcurrentEditErrorV1,churnLanguagesV1,churnProtocolV1,churnTextV1,concurrentEditOfV1,concurrentEditV1,isConcurrentEditV1,retryOnConcurrentEditSyncV1,retryOnConcurrentEditV1,type ChurnProfileNameV1,type ChurnRetryEventV1,type ConcurrentEditReasonV1} from "../🟦️.ts";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),protocol=churnProtocolV1();
const oracle=JSON.parse(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),join(here,"🔣️.json"),join(here,"..","🧬️schema","🔣️.json")]).stdout.toString()) as Record<string,unknown>;
type Scenario={id:string;layers:ChurnProfileNameV1[];outcomes:string[];environment:Record<string,string>};
const scenarios=fixture.scenarios as Scenario[];

async function replay(row:Scenario,synchronous:boolean):Promise<Record<string,unknown>>{
 const outcomes=[...row.outcomes],sleeps:number[]=[],events:ChurnRetryEventV1[]=[];let calls=0;
 const options={environment:row.environment,onRetry:(event:ChurnRetryEventV1)=>events.push(event),sleep:async(milliseconds:number)=>{sleeps.push(milliseconds);},sleepSync:(milliseconds:number)=>{sleeps.push(milliseconds);}};
 const body=():string=>{calls++;const step=outcomes.shift()!;if(step==="ok")return "ok";if(step==="other")throw new RangeError("not a concurrent edit");throw concurrentEditV1(step as ConcurrentEditReasonV1,"/p/"+calls);};
 const layer=(index:number):Promise<string>=>index===row.layers.length?Promise.resolve().then(body):retryOnConcurrentEditV1(row.layers[index]!,()=>layer(index+1),options);
 const layerSync=(index:number):string=>index===row.layers.length?body():retryOnConcurrentEditSyncV1(row.layers[index]!,()=>layerSync(index+1),options);
 let result:Record<string,unknown>;
 try{result={result:synchronous?layerSync(0):await layer(0)};}
 catch(error){
  if(error instanceof ConcurrentEditErrorV1)result={result:"exhausted",reason:error.reason,path:error.path,rank:error.rank,message:error.message};
  else if(error instanceof RangeError)result={result:"other"};
  else throw error;
 }
 return{...result,calls,sleeps,messages:events.map(event=>event.text)};
}

test("the policy table is closed under its schema: attempts, pauses and EN/DE texts agree",()=>{
 const ajv=new Ajv({strict:false}),profile=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Profile"}),text=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Text"});
 const ranks=new Set<number>();
 for(const [name,row] of Object.entries(protocol.profiles)){
  expect(profile(row),name+JSON.stringify(profile.errors)).toBe(true);
  expect(row.backoffMs.length,name).toBe(row.maximumAttempts-1);
  expect([...row.backoffMs].sort((left,right)=>left-right),name).toEqual([...row.backoffMs]);
  expect(ranks.has(row.rank),name).toBe(false);ranks.add(row.rank);
 }
 expect(protocol.profiles.read.rank).toBeLessThan(protocol.profiles.snapshot.rank);
 expect(protocol.profiles.snapshot.rank).toBeLessThan(protocol.profiles.build.rank);
 const reasons=schema.definitions.Reason.enum;
 for(const language of protocol.languages){
  expect(Object.keys(protocol.reasons[language]!).sort()).toEqual([...reasons].sort());
  for(const value of Object.values(protocol.reasons[language]!))expect(text(value)).toBe(true);
  for(const key of ["retry","exhausted"] as const)expect(text(protocol.texts[language]![key])).toBe(true);
 }
 const placeholders=(language:string,key:"retry"|"exhausted")=>[...protocol.texts[language]![key].matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 for(const key of ["retry","exhausted"] as const)expect(placeholders("en",key)).toEqual(placeholders("de",key));
});

test("the implementation replays every scripted scenario exactly like the independent Python model, asynchronous and synchronous",async()=>{
 expect(Object.keys(oracle).sort()).toEqual(scenarios.map(row=>row.id).sort());
 for(const row of scenarios){
  expect(await replay(row,false),row.id).toEqual(oracle[row.id] as Record<string,unknown>);
  expect(await replay(row,true),row.id+" (sync)").toEqual(oracle[row.id] as Record<string,unknown>);
 }
});

test("a layer retries raw edits and edits exhausted by narrower layers only, and a wider layer never retries its own exhaustion",async()=>{
 const raw=concurrentEditV1("bytes","/a");
 expect(concurrentEditOfV1(raw,1)?.path).toBe("/a");
 const exhausted=await retryOnConcurrentEditV1("read",async()=>{throw raw;},{onRetry:()=>undefined,sleep:async()=>undefined}).catch(error=>error);
 expect(exhausted).toBeInstanceOf(ConcurrentEditErrorV1);
 expect(exhausted.rank).toBe(protocol.profiles.read.rank);expect(exhausted.attempts).toBe(protocol.profiles.read.maximumAttempts);
 expect(exhausted.cause).toBe(raw);expect(exhausted.path).toBe("/a");
 expect(concurrentEditOfV1(exhausted,protocol.profiles.read.rank)).toBeUndefined();
 expect(concurrentEditOfV1(exhausted,protocol.profiles.snapshot.rank)?.path).toBe("/a");
 expect(isConcurrentEditV1(exhausted)).toBe(true);
 expect(isConcurrentEditV1(new Error("x"))).toBe(false);
});

test("a batch of failures is one concurrent edit only when every member is, and a file-observation 'changed' refusal counts",()=>{
 expect(concurrentEditOfV1(new AggregateError([concurrentEditV1("roster","/d"),concurrentEditV1("bytes","/f")],"batch"))?.reason).toBe("roster");
 expect(concurrentEditOfV1(new AggregateError([concurrentEditV1("roster","/d"),new Error("disk")],"batch"))).toBeUndefined();
 const refusal=Object.assign(new Error("Physical file observation: changed: /f"),{name:"FileObservationErrorV1",code:"changed",path:"/f"});
 expect(concurrentEditOfV1(refusal)?.path).toBe("/f");
 expect(concurrentEditOfV1(Object.assign(new Error("x"),{name:"FileObservationErrorV1",code:"io",path:"/f"}))).toBeUndefined();
});

test("cancellation ends the pause at once and the retry is never started",async()=>{
 const controller=new AbortController();let calls=0;
 const run=retryOnConcurrentEditV1("build",async()=>{calls++;throw concurrentEditV1("bytes","/slow");},{signal:controller.signal,onRetry:()=>controller.abort(new Error("cancelled")),environment:{LANG:"en"}});
 const started=Date.now();
 await expect(run).rejects.toThrow("cancelled");
 expect(calls).toBe(1);expect(Date.now()-started).toBeLessThan(protocol.profiles.build.backoffMs[0]!);
});

test("messages render in the chosen language, in both when none is chosen, and refuse unknown keys",()=>{
 expect(churnLanguagesV1({LANG:"de_CH.UTF-8"})).toEqual(["de"]);
 expect(churnLanguagesV1({SEMIO_LANGUAGE:"en",LANG:"de"})).toEqual(["en"]);
 expect(churnLanguagesV1({LANG:"fr_FR"})).toEqual(["en","de"]);
 expect(churnTextV1("de","retry",{path:"/p",reason:"r",attempt:2,maximum:3,delay:5})).toContain("Versuch 2 von 3");
 expect(()=>churnTextV1("fr","retry",{})).toThrow("Declared churn text required");
 expect(()=>churnTextV1("en","retry",{path:"/p"})).toThrow("placeholder required");
});

const scratches:string[]=[];
const languageVariables=["SEMIO_LANGUAGE","LC_ALL","LANG"],savedLanguage=languageVariables.map(name=>[name,process.env[name]] as const);
beforeAll(()=>{for(const name of languageVariables)delete process.env[name];});
afterAll(()=>{for(const [name,value] of savedLanguage)if(value===undefined)delete process.env[name];else process.env[name]=value;for(const path of scratches)rmSync(path,{recursive:true,force:true});});
const scratch=()=>{const path=mkdtempSync(join(realpathSync(tmpdir()),"churn-"));scratches.push(path);return path;};
const sha256=async(bytes:string)=>Buffer.from(await crypto.subtle.digest("SHA-256",new TextEncoder().encode(bytes))).toString("hex");
const owner=(root:string,onProgress:(event:FileObservationProgressV1)=>void=()=>undefined)=>{const control:FileObservationControlV1={maxBytes:1<<26,maxWork:1<<20,chunkBytes:4,cancelled:()=>false,remainingMs:()=>600_000,onProgress};return new CurrentPhysicalOwnerV1(root,control);};

test("a file edited while it is read is read again, and the digest is that of the bytes finally on disk",async()=>{
 const root=scratch(),path=join(root,"input.txt");writeFileSync(path,"first version of the bytes");
 let edits=0;
 const reader=owner(root,event=>{if(event.phase==="read"&&edits<2){edits++;writeFileSync(path,"edit "+edits+" by a peer that is longer");}});
 const digest=await reader.digest(path);
 expect(edits).toBe(2);expect(digest).toBe(await sha256("edit 2 by a peer that is longer"));
 await reader.recheck();
});

test("a rewrite with identical bytes, such as a touch or an atomic same-content replace, is accepted and its new identity adopted",async()=>{
 const root=scratch(),path=join(root,"input.txt");writeFileSync(path,"stable bytes");
 const reader=owner(root),first=await reader.digest(path);
 utimesSync(path,new Date(Date.now()+60_000),new Date(Date.now()+60_000));writeFileSync(path,"stable bytes");
 expect(await reader.digest(path)).toBe(first);
 await reader.recheck();
 expect(reader.custody.identities.get(path)?.length).toBe(5);
});

test("bytes that differ from the claim of the same snapshot are not re-read: only a new snapshot can cure them",async()=>{
 const root=scratch(),path=join(root,"input.txt");writeFileSync(path,"before");
 const reader=owner(root);await reader.digest(path);
 writeFileSync(path,"after, and longer");
 const failure=await reader.digest(path).catch(error=>error);
 expect(failure).toBeInstanceOf(ConcurrentEditErrorV1);
 expect(failure.reason).toBe("bytes");expect(failure.path).toBe(path);expect(failure.attempts).toBe(0);
 expect(failure.rank).toBe(protocol.profiles.snapshot.rank-1);
 const fresh=owner(root);expect(await fresh.digest(path)).toBe(await sha256("after, and longer"));
 const recheck=await reader.recheck().catch(error=>error);
 expect(recheck).toBeInstanceOf(ConcurrentEditErrorV1);expect(recheck.path).toBe(path);
});

test("a directory listed while a peer adds a file is listed again; a roster that differs from the snapshot's earlier listing needs a new snapshot",async()=>{
 const root=scratch(),dir=join(root,"d");mkdirSync(dir);
 for(let index=0;index<60;index++)writeFileSync(join(dir,"f"+String(index).padStart(2,"0")),"x");
 let added=false;
 const reader=owner(root,()=>{if(!added){added=true;writeFileSync(join(dir,"late"),"x");}});
 const names=(await reader.directoryEntries(dir))!.map(row=>row.path.slice(dir.length+1));
 expect(added).toBe(true);expect(names).toContain("late");expect(names.length).toBe(61);
 writeFileSync(join(dir,"later"),"x");
 const failure=await reader.directoryEntries(dir).catch(error=>error);
 expect(failure).toBeInstanceOf(ConcurrentEditErrorV1);expect(failure.reason).toBe("roster");expect(failure.rank).toBe(protocol.profiles.snapshot.rank-1);
 const recheck=await reader.recheckStructure().catch(error=>error);
 expect(recheck).toBeInstanceOf(ConcurrentEditErrorV1);expect(recheck.path).toBe(dir);
 expect((await owner(root).directoryEntries(dir))!.length).toBe(62);
});

test("an input that is rewritten on every read fails after the declared attempts, naming the path in the chosen language",async()=>{
 const root=scratch(),path=join(root,"busy.txt");writeFileSync(path,"x");
 let version=0;
 const reader=owner(root,event=>{if(event.phase==="read")writeFileSync(path,"v"+(++version)+" ".repeat(version%3));});
 const saved=process.env.SEMIO_LANGUAGE;process.env.SEMIO_LANGUAGE="de";
 const failure=await reader.digest(path).catch(error=>error).finally(()=>{if(saved===undefined)delete process.env.SEMIO_LANGUAGE;else process.env.SEMIO_LANGUAGE=saved;});
 expect(failure).toBeInstanceOf(ConcurrentEditErrorV1);
 expect(failure.path).toBe(path);expect(failure.attempts).toBe(protocol.profiles.read.maximumAttempts);
 expect(failure.message).toContain(path);expect(failure.message).toContain("änderte sich");expect(failure.message).not.toContain("kept changing");
 expect(version).toBe(protocol.profiles.read.maximumAttempts);
},30_000);

test("a failure the caller marks as not retryable passes through at once, while an edit raised outside that work is still retried",async()=>{
 const inside=new WeakSet<object>(),events:string[]=[];let calls=0;
 const options={environment:{LANG:"en"},onRetry:(event:ChurnRetryEventV1)=>events.push(event.text),sleep:async()=>undefined,retryable:(error:unknown)=>typeof error!=="object"||error===null||!inside.has(error)};
 const failure=await retryOnConcurrentEditV1("snapshot",async()=>{calls++;const error=concurrentEditV1("bytes","/inside");inside.add(error);throw error;},options).catch(error=>error);
 expect(failure.path).toBe("/inside");expect(failure.attempts).toBe(0);expect(calls).toBe(1);expect(events.length).toBe(0);
 calls=0;
 const result=await retryOnConcurrentEditV1("snapshot",async()=>{calls++;if(calls<3)throw concurrentEditV1("generation","/outside");return "done";},options);
 expect(result).toBe("done");expect(calls).toBe(3);expect(events.length).toBe(2);
});
