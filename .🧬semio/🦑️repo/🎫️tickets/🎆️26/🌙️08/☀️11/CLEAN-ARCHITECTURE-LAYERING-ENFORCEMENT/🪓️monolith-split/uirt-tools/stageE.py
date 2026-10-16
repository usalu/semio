"""✂️ Stage E: writes the split suites (cursor, authority, primitives) from the pristine monolith suites."""
import re,sys,os
sys.path.insert(0,".")
from splittests import parse,classify,S,RT
REPO="/Users/ueli/Documents/semio"
R=f"{REPO}/{RT}"
D="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt/E"
os.makedirs(D,exist_ok=True)
def emit(blocks,side,want,skip_names=()):
    """Re-emits blocks of `want` sides plus the helper fns they call, keeping region pairs that enclose an emitted block."""
    from widen import strip_comments
    sel=set()
    for i,b in enumerate(blocks):
        if b["kind"]=="item" and b["name"] and b["name"] not in skip_names and not b["name"].startswith("impl:"):
            s=side.get(b["name"],"ALL")
            if s=="ALL" or s in want: sel.add(i)
    def is_test(b): return "#[test]" in b["text"].split("\n")[0]
    helpers={i:b for i,b in enumerate(blocks) if b["kind"]=="item" and b["name"] and not b["name"].startswith("impl:") and not is_test(b) and b["name"] not in skip_names}
    changed=True
    while changed:
        changed=False
        body="\n".join(strip_comments(blocks[i]["text"]) for i in sel)
        for i,b in helpers.items():
            if i in sel: continue
            nm=re.escape(b["name"])
            pat=(r"(?<![A-Za-z0-9_.])"+nm+r"\(|[,(]\s*"+nm+r"\s*[,)]") if b["kw"]=="fn" else r"(?<![A-Za-z0-9_])"+nm+r"(?![A-Za-z0-9_])"
            if re.search(pat,body): sel.add(i); changed=True
        names={blocks[i]["name"] for i in sel}
        for i,b in enumerate(blocks):
            if b["kind"]=="item" and b["name"] and b["name"].startswith("impl:") and i not in sel and b["name"][5:] in names: sel.add(i); changed=True
    out=[];stack=[];pending=[]
    for i,b in enumerate(blocks):
        if b["kind"]=="marker":
            if b["text"].startswith("//#region"):
                stack.append(b); pending.append(b)
            else:
                top=stack.pop()
                if top in pending: pending.remove(top)
                else: out.append(b["text"])
            continue
        if i not in sel: continue
        for pnd in list(pending):
            out.append(pnd["text"]); pending.remove(pnd)
        out.append(b["text"])
    return "\n".join(out)
def write(rel_out,text):
    p=f"{R}/{rel_out}"
    os.makedirs(os.path.dirname(p),exist_ok=True)
    open(p,"w",encoding="utf-8").write(text.rstrip("\n")+"\n")
    print("wrote",rel_out,len(text.split("\n")),"lines")
def cur(t):
    t=t.replace("SurfaceReconciler::new(","SurfaceShadow::from_text(").replace("SurfaceReconciler","SurfaceShadow")
    t=t.replace("crate::surface_reconcile_registry_test_guard()","registry_guard()")
    return t
def up(t):
    t=t.replace("SurfaceReconciler::new(","SurfaceReconciler::from_text(")
    return t
# ---- unit
lines,blocks=parse("🧪️tests/🔬️reconcile-unit/🦀️.rs"); side=classify(blocks)
FIX={"ui_text","leaf","text","container","tree","ui_list","ui_map","styled","with_shortcut","with_binding","with_menu","id_of","assert_snapshot_matches_state"}
# fixtures region removed from both (they live in the cursor testkit); reconcile_resumable stays authority-side
body_cur=emit(blocks,side,{"CUR"},skip_names=FIX)
body_up=emit(blocks,side,{"UP"},skip_names=FIX)
# drop the (now empty) Fixtures region on the cursor side, keep for the authority side (holds reconcile_resumable)
write("🧪️tests/🔬️cursor-unit/🦀️.rs","use super::*;\nuse crate::testkit::*;\n\n#[cfg(test)]\n#[path = \"../../📏️ownership/🧪️tests/📏️ownership/🦀️.rs\"]\nmod ownership;\n\n#[cfg(test)]\n#[path = \"../../📃️document/🧪️tests/📃️document/🦀️.rs\"]\nmod canonical_document_tests;\n\n"+cur(body_cur))
write("🧪️tests/🔬️authority-unit/🦀️.rs","use super::*;\nuse semio_framework_ui_runtime_reconcile_cursor::testkit::*;\n\n#[cfg(test)]\n#[path = \"../../📏️ownership/🧪️tests/🔒️authority/🦀️.rs\"]\nmod ownership_authority;\n\n#[cfg(test)]\n#[path = \"../../📃️document/🧪️tests/🔒️authority/🦀️.rs\"]\nmod canonical_document_authority;\n\n#[cfg(test)]\n#[path = \"../../📤️output/🧪️tests/📤️output/🦀️.rs\"]\nmod output_pool_tests;\n\n#[cfg(test)]\n#[path = \"../../♻️retirement/🌲️tree/🧪️tests/🔒️authority/🦀️.rs\"]\nmod tree_retirement_authority;\n\n"+up(body_up))
# ---- ownership
lines,blocks=parse("📏️ownership/🧪️tests/📏️ownership/🦀️.rs"); side=classify(blocks)
head="use super::*;\n\n"
write("📏️ownership/🧪️tests/📏️ownership/🦀️.rs",head+cur(emit(blocks,side,{"CUR"})))
OWN_HELP="fixture"
upb=emit(blocks,side,{"UP"})
write("📏️ownership/🧪️tests/🔒️authority/🦀️.rs",head+up(upb))
# ---- document
lines,blocks=parse("📃️document/🧪️tests/📃️document/🦀️.rs"); side=classify(blocks)
write("📃️document/🧪️tests/📃️document/🦀️.rs",head+cur(emit(blocks,side,{"CUR"})))
write("📃️document/🧪️tests/🔒️authority/🦀️.rs",head+up(emit(blocks,side,{"UP"},)).replace("","",1))
# ---- tree
lines,blocks=parse("♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs"); side=classify(blocks)
write("♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs","use super::*;\n\n"+emit(blocks,side,{"CUR"}))
write("♻️retirement/🌲️tree/🧪️tests/🔒️authority/🦀️.rs",head+emit(blocks,side,{"UP"}))
# ---- primitives suite from the unit PRIM tests
lines,blocks=parse("🧪️tests/🔬️reconcile-unit/🦀️.rs"); side=classify(blocks)
write("🧪️tests/🔬️primitives-unit/🦀️.rs","use super::*;\n\n"+emit(blocks,side,{"PRIM"}))
