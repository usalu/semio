"""🖼️ Stage G: presentation keeps its producer suite (with a ledger guard of its own) and the present module re-mounts it."""
import os
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
R=f"{REPO}/{RT}"
p=f"{R}/🧪️tests/🔬️present-unit/🦀️.rs"
t=open(p,encoding="utf-8").read()
assert t.count("crate::surface_reconcile_registry_test_guard()")==4
t=t.replace("crate::surface_reconcile_registry_test_guard()","registry_guard()")
guard='''use super::*;
use std::sync::{Mutex, MutexGuard, OnceLock};

struct RegistryGuard {
    _lock: MutexGuard<'static, ()>,
}

fn registry_guard() -> RegistryGuard {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    let lock = GUARD.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    drain_page_registries_until_idle();
    RegistryGuard { _lock: lock }
}

impl Drop for RegistryGuard {
    fn drop(&mut self) {
        drain_page_registries_until_idle();
    }
}

fn drain_page_registries_until_idle() {
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
t=t.lstrip("\n"); assert t.startswith("use super::*;\n")
t=guard+t[len("use super::*;\n"):]
open(p,"w",encoding="utf-8").write(t)
# present module: restore the suite mount (the guard no longer needs the reconcile registry)
p=f"{R}/🎭️present/🦀️.rs"
t=open(p,encoding="utf-8").read()
t=t.rstrip("\n")+'\n\n//#region 🧪️Tests\n#[cfg(test)]\n#[path = "../🧪️tests/🔬️present-unit/🦀️.rs"]\nmod tests;\n//#endregion 🧪️Tests\n'
open(p,"w",encoding="utf-8").write(t)
print("ok")
