"""📦 Neural engine carve: manifests and project files of the three new producers, parent manifest update (run once after gen.py)."""
import os,sys,toml,json
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from common import *
import deps
N=REPO+OS+"🔨️modules/🧠️neural/⚙️engine/"
SNAPMAN="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/snap/neural-Cargo.toml"
par=toml.load(SNAPMAN)
PP=N+"📦️packages/🦀️rust"
IDENT_TO_KEY={"semio_framework_job":"semio-framework-job","semio_framework_pack_json":"semio-framework-pack-json","semio_framework_value":"semio-framework-value","protocol":"semio-framework-replication","pack":"pack"}
CR={"M":("🧱️model","semio-framework-os-kernel-neural-engine-model","os-neural-engine-model","semio_framework_os_kernel_neural_engine_model","Neural engine model: the dictionary, schema, tree and operator vocabulary with their retirement, hashing and validation laws"),
    "R":("📔️registry","semio-framework-os-kernel-neural-engine-registry","os-neural-engine-registry","semio_framework_os_kernel_neural_engine_registry","Neural engine registry: the operator registry, schema components, shared registry ownership and its retirement"),
    "B":("⏱️evaluation","semio-framework-os-kernel-neural-engine-budgeted","os-neural-engine-budgeted","semio_framework_os_kernel_neural_engine_budgeted","Neural engine budgeted walk: the budgeted evaluation state, topology, input merge, tree copy and cache insert owners")}
LOWER={"M":[],"R":["M"],"B":["M","R"]}
def emit(key,spec,base):
    if isinstance(spec,str):return f'{key} = "{spec}"'
    if spec==True:return f'{key}.workspace = true'
    parts=[]
    for k,v in spec.items():
        if k=="path":v=os.path.relpath(os.path.normpath(os.path.join(PP,v)),base)
        if isinstance(v,bool):parts.append(f"{k} = {str(v).lower()}")
        elif isinstance(v,list):parts.append(f"{k} = ["+", ".join(f'"{x}"' for x in v)+"]")
        else:parts.append(f'{k} = "{v}"')
    return f"{key} = {{ {', '.join(parts)} }}"
def files_of(entry,pkg,cfg):
    return [r[0] for r in closure(entry,pkg,cfg) if r[0].endswith('.rs') and not r[0].startswith('/')]
def spec_for(key):
    d=par["dependencies"].get(key)
    if key=="semio-framework-job":return {"workspace":True}
    return d
for c,(folder,pkgname,idv,ident,desc) in CR.items():
    cp=N+folder+"/📦️packages/🦀️rust";os.makedirs(cp,exist_ok=True)
    entry=OS+"🔨️modules/🧠️neural/⚙️engine/"+folder+"/🦀️.rs";pkg=OS+"🔨️modules/🧠️neural/⚙️engine/"+folder+"/📦️packages/🦀️rust"
    lib=set(files_of(entry,pkg,""));alltests=set(files_of(entry,pkg,"test"))
    libtok=deps.used(lib);tok=deps.used(alltests)
    normal=[IDENT_TO_KEY[i] for i in IDENT_TO_KEY if i in libtok and i!="serde"]
    lines=["[package]",f'workspace = "{os.path.relpath(REPO.rstrip("/"),cp)}"',f'name = "{pkgname}"','version = "0.1.0"','edition = "2021"','rust-version.workspace = true',f'description = "{desc}"','',"[package.metadata.semio]",'role = "product"',f'id = "{idv}"','',"[package.metadata.semio.rust-source-boundary]","maxSourceBytes = 262144","maxSourceFiles = 32","maxActiveLines = 4000",'',"[lints]","workspace = true",'',"[lib]",'path = "../../🦀️.rs"','']
    if c=="M":lines+=["[features]","testkit = []",'']
    lines.append("[dependencies]")
    for k in normal:
        if k=="semio-framework-job":lines.append("semio-framework-job.workspace = true")
        else:lines.append(emit(k,par["dependencies"][k],cp))
    for l in LOWER[c]:
        lf=CR[l];lines.append(f'{lf[1]} = {{ path = "{os.path.relpath(N+lf[0]+"/📦️packages/🦀️rust",cp)}" }}')
    dev=[]
    dev.append('semio-framework-job = { workspace = true, features = ["testkit"] }') if "semio_framework_job" in tok else None
    for k,idn in [("semio-framework-trace","semio_framework_trace"),("semio-framework-async","semio_framework_async"),("serde","serde"),("serde_json","serde_json")]:
        if idn in tok:dev.append(emit(k,par["dev-dependencies"][k],cp))
    if c!="M":
        dev.append(f'{CR["M"][1]} = {{ path = "{os.path.relpath(N+CR["M"][0]+"/📦️packages/🦀️rust",cp)}", features = ["testkit"] }}')
    dev=[d for d in dev if d]
    # replication is a normal dependency when protocol is used, otherwise a dev dependency like the monolith
    lines+=["","[dev-dependencies]"]+dev
    open(cp+"/Cargo.toml","w",encoding="utf-8").write("\n".join(lines)+"\n")
    open(cp+"/📋️project.json","w",encoding="utf-8").write('{\n  "name": "@semio-tech/neural-engine-%s-rs",\n  "tags": [\n    "lang:rust",\n    "role:product",\n    "family:neural"\n  ]\n}\n'%idv.rsplit("-",1)[-1])
    print(c,"normal",normal,"| dev",len(dev))
# parent: add the three crates, testkit dev-dep
t=open(SNAPMAN,encoding="utf-8").read()
add="".join(f'{CR[c][1]} = {{ path = "{os.path.relpath(N+CR[c][0]+"/📦️packages/🦀️rust",PP)}" }}\n' for c in "MRB")
anchor='pack = { path = "../../../../../../../🔨️modules/🎒️pack/📦️packages/🦀️rust", package = "semio-framework-pack" }\n'
assert t.count(anchor)==1
t=t.replace(anchor,add)
for rm in ["semio-framework-job.workspace = true\n",
 "# 🎒️ `pack::json::to_json_string` — the evaluator's pending-extension branch needs a JSON string of\n# a `Dictionary` for its host-bound `input_json` payload (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-\n# AND-ARTIFACTS, 26/09/01, tenth-seam pass). No cycle: `pack` depends on `replication`, not the\n# reverse, and this crate already depends on `replication` directly.\n",
 ]:
    assert t.count(rm)==1,rm[:50]
    t=t.replace(rm,"")
anchor2='serde_json = "1.0.140"\n'
assert t.count(anchor2)==1
t=t.replace(anchor2,anchor2+f'{CR["M"][1]} = {{ path = "{os.path.relpath(N+CR["M"][0]+"/📦️packages/🦀️rust",PP)}", features = ["testkit"] }}\n')
open(PP+"/Cargo.toml","w",encoding="utf-8").write(t)
print("parent updated")
