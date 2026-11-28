import sys,os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def edit(rel,pairs,revert=False):
    global check
    p=B+rel;s=open(p,encoding="utf-8").read()
    for old,new in pairs:
        a,b=(new,old) if revert else (old,new)
        if s.count(a)!=1:sys.exit("anchor count %d in %s: %s"%(s.count(a),rel,a[:70]))
        s=s.replace(a,b)
    if check:return
    t=p+".h43.tmp";open(t,"w",encoding="utf-8").write(s);os.replace(t,p)
revert=len(sys.argv)>1 and sys.argv[1]=="revert"
check=len(sys.argv)>1 and sys.argv[1]=="check"
st="🏗️build/📥️inputs/💾️storage/🟨️.mjs"
edit(st,[
 ("const leasePhase=phaseControl,leases=await openRepositoryCachePartitionV1(workspace,schema.definitions.GraphWriterLeaseProtocol.const.partition,leasePhase);let leaseFailed=false;",
  "const t0=performance.now(),dbg=label=>process.stderr.write(\"[DEBUG] graph-writer \"+label+\" +\"+Math.round(performance.now()-t0)+\"ms\\n\");const leasePhase=phaseControl,leases=await openRepositoryCachePartitionV1(workspace,schema.definitions.GraphWriterLeaseProtocol.const.partition,leasePhase);dbg(\"leases-open\");let leaseFailed=false;"),
 ("  if(!current&&optional)await maintain(false);\n","  if(!current&&optional)await maintain(false);dbg(\"maintain\");\n"),
 ("    const checkActual=()=>{check();requireIssuedResourceControlRefinementV1(leasePhase,actualControl);checkResourceLeaseControl(actualControl);};checkActual();\n",
  "    const checkActual=()=>{check();requireIssuedResourceControlRefinementV1(leasePhase,actualControl);checkResourceLeaseControl(actualControl);};checkActual();dbg(\"store-open\");\n"),
 ("    let primary;\n    try{\n     await leases.recheck();checkActual();\n     await original.recheck();checkActual();const status=statSync(original.root);let ledger=await openOriginalGraphLedgerV1(workspace,original,actualControl),failed=false;",
  "    let primary;dbg(\"lease\");\n    try{\n     await leases.recheck();checkActual();\n     await original.recheck();checkActual();const status=statSync(original.root);dbg(\"rechecked\");let ledger=await openOriginalGraphLedgerV1(workspace,original,actualControl),failed=false;dbg(\"ledger\");"),
 ("ledger.close();ledger=await openOriginalGraphLedgerV1(workspace,original,actualControl);checkActual();observed=ledger.observe();}const space","ledger.close();ledger=await openOriginalGraphLedgerV1(workspace,original,actualControl);checkActual();observed=ledger.observe();dbg(\"evicted\");}const space"),
],revert)
rs="⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts"
edit(rs,[
 ("export async function openRepositoryCachePartitionV1(workspace:string,id:RepositoryCachePartitionIdV1,control:ResourceLeaseControl):Promise<NativeImageStorageV1>{\n checkResourceLeaseControl(control);",
  "export async function openRepositoryCachePartitionV1(workspace:string,id:RepositoryCachePartitionIdV1,control:ResourceLeaseControl):Promise<NativeImageStorageV1>{\n const started=performance.now();try{return await openRepositoryCachePartitionDebugV1(workspace,id,control);}finally{process.stderr.write(\"[DEBUG] open-partition \"+id+\" \"+Math.round(performance.now()-started)+\"ms\\n\");}\n}\nasync function openRepositoryCachePartitionDebugV1(workspace:string,id:RepositoryCachePartitionIdV1,control:ResourceLeaseControl):Promise<NativeImageStorageV1>{\n checkResourceLeaseControl(control);"),
],revert)
print("reverted" if revert else "applied")
