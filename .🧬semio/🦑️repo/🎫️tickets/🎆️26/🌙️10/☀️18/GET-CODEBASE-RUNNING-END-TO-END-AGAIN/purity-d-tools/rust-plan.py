#!/usr/bin/env python3
"""rust-plan.py <findings.tsv> <scope-prefix> > pairs.tsv : Rust module files/dirs inside a package that must move onto the owner.
A finding row is kind\\tpath\\tmessage; only rows below `<owner>/📦️packages/🦀️rust/` of owners that carry a package Cargo.toml are used."""
import sys,os,re,unicodedata
root="/Users/ueli/Documents/semio/"
rows=[l.rstrip("\n").split("\t") for l in open(sys.argv[1],encoding="utf8")]
scope=sys.argv[2]
owners={}
for k,s,m in rows:
    if not s.startswith(scope): continue
    mm=re.match(r'(.*?)/📦️packages/🦀️rust/(.*)$',s)
    if not mm: continue
    owners.setdefault(mm.group(1),set()).add(mm.group(2))
out=[];notes=[]
for o,rels in sorted(owners.items()):
    p=f"{o}/📦️packages/🦀️rust"
    if not os.path.exists(root+p+"/Cargo.toml"): notes.append(f"NO-MANIFEST {p}"); continue
    tops=sorted({r.split("/")[0] for r in rels})
    if os.path.exists(root+p+"/🦀️.rs") and "🦀️.rs" not in tops and any(os.path.isdir(root+f"{p}/{t}") for t in tops): tops.append("🦀️.rs")
    for t in tops:
        src=f"{p}/{t}"
        if t in ("🦀️.rs",) or os.path.isdir(root+src):
            if t=="🧪️tests" : notes.append(f"SKIP-TESTS {src}"); continue
            dst=f"{o}/{t}"
            if os.path.exists(root+dst):
                if os.path.isdir(root+dst) and os.path.isdir(root+src):
                    for dp,dn,fn in os.walk(root+src):
                        for f in fn:
                            rp=os.path.relpath(os.path.join(dp,f),root+src)
                            if os.path.exists(root+dst+"/"+rp): notes.append(f"COLLISION {src}/{rp} -> {dst}/{rp}")
                            else: out.append((f"{src}/{rp}",f"{dst}/{rp}"))
                else: notes.append(f"COLLISION {src} -> {dst}")
                continue
            out.append((src,dst))
        elif t.endswith(".rs") and t!="🦀️.rs":
            dst=f"{o}/🚪️entrypoint/🦀️.rs"
            if os.path.exists(root+dst): notes.append(f"COLLISION {src} -> {dst}")
            else: out.append((src,dst))
        else:
            notes.append(f"OTHER {src}")
for a,b in out: print(f"{a}\t{b}")
for n in notes: print(n,file=sys.stderr)
