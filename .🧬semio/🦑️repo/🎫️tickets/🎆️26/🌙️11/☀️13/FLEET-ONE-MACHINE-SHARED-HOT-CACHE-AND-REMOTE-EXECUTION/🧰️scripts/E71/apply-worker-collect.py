B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/"
p=B+"👷️worker/🟦️.ts"
s=open(p,encoding="utf8").read()
a=s.index("/** 🧾️ Collects the workers' home for `repo cache gc`")
b=s.index("/** 📮️ Talks to the scheduler's worker API. */")
new='''/** 🧾️ Collects the workers' home for `repo cache gc`: trims the shared blob cache to its bound and retires the private input roots of workers that are gone (their owner record names a dead process; a folder without one after `ownerlessWorkSeconds`); answers one language-neutral line for the ledger report. The home of the area `remote-execution-worker` (the sidecar agents' home inside its partition image) is the one the agents write, the area `remote-execution` is the checkout directory of the policy; a home whose image is not attached answers `absent`. */
export async function collectRemoteExecutionForLedgerV1(workspace:string,dryRun:boolean,signal:AbortSignal,area:string="remote-execution"):Promise<string>{
 const policy=readRemoteExecutionPolicyV1().worker;
 let home:string;
 if(area==="remote-execution-worker"){
  try{home=(await import("../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts")).cacheAreaHomeV1(workspace,area).directory;}catch{return "absent";}
 }else home=isAbsolute(policy.directory)?policy.directory:join(workspace,policy.directory);
 const cache=new RemoteExecutionBlobCacheV1(join(home,"blobs"),undefined as never,policy.maximumBlobBytes);
 if(!await stat(home).then(info=>info.isDirectory(),()=>false))return "absent";
 const retired=await retireRemoteExecutionWorkV1(join(home,"work"),{ownerlessSeconds:policy.ownerlessWorkSeconds,dryRun,signal});
 if(signal.aborted)return "cancelled";
 const removed=await cache.prune(dryRun);
 return `${dryRun?"would remove":"removed"} ${removed} blobs and ${retired.roots} input roots of ${retired.workers} dead workers`;
}

'''
s=s[:a]+new+s[b:]
open(p,"w",encoding="utf8").write(s)
