import os
base="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/"
def edit(path,pairs):
    p=base+path;t=open(p,encoding="utf8").read()
    for a,b in pairs:
        assert t.count(a)==1,a[:70]
        t=t.replace(a,b,1)
    tmp=p+".m7.tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,p)
edit("📈️metrics/🟦️.ts",[('''   if(counted.sinceMs===undefined&&nx.sinceMs===undefined)return null;
   const size''','''   const size'''),
('the hit and miss counters and the census size of the host content store and of Nx (`nx-counters`),','the hit and miss counters and the census size of the host content store and of Nx (`nx-counters`; a store that has counted nothing yet has zero hits and misses, not a gap: its counter files appear with the first event, which the next step then counts),')])
edit("👷️agent/🟦️.ts",[('''const ready=context.environment.SEMIO_FLEET_ORIGIN!==undefined&&context.environment.SEMIO_FLEET_ORIGIN===`${context.environment.SEMIO_FLEET_GROUP}-server`?undefined:superviseGraphChildV1(context,join(import.meta.dir,"..","🟦️.ts"),"ready");''','''const ready=superviseGraphChildV1(context,join(import.meta.dir,"..","🟦️.ts"),"ready");''')])
