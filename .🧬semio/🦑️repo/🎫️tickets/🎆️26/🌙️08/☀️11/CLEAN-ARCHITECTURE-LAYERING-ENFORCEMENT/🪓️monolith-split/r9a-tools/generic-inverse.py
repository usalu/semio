#!/usr/bin/env python3
"""Makes the nine one-line support/load inverse companions generic over the aggregate (the shape convertMutationLeaves writes for every other leaf). usage: generic-inverse.py [--apply]"""
import re,sys,os,glob
APPLY="--apply" in sys.argv
M="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
P="crate::standards::v1::subsets::any::schema::mutations::"
def snake(name): return re.sub(r"(?<!^)(?=[A-Z])","_",name).lower()
for slug in ["delete-load","set-load","delete-support","set-support","delete-load-case","set-load-case","create-load-case","create-load","create-support"]:
    f=glob.glob(M+"*"+slug+"/↩️inverse/🦀️.rs")
    assert len(f)==1,(slug,f)
    f=f[0]
    t=open(f).read()
    variants=sorted(set(re.findall(r"ModelMutation::(\w+)\(",t)))
    assert variants,(slug,t)
    wh=[]
    for v in variants:
        wh.append(f"{P}{snake(v)}::{v}")
    t2=re.sub(r"pub fn inverse\(([^)]*)\) -> Vec<ModelMutation> \{",lambda m:"pub fn inverse<Op>("+m.group(1)+") -> Vec<Op>\nwhere\n    Op: "+" + ".join(f"From<{w}>" for w in wh)+",\n{",t)
    assert t2!=t,slug
    t2=re.sub(r"ModelMutation::(\w+)\(","Op::from(",t2)
    t2=t2.replace("mutations::ModelMutation, ","").replace("{mutations::ModelMutation}","")
    t2=re.sub(r"super::super::(\w+)::(\w+) \{",lambda m:f"{P}{m.group(1)}::{m.group(2)} {{",t2)
    t2=t2.replace("schema::{snapshot::ModelSnapshot}","schema::snapshot::ModelSnapshot").replace("//! ↩️ Concrete semantic inverse restoring only authored parameters.","//! ↩️ Semantic inverse restoring only authored parameters, generic over the aggregate that wraps the payloads.")
    assert "ModelMutation" not in t2.replace("//!",""),slug
    print("==",slug); print(t2 if not APPLY else "")
    if APPLY: open(f,"w").write(t2)
