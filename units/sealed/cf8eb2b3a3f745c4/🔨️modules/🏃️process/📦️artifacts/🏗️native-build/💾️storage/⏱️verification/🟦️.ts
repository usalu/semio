import {AsyncLocalStorage} from "node:async_hooks";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

type Backend="mounted-image"|"directory-partition";
type Phase="storeIdentity"|"reservationOperation"|"allocationJournal"|"journalGrant"|"journalRead"|"mappingGrant"|"volumeIdentity"|"mappingProof"|"physicalGateCached"|"physicalGateJoin"|"physicalGateVerify"|"imageInventory"|"descendantChecks"|"allocatedUsage";
export type NativeStorageVerificationObserverV1=(backend:Backend,phase:Phase)=>void|(()=>void);
export type NativeStorageVerificationTimingV1=Readonly<{started:number;active:number;milliseconds:number}>;
export type NativeStorageVerificationSnapshotV1=Readonly<Record<string,Readonly<Record<string,NativeStorageVerificationTimingV1>>>>;
const backends=new Set(schema.definitions.Backend.enum),phases=new Set(schema.definitions.Phase.enum),maximum=schema.definitions.Policy.const.maximumCounter;
type Observation={backend:Backend;observe?:NativeStorageVerificationObserverV1};
const observers=new AsyncLocalStorage<Observation>();

/** 🛰️ Enables bounded verification diagnostics for one accepted caller and no other asynchronous frame. */
export function withNativeStorageVerificationObservationV1<T>(backend:string,observe:NativeStorageVerificationObserverV1,body:()=>Promise<T>):Promise<T>{
 if(!backends.has(backend))return observers.exit(body);
 const frame:Observation={backend:backend as Backend,observe};
 return observers.run(frame,async()=>{try{return await body();}finally{frame.observe=undefined;}});
}

/** 🧵️ Carries only the verification observer across an existing deferred Native queue. */
export function bindNativeStorageVerificationObservationV1<T>(body:()=>T):()=>T{
 const frame=observers.getStore();
 return ()=>frame?.observe?observers.run(frame,body):observers.exit(body);
}

/** ⏳️ Starts an opt-in timing span. A failing observer cannot change the Native decision. */
export function startNativeStorageVerificationPhaseV1(phase:Phase):(()=>void)|undefined{
 const frame=observers.getStore();if(!frame?.observe||!phases.has(phase))return;
 try{const end=frame.observe(frame.backend,phase);if(typeof end==="function")return ()=>{try{end();}catch{}};}catch{}
}

/** 🔍️ Observes one existing awaited verification operation while preserving its result and failure. */
export function observeNativeStorageVerificationAwaitV1<T>(phase:Phase,body:()=>Promise<T>):Promise<T>{
 const end=startNativeStorageVerificationPhaseV1(phase);if(!end)return body();
 try{return body().finally(end);}catch(error){end();throw error;}
}

/** 📊️ Summarizes finite inclusive wall time by physical backend and phase, including active waits. */
export function createNativeStorageVerificationTimingsV1(clock=()=>performance.now()){
 type Row={started:number;active:number;elapsed:number;at:number};
 const counters=new Map<Backend,Map<Phase,Row>>();
 const elapsed=(row:Row,now:number)=>Math.min(maximum,row.elapsed+Math.max(0,now-row.at)*row.active);
 const observe:NativeStorageVerificationObserverV1=(backend,phase)=>{
  if(!backends.has(backend)||!phases.has(phase))return;
  let group=counters.get(backend);if(!group)counters.set(backend,group=new Map());
  let row=group.get(phase);const now=clock();if(!row)group.set(phase,row={started:0,active:0,elapsed:0,at:now});
  const current=row,tick=(at:number)=>{current.elapsed=elapsed(current,at);current.at=at;};tick(now);current.started=Math.min(maximum,current.started+1);current.active++;
  let open=true;return ()=>{if(open){open=false;tick(clock());current.active--;}};
 };
 const stats=():NativeStorageVerificationSnapshotV1=>{const now=clock();return Object.fromEntries([...counters].map(([kind,rows])=>[kind,Object.fromEntries([...rows].map(([phase,row])=>[phase,{started:row.started,active:Math.min(maximum,row.active),milliseconds:Math.floor(elapsed(row,now))}]))]));};
 return{observe,stats};
}
