"""Brings committed Draw `🔺️diff/🔣️.json` fixtures to the canonical DrawingLayerPatch lanes: image lanes present as null, font family only on text layers.
Every number in a patch is an f64 lane, so it is written as a float (`20.0`).
Usage: python3 -I canonicalize-diff-fixtures.py <diff fixture>..."""
import json, pathlib, sys

IMAGE = ("imageKey", "imageWidth", "imageHeight")


def floats(value, key=None):
    if isinstance(value, bool) or key == "index":
        return value
    if isinstance(value, int):
        return float(value)
    if isinstance(value, list):
        return [floats(item) for item in value]
    if isinstance(value, dict):
        return {name: floats(item, name) for name, item in value.items()}
    return value


def kinds(layers, out):
    for layer in layers:
        if not isinstance(layer, dict):
            continue
        out[layer["id"]] = layer["kind"]
        kinds(layer.get("children", []), out)
    return out


for name in sys.argv[1:]:
    path = pathlib.Path(name)
    before = path.parent.parent / "📸️snapshot" / "⬅️before" / "🔣️.json"
    known = kinds(json.loads(before.read_text())["layers"], {}) if before.exists() else {}
    diff = json.loads(path.read_text())
    changed = False
    for entry in (diff.get("layers") or {}).get("modified", []):
        patch = entry["patch"]
        if len(patch) < 3:
            continue
        for lane in IMAGE:
            if lane not in patch:
                patch[lane] = None
                changed = True
        if patch.get("fontFamily") == "anta" and known.get(entry["id"]) != "text":
            patch["fontFamily"] = None
            changed = True
        canonical = floats(patch)
        if canonical != patch or json.dumps(canonical) != json.dumps(patch):
            entry["patch"] = canonical
            changed = True
    if changed:
        text = json.dumps(diff, indent=2, ensure_ascii=False)
        path.write_bytes((text.replace("\n", "\r\n") + "\n").encode())
        print("fixed", path)
