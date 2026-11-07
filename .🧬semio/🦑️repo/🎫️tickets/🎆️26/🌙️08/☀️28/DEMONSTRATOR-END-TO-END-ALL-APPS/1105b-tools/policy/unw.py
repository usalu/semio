import csv,os,re,sys
C=sys.argv[1]
for r in csv.DictReader(open(C+"/1105b-policy-counts.tsv"),delimiter="\t"):
    if r["unwrap_expect"]=="0":continue
    d=r["dir"]
    for root,ds,fs in os.walk(d):
        ds[:]=[x for x in ds if x not in("target","node_modules") and not os.path.exists(os.path.join(root,x,"Cargo.toml"))]
        if "tests" in root.split(d,1)[1] or "🧪" in root.split(d,1)[1]:continue
        for f in fs:
            if not f.endswith(".rs") or "test" in f or "🧪" in f:continue
            p=os.path.join(root,f);t=open(p,encoding="utf-8").read();k=t.find("#[cfg(test)]")
            head=t if k<0 else t[:k]
            for i,l in enumerate(head.split("\n"),1):
                if re.search(r"\.unwrap\(\)|\.expect\(",l):print(r["crate"],p,i,l.strip()[:200])
