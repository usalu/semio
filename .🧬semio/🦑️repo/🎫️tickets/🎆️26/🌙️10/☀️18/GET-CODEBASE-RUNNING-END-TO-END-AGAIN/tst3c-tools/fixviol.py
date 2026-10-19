import os,re,sys,json
R="/Users/ioel"
R="/Users/ueli/Documents/semio"
skip={"node_modules",".git",".nx","target","🗑️generated",".🧬semio"}
bad=[]
for base in sys.argv[1:]:
    for dp,dn,fn in os.walk(R+"/"+base):
        dn[:]=[d for d in dn if d not in skip]
        for f in fn:
            p=os.path.join(dp,f); parts=p.split("/")
            if "🧫️fixtures" not in parts: continue
            i=parts.index("🧫️fixtures")
            ok="🧪️tests" in parts and parts.index("🧪️tests")<i and not any(x in ["🧬️schema","📦️packages","🎯️targets","🧩️implementations"] for x in parts[len(R.split("/"))+3:i])
            if ok: continue
            if f=="🟦️.ts" or p.endswith("/🟦️.ts"): continue
            bad.append(p[len(R)+1:])
for b in sorted(bad): print(b)
