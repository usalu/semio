#!/usr/bin/env python3
# 🕹️ Carve of `interaction`: root `🕹️interaction/🦀️.rs` (local interaction components) + topology + the UiAssemblyResult / ui_assembly_error pair lifted out of `window_surface`.
from carvelib import *
import wp1files

R = PLUG + "/🕹️interaction/🦀️.rs"
t = read(R)
for name in ("authority", "live", "query", "retirement", "mutations"):
    old = f"#[path = \"{ {'authority':'🔐️authority','live':'📡️live','query':'📃️query','retirement':'♻️retirement','mutations':'🧬️mutations'}[name] }/🦀️.rs\"]\npub(crate) mod {name};"
    assert t.count(old) == 1, name
    t = t.replace(old, old.replace("pub(crate) mod", "pub mod"))
assert t.count("mod capture;") == 1
t = t.rstrip("\n") + '''

#[path = "🧭️topology/🦀️.rs"]
pub mod interaction_topology;

/// 🧩 Fallible fixed-capacity UI assembly result shared by plugin render boundaries.
pub type UiAssemblyResult<T> = Result<T, PluginAssemblyError>;

pub fn ui_assembly_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("fixed UI admission failed at {stage}"))
}
'''
t = t.replace("//! 🕹️ Exact local interaction ownership, query, and publication components.\n", "//! 🕹️ Exact local interaction ownership, query, and publication components.\nuse semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;\n#[cfg(test)]\nuse semio_framework_os_kernel::io;\n", 1)
write(R, t)
files = [f for f, _ in wp1files.tree(R)]
al = aliases_used(files)
print("aliases", al)
ensure_externs(R, al)

W = PLUG + "/🪟️window/🦀️.rs"
edit(W, '''/// 🧩 Fallible fixed-capacity UI assembly result shared by plugin render boundaries.
pub type UiAssemblyResult<T> = Result<T, PluginAssemblyError>;

pub(crate) fn ui_assembly_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("fixed UI admission failed at {stage}"))
}
''', '''pub use semio_framework_plugin_interaction::{ui_assembly_error, UiAssemblyResult};
''')
hub_edit('''#[path = "../../🕹️interaction/🧭️topology/🦀️.rs"]
pub(crate) mod interaction_topology;
#[path = "../../🕹️interaction/🦀️.rs"]
pub(crate) mod local_interaction;
''', '''pub(crate) use semio_framework_plugin_interaction::interaction_topology;
pub(crate) extern crate semio_framework_plugin_interaction as local_interaction;
''')
set_lib_path("🕹️interaction")
remove_stub("🕹️interaction")
mark_carved("interaction")
