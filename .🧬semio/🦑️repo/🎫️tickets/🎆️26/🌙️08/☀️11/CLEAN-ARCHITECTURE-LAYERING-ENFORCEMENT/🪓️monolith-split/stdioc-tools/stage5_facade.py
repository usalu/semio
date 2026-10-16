#!/usr/bin/env python3
# 🧩 Stage 5: writes the facade lib root from the baseline public surface (every formerly public name becomes one explicit re-export).
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(__file__))
from lib import *  # noqa: E402,F401,F403

BASE_C = os.environ["SCRATCH"] + "/baseline/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract"


def old_names(relative):
    names, items = all_public_names(BASE_C + "/" + relative)
    return set(names), items


def split(names, crate_key):
    return sorted(set(names))


root_names, root_items = old_names("🦀️.rs")
editing_names, editing_items = old_names("✏️editing/🦀️.rs")
patch_pub, _ = old_names("✏️editing/🩹️patch/🦀️.rs")
details_pub, _ = old_names("✏️editing/🪟️details/🦀️.rs")

wire_new, wire_items = all_public_names(BY_KEY["wire"]["root"])
window_new, _ = all_public_names(BY_KEY["window-kit"]["root"])
assembly_new, assembly_items = all_public_names(BY_KEY["assembly"]["root"])
editing_new, editing_new_items = all_public_names(BY_KEY["editing"]["root"])
patch_new, _ = all_public_names(BY_KEY["editing-patch"]["root"])
details_new, _ = all_public_names(BY_KEY["editing-details"]["root"])

root_fn_names = {name for name in root_names if name not in {"pack", "kernel", "value", "diagnostic", "locale"}}
placed = {}
for key, names in (("wire", wire_new), ("window-kit", window_new), ("assembly", assembly_new)):
    for name in names:
        placed.setdefault(name, key)
missing = sorted(name for name in root_fn_names if name not in placed)
print("root names with no new home:", missing)
for forbidden in ("pack", "kernel", "value", "diagnostic", "locale"):
    root_fn_names.discard(forbidden)

old_editing_public = sorted(set(editing_names))
editing_public_in_new = set(editing_new)
core_exports = sorted(name for name in old_editing_public if name in editing_public_in_new and name not in {"bytes", "raster", "rules"})
lost = sorted(name for name in old_editing_public if name not in editing_public_in_new and name not in {"bytes", "raster", "rules"})
print("editing core names lost (expected: re-exports of patch/details):", lost)
old_patch_exports = sorted(name for name in lost if name in set(patch_new))
old_details_exports = sorted(name for name in lost if name in set(details_new))
old_wire_exports = sorted(name for name in lost if name in set(wire_new))
unplaced = sorted(set(lost) - set(old_patch_exports) - set(old_details_exports) - set(old_wire_exports))
assert not unplaced, unplaced

by_crate = {"wire": [], "window-kit": [], "assembly": []}
for name in sorted(root_fn_names):
    key = placed.get(name)
    if key:
        by_crate[key].append(name)


def group(crate_lib, names, alias=None, indent=0):
    names = sorted(names, key=lambda n: (n.upper() != n, n))
    return f"pub use {crate_lib}::{{{', '.join(names)}}};"


lines = []
lines.append("//! 🧩 Shared, catalog-independent contracts for one stdio artifact package: the facade over the producer crates carved from the former single crate.")
lines.append("")
lines.append("pub use pack;")
lines.append("pub use semio_framework_diagnostic as diagnostic;")
lines.append("pub use semio_framework_os_kernel as kernel;")
lines.append("pub use semio_framework_ui_locale as locale;")
lines.append("pub use semio_framework_value as value;")
lines.append("")
assembly_mods = ["definition_hierarchy", "registry"]
lines.append(group(BY_KEY["assembly"]["lib"], by_crate["assembly"] + assembly_mods))
lines.append(group(BY_KEY["window-kit"]["lib"], by_crate["window-kit"]))
lines.append(group(BY_KEY["wire"]["lib"], by_crate["wire"] + ["impl_serde_op_codec"]))
lines.append(f"pub use {BY_KEY['list-delta']['lib']} as list_delta;")
lines.append(f"pub use {BY_KEY['list-delta']['lib']}::stdio_list_delta;")
lines.append(f"pub use {BY_KEY['media-export']['lib']} as media_export;")
lines.append(f"pub use {BY_KEY['native-print']['lib']} as native_print;")
lines.append(f"pub use {BY_KEY['part21']['lib']} as part21;")
lines.append('#[cfg(feature = "testing")]')
lines.append(f"pub use {BY_KEY['testing']['lib']} as testing;")
macros = ["bounded_native_editing_editor", "snapshot_details_editor_support", "snapshot_editing_bounded_first_step_tool_proofs", "snapshot_editing_command_roster", "snapshot_editing_publication_authority"]
lines.append(group(BY_KEY["editing"]["lib"], macros))
lines.append("")
lines.append("pub mod editing {")
lines.append(f"    pub use {BY_KEY['editing']['lib']}::{{{', '.join(['bytes', 'raster', 'rules'] + core_exports)}}};")
lines.append(f"    pub use {BY_KEY['wire']['lib']}::{{{', '.join(old_wire_exports)}}};")
lines.append(f"    pub use {BY_KEY['editing-patch']['lib']} as patch;")
lines.append(f"    pub use {BY_KEY['editing-details']['lib']} as details;")
lines.append(f"    pub use {BY_KEY['editing-patch']['lib']}::{{{', '.join(old_patch_exports)}}};")
lines.append(f"    pub use {BY_KEY['editing-details']['lib']}::{{{', '.join(old_details_exports)}}};")
lines.append("}")
lines.append("")
text = "\n".join(lines)
write(C + "/🦀️.rs", text)
subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--config", "skip_children=true,unstable_features=true,max_width=250", C + "/🦀️.rs"], check=True)
print(read(C + "/🦀️.rs"))
