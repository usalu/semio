# 📕️ Replays every norm case-table bundle through the shared independent Python norm implementation (the same handlers each `mutate-<standard>-1/🐍️.py` registers). usage: norm-check.py [--out file]
import glob,importlib.util,json,os,sys,types
ROOT="/Users/ueli/Documents/semio"
NORM=ROOT+"/✏️s/🔌️plugins/📕️norm"
host=types.ModuleType("semio_repo_test")
class Adapter:
    def __init__(self,name): self.name=name
    def oracle(self,*a,**k): return self
    def subject(self,*a,**k): return self
class Outcome:
    def __init__(self,payload=None,raw=None): self.payload=payload; self.raw=raw
host.Adapter=Adapter; host.Outcome=Outcome; host.digest=lambda data: __import__("hashlib").sha256(data if isinstance(data,bytes) else data.encode()).hexdigest()
sys.modules["semio_repo_test"]=host
sys.path.insert(0,NORM+"/🔮️oracles/🏃️execution")
rows=[]
for adapter_path in sorted(glob.glob(NORM+"/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/*mutate-*-1/🐍️.py")):
    subset_root=adapter_path.split("/🧪️tests/")[0]
    tables=glob.glob(subset_root+"/🧬️schema/🧬️mutations/*/🧪️tests/🔣️.cases.json")
    if not tables: continue
    spec=importlib.util.spec_from_file_location("adapter_"+str(len(rows)),adapter_path)
    mod=importlib.util.module_from_spec(spec)
    captured=[]
    spec.loader.exec_module(mod)
    mod.build_adapter=lambda subset,captured=captured: captured.append(subset)
    mod.adapter()
    subset=captured[0]
    engine=sys.modules["🐍️"] if "🐍️" in sys.modules else mod._vocabulary
    fixtures=subset_root+"/🧪️tests/🧫️fixtures/🧬️mutations"
    class Ctx:
        def input_bytes(self,uri):
            assert uri.startswith(subset.vector_root+"/"),uri
            return open(os.path.join(fixtures,uri[len(subset.vector_root)+1:]),"rb").read()
    for table in sorted(tables):
        leaf=os.path.basename(os.path.dirname(os.path.dirname(table)))
        slug=leaf.split("️",1)[-1] if "️" in leaf else leaf
        slug=''.join(ch for ch in leaf if ch.isascii() and (ch.isalnum() or ch in "-_"))
        for case in json.load(open(table,encoding="utf8"))["cases"]:
            if not os.path.isdir(os.path.join(fixtures,leaf,case,"🦠️mutation")): continue
            sub=type("S",(),{})()
            sub.__dict__.update(subset.__dict__)
            sub.vectors={slug:(leaf,case)}; sub.rows={slug:slug}
            verdict=[]
            for name,make in (("mutate",lambda: engine.mutate_handler(sub,slug)),("inverse",lambda: engine.inverse_handler(sub,slug))):
                try: make()(Ctx()); verdict.append(name+":ok")
                except Exception as error: verdict.append(name+":FAIL "+str(error).splitlines()[0][:220])
            rows.append((os.path.basename(subset_root.split("/🏅️standards")[0]),leaf,case,slug," | ".join(verdict)))
fail=[r for r in rows if "FAIL" in r[4]]
print("replayed",len(rows),"bundles;",len(rows)-len(fail),"pass both laws;",len(fail),"fail")
for r in fail: print("  FAIL",r[0],r[1],r[2],"|",r[4])
if "--out" in sys.argv: json.dump(rows,open(sys.argv[sys.argv.index("--out")+1],"w",encoding="utf8"),ensure_ascii=False,indent=0)
