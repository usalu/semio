import os,sys
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/"
def patch(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:60],s.count(old))
        s=s.replace(old,new)
    open(path+".f121tmp","w",encoding="utf8").write(s);os.replace(path+".f121tmp",path)
patch(B+"🧬️schema/🔣️.json",[
 ('        "graphLeaseWaitMilliseconds": 600000,\n','        "graphLeaseWaitMilliseconds": 600000,\n        "imageReadLeaseWaitMilliseconds": 30000,\n'),
 ('          "graphLeaseWaitMilliseconds": "How long the graph reclaim','          "imageReadLeaseWaitMilliseconds": "How long a collector waits for the shared lease of a cache image before its read (the tools census, the dependency collection) gives up behind a waiting exclusive writer (30 seconds).",\n          "graphLeaseWaitMilliseconds": "How long the graph reclaim'),
 ('          "graphLeaseWaitMilliseconds": "Wie lange die Graph-Rückgewinnung','          "imageReadLeaseWaitMilliseconds": "Wie lange ein Sammler auf die geteilte Lease eines Cache-Images wartet, bevor sein Lesen (die Werkzeug-Zählung, die Abhängigkeitssammlung) hinter einem wartenden exklusiven Schreiber aufgibt (30 Sekunden).",\n          "graphLeaseWaitMilliseconds": "Wie lange die Graph-Rückgewinnung'),
])
patch(B+"⌨️command/🟦️.ts",[
 ('import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";\n','import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";\nimport {withNativeImageSharedReadV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts";\n'),
 ('census:()=>censusNxToolingStoreV1(root,mount,{signal,progress}),','census:()=>withNativeImageSharedReadV1({leaseDirectory,imageDirectory:join(root,budgetPolicy.partitions.tools.directory),signal,timeoutMs:CACHE_GC_LIMITS_V1.imageReadLeaseWaitMilliseconds},()=>censusNxToolingStoreV1(root,mount,{signal,progress})),'),
 ('   const result=await collectUnpinnedDependencyGenerationsV1(store,{dryRun,','   const result=await withNativeImageSharedReadV1({leaseDirectory:directory,imageDirectory:join(root,...budgetPolicy.partitions.dependencies.directory.split("/")),signal:invocation.control.signal,timeoutMs:CACHE_GC_LIMITS_V1.imageReadLeaseWaitMilliseconds},()=>collectUnpinnedDependencyGenerationsV1(store,{dryRun,'),
 ('throw error;}}});\n   if(result.refused!==undefined)','throw error;}}}));\n   if(result.refused!==undefined)'),
])
