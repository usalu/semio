import {laneMessageV1,fleetRemoteEnvironmentV1} from "../../../🚪️launch/🟦️.ts";
import {fleetClientV1} from "../../🟦️.ts";
import {clearSeedV1,finishSeedV1,seedClosureIdV1,seedFingerprintV1,seedLaneV1,seedRunReportV1,type SeedGraphV1,type SeedStatsV1} from "../🟦️.ts";

/** 🌱️ What a lane needs to be seeded: the cache root and the lane, the planned graph of the request, the launch environment (the fleet tier is read from it like the wrappers read it), the language of the operator, where lines go and the signal that cancels, and whether the lane is cold (new, or reused for other crates): only a cold lane is prefetched, a warm one only reads the record. */
export type SeededLaneInputV1=Readonly<{cacheRoot:string;laneRoot:string;lane:string;graph:SeedGraphV1;environment:Readonly<Record<string,string|undefined>>;language:"en"|"de";report:(line:string)=>void;signal:AbortSignal;cold:boolean}>;

const mebibytes=(bytes:number):number=>Math.round(bytes/1048576);

/** 🌱️ Runs the compile of a lane seeded from the fleet cache: before `body` (which starts Cargo) the unit closure of the request is prefetched in parallel (see {@link seedLaneV1}) so no wrapper discovers a fleet unit one round trip at a time; after it, when it succeeded, the closure record of the request is published or grown, one line says what the run did with its units (local hits, fleet fetches, compiles, offloads) and the seed is removed. A request nobody ran yet has no record: it runs as before and leaves one. Seed trouble never fails the run. */
export async function withSeededLaneV1<T>(input:SeededLaneInputV1,body:()=>Promise<T>):Promise<T>{
 const sinceMs=Date.now(),say=(kind:Parameters<typeof laneMessageV1>[0],parameters:Readonly<Record<string,string|number>>)=>input.report(laneMessageV1(kind,parameters,input.language));
 let client:ReturnType<typeof fleetClientV1>=undefined,closureId="",toolchain="",record:readonly string[]=[];
 try{
  client=fleetClientV1(input.cacheRoot,{...input.environment,...fleetRemoteEnvironmentV1(input.environment),SEMIO_UNIT_CACHE_LANE_ROOT:undefined});
  closureId=seedClosureIdV1(seedFingerprintV1(input.graph));toolchain=input.graph.units.find(unit=>unit.compiler!==null)?.compiler?.digest??"";
  clearSeedV1(input.laneRoot);
  const progress=(stats:Readonly<SeedStatsV1>)=>say("seedProgress",{done:stats.done,total:stats.slots,mebibytes:mebibytes(stats.bytes),seconds:Math.round((Date.now()-sinceMs)/1000)});
  const seeded=await seedLaneV1({client,cacheRoot:input.cacheRoot,laneRoot:input.laneRoot,closureId,signal:input.signal,prefetch:input.cold,progress});record=seeded.record;
  if(seeded.stats.state==="no-record"&&client!==undefined)say("seedNone",{reason:client.usable()?"none":client.state()});
  else if(input.cold&&seeded.stats.slots>0)say("seedDone",{fetched:seeded.stats.fetched,total:seeded.stats.slots,blobs:seeded.stats.blobs,mebibytes:mebibytes(seeded.stats.bytes),seconds:Math.round(seeded.stats.milliseconds/1000),absent:seeded.stats.absent,stopped:seeded.stats.stopped?" ("+seeded.stats.state+")":""});
 }catch{}
 let succeeded=false;
 try{const result=await body();succeeded=typeof result!=="number"||result===0;return result;}
 finally{
  try{
   const published=await finishSeedV1({client,laneRoot:input.laneRoot,closureId,record,toolchain,succeeded});
   if(published.published)say("seedPublished",{slots:published.slots,added:published.added});
   const report=seedRunReportV1({cacheRoot:input.cacheRoot,lane:input.lane,laneRoot:input.laneRoot,sinceMs});
   if(report.total>0)say("runReport",report);
  }catch{}finally{clearSeedV1(input.laneRoot);}
 }
}
