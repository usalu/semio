import {createConnection} from "node:net";
import {UNIT_CACHE_POLICY_V1,type AdmitFactsV1} from "../🔏️key/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1.broker;
type Environment=Readonly<Record<string,string|undefined>>;
/** 🧾️ What the compiler measured when the unit was first built, kept in the cache entry so a later hit can report it. */
export type UnitProvenanceV1=Readonly<{peakRssBytes:number;userMs:number;sysMs:number;wallMs:number;rmetaOffsetMs:number|null}>;
type Endpoint=Readonly<{endpoint:string;pass:string}>;

/** 🎚️ The broker the build driver started for this compiler call, when replay is switched on. */
export function brokerOfV1(environment:Environment):Endpoint|undefined{
 const endpoint=environment[policy.environment],pass=environment[policy.pass];
 return environment.SEMIO_UNIT_CACHE_BROKER_REPLAY==="on"&&endpoint&&pass?{endpoint,pass}:undefined;
}

function exchange(endpoint:string,message:object,reply:"verdict"|"unit"):Promise<Record<string,unknown>|undefined>{
 return new Promise(done=>{
  const socket=createConnection(endpoint),timer=setTimeout(()=>{socket.destroy();done(undefined);},policy.timeoutMilliseconds);let text="";
  const finish=(value:Record<string,unknown>|undefined)=>{clearTimeout(timer);socket.destroy();done(value);};
  socket.on("error",()=>finish(undefined));
  socket.on("connect",()=>socket.write(JSON.stringify(message)+"\n"));
  socket.on("data",chunk=>{text+=chunk;const at=text.indexOf("\n");if(at<0)return;try{const row=JSON.parse(text.slice(0,at));finish(row.op===reply?row:undefined);}catch{finish(undefined);}});
  socket.on("close",()=>finish(undefined));
 });
}

/** 🔭️ Asks the broker for the measurements of a unit its scheduler just finished (the scheduler reports after the compiler exits, so the answer is awaited briefly). */
export async function recallProvenanceV1(environment:Environment,facts:AdmitFactsV1):Promise<UnitProvenanceV1|undefined>{
 const broker=brokerOfV1(environment);if(!broker)return undefined;
 for(let attempt=0;attempt<policy.recallAttempts;attempt++){
  const row=await exchange(broker.endpoint,{v:1,op:"recall",pass:broker.pass,crate:facts.crate,outDir:facts.outDir,extraFilename:facts.extraFilename},"unit");
  const record=row?.record as {startMs:number;endMs:number;rmetaMs:number|null;peakRssBytes:number;userMs:number;sysMs:number}|null|undefined;
  if(record)return{peakRssBytes:record.peakRssBytes,userMs:record.userMs,sysMs:record.sysMs,wallMs:Math.max(0,record.endMs-record.startMs),rmetaOffsetMs:record.rmetaMs===null?null:Math.max(0,record.rmetaMs-record.startMs)};
  if(row===undefined)return undefined;
  await new Promise(done=>setTimeout(done,policy.recallPauseMilliseconds));
 }
 return undefined;
}

/** 📣️ Tells the broker that a unit was served from the cache, with the measurements recorded when it was built, so the build's unit list, critical path and provenance are complete. */
export async function replayCachedUnitV1(environment:Environment,facts:AdmitFactsV1,cacheKey:string,provenance:UnitProvenanceV1|null|undefined):Promise<boolean>{
 const broker=brokerOfV1(environment);if(!broker)return false;
 const row=await exchange(broker.endpoint,{v:1,op:"cached",pass:broker.pass,pid:process.pid,...facts,cacheKey,provenance:provenance??null},"verdict");
 return row?.admitted===true;
}
