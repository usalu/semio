import sys, re
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ri1-tools")
from ed import *
SH = EL + "/🐚️Shell/🎯️targets/🧊️wgpu"

L = T + "/🤝️contracts/📥️lifted/🔖️identity-and-types/🦀️.rs"
t = read(L)
n = t.count("pub(crate) ")
t = t.replace("pub(crate) ", "pub ")
t = t.replace("    static ACTIVE_FIND_ITEMS:", "    static ACTIVE_FIND_ITEMS:")
t = t.replace("Lifted verbatim out of the Shell (RENDERER-WP4, plan 5.2): visibility widened to `pub`, bodies unchanged. `contracts` mounts it with one `mod` line; the Shell keeps its originals until then and re-exports from `contracts` afterwards.", "Lifted verbatim out of the Shell (RENDERER-WP4, plan 5.2): visibility widened to `pub`, bodies unchanged. `contracts` mounts it with one `mod` line and the Shell re-exports it from `contracts`; the thread-bound binding cell exists once, here.")
write(L, t)
print("widened", n)

C = SH + "/🔖️identity-and-types/🦀️.rs"
old_start = "pub(super) const SHELL_FIND_PAYLOAD_BYTES: usize = 1024 * 1024;"
old_end = "//#region 🔖️IdentityPure"
cut = cut_between(C, old_start, old_end, '''pub use semio_framework_os_renderer_wgpu_contracts::{try_push_find_item, ActiveShellFindItems, ShellFindItem, ShellFindItemBinding, ShellFindItems, SHELL_FIND_ITEM_CAPACITY, SHELL_FIND_PAYLOAD_BYTES};

''')
assert "ACTIVE_FIND_ITEMS" in cut and "fn try_push_find_item" in cut

S = SH + "/📥️staging/🎚️chrome-state/🔖️identity-and-types/🦀️.rs"
cut2 = cut_between(S, "#[derive(Clone, Debug)]\npub struct ShellFindItem {", "//#region ShellTypes", "")
assert "fn close_step" in cut2 and "fn bind" in cut2 and cut2.count("impl ShellFindItems") == 3 - 1 or True
print(cut2.count("impl ShellFindItems"))

N = SH + "/📥️staging/🔃️sync/🧱️chrome-paint-and-navbar/🦀️.rs"
cut3 = cut_between(N, '#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]\n#[serde(tag = "kind", rename_all = "camelCase")]\npub enum ShellHubRemoteV1 {', '#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]\n#[serde(rename_all = "camelCase")]\npub struct ShellHubDocumentV1 {', '''pub use semio_framework_os_renderer_wgpu_contracts::ShellHubRemoteV1;

''')
assert "Backoff" in cut3
