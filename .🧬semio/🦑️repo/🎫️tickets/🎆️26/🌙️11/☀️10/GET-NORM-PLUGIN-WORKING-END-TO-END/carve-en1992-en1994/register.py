"""📦️ Registers a new crate in a norm family workspace: member, workspace dependency, package script list, consumer dependency lines."""
import json,re,sys
from pathlib import Path
ART=Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
FAM={"en1992":"🏛️en1992","en1994":"🧩️en1994"}
SUB="🏅️standards/🔖️1/🪆️subsets/✳️any/"
def register(family,name,reldir):
    reldir=reldir.removeprefix(SUB)
    root=ART/FAM[family]
    cargo=root/"Cargo.toml"; s=cargo.read_text(encoding="utf8")
    member=f'    "{SUB}{reldir}/📦️packages/🦀️rust",\n'
    if member not in s:
        i=s.index("]\n",s.index("members = ["))
        s=s[:i]+member+s[i:]
    dep=f'[workspace.dependencies.{name}]\npath = "{SUB}{reldir}/📦️packages/🦀️rust"\n\n'
    if f"[workspace.dependencies.{name}]" not in s:
        j=s.index("[workspace.dependencies.")
        s=s[:j]+dep+s[j:]
    cargo.write_text(s,encoding="utf8")
    sc=root/"📦️packages/🦀️rust/📜️script.ts"; t=sc.read_text(encoding="utf8")
    if f'"{name}"' not in t:
        k=t.index("packages:[")+len("packages:[")
        t=t[:k]+f'"{name}",'+t[k:]
        sc.write_text(t,encoding="utf8")
if __name__=="__main__":
    register(*sys.argv[1:4])
