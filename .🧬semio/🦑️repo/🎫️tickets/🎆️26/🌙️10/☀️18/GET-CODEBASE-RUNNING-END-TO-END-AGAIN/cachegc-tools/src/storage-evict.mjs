
/** 🪫️ Brings the optional cache down to its low-water mark before a writer inserts, instead of declining every later record: least recently written groups first, each under its own non-blocking writer lease, never the writer's own namespace or lease shard; returns whether anything was retired. */
async function evictOptionalGraphLruV1(workspace,root,own,namespace,control){
 const{retireBuildGraphOptionalGroupsLruV1}=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts"),directory=repositoryLeaseDirectoryV1(workspace),[kind,identity]=namespace?namespace.split(":"):["canonical",""];
 const lease=async(group,id)=>{const resource=group==="canonical"?"repository-graph-facts:"+resolve(workspace):buildGraphWriterResourceV1(workspace,group,id);if(resource===own)return undefined;try{return await acquireResourceLease({directory,resource,mode:"exclusive",timeoutMs:0,signal:control.signal,control});}catch(error){checkResourceLeaseControl(control);if(error instanceof Error&&error.message==="Resource lease timed out: "+resource)return undefined;throw error;}};
 const report=await retireBuildGraphOptionalGroupsLruV1(root,{lease},{cancelled:()=>control.signal.aborted,progress:async()=>{checkResourceLeaseControl(control);await control.publish({resource:"repository:graph-optional-lru",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}},{exclude:[kind==="selection"?"selections/"+identity:kind==="dataset"?"contexts/"+identity:"canonical"]});
 checkResourceLeaseControl(control);return report.retiredFiles>0;
}
