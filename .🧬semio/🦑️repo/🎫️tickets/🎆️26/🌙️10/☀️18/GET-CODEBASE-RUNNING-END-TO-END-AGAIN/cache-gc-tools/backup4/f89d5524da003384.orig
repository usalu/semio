import {cacheTextV1,formatCacheBytesV1,type CacheLanguageV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";

/** 🔌️ What the reclaim needs from the repository: the single-flight lease, the retained graph partition owner, its free space and the two shared recoveries; tests replace them. */
export type GraphReclaimPortsV1=Readonly<{
 exclusive(resource:string):Promise<Readonly<{release():void}>|undefined>;
 open():Promise<Readonly<{close():Promise<void>}>>;
 free(image:object):Promise<number>;
 selected(image:object,request:Readonly<{minimumAvailableBytes:number;wait:false}>):Promise<unknown>;
 optional(image:object,request:Readonly<{minimumAvailableBytes:number}>):Promise<unknown>;
}>;

/** 🚰️ Reclaims optional graph caches up to the ledger's target free space, in the background and under the shared hold of the retained graph image: scope by scope, each under its own non-blocking writer lease, never the exclusive maintenance of the image. Returns a localized report line. */
export async function reclaimGraphOptionalV1(workspace:string,ledger:CacheLedgerV1,dryRun:boolean,language:CacheLanguageV1,ports:GraphReclaimPortsV1):Promise<string>{
 const text=(key:string,variables:Readonly<Record<string,string|number>>={})=>cacheTextV1(language,key,variables),reclaim=ledger.areas.find(row=>row.id==="graph-inference")?.reclaim;
 if(!reclaim)return text("gc.graph.undeclared");
 const lease=await ports.exclusive("repository-graph-optional-collector:"+workspace);
 if(!lease)return text("gc.graph.running");
 let image:Awaited<ReturnType<GraphReclaimPortsV1["open"]>>|undefined;
 try{
  image=await ports.open();const before=await ports.free(image);
  if(before>=reclaim.targetAvailableBytes)return text("gc.graph.enough",{free:formatCacheBytesV1(before),target:formatCacheBytesV1(reclaim.targetAvailableBytes)});
  if(dryRun)return text("gc.graph.plan",{free:formatCacheBytesV1(before),target:formatCacheBytesV1(reclaim.targetAvailableBytes)});
  await ports.selected(image,{minimumAvailableBytes:reclaim.targetAvailableBytes,wait:false});
  if(await ports.free(image)<reclaim.targetAvailableBytes)await ports.optional(image,{minimumAvailableBytes:reclaim.targetAvailableBytes});
  const after=await ports.free(image);
  return text("gc.graph.done",{freed:formatCacheBytesV1(Math.max(0,after-before)),free:formatCacheBytesV1(after),target:formatCacheBytesV1(reclaim.targetAvailableBytes)});
 }finally{
  try{await image?.close();}finally{lease.release();}
 }
}
