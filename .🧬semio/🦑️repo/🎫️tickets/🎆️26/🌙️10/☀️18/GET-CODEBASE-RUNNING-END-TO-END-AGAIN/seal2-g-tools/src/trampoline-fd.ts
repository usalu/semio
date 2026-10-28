import{spawn}from"node:child_process";
import{closeSync,createReadStream,fstatSync,openSync}from"node:fs";
import{constants as osConstants}from"node:os";
import{relative}from"node:path";
import{fileURLToPath}from"node:url";
import door from"../../🚪️door/🔣️.json"with{type:"json"};
import{implementationModeV1,readSealedLoaderV1,readSealedPointerV1,readSealedReceiptV1,sealedEntryOfV1,sealedLanguagesV1,sealedLightCommandV1,sealedPolicyV1,sealedTextV1,sealedTreeDriftV1}from"../🟦️.ts";

/** 🚦️ The role argument that makes the sealed entry run the front door (the same literal the entry module exports; a law compares the two). */
const doorRoleV1="--semio-role=door";
/** 🔁️ What a promotion answers the trampoline. */
export interface TrampolinePromotionV1{readonly status:string;readonly pointer:Readonly<{digest:string}>}
/** 🔌️ What the trampoline needs beyond Node: the entry of the store and the promotion of a drifted tree (live code, loaded only when the tree drifted). */
export interface TrampolineOptionsV1{readonly entry?:string;readonly promote?:(root:string,entry:string,environment:NodeJS.ProcessEnv,write:(text:string)=>void)=>Promise<TrampolinePromotionV1>}
const promoteLiveV1=async(root:string,entry:string,environment:NodeJS.ProcessEnv,write:(text:string)=>void):Promise<TrampolinePromotionV1>=>(await import("../🔁️promoter/🟦️.ts")).promoteLiveTreeV1(root,entry,environment,write);

/** 🚦️ Starts the build tool's front door from the sealed snapshot. This is the only code of the tool that runs from the live tree on every start, so it imports Node built-ins and the store reader only; everything heavier is loaded when it is needed. In `sealed` mode it compares the live tree with the tree the current snapshot was captured from (a stat fingerprint, no file read) and, when the tree drifted, promotes it from live code first (the store promotes only a tree that parses and boots, under one lock for all processes); a promotion that is blocked or fails says so in English and German and the last good snapshot runs, so a half-finished edit anywhere in the tool cannot stop a command. `live` (the declared opt-in of the agent that edits the tool) runs the live front door, `promote` promotes and runs nothing, a store without a snapshot runs the live door, and a sealed door that ends before it reported that it started (a snapshot that no longer loads here; not on Windows, where the report cannot be trusted and a command must never run twice) says so and runs the live door too. Signals reach the sealed door, its exit code is ours.
 * @see ../🔁️promoter/🟦️.ts the promoter, ../🚪️entry/🟦️.ts the entry the snapshot runs, ../🟦️.ts the policy and the store */
export async function startImplementationV1(root:string,argv:readonly string[],environment:NodeJS.ProcessEnv,live:()=>Promise<void>,options:TrampolineOptionsV1={}):Promise<void>{
 let mode;
 try{mode=implementationModeV1(environment);}catch(error){process.stderr.write((error as Error).message+"\n");process.exitCode=2;return;}
 if(mode==="live")return live();
 const entry=options.entry??relative(root,fileURLToPath(new URL("../🚪️entry/🟦️.ts",import.meta.url))).split("\\").join("/"),store=sealedEntryOfV1(root,entry),languages=sealedLanguagesV1(environment),promote=options.promote??promoteLiveV1,say=(text:string)=>{try{process.stderr.write(text);}catch{}};
 let pointer=readSealedPointerV1(store);
 const receipt=pointer&&readSealedReceiptV1(store,pointer.digest);
 if(mode==="promote"||!pointer||!receipt?.tree||!sealedLightCommandV1(argv,door.light.map(row=>row.operation))&&Date.now()-pointer.promotedAtMs>=sealedPolicyV1.coalesceMilliseconds&&!sealedTreeDriftV1(root,receipt.tree).fresh){
  try{
   const done=await promote(root,entry,environment,say);
   if(mode==="promote"){say(sealedTextV1(languages,"promoteDone",{digest:done.pointer.digest.slice(0,12),status:done.status})+"\n");process.exitCode=["fresh","refreshed","promoted","served"].includes(done.status)?0:1;return;}
  }catch(error){
   const message=String((error as Error).message??error);
   say((error as{code?:string}).code==="SEMIO_IMPLEMENTATION_NONE"?message+"\n":sealedTextV1(languages,"promoter",{reason:message.replace(/\s+/gu," ").slice(0,200)})+"\n");
   if(mode==="promote"){process.exitCode=1;return;}
  }
  pointer=readSealedPointerV1(store);
 }
 const loader=pointer&&readSealedLoaderV1(store,pointer);
 if(!pointer||loader===undefined)return live();
 let snapshot:number|undefined;
 if(process.platform!=="win32")try{snapshot=openSync(store.snapshot(pointer.digest),"r");if(fstatSync(snapshot).size!==pointer.bytes){closeSync(snapshot);return live();}}catch{if(snapshot!==undefined)try{closeSync(snapshot);}catch{}return live();}
 const code=await new Promise<number|undefined>(accept=>{
  const child=spawn(process.execPath,["--eval",loader,"--",pointer.digest,doorRoleV1,...argv],{stdio:["inherit","inherit","inherit",snapshot??"pipe","pipe"],env:{...environment,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}}),forwards=(["SIGINT","SIGTERM","SIGHUP"] as const).map(signal=>[signal,()=>{stopped=true;try{child.kill(signal);}catch{}}] as const);
  let ready=false,stopped=false;
  for(const[signal,forward] of forwards)process.on(signal,forward);
  const finish=(value:number|undefined)=>{for(const[signal,forward] of forwards)process.off(signal,forward);accept(value);};
  if(snapshot!==undefined)try{closeSync(snapshot);}catch{}
  const feed=child.stdio[3] as NodeJS.WritableStream|null;
  if(feed){feed.on("error",()=>{});createReadStream(store.snapshot(pointer.digest)).on("error",()=>{try{feed.end();}catch{}}).pipe(feed);}
  child.stdio[4]?.on("data",()=>{ready=true;});child.stdio[4]?.on("error",()=>{});
  child.once("error",()=>finish(undefined));
  child.once("close",(status,signal)=>{const exit=status??(signal?128+(osConstants.signals[signal]??0):1);finish(ready||stopped||process.platform==="win32"||status===0?exit:undefined);});
 });
 if(code===undefined){say(sealedTextV1(languages,"startup",{code:"-"})+"\n");return live();}
 process.exitCode=code;
}
