#!/usr/bin/env python3
# 🎭️ Carve of `actor-host`: root `🌐host/🦀️.rs` (host, body, requests) + reactor executor + backbone binding + the inbound `cancel_job` lift stub.
from carvelib import *

R = PLUG + "/🌐host/🦀️.rs"
t = read(R)
old = '''#[path = "📖️body/🦀️.rs"]
pub mod body;
#[path = "📮️requests/🦀️.rs"]
pub mod requests;
pub use body::BodyReader;
'''
new = '''#[path = "📖️body/🦀️.rs"]
pub mod body;
#[path = "📮️requests/🦀️.rs"]
pub mod requests;
pub use body::BodyReader;
#[path = "../⚛️reactor/🧵️executor/🦀️.rs"]
pub mod executor;
#[path = "../📡️backbone/🔗️binding/🦀️.rs"]
pub mod document_backbone_binding;
#[path = "🛬️lifted/💼️jobs/🦀️.rs"]
mod lifted_jobs;
pub use lifted_jobs::*;
'''
assert t.count(old) == 1
t = t.replace(old, new)
assert t.count("crate::component::wasip2::semio::framework::pure::") == 3
t = t.replace("crate::component::wasip2::semio::framework::pure::", "semio_framework_plugin_abi::wasip2::semio::framework::pure::")
write(R, t)
ensure_externs(R, ["store"])

hub_edit('''// 🧬️ A2 (design-abi.md §4): `host_port` is deleted''', '''// 🧬️ A2 (design-abi.md §4): `host_port` is deleted''')
hub_edit('''#[path = "../../🌐host/🦀️.rs"]
pub mod host;
''', '''pub extern crate semio_framework_plugin_actor_host as host;
''')
hub_edit('''#[path = "../../📡️backbone/🔗️binding/🦀️.rs"]
pub mod document_backbone_binding;
''', '''pub use semio_framework_plugin_actor_host::document_backbone_binding;
''')
RC = PLUG + "/⚛️reactor/🦀️.rs"
edit(RC, '''#[path = "🧵️executor/🦀️.rs"]
pub mod executor;
''', '''pub use semio_framework_plugin_actor_host::executor;
''')
set_lib_path("🌐host")
remove_stub("🌐host")
mark_carved("actor-host")
