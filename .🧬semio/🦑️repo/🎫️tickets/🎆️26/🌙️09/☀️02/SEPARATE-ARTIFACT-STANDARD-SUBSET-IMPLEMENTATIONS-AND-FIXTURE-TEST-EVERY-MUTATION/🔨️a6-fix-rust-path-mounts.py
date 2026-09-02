#!/usr/bin/env python3
"""🩹️ Repair the 120 `#[path]` mounts in the shared stdio plugin crate that pointed at each glTF
2.0 mutation's `.rs` leaf under the old `✳️any` location, now that shard A6 physically relocated
those leaves to their real domain subset. The Rust module TREE (`subsets::any::schema::mutations::
<name>`) is deliberately left unrenamed -- `#[path]` exists precisely to decouple a module's name
from its file location, and renaming the tree too would ripple into every downstream reference
(dispatch tables, the `engine::mutations` barrel) for zero behavioural gain; see the shard report."""
import json, re

REPO = "/Users/ueli/Documents/semio"
TICKET = f"{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
CRATE_FILE = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs"

with open(f"{TICKET}/🗑️generated/a6-gltf-subset-mapping.json", encoding="utf-8") as f:
    mapping = json.load(f)

with open(CRATE_FILE, encoding="utf-8") as f:
    text = f.read()

replacements = 0
for dirname, info in mapping.items():
    old = f"../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{dirname}/🦀️.rs"
    new = f"../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️{info['subset']}/🧬️schema/🧬️mutations/{dirname}/🦀️.rs"
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected exactly 1 occurrence of {old!r}, found {count}")
    text = text.replace(old, new)
    replacements += 1

with open(CRATE_FILE, "w", encoding="utf-8") as f:
    f.write(text)

print("replaced", replacements, "path mounts")
