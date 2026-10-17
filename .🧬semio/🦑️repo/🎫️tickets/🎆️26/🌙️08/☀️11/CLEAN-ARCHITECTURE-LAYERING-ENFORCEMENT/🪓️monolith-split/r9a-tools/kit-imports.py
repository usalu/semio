#!/usr/bin/env python3
"""Rewrites `use super::{..}` of the snapshot kits to the modules that define the names and drops names the file does not use. usage: kit-imports.py [--apply]"""
import re,os,json,sys
APPLY="--apply" in sys.argv
S="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/"
owner=json.load(open("/Users/ueli/.cache/r9a/snapshot-owner.json"))
dirs=["⚓️anchors","⚙️coordination-values","🤝️coordination","🏷️property-kit","🪛️property-kit-values","📄️sheets","🔧️sheets-values","🔥️conditions","🖼️views","🧷️views-values","🧲️anchors-values","📋️schedule-kit","🔩️schedule-kit-values","✅️validity","🦴️structure"]
selfmod={"⚓️anchors":"anchors","⚙️coordination-values":"coordination_values","🤝️coordination":"coordination","🏷️property-kit":"property_kit","🪛️property-kit-values":"property_kit_values","📄️sheets":"sheets","🔧️sheets-values":"sheets_values","🔥️conditions":"conditions","🖼️views":"views","🧷️views-values":"views_values","🧲️anchors-values":"anchors_values","📋️schedule-kit":"schedule_kit","🔩️schedule-kit-values":"schedule_kit_values","✅️validity":"validity","🦴️structure":"structure"}
for d in dirs:
    p=S+d+"/🦀️.rs"
    t=open(p).read()
    m=re.search(r"^use super::\{([^}]*)\};\n",t,re.M)
    if not m:
        print("no super use",d); continue
    names=[n.strip() for n in m.group(1).split(",") if n.strip()]
    body=t.replace(m.group(0),"")
    # identifiers used in the body excluding comments
    code="\n".join(l for l in body.split("\n") if not l.strip().startswith("//"))
    used=[n for n in names if re.search(r"\b"+re.escape(n)+r"\b",code)]
    groups={}
    for n in used:
        o=owner.get(n)
        if o is None or o==selfmod[d]: groups.setdefault(None,[]).append(n)
        else: groups.setdefault(o,[]).append(n)
    lines=[]
    for o,ns in groups.items():
        path="super" if o is None else f"super::{o}"
        lines.append(f"use {path}::{{{', '.join(ns)}}};" if len(ns)>1 else f"use {path}::{ns[0]};")
    new="\n".join(lines)+"\n" if lines else ""
    print(d,"dropped",[n for n in names if n not in used],"->",[ (o,len(ns)) for o,ns in groups.items()])
    if APPLY: open(p,"w").write(t.replace(m.group(0),new))
