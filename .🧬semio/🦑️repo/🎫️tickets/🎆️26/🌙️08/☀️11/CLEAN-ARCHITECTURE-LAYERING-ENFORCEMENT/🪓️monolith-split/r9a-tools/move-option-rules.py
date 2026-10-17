#!/usr/bin/env python3
"""Moves the snapshot-only helper module mutations::option_rules to authored::option_rules (authored holds the pure functions that mutations and inferences share) and updates every caller. usage: move-option-rules.py [--apply]"""
import subprocess,os,re,sys,shutil
APPLY="--apply" in sys.argv
A="✏️s/🔌️plugins/🏙️bim"
S="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/"
old=S+"🧬️mutations/🧭️option-rules/🦀️.rs"
new_dir=S+"📏️authored/🎚️option-rules"
P="crate::standards::v1::subsets::any::schema::"
text=open(old).read()
fs=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","--",A],capture_output=True,text=True).stdout.split("\n")
edits={}
for f in fs:
    if not (f.endswith(".rs") and os.path.isfile(f)) or f==old: continue
    t=open(f).read(); o=t
    t=t.replace(P+"mutations::option_rules::",P+"authored::option_rules::")
    t=t.replace("use super::super::{elements, option_rules};","use super::super::elements;\nuse "+P+"authored::option_rules;")
    t=t.replace("use super::super::option_rules;","use "+P+"authored::option_rules;")
    if t!=o: edits[f]=t
agg=S+"🧬️mutations/🦀️.rs"
a=edits.get(agg) or open(agg).read()
mount='#[path = "🧭️option-rules/🦀️.rs"]\npub mod option_rules;\n'
assert a.count(mount)==1
edits[agg]=a.replace(mount,"")
au=S+"📏️authored/🦀️.rs"
b=open(au).read()
anchor='#[path = "🧭️plan/🦀️.rs"]\npub mod plan;\n'
assert b.count(anchor)==1
edits[au]=b.replace(anchor,'#[path = "🎚️option-rules/🦀️.rs"]\npub mod option_rules;\n'+anchor)
print("edited",len(edits))
for f in edits: print("  ",f.split("/🪆️subsets/✳️any/")[-1][:100])
left=[f for f in fs if f.endswith(".rs") and os.path.isfile(f) and re.search(r"mutations::option_rules|super::super::.*option_rules",open(f).read()) and f!=old]
print("left",left)
if APPLY:
    for f,t in edits.items(): open(f,"w").write(t)
    os.makedirs(new_dir,exist_ok=True)
    open(new_dir+"/🦀️.rs","w").write(text.replace("//! 🧭️ Authored-only validity of options, worksets and their memberships.","//! 🎚️ Authored-only validity of options, worksets and their memberships."))
    os.remove(old); os.rmdir(os.path.dirname(old))
