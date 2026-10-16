"""🧪️ Stage F: test mounts of the three suites, the shared ledger guard and the owner-level cleanup."""
import os,re,shutil
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
R=f"{REPO}/{RT}"
CFG='#[cfg(any(test, feature = "testkit"))]'
def rd(rel): return open(f"{R}/{rel}",encoding="utf-8").read()
def wr(rel,t): 
    os.makedirs(os.path.dirname(f"{R}/{rel}"),exist_ok=True); open(f"{R}/{rel}","w",encoding="utf-8").write(t)
def sub1(t,old,new):
    assert t.count(old)==1,(t.count(old),old[:80]); return t.replace(old,new)
# ---- primitives testkit (guard)
cur_tk=rd("⏭️cursor/🧰️testkit/🦀️.rs")
g0=cur_tk.index("pub struct RegistryGuard")
guard=cur_tk[g0:]
wr("🧱️primitives/🧰️testkit/🦀️.rs",'''//! 🧰️ The process-wide resident-ledger guard that serialises the laws admitting into the shared ledgers and drains their pending returns before and after each law. Exposed under the `testkit` feature so every reconcile crate guards its suites the same way.

use std::sync::{Mutex, MutexGuard, OnceLock};

'''+guard)
cur_tk=cur_tk[:g0].rstrip("\n")+"\n"
cur_tk=sub1(cur_tk,"use std::sync::{Mutex, MutexGuard, OnceLock};\n\n","")
cur_tk=sub1(cur_tk,"use semio_framework_ui_runtime_presentation::{ComponentTree, TreeNode};\n","use semio_framework_ui_runtime_presentation::{ComponentTree, TreeNode};\npub use semio_framework_ui_runtime_reconcile_primitives::testkit::{RegistryGuard, registry_guard};\n")
wr("⏭️cursor/🧰️testkit/🦀️.rs",cur_tk)
# ---- primitives root: testkit mount, test imports, suites
p=rd("🧱️primitives/🦀️.rs")
p=sub1(p,'#[path = "../♻️retirement/🌲️tree/🦀️.rs"]\nmod tree_retirement;\n',f'#[path = "../♻️retirement/🌲️tree/🦀️.rs"]\nmod tree_retirement;\n{CFG}\n#[path = "🧰️testkit/🦀️.rs"]\npub mod testkit;\n')
p=p.rstrip("\n")+'''

#[cfg(test)]
use semio_framework_ui_runtime_presentation::{ComponentTree, TreeNode};

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️primitives-unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "../♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs"]
mod tree_retirement_tests;
//#endregion 🧪️Tests
'''
wr("🧱️primitives/🦀️.rs",p)
for rel in ["🧪️tests/🔬️primitives-unit/🦀️.rs","♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs"]:
    t=rd(rel); t=t.replace("crate::surface_reconcile_registry_test_guard()","testkit::registry_guard()"); wr(rel,t)
# the census cursor's depth is read by the primitives suite at the crate root
c=rd("🧱️primitives/📊️census/🦀️.rs")
c=sub1(c,"    depth: usize,\n","    pub(crate) depth: usize,\n")
wr("🧱️primitives/📊️census/🦀️.rs",c)
# ---- cursor root: suite mount
c=rd("⏭️cursor/🦀️.rs")
c=c.rstrip("\n")+'''

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️cursor-unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
'''
wr("⏭️cursor/🦀️.rs",c)
# ---- authority root: suite mount
u=rd("♻️reconcile/🦀️.rs")
u=u.rstrip("\n")+'''

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️authority-unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
'''
wr("♻️reconcile/🦀️.rs",u)
# ---- orphan monolith suite and the abandoned laws crate scaffold
os.remove(f"{R}/🧪️tests/🔬️reconcile-unit/🦀️.rs"); os.rmdir(f"{R}/🧪️tests/🔬️reconcile-unit")
shutil.rmtree(f"{R}/⚖️laws")
print("ok")
