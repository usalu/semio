import sys,os,re
sys.path.insert(0,os.path.dirname(__file__))
from lib import *
A=F+"/🔨️modules/⏳️async"
TW=A+"/⏰️timer-wheel";SY=A+"/🚦️sync-primitives";SL=A+"/🧱️boxed-slots";TK=A+"/🎭️testkit"
def unlink(path,names):
    s=get(path)
    for n in names:s=s.replace("[`%s`]"%n,"`%s`"%n)
    texts[path]=s
# timer
p=TW+"/🦀️.rs"
unlink(p,["WorkerPool","Lane::Timer","WorkerPool::pump","Waker"]);
s=get(p)
s=s.replace("pub(super) const TIMER_ACTIONS_PER_POOL_TURN","pub const TIMER_ACTIONS_PER_POOL_TURN")
s=s.replace("    #[cfg(not(target_arch = \"wasm32\"))]\n    pub(super) fn install_earlier_deadline_hook","    #[cfg(not(target_arch = \"wasm32\"))]\n    #[doc(hidden)]\n    pub fn install_earlier_deadline_hook")
s=s.replace("pub(super)","pub(crate)")
texts[p]=s
# sync (oneshot mounted under the sync-primitives root)
p=SY+"/🦀️.rs"
unlink(p,["WorkerPool","ComputePool","TimerWheel"])
s=get(p).replace("pub(super)","pub(crate)")
head="//! 🚦️ `Notify` and the owned-permit `Semaphore`: the repo-owned wait primitives.\n"
assert s.startswith(head)
s=head+'''
//#region 🪢️Oneshot
#[path = "../🪢️oneshot/🦀️.rs"]
pub mod oneshot;
//#endregion 🪢️Oneshot
'''+s[len(head):]
texts[SY+"/🦀️.rs"]=s
# slots
p=SL+"/🦀️.rs"
unlink(p,["WorkerPool"])
s=get(p)
head="//! 🧱️ Boxed fixed-slot constructors, `FixedSlotTableBudget` and the bounded-stack runner.\n"
assert s.startswith(head)
s=head+'''
//#region 🗺️ShardedMap
#[path = "../🗺️sharded-map/🦀️.rs"]
mod sharded_map;
pub use sharded_map::ShardedMap;
//#endregion 🗺️ShardedMap

//#region 🎰️FixedSlotPages
#[path = "../🎰️slots/📄️pages/🦀️.rs"]
mod original_fixed_slot_pages;
pub use original_fixed_slot_pages::OriginalFixedSlotPages;
//#endregion 🎰️FixedSlotPages
'''+s[len(head):]
texts[p]=s
# testkit
p=TK+"/🦀️.rs"
sub(p,"use super::*;\nuse std::collections::HashMap;\nuse std::sync::atomic::{AtomicBool, AtomicU64};\nuse std::sync::{Condvar, Mutex, Weak};\n",
 "use semio_framework_async::{CancelToken, HostAsyncRuntime, HostFuture, OperationContext, ScopeDrainReport, ScopeHandle, ScopeId, ScopeOwner};\nuse std::collections::HashMap;\nuse std::future::Future;\nuse std::pin::Pin;\nuse std::sync::atomic::{AtomicBool, AtomicU64, Ordering};\nuse std::sync::{Arc, Condvar, Mutex, Weak};\n")
# async root
p=A+"/🦀️.rs"
sub(p,'''#[cfg(feature = "testkit")]
use std::future::Future;
#[cfg(feature = "testkit")]
use std::pin::Pin;
''','')
sub(p,'''#[path = "⏰️timer-wheel/🦀️.rs"]
mod timer_wheel;
use timer_wheel::TIMER_ACTIONS_PER_POOL_TURN;
pub use timer_wheel::{TimerSleep, TimerWheel};
''','''use semio_framework_async_timer::{TIMER_ACTIONS_PER_POOL_TURN, TimerWheel};
''')
subre(p,r'//#region 🪢️Oneshot\n.*?//#endregion 🪢️Oneshot\n\n#\[path = "🚦️sync-primitives/🦀️.rs"\]\nmod sync_primitives;\npub use sync_primitives::\{[^}]*\};\n\n','',flags=re.S)
sub(p,'''#[path = "🗺️sharded-map/🦀️.rs"]
mod sharded_map;
pub use sharded_map::ShardedMap;

#[path = "🧱️boxed-slots/🦀️.rs"]
mod boxed_slots;
pub use boxed_slots::{FixedSlotTableBudget, boxed_fixed_slots, boxed_slots};
#[cfg(not(target_arch = "wasm32"))]
pub use boxed_slots::{assert_fixed_slot_tables, on_bounded_stack};

''','')
subre(p,r'//#region 🧪️ManualRuntime\n.*?//#endregion 🧪️ManualRuntime\n\n','',flags=re.S)
sub(p,'''#[path="🎰️slots/📄️pages/🦀️.rs"]
mod original_fixed_slot_pages;
pub use original_fixed_slot_pages::OriginalFixedSlotPages;

''','')
# package glue + manifests
async_pkg=A+"/📦️packages/🦀️rust"
d=lambda n:f'{n} = {{ workspace = true }}'
NEW=[("semio-framework-async-timer","async-timer",TW),("semio-framework-async-sync","async-sync",SY),("semio-framework-async-slots","async-slots",SL),("semio-framework-async-testkit","async-testkit",TK)]
newcrate(TW,"semio-framework-async-timer","async-timer","Deadline heap and sleep future the worker pool fires timers from","../../🦀️.rs",[],workspace=F)
newcrate(SY,"semio-framework-async-sync","async-sync","Runtime-independent wait primitives: Notify, owned-permit Semaphore and the oneshot channel","../../🦀️.rs",[],workspace=F)
newcrate(SL,"semio-framework-async-slots","async-slots","Boxed fixed-slot tables, original slot pages and the sharded concurrent map","../../🦀️.rs",[d("semio-framework-value")],workspace=F)
newcrate(TK,"semio-framework-async-testkit","async-testkit","Injected-clock HostAsyncRuntime test double that needs no executor","../../🦀️.rs",[d("semio-framework-async")],workspace=F)
# async manifest: depend on timer
AM=async_pkg+"/Cargo.toml"
sub(AM,"semio-framework-trace = { workspace = true }\n","semio-framework-trace = { workspace = true }\nsemio-framework-async-timer = { workspace = true }\n")
sub(AM,"# 🧪️ Turns on the `testkit::ManualRuntime` `HostAsyncRuntime` test double for downstream crates\n# (packets R2/R4) to unit-test against without linking tokio, the `schema_metadata` projection and\n# the `internals` hooks the companion crate `semio-framework-async-tests` drives.\n","# 🧪️ Turns on the `schema_metadata` projection and the `internals` hooks the companion crate\n# `semio-framework-async-tests` drives; the `ManualRuntime` double lives in `semio-framework-async-testkit`.\n")
# registries
rel=lambda p:os.path.relpath(p,R)
for name,id_,dirn in NEW:
    insert_sorted_member(F+"/Cargo.toml",os.path.relpath(dirn+"/📦️packages/🦀️rust",F))
    add_build_owner(id_ if False else id_,rel(dirn+"/📦️packages/🦀️rust"))
    pkgrel=rel(dirn+"/📦️packages/🦀️rust")
    add_ws_dep_table(F+"/Cargo.toml","semio-framework-async-macros",name,os.path.relpath(dirn+"/📦️packages/🦀️rust",F))
    add_ws_dep_inline(R+"/Cargo.toml","semio-framework-async-macros",name,pkgrel)
    add_ws_dep_inline(R+"/🌎️hub/Cargo.toml","semio-framework-async-macros",name,"../"+pkgrel)
# companion tests
CP=A+"/🧪️tests/🥣️companion"
sub(CP+"/🦀️.rs","pub use semio_framework_async::*;\n","pub use semio_framework_async::*;\npub use semio_framework_async_slots::*;\npub use semio_framework_async_sync::*;\npub use semio_framework_async_timer::*;\n")
sub(A+"/🧪️tests/🔬️unit/🦀️.rs","    use testkit::ManualRuntime;\n","    use semio_framework_async_testkit::ManualRuntime;\n",5)
CM=CP+"/📦️packages/🦀️rust/Cargo.toml"
sub(CM,'semio-framework-test-fixtures =','semio-framework-async-slots = { path = "../../../../🧱️boxed-slots/📦️packages/🦀️rust" }\nsemio-framework-async-sync = { path = "../../../../🚦️sync-primitives/📦️packages/🦀️rust" }\nsemio-framework-async-testkit = { path = "../../../../🎭️testkit/📦️packages/🦀️rust" }\nsemio-framework-async-timer = { path = "../../../../⏰️timer-wheel/📦️packages/🦀️rust" }\nsemio-framework-test-fixtures =')
flush()
