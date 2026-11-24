import json,sys,collections,statistics
# usage: python3 -I carve-C1-telemetry.py <units.jsonl> [<cargo-metadata.json>]   (read-only)
rows=[json.loads(l) for l in open(sys.argv[1]) if l.strip()]
fin=[r for r in rows if r["key"].endswith("|cdylib") and not r["key"].split("|")[1].startswith("aarch64") and "wasm32" in r["key"]]
by=collections.defaultdict(list)
for r in fin: by[r["key"].split("@")[1].split("|")[0]+"|"+r["key"].split("|")[1]].append(r)
print("crate|target\tn\tpeakMiB min/med/max\twallS min/max\tcpu/wall\toutMB")
for k,v in sorted(by.items(),key=lambda kv:max(r["peakRssBytes"] for r in kv[1])):
    pk=[r["peakRssBytes"]/2**20 for r in v]; w=[r["wallMs"]/1000 for r in v]; c=[r["cpuMs"]/r["wallMs"] for r in v]
    print(f"{k}\t{len(v)}\t{min(pk):.0f}/{statistics.median(pk):.0f}/{max(pk):.0f}\t{min(w):.0f}/{max(w):.0f}\t{statistics.mean(c):.2f}\t{v[-1]['outputBytes']/1e6:.0f}")
xs=[(r["outputBytes"]/1e6,r["peakRssBytes"]/2**20) for r in fin]
n=len(xs); mx=sum(x for x,_ in xs)/n; my=sum(y for _,y in xs)/n
b=sum((x-mx)*(y-my) for x,y in xs)/sum((x-mx)**2 for x,_ in xs); print(f"fit peakMiB = {my-b*mx:.0f} + {b:.1f} * outputMB over {n} rows")
if len(sys.argv)>2:
    m=json.load(open(sys.argv[2])); pk={p["id"]:p for p in m["packages"]}; nd={n["id"]:n for n in m["resolve"]["nodes"]}
    def cl(r):
        s=set(); st=[r]
        while st:
            x=st.pop()
            if x in s: continue
            s.add(x); st+=[d["pkg"] for d in nd[x]["deps"] if any(k["kind"] in (None,"build") for k in d["dep_kinds"])]
        return s
    print("closure (all crates / workspace crates):")
    for i,p in sorted(pk.items(),key=lambda kv:kv[1]["name"]):
        if i in nd and p["name"] in ("semio-hub-cad-component","semio-hub-puzzle-component","semio-hub-stdio-media-component","semio-hub-flow-component"):
            c=cl(i); print(p["name"],len(c),sum(1 for x in c if pk[x]["source"] is None))
