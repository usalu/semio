"""✏️ F-04(a) Rust text re-pointing: replaces `semio_framework::A::`, `semio_framework_plugin::A::`, `semio_framework_os_host::A::` by the real producer crate path for the given aliases, in the files of a plan, outside the skip zones (🔌️plugin, 📺️renderer) unless --zone all.
Every file is re-read right before its write; the number of replacements must equal the planned hits; the pre-image goes to the scratchpad rsl-pre/.
usage: facade_apply.py <plan.json> <alias,alias,...> [--write] [--zone ok|all]"""
import json, os, re, sys, shutil
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from facade_lib import *
PRE="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/rsl-pre/"
REALP={"abi":"semio_framework_abi","action_bus":"semio_framework_action_bus","interaction":"semio_framework_interaction","onboarding":"semio_framework_onboarding","history_edit":"semio_framework_time_travel_actions","tool_run":"semio_framework_tool_run_actions","platform":"semio_framework_action_bus::platform","ui":"semio_framework::manifest","io_schema":"semio_framework_io_schema","sqlite_snapshot":"semio_framework_io_sqlite_snapshot","mesh_io":"semio_framework_mesh_engine::io"}
plan=json.load(open(sys.argv[1])); aliases=sys.argv[2].split(","); write="--write" in sys.argv; zone="all" if "--zone" in sys.argv and sys.argv[sys.argv.index("--zone")+1]=="all" else "ok"
PREF="|".join(["semio_framework","semio_framework_plugin","semio_framework_os_host"])
rx=re.compile(r'(?<![A-Za-z0-9_])(?:'+PREF+r')::('+"|".join(aliases)+r')(?=::)')
changed=[];skipped=[]
for p in plan:
    f=p["file"]
    if zone=="ok" and in_skip_zone(f): skipped.append(f); continue
    want=sum(1 for h in p["hits"] if h["alias"] in aliases and h["auto"])
    if not want: continue
    t=open(f,encoding="utf8").read()
    def sub(m):
        a=m.group(1)
        if a=="ui" and t[m.end():m.end()+8]=="::kernel": return "semio_framework"
        return REALP[a]
    n=len(rx.findall(t))
    if n!=want: print("MISMATCH",f,n,want); sys.exit(2)
    out=rx.sub(sub,t)
    if out==t: continue
    changed.append((f,n))
    if write:
        dest=PRE+f
        if not os.path.exists(dest):
            os.makedirs(os.path.dirname(dest),exist_ok=True); shutil.copy2(f,dest)
        cur=open(f,encoding="utf8").read()
        assert cur==t, "concurrent edit "+f
        open(f,"w",encoding="utf8").write(out)
print(("WROTE" if write else "DRY"),len(changed),"files",sum(n for _,n in changed),"replacements; skipped by zone",len(skipped))
open(G+"/rsl-rust-"+("write" if write else "dry")+".txt","w").write("\n".join(f"{n}\t{f}" for f,n in changed)+"\n")
