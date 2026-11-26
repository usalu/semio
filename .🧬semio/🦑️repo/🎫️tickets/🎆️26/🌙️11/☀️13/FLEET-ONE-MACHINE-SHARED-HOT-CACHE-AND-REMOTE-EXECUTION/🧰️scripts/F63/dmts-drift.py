#!/usr/bin/env python3
"""Lists names exported by a .mjs that its sibling .d.mts does not declare (hand-written declarations drift)."""
import os,re,sys
root=sys.argv[1] if len(sys.argv)>1 else "."
skip={"node_modules","target",".git","🗑️generated","🤖️generated"}
bad=0
for d,ds,fs in os.walk(root):
    ds[:]=[x for x in ds if x not in skip]
    for f in fs:
        if not f.endswith(".mjs"):continue
        decl=os.path.join(d,f[:-4]+".d.mts")
        if not os.path.exists(decl):continue
        js=open(os.path.join(d,f),encoding="utf8").read()
        dt=open(decl,encoding="utf8").read()
        names=set()
        for m in re.finditer(r'export\s+(?:async\s+)?(?:function\*?|const|let|class)\s+([A-Za-z0-9_$]+)',js):names.add(m.group(1))
        for m in re.finditer(r'export\s*\{([^}]*)\}',js):
            for part in m.group(1).split(","):
                part=part.strip()
                if not part:continue
                names.add(part.split(" as ")[-1].strip())
        star="export * from" in dt or "export*from" in dt
        miss=sorted(n for n in names if not re.search(r'\b'+re.escape(n)+r'\b',dt))
        if miss and not (star and False):
            bad+=len(miss);print(os.path.join(d,decl.split("/")[-1]),miss)
print("missing",bad)
