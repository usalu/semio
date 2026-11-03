from lib import *
import collections
IDS = "japanese-maple lombardy-poplar london-plane-pollarded miscanthus-grass multi-stem-birch scots-pine small-leaved-lime weeping-willow".split()
for i in IDS:
    d,_=read_doc(i); a=d["asset"]
    for sc in SCALES:
        ks=collections.Counter()
        hat=collections.Counter()
        for s in a["representations"][sc]["geometry"]["elevation"]:
            for k in s: ks[k]+=1
            if "hatch" in s: hat[(s["id"],s["hatch"])]+=1
        print(i,sc,dict(ks), list(hat)[:3])
    print(" regions", [ (r.get("id")) for r in a["thermal"]["regions"]] if "regions" in a["thermal"] else list(a["thermal"].keys()))
