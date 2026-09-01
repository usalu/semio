import io,os,re,glob,json,sys
def strip_head(t):
    ls=t.split("\n"); i=0
    while i<len(ls) and (ls[i].startswith("//!") or ls[i].strip()==""): i+=1
    return "\n".join(ls[i:])
def merge_imports(out):
    lines=out.split("\n")
    uses=[l.strip() for l in lines if re.match(r"^use [^;]+;$",l.strip())]
    rest=[l for l in lines if not re.match(r"^use [^;]+;$",l.strip())]
    flat=set()
    for u in uses:
        body=u[4:-1].strip(); m=re.match(r"^(.*)::\{(.*)\}$",body,re.S)
        if m:
            for sym in m.group(2).split(","):
                if sym.strip(): flat.add(f"{m.group(1)}::{sym.strip()}")
        else: flat.add(body)
    groups={}
    for e in flat:
        mod,_,sym=e.rpartition("::"); groups.setdefault(mod,set()).add(sym)
    emitted=[f"use {m}::{sorted(v)[0]};" if len(v)==1 else f"use {m}::{{{', '.join(sorted(v))}}};" for m,v in sorted(groups.items())]
    hi=0
    while hi<len(rest) and (rest[hi].startswith("//!") or rest[hi].strip()==""): hi+=1
    return "\n".join(rest[:hi]+emitted+[""]+rest[hi:])

def migrate_owner(root):
    leaves=[d for d in sorted(glob.glob(os.path.join(root,"*")))
            if os.path.isdir(d) and os.path.exists(os.path.join(d,"🦠️mutation","🦀️component.rs"))]
    if not leaves: return 0,0
    # every nested leaf must carry a descriptor: the aggregate derive reads all of them
    if any(not os.path.exists(os.path.join(d,"🔣️.json")) for d in leaves): return 0,len(leaves)
    kinds=set()
    for leaf in leaves:
        desc=json.load(open(os.path.join(leaf,"🔣️.json")))
        variant=desc["aggregateVariant"]; kinds.add(desc["semanticKind"].replace("-","_"))
        parts=[]
        for sub in ("🦠️mutation","🔺️diff","↩️inverse"):
            p=os.path.join(leaf,sub,"🦀️component.rs")
            if os.path.exists(p): parts.append(io.open(p,encoding='utf-8').read())
        merged=[parts[0].rstrip()]
        for t in parts[1:]:
            merged.append("\n"+re.sub(r"^use super::mutation::[^;]+;\n","",strip_head(t),flags=re.M).strip())
        out="\n".join(merged)+"\n"
        pat=re.compile(r"(#\[derive\(([^)]*)\)\]\s*\n(?:#\[[^\]]*\]\s*\n)*pub struct "+re.escape(variant)+r"\b)")
        m=pat.search(out)
        if m and "MutationLeaf" not in m.group(2):
            new=m.group(1).replace(f"#[derive({m.group(2)})]", f"#[derive({m.group(2)}, dsl::MutationLeaf)]\n#[mutation_leaf(contract = ::protocol)]",1)
            out=out[:m.start(1)]+new+out[m.end(1):]
        out=re.sub(r"\bsuper::(?:diff|inverse)::","",out)
        out=re.sub(r"\b([a-z_]+)::mutation::", r"\1::", out)
        io.open(os.path.join(leaf,"🦀️.rs"),'w',encoding='utf-8').write(merge_imports(out))
        for sub in ("🦠️mutation","🔺️diff","↩️inverse"):
            d=os.path.join(leaf,sub)
            if os.path.isdir(d):
                for f in glob.glob(os.path.join(d,"*")): os.remove(f)
                os.rmdir(d)
    for agg in (os.path.join(root,"🦀️.rs"), os.path.join(root,"🦀️component.rs")):
        if os.path.exists(agg):
            s=io.open(agg,encoding='utf-8').read()
            for nm in kinds: s=s.replace(f"{nm}::mutation::",f"{nm}::")
            io.open(agg,'w',encoding='utf-8').write(s)
    return len(leaves),0

done=skipped=0
for agg in sorted(glob.glob("✏️s/🔌️plugins/*/🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/🧬️schema/🧬️mutations")):
    d,s=migrate_owner(agg); done+=d; skipped+=s
print(f"migrated {done} leaf/leaves; {skipped} left in owners with an undescribed leaf")
