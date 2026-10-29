import os,re,sys
R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy"
LC={"delete-row","create-row"};SG={"network-singleton","output-variable-row","root-link","slot-field","scalar-field"}
crates=[]
for dp,dn,fs in os.walk(R):
    dn[:]=[d for d in dn if d not in("node_modules","target")]
    if "Cargo.toml" in fs and dp.endswith("📦️packages/🦀️rust"):
        s=open(os.path.join(dp,"Cargo.toml"),encoding="utf-8").read()
        if re.search(r"^semio-s-artifact-energy-model-native-mutations = ",s,re.M):crates.append(dp)
crates=[c for c in crates if "native-lifecycle" not in c and "native-singletons" not in c]
roots={os.path.dirname(os.path.dirname(c)):c for c in crates}
res={}
for root,c in roots.items():
    need=set()
    for dp,dn,fs in os.walk(root):
        dn[:]=[d for d in dn if not (os.path.join(dp,d)!=root and os.path.join(dp,d) in roots)]
        for f in fs:
            if not f.endswith((".json",".rs")):continue
            t=open(os.path.join(dp,f),encoding="utf-8",errors="ignore").read()
            for cat in re.findall(r'"category":\s*"([a-z-]+)"',t):
                if cat in LC:need.add("lifecycle")
                if cat in SG:need.add("singletons")
            if "native_mutations_lifecycle" in t or "EnergyNativeRowBirth" in t:need.add("lifecycle")
            if "native_mutations_singletons" in t or "EnergyNativeActions" in t:need.add("singletons")
    res[c]=need
for c,n in sorted(res.items()):print(sorted(n),c.replace(R+"/","")[-110:-30])
if "--apply" in sys.argv:
    N="semio-s-artifact-energy-model-native-mutations"
    for c,n in res.items():
        p=c+"/Cargo.toml";s=open(p,encoding="utf-8").read()
        line=f"{N} = {{ workspace = true }}\n"
        add="".join(f"{N}-{x} = {{ workspace = true }}\n" for x in sorted(n) if f"{N}-{x} =" not in s)
        if add:
            assert line in s,p
            s=s.replace(line,line+add,1);open(p,"w",encoding="utf-8").write(s)
