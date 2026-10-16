"""✂️ Re-point the per-crate `test_allocation` alias of semio_framework_trace_memory_testing to the real crate path and delete the alias. Dry run unless --apply."""
import json,re,os,subprocess,collections,sys
os.chdir("/Users/ueli/Documents/semio")
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
APPLY="--apply" in sys.argv
reach=json.load(open(G+"/dac-reach.json"))
plan=json.load(open(G+"/dac-ta-plan.json"))
SKIP_LIBS={"semio_s_artifact_stdio_dxf","semio_framework_plugin_host_test_component","semio_framework_os_mcp","semio_framework_os_mcp_plugin_runtime","semio_framework_os_mcp_foundation"}
DEF=re.compile(r"^(?:[ \t]*#\[cfg\(test\)\][ \t]*\n)?[ \t]*(?:#\[cfg\(test\)\][ \t]*)?(?:pub(?:\([^)]*\))?[ \t]+)?use[ \t]+semio_framework_trace_memory_testing[ \t]+as[ \t]+test_allocation[ \t]*;[ \t]*\n",re.M)
SUPERUSE=re.compile(r"^[ \t]*use[ \t]+(?:super|crate|semio_framework_os_store_ephemeral)::test_allocation[ \t]*;[ \t]*\n",re.M)
NEW="semio_framework_trace_memory_testing"
CRATE=re.compile(r"(?:crate|super)::test_allocation::")
BARE=re.compile(r"(?<!::)(?<![A-Za-z0-9_])test_allocation::")
tok=re.compile(r"(?<![A-Za-z0-9_])test_allocation(?![A-Za-z0-9_])")
files=plan["text_files"]
blocked=set(); filelibs={}
text={f:open(f,encoding="utf8").read() for f in files}
deflibs=set()
for f in files:
    if DEF.search(text[f]):
        for o in reach.get(f,[]): deflibs.add(o["lib"])
for f in files:
    owners=reach.get(f,[])
    libs={o["lib"] for o in owners}
    filelibs[f]=libs
    if not owners or libs&SKIP_LIBS or (libs-deflibs): blocked.add(f)
    if "mod test_allocation" in text[f] or "semio_framework_os_mcp_foundation::test_allocation" in text[f]: blocked.add(f)
# a crate with any blocked file is blocked entirely (all-or-nothing)
changed=True
while changed:
    changed=False
    badlibs={l for f in blocked for l in filelibs[f]}
    for f in files:
        if f not in blocked and filelibs[f]&badlibs: blocked.add(f); changed=True
EXTRA=[f for f in files if f.endswith("🔬️schema-hash/💰️storage/🦀️.rs")]
todo=[f for f in files if f not in blocked]
manifest_ok=True
for f in todo:
    for o in reach.get(f,[]):
        mt=open(o["manifest"],encoding="utf8").read()
        if "semio-framework-trace-memory-testing" not in mt: print("NODEP",o["manifest"][-80:]); manifest_ok=False
print("todo",len(todo),"blocked",len(blocked),"manifest_ok",manifest_ok)
for f in sorted(blocked): print("BLOCKED",sorted(filelibs[f]) or "NOOWNER",f[-80:])
tot=0
for f in todo:
    t=text[f]; n0=len(tok.findall(t))
    t=DEF.sub("",t); t=SUPERUSE.sub("",t)
    t=CRATE.sub(NEW+"::",t); t=t.replace("semio_framework_os_kernel::test_allocation::",NEW+"::"); t=BARE.sub(NEW+"::",t)
    left=tok.findall(t)
    if left: print("RESIDUAL",f[-80:],len(left))
    tot+=n0
    if APPLY and t!=text[f]:
        assert open(f,encoding="utf8").read()==text[f],"changed under me "+f
        open(f,"w",encoding="utf8").write(t)
print("tokens",tot)
