
/** 📉️ What one least-recently-used pass over the optional graph cache did: its bytes before and after (the logical size of every optional leaf, the measure of the optional ledger) against the ledger's fixed limit and the pass's target, and how many groups were retired, busy (their writer is taken), in use (an optional leaf written within the protected idle time) or excluded (the caller's own namespace). */
export type OptionalLruReportV1=Readonly<{usedBytes:number;maximumBytes:number;targetBytes:number;remainingBytes:number;groups:number;retiredGroups:number;retiredFiles:number;retiredBytes:number;busy:number;protected:number;excluded:number;cancelled:boolean}>;
type OptionalLeafV1=Readonly<{name:string;path:string;bytes:number;rank:number}>;
const optionalLeafPatternV1=new RegExp(graphStorageSchema.definitions.OptionalQuota.optionalPath,"u");
const optionalDirectoryOfV1=(root:string,group:string)=>join(root,"inference",...(group==="canonical"?[]:group.split("/")));

/** 🗃️ Lists the optional leaves of one group (index files, receipts and publications never match the optional pattern), records before the packs they index and observation blocks last, so a pass cut short never leaves a record naming a removed block. */
function optionalLeavesOfV1(root:string,group:string):{leaves:OptionalLeafV1[];newest:number}{
 const directory=optionalDirectoryOfV1(root,group),prefix=group==="canonical"?"":group+"/",leaves:OptionalLeafV1[]=[],pending=[""];let newest=0;
 while(pending.length){
  const at=pending.pop()!;let entries;try{entries=readdirSync(join(directory,at),{withFileTypes:true});}catch(error){if(absent(error))continue;throw error;}
  for(const entry of entries){
   const name=at?at+"/"+entry.name:entry.name;
   if(entry.isDirectory()){if(group==="canonical"&&!at&&(entry.name==="selections"||entry.name==="contexts"))continue;pending.push(name);continue;}
   if(!entry.isFile()||!optionalLeafPatternV1.test(prefix+name))continue;
   const path=join(directory,name);let row;try{row=lstatSync(path);}catch(error){if(absent(error))continue;throw error;}
   if(!row.isFile()||row.nlink!==1)continue;newest=Math.max(newest,row.mtimeMs);
   leaves.push({name,path,bytes:row.size,rank:/(?:^|\/)ob\//u.test(name)?2:/\.pack(?:\.[a-f0-9]{8}\.tmp)?$/u.test(name)?1:0});
  }
 }
 return{leaves:leaves.sort((left,right)=>left.rank-right.rank||(left.name<right.name?-1:left.name>right.name?1:0)),newest};
}

/** ♻️ Retires optional graph cache groups least recently written first until the optional bytes are at most `targetBytes` (the ledger's low-water mark unless given), so an insertion into a full cache stores its records instead of being declined. Optional leaves are recomputable accelerators that no generation, pin or mandatory publication names; a group is retired only under its non-blocking writer lease (every reader and writer of a namespace holds it, so a busy group is skipped, never waited for), never when it is the caller's own (`exclude`) and never while an optional leaf of it is younger than the protected idle time. Its member index is compacted to the leaves that remain. Reports progress per group and stops at the next group when cancelled; `dryRun` only plans. */
export async function retireBuildGraphOptionalGroupsLruV1(root:string,ports:Pick<MemberIndexPortsV1,"lease">,control:MemberScanControlV1,{targetBytes=graphStorageSchema.definitions.OptionalQuota.policy.lowWaterBytes,dryRun=false,exclude=[],protectedMilliseconds=M.groupProtectedIdleMilliseconds,now=Date.now()}:Readonly<{targetBytes?:number;dryRun?:boolean;exclude?:readonly string[];protectedMilliseconds?:number;now?:number}>):Promise<OptionalLruReportV1>{
 if(!Number.isSafeInteger(targetBytes)||targetBytes<0)throw Error("Exact optional graph target bytes required");
 const groups=["canonical",...listBuildGraphMemberGroupsV1(root).filter(group=>group.startsWith("selections/")||group.startsWith("contexts/"))].map(group=>{const{leaves,newest}=optionalLeavesOfV1(root,group);return{group,newest,bytes:leaves.reduce((sum,leaf)=>sum+leaf.bytes,0)};}).filter(row=>row.bytes>0).sort((left,right)=>left.newest-right.newest||(left.group<right.group?-1:left.group>right.group?1:0));
 const usedBytes=groups.reduce((sum,row)=>sum+row.bytes,0);let remaining=usedBytes,retiredGroups=0,retiredFiles=0,retiredBytes=0,busy=0,protectedGroups=0,excluded=0,cancelled=false;
 for(let at=0;at<groups.length&&remaining>targetBytes;at++){
  if(control.cancelled()){cancelled=true;break;}
  const row=groups[at]!;await control.progress(row.group,at,groups.length);await turn();
  if(exclude.includes(row.group)){excluded++;continue;}
  if(memberGroupProtectedV1(row.newest,now,protectedMilliseconds)){protectedGroups++;continue;}
  if(dryRun){retiredGroups++;retiredBytes+=row.bytes;remaining-=row.bytes;continue;}
  const[kind,identity]=row.group.split("/") as [string,string|undefined],lease=await ports.lease(kind==="contexts"?"dataset":kind==="selections"?"selection":"canonical",identity??"");
  if(!lease){busy++;continue;}
  try{
   const{leaves,newest}=optionalLeavesOfV1(root,row.group);
   if(memberGroupProtectedV1(newest,Date.now(),protectedMilliseconds)){protectedGroups++;continue;}
   let bytes=0;for(const leaf of leaves){try{unlinkSync(leaf.path);}catch(error){if(absent(error))continue;throw error;}retiredFiles++;bytes+=leaf.bytes;}
   const target=join(root,...directoryOf(row.group).split("/"),MEMBER_INDEX_FILE_V1),index=parseMemberIndexV1(indexText(root,row.group),row.group);
   if(index.valid&&index.count){const live=new Map(leavesOf(root,row.group).map(leaf=>[leaf.name,leaf.stamp])),temporary=target+".tmp";writeFileSync(temporary,compactMemberIndexV1(index,row.group,live));renameSync(temporary,target);}
   retiredGroups++;retiredBytes+=bytes;remaining-=row.bytes;
  }finally{lease.release();}
 }
 return Object.freeze({usedBytes,maximumBytes:graphStorageSchema.definitions.OptionalQuota.policy.maximumBytes,targetBytes,remainingBytes:Math.max(0,remaining),groups:groups.length,retiredGroups,retiredFiles,retiredBytes,busy,protected:protectedGroups,excluded,cancelled});
}
