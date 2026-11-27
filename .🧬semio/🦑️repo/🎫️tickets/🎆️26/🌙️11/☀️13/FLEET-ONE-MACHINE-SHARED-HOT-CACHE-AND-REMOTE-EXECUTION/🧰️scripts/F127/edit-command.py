import os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def patch(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:80],s.count(old));s=s.replace(old,new)
    tmp=path+".f127tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
C=B+"🏗️build/🧹️cache-gc/⌨️command/🟦️.ts"
patch(C,[
('import {cacheRunnerPortsV1,collectCacheV1,loadCacheSizesV1,','import {cacheRunnerPortsV1,collectCacheV1,createCacheImagesV1,loadCacheSizesV1,'),
('opening.repositoryLeaseDirectoryV1(root);\n const ports:CacheRunnerPortsV1=cacheRunnerPortsV1({\n  signal:invocation.control.signal,\n',
 'opening.repositoryLeaseDirectoryV1(root);\n const images=createCacheImagesV1({workspace:root,partitions:budgetPolicy.partitions,leaseDirectory:directory,signal:invocation.control.signal,timeoutMs:CACHE_GC_LIMITS_V1.imageReadLeaseWaitMilliseconds});\n const ports:CacheRunnerPortsV1=cacheRunnerPortsV1({\n  signal:invocation.control.signal,\n  images,\n'),
('module.collectRemoteExecutionForLedgerV1(root,dryRun,phaseControl.signal,id))','module.collectRemoteExecutionForLedgerV1(root,dryRun,phaseControl.signal,id,images))'),
('''  await import("../../🌐️remote-execution/👷️worker/🟦️.ts").then(async module=>writeResultLineV1("Remote execution worker home: "+await module.collectRemoteExecutionForLedgerV1(root,false,signal,"remote-execution-worker")),()=>undefined).catch(()=>undefined);
''','''  await import("../../🌐️remote-execution/👷️worker/🟦️.ts").then(async module=>writeResultLineV1("Remote execution worker home: "+await module.collectRemoteExecutionForLedgerV1(root,false,signal,"remote-execution-worker",images)),()=>undefined).catch(()=>undefined);
  await images.settle();
'''),
(' const code=await runCacheLedgerCommandV1(args,{workspace:root,environment:process.env,write:writeResultLineV1,verify,leaseDirectory:directory},ports);',
 ' const code=await runCacheLedgerCommandV1(args,{workspace:root,environment:process.env,write:writeResultLineV1,verify,leaseDirectory:directory},ports).finally(()=>images.settle());'),
])
print("command ok")
