import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from edit import edit, read
S = "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/"
mods = ["completion_sink", "compute_pool", "event_router", "fixed_file_storage", "host_runtime", "http_contract", "http_pool", "http_socket", "storage_scheduler", "timer_wheel"]
kit = "//#region 🧬️Tests\n/// 🪝️ Test hook of the companion crate `semio-framework-os-services-tests`: the private service modules, public only behind the `testkit` feature.\n#[cfg(feature = \"testkit\")]\n#[doc(hidden)]\npub mod testkit {\n" + "".join(f"    pub mod {m} {{\n        pub use super::super::{m}::*;\n    }}\n" for m in mods) + "}\n//#endregion 🧬️Tests"
edit(S + "🦀️.rs", "//#region 🧬️Tests\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️component-unit/🦀️.rs\"]\nmod tests;\n//#endregion 🧬️Tests", kit)
edit(S + "👀️file-watcher/🦀️.rs", "\n#[cfg(test)]\n#[path = \"../🧪️tests/🔬️file-watcher-unit/🦀️.rs\"]\nmod tests;\n", "")
edit(S + "🚪️native-io/🦀️.rs", "\n#[cfg(test)]\n#[path = \"../🧪️tests/🔬️native-io-unit/🦀️.rs\"]\nmod tests;\n", "")
glue = S + "📦️packages/🦀️rust/🦀️.rs"
t = read(glue)
old = "\n#[cfg(test)]\n#[global_allocator]\nstatic ORIGINAL_SERVICES_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;\n"
assert t.count(old) == 1, t[-400:]
new = "\n/// 🪝️ Test hook of the companion crate `semio-framework-os-services-tests`: the private host modules next to the owner's service hooks, public only behind the `testkit` feature.\n#[cfg(feature = \"testkit\")]\n#[doc(hidden)]\npub mod testkit {\n    pub use super::component::testkit::*;\n    pub mod file_watcher {\n        pub use crate::file_watcher::*;\n    }\n    pub mod native_io {\n        pub use crate::native_io::*;\n    }\n}\n"
edit(glue, old, new)
