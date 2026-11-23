# 🏷️ Reports mutation payload fields that have neither a bilingual x-semio-ui label nor a glossary entry.
import json
from pathlib import Path

ROOT = Path("/home/deck/semio")
GLOSSARY = json.loads((ROOT / "🧰️framework/🔨️modules/🛂️manifest/🔣️input-labels.json").read_text())["labels"]
PLUGIN = ROOT / "✏️s/🔌️plugins/🎥️shooting"
missing = []
scanned = 0
for path in PLUGIN.rglob("*.json"):
    parts = set(path.parts)
    if "fixtures" in parts or "tests" in str(path):
        continue
    if "mutations" not in str(path):
        continue
    try:
        document = json.loads(path.read_text())
    except json.JSONDecodeError:
        continue
    if "properties" not in document:
        continue
    scanned += 1

    def walk(node, pointer):
        if not isinstance(node, dict):
            return
        properties = node.get("properties")
        if isinstance(properties, dict):
            for key, spec in properties.items():
                if not isinstance(spec, dict) or "const" in spec:
                    continue
                ui = spec.get("x-semio-ui") if isinstance(spec.get("x-semio-ui"), dict) else {}
                label = ui.get("label")
                bilingual = isinstance(label, dict) and {"en", "de"} <= set(label)
                if not bilingual and key not in GLOSSARY:
                    missing.append(f"{path.relative_to(ROOT)} {pointer}/{key}")
                walk(spec, f"{pointer}/{key}")
        for key, value in node.items():
            if key == "properties":
                continue
            if isinstance(value, dict):
                walk(value, f"{pointer}/{key}")

    def resolve(node):
        if not isinstance(node, dict):
            return node
        ref = node.get("$ref")
        if isinstance(ref, str) and ref.startswith("#/$defs/"):
            target = document.get("$defs", {}).get(ref.split("/")[-1])
            if isinstance(target, dict):
                return target
        if isinstance(ref, str) and "#/$defs/" in ref and "artifact.json" in ref:
            artifact = json.loads((PLUGIN / "🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json").read_text())
            target = artifact.get("$defs", {}).get(ref.split("/")[-1])
            if isinstance(target, dict):
                merged = dict(target)
                ui = node.get("x-semio-ui")
                if ui:
                    merged["x-semio-ui"] = ui
                return merged
        return node

    def walk(node, pointer):
        node = resolve(node)
        if not isinstance(node, dict):
            return
        properties = node.get("properties")
        if isinstance(properties, dict):
            for key, spec in properties.items():
                if not isinstance(spec, dict) or "const" in spec:
                    continue
                spec = resolve(spec)
                ui = spec.get("x-semio-ui") if isinstance(spec.get("x-semio-ui"), dict) else {}
                label = ui.get("label")
                bilingual = isinstance(label, dict) and {"en", "de"} <= set(label)
                if not bilingual and key not in GLOSSARY:
                    missing.append(f"{path.relative_to(ROOT)} {pointer}/{key}")
                walk(spec, f"{pointer}/{key}")

    walk(document, "")
print(f"scanned={scanned} glossary={len(GLOSSARY)} missing={len(missing)}")
for row in missing:
    print(row)
