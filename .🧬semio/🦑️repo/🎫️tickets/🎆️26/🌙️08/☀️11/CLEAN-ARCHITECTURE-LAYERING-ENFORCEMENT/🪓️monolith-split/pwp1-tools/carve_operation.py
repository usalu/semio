#!/usr/bin/env python3
# ♻️ Carve of `operation`: root `♻️cancellation/🦀️.rs` (tool cancellation, issuer, lease) + completion effects/vacant + original catalog constructor; `operation_progress` stays (blocked on the WP2 lift).
import subprocess
from carvelib import *
import wp1files, explode_glob, rewrite_paths

R = PLUG + "/♻️cancellation/🦀️.rs"
subprocess.check_call([sys.executable, TOOLS + "/rewrite_paths.py", "operation", "--apply"], stdout=subprocess.DEVNULL)

edit(PLUG + "/📬️completion/♻️effects/🦀️.rs", "use crate::Effect;", "use semio_framework::Effect;")

catalog = PLUG + "/🌱️initialization/🏪️catalog/🦀️.rs"
text, items, un = explode_glob.explode(read(catalog), read(PLUG + "/🗃️document/🦀️.rs"), "operation")
assert not items and not un, (items, un)
ct = read(catalog)
assert ct.count("use super::*;") == 1
write(catalog, ct.replace("use super::*;", text, 1))

t = read(R)
mounts = '''
#[path = "../📬️completion/♻️effects/🦀️.rs"]
pub mod completion_effects;
#[path = "../📬️completion/♻️vacant/🦀️.rs"]
pub mod completion_vacant;
#[path = "../🌱️initialization/🏪️catalog/🦀️.rs"]
pub mod original_catalog_constructor;
#[path = "🛬️lifted/👁️view/🦀️.rs"]
mod lifted_view;
pub use lifted_view::*;
#[path = "🛬️lifted/🧵️task/🦀️.rs"]
mod lifted_task;
pub use lifted_task::*;
'''
write(R, t.rstrip("\n") + "\n" + mounts)
files = [f for f, _ in wp1files.tree(R)]
ensure_externs(R, aliases_used(files))

hub_edit('''#[path = "../../♻️cancellation/🦀️.rs"]
pub(crate) mod tool_cancellation;
''', '''pub(crate) extern crate semio_framework_plugin_operation as tool_cancellation;
''')
hub_edit('''#[path="../../📬️completion/♻️effects/🦀️.rs"]
pub(crate) mod completion_effects;
''', '''pub(crate) use semio_framework_plugin_operation::completion_effects;
''')
hub_edit('''#[path="../../📬️completion/♻️vacant/🦀️.rs"]
pub(crate) mod completion_vacant;
''', '''pub(crate) use semio_framework_plugin_operation::completion_vacant;
''')
edit(PLUG + "/🗃️document/🦀️.rs", '''#[path="../🌱️initialization/🏪️catalog/🦀️.rs"]
mod original_catalog_constructor;
''', '''use semio_framework_plugin_operation::original_catalog_constructor;
''')
set_lib_path("♻️cancellation")
remove_stub("♻️cancellation")
mark_carved("operation")
