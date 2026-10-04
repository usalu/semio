/** 🧪️ Unmounted Pack cause projections retain source authority independently of display prose. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import captureFixture from "../🧫️fixtures/🔌️capture/🔣️.json";
import captureSchema from "../🧬️schema/🔌️capture/🔣️.json";
import {captureTransport,type CaptureAdmission,type CaptureStorage} from "../🔌️capture/🟦️.ts";
import {reserveTransport} from "../🔌️capture/🚦️provider/🟦️.ts";
import providerFixture from "../🧫️fixtures/🔌️capture/🚦️provider/🔣️.json";
import providerSchema from "../🧬️schema/🔌️capture/🚦️provider/🔣️.json";
import contextFixture from "../🧫️fixtures/🔌️capture/👥️context/🔣️.json";
import contextSchema from "../🧬️schema/🔌️capture/👥️context/🔣️.json";
import {createSharedContext,type SharedReservation} from "../🔌️capture/👥️context/🟦️.ts";
import policyFixture from "../🧫️fixtures/🔌️capture/👥️context/🎛️policy/🔣️.json";
import policySchema from "../🧬️schema/🔌️capture/👥️context/🎛️policy/🔣️.json";
import claimFixture from "../🧫️fixtures/🔌️capture/👥️context/🧾️claim/🔣️.json";
import claimSchema from "../🧬️schema/🔌️capture/👥️context/🧾️claim/🔣️.json";
import lifecycleFixture from "../🧫️fixtures/🔌️capture/👥️context/💥️lifecycle/🔣️.json";
import lifecycleSchema from "../🧬️schema/🔌️capture/👥️context/💥️lifecycle/🔣️.json";
import catalogFixture from "../🧭️producer/🗂️catalog/🧫️fixtures/🔣️.json";
import catalogSchema from "../🧭️producer/🗂️catalog/🧬️schema/🔣️.json";
import {projectCatalogFault,type CatalogFault} from "../🧭️producer/🗂️catalog/🟦️.ts";

import sourceFixture from "../🧭️producer/📥️source/🧫️fixtures/🔣️.json";
import sourceSchema from "../🧭️producer/📥️source/🧬️schema/🔣️.json";
import {sourceFault,projectSourceFault,segmentAdmission,type SourceCause} from "../🧭️producer/📥️source/🟦️.ts";
import storeFixture from "../🧭️producer/🏪️store/🧫️fixtures/🔣️.json";
import storeSchema from "../🧭️producer/🏪️store/🧬️schema/🔣️.json";
import {typedPackAllocationFault} from "../🧭️producer/🏪️store/🟦️.ts";

test("higher typed Pack allocation faults retain the original cause and their distinct reservation charge",()=>{
 expect(new Ajv({strict:true}).compile(storeSchema)(storeFixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  for(const row of storeFixture.cases){
   const original=row.sourceBytes===null?new PackError({kind:"ValueRefusal",error:new ValueError(row.kind as ValueRefusalKind,row.reason)}):new PackError({kind:"RetainedAllocation",refusalKind:row.kind as ValueRefusalKind,allocatedBytes:row.sourceBytes,what:"actual producer",offset:7n,detail:row.reason});
   const actual=typedPackAllocationFault(row.reservationBytes,original);
   const reference=database.query("SELECT ?1 AS kind,?2 AS sourceBytes,?3 AS reservationBytes").get(row.kind,row.sourceBytes,row.reservationBytes) as {kind:string;sourceBytes:number|null;reservationBytes:number};
   expect(actual.cause).toBe(original);expect(actual.cause.refusalKind).toBe(row.kind as ValueRefusalKind);expect(reference.kind).toBe(row.kind);expect(actual.allocatedBytes).toBe(reference.reservationBytes);
   expect(actual.cause.data.kind==="RetainedAllocation"?actual.cause.data.allocatedBytes:null).toBe(reference.sourceBytes);
  }
  console.info("[DEBUG] ten closed higher owner causes matched independent SQLite without erasing source or reservation bytes");
 }finally{database.close();}
});

test("borrowed Segment admission separates scheduling/lifecycle and keeps stored fault identity",()=>{
 expect(new Ajv({strict:true}).compile(sourceSchema)(sourceFixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  for(const row of sourceFixture.admissions){
   const original=new PackError({kind:"ValueRefusal",error:new ValueError("canceled","same misleading structural grammar prose")});
   const actual=segmentAdmission({...row.state,fault:row.state.fault?original:null});
   const reference=database.query("SELECT CASE WHEN ?1 THEN 'fault' WHEN ?2 THEN 'closed' WHEN ?3 THEN 'complete' WHEN ?4 THEN 'pending' WHEN ?5 THEN 'inflaterBackpressure' ELSE 'ready' END AS outcome").get(Number(row.state.fault),Number(row.state.closed),Number(row.state.complete),Number(row.state.pending),Number(row.state.inflaterBackpressure)) as {outcome:string};
   expect(reference.outcome).toBe(row.expected);expect(reference.outcome).toBe(actual.kind);if(actual.kind==="fault"){expect(actual.error).toBe(original);expect(actual.error.refusalKind).toBe("canceled");}
  }
  console.info("[DEBUG] six closed borrowed admission states matched independent SQLite and preserved the original fault");
 }finally{database.close();}
});

test("retained Source factories preserve every precise kind and actual allocation witness",()=>{
 expect(new Ajv({strict:true}).compile(sourceSchema)(sourceFixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  for(const row of sourceFixture.cases){
   const cause:SourceCause=row.cause.type==="value"?{type:"value",kind:typedKind(row.cause.kind),reason:row.cause.reason}:row.cause.type==="paged"?{type:"paged",error:{kind:pagedKind(row.cause.kind),reason:row.cause.reason}}:{type:"allocation",error:{kind:pagedKind(row.cause.kind),reason:row.cause.reason,allocatedBytes:row.cause.allocatedBytes!}};
   const fault=sourceFault(cause),error=projectSourceFault(fault,"retained-pack-source",BigInt(row.offset)),data=error.data;
   if(data.kind!=="RetainedMalformed"&&data.kind!=="RetainedAllocation")throw new Error("Source metadata erased");
   const reference=database.query("SELECT json_object('kind',?1,'reason',?2,'allocatedBytes',CASE WHEN ?3='allocation' THEN ?4 ELSE NULL END) AS result").get(row.cause.kind,row.cause.reason,row.cause.type,row.cause.type==="allocation"?row.cause.allocatedBytes!:null) as {result:string};
   const expected:unknown=JSON.parse(reference.result);expect(expected).toEqual(row.expected);expect(fault as unknown).toEqual(expected);expect({kind:error.refusalKind,reason:data.detail,allocatedBytes:data.kind==="RetainedAllocation"?data.allocatedBytes:null} as unknown).toEqual(expected);expect(data.offset).toBe(7n);expect(data.what).toBe("retained-pack-source");
  }
  console.info("[DEBUG] Source17 source-authored kind/reason/actual allocation witness matched independent SQLite");
 }finally{database.close();}
});

test("retained Catalog projections retain actual source metadata without interpreting code or allocation bytes",()=>{
 expect(new Ajv({strict:true}).compile(catalogSchema)(catalogFixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  for(const row of catalogFixture.cases){
   const cause=row.cause.type==="value"?{type:"value" as const,kind:typedKind(row.cause.kind),reason:row.cause.reason}:row.cause.type==="paged"?{type:"paged" as const,error:{kind:pagedKind(row.cause.kind),reason:row.cause.reason}}:{type:"allocation" as const,error:{kind:pagedKind(row.cause.kind),reason:row.cause.reason,allocatedBytes:row.cause.allocatedBytes!}};
   const fault:CatalogFault={cause,code:row.code,offset:BigInt(row.offset)},projected=projectCatalogFault(fault,"retained-record-body-symbol");
   const data=projected.data;if(data.kind!=="RetainedMalformed"&&data.kind!=="RetainedAllocation")throw new Error("catalog typed source erased");
   const actual={kind:projected.refusalKind,reason:data.detail,allocatedBytes:data.kind==="RetainedAllocation"?data.allocatedBytes:null};
   const reference=database.query("SELECT json_object('kind',?1,'reason',?2,'allocatedBytes',CASE WHEN ?3='allocation' THEN ?4 ELSE NULL END) AS result").get(row.cause.kind,row.cause.reason,row.cause.type,row.cause.type==="allocation"?row.cause.allocatedBytes!:null) as {result:string};
   const expected:unknown=JSON.parse(reference.result);expect(expected).toEqual(row.expected);expect(actual as unknown).toEqual(expected);expect(data.offset).toBe(7n);expect(data.what).toBe("retained-record-body-symbol");expect(fault.code).toBe(row.code);expect(fault.cause).toBe(cause);
  }
  console.info("[DEBUG] retained Catalog17 actual kind/reason/byte projections retained source authority independently of code");
 }finally{database.close();}
});

function pagedKind(kind:string):"ownershipLimit"|"allocationFailed"|"invariantViolated"{switch(kind){case "ownershipLimit":case "allocationFailed":case "invariantViolated":return kind;default:throw new Error("unknown closed PagedList kind");}}

for(const row of lifecycleFixture.cases)test(`context/source policy panic keeps cleanup ownership: ${row.phase}`,()=>{
 expect(new Ajv({strict:true}).compile(lifecycleSchema)(lifecycleFixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  let contextFrees=0,errorFrees=0,committedAfter=16,armed=false;const marker=new Error("original lifecycle panic");
  const policy={checkpoint(event:import("../🔌️capture/👥️context/🟦️.ts").ContextProgress){if(row.phase==="contextAfter"&&event.phase==="contextAfter")throw marker;return null;},progress(event:import("../🔌️capture/👥️context/🟦️.ts").ContextProgress){if((row.phase==="contextReady"&&event.phase==="reserved"&&event.category===null)||(row.phase==="published"&&event.phase==="published")||(row.phase==="closed"&&armed&&event.phase==="released"))throw marker;}};
  let context:import("../🔌️capture/👥️context/🟦️.ts").SharedContext|undefined,caught:unknown;
  try{
   const result=createSharedContext(256n,16n,32n,policy,{reserve(bytes){return bytes;},release(bytes){contextFrees++;committedAfter=Number(bytes);}},{reserve(){return{allocatedBytes:64n,source:null,references:0};},release(){errorFrees++;}});
   if(result.kind!==null)throw new Error("lifecycle context unexpectedly refused");context=result.value;
   if(row.phase==="published"){const reserve=context.reserve(64n,"nativeIo","never");if(reserve.kind!==null)throw new Error("source unexpectedly refused");reserve.value.publish(new Error("original retained source"));}
   else{armed=true;context.dispose();}
  }catch(error){caught=error;}finally{if(context)context.dispose(false);}
  expect(caught).toBe(marker);
  const reference=database.query("SELECT json_object('contextFrees',1,'errorFrees',CASE WHEN ?1='published' THEN 1 ELSE 0 END,'committedAfter',CASE WHEN ?1='contextAfter' THEN 16 WHEN ?1='published' THEN 112 ELSE 48 END) AS result").get(row.phase) as {result:string};
  const expected:unknown=JSON.parse(reference.result);expect(expected).toEqual(row.expected);expect({contextFrees,errorFrees,committedAfter} as unknown).toEqual(expected);
  console.info(`[DEBUG] lifecycle panic cleanup retained original throw at ${row.phase}`);
 }finally{database.close();}
});

for(const row of claimFixture.cases)test(`pending claim/policy unwind preserves its owned charge: ${row.scenario}`,()=>{
 expect(new Ajv({strict:true}).compile(claimSchema)(claimFixture)).toBe(true);
 const database=new Database(":memory:");
 try{
   let allocatorCalls=0,errorFrees=0,armed=false,context:import("../🔌️capture/👥️context/🟦️.ts").SharedContext;
   const marker=new Error("original policy panic");
   const policy={checkpoint(event:import("../🔌️capture/👥️context/🟦️.ts").ContextProgress){if(armed&&row.scenario==="after"&&event.phase==="after"){armed=false;throw marker;}return null;},progress(event:import("../🔌️capture/👥️context/🟦️.ts").ContextProgress){if(armed&&row.scenario==="reserved"&&event.phase==="reserved"){armed=false;throw marker;}}};
   let blocked:import("../🔌️capture/👥️context/🟦️.ts").SharedReservationResult|undefined;
   const result=createSharedContext(BigInt(row.maximumBytes),BigInt(row.ownedBefore),BigInt(row.contextBytes),policy,{reserve(bytes){return bytes;},release(){}},{reserve(){allocatorCalls++;if(row.scenario==="pending"&&allocatorCalls===1)blocked=context.reserve(BigInt(row.errorBytes),"httpBody","never");return{allocatedBytes:BigInt(row.errorBytes),source:null,references:0};},release(){errorFrees++;}});
   if(result.kind!==null)throw new Error("claim fixture context unexpectedly refused");context=result.value;
   let unrelated:SharedReservation|undefined,kind:string;
   if(row.scenario==="pending"){
    const first=context.reserve(BigInt(row.errorBytes),"nativeIo","never");
    if(first.kind!==null||blocked===undefined||blocked.kind===null)throw new Error("pending physical credit admitted a second allocator");
    kind=blocked.kind;expect(allocatorCalls).toBe(1);first.value.dispose();
   }else{
    const reserve=context.reserve(BigInt(row.errorBytes),"httpRequest","never");if(reserve.kind!==null)throw new Error("unrelated reserve failed");unrelated=reserve.value;armed=true;
    let caught:unknown;try{context.reserve(BigInt(row.errorBytes),"nativeIo","never");}catch(error){caught=error;}
    expect(caught).toBe(marker);kind="panic";
   }
   const afterAttempt=Number(context.committedBytes),next=context.reserve(BigInt(row.errorBytes),"nativeIo","never");
   if(next.kind!==null)throw new Error("successful reservation after rollback refused");
   const afterSuccess=Number(context.committedBytes);next.value.dispose();const afterUnused=Number(context.committedBytes);
   if(unrelated){const source=unrelated.publish(new Error("original unrelated cause"));source.dispose();}
   const afterFinal=Number(context.committedBytes);context.dispose();
   const reference=database.query("SELECT json_object('kind',CASE WHEN ?1='pending' THEN 'ownershipLimit' ELSE 'panic' END,'afterAttempt',?2+?3+CASE WHEN ?1='pending' THEN 0 ELSE ?4 END,'afterSuccess',?2+?3+CASE WHEN ?1='pending' THEN ?4 ELSE 2*?4 END,'afterUnused',?2+?3+CASE WHEN ?1='pending' THEN 0 ELSE ?4 END,'afterFinal',?2+?3+CASE WHEN ?1='pending' THEN 0 ELSE ?4 END,'allocatorCalls',CASE WHEN ?1='pending' THEN 2 ELSE 3 END,'errorFrees',CASE WHEN ?1='pending' THEN 2 ELSE 3 END) AS result").get(row.scenario,row.ownedBefore,row.contextBytes,row.errorBytes) as {result:string};
   const expected:unknown=JSON.parse(reference.result);expect(expected).toEqual(row.expected);
   expect({kind,afterAttempt,afterSuccess,afterUnused,afterFinal,allocatorCalls,errorFrees} as unknown).toEqual(expected);
  console.info("[DEBUG] pending credit and two policy panic boundaries preserved unrelated cumulative charges");
 }finally{database.close();}
});

test("shared context preserves original typed policy refusals at every admission boundary",()=>{
 expect(new Ajv({strict:true}).compile(policySchema)(policyFixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  for(const row of policyFixture.cases){
   let contextFrees=0,errorFrees=0,observedCharge=16;
   const original=new ValueError(typedKind(row.kind),row.message);
   const policy={checkpoint(event:import("../🔌️capture/👥️context/🟦️.ts").ContextProgress){return event.phase===row.phase?original:null;},progress(event:import("../🔌️capture/👥️context/🟦️.ts").ContextProgress){observedCharge=Number(event.committedBytes);}};
   const result=createSharedContext(256n,16n,32n,policy,{reserve(){return 32n;},release(){contextFrees++;}},{reserve(){return{allocatedBytes:64n,source:null,references:0};},release(){errorFrees++;}});
   let actualKind:ValueRefusalKind,actualCause:ValueError|null;
   if(result.kind!==null){actualKind=result.kind;actualCause=result.error;expect(result.policy).toBe(policy);}
   else{
    const reserve=result.value.reserve(64n,"nativeIo","never");
    if(reserve.kind===null)throw new Error("authored policy refusal was ignored");
    actualKind=reserve.kind;actualCause=reserve.error;result.value.dispose();
   }
   expect(actualCause).toBe(original);expect(actualKind as unknown).toBe(row.kind);
   const reference=database.query("SELECT json_object('committed',CASE WHEN ?1 IN ('contextBefore','contextAfter') THEN 16 ELSE 48 END,'contextFrees',CASE WHEN ?1='contextBefore' THEN 0 ELSE 1 END,'errorFrees',CASE WHEN ?1='after' THEN 1 ELSE 0 END) AS result").get(row.phase) as {result:string};
   const expected:unknown=JSON.parse(reference.result);
   expect(expected).toEqual({committed:row.expectedCommitted,contextFrees:row.expectedContextFrees,errorFrees:row.expectedErrorFrees});
   expect({committed:observedCharge,contextFrees,errorFrees} as unknown).toEqual(expected);
  }
  console.info("[DEBUG] shared context retained32 original typed policy causes without classification");
 }finally{database.close();}
});

test("shared context rollback preserves concurrent charges and outlives the caller through an owned source",()=>{
 expect(new Ajv({strict:true}).compile(contextSchema)(contextFixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  for(const row of contextFixture.cases){
   let contextFrees=0,errorFrees=0,closedCharge=0,observedCharge=row.ownedBefore;
   const result=createSharedContext(BigInt(row.maximumBytes),BigInt(row.ownedBefore),32n,{checkpoint(){return null;},progress(event){observedCharge=Number(event.committedBytes);}},{reserve(){return BigInt(row.contextBytes);},release(bytes){contextFrees++;closedCharge=Number(bytes);}},{reserve(){return{allocatedBytes:BigInt(row.errorBytes),source:null,references:0};},release(cell){errorFrees++;cell.source=null;}});
   if(result.kind!==null)throw new Error("closed context fixture unexpectedly refused");
   const context=result.value,committed:number[]=[],refusals:Array<"ownershipLimit"|null>=[];
   let reservationA:SharedReservation|undefined,reservationB:SharedReservation|undefined,source:import("../🔌️capture/🟦️.ts").CapturedTransport|undefined;
   const original=new Error("same deliberately misleading permanent cancellation prose");
   for(const action of row.actions){
    let refusal:"ownershipLimit"|null=null;
    switch(action){
     case "reserveA":case "reserveB":{
      const reserve=context.reserve(BigInt(row.errorBytes),"nativeIo","never");
      if(reserve.kind!==null){if(reserve.kind!=="ownershipLimit")throw new Error("unexpected shared reservation refusal");refusal=reserve.kind;}
      else if(action==="reserveA")reservationA=reserve.value;else reservationB=reserve.value;
      break;
     }
     case "releaseA":reservationA!.dispose();break;
     case "publishB":source=reservationB!.publish(original);expect(source.cause).toBe(original);break;
     case "dropCaller":context.dispose();break;
     case "dropB":source!.dispose();break;
     default:throw new Error("unknown closed context action");
    }
    committed.push(observedCharge);refusals.push(refusal);
   }
   const reference=database.query("WITH RECURSIVE actions AS (SELECT CAST(key AS INTEGER) AS ordinal,value AS action FROM json_each(?1)), ledger(ordinal,charge) AS (SELECT -1,?2 UNION ALL SELECT actions.ordinal,CASE WHEN action IN ('reserveA','reserveB') AND ledger.charge+?3<=?4 THEN ledger.charge+?3 WHEN action='releaseA' THEN ledger.charge-?3 ELSE ledger.charge END FROM ledger JOIN actions ON actions.ordinal=ledger.ordinal+1) SELECT json_group_array(charge) AS result FROM (SELECT charge FROM ledger WHERE ordinal>=0 ORDER BY ordinal)").get(JSON.stringify(row.actions),row.ownedBefore+row.contextBytes,row.errorBytes,row.maximumBytes) as {result:string};
   expect(JSON.parse(reference.result)).toEqual(row.expected.committed);
   expect({committed,refusals,contextFrees,errorFrees,closedCharge} as unknown).toEqual(row.expected);
  }
  console.info("[DEBUG] shared context interleavings preserved concurrent reservation charges and source-owned context lifetime");
 }finally{database.close();}
});

test("provider pre-reservation prevents unowned IO and publishes failures without a second reserve",()=>{
 expect(new Ajv({strict:true}).compile(providerSchema)(providerFixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  for(const row of providerFixture.cases){
   if(row.category!=="nativeIo"&&row.category!=="httpRequest"&&row.category!=="httpBody")throw new Error("unknown provider category");
   if(row.retry!=="never"&&row.retry!=="transient")throw new Error("unknown provider retry");
   let operationCalls=0,allocatorCalls=0,freeCount=0;
   const admission:CaptureAdmission={maximumBytes:BigInt(row.maximumBytes),ownedBytes:BigInt(row.ownedBefore),observe(){return true;}};
   const storage:CaptureStorage={reserve(){allocatorCalls++;return{allocatedBytes:BigInt(row.allocatedBytes),source:null,references:0};},release(cell){freeCount++;cell.source=null;}};
   const result=reserveTransport(row.category,row.retry,BigInt(row.requestedBytes),admission,storage);
   let sourcePresent=false;
   if(result.kind===null){
    operationCalls++;
    if(row.outcome==="success")result.value.dispose();
    else{
     const source=new Error("same deliberately misleading permanent cancellation prose"),error=result.value.publish(source);
     expect(error.cause).toBe(source);expect(error.retry as unknown).toBe(row.retry);
     expect(error.causeKind as unknown).toEqual({kind:"transport",category:row.category});
     const projected=new PackError({kind:"TransportFailure",error});
     expect(projected.cause).toBe(source);expect(projected.refusalKind).toBeNull();
     expect(projected.retry as unknown).toBe(row.retry);expect(projected.causeKind as unknown).toEqual({kind:"transport",category:row.category});
     sourcePresent=true;error.dispose();
    }
    expect(allocatorCalls).toBe(1);
   }else expect(allocatorCalls).toBe(0);
   const reference=database.query("SELECT json_object('kind',CASE WHEN ?1>?2-?3 THEN 'ownershipLimit' ELSE NULL END,'operationCalls',CASE WHEN ?1>?2-?3 THEN 0 ELSE 1 END,'committedAfter',?3+CASE WHEN ?1<=?2-?3 AND ?4='error' THEN ?5 ELSE 0 END,'sourcePresent',json(CASE WHEN ?1<=?2-?3 AND ?4='error' THEN 'true' ELSE 'false' END),'freeCount',CASE WHEN ?1>?2-?3 THEN 0 ELSE 1 END) AS result").get(row.requestedBytes,row.maximumBytes,row.ownedBefore,row.outcome,row.allocatedBytes) as {result:string};
   const expected:unknown=JSON.parse(reference.result);expect(expected).toEqual(row.expected);
   expect({kind:result.kind,operationCalls,committedAfter:Number(admission.ownedBytes),sourcePresent,freeCount} as unknown).toEqual(expected);
  }
  console.info("[DEBUG] draft provider pre-reservation matched three independent SQLite transitions");
 }finally{database.close();}
});

test("shared transport capture matches independent SQLite admission and rollback transitions",()=>{
 const validate=new Ajv({strict:true}).compile(captureSchema);
 expect(validate(captureFixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  for(const row of captureFixture.cases){
   if(row.category!=="nativeIo"&&row.category!=="httpRequest"&&row.category!=="httpBody")throw new Error("unknown category");
   if(row.retry!=="never"&&row.retry!=="transient")throw new Error("unknown retry");
   let allocatorCalls=0,freeCount=0,observerCalls=0,sourceReleaseCount=0,physicalBytes=0;
   const error=new Error("same deliberately misleading permanent cancellation prose");
   const admission:CaptureAdmission={maximumBytes:BigInt(row.maximumBytes),ownedBytes:BigInt(row.ownedBefore),observe(stage){observerCalls++;return stage!==row.cancelAt;}};
   const storage:CaptureStorage={reserve(){allocatorCalls++;if(row.allocatedBytes===null)return null;physicalBytes=row.allocatedBytes;return{allocatedBytes:BigInt(row.allocatedBytes),source:null,references:0};},release(cell){freeCount++;if(cell.source!==null){sourceReleaseCount++;cell.source=null;}}};
   const result=captureTransport(error,row.category,row.retry,BigInt(row.requestedBytes),admission,storage);
   const retainedBytes=result.kind===null?Number(result.value.allocatedBytes):0;
   if(result.kind===null){
    expect(result.value.cause).toBe(error);
    expect(result.value.causeKind as unknown).toEqual({kind:"transport",category:row.category});
    expect(result.value.retry as unknown).toBe(row.retry);
    const copies=Array.from({length:row.cloneCount},()=>result.value.clone());
    for(const copy of copies)expect(copy.cause).toBe(error);
    expect(allocatorCalls).toBe(1);
    result.value.dispose();for(const copy of copies)copy.dispose();
   }else{expect(result.source).toBe(error);expect(sourceReleaseCount).toBe(0);}
   const reference=database.query("WITH state AS (SELECT CASE WHEN CAST(?1 AS REAL)>CAST(?2 AS REAL) THEN 'ownershipLimit' WHEN ?3='before' THEN 'canceled' WHEN ?4 IS NULL THEN 'allocationFailed' WHEN ?4<CAST(?1 AS REAL) THEN 'invariantViolated' WHEN ?4>?2 THEN 'ownershipLimit' WHEN ?3='after' THEN 'canceled' ELSE NULL END AS refusal,CASE WHEN CAST(?1 AS REAL)>CAST(?2 AS REAL) OR ?3='before' THEN 0 ELSE COALESCE(?4,0) END AS physical) SELECT json_object('kind',refusal,'committedAfter',?5+CASE WHEN refusal IS NULL THEN physical ELSE 0 END,'retainedBytes',CASE WHEN refusal IS NULL THEN physical ELSE 0 END,'physicalBytes',physical,'allocatorCalls',CASE WHEN CAST(?1 AS REAL)>CAST(?2 AS REAL) OR ?3='before' THEN 0 ELSE 1 END,'freeCount',CASE WHEN physical>0 THEN 1 ELSE 0 END,'observerCalls',CASE WHEN CAST(?1 AS REAL)>CAST(?2 AS REAL) THEN 0 WHEN ?3='before' OR ?4 IS NULL OR ?4<CAST(?1 AS REAL) OR ?4>?2 THEN 1 ELSE 2 END,'sourceReleaseCount',CASE WHEN refusal IS NULL THEN 1 ELSE 0 END) AS result FROM state").get(row.requestedBytes,row.maximumBytes-row.ownedBefore,row.cancelAt,row.allocatedBytes,row.ownedBefore) as {result:string};
   const expected:unknown=JSON.parse(reference.result);
   expect(expected).toEqual(row.expected);
   expect({kind:result.kind,committedAfter:Number(admission.ownedBytes),retainedBytes,physicalBytes,allocatorCalls,freeCount,observerCalls,sourceReleaseCount} as unknown).toEqual(expected);
  }
  console.info("[DEBUG] draft capture matched9 injected storage transitions and preserved original Error identities");
 }finally{database.close();}
});

import {inflateRawSync} from "node:zlib";
import grammarFixture from "../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/⚠️error/🧫️fixtures/🧭️cause/📡️codec/🔣️.json";
import grammarSchema from "../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/⚠️error/🧬️schema/🧭️cause/📡️codec/🔣️.json";

test("system zlib independently confirms the closed retained DEFLATE grammar corpus",()=>{
 const validate=new Ajv({strict:true}).compile(grammarSchema);
 expect(validate(grammarFixture)).toBe(true);
 for(const row of grammarFixture.cases){
  let kind:"invalidValue"|null=null,raw:number[]=[];
  try{raw=Array.from(inflateRawSync(Uint8Array.from(row.stored)));}catch(error){
   if(!(error instanceof Error)||!("code"in error)||error.code!=="Z_DATA_ERROR")throw error;
   kind="invalidValue";
  }
  expect(kind as unknown).toBe(row.expectedKind);
  expect(raw).toEqual(row.raw);
  if(kind!==null){const projected=new PackError({kind:"RetainedMalformed",refusalKind:kind,what:"deflate",offset:0n,detail:"same deliberately misleading cancellation/limit prose"});expect(projected.refusalKind).toBe(kind);}
 }
});

import Ajv from "ajv/dist/2020.js";
import causeKindSchema from "../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/⚠️error/🧬️schema/🧭️cause/🪪️kind/🔣️.json";
import causeKindFixture from "../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/⚠️error/🧫️fixtures/🧭️cause/🪪️kind/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {PackError,ValueError,TextError,type PackErrorData,type ValueRefusalKind} from "../🟦️.ts";

const kinds=["invalidValue","canceled","ownershipLimit","allocationFailed","workLimit","depthLimit","unsupportedOwner","invariantViolated"] as const;
const typedKind=(kind:string|null):ValueRefusalKind=>{
 const value=kinds.find(value=>value===kind);
 if(!value)throw new Error("missing authored kind");
 return value;
};

function input(row:typeof fixture.cases[number]):PackErrorData {
 switch(row.variant){
  case "BadMagic":case "ContentHashMismatch":return{kind:row.variant};
  case "UnsupportedVersion":return{kind:row.variant,major:2,minor:3};
  case "UnknownRequiredFlags":return{kind:row.variant,flags:4};
  case "UnsupportedCodec":return{kind:row.variant,codec:7};
  case "Truncated":return{kind:row.variant,offset:BigInt(row.offset)};
  case "ChecksumMismatch":return{kind:row.variant,segment:row.what,offset:BigInt(row.offset)};
  case "NonCanonical":return{kind:row.variant,detail:row.message};
  case "LimitExceeded":return{kind:row.variant,refusalKind:typedKind(row.kind),limit:row.message};
  case "RetainedMalformed":case "Malformed":return{kind:row.variant,refusalKind:typedKind(row.kind),what:row.what,offset:BigInt(row.offset),detail:row.message};
  case "RetainedAllocation":if(row.allocatedBytes===null)throw new Error("missing allocation authority");return{kind:row.variant,refusalKind:typedKind(row.kind),allocatedBytes:row.allocatedBytes,what:row.what,offset:BigInt(row.offset),detail:row.message};
  case "ValueRefusal":return{kind:row.variant,error:new ValueError(typedKind(row.kind),row.message)};
  case "TextRefusal":return{kind:row.variant,error:new TextError(typedKind(row.kind),row.message,{line:2,column:3,length:1})};
  case "Io":if(row.retry!=="never"&&row.retry!=="transient")throw new Error("missing authored retry policy");return{kind:row.variant,error:new ValueError(typedKind(row.kind),row.message),retry:row.retry};
  default:throw new Error("unknown authored variant");
 }
}

test("the closed producer corpus refuses absent kinds and absent transport policy",()=>{
 const validateCause=new Ajv({strict:true}).compile(causeKindSchema);
 for(const cause of causeKindFixture)expect(validateCause(cause)).toBe(true);
 for(const cause of [{kind:"transport",category:"unknown"},{kind:"transport",category:"nativeIo",refusalKind:"invalidValue"},{kind:"refusal",refusalKind:"externalFailure"},{kind:"refusal"},{kind:"transport"}])expect(validateCause(cause)).toBe(false);
 const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(fixture)).toBe(true);
 expect(new Set(fixture.cases.map(row=>row.expectedKind))).toEqual(new Set(kinds));
 const replace=(id:string,patch:Record<string,unknown>)=>({...fixture,cases:fixture.cases.map(row=>row.id===id?{...row,...patch}:row)});
 for(const patch of [{kind:null},{kind:"unknown"},{unexpected:true}])expect(validate(replace("physical-credit",patch))).toBe(false);
 expect(validate(replace("injected-transport-workLimit-transient",{retry:null}))).toBe(false);
 expect(validate({...fixture,implicitKind:"invalidValue"})).toBe(false);
});

test("draft Pack projection matches independent SQLite output without deriving kind from prose",()=>{
 const database=new Database(":memory:");
 try{
  for(const row of fixture.cases){
   const data=input(row),error=new PackError(data);
   expect(error.causeKind as unknown).toEqual({kind:"refusal",refusalKind:row.expectedKind});
   const reference=database.query(`SELECT json_object('kind',CASE WHEN ?1 IN ('BadMagic','Truncated','ChecksumMismatch','ContentHashMismatch','NonCanonical') THEN 'invalidValue' WHEN ?1 IN ('UnsupportedVersion','UnknownRequiredFlags','UnsupportedCodec') THEN 'unsupportedOwner' ELSE ?2 END,'display',CASE ?1 WHEN 'BadMagic' THEN 'bad magic' WHEN 'UnsupportedVersion' THEN 'unsupported version 2.3' WHEN 'UnknownRequiredFlags' THEN 'unknown required feature bits 0x4' WHEN 'Truncated' THEN 'truncated at offset ' || ?4 WHEN 'ChecksumMismatch' THEN 'checksum mismatch in ' || ?5 || ' at offset ' || ?4 WHEN 'ContentHashMismatch' THEN 'content hash mismatch' WHEN 'NonCanonical' THEN 'non-canonical encoding: ' || ?3 WHEN 'UnsupportedCodec' THEN 'unsupported codec 7' WHEN 'LimitExceeded' THEN 'limit exceeded: ' || ?3 WHEN 'ValueRefusal' THEN 'schema error: ' || ?3 WHEN 'TextRefusal' THEN 'schema error: ' || ?3 || ' at 2:3' WHEN 'Io' THEN 'io error: ' || ?3 ELSE 'malformed ' || ?5 || ' at offset ' || ?4 || ': ' || ?3 END,'retry',?6,'allocatedBytes',?7) AS result`).get(row.variant,row.kind,row.message,row.offset,row.what,row.retry,row.allocatedBytes) as {result:string};
   const expected:unknown=JSON.parse(reference.result);
   expect(expected).toEqual({kind:row.expectedKind,display:row.display,retry:row.retry,allocatedBytes:row.allocatedBytes});
   expect({kind:error.refusalKind,display:error.message,retry:error.retry,allocatedBytes:data.kind==="RetainedAllocation"?data.allocatedBytes:null} as unknown).toEqual(expected);
   if(data.kind==="ValueRefusal"||data.kind==="TextRefusal"||data.kind==="Io")expect(error.cause).toBe(data.error);
   if(data.kind==="RetainedAllocation")expect(data.allocatedBytes).toBe(row.allocatedBytes!);
  }
  expect(new Set(fixture.cases.filter(row=>row.variant==="RetainedMalformed"||row.variant==="RetainedAllocation").map(row=>row.expectedKind)).size).toBeGreaterThan(3);
 }finally{database.close();}
});
import pagedFixture from "../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/⚠️error/🧫️fixtures/🧭️cause/📋️paged/🔣️.json";
import pagedSchema from "../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/⚠️error/🧬️schema/🧭️cause/📋️paged/🔣️.json";

test("borrowed factories preserve lower kinds and byte witnesses against independent SQLite",()=>{
 const validate=new Ajv({strict:true}).compile(pagedSchema);
 expect(validate(pagedFixture)).toBe(true);
 for(const patch of [{kind:"canceled"},{allocatedBytes:-1},{unexpected:true},{expectedKind:"invalidValue"}])expect(validate({...pagedFixture,cases:pagedFixture.cases.map((row,index)=>index===3?{...row,...patch}:row)})).toBe(false);
 const database=new Database(":memory:");
 try{
  for(const row of pagedFixture.cases){
   if(row.kind!=="ownershipLimit"&&row.kind!=="allocationFailed"&&row.kind!=="invariantViolated")throw new Error("unknown lower kind");
   const source:{kind:"ownershipLimit"|"allocationFailed"|"invariantViolated";reason:string}={kind:row.kind,reason:row.reason};
   const error=row.factory==="refusal"?PackError.fromPagedRefusal(source,"paged",71n):PackError.fromPagedAllocation({...source,allocatedBytes:row.allocatedBytes!},"paged",71n);
   const reference=database.query("SELECT json_object('kind',?1,'allocatedBytes',CASE ?2 WHEN 'allocation' THEN ?3 ELSE NULL END,'detail',?4,'what','paged','offset','71') AS result").get(row.kind,row.factory,row.allocatedBytes,row.reason) as {result:string};
   const expected:unknown=JSON.parse(reference.result);
   if(error.data.kind!=="RetainedMalformed"&&error.data.kind!=="RetainedAllocation")throw new Error("borrowed metadata was erased");
   expect({kind:error.refusalKind,allocatedBytes:error.data.kind==="RetainedAllocation"?error.data.allocatedBytes:null,detail:error.data.detail,what:error.data.what,offset:error.data.offset.toString()} as unknown).toEqual(expected);
   expect(error.refusalKind as unknown).toBe(row.expectedKind);
   expect(error.cause).toBeUndefined();
  }
 }finally{database.close();}
});
