import {mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {GENERATION_POLICY_V1,createNodeFilePortV1,readGenerationPointerV1,type GenerationFilePortV1} from "../🟨️.mjs";
import {localGenerationIdentityV1,remoteGenerationExchangeV1,type GenerationCursorsV1,type GenerationExchangeOutcomeV1} from "../🌐️remote-exchange/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📡️ The declared numbers of the wave upload. @see ./🧬️schema/🔣️.json */
export const WAVE_UPLOAD_POLICY_V1=Object.freeze({...schema.definitions.Policy.const});
/** 🗣️ The words of the server log lines of the wave upload, in English and German. */
export const WAVE_UPLOAD_TEXTS_V1=schema.definitions.Texts.const;
export type WaveUploadPolicyV1=typeof WAVE_UPLOAD_POLICY_V1;

/** 🔌️ What the uploader needs of a machine: the generation number of the newest sealed generation of the local store, and one upload of that generation as a delta (`stale` is the number of projects the producer still knows stale in it, absent when it does not know). */
export type GenerationUploadSourceV1=Readonly<{pointer():Promise<number|undefined>;upload(input:Readonly<{stale?:number;signal:AbortSignal}>):Promise<GenerationExchangeOutcomeV1>}>;
/** 🌊️ One wave of the read as the graph server sees it: the generation it published and how many of the projects of the read are done. */
export type WaveUploadRowV1=Readonly<{generation:number;done:number;total:number}>;
export type GenerationUploaderV1=Readonly<{wave(row:WaveUploadRowV1):void;tick():void;idle():Promise<void>;stop():void}>;
type State=Readonly<{version:1;generation:number;atMs:number;uploaded:boolean}>;

const fill=(template:string,fields:Readonly<Record<string,string|number>>):string=>template.replace(/\{(\w+)\}/gu,(_,name:string)=>String(fields[name]??""));

function readState(path:string|undefined):State|undefined{
 if(path===undefined)return undefined;
 try{const value=JSON.parse(readFileSync(path,"utf8")) as State;return value.version===1&&Number.isSafeInteger(value.generation)&&Number.isSafeInteger(value.atMs)&&typeof value.uploaded==="boolean"?value:undefined;}catch{return undefined;}
}
function writeState(path:string|undefined,state:State):void{
 if(path===undefined)return;
 const temporary=path+"."+process.pid+".tmp";
 try{mkdirSync(dirname(path),{recursive:true});writeFileSync(temporary,JSON.stringify(state));renameSync(temporary,path);}catch{rmSync(temporary,{force:true});}
}

/** ⏲️ Keeps the generation of the fleet cache close to the waves of the serving machine: `wave` notes the stale count of a published wave, `tick` (called every round of the server, never awaited) starts at most one upload at a time of the newest sealed generation when it is not the one looked at last and the minimum interval since the last look has passed, and the upload sends only the documents the cache misses, so an interrupted one resumes from what arrived. A failure backs off and never reaches the caller; the memory of the last look survives a restart. */
export function createGenerationUploaderV1(input:Readonly<{source:GenerationUploadSourceV1;statePath?:string;language:"en"|"de";say(line:string):void;may?():Promise<boolean>;now?:()=>number;policy?:WaveUploadPolicyV1}>):GenerationUploaderV1{
 const policy=input.policy??WAVE_UPLOAD_POLICY_V1,now=input.now??Date.now,texts=WAVE_UPLOAD_TEXTS_V1[input.language],controller=new AbortController();
 let state=readState(input.statePath),blockedUntil=-Infinity,flight:Promise<void>|undefined,wave:{generation:number;stale:number}|undefined;
 const say=(key:keyof typeof texts,fields:Readonly<Record<string,string|number>>)=>{try{input.say(fill(texts[key],fields));}catch{}};
 const remember=(generation:number,uploaded:boolean)=>{state={version:1,generation,atMs:now(),uploaded};writeState(input.statePath,state);};
 const run=async():Promise<void>=>{
  let generation:number|undefined;
  try{
   if(controller.signal.aborted||(input.may!==undefined&&!await input.may()))return;
   generation=await input.source.pointer();
   if(generation===undefined)return;
   if(state!==undefined){
    if(generation===state.generation&&(state.uploaded||now()-state.atMs<policy.retryMilliseconds))return;
    if(generation!==state.generation&&now()-state.atMs<policy.minimumIntervalMilliseconds)return;
   }
   const stale=wave?.generation===generation?wave.stale:undefined,started=now(),outcome=await input.source.upload({stale,signal:AbortSignal.any([controller.signal,AbortSignal.timeout(policy.timeoutMilliseconds)])});
   if(outcome.status==="uploaded"){
    remember(generation,true);
    say("uploaded",{generation,sent:outcome.sent??outcome.documents??0,documents:outcome.documents??0,megabytes:Math.round((outcome.bytes??0)/1048576),seconds:Math.round((now()-started)/1000),stale:stale??texts.unknown});
   }else if(outcome.status==="remote-better"){remember(generation,false);say("better",{generation});}
   else if(outcome.status==="current"){remember(generation,true);say("current",{generation});}
   else if(outcome.status==="empty"||outcome.status==="absent"||outcome.status==="local")return;
   else{blockedUntil=now()+policy.failureBackoffMilliseconds;say("failed",{generation,reason:outcome.reason??outcome.status,seconds:Math.round(policy.failureBackoffMilliseconds/1000)});}
  }catch(error){
   if(controller.signal.aborted)return;
   blockedUntil=now()+policy.failureBackoffMilliseconds;
   say("failed",{generation:generation??-1,reason:String((error as Error)?.message??error).slice(0,200),seconds:Math.round(policy.failureBackoffMilliseconds/1000)});
  }
 };
 return Object.freeze({
  wave(row){wave={generation:row.generation,stale:Math.max(0,row.total-row.done)};},
  tick(){if(flight!==undefined||controller.signal.aborted||now()<blockedUntil)return;flight=run().finally(()=>{flight=undefined;});},
  idle:async()=>{await flight;},
  stop(){controller.abort();}
 });
}

/** 🗄️ The upload source of the machine that owns the generation store at `root`: the pointer is read from the store, every upload builds its own exchange (a failure of the cache disables an exchange for its lifetime, so the next attempt gets a fresh one) and publishes the generation from the files of the store. */
export function localGenerationUploadSourceV1(input:Readonly<{workspace:string;root:string;environment:Readonly<Record<string,string|undefined>>;platform?:string;cursors?:GenerationCursorsV1;origins?:Readonly<{self:string;all:readonly string[]}>;files?:GenerationFilePortV1}>):GenerationUploadSourceV1{
 const files=input.files??createNodeFilePortV1();
 return Object.freeze({
  async pointer(){
   const bytes=await files.read(join(input.root,GENERATION_POLICY_V1.pointer));if(!bytes)return undefined;
   try{return readGenerationPointerV1(bytes).generation;}catch{return undefined;}
  },
  async upload({stale,signal}){
   const exchange=remoteGenerationExchangeV1({environment:input.environment,workspace:input.workspace,identity:()=>localGenerationIdentityV1(input.root,files),platform:input.platform,cursors:input.cursors,origins:input.origins});
   if(exchange===undefined||!exchange.upload)return{status:"absent"};
   return exchange.publish({root:input.root,fs:files,signal,stale});
  }
 });
}
