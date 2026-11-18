import sys
p="work/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts"
s=open(p,encoding="utf8").read()
a=s.index("/** 📉️ What one in-group pass did")
b=s.index("/** 🗜️ Rewrites the member index of a group")
new='''/** 📉️ What one in-group pass did: the group's bytes plus the bytes of the other groups before and after, against the target, how many units lost only their accelerator packs (`shedUnits`, `shedBytes`) and how many were retired whole or protected. */
export type OptionalEntriesReportV1=Readonly<{usedBytes:number;targetBytes:number;remainingBytes:number;entries:number;shedUnits:number;shedBytes:number;retiredEntries:number;retiredFiles:number;retiredBytes:number;protected:number;cancelled:boolean}>;

/** 🏎️ Whether an optional leaf only speeds a recomputation up (the fact packs under `<record>.records/` and the memory packs of `mr/`): losing it costs time, never an answer, while a record header with its result is the answer a later read takes without inferring. */
export const optionalAcceleratorV1=(name:string):boolean=>/\\.records\\/|^mr\\//u.test(name);

/** ♻️ Retires the least recently written units of ONE group (the writer's own, whose group a pass over other groups must keep) until its bytes plus `otherBytes` are at most `targetBytes`, so a full cache whose bytes sit in the live group still takes new records instead of declining every publication for ever. Two tiers, each oldest first: first the accelerator packs of every unit (a finished owner keeps its record), then, only when that is not enough, whole units, header first and packs after, so a pass cut short never leaves a header naming a removed pack. A unit written within the protected idle time stays; the member index is compacted to the leaves that remain. The caller holds the group's writer lease. */
export async function retireBuildGraphOptionalEntriesLruV1(root:string,group:string,control:MemberScanControlV1,{targetBytes=graphStorageSchema.definitions.OptionalQuota.definitions.DefaultPolicy.const.lowWaterBytes,otherBytes=0,dryRun=false,protectedMilliseconds=M.groupProtectedIdleMilliseconds,now=Date.now()}:Readonly<{targetBytes?:number;otherBytes?:number;dryRun?:boolean;protectedMilliseconds?:number;now?:number}>={}):Promise<OptionalEntriesReportV1>{
 if(!Number.isSafeInteger(targetBytes)||targetBytes<0||!Number.isSafeInteger(otherBytes)||otherBytes<0)throw Error("exact-optional-graph-target-bytes-required");
 const{leaves}=optionalLeavesOfV1(root,group),units=new Map<string,{key:string;newest:number;bytes:number;leaves:OptionalLeafV1[]}>();
 for(const leaf of leaves){const key=optionalEntryOfV1(leaf.name);if(key===undefined)continue;const unit=units.get(key)??{key,newest:0,bytes:0,leaves:[]};unit.newest=Math.max(unit.newest,leaf.mtimeMs);unit.bytes+=leaf.bytes;unit.leaves.push(leaf);units.set(key,unit);}
 const order=[...units.values()].sort((left,right)=>left.newest-right.newest||(left.key<right.key?-1:left.key>right.key?1:0)),usedBytes=leaves.reduce((sum,leaf)=>sum+leaf.bytes,0)+otherBytes,byRank=(left:OptionalLeafV1,right:OptionalLeafV1)=>left.rank-right.rank||(left.name<right.name?-1:left.name>right.name?1:0);
 let remaining=usedBytes,retiredEntries=0,retiredFiles=0,retiredBytes=0,shedUnits=0,shedBytes=0,protectedEntries=0,cancelled=false;
 const drop=(list:OptionalLeafV1[])=>{if(dryRun)return;for(const leaf of list.sort(byRank)){try{unlinkSync(leaf.path);}catch(error){if(absent(error))continue;throw error;}retiredFiles++;}};
 for(let at=0;at<order.length&&remaining>targetBytes;at++){
  if(control.cancelled()){cancelled=true;break;}
  const unit=order[at]!,fast=unit.leaves.filter(leaf=>optionalAcceleratorV1(leaf.name));
  if(!fast.length||memberGroupProtectedV1(unit.newest,now,protectedMilliseconds))continue;
  await control.progress(group+"/"+unit.key,at,order.length);await turn();
  const freed=fast.reduce((sum,leaf)=>sum+leaf.bytes,0);drop(fast);
  shedUnits++;shedBytes+=freed;remaining-=freed;unit.bytes-=freed;unit.leaves=unit.leaves.filter(leaf=>!optionalAcceleratorV1(leaf.name));
 }
 for(let at=0;at<order.length&&remaining>targetBytes&&!cancelled;at++){
  if(control.cancelled()){cancelled=true;break;}
  const unit=order[at]!;if(!unit.leaves.length)continue;await control.progress(group+"/"+unit.key,at,order.length);await turn();
  if(memberGroupProtectedV1(unit.newest,now,protectedMilliseconds)){protectedEntries++;continue;}
  drop(unit.leaves);
  retiredEntries++;retiredBytes+=unit.bytes;remaining-=unit.bytes;
 }
 if(!dryRun&&retiredFiles>0)compactGroupIndexV1(root,group);
 return Object.freeze({usedBytes,targetBytes,remainingBytes:Math.max(0,remaining),entries:order.length,shedUnits,shedBytes,retiredEntries,retiredFiles,retiredBytes,protected:protectedEntries,cancelled});
}

'''
# keep the optionalEntryOfV1 + OptionalEntries... wait: optionalEntryOfV1 is between the two anchors
mid_start=s.index("/** 🧺️ The retirement unit of an optional leaf")
mid_end=s.index("/** 📉️ What one in-group pass did")
assert mid_start<mid_end
s=s[:a]+new+s[b:]
open(p,"w",encoding="utf8").write(s)
