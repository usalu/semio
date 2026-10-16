import sys,os,shutil,json,collections
sys.path.insert(0,os.path.dirname(__file__))
from oracle import *
dst=f"{S}/base"
if not os.path.isdir(dst) or "--fresh" in sys.argv:
    shutil.rmtree(dst,ignore_errors=True); shutil.copytree(f"{S}/orig",dst)
    patch_tree(dst)
lib=f"{dst}/{RT}/📦️packages/🦀️rust/🦀️.rs"
res={}
for cfg in sys.argv[1:] or ["native","test"]:
    if cfg.startswith("--"): continue
    extra=CONFIGS.get(cfg) or TARGETS.get(cfg.split("+")[0],[])
    if "+" in cfg: extra=TARGETS[cfg.split("+")[0]]+CONFIGS[cfg.split("+")[1]]
    out,err=run(lib,"semio_framework_ui_runtime",extra,dst)
    res[cfg]=out
    print(cfg,sum(out.values()),"errors",len(out),"distinct")
    json.dump([[list(k),v] for k,v in sorted(out.items())],open(f"{S}/base-{cfg}.json","w"),ensure_ascii=False,indent=0)
