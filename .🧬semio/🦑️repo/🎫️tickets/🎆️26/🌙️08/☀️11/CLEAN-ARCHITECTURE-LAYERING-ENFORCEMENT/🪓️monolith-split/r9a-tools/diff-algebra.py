#!/usr/bin/env python3
"""Moves the patch algebra (Patch, Assigned, helpers, property/classification patches, KeyedDelta, Entry, merge_slot) out of the diff root into diff::algebra so the patch families and the root no longer form one cycle. usage: diff-algebra.py [--apply]"""
import re,sys,os,glob
APPLY="--apply" in sys.argv
D="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/"
root=D+"🦀️.rs"
t=open(root).read()
P="crate::standards::v1::subsets::any::schema::"
mac=re.search(r"/// ➕️ Later slot wins.*?\n}\n\n",t,re.S); assert mac
t=t.replace(mac.group(0),"")
a=t.index("//#region 🔖️PatchAlgebra"); b=t.index("//#endregion 🔖️KeyedDelta")+len("//#endregion 🔖️KeyedDelta\n")
body=t[a:b]
t=t[:a]+t[b:].lstrip("\n")
t=t.replace('#[path = "🩹️patches/🦀️.rs"]\npub mod patches;\n\npub use patches::*;\n','#[path = "🧮️algebra/🦀️.rs"]\npub mod algebra;\n#[path = "🩹️patches/🦀️.rs"]\npub mod patches;\n\npub use algebra::*;\npub use patches::*;\n')
assert "pub mod algebra;" in t
body=re.sub(r"\nfn non_empty<","\npub fn non_empty<",body)
heads=[l for l in t.split("\n")[:12] if l.startswith("use ")]
alg_head="\n".join(heads)
alg='''//! 🧮️ The patch algebra of the model diff: the sparse field patch trait, the assigned-null wrapper, the keyed delta of a collection and the later-slot-wins merge, shared by the root diff and every patch family.

'''+alg_head+'''

/// ➕️ Later slot wins: writes `later` over `slot` when it names a value.
pub fn merge_slot<T>(slot: &mut Option<T>, later: Option<T>) {
    if later.is_some() {
        *slot = later;
    }
}

'''+body
# macro callers: merge_slot!( -> merge_slot(
files=[root]+glob.glob(D+"🩹️patches/**/🦀️.rs",recursive=True)
out={}
for f in files:
    s=t if f==root else open(f).read()
    s2=s.replace("merge_slot!(","merge_slot(")
    if f!=root and f.endswith("🩹️patches/🦀️.rs"):
        assert s2.count("use super::*;")==1
        s2=s2.replace("use super::*;","\n".join(heads+[f"use {P}diff::algebra::*;"]).replace("\n","\n",1))
    if f!=root and "merge_slot(" in s2 and "algebra::*" not in s2 and not f.endswith("🩹️patches/🦀️.rs"): pass
    if s2!=(open(f).read() if f!=root else None): out[f]=s2
print("files",len(out))
if APPLY:
    os.makedirs(D+"🧮️algebra",exist_ok=True)
    open(D+"🧮️algebra/🦀️.rs","w").write(alg)
    for f,s in out.items(): open(f,"w").write(s)
else:
    print(alg[:1200]); print(t[:1500])
