import os,sys
R="/Users/ueli/Documents/semio"
skip={"node_modules",".git",".nx","target","🗑️generated",".🧬semio"}
FORB=["🧬️schema","📦️packages","🎯️targets","🧩️implementations"]
for base in sys.argv[1:]:
    for dp,dn,fn in os.walk(R+"/"+base):
        dn[:]=[d for d in dn if d not in skip]
        for f in fn:
            p=os.path.join(dp,f); parts=p[len(R)+1:].split("/")
            if "🧫️fixtures" not in parts: continue
            i=parts.index("🧫️fixtures")
            ok="🧪️tests" in parts and parts.index("🧪️tests")<i and not any(x in FORB for x in parts[:i])
            if ok or f=="🟦️.ts": continue
            print("/".join(parts))
