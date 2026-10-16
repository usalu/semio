"""📦 Declared-versus-literal dependency check per crate (lib closure and test closure). Read-only."""
import os,re,sys,json,subprocess
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import ffu
W=sys.argv[1].rstrip("/")+"/"
LIB={"model":"🧱️model","document":"🧾️document","copy":"🧵️retained/📑️copy","mutation_laws":"🌿️vcs/⚖️laws","snapshot_laws":"🚪️io/⚖️laws"}
ALIAS={"neural_engine":"neural_engine","pack":"semio-framework-pack","protocol":"semio-framework-replication","semio_framework_os_kernel":"semio-framework-os-kernel"}
def literal(files,tests):
    s=set()
    for f in files:
        if ("🧪️" in f)!=tests and not tests:continue
        t=open(f,encoding="utf-8").read()
        t=re.sub(r'//[^\n]*','',t)
        for m in re.finditer(r'(?<![\w:])(semio_framework_[a-z_0-9]+|neural_engine|serde_json|serde|protocol|pack)\s*::',t):s.add(m.group(1))
        for m in re.finditer(r'owner\s*=\s*(semio_framework_[a-z_]+)',t):s.add(m.group(1))
        for m in re.finditer(r'use\s+(semio_framework_[a-z_0-9]+|serde|neural_engine)\b',t):s.add(m.group(1))
        for m in re.finditer(r'extern crate (semio_framework_[a-z_]+)',t):s.add(m.group(1))
    return s
for cid,node in LIB.items():
    cargo=json.loads(subprocess.run(["bun","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/osflow-tools/tomljson.ts",W+node+"/📦️packages/🦀️rust/Cargo.toml"],capture_output=True,text=True).stdout)
    deps=set(cargo.get("dependencies",{}));dev=set(cargo.get("dev-dependencies",{}))
    files=ffu.owned(W+node+"/🦀️.rs")
    allf=[f for f in files]
    lib=literal(allf,False)
    tst=set()
    for f in allf:
        if "🧪️" in f:
            tst|=literal([f],True)
    norm=lambda n:n.replace("_","-")
    def covered(name,pool):
        n=norm(name)
        return any(norm(d)==n or d==ALIAS.get(name) for d in pool) or n=="semio-framework-artifact-flow-flow-document" 
    print("==",cid)
    print("  lib literal not declared:",sorted(x for x in lib if not covered(x,deps|dev) and not x.startswith("semio_framework_artifact_flow_flow")))
    print("  test literal not declared:",sorted(x for x in tst if not covered(x,deps|dev) and not x.startswith("semio_framework_artifact_flow_flow")))
    declared_unused=[d for d in deps if norm(d) not in {norm(x) for x in lib|tst} and d not in ALIAS.values() and not d.startswith("semio-framework-artifact-flow-flow")]
    print("  declared deps with no literal use:",sorted(declared_unused))
