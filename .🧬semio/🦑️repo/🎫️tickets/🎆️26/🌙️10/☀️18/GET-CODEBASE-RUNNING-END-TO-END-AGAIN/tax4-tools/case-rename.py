# 🔧 Renames implementation case directories of mutation leaves and re-points every `<leaf>/🧪️tests/<from>/` text reference (and `case_<slug>` module names) inside the owning plugin. usage: python3 case-rename.py <moves.json> [--apply]  moves: [{"subset":..., "leaf":"<dom>/<leaf>", "from":..., "to":...}]
import json,os,re,subprocess,sys
moves=json.load(open(sys.argv[1],encoding="utf8")); apply="--apply" in sys.argv
def slug(name): return re.sub(r"-","_",re.sub(r"^[^A-Za-z0-9]+","",name))
by_plugin={}
for m in moves:
    plugin="/".join(m["subset"].split("/")[:3]); by_plugin.setdefault(plugin,[]).append(m)
changed={}
for plugin,ms in by_plugin.items():
    files=[f for f in subprocess.run(["git","ls-files","-co","--exclude-standard","--",plugin],capture_output=True,text=True).stdout.split("\n") if f.endswith((".rs",".ts",".toml",".py")) and "node_modules" not in f and "/target/" not in f and os.path.isfile(f)]
    for f in files:
        t=open(f,encoding="utf8").read(); n=t
        if not any(m["from"] in t for m in ms): continue
        lines=n.split("\n"); out=[]; i=0
        while i<len(lines):
            ln=lines[i]; hit=None
            for m in ms:
                if f'{m["leaf"].split("/")[-1]}/🧪️tests/{m["from"]}/' in ln: hit=m; break
            if hit:
                ln=ln.replace(f'/🧪️tests/{hit["from"]}/',f'/🧪️tests/{hit["to"]}/'); out.append(ln)
                if i+1<len(lines) and re.match(r"\s*(pub )?mod case_"+re.escape(slug(hit["from"]))+";",lines[i+1]):
                    out.append(lines[i+1].replace("case_"+slug(hit["from"]),"case_"+slug(hit["to"]))); i+=1
            else: out.append(ln)
            i+=1
        n="\n".join(out)
        if n!=t: changed[f]=n
for m in moves:
    s=f'{m["subset"]}/🧬️schema/🧬️mutations/{m["leaf"]}/🧪️tests/{m["from"]}'; d=f'{m["subset"]}/🧬️schema/🧬️mutations/{m["leaf"]}/🧪️tests/{m["to"]}'
    assert os.path.isdir(s) and not os.path.exists(d),(s,d)
print("moves",len(moves),"files",len(changed))
for f in changed: print("  ",f.split("🪆️subsets/")[-1][:140])
if apply:
    for f,t in changed.items(): open(f,"w",encoding="utf8").write(t)
    for m in moves:
        os.rename(f'{m["subset"]}/🧬️schema/🧬️mutations/{m["leaf"]}/🧪️tests/{m["from"]}',f'{m["subset"]}/🧬️schema/🧬️mutations/{m["leaf"]}/🧪️tests/{m["to"]}')
    print("applied")
