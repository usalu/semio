"""Add kit.json to metabolism.zip from metabolism.kit.compose.json."""
import json
import shutil
import tempfile
import zipfile
from pathlib import Path

repo = Path(__file__).resolve().parents[6]
fixtures = repo / "compose" / "assets" / "fixtures"
kit_json_src = fixtures / "metabolism.kit.compose.json"
zip_path = fixtures / "metabolism.zip"
normalized = json.loads(kit_json_src.read_text(encoding="utf-8"))
wip = normalized.get("wip", {}).get("initialKit", normalized)
# unwrap hash/items one level for top-level collections
def unwrap(node):
    if isinstance(node, dict):
        changed = False
        for k, v in list(node.items()):
            if isinstance(v, dict) and "hash" in v and "items" in v:
                node[k] = v["items"]
                changed = True
            else:
                changed |= unwrap(v)
        return changed
    if isinstance(node, list):
        return any(unwrap(x) for x in node)
    return False
while unwrap(wip):
    pass

def rename_updated_at(node):
    if isinstance(node, dict):
        if "updatedAt" in node and "modificationdAt" not in node:
            node["modificationdAt"] = node.pop("updatedAt")
        for v in node.values():
            rename_updated_at(v)
    elif isinstance(node, list):
        for x in node:
            rename_updated_at(x)

rename_updated_at(wip)

def wire_design_parents(designs):
    variants_names = {"Slanted", "Twisted", "Dancing"}
    nakagin = None
    variants = []
    flats = []
    for d in designs:
        name = d.get("name")
        if not name:
            continue
        if name == "Nakagin Capsule Tower":
            nakagin = d
        elif name in variants_names:
            variants.append(d)
        elif name == "Flat":
            flats.append(d)
    if not nakagin or not nakagin.get("id"):
        return
    nakagin_id = nakagin["id"]
    for v in variants:
        if not v.get("parent"):
            v["parent"] = {"id": nakagin_id}
    for i, flat in enumerate(flats):
        if flat.get("parent"):
            continue
        parent_id = nakagin_id if i == 0 else (variants[i - 1]["id"] if i - 1 < len(variants) else None)
        if parent_id:
            flat["parent"] = {"id": parent_id}
    orphan_roots = [
        d for d in designs
        if not d.get("parent") and d.get("name") not in (None, "Nakagin Capsule Tower", "Flat")
        and d.get("name") not in variants_names
    ]
    for flat in flats:
        if flat.get("parent"):
            continue
        if not orphan_roots:
            break
        flat["parent"] = {"id": orphan_roots.pop(0)["id"]}

if isinstance(wip.get("designs"), list):
    wire_design_parents(wip["designs"])

kit_payload = json.dumps(wip, separators=(",", ":"))

with tempfile.TemporaryDirectory() as td:
    td_path = Path(td)
    with zipfile.ZipFile(zip_path, "r") as zin:
        zin.extractall(td_path)
    (td_path / "kit.json").write_text(kit_payload, encoding="utf-8")
    backup = zip_path.with_suffix(".zip.bak")
    shutil.copy2(zip_path, backup)
    with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zout:
        for f in sorted(td_path.rglob("*")):
            if f.is_file():
                zout.write(f, f.relative_to(td_path).as_posix())
print("patched", zip_path, "kit.json bytes", len(kit_payload))
