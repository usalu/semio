#!/usr/bin/env python3
"""Moves encode_inference_projection_json + projection_of from io::text::snapshot to the new io::text::projection module and updates every caller. usage: projection-move.py [--apply]"""
import subprocess,os,re,sys
APPLY="--apply" in sys.argv
A="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model"
S=A+"/🏅️standards/🔖️1/🪆️subsets/✳️any/"
snap=S+"🚪️io/📝️text/📸️snapshot/🦀️.rs"
t=open(snap).read()
start=t.index("/// 💡️ The canonical JSON of one inferred table")
body=t[start:]
assert body.count("pub fn encode_inference_projection_json")==1 and body.count("fn projection_of")==1
new_snap=t[:start].rstrip("\n")+"\n"
doc='''//! 🔭️ Inference projections of the text facet: the canonical JSON of one inferred table of a model snapshot, by slug, which the language-neutral inference and export oracles compare.

use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

'''
newfile=doc+body
if not newfile.endswith("\n"): newfile+="\n"
files=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","--",A],capture_output=True,text=True).stdout.split("\n")
files=[f for f in files if f.endswith(".rs") and os.path.isfile(f) and f!=snap]
fn="encode_inference_projection_json"
edits={}
grp=re.compile(r"(use\s+(?:crate|semio_s_artifact_bim_model)::standards::v1::subsets::any::io::text::)snapshot::\{([^}]*)\};")
single=re.compile(r"((?:crate|semio_s_artifact_bim_model)::standards::v1::subsets::any::io::text::)snapshot::"+fn)
for f in files:
    s=open(f).read()
    if fn not in s: continue
    o=s
    def g(m):
        names=[n.strip() for n in m.group(2).split(",") if n.strip()]
        if fn not in names: return m.group(0)
        rest=[n for n in names if n!=fn]
        head=m.group(1)
        # find indentation of the line start
        line_start=s.rfind("\n",0,m.start())+1
        indent=re.match(r"[ \t]*",s[line_start:]).group(0)
        proj=f"{head}projection::{fn};"
        if not rest: return proj
        r=rest[0] if len(rest)==1 else "{"+", ".join(rest)+"}"
        return f"{head}snapshot::{r};\n{indent}{proj}"
    s=grp.sub(g,s)
    s=single.sub(lambda m:m.group(1)+"projection::"+fn,s)
    if s!=o: edits[f]=s
    left=len(re.findall(r"snapshot::[^;\n]*"+fn,s))
    if left: print("LEFT",f,left)
print("callers",len(edits))
if APPLY:
    for f,s in edits.items(): open(f,"w").write(s)
    open(snap,"w").write(new_snap)
    d=S+"🚪️io/📝️text/🔭️projection"
    os.makedirs(d,exist_ok=True)
    open(d+"/🦀️.rs","w").write(newfile)
    tx=S+"🚪️io/📝️text/🦀️.rs"
    x=open(tx).read()
    x=x.rstrip("\n")+'\n\n#[path = "🔭️projection/🦀️.rs"]\npub mod projection;\n'
    open(tx,"w").write(x)
