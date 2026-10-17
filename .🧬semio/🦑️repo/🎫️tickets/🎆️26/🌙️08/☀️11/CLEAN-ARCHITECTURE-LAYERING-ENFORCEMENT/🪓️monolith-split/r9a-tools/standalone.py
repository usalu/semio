#!/usr/bin/env python3
"""Moves the twelve cfg(test) compute_* helpers out of the inference modules (they called the model-graph driver, an upward edge) into the driver's own module model_graph::standalone as plain pub fns, and updates every caller. usage: standalone.py [--apply]"""
import subprocess,os,re,sys,collections
APPLY="--apply" in sys.argv
A="✏️s/🔌️plugins/🏙️bim"
S="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/"
P="crate::standards::v1::subsets::any::schema::inferences::"
ST=P+"model_graph::standalone::"
fs=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","--",A],capture_output=True,text=True).stdout.split("\n")
fs=[f for f in fs if f.endswith(".rs") and os.path.isfile(f)]
pat=re.compile(r"((?:[ \t]*///[^\n]*\n)*)[ \t]*#\[cfg\(test\)\]\n[ \t]*pub fn (compute_\w+)\(([^)]*)\) -> ([^{]+)\{\n\s*std::mem::take\(&mut super::super::model_graph::infer_selected::<\{ super::super::model_graph::kinds::(\w+) \}>\((\w+)\)\.(\w+)\)\n\}\n")
defs=[]  # (name, args, ret, kind, field, doc, modname, file)
edits={}
for f in fs:
    if not f.startswith(S) : continue
    t=open(f).read()
    for m in pat.finditer(t):
        mod=f[len(S):].split("/")[0]
        defs.append((m.group(2),m.group(3),m.group(4).strip(),m.group(5),m.group(7),m.group(1),mod,f))
    if pat.search(t):
        t2=pat.sub("",t)
        t2=re.sub(r"//#region 🔖️Projection\n\s*//#endregion 🔖️Projection\n","",t2)
        t2=re.sub(r"\n{3,}","\n\n",t2)
        edits[f]=t2
names=[d[0] for d in defs]
print("defs",len(defs),names)
typeimp={}
# types per fn from the original imports of the defining file: resolve by known type names
TYPES={"Diagnostic":"diagnostics::Diagnostic","SpaceRoom":"spaces::SpaceRoom","StoreyLevel":"storey_levels::StoreyLevel","ViewLinework":"view_linework::ViewLinework","PlanLinework":"plan_linework::PlanLinework","RampRun":"ramp_runs::RampRun","ElementSolid":"element_solids::ElementSolid","ModelQuantities":"quantities::ModelQuantities","WallLayout":"wall_layout::WallLayout","StairRun":"stair_runs::StairRun","CurtainLayout":"curtain_layout::CurtainLayout","OpeningFrame":"opening_frames::OpeningFrame","ModelInference":"ModelInference"}
out=["//! 🧰️ Standalone projections of one inference kind: each reads the model graph through its driver and takes the one table, for tests, oracles and one-shot callers that want a single kind without a session.","","use super::{infer_selected, kinds};","use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;","use crate::standards::v1::subsets::any::schema::inferences::*;","use std::collections::BTreeMap;",""]
out=["//! 🧰️ Standalone projections of one inference kind: each reads the model graph through its driver and takes the one table, for tests, oracles and one-shot callers that want a single kind without a session.","","use super::{infer_selected, kinds};","use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;","use std::collections::BTreeMap;",""]
used=set()
body=[]
for name,args,ret,kind,field,doc,mod,f in sorted(defs):
    r=ret
    a=args
    d=doc.replace("    ","")
    body.append(f"{d}pub fn {name}({a}) -> {r} {{\n    std::mem::take(&mut infer_selected::<{{ kinds::{kind} }}>(snapshot).{field})\n}}\n")
imports="use crate::standards::v1::subsets::any::schema::inferences::{"+", ".join(sorted(path for ty,path in TYPES.items()))+"};"
out.insert(5,imports)
new_file="\n".join(out)+"\n"+"\n".join(body)
# callers
modnames=set(d[6] for d in defs)
import itertools
rx_name=re.compile(r"\b("+"|".join(names)+r")\b")
for f in fs:
    t=open(f).read() if f not in edits else edits[f]
    if not rx_name.search(t): continue
    o=t
    # qualified: mod::compute_x(...) and crate-qualified paths
    t=re.sub(r"crate::standards::v1::subsets::any::schema::inferences::\w+::("+"|".join(names)+r")\b",lambda m:ST+m.group(1),t)
    t=re.sub(r"(?<![\w:])(?:"+"|".join(modnames)+r")::("+"|".join(names)+r")\b",lambda m:ST+m.group(1),t)
    # use with braces from inferences::mod::{a, compute_x, b}
    def brace(m):
        head=m.group(1); items=[i.strip() for i in m.group(2).split(",") if i.strip()]
        mine=[i for i in items if i in names]; rest=[i for i in items if i not in names]
        if not mine: return m.group(0)
        lines=[f"use {ST}{i};" for i in mine]
        if rest: lines.insert(0,f"use {head}{{{', '.join(rest)}}};" if len(rest)>1 else f"use {head}{rest[0]};")
        return "\n".join(lines)
    t=re.sub(r"use ((?:crate::standards::v1::subsets::any::schema::inferences|super(?:::super)*)::\w+::)\{([^}]*)\};",brace,t)
    t=re.sub(r"use (?:super::)+("+"|".join(names)+r");",lambda m:f"use {ST}{m.group(1)};",t)
    t=re.sub(r"use (?:super::)+\{([^}]*)\};",lambda m:(lambda items:"\n".join([f"use {ST}{i};" for i in items if i in names]+([f"use super::{{{', '.join(i for i in items if i not in names)}}};"] if any(i not in names for i in items) and any(i in names for i in items) else []))if any(i in names for i in items) else m.group(0))([x.strip() for x in m.group(1).split(",") if x.strip()]),t)
    # remaining bare uses without an import
    bare=set()
    for m in rx_name.finditer(t):
        n=m.group(1)
        line_start=t.rfind("\n",0,m.start())+1; line=t[line_start:t.find("\n",m.end())]
        if "::" in t[max(0,m.start()-2):m.start()]: continue
        if re.match(r"\s*(?:pub\s+)?use\b",line): continue
        bare.add(n)
    need=[n for n in sorted(bare) if f"{ST}{n};" not in t and not re.search(r"use [^;]*\b"+n+r"\b[^;]*;",t)]
    if need:
        # insert after the last top-level `use` line, or after leading inner docs
        ms=list(re.finditer(r"^use [^\n]*;\n",t,re.M))
        ins="".join(f"use {ST}{n};\n" for n in need)
        if ms:
            e=ms[-1].end(); t=t[:e]+ins+t[e:]
        else:
            m0=re.match(r"(?:\s*//![^\n]*\n)*\n?",t); e=m0.end(); t=t[:e]+ins+"\n"+t[e:]
    if t!=o: edits[f]=t
print("files edited",len(edits))
if APPLY:
    for f,t in edits.items(): open(f,"w").write(t)
    d=S+"🕸️model-graph/🧰️standalone"
    os.makedirs(d,exist_ok=True); open(d+"/🦀️.rs","w").write(new_file)
    mg=S+"🕸️model-graph/🦀️.rs"
    mgt=open(mg).read()
    anchor=re.search(r'#\[path = "[^"]*"\]\n(?:pub )?mod \w+;\n',mgt)
    print("first mount at",anchor.start() if anchor else None)
else:
    import difflib
    os.makedirs("/Users/ueli/.cache/r9a/standalone-preview",exist_ok=True)
    for i,(f,t2) in enumerate(sorted(edits.items())):
        d="".join(difflib.unified_diff(open(f).read().splitlines(True),t2.splitlines(True),"a","b",n=0))
        open(f"/Users/ueli/.cache/r9a/standalone-preview/{i:02d}.diff","w").write(f+"\n"+d)
    open("/Users/ueli/.cache/r9a/standalone-preview/new.rs","w").write(new_file)
