"""📦️ Stage B3: cursor and reconcile manifests after the shadow/cursor decomposition."""
import re,sys
sys.path.insert(0,".")
import importlib.util
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
R=f"{REPO}/{RT}"
src=open("stageB.py",encoding="utf-8").read()
ns={}
exec(src[:src.index("mf={}")],ns)   # reuse helpers and constants only
manifest=ns["manifest"];UIC=ns["UIC"];JOB=ns["JOB"];JOBT=ns["JOBT"];sib=ns["sib"]
PRES=ns["PRES"];PRIM=ns["PRIM"];RECO=ns["RECO"]
CUR=("semio-framework-ui-runtime-reconcile-cursor","⏭️cursor")
mf={}
mf["⏭️cursor"]=manifest(CUR[0],"ui-runtime-reconcile-cursor","Headless UI runtime reconcile cursor: the retained surface shadow and the resumable keyed cursor that diffs a presented tree against it",
    JOB+UIC+sib(PRES[0],PRES[1])+sib(PRIM[0],PRIM[1]),
    JOBT+'serde_json = { version = "1.0.140" }\n'+sib(PRIM[0],PRIM[1],', features = ["testkit"]'),
    features=f'testkit = ["{PRIM[0]}/testkit"]\n')
mf["♻️reconcile"]=manifest(RECO[0],"ui-runtime-reconcile","Headless UI runtime reconcile authority: the thin reconciler owner, credit reservation, handback registry, resumable job, generation-qualified patches, terminal close and surface output pool",
    JOB+UIC+sib(PRES[0],PRES[1])+sib(PRIM[0],PRIM[1])+sib(CUR[0],CUR[1]),
    JOBT+'serde_json = { version = "1.0.140" }\n'+sib(PRIM[0],PRIM[1],', features = ["testkit"]')+sib(CUR[0],CUR[1],', features = ["testkit"]'),
    features=f'testkit = ["{PRIM[0]}/testkit", "{CUR[0]}/testkit"]\n')
for node,t in mf.items():
    p=f"{R}/{node}/📦️packages/🦀️rust/Cargo.toml"
    import os
    os.makedirs(os.path.dirname(p),exist_ok=True)
    open(p,"w",encoding="utf-8").write(t)
    id_=re.search(r'^id = "([^"]+)"',t,re.M).group(1)
    open(f"{R}/{node}/📦️packages/🦀️rust/📋️project.json","w",encoding="utf-8").write('{\n  "name": "@semio-tech/'+id_+'-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n')
print("ok")
