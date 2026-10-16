"""🧱️ Stage A: carves the primitives crate files out of the reconcile module and trims the reconcile root (from the pristine copy, idempotent)."""
import os,re,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt/orig"
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
src=open(f"{S}/{RT}/♻️reconcile/🦀️.rs",encoding="utf-8").read()
cur=open(f"{REPO}/{RT}/♻️reconcile/🦀️.rs",encoding="utf-8").read()
lines=src.split("\n")
def L(a,b): return "\n".join(lines[a-1:b])
def chk(n,frag):
    assert frag in lines[n-1],(n,frag,lines[n-1][:80])
# anchors
chk(29,"//#region 🔖️Identity");chk(237,"//#endregion 🔖️Identity");chk(554,"impl<K, V, const N: usize> SurfaceLinearMap");chk(576,"}")
chk(578,"//#endregion 🔖️Reconciler");chk(580,"//#region ⏭️ResumableReconcile");chk(599,"/// 🚧️ Fixed admission bounds");chk(656,"}");chk(658,"struct FlatPresentedNode")
chk(744,"}") if lines[743].strip()=="}" else None
chk(746,"const SURFACE_RECONCILE_VALUE_DEPTH");chk(747,"SEMANTIC_COPIES");chk(749,"#[cfg(test)]");chk(766,"}");chk(768,"struct SurfaceSemanticMapPage");chk(1380,"}")
chk(2404,"SURFACE_RECONCILE_PAGE_BYTES");chk(2411,"SURFACE_RECONCILE_SURFACE_BYTES");chk(3851,"fn diff_record_field");chk(3945,"}");chk(3947,"//#endregion ⏭️ResumableReconcile")
chk(4137,"fn build_record_owned");chk(4165,"}");chk(4167,"//#endregion 🔖️Diff");chk(4169,"//#region 🧪️Tests");chk(4173,"//#endregion 🧪️Tests")
chk(3028,"#[cfg(test)]");chk(3030,"mod patch_handoff_tests");chk(3845,"#[cfg(test)]");chk(3847,"mod handback_entry_tests")
PRES="semio_framework_ui_runtime_presentation"
def pres_imports(text):
    names=[n for n in ("ComponentTree","TreeNode","UiNativeResourceLoans","UiNativeResources") if re.search(r"\b"+n+r"\b",text)]
    return names
def retarget(text):
    for n in ("ComponentTree","TreeNode","UiNativeResourceLoans","UiNativeResources"):
        text=text.replace("crate::"+n,n)
    return text
def std_uses(text):
    out=[]
    names=[]
    if re.search(r"\bsize_of\b",text): names.append("size_of")
    if re.search(r"(?<![.\w])take\(",text): names.append("take")
    if names: out.append("use std::mem::{"+", ".join(names)+"};")
    return out
def file_head(doc,text,extra=()):
    uses=[]
    uses+=std_uses(text)
    uses+=list(extra)
    n=pres_imports(text)
    if n: uses.append("use "+PRES+"::{"+", ".join(n)+"};")
    return doc+"\n\n"+("\n".join(uses)+"\n\n" if uses else "")
out={}
# containers
ident=L(29,237)
recon_impls=L(554,576)
cont_body=ident+"\n\n//#region 🔖️Reconciler\n\n"+recon_impls+"\n\n//#endregion 🔖️Reconciler\n"
cont_body=retarget(cont_body)
doc_cont="//! 🗃️ Fixed-capacity linear containers — the bounded vector, map and set every reconcile cursor and retained reconciler keys its identities with. They allocate their whole capacity once and refuse the next push instead of growing."
out["🗃️containers"]=file_head(doc_cont,cont_body,extra=("#[cfg(test)]\nuse std::collections::HashSet;",))+cont_body
# limits
lim_chunk=L(599,656)+"\n\n"+L(746,747)+"\n\n"+L(749,766)
ra="//#region 🎟️RetainedAuthority\n\n"+L(2404,2411)+"\n\n//#endregion 🎟️RetainedAuthority"
lim_body="//#region ⏭️ResumableReconcile\n\n"+lim_chunk+"\n\n"+ra+"\n\n//#endregion ⏭️ResumableReconcile\n"
doc_lim="//! 🚧️ Admission bounds of one reconciliation — the limits a reconcile may spend, the usage it has observed, the typed faults that end it, and the page and work-byte constants every step is priced in."
out["🚧️limits"]=file_head(doc_lim,lim_body)+lim_body
# census
cen_body="//#region ⏭️ResumableReconcile\n\n"+L(768,1380)+"\n\n//#endregion ⏭️ResumableReconcile\n"
cen_body=retarget(cen_body)
doc_cen="//! 📊️ Semantic ownership census — charges the complete semantic ownership of one presented node in bounded pages, so a producer can price a subtree before it reconciles it."
out["📊️census"]=file_head(doc_cen,cen_body)+cen_body
# records
rec_body="//#region ⏭️ResumableReconcile\n\n"+L(658,744).rstrip("\n")+"\n\n"+L(3851,3945)+"\n\n//#endregion ⏭️ResumableReconcile\n\n//#region 🔖️Diff\n\n"+L(4137,4165)+"\n\n//#endregion 🔖️Diff\n"
rec_body=retarget(rec_body)
doc_rec="//! 🧩️ Record diff primitives — the bounded copy, comparison and retirement cursors one node record is diffed and retired with, and the record builders that assemble it."
out["🧩️records"]=file_head(doc_rec,rec_body)+rec_body
# trimmed reconcile root
cut=[]
def drop(a,b): cut.append((a,b))
drop(17,18)   # cfg(test) HashSet
drop(22,27)   # output + tree mounts rewritten below
drop(29,237)  # identity
drop(238,238)
drop(554,577) # impls on moved containers (+ blank)
drop(599,657) # limits, usage, fault
drop(658,745) # records
drop(746,748) # consts + blank
drop(749,767) # admit_vec_backing
drop(768,1381) # census
drop(2404,2411)
drop(3028,3032) # patch tests mount (3028-3030 + blanks)
drop(3845,3848)
drop(3851,3946) # diff fns
drop(4137,4166) # build_record*
drop(4168,4173) # tests region
keep=[True]*(len(lines)+1)
for a,b in cut:
    for i in range(a,b+1): keep[i]=False
core_lines=[lines[i-1] for i in range(1,len(lines)+1) if keep[i]]
core="\n".join(core_lines)
core=core.rstrip("\n")+"\n"
out["__core__"]=core
for k,v in out.items(): pass
import json
json.dump({k:v for k,v in out.items()},open("/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt/stageA-draft.json","w"),ensure_ascii=False)
for k,v in out.items(): print(k,len(v.split("\n")),"lines",len(v.encode()),"bytes")

if "--write" in sys.argv:
    P=f"{REPO}/{RT}/🧱️primitives"
    for mod in ("🗃️containers","🚧️limits","📊️census","🧩️records"):
        os.makedirs(f"{P}/{mod}",exist_ok=True)
        open(f"{P}/{mod}/🦀️.rs","w",encoding="utf-8").write(out[mod])
    open(f"{REPO}/{RT}/♻️reconcile/🦀️.rs","w",encoding="utf-8").write(out["__core__"])
    print("written")
