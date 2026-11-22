import {readFileSync,readdirSync,statSync} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {readyWorkerLivenessV1,type ReadyLivenessV1} from "../💔️silence/🟦️.ts";

const P=schema.definitions.Policy.const;

/** 🩺️ How long a machine may show no sign of life before the fleet reports it offline, in seconds. */
export const READY_OFFLINE_SECONDS_V1=P.offlineMilliseconds/1000;

/** 🩺️ One worker as a reader on another machine sees it: who it is, what its record claims and what its liveness really is. */
export type ReadyWorkerRowV1=Readonly<{origin:string;platform:string|null;liveness:ReadyLivenessV1["liveness"];recorded:string|null;stage:string|null;stageIndex:number;stageCount:number;attempts:number;recordAgeSeconds:number|null;heartbeatAgeSeconds:number|null;refusal:Readonly<{code:string;en:string;de:string}>|null}>;

/** 🩺️ Reads every origin of the fleet store (`<store>/v1/origins/<origin>`): its `ready.json` and the newest modification time of the heartbeat files its replica writes every few seconds, and judges each with {@link readyWorkerLivenessV1}, so a machine that went away is `offline` however `running` its last record says it was. An unreadable store gives no rows; an origin without a record is `absent`.
 * @see ./🧪️tests/🟦️.ts */
export function readReadyWorkersV1(input:Readonly<{store:string;nowMs:number;policy?:typeof P}>):readonly ReadyWorkerRowV1[]{
 const policy=input.policy??P,base=join(input.store,"v1","origins");
 let names:string[]=[];
 try{names=readdirSync(base,{withFileTypes:true}).filter(row=>row.isDirectory()&&/^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/u.test(row.name)).map(row=>row.name).sort();}catch{return[];}
 return names.map(origin=>{
  const directory=join(base,origin);
  let status:unknown=null,heartbeat:number|null=null;
  try{status=JSON.parse(readFileSync(join(directory,policy.statusName),"utf8"));}catch{}
  for(const name of policy.heartbeatNames)try{heartbeat=Math.max(heartbeat??0,Math.floor(statSync(join(directory,name)).mtimeMs));}catch{}
  const verdict=readyWorkerLivenessV1({status,heartbeatMs:heartbeat,nowMs:input.nowMs,policy}),row=(status!==null&&typeof status==="object"?status:{}) as Record<string,unknown>;
  const text=(key:string):string|null=>typeof row[key]==="string"?row[key] as string:null,count=(key:string):number=>Number.isSafeInteger(row[key])&&(row[key] as number)>=0?row[key] as number:0,refusal=row.refusal as {code?:unknown;en?:unknown;de?:unknown}|null|undefined;
  return Object.freeze({origin,platform:text("platform"),liveness:verdict.liveness,recorded:verdict.liveness==="absent"?null:text("state"),stage:text("stage"),stageIndex:count("stageIndex"),stageCount:count("stageCount"),attempts:count("attempts"),recordAgeSeconds:verdict.ageMs===null?null:Math.floor(verdict.ageMs/1000),heartbeatAgeSeconds:verdict.heartbeatAgeMs===null?null:Math.floor(verdict.heartbeatAgeMs/1000),refusal:refusal!==null&&refusal!==undefined&&typeof refusal.code==="string"&&typeof refusal.en==="string"&&typeof refusal.de==="string"?Object.freeze({code:refusal.code,en:refusal.en,de:refusal.de}):null});
 });
}
