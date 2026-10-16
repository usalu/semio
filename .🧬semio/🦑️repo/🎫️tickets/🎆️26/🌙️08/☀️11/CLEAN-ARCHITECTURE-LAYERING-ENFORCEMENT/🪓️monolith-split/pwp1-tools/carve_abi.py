#!/usr/bin/env python3
# 🌉️ Carve of `abi`: root `🚪️io/🦀️.rs` + authority + sqlite + wasip2 bindings + component_persistent_local (owned_abi stays in the facade root: its `deallocate` calls reactor-context).
from carvelib import *

P2 = '#[cfg(all(target_arch = "wasm32", target_env = "p2"))]'
R = PLUG + "/🚪️io/🦀️.rs"

rootrs = read(ROOTRS)
a = "//#region 🔖️ComponentPersistentLocal\n"
b = "//#endregion 🔖️ComponentPersistentLocal\n"
assert rootrs.count(a) == 1 and rootrs.count(b) == 1
i, j = rootrs.index(a), rootrs.index(b) + len(b)
region = rootrs[i:j]
new_file = PLUG + "/🧬️schema/🔁️persistent-local/🦀️.rs"
assert not os.path.exists(new_file)
write(new_file, "//! 🔖️ Component-instance persistent locals: the wasip2 linear-memory global holder and the `component_persistent_local!` macro, whose `$crate` paths stay inside this crate.\n\n" + region)
write(ROOTRS, rootrs[:i] + "#[doc(hidden)]\npub use semio_framework_plugin_abi::component_persistent_local;\n" + rootrs[j:])

t = read(R)
assert t.startswith("//! 🚪️ Artifact io capabilities")
head_end = t.index("\n") + 1
imports = f"extern crate semio_framework_os_kernel as protocol;\nextern crate semio_framework_os_kernel as store;\nuse semio_framework_os_kernel::io;\n{P2}\nuse semio_framework::Fault;\n"
mounts = f'''
#[path = "🛂️authority/🦀️.rs"]
pub mod operation_authority;
#[path = "../🧬️schema/🪶️sqlite/🦀️.rs"]
pub mod sqlite_wire;
{P2}
#[path = "../🧬️schema/🦀️.rs"]
pub mod wasip2;
{P2}
#[doc(hidden)]
pub mod component {{
    pub use super::wasip2;
}}
#[path = "../🧬️schema/🔁️persistent-local/🦀️.rs"]
mod persistent_local;
{P2}
#[doc(hidden)]
pub use persistent_local::component_persistent_local;
'''
write(R, t[:head_end] + imports + t[head_end:].rstrip("\n") + "\n" + mounts)

W = PLUG + "/🧬️schema/🦀️.rs"
edit(W, 'path: "../../🧬️schema",', 'path: "../../../🧬️schema",')
