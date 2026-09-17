#!/usr/bin/env python3
"""🔢 Rewrite integer literals under the f32/f64 fields of the remodeling fixtures as floats (`2` → `2.0`):
the Rust codec's decode→encode fixed point prints every float with a decimal, and the fixture oracle
compares JSON values, which distinguish `2` from `2.0` (215 `committed … is not canonical` /
`produced diff differs` failures on 2026-09-17). The float keys are DERIVED from the normative JSON
schemas (`🧬️schema/🔣️.json` + every `🧬️mutations/*/🧬️schema/🔣️.json`: keys typed `number` — also inside a nullable
`["number", "null"]` — and never `integer`), never hand-listed. Run from the repo root; idempotent."""
import glob, json, os
A = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
keys = {}

def collect(node, key=None):
    if isinstance(node, dict):
        kind = node.get("type")
        kinds = kind if isinstance(kind, list) else [kind]
        for one in kinds:
            if key and one in ("number", "integer"):
                keys.setdefault(key, set()).add(one)
        for name, value in node.items():
            if name == "properties" and isinstance(value, dict):
                for child_key, child in value.items():
                    collect(child, child_key)
            elif name in ("$defs", "definitions"):
                for child in value.values():
                    collect(child, None)
            elif isinstance(value, (dict, list)):
                collect(value, key)
    elif isinstance(node, list):
        for child in node:
            collect(child, key)

for path in [os.path.join(A, "🧬️schema", "🔣️.json"), *glob.glob(os.path.join(A, "🧬️schema", "🧬️mutations", "*", "🧬️schema", "🔣️.json"))]:
    collect(json.load(open(path, encoding="utf-8")))
conflicts = {k: v for k, v in keys.items() if len(v) > 1}
assert not conflicts, f"a key is typed both ways across the schemas: {conflicts}"
FLOAT_KEYS = {k for k, v in keys.items() if v == {"number"}}

def walk(value, key):
    if isinstance(value, dict):
        return {k: walk(v, k) for k, v in value.items()}
    if isinstance(value, list):
        return [walk(v, key) for v in value]
    if isinstance(value, bool):
        return value
    if isinstance(value, int) and key in FLOAT_KEYS:
        return float(value)
    return value

changed = 0
for root, _, files in os.walk(os.path.join(A, "🧫️fixtures")):
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
print(f"[DEBUG] float keys: {sorted(FLOAT_KEYS)}")
print(f"[DEBUG] rewrote {changed} fixture files")
