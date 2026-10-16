"""📦️ Stage B: crate roots, manifests and project files for exchange, presentation, primitives, reconcile and laws."""
import os,re,sys
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
R=f"{REPO}/{RT}"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt/orig"
def w(path,text):
    os.makedirs(os.path.dirname(path),exist_ok=True)
    open(path,"w",encoding="utf-8").write(text)
BOUNDARY='''[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000
'''
def manifest(name,id_,desc,deps,devdeps="",features="",extra=""):
    t=f'''[package]
workspace = "../../../../../.."
name = "{name}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "{desc}"

[package.metadata.semio]
role = "framework"
id = "{id_}"

{BOUNDARY}
[lints]
workspace = true

[lib]
path = "../../🦀️.rs"
'''
    if features: t+="\n[features]\n"+features
    t+="\n[dependencies]\n"+deps
    if devdeps: t+="\n[dev-dependencies]\n"+devdeps
    t+=extra
    return t
UIC='ui_contract = { path = "../../../../🧬️contract/📦️packages/🦀️rust", package = "semio-framework-ui-contract" }\n'
SCENE='ui_scene = { path = "../../../../🎬️scene/📦️packages/🦀️rust", package = "semio-framework-ui-scene" }\n'
VALUE='semio-framework-value = { path = "../../../../../🌱️value/📦️packages/🦀️rust" }\n'
JOB='semio-framework-job = { path = "../../../../../🧵️job/📦️packages/🦀️rust" }\n'
JOBT='semio-framework-job = { path = "../../../../../🧵️job/📦️packages/🦀️rust", features = ["testkit"] }\n'
def sib(n,node,extra=""): return f'{n} = {{ path = "../../../{node}/📦️packages/🦀️rust"{extra} }}\n'
EXCH=("semio-framework-ui-runtime-exchange","📡️exchange")
PRES=("semio-framework-ui-runtime-presentation","🖼️presentation")
PRIM=("semio-framework-ui-runtime-reconcile-primitives","🧱️primitives")
RECO=("semio-framework-ui-runtime-reconcile","♻️reconcile")
LAWS=("semio-framework-ui-runtime-laws","⚖️laws")
TRACE='semio-framework-trace = { workspace = true }\n'
SERDE_NOTE='''# 🧪️ The fixture-oracle tests diff published patches and snapshots against `serde_json` directly — ticket
# 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS. No non-test symbol names `serde`.
'''
mf={}
mf["📡️exchange"]=manifest(EXCH[0],"ui-runtime-exchange","Headless UI runtime exchange: the bounded command gateway, the coalescing projection inbox and the presence hub",UIC)
mf["🖼️presentation"]=manifest(PRES[0],"ui-runtime-presentation","Headless UI runtime presentation: typed entities with mutation leasing, actual-read dependency tracking, intent routing and the component tree a presenter builds",
    VALUE+SCENE+JOB+UIC+sib(EXCH[0],EXCH[1]),
    JOBT+TRACE+SERDE_NOTE+'serde = { version = "1.0.219", features = ["derive"] }\nserde_json = { version = "1.0.140" }\n',
    extra='\n[[test]]\nname = "native_resource_custody"\npath = "../../../🎭️present/📦️resources/🧪️tests/🦀️.rs"\n')
mf["🧱️primitives"]=manifest(PRIM[0],"ui-runtime-reconcile-primitives","Headless UI runtime reconcile primitives: bounded containers, admission limits and censuses, record-diff cursors and tree retirement",
    JOB+UIC+sib(PRES[0],PRES[1]),features='testkit = []\n')
mf["♻️reconcile"]=manifest(RECO[0],"ui-runtime-reconcile","Headless UI runtime reconcile: the keyed reconciler, its resumable cursor, retained authority, handback registry and output pool",
    JOB+UIC+sib(PRES[0],PRES[1])+sib(PRIM[0],PRIM[1]),
    JOBT+'serde_json = { version = "1.0.140" }\n',
    features=f'testkit = ["{PRIM[0]}/testkit"]\n')
mf["⚖️laws"]=manifest(LAWS[0],"ui-runtime-laws","Headless UI runtime laws: the reconcile, ownership, document, output-pool and tree-retirement suites over the carved runtime crates",
    "",
    JOBT+TRACE+UIC+SCENE+VALUE+sib(EXCH[0],EXCH[1])+sib(PRES[0],PRES[1])+sib(PRIM[0],PRIM[1],', features = ["testkit"]')+sib(RECO[0],RECO[1],', features = ["testkit"]')+'serde_json = { version = "1.0.140" }\n')
# path corrections: dev/dep paths of value/job/ui_scene/ui_contract are written for depth 5 (job,value) and depth 4 (contract, scene)
for node,t in mf.items():
    w(f"{R}/{node}/📦️packages/🦀️rust/Cargo.toml",t)
    nm=re.search(r'^name = "([^"]+)"',t,re.M).group(1)
    id_=re.search(r'^id = "([^"]+)"',t,re.M).group(1)
    proj='{\n  "name": "@semio-tech/'+id_+'-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n'
    w(f"{R}/{node}/📦️packages/🦀️rust/📋️project.json",proj)
print("written",list(mf))
