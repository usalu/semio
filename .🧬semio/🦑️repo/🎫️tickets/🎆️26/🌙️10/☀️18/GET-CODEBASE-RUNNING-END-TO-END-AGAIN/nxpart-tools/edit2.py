import sys,re
N="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/"
P="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts"
def load(p): return open(p,encoding='utf8').read()
def save(p,s): open(p,'w',encoding='utf8').write(s)
def rep(s,old,new,count=1):
    c=s.count(old)
    if c!=count: raise SystemExit(f"expected {count} got {c}: {old[:100]}")
    return s.replace(old,new)

# schema
p=N+"🧬️schema/🔣️.json"; s=load(p)
s=rep(s,'''                  "enum": [
                    "repository:nx-graph-pool",
                    "repository:graph-store"
                  ]''','''                  "enum": [
                    "repository:nx-store",
                    "repository:graph-store"
                  ]''')
s=rep(s,'''        "partition",
        "hostDirectory",
        "workspace",
        "producerLogSha256",
        "intent",
        "containers",
        "targets"
      ],''','''        "partition",
        "hostDirectory",
        "poolDirectory",
        "workspace",
        "producerLogSha256",
        "intent",
        "containers",
        "targets"
      ],''')
s=rep(s,'''        "partition": {
          "enum": [
            "graph",
            "products"
          ]
        },
        "hostDirectory": {
          "type": "string",
          "minLength": 1,
          "maxLength": 256
        },''','''        "partition": {
          "enum": [
            "nx",
            "products"
          ]
        },
        "hostDirectory": {
          "type": "string",
          "minLength": 1,
          "maxLength": 256
        },
        "poolDirectory": {
          "type": "string",
          "minLength": 1,
          "maxLength": 256
        },''')
s=rep(s,'''        "containers": {
          "type": "array",
          "maxItems": 2,''','''        "containers": {
          "type": "array",
          "maxItems": 1,''')
save(p,s)

# planner
p=N+"🟦️.ts"; s=load(p)
s=rep(s,'readonly partition:"graph"|"products";readonly hostDirectory:string;readonly workspace:string;','readonly partition:"nx"|"products";readonly hostDirectory:string;readonly poolDirectory:string;readonly workspace:string;')
s=rep(s,'owner:"repository:nx-graph-pool"|"repository:graph-store"','owner:"repository:nx-store"|"repository:graph-store"')
s=rep(s,'partition=selection.kind==="products-graph-fixture"?"products":"graph",host=','partition=selection.kind==="products-graph-fixture"?"products":"nx",host=')
old=s[s.index('const budget=repositoryCacheBudgetV1(workspace),graph=budget.partitions.graph,graphRoot='):s.index('return Object.freeze({version:1,purpose:protocol.purpose as NxGraphCurrentMaintenanceIntentV1["purpose"]')]
new='''const budget=repositoryCacheBudgetV1(workspace),graph=budget.partitions.graph,graphRoot=nativeImageRootV1(graph.directory),nx=budget.partitions.nx,poolDirectory=nx.directory,poolRoot=nativeImageRootV1(poolDirectory),globalLeases=repositoryLeaseDirectoryV1(repositoryRoot),fixtureLeases=repositoryLeaseDirectoryV1(workspace),image=(role:NxGraphCurrentMaintenanceImageV1["role"],directory:string,parentDirectory:string,leaseDirectory:string,filesystemBytes:number,maximumPhysicalBytes:number,ledger:NxGraphCurrentMaintenanceImageV1["ledger"],maximumLeaseDatabaseBytes?:number):NxGraphCurrentMaintenanceImageV1=>{for(const path of[directory,parentDirectory,leaseDirectory,nativeImageRootV1(directory)])requireMaterializedPathV1(path);return Object.freeze({role,directory,root:nativeImageRootV1(directory),parentDirectory,leaseDirectory,filesystemBytes,maximumPhysicalBytes,ledger,...(maximumLeaseDatabaseBytes===undefined?{}:{maximumLeaseDatabaseBytes})});},empty=Object.freeze({requirement:"empty"as const}),poolPolicy=Object.freeze({version:1 as const,root:poolRoot,leaseDirectory:fixtureLeases,capacityBytes:nx.capacityBytes,maximumWorkspaceBytes:nx.filesystemBytes,maximumArtifactBytes:nx.filesystemBytes}),poolImage=image("pool",poolDirectory,host.directory,fixtureLeases,poolPolicy.maximumWorkspaceBytes,poolPolicy.capacityBytes,Object.freeze({requirement:"current-domain-artifact",owner:"repository:nx-store",bytes:poolPolicy.maximumArtifactBytes,policy:poolPolicy})),namespaces=selection.kind==="graph-namespace"?[selection.namespace]:selection.namespaces,targets=namespaces.map(namespace=>{descriptor(namespace);requireMaterializedPathV1(namespace.workspaceRoot);return image("namespace",child(poolRoot,join(poolRoot,"nx/graphs",namespace.toolingIdentity.slice(0,20),createHash("sha256").update(JSON.stringify([namespace.workspaceRoot.replaceAll("\\\\","/"),namespace.graphIdentity,namespace.implementationIdentity])).digest("hex").slice(0,20),nativePolicy.directory)),poolDirectory,globalLeases,nativePolicy.filesystemBytes,nativePolicy.maximumPhysicalBytes,empty);});if(new Set(targets.map(target=>target.directory)).size!==targets.length)throw Error("Distinct current maintenance namespace directories required");const containers:NxGraphCurrentMaintenanceImageV1[]=[];if(selection.kind==="products-graph-fixture"){const graphPolicy=Object.freeze({version:1 as const,root:graphRoot,leaseDirectory:fixtureLeases,capacityBytes:graph.capacityBytes,maximumWorkspaceBytes:graph.filesystemBytes,maximumArtifactBytes:graph.filesystemBytes}),graphImage=image("graph",graph.directory,host.directory,fixtureLeases,graph.filesystemBytes,graph.capacityBytes,Object.freeze({requirement:"current-domain-artifact",owner:"repository:graph-store",bytes:graphPolicy.maximumArtifactBytes,policy:graphPolicy}));containers.push(poolImage);targets.push(poolImage,graphImage,image("leases",budget.partitions.leases.directory,host.directory,budget.controlDirectory,budget.partitions.leases.filesystemBytes,budget.partitions.leases.capacityBytes,empty,protocol.maximumLeaseDatabaseBytes));}'''
s=s.replace(old,new)
s=rep(s,'repositoryRoot,partition,hostDirectory:host.directory,workspace,producerLogSha256','repositoryRoot,partition,hostDirectory:host.directory,poolDirectory,workspace,producerLogSha256')
# consumers inside nx/graph
s=rep(s,'plan.containers.find(item=>item.role==="pool")!.root','nativeImageRootV1(plan.poolDirectory)',2)
save(p,s)

# publication consumer
s=load(P)
s=rep(s,'const pool=plan.containers.find(image=>image.role==="pool")!,resources=["nx:graph-pool:"+createHash("sha256").update(pool.root).digest("hex")','const resources=["nx:graph-pool:"+createHash("sha256").update(nativeImageRootV1(plan.poolDirectory)).digest("hex")',3)
save(P,s)
print("ok")
