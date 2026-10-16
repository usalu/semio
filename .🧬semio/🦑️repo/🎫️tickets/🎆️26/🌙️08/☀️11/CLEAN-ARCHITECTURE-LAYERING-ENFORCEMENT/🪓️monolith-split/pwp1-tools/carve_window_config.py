#!/usr/bin/env python3
# 🎚️ Carve of `window-config`: root `🪟️window/🎚️config/🦀️.rs` + the erased mutation payload trait and refresh transfer moved in from `window_mutation` (intra-package move, plan 5.2).
import subprocess
from carvelib import *
import wp1files

OLD = PLUG + "/🪟️window/📬️mutation/🦀️.rs"
old = read(OLD)
a = old.index("use semio_framework_value::{FactoryBoxedValue")
b_marker = "impl RetireOwned for Box<dyn ErasedWindowMutationValue>{"
b = old.index(b_marker)
b_end = old.index("\n}\n", b) + 3
payload = old[a:b_end]
tail_marker = '#[path="🔄️refresh/🦀️.rs"]\npub(crate) mod refresh;\npub use refresh::WindowAuthorityRefreshStep;\nuse refresh::{WindowRefreshSnapshot,refresh_demands,refresh_transfer};\n'
assert old.count(tail_marker) == 1
payload = payload.replace("pub(crate) trait ErasedWindowMutationValue", "pub trait ErasedWindowMutationValue")
new_file = PLUG + "/🪟️window/🎚️config/📬️mutation/🦀️.rs"
assert not os.path.exists(new_file)
write(new_file, "//! 📬️ Original window mutation payloads carry their captured typed retirement issuer; the displaced-snapshot refresh transfer lives beside them.\n" + payload + '\n#[path = "../../📬️mutation/🔄️refresh/🦀️.rs"]\npub mod refresh;\npub use refresh::WindowAuthorityRefreshStep;\n')
rest = old[:a] + old[b_end:]
rest = rest.replace(tail_marker, "pub use semio_framework_plugin_window_config::window_mutation::{refresh, ErasedWindowMutationValue, WindowAuthorityRefreshStep};\n#[cfg(test)]\nuse semio_framework_plugin_window_config::window_mutation::refresh::{refresh_demands, refresh_transfer, WindowRefreshSnapshot};\n")
write(OLD, rest)

subprocess.check_call([sys.executable, TOOLS + "/rewrite_paths.py", "window-config", "--apply"], stdout=subprocess.DEVNULL)

R = PLUG + "/🪟️window/🎚️config/🦀️.rs"
t = read(R)
assert t.count('#[path = "🗂️registry/🦀️.rs"]\npub(crate) mod registry;') == 1
t = t.replace('#[path = "🗂️registry/🦀️.rs"]\npub(crate) mod registry;', '#[path = "🗂️registry/🦀️.rs"]\npub mod registry;\n#[path = "📬️mutation/🦀️.rs"]\npub mod window_mutation;')
write(R, t)
files = [f for f, _ in wp1files.tree(R)]
ensure_externs(R, aliases_used(files))
hub_edit('''#[path = "../../🪟️window/🎚️config/🦀️.rs"]
pub(crate) mod window_config;
''', '''pub(crate) extern crate semio_framework_plugin_window_config as window_config;
''')
set_lib_path("🪟️window/🎚️config")
remove_stub("🪟️window/🎚️config")
mark_carved("window-config")
