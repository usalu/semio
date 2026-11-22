import re,sys
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/"
p=D+"💾️store/🟦️.ts"; s=open(p).read()
old='/** 🗂️ Names the three keyspaces one store partitions: Bazel action results, Bazel content blobs and Nx task archives. */\nexport type RemoteCacheKindV1="ac"|"cas"|"nx";'
new='/** 🗂️ Names the three keyspaces one store partitions: Bazel action results, Bazel content blobs and Nx task archives; the only place that spells them, so every other module reads them from here. */\nexport const REMOTE_CACHE_KINDS_V1={ac:"ac",cas:"cas",nx:"nx"} as const;\n\n/** 🧺️ One keyspace name of {@link REMOTE_CACHE_KINDS_V1}. */\nexport type RemoteCacheKindV1=typeof REMOTE_CACHE_KINDS_V1[keyof typeof REMOTE_CACHE_KINDS_V1];'
assert s.count(old)==1; s=s.replace(old,new)
a='return kind==="nx"&&'; assert s.count(a)==1; s=s.replace(a,'return kind===REMOTE_CACHE_KINDS_V1.nx&&')
open(p,"w").write(s)
p=D+"🖥️server/🟦️.ts"; s=open(p).read()
i='import {RemoteCacheStoreV1,type RemoteCacheKindV1,'; assert s.count(i)==1
s=s.replace(i,'import {RemoteCacheStoreV1,REMOTE_CACHE_KINDS_V1,type RemoteCacheKindV1,')
t='kind:"ac"|"nx";key:string;slot:string'; assert s.count(t)==1; s=s.replace(t,'kind:Exclude<RemoteCacheKindV1,"cas">;key:string;slot:string')
n=s.count('"nx"'); s=s.replace('"nx"','NX_KIND')
anchor='/** 🔐️ Operator tokens'; assert s.count(anchor)==1
s=s.replace(anchor,'const NX_KIND=REMOTE_CACHE_KINDS_V1.nx;\n\n'+anchor)
open(p,"w").write(s); print("replaced",n)
