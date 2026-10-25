#!/usr/bin/env python3
"""usage: relocate_semio.py SUBSET SNAPSHOT_CRATE_DIR MUTATION_CRATE_DIR [--dry]
Moves the root-crate retirement/member-owner macros of one stdio semio subset into the crates that define the types (orphan rule)."""
import re,sys,os
S="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/"
sub,snap,mut=sys.argv[1:4]; dry="--dry" in sys.argv
root=S+sub+"/🦀️.rs"
snapfile=S+sub+"/🧬️schema/📸️snapshot/🦀️.rs"
sl=S+snap+"/📦️packages/🦀️rust/🦀️.rs"; ml=S+mut+"/📦️packages/🦀️rust/🦀️.rs"
r=open(root).read(); snaptext=open(snapfile).read()
keep=[];tosnap=[];tomut=[];dropped=0
for line in r.split("\n"):
    m=re.match(r"semio_framework_value::artifact_retire_(leaf|struct)!\(schema::snapshot::(\w+)",line)
    if m:
        t=m.group(2)
        dm=re.search(r"((?:#\[[^\n]*\]\n|///[^\n]*\n)*)pub (?:struct|enum) "+t+r"\b",snaptext)
        if dm and "RetireOwned" in dm.group(1): dropped+=1; continue
        tosnap.append(line); continue
    if line.startswith("semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!"): tosnap.append(line); continue
    if line.startswith("semio_s_artifact_stdio_semio_base::semio_member_owner!"): tomut.append(line); continue
    keep.append(line)
print(sub,"dropped (derive exists):",dropped,"to snapshot:",len(tosnap),"to mutations:",len(tomut))
if not dry:
    open(root,"w").write("\n".join(keep).rstrip("\n")+"\n")
    for p,ls in ((sl,tosnap),(ml,tomut)):
        t=open(p).read()
        for l in ls:
            if l not in t: t=t.rstrip("\n")+"\n"+l+"\n"
        open(p,"w").write(t)
