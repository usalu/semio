#!/usr/bin/env python3
"""🚚️ Physically relocate each glTF 2.0 mutation's own schema/mutations/<name> subtree and its
subset-level fixture pair to its smallest (domain) owner subset directory, and repair the
self-referential `owner` path recorded inside each moved mutation's own 🔣️.json."""
import json, shutil, os, re

REPO = "/Users/ueli/Documents/semio"
TICKET = f"{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
ANY = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any"
STANDARDS_ROOT = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets"

with open(f"{TICKET}/🗑️generated/a6-gltf-subset-mapping.json", encoding="utf-8") as f:
    mapping = json.load(f)

OLD_OWNER_PREFIX = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"

moved = []
for dirname, info in mapping.items():
    subset = info["subset"]
    src = f"{ANY}/🧬️schema/🧬️mutations/{dirname}"
    dst_dir = f"{STANDARDS_ROOT}/✳️{subset}/🧬️schema/🧬️mutations"
    dst = f"{dst_dir}/{dirname}"
    assert os.path.isdir(src), src
    os.makedirs(dst_dir, exist_ok=True)
    assert not os.path.exists(dst), dst
    shutil.move(src, dst)

    # repair the self-referential "owner" field inside the moved mutation's own 🔣️.json
    manifest_path = f"{dst}/🔣️.json"
    if os.path.isfile(manifest_path):
        with open(manifest_path, encoding="utf-8") as f:
            text = f.read()
        new_owner = f"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️{subset}/🧬️schema/🧬️mutations/{dirname}"
        old_owner = OLD_OWNER_PREFIX + dirname
        if old_owner in text:
            text = text.replace(old_owner, new_owner)
            with open(manifest_path, "w", encoding="utf-8") as f:
                f.write(text)
        else:
            print("WARN: no owner match in", manifest_path)

    # move the matching subset-level fixture pair
    fixture_name = f"{info['ascii']}-applied"
    fsrc = f"{ANY}/🧫️fixtures/{fixture_name}"
    fdst_dir = f"{STANDARDS_ROOT}/✳️{subset}/🧫️fixtures"
    fdst = f"{fdst_dir}/{fixture_name}"
    if os.path.isdir(fsrc):
        os.makedirs(fdst_dir, exist_ok=True)
        assert not os.path.exists(fdst), fdst
        shutil.move(fsrc, fdst)
    else:
        print("WARN: no fixture dir for", dirname, fsrc)

    moved.append({"dir": dirname, "subset": subset, "dst": dst})

print("moved", len(moved), "mutations")
with open(f"{TICKET}/🗑️generated/a6-gltf-moved.json", "w", encoding="utf-8") as f:
    json.dump(moved, f, ensure_ascii=False, indent=2)
