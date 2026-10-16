"""📦️ Stage B2: exchange and presentation roots, dispatch retarget, present test mount moved to the laws."""
import re
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
R=f"{REPO}/{RT}"
def w(p,t): open(p,"w",encoding="utf-8").write(t)
# presentation manifest cleanup
p=f"{R}/🖼️presentation/📦️packages/🦀️rust/Cargo.toml"
t=open(p,encoding="utf-8").read()
old=t[t.index("# 🧪️ The fixture-oracle"):t.index('serde_json = {')]
t=t.replace(old,"")
open(p,"w",encoding="utf-8").write(t)
# exchange root
w(f"{R}/📡️exchange/🦀️.rs",'''//! 📡️ The headless runtime exchange — the bounded command gateway, the coalescing projection inbox and the presence hub through which a surface talks to the actor world and to its peers. Pure bounded queues and tables: no entity, no presenter, no reconciler.

#[path = "../📮️gateway/🦀️.rs"]
mod gateway;
#[path = "../📥️inbox/🦀️.rs"]
mod inbox;
#[path = "../👥️presence/🦀️.rs"]
mod presence;

pub use gateway::*;
pub use inbox::*;
pub use presence::*;
''')
# presentation root
w(f"{R}/🖼️presentation/🦀️.rs",'''//! 🖼️ The headless runtime presentation — typed entities with mutation leasing, actual-read dependency tracking, revision-guarded intent routing, and the component tree and producer a presenter builds before anything is reconciled.
//!
//! ```text
//! projections → Entity<T> state → Present → ComponentTree
//! ```
//!
//! It renders nothing and knows about no renderer, so it runs on both sides of the plugin boundary and compiles for
//! `wasm32-wasip2` and `wasm32-unknown-unknown` by construction.

#[path = "../🎛️context/🦀️.rs"]
mod context;
#[path = "../🎯️dispatch/🦀️.rs"]
mod dispatch;
#[path = "../🪪️entity/🦀️.rs"]
mod entity;
#[path = "../🎭️present/🦀️.rs"]
mod present;
#[path = "../🕸️tracking/🦀️.rs"]
mod tracking;

pub use context::*;
pub use dispatch::*;
pub use entity::*;
pub use present::*;
pub use tracking::*;
''')
# dispatch retarget
p=f"{R}/🎯️dispatch/🦀️.rs"
t=open(p,encoding="utf-8").read()
assert t.count("Vec<crate::Command>")==1 and t.count("SubmitCommand(crate::Command)")==1
t=t.replace("Vec<crate::Command>","Vec<Command>").replace("SubmitCommand(crate::Command)","SubmitCommand(Command)")
anchor="//#region 🔖️Intent\n"
assert t.count(anchor)==1
t=t.replace(anchor,"use semio_framework_ui_runtime_exchange::Command;\n\n"+anchor)
w(p,t)
# present: drop the unit-test mount (the laws crate mounts it, it needs the reconcile registry guard)
p=f"{R}/🎭️present/🦀️.rs"
t=open(p,encoding="utf-8").read()
mount='//#region 🧪️Tests\n#[cfg(test)]\n#[path = "../🧪️tests/🔬️present-unit/🦀️.rs"]\nmod tests;\n//#endregion 🧪️Tests\n'
assert t.count(mount)==1
t=t.replace(mount,"")
w(p,t.rstrip("\n")+"\n")
# tree test file: relative super path of the handback imports now lives at the laws root
p=f"{R}/♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs"
t=open(p,encoding="utf-8").read()
assert t.count("use super::super::{")==1
t=t.replace("use super::super::{","use super::{")
w(p,t)
print("ok")
