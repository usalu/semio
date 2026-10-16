#!/usr/bin/env python3
# 🏛️ Step 1 of the host-base carve: split the renderer root into the host-base crate root text and the slimmed root (dry run writes into the scratchpad).
import re, sys, os
from common import *
from rs import blank

ROOT = R + "/🦀️.rs"
text = read(ROOT)
lines = text.split("\n")


def find(anchor, start=0):
    idx = [i for i, l in enumerate(lines) if l.startswith(anchor)]
    idx = [i for i in idx if i >= start]
    assert len(idx) >= 1, anchor
    return idx[0]


region_start = find("//#region 🔖️ActorBudgetBridge")
boot_end = find("//#endregion 🧭️BootDescriptor")
plugin_wrapper = find('#[cfg(target_arch="wasm32")]', boot_end)
assert "semioWgpuSetPluginCatalog" in lines[plugin_wrapper + 1]
moved = lines[region_start:boot_end + 1]
# wrappers that stay in the root
wrap_re = re.compile(r'((?:///[^\n]*\n)*)#\[cfg\(target_arch = "wasm32"\)\]\n#\[wasm_bindgen\(js_name = (\w+)\)\]\n(?:pub )?fn [^\n]*\{\n(?:[^\n]*\n)*?\}\n')
moved_text = "\n".join(moved) + "\n"
wrappers = []
def take(m):
    wrappers.append((m.group(2), m.group(0)))
    return ""
# the boot-hook doc fragment sits above the diagnostics wrapper: take the whole 644..662 block as one unit
diag_i = moved_text.index("/// 🌐️ wasm boot hook")
boot_fn_end = moved_text.index("apply_boot_descriptor(descriptor).map_err(|error| JsValue::from_str(&error))\n}\n", diag_i) + len("apply_boot_descriptor(descriptor).map_err(|error| JsValue::from_str(&error))\n}\n")
boot_block = moved_text[diag_i:boot_fn_end]
moved_text = moved_text[:diag_i] + moved_text[boot_fn_end:]
cleaned = wrap_re.sub(take, moved_text)
print("wrappers", [w[0] for w in wrappers])
assert len(wrappers) == 4, [w[0] for w in wrappers]
os.makedirs(RS, exist_ok=True)
write(RS + "/hostbase-moved.rs", cleaned)
write(RS + "/hostbase-wrappers.rs", "".join(w[1] for w in wrappers) + boot_block)
print(len(cleaned.split("\n")), "lines moved")
