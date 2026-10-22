/** 📥️ Contract: the pinned Nx tarball is acquired through an injected host exactly as the corpus in `./🔣️.json` states: the lock row is closed, the authority must own the checkout and grant the network, the registry URL and options are fixed, the length header and the byte ceiling bound the read, the integrity of the real bytes must equal the lock and the decoded entries equal what Python's tarfile wrote; a cancelled control stops the read and releases the stream. No network is used. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {acquireNxPackageSourceV1,NX_PACKAGE_SOURCE_HOST_V1,type NxPackageSourceHostV1} from "../../🟦️.ts";
import policy from "../../../🗃️archive/📋️policy.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),workspace=resolve("/lawcov-workspace");
const ask=(request:object):any=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:256*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const swap=(value:unknown,integrity:string):unknown=>value==="@integrity"?integrity:Array.isArray(value)?value.map(item=>swap(item,integrity)):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,swap(item,integrity)])):value;
const text=(bytes:Uint8Array)=>new TextDecoder().decode(bytes);
const cache=new Map<string,any>();
const archiveOf=(name:string,tamper:boolean)=>{const key=name+tamper;if(!cache.has(key))cache.set(key,ask({op:"archive",entries:(corpus.archives as Record<string,unknown>)[name],tamper}));return cache.get(key);};

async function run(scenario:(typeof corpus.scenarios)[number]){
 const built=archiveOf(scenario.archive,scenario.tamper),bytes=Buffer.from(built.base64,"base64"),stopped=new AbortController(),events={fetches:[] as {url:string;redirect?:string;signal:boolean}[],requires:0,derived:[] as number[],published:[] as string[],cancelled:false,closed:false,reads:0};
 const control={signal:stopped.signal,remainingMilliseconds:()=>scenario.authority.elapsed,publish:async(row:{resource:string;mode:string})=>{events.published.push(row.resource+" "+row.mode);},yieldContinuation:async()=>{if(scenario.abortAfterReads!==null&&++events.reads===scenario.abortAfterReads)stopped.abort(Error("stopped by the test"));}} as never;
 const body=(scenario.response.body as {archive?:boolean;zeros?:number;none?:boolean}),size=body.zeros??bytes.length,step=scenario.response.chunk||(body.zeros?1048576:bytes.length||1);
 const source=body.zeros?{at:0,next:(length:number)=>new Uint8Array(length)}:{at:0,next:(length:number)=>new Uint8Array(bytes.subarray(source.at,source.at+length))};
 const stream=new ReadableStream<Uint8Array>({pull(controller){if(source.at>=size){events.closed=true;controller.close();return;}const length=Math.min(step,size-source.at),chunk=source.next(length);source.at+=length;controller.enqueue(chunk);},cancel(){events.cancelled=true;}});
 const headers:Record<string,string>={};if(scenario.response.contentLength!==null)headers["content-length"]=scenario.response.contentLength==="@length"?String(size):scenario.response.contentLength;
 const host:NxPackageSourceHostV1=Object.freeze({require:()=>{events.requires++;},authority:()=>({repositoryRoot:scenario.authority.root==="workspace"?workspace:resolve("/elsewhere"),network:{offline:scenario.authority.offline},child:{maximumElapsedMilliseconds:scenario.authority.elapsed??1}}),derive:(phase,milliseconds)=>{events.derived.push(milliseconds);return phase;},fetch:(async(input:string|URL|Request,init?:RequestInit)=>{events.fetches.push({url:String(input),redirect:init?.redirect,signal:init?.signal!==undefined});return body.none?new Response(null,{status:scenario.response.status}):new Response(stream,{status:scenario.response.status,headers});}) as never});
 const lock=swap(scenario.lock,built.integrity);
 try{return{archive:await acquireNxPackageSourceV1(workspace,{} as never,lock,control,host),events,built};}catch(error){return{error:(error as Error).message,events,built};}
}

test("the corpus is closed by its schema and the maximum matches the archive policy",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,scenarios:[{...corpus.scenarios[0],expect:{}}]})).toBe(false);
 expect(policy.maximumCompressedBytes).toBe(corpus.maximumCompressedBytes);
 expect(Object.isFrozen(NX_PACKAGE_SOURCE_HOST_V1)).toBe(true);
});

test("every scenario ends as authored, with the same lock verdict as Python's regular expressions",async()=>{
 const locks=corpus.scenarios.map(scenario=>swap(scenario.lock,"sha512-"+"A".repeat(86)+"==")),verdicts:boolean[]=ask({op:"locks",locks});
 let succeeded=0,refused=0;
 for(const[index,scenario]of corpus.scenarios.entries()){
  const result=await run(scenario),expectation=scenario.expect as {ok?:true;error?:string},label=scenario.name;
  if(expectation.ok){
   succeeded++;expect(result.error,label).toBeUndefined();const archive=result.archive!;
   expect(archive.integrity,label).toBe(result.built.integrity);
   expect(archive.entries.map((entry:any)=>({path:entry.path,text:text(entry.bytes)})),label).toEqual(result.built.entries.map((entry:any)=>({path:entry.path,text:entry.text})));
   expect(result.events.fetches,label).toEqual([{url:corpus.registry+"23.2.0.tgz",redirect:"error",signal:true}]);expect(result.events.requires,label).toBe(1);expect(result.events.derived,label).toEqual([scenario.authority.elapsed??1]);
   const counts=result.events.published.filter((row:string)=>row.startsWith("nx:package-source:")).map((row:string)=>Number(row.split(" ")[0]!.slice(18)));expect(counts.length,label).toBeGreaterThan(0);expect(counts,label).toEqual([...counts].sort((left,right)=>left-right));expect(counts.at(-1),label).toBe(result.built.length);expect(result.events.published.filter((row:string)=>row.startsWith("nx:package-source:")).every((row:string)=>row.endsWith(" shared")),label).toBe(true);
   expect(result.events.cancelled||result.events.closed,label+" the stream is released").toBe(true);
  }else{
   refused++;expect(result.error,label).toBeDefined();
   if(expectation.error==="@abort")expect(result.error,label).toBe("stopped by the test");else if(expectation.error!=="*")expect(result.error,label).toBe(expectation.error);
   if(expectation.error!=="Exact pinned Nx registry package required")expect(verdicts[index],label+" lock verdict").toBe(true);else expect(verdicts[index],label+" lock verdict").toBe(false);
  }
 }
 expect(succeeded).toBe(5);expect(refused).toBe(corpus.scenarios.length-5);
});

test("refusals before the network never reach it and refusals after it always release the stream",async()=>{
 for(const scenario of corpus.scenarios){
  const result=await run(scenario),error=(scenario.expect as {error?:string}).error;
  if(error==="Exact pinned Nx registry package required"||error==="Original package Source workspace required"||error==="Original network grant required for pinned Nx package"){expect(result.events.fetches.length,scenario.name).toBe(0);expect(result.events.derived.length,scenario.name).toBe(0);}
  else if(error!==undefined&&!("none" in scenario.response.body))expect(result.events.cancelled||result.events.closed,scenario.name).toBe(true);
  expect(result.events.requires,scenario.name).toBe(1);
 }
});
