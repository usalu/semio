# 🏛️ Replays every architect case-table bundle through the architect case's independent Python implementation (🐍️.py), with the host's mutate and inverse handlers. usage: architect-check.py [--all]
# The host module `semio_repo_test` is stubbed: only Adapter/Context/Outcome are used by the handlers. Nothing is written.
import importlib.util,json,os,sys,types
ROOT="/Users/ueli/Documents/semio"
SUBSET=ROOT+"/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any"
FIXTURES=SUBSET+"/🧪️tests/🧫️fixtures/🧬️mutations"
PY=SUBSET+"/🧪️tests/🏛️mutate-program-1/🐍️.py"
host=types.ModuleType("semio_repo_test")
class Adapter:
    def __init__(self,name): self.name=name
    def oracle(self,*a,**k): return self
    def subject(self,*a,**k): return self
class Outcome:
    def __init__(self,payload=None,raw=None): self.payload=payload; self.raw=raw
class Context:
    def __init__(self,steps,base): self.scenario={"id":"replay","steps":steps}; self.base=base
    def input_bytes(self,uri):
        scheme,rest=uri.split("://",1)
        assert scheme=="shared",uri
        assert rest.startswith("🧬️mutations/"),uri
        return open(os.path.join(FIXTURES,rest[len("🧬️mutations/"):]),"rb").read()
host.Adapter=Adapter; host.Outcome=Outcome; host.Context=Context
sys.modules["semio_repo_test"]=host
spec=importlib.util.spec_from_file_location("architect_py",PY)
mod=importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
tag_to_kind={}
for k in mod.KINDS:
    verb,noun=mod.parts(k)
    tag_to_kind[(mod.TAGS[k],mod.camel(noun) if k!="%s-%s"%(verb,noun) else None)]=k
def kind_of(payload):
    tagged=payload.get("row")
    register=tagged.get("register") if isinstance(tagged,dict) else payload.get("register")
    return tag_to_kind.get((payload.get("mutation"),register)) or tag_to_kind.get((payload.get("mutation"),None))
def replay(vector):
    base=os.path.join(FIXTURES,vector)
    payload=json.load(open(base+"/🦠️mutation/🔣️.json",encoding="utf8"))
    kind=kind_of(payload)
    if kind is None: return "no-kind",payload.get("mutation")
    steps=[{"text":"the vector "+" ".join("shared://🧬️mutations/%s/%s"%(vector,leaf) for leaf in ["📸️snapshot/⬅️before/🔣️.json","🦠️mutation/🔣️.json","📸️snapshot/➡️after/🔣️.json","🎯️outcome/🔣️.json"]),"docString":json.dumps({"kind":kind,"vector":vector})}]
    ctx=Context(steps,base)
    verdict=[]
    for name,handler in (("mutate",mod.mutate_handler(kind)),("inverse",mod.inverse_handler(kind))):
        try: handler(ctx); verdict.append(name+":ok")
        except Exception as error: verdict.append(name+":FAIL "+str(error)[:200].replace("\n"," "))
    return kind," | ".join(verdict)
rows=[]
for tablefile in sorted(__import__("glob").glob(SUBSET+"/🧬️schema/🧬️mutations/**/🧪️tests/🔣️.cases.json",recursive=True)):
    owner=tablefile[len(SUBSET+"/🧬️schema/🧬️mutations/"):-len("/🧪️tests/🔣️.cases.json")]
    cases=json.load(open(tablefile,encoding="utf8"))["cases"]
    for case in cases:
        if "--all" not in sys.argv and not os.path.isdir(os.path.join(FIXTURES,owner,case)): continue
        kind,verdict=replay(owner+"/"+case)
        rows.append((owner,case,kind,verdict))
fail=[r for r in rows if "FAIL" in r[3] or r[2]=="no-kind"]
print("replayed",len(rows),"bundles;",len(rows)-len(fail),"pass both laws;",len(fail),"fail")
for r in fail: print("  FAIL",r[0],r[1],r[2],"|",r[3])
json.dump(rows,open(sys.argv[sys.argv.index("--out")+1],"w",encoding="utf8"),ensure_ascii=False,indent=0) if "--out" in sys.argv else None
