# 🔧 Repairs the gltf pairing: renames hashed implementation case directories to their fixture case, renames orphan fixture leaves to the schema leaf and re-points mounts, Cargo members and assert_case literals. usage: python3 gltf-apply.py <plan.json> [--apply]
import json,os,re,subprocess,sys,collections
plan=json.load(open(sys.argv[1],encoding="utf8")); apply="--apply" in sys.argv
A="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf"
G=f"{A}/🏅️standards/🔖️2.0/🪆️subsets/♾️any"; M=f"{G}/🧬️schema/🧬️mutations/"; F=f"{G}/🧪️tests/🧫️fixtures/🧬️mutations/"
def slug(name): return re.sub(r"-","_",re.sub(r"^[^A-Za-z0-9]+","",name))
casemv=plan["casemv"]; leafmv=plan["leafmv"]
fn=collections.defaultdict(set)
for c in casemv: fn[c["from"]].add(c["to"])
amb=[k for k,v in fn.items() if len(v)>1]
print("ambiguous from-names",amb)
files=[f for f in subprocess.run(["git","ls-files","-co","--exclude-standard","--",A],capture_output=True,text=True).stdout.split("\n") if f.endswith((".rs",".toml")) and "/target/" not in f and "node_modules" not in f]
changed={}
byfrom={c["from"]:c for c in casemv}
for f in files:
    try: t=open(f,encoding="utf8").read()
    except Exception: continue
    n=t
    lines=n.split("\n"); out=[]; i=0
    while i<len(lines):
        ln=lines[i]
        hit=None
        for fr in byfrom:
            if f"🧪️tests/{fr}/" in ln: hit=fr; break
        if hit and byfrom[hit]["to"] not in fn.get(hit,()) : hit=None
        if hit:
            to=byfrom[hit]["to"]
            ln=ln.replace(f"🧪️tests/{hit}/",f"🧪️tests/{to}/")
            out.append(ln)
            if i+1<len(lines) and re.match(r"\s*pub mod case_"+re.escape(slug(hit))+";",lines[i+1]):
                out.append(lines[i+1].replace("case_"+slug(hit),"case_"+slug(to))); i+=1
        else: out.append(ln)
        i+=1
    n="\n".join(out)
    if f.startswith(M) and "/🧪️tests/" in f[len(M):]:
        for d,a,b in leafmv:
            n=n.replace(f'assert_case("{d}/{a}/',f'assert_case("{d}/{b}/')
    if n!=t: changed[f]=n
print("case moves",len(casemv),"leaf moves",len(leafmv),"files rewritten",len(changed))
for d,a,b in leafmv: assert os.path.isdir(f"{F}{d}/{a}") and not os.path.exists(f"{F}{d}/{b}"),(d,a,b)
for c in casemv:
    s=f"{M}{c['leaf']}/🧪️tests/{c['from']}"; d=f"{M}{c['leaf']}/🧪️tests/{c['to']}"
    assert os.path.isdir(s) and not os.path.exists(d),(s,d)
if apply:
    for f,t in changed.items(): open(f,"w",encoding="utf8").write(t)
    for d,a,b in leafmv: os.rename(f"{F}{d}/{a}",f"{F}{d}/{b}")
    for c in casemv: os.rename(f"{M}{c['leaf']}/🧪️tests/{c['from']}",f"{M}{c['leaf']}/🧪️tests/{c['to']}")
    print("applied")
