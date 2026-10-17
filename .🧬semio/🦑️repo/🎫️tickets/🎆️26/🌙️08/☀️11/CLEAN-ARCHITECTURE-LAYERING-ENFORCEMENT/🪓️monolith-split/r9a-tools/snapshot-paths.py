#!/usr/bin/env python3
"""Inside the snapshot submodule files, rewrites absolute paths through the snapshot root (`...::snapshot::Name`) to the module that defines the name (`...::snapshot::<module>::Name`), so no submodule depends on the root that mounts it. usage: snapshot-paths.py [--apply]"""
import re,os,json,sys,glob
APPLY="--apply" in sys.argv
S="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/"
P="crate::standards::v1::subsets::any::schema::snapshot"
owner=json.load(open("/Users/ueli/.cache/r9a/snapshot-owner.json"))
files=[f for f in glob.glob(S+"*/🦀️.rs") ]
n=0
for f in sorted(files):
    t=open(f).read(); o=t
    def grp(m):
        names=[x.strip() for x in m.group(1).split(",") if x.strip()]
        by={}
        for x in names:
            if "::" in x or x in ("self",) : by.setdefault("__raw__",[]).append(x); continue
            base=x.split(" as ")[0].strip()
            by.setdefault(owner.get(base),[]).append(x)
        lines=[]
        for o2,ns in by.items():
            if o2=="__raw__" or o2 is None:
                lines.append(f"use {P}::{{{', '.join(ns)}}};")
            else:
                lines.append(f"use {P}::{o2}::{{{', '.join(ns)}}};" if len(ns)>1 else f"use {P}::{o2}::{ns[0]};")
        return "\n".join(lines)
    t=re.sub(r"use "+re.escape(P)+r"::\{([^}]*)\};",grp,t)
    def single(m):
        name=m.group(1); o2=owner.get(name)
        return f"use {P}::{o2}::{name};" if o2 else m.group(0)
    t=re.sub(r"use "+re.escape(P)+r"::(\w+);",single,t)
    # inline qualified: P::Name( or P::Name {
    t=re.sub(re.escape(P)+r"::([A-Z]\w*|[A-Z_]{2,})\b",lambda m:(f"{P}::{owner[m.group(1)]}::{m.group(1)}" if m.group(1) in owner else m.group(0)),t)
    if t!=o:
        n+=1
        print(f.replace(S,"")[:50])
        if APPLY: open(f,"w").write(t)
print(n)
