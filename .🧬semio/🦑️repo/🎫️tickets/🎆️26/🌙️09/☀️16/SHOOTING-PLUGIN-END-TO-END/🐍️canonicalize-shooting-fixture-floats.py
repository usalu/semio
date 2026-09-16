#!/usr/bin/env python3
"""🔢 Rewrite integer literals under f64 fields of the shooting fixtures as floats (`5` → `5.0`): the
Rust codec's decode→encode fixed point prints every f64 with a decimal, and the fixture oracle
compares `serde_json::Value`s, which distinguish `5` from `5.0`. Also drops the retired
`activeUtilityId` config field. Run from the repo root; idempotent."""
import json, os, sys
A = "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any"
FLOAT_KEYS = {"angle", "ax", "ay", "az", "azimuth", "dx", "dy", "dz", "elevation", "emissiveIntensity", "emissive_intensity", "fov", "intensity", "metalness",
              "new_azimuth", "new_elevation", "new_intensity", "new_roughness", "opacity", "orientation", "origin", "position", "roughness", "scale", "softness",
              "sx", "sy", "sz", "target", "up", "zoom"}
DROP_KEYS = {"activeUtilityId"}

def walk(value, key):
    if isinstance(value, dict):
        return {k: walk(v, k) for k, v in value.items() if k not in DROP_KEYS}
    if isinstance(value, list):
        return [walk(v, key) for v in value]
    if isinstance(value, bool):
        return value
    if isinstance(value, int) and key in FLOAT_KEYS:
        return float(value)
    return value

changed = 0
for root, _, files in os.walk(A):
    if "🧫️fixtures" not in root:
        continue
    for name in files:
        if not name.endswith(".json"):
            continue
        path = os.path.join(root, name)
        raw = open(path, encoding="utf-8").read()
        data = json.loads(raw)
        out = json.dumps(walk(data, ""), indent=2, ensure_ascii=False) + "\n"
        if out != raw:
            open(path, "w", encoding="utf-8").write(out)
            changed += 1
print(f"[DEBUG] rewrote {changed} fixture files")
