"""🔬️ Union differential: simulates the carved crates as modules of one crate (crate-boundary privacy kept via pub(in …)), analysis only."""
import collections,json,os,re,shutil,subprocess,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from oracle import S,REPO,RT,EXTRA_FIXTURES,TOOLCHAIN,patch_tree,run
CRATES={
 "semio_framework_ui_runtime_exchange":"📡️exchange/🦀️.rs",
 "semio_framework_ui_runtime_presentation":"🖼️presentation/🦀️.rs",
 "semio_framework_ui_runtime_reconcile_primitives":"🧱️primitives/🦀️.rs",
 "semio_framework_ui_runtime_reconcile_cursor":"⏭️cursor/🦀️.rs",
 "semio_framework_ui_runtime_reconcile":"♻️reconcile/🦀️.rs",
 "semio_framework_ui_runtime":"📦️packages/🦀️rust/🦀️.rs",
}
PATHRE=re.compile(r'#\[path\s*=\s*"([^"]+)"\s*\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+')
def closure(root_abs,tree):
    seen={};stack=[root_abs]
    while stack:
        f=stack.pop()
        if f in seen: continue
        seen[f]=True
        if not os.path.exists(f): continue
        t=open(f,encoding="utf-8").read()
        for m in PATHRE.finditer(t):
            stack.append(os.path.normpath(os.path.join(os.path.dirname(f),m.group(1))))
    return list(seen)
def build(name):
    dst=f"{S}/{name}"
    shutil.rmtree(dst,ignore_errors=True); os.makedirs(dst)
    for rel in [RT]+EXTRA_FIXTURES:
        src=f"{REPO}/{rel}"; out=f"{dst}/{rel}"
        os.makedirs(os.path.dirname(out),exist_ok=True)
        shutil.copytree(src,out,ignore=shutil.ignore_patterns("target","Cargo.toml","📋️project.json"))
    base=f"{dst}/{RT}"
    owner={}
    for crate,rel in CRATES.items():
        for f in closure(f"{base}/{rel}",base): owner.setdefault(f,crate)
    names=sorted(CRATES,key=len,reverse=True)
    for f,crate in owner.items():
        if not os.path.exists(f): print("MISSING mount",f.replace(base+"/","")); continue
        t=open(f,encoding="utf-8").read()
        t=re.sub(r"(?<![\w:])crate::",f"crate::{crate}::",t)
        for n in names:
            t=re.sub(r"(?<![\w:])"+n+r"(?![\w])","crate::"+n,t)
        t=re.sub(r"pub\(crate\)",f"pub(in crate::{crate})",t)
        open(f,"w",encoding="utf-8").write(t)
    root=f"{base}/📦️packages/🦀️rust/union.rs"
    lines=[]
    for crate,rel in CRATES.items():
        lines.append(f'#[path = "../../{rel}"]\npub mod {crate};')
    open(root,"w",encoding="utf-8").write("\n".join(lines)+"\n")
    patch_tree(dst)
    return dst,root,owner
def norm(msg):
    msg=re.sub(r"`?crate::semio_framework_ui_runtime\w*::","`",msg)
    msg=re.sub(r"semio_framework_ui_runtime\w*::","",msg)
    return msg.replace("crate::","")
EXT=re.compile(r"`(ui_contract|ui_scene|semio_framework_job|semio_framework_value|semio_framework_trace|serde_json|serde)`")
def residual(counter):
    res=collections.Counter()
    for (code,msg,loc,label),n in counter.items():
        if EXT.search(msg) or msg.startswith("aborting") : continue
        res[(code,norm(msg),re.sub(r".*"+re.escape(RT)+"/","",loc))]+=n
    return res
if __name__=="__main__":
    cfgs=sys.argv[1:] or ["native","testkit","test","testkit-test"]
    from oracle import CONFIGS,TARGETS
    CONFIGS={**CONFIGS,**TARGETS}
    dst,root,owner=build("union")
    allres={}
    for cfg in cfgs:
        out,err=run(root,"union",CONFIGS[cfg],dst)
        r=residual(out)
        allres[cfg]=r
        print(f"== {cfg}: total {sum(out.values())} errors, residual (non-external) {sum(r.values())}")
        for (code,msg,loc),n in sorted(r.items(),key=lambda x:(x[0][2],x[0][1]))[:80]: print(f"   {n:3} {code} {msg[:110]} @ {loc[-70:]}")
