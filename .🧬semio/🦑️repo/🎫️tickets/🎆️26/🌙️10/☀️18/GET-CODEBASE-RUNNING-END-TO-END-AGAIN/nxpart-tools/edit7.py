E="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/"
B=E+"📦️budget/"
def load(p): return open(p,encoding='utf8').read()
def save(p,s): open(p,'w',encoding='utf8').write(s)
def rep(s,old,new,count=1):
    c=s.count(old)
    if c!=count: raise SystemExit(f"expected {count} got {c}: {old[:110]}")
    return s.replace(old,new)
# policy
p=B+"🔣️policy.json";s=load(p)
s=rep(s,'''      "directory": ".cache/b/n",
      "capacityBytes": 939524096,
      "maximumLogicalBytes": 938475520''','''      "directory": ".cache/b/n",
      "capacityBytes": 1610612736,
      "maximumArtifactBytes": 536870912,
      "maximumLogicalBytes": 1609564160''')
save(p,s)
# schema
p=B+"🧬️schema/🔣️.json";s=load(p)
s=rep(s,'''        "capacityBytes": {
          "type": "integer",
          "minimum": 33554432,
          "maximum": 68719476736
        },
        "maximumLogicalBytes": {''','''        "capacityBytes": {
          "type": "integer",
          "minimum": 33554432,
          "maximum": 68719476736
        },
        "maximumArtifactBytes": {
          "type": "integer",
          "minimum": 33554432,
          "maximum": 68719476736
        },
        "maximumLogicalBytes": {''')
save(p,s)
# budget ts
p=B+"🟦️.ts";s=load(p)
s=rep(s,'export interface RepositoryCachePartitionV1 {readonly directory:string;readonly capacityBytes:number;readonly filesystemBytes:number;readonly maximumLogicalBytes:number}','export interface RepositoryCachePartitionV1 {readonly directory:string;readonly capacityBytes:number;readonly filesystemBytes:number;readonly maximumArtifactBytes:number;readonly maximumLogicalBytes:number}')
s=rep(s,'partitions:Record<RepositoryCachePartitionIdV1,{directory:string;capacityBytes:number;maximumLogicalBytes:number}>}','partitions:Record<RepositoryCachePartitionIdV1,{directory:string;capacityBytes:number;maximumArtifactBytes?:number;maximumLogicalBytes:number}>}')
s=rep(s,'partitions[id as RepositoryCachePartitionIdV1]=Object.freeze({...partition,filesystemBytes:partition.capacityBytes-1048576});','if(partition.maximumArtifactBytes!==undefined&&partition.maximumArtifactBytes>partition.capacityBytes-1048576)throw Error("Artifact charge must fit its partition filesystem");partitions[id as RepositoryCachePartitionIdV1]=Object.freeze({...partition,filesystemBytes:partition.capacityBytes-1048576,maximumArtifactBytes:partition.maximumArtifactBytes??partition.capacityBytes-1048576});')
save(p,s)
# store reservation
p=B+"📥️opening/🏪️store/🟦️.ts";s=load(p)
s=rep(s,'maximumArtifactBytes:id==="products"?persistentStateArtifactBudgetV1(declaration.filesystemBytes):declaration.filesystemBytes},receipt=','maximumArtifactBytes:id==="products"?persistentStateArtifactBudgetV1(declaration.filesystemBytes):declaration.maximumArtifactBytes},receipt=')
save(p,s)
# fixtures expected
p=B+"🧪️tests/🧫️fixtures/🔣️.json";s=load(p)
s=rep(s,'"expectedPartitionBytes": 67578626048','"expectedPartitionBytes": 68249714688')
s=rep(s,'"expectedHeadroomBytes": 1139802112','"expectedHeadroomBytes": 468713472')
save(p,s)
# nx graph: pool policy
p=E+"🔌️nx/📦️graph/🟦️.ts";s=load(p)
s=rep(s,'maximumWorkspaceBytes:repositoryCacheBudgetV1(workspace).partitions.nx.filesystemBytes,maximumArtifactBytes:repositoryCacheBudgetV1(workspace).partitions.nx.filesystemBytes}));','maximumWorkspaceBytes:repositoryCacheBudgetV1(workspace).partitions.nx.filesystemBytes,maximumArtifactBytes:repositoryCacheBudgetV1(workspace).partitions.nx.maximumArtifactBytes}));')
s=rep(s,'poolPolicy=Object.freeze({version:1 as const,root:poolRoot,leaseDirectory:fixtureLeases,capacityBytes:nx.capacityBytes,maximumWorkspaceBytes:nx.filesystemBytes,maximumArtifactBytes:nx.filesystemBytes})','poolPolicy=Object.freeze({version:1 as const,root:poolRoot,leaseDirectory:fixtureLeases,capacityBytes:nx.capacityBytes,maximumWorkspaceBytes:nx.filesystemBytes,maximumArtifactBytes:nx.maximumArtifactBytes})')
s=rep(s,'poolImage=image("pool",poolDirectory,host.directory,fixtureLeases,poolPolicy.maximumWorkspaceBytes,poolPolicy.capacityBytes,','poolImage=image("pool",poolDirectory,host.directory,fixtureLeases,nx.filesystemBytes,poolPolicy.capacityBytes,')
save(p,s)
print("ok")
