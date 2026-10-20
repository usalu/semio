# 📕️ Replays every `Examples` row of each norm feature through the Python adapter exactly as the host registers it: mutate rows against `mutate-<id>`, inverse rows against `inverse-<id>`. usage: norm-feature-check.py
import glob,importlib.util,os,sys,types
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
def cells(line): return [cell.strip() for cell in line.strip().strip("|").split("|")]
def tables(path):
    lines=open(path,encoding="utf8").read().split("\n"); out=[]; i=0
    while i<len(lines):
        if lines[i].strip()=="Examples:":
            j=i+2; ids=[]
            while j<len(lines) and lines[j].strip().startswith("|"): ids.append(cells(lines[j])[0]); j+=1
            out.append(ids); i=j
        else: i+=1
    return out
ran=failed=0
for adapter_path in sorted(glob.glob(NORM+"/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/*mutate-*-1/🐍️.py")):
    case=os.path.dirname(adapter_path); subset_root=case.split("/🧪️tests/")[0]
    spec=importlib.util.spec_from_file_location("adapter_%d"%ran,adapter_path); mod=importlib.util.module_from_spec(spec); captured=[]
    spec.loader.exec_module(mod); mod.build_adapter=lambda subset,captured=captured: captured.append(subset); mod.adapter(); subset=captured[0]
    engine=mod._vocabulary
    mutate_ids,inverse_ids=tables(case+"/🥒️.feature")[:2]
    fixtures=subset_root+"/🧪️tests/🧫️fixtures/🧬️mutations"
    class Ctx:
        def input_bytes(self,uri): return open(os.path.join(fixtures,uri[len(subset.vector_root)+1:]),"rb").read()
    name=os.path.basename(subset_root.split("/🏅️standards")[0])
    bad=[]
    for row in mutate_ids:
        if row not in subset.vectors: bad.append("unregistered mutate-"+row); continue
        try: engine.mutate_handler(subset,row)(Ctx())
        except Exception as error: bad.append("mutate-%s: %s"%(row,str(error).splitlines()[0][:160]))
        ran+=1
    for row in inverse_ids:
        if row not in subset.kinds: bad.append("unregistered inverse-"+row); continue
        try: engine.inverse_handler(subset,row)(Ctx())
        except Exception as error: bad.append("inverse-%s: %s"%(row,str(error).splitlines()[0][:160]))
        ran+=1
    failed+=len(bad)
    print(name,len(mutate_ids),"mutate rows,",len(inverse_ids),"inverse rows,",len(bad),"failing"); [print("   ",b) for b in bad]
print("ran",ran,"failed",failed)
