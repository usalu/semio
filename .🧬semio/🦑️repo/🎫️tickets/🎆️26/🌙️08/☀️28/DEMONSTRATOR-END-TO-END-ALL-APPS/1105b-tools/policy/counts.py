import json,os,sys,re,collections
C=sys.argv[1]
cl=json.load(open(C+"/🗑️generated/1105b/policy-closure.json"))
dirs={os.path.dirname(k):v for k,v in cl.items()}
rows=[]
for d,name in dirs.items():
    u=a=files=0
    for root,ds,fs in os.walk(d):
        ds[:]=[x for x in ds if x not in("target","node_modules") and not os.path.exists(os.path.join(root,x,"Cargo.toml"))]
        if "tests" in root.split(d,1)[1] or "🧪" in root.split(d,1)[1]:continue
        for f in fs:
            if not f.endswith(".rs") or "test" in f or "🧪" in f:continue
            try:t=open(os.path.join(root,f),encoding="utf-8").read()
            except:continue
            k=t.find("#[cfg(test)]")
            if k>=0:t=t[:k]
            files+=1
            u+=len(re.findall(r"\.unwrap\(\)|\.expect\(",t))
            a+=len(re.findall(r"#!?\[allow\(",t))
    if u or a:rows.append((name,d,files,u,a))
rows.sort(key=lambda r:-(r[3]+r[4]))
with open(C+"/1105b-policy-counts.tsv","w") as o:
    o.write("crate\tdir\tfiles\tunwrap_expect\tallow\n")
    for r in rows:o.write("\t".join(map(str,r))+"\n")
print(len(rows),sum(r[3] for r in rows),sum(r[4] for r in rows))
for r in rows[:25]:print(r[0],r[3],r[4])
