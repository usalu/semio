"""🏗️ Declares the workflow producer crates and (re)writes manifests and project files. usage: build_wf.py [--write]"""
import os,sys
os.environ["OSFAM"]="workflow"
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from cg import *
import wf_families as W
CR={}
DESC={"graph":"Workflow graph model: media contracts and ports, nodes and edges, declared inputs, validation, planning and parameters",
 "snapshot":"Workflow snapshot: the os.workflow persisted artifact with its bindings, snapshot validation and native snapshot codecs",
 "schema":"Workflow schema: the mutation vocabulary, operation application, the diff algebra and the native mutation codecs",
 "schema-laws":"Workflow laws: the schema, snapshot codec and mutation leaf suites over the whole workflow artifact"}
NODE={"graph":"🕸️graph","snapshot":"📸️snapshot","schema":"🧬️schema","schema-laws":"🧬️schema/⚖️laws"}
INTERNAL={"graph":[],"snapshot":["graph"],"schema":["graph","snapshot"],"schema-laws":["graph","snapshot","schema"]}
for cid in ("graph","snapshot","schema","schema-laws"):
    c=Crate(cid,NODE[cid],DESC[cid],"artifact","🦀️.rs")
    libs=[FL+p for p in W.LIB[cid] if os.path.exists(FL+p)]
    tests=[FL+p for p in W.TESTS[cid] if os.path.exists(FL+p)]
    ul=idents_used(libs) if not cid.endswith('-laws') else set();ut=idents_used(tests+(libs if cid.endswith('-laws') else [])) if (tests or cid.endswith('-laws')) else set()
    seen=set()
    for d in DEPS:
        if d["where"]=="dep" and d["ident"] in ul and (d["key"],d["cfg"]) not in seen:
            seen.add((d["key"],d["cfg"]));c.deps.append(d)
    depkeys={d["key"] for d in c.deps if d["cfg"] is None}
    for d in DEPS:
        if d["where"]=="dev" and (d["key"] in depkeys or d["ident"] in ut):c.devdeps.append(d)
    if tests:
        for d in DEPS:
            if d["where"]=="dep" and d["cfg"] is None and d["ident"] in ut and d["ident"] not in ul and not any(x["key"]==d["key"] for x in c.devdeps):c.devdeps.append(d)
    c.internal=[(NAMEPRE+i,FL+NODE[i]+"/"+PKG) for i in INTERNAL[cid]]
    CR[cid]=c
if __name__=="__main__":
    for cid,c in CR.items():
        print(cid,[d["key"] for d in c.deps],[d["key"] for d in c.devdeps])
        if "--write" in sys.argv:write_crate(c)
