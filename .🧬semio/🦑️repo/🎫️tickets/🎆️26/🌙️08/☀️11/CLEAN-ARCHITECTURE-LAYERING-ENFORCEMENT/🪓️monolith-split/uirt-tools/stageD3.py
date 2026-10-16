"""⏭️ Stage D3: assembles the cursor crate root and the authority crate root from the draft slices and writes them."""
import re,sys,os
sys.path.insert(0,".")
from prim_names import prim_items,REPO,RT
R=f"{REPO}/{RT}"
D="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt"
rd=lambda n:open(f"{D}/{n}",encoding="utf-8").read()
CFG='#[cfg(any(test, feature = "testkit"))]'
shadow,cursor,oracle,owner,upper=rd("shadow-draft.rs"),rd("cursor-slice.rs"),rd("oracle-slice.rs"),rd("owner-slice.rs"),rd("upper-slice.rs")
items=prim_items()
def used(name,text): return re.search(r"(?<![A-Za-z0-9_])"+re.escape(name)+r"(?![A-Za-z0-9_])",text) is not None
def imports(body,cursor_names=()):
    out=[]
    std=[n for n in ("size_of","take") if (n=="size_of" and re.search(r"\bsize_of\b",body)) or (n=="take" and re.search(r"(?<![.\w])take\(",body))]
    if std: out.append("use std::mem::{"+", ".join(std)+"};")
    if re.search(r"\bLazyLock\b|\bMutex\b|\bOnceLock\b|\bMutexGuard\b",body):
        names=[n for n in ("LazyLock","Mutex","MutexGuard","OnceLock") if used(n,body)]
        out.append("use std::sync::{"+", ".join(names)+"};")
    out.append("")
    pres=[n for n in ("ComponentTree","TreeNode","UiNativeResourceLoans","UiNativeResources") if used(n,body)]
    if pres: out.append("use semio_framework_ui_runtime_presentation::{"+", ".join(pres)+"};")
    plain=sorted(n for n,(m,k,g) in items.items() if not g and used(n,body))
    gated=sorted(n for n,(m,k,g) in items.items() if g and used(n,body))
    if plain: out.append("use semio_framework_ui_runtime_reconcile_primitives::{"+", ".join(plain)+"};")
    if gated: out+= [CFG,"use semio_framework_ui_runtime_reconcile_primitives::{"+", ".join(gated)+"};"]
    return out,plain,gated
# ---- cursor root
cbody=shadow+"\n\n"+cursor+"\n\n"+oracle+"\n"
cdoc='''//! ⏭️ The retained surface shadow and the resumable reconcile cursor that diffs a presented tree against it — presentation discovery, identity allocation, postorder record diffing and stale-tree removal each advance one node at a time, and the shadow is read-only until the cursor completes. The authority crate composes the shadow with the process-wide handback registry; this crate knows no registry, no job and no renderer.
//!
//! The one property that makes the emitted patches worth anything: identity comes from **`(parent, key)`, never from
//! position**. A node keeps its [`ui_contract::UiNodeId`] across reorders, insertions and removals of its siblings,
//! which is exactly what lets renderer-side state (scroll offset, focus, a DOM node, a GPU cache entry) survive a
//! re-present instead of being torn down and rebuilt every frame.
'''
imp,plain,gated=imports(cbody)
croot=cdoc+"\n"+"\n".join(imp)+"\n\n"+CFG+'\n#[path = "🧰️testkit/🦀️.rs"]\npub mod testkit;\n\n'+cbody
os.makedirs(f"{R}/⏭️cursor",exist_ok=True)
open(f"{R}/⏭️cursor/🦀️.rs","w",encoding="utf-8").write(croot)
print("cursor imports",plain,gated)
# ---- authority root
udoc='''//! ♻️ The reconcile authority — the thin [`SurfaceReconciler`] owner that composes a retained shadow with its handback reservation, the credit reservation, the process-wide handback registry, the resumable job, the generation-qualified patch and document owners, the terminal close authority and the surface output pool. The shadow and cursor it drives live in `semio_framework_ui_runtime_reconcile_cursor`; the bounded primitives in `semio_framework_ui_runtime_reconcile_primitives`.
'''
ubody=owner+"\n\n"+upper+"\n"
imp,plain,gated=imports(ubody)
cur_names=[n for n in ("SurfaceShadow","SurfaceReconcileCursor","SurfaceReconcileStep","SurfaceReconcileStage","SurfaceRecordRead") if used(n,ubody)]
imp.append("use semio_framework_ui_runtime_reconcile_cursor::{"+", ".join(cur_names)+"};")
uroot=udoc+"\n"+"\n".join(imp)+'''

#[path = "../📤️output/🦀️.rs"]
mod output;
pub use output::{SurfaceReconcileOutputReservation, SurfaceReconcileOutputs, SurfaceReconcileOutputTransfer};

'''+ubody
open(f"{R}/♻️reconcile/🦀️.rs","w",encoding="utf-8").write(uroot)
print("upper imports",plain,gated,cur_names)
