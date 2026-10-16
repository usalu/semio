"""🧰️ Stage D4: writes the cursor testkit module (shared fixtures and the ledger guard) from the unit-test fixtures."""
import re
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
R=f"{REPO}/{RT}"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt/orig"
src=open(f"{S}/{RT}/🧪️tests/🔬️reconcile-unit/🦀️.rs",encoding="utf-8").read()
a=src.index("fn ui_text(")
b=src.index("fn reconcile_resumable(")
fx=src[a:b].rstrip("\n")
fx=fx.replace("crate::TreeNode","TreeNode").replace("crate::ComponentTree","ComponentTree")
fx=re.sub(r"^fn ","pub fn ",fx,flags=re.M)
guard='''
pub struct RegistryGuard {
    _lock: MutexGuard<'static, ()>,
}

pub fn registry_guard() -> RegistryGuard {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    let lock = GUARD.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    drain_resident_registries_until_idle();
    RegistryGuard { _lock: lock }
}

impl Drop for RegistryGuard {
    fn drop(&mut self) {
        drain_resident_registries_until_idle();
    }
}

fn drain_resident_registries_until_idle() {
    for _ in 0..(64 * 4 + ui_contract::UI_RESIDENT_SLOTS * 6 * 8) {
        let resident_pending = ui_contract::UiResidentPermit::has_pending_returns();
        if resident_pending {
            let _ = ui_contract::UiResidentPermit::drain_one();
        }
        let value_idle = ui_contract::close_ui_value_page_one();
        let built_idle = ui_contract::close_built_node_page_one();
        if !resident_pending && value_idle && built_idle {
            break;
        }
    }
}
'''
out='''//! 🧰️ Shared fixtures of the reconcile suites — bounded tree builders, snapshot oracles and the process-wide resident-ledger guard that serialises the laws admitting into the shared ledgers. Exposed under the `testkit` feature so the authority crate's suites build their trees the same way.

use std::sync::{Mutex, MutexGuard, OnceLock};

use semio_framework_ui_runtime_presentation::{ComponentTree, TreeNode};

//#region 🔖️Fixtures
'''+fx+'''
//#endregion 🔖️Fixtures
'''+guard
import os
os.makedirs(f"{R}/⏭️cursor/🧰️testkit",exist_ok=True)
open(f"{R}/⏭️cursor/🧰️testkit/🦀️.rs","w",encoding="utf-8").write(out)
print(len(out.split("\n")),"lines")
