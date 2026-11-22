p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts"
s=open(p,encoding="utf8").read()
def rep(old,new):
    global s
    assert s.count(old)==1,old[:70]
    s=s.replace(old,new)
rep("module.collectRemoteExecutionForLedgerV1(root,dryRun,phaseControl.signal))","module.collectRemoteExecutionForLedgerV1(root,dryRun,phaseControl.signal,id))")
rep(''' if(idleOnly){await collectDependencies(false);await reclaimImages(''',''' const retireIdle=async():Promise<void>=>{
  const signal=invocation.control.signal;
  await import("../../🌐️remote-execution/👷️worker/🟦️.ts").then(async module=>writeResultLineV1("Remote execution worker home: "+await module.collectRemoteExecutionForLedgerV1(root,false,signal,"remote-execution-worker")),()=>undefined).catch(()=>undefined);
  const registry=await import("../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts"),order=await import("../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts");
  const pruned=await registry.pruneLeaseRegistryIfDueV1(registry.leaseRegistryRootV1(),order.leaseOrderPolicyV1(process.env),Date.now(),signal).catch(()=>undefined);
  if(pruned!==undefined)writeResultLineV1("Lease registry: removed "+pruned.removedFolders+" folders and "+pruned.removedEntries+" rows"+(pruned.more?" (more follow)":"")+" / Lease-Register: "+pruned.removedFolders+" Ordner und "+pruned.removedEntries+" Zeilen entfernt"+(pruned.more?" (weitere folgen)":""));
 };
 if(idleOnly){await collectDependencies(false);await retireIdle();await reclaimImages(''')
open(p,"w",encoding="utf8").write(s)
