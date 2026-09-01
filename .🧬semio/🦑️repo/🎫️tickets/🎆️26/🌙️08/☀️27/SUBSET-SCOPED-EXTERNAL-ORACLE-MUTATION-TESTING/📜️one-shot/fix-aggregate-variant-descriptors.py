import json,os,re,sys
APPLY = len(sys.argv)>1 and sys.argv[1]=="--apply"
BASE="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
def kebab(v): return re.sub(r'(?<!^)(?=[A-Z])','-',v).lower()
roots=[dp for dp,dn,fn in os.walk(BASE) if dp.endswith("🧬️mutations") and os.path.isfile(os.path.join(dp,"🦀️.rs"))]
tot=fixed=0; unmapped=[]
for root in roots:
    src=open(os.path.join(root,"🦀️.rs"),encoding="utf8").read()
    m=re.search(r"pub enum \w+ \{(.*?)\n\}", src, re.S)
    if not m: continue
    variants=re.findall(r"^\s{4}(\w+)\(", m.group(1), re.M)
    if not variants: continue
    keb2var={kebab(v):v for v in variants}
    for name in sorted(os.listdir(root)):
        p=os.path.join(root,name,"🔣️.json")
        if not os.path.isfile(p): continue
        d=json.load(open(p,encoding="utf8")); tot+=1
        var=keb2var.get(d["semanticKind"])
        if var is None:
            unmapped.append((root.split("🗿️artifacts/")[-1][:45],name,d["semanticKind"])); continue
        if d["aggregateVariant"]!=var:
            fixed+=1
            if APPLY:
                d["aggregateVariant"]=var
                open(p,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
            elif fixed<=5: print(f"  {name}: {d['aggregateVariant']} -> {var}")
print(f"{'APPLIED' if APPLY else 'DRY-RUN'}: {fixed} of {tot} fixed; unmapped={len(unmapped)}")
for u in unmapped[:10]: print("   unmapped:",u)
