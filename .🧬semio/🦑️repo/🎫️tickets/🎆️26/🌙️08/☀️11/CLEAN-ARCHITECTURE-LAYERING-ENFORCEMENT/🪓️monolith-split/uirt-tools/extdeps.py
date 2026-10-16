"""🔎️ External crate names each ui-runtime crate's closure references, split into production and test files, against the manifest."""
import os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from union import CRATES,closure
from oracle import REPO,RT
from widen import strip_comments
base=f"{REPO}/{RT}"
EXT=["ui_contract","ui_scene","semio_framework_job","semio_framework_value","semio_framework_trace","serde_json","serde","semio_framework_ui_runtime_exchange","semio_framework_ui_runtime_presentation","semio_framework_ui_runtime_reconcile_primitives","semio_framework_ui_runtime_reconcile_cursor","semio_framework_ui_runtime_reconcile"]
MAN={"semio_framework_ui_runtime_exchange":"📡️exchange","semio_framework_ui_runtime_presentation":"🖼️presentation","semio_framework_ui_runtime_reconcile_primitives":"🧱️primitives","semio_framework_ui_runtime_reconcile_cursor":"⏭️cursor","semio_framework_ui_runtime_reconcile":"♻️reconcile","semio_framework_ui_runtime":"📦️packages/🦀️rust/.."}
for crate,rel in CRATES.items():
    node=MAN[crate]
    cargo=f"{base}/{node}/📦️packages/🦀️rust/Cargo.toml" if not node.startswith("📦") else f"{base}/📦️packages/🦀️rust/Cargo.toml"
    txt=open(cargo,encoding="utf-8").read()
    def section(name):
        mm=re.search(r"^\["+re.escape(name)+r"\]\n(.*?)(?=^\[|\Z)",txt,re.M|re.S)
        out=set()
        if mm:
            for l in mm.group(1).split("\n"):
                k=re.match(r"^([A-Za-z0-9_\-]+)\s*=",l)
                if k: out.add(k.group(1).replace("-","_"))
        return out
    deps=section("dependencies"); dev=section("dev-dependencies")
    prod=collections.Counter(); test=collections.Counter()
    for f in closure(f"{base}/{rel}",base):
        if not os.path.exists(f) or not f.endswith(".rs"): continue
        t=strip_comments(open(f,encoding="utf-8").read())
        is_test=("🧪️tests" in f) or ("/🧰️testkit/" in f)
        for e in EXT:
            if e==crate: continue
            n=len(re.findall(r"(?<![\w:])"+e+r"(?![\w])",t))
            if n: (test if is_test else prod)[e]+=n
    used_prod=set(prod); used_test=set(test)-used_prod
    missing_prod=sorted(e for e in used_prod if e not in deps)
    missing_test=sorted(e for e in used_test if e not in deps and e not in dev)
    unused=sorted(d for d in deps if d not in used_prod and d not in used_test)
    print(crate.replace("semio_framework_ui_runtime","rt"),"| prod",sorted(used_prod),"| test-only",sorted(used_test))
    print("    missing(prod):",missing_prod,"missing(test):",missing_test,"unused deps:",unused,"dev:",sorted(dev))
