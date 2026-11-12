import subprocess,re,json,os,sys,glob
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
F="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
manifests={}
for root,dirs,files in os.walk("."):
    dirs[:]=[d for d in dirs if d not in("node_modules",".git","target")]
    if "package.json" in files:
        try: m=json.load(open(os.path.join(root,"package.json"),encoding="utf8"))
        except Exception: continue
        if str(m.get("name","")).startswith("@semio-tech/"): manifests[m["name"]]=(os.path.join(root,"package.json")[2:],m)
for i in range(60):
    out=subprocess.run(["repo",T+"/reg3-closure.ts"],capture_output=True,text=True,timeout=300)
    txt=re.sub(r"\x1b\[[0-9;]*m","",out.stdout+out.stderr)
    if "reg3-ok" in txt: print(txt.split("reg3-ok")[1][:20]); break
    m=re.search(r"^error: Registry implementation import is not schema-owned: (\S+) in (\S+)",txt,re.M)
    if not m: print(txt[-1500:]); sys.exit(1)
    spec=m.group(1); parts=spec.split("/"); name="/".join(parts[:2]); sub="./"+"/".join(parts[2:]) if len(parts)>2 else "."
    mp,man=manifests[name]; ex=man["exports"][sub]
    entry=os.path.normpath(os.path.join(os.path.dirname(mp),ex if isinstance(ex,str) else ex.get("default") or ex.get("import"))).replace("\\","/")
    s=open(F,encoding="utf8").read()
    anchor='"@semio-tech/schema-validator-ts": {\n            "manifestPath": "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/package.json",\n            "entryPath": "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts"\n          }'
    assert s.count(anchor)==1
    s=s.replace(anchor,anchor+',\n          "%s": {\n            "manifestPath": "%s",\n            "entryPath": "%s"\n          }'%(spec,mp,entry))
    open(F,"w",encoding="utf8").write(s); print("added",spec,entry)
