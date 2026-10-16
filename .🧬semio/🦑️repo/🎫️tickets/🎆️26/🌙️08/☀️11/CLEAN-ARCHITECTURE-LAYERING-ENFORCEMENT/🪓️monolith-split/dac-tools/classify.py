"""📋️ Classify the 183 audited alias rows by outcome; prints per-outcome counts and the markdown table body."""
import json,re,os,collections
os.chdir("/Users/ueli/Documents/semio")
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
rows=[l.rstrip("\n").split("\t") for l in open(G+"/dac-alias-hits.tsv",encoding="utf8")]
def exists(path,line):
    try: t=open(path,encoding="utf8").read()
    except Exception: return None
    return line[:70] in t
def classify(path,line):
    if path.endswith(".ts"): return "n/a","not an alias: TS test text about Rust source","TS"
    if "🗄️stdio" in path: return "x","stdio artifacts owned by the running pch slice; untouched","stdio"
    if "wp3-owners" in path or "wp1-foundation/🧪️test-hosts" in path: return "x","plugin hub owned by the running pi1 slice; untouched","pi1 hub"
    m=re.search(r"trace_memory_testing as test_allocation",line)
    if m:
        return ("a-done" if not exists(path,line) else "a-blocked"),("alias deleted; every `test_allocation::` use now names `semio_framework_trace_memory_testing`" if not exists(path,line) else "shared test mount with a crate that has no alias or dependency (see blocked list)"),"test_allocation"
    return None
out=[]
for path,line in rows:
    c=classify(path,line)
    out.append((path,line,c))
cnt=collections.Counter(c[0] if c else "?" for _,_,c in out)
print(cnt)
for p,l,c in out:
    if c is None: print("?",p[-60:],l[:80])

MAP=[
("📦️packages/🦀️rust/🦀️.rs#fw","b","framework facade `semio_framework`: the module names are the public API; ext consumers ui 640, abi 48, action_bus 328, interaction 10, onboarding 210, history_edit 48, tool_run 4. `source_projection` had 0 consumers and was deleted."),
]
def grp(path,line):
    p=path
    if p=="🧰️framework/📦️packages/🦀️rust/🦀️.rs":
        return "fw"
    for key,name in [("◻️2d/🦀️.rs","2d"),("🎒️pack/🌱️value/🦀️.rs","packval"),("🎒️pack/📐️format","packfmt"),("🎒️pack/📦️packages","packpkg"),("🎟️retained-turn","rt"),("💼️job/📨️publication","jp"),("📖️vocabulary","voc"),("🎭️actor/🦀️.rs","actor"),("🎮️mutation/📨️messages","repmsg"),("📡️replication/📦️packages","reppkg"),("🎬️scene","scene"),("🔌️device/📥️platform","platform"),("🖍️draw/🦀️.rs","draw"),("🗺️surface","surface"),("🐾️pets","pets"),("wp1-leaves","leaves"),("♾️infinite","inf"),("🌊️flow/📦️packages","flow"),("🌊️flow/🗿️artifacts/🌊️flow/🧱️model","neural"),("🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs","copy"),("📸️snapshot/🦀️.rs","cst"),("🏪️store/🚪️io/🦀️.rs","textio"),("💻️os/🔨️modules/🏪️store/🦀️.rs","store"),("📡️spr/🦀️.rs","spr"),("prepared-engine-canvas","egpu"),("IconRenderHost","icon"),("🎭️guest-runtime","grt"),("🔗️wit-bindings","wit"),("🪟️window/🦀️.rs","window"),("🚪️io/🦀️.rs","osio"),("🛢️db","db"),("🧠️neural","nn"),("🪡️list-delta/🦀️.rs","ld"),("🖥️host/📦️packages","hostpkg"),("🖥️host/🦀️.rs","host"),("🖥️server","server"),("⌨️cli/🦀️.rs","cli"),("🧭️repo-cli","repocli")]:
        if key in p: return name
    return "?"
