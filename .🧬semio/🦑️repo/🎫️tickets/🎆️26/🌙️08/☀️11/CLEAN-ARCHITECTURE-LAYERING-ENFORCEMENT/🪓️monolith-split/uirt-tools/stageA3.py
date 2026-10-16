"""♻️ Stage A3: rewrites the reconcile root header, cfg gating and presentation paths after stage A."""
import re,sys
sys.path.insert(0,".")
from prim_names import prim_items,REPO,RT
path=f"{REPO}/{RT}/♻️reconcile/🦀️.rs"
t=open(path,encoding="utf-8").read()
def once(old,new):
    global t
    assert t.count(old)==1,(t.count(old),old[:80])
    t=t.replace(old,new)
once("//! ♻️ Keyed reconciliation of a [`crate::ComponentTree`] into","//! ♻️ Keyed reconciliation of a [`ComponentTree`] into")
once("//! [`crate::present`] produces into","//! [`semio_framework_ui_runtime_presentation`] produces into")
once('''    #[cfg(not(test))]
    pub fn new(surface: ui_contract::SurfaceId) -> Self {
        Self::from_surface_id(surface)
    }

    #[cfg(test)]
    pub fn new(surface: impl AsRef<str>) -> Self {''','''    pub fn new(surface: ui_contract::SurfaceId) -> Self {
        Self::from_surface_id(surface)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn from_text(surface: impl AsRef<str>) -> Self {''')
t=t.replace("#[cfg(test)]",'#[cfg(any(test, feature = "testkit"))]')
for n in ("ComponentTree","TreeNode","UiNativeResourceLoans","UiNativeResources"):
    t=t.replace("crate::"+n,n)
items=prim_items()
plain=[];gated=[]
body=t
for n,(m,k,g) in sorted(items.items()):
    if re.search(r"(?<![A-Za-z0-9_])"+re.escape(n)+r"(?![A-Za-z0-9_])",body):
        (gated if g else plain).append(n)
pres=[n for n in ("ComponentTree","TreeNode","UiNativeResourceLoans","UiNativeResources") if re.search(r"(?<![A-Za-z0-9_])"+n+r"(?![A-Za-z0-9_])",body)]
hdr='''use std::mem::{size_of, take};
use std::sync::{LazyLock, Mutex, MutexGuard, OnceLock};

use semio_framework_ui_runtime_presentation::{'''+", ".join(pres)+'''};
use semio_framework_ui_runtime_reconcile_primitives::{'''+", ".join(plain)+'''};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_ui_runtime_reconcile_primitives::{'''+", ".join(gated)+'''};

#[path = "../📤️output/🦀️.rs"]
mod output;
pub use output::{SurfaceReconcileOutputReservation, SurfaceReconcileOutputs, SurfaceReconcileOutputTransfer};
'''
old='''use std::mem::{size_of, take};
use std::sync::{LazyLock, Mutex, MutexGuard, OnceLock};
'''
assert t.count(old)==1
t=t.replace(old,hdr)
open(path,"w",encoding="utf-8").write(t)
print("plain",plain);print("gated",gated);print("pres",pres)
