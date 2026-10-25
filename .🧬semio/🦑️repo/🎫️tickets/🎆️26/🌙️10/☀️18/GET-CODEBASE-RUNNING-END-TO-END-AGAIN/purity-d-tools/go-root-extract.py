#!/usr/bin/env python3
"""go-root-extract.py <go package root dir> [apply]: moves the body of the package-root 🐹️.go into the registered member `🏛️core/🐹️.go`
(the canonical Go plan merges registered member leaves into their parent package) and leaves the header comment, the package
documentation and the package clause in the root. Prints the plan; with `apply` writes both files."""
import sys,re,os
d=sys.argv[1]; apply=len(sys.argv)>2 and sys.argv[2]=="apply"
root=os.path.join(d,"🐹️.go")
text=open(root,encoding="utf8").read()
m=re.search(r'^package\s+(\S+)\s*$',text,re.M)
if not m: sys.exit(f"no package clause: {root}")
head=text[:m.end()]
rest=text[m.end():]
lead=""
while True:
    n=re.match(r'(\s*\n)?(//[ ]?#endregion[^\n]*\n)',rest)
    if not n: break
    lead+=n.group(0); rest=rest[n.end():]
member=f"package {m.group(1)}\n{rest.lstrip(chr(10)) if not rest.startswith(chr(10)) else rest}"
body_lines=len([l for l in rest.splitlines() if l.strip()])
target=os.path.join(d,"🏛️core","🐹️.go")
if os.path.exists(target): sys.exit(f"exists: {target}")
newroot=head+"\n"+lead
print(f"{os.path.basename(os.path.dirname(os.path.dirname(os.path.dirname(d))))}: root {len(text.splitlines())} lines -> member {body_lines} lines; root keeps {len(newroot.splitlines())}")
if apply:
    os.makedirs(os.path.dirname(target),exist_ok=True)
    open(target,"w",encoding="utf8").write(member if member.endswith("\n") else member+"\n")
    open(root,"w",encoding="utf8").write(newroot if newroot.endswith("\n") else newroot+"\n")
