"""🧾️ Adds owner-debt manifest rows (cad object verbs, layout rotate-frame) with outcomes read from each leaf's diff."""
import json, sys
root = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/"
def row(kind, variant, cap, outcomes, subset=None):
    r = {"id": kind, "capability": cap, "payloadSchema": "🧬️.schema.json", "outcomes": outcomes, "productionDispatch": {"operation": kind, "bridgeVersion": 1, "variant": variant}, "oracleRequirements": [{"capability": cap, "qualifyingKind": "verified-native-second-implementation"}]}
    if subset: r["subset"] = subset
    return r
edits = {
    "📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json": [
        row("create-object", "CreateObject", "cad-1-mutate", ["rejected", "no-op", "applied"], "object"),
        row("delete-object", "DeleteObject", "cad-1-mutate", ["rejected", "applied"], "object"),
        row("move-objects", "MoveObjects", "cad-1-mutate", ["rejected", "no-op", "applied"], "object"),
        row("rotate-objects", "RotateObjects", "cad-1-mutate", ["rejected", "no-op", "applied"], "object"),
        row("scale-objects", "ScaleObjects", "cad-1-mutate", ["rejected", "no-op", "applied"], "object"),
    ],
    "📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json": [
        row("rotate-frame", "RotateFrame", "layout-1-mutate", ["rejected", "applied"]),
    ],
}
for rel, rows in edits.items():
    path = root + rel
    text = open(path, encoding="utf-8").read()
    d = json.loads(text)
    m = d["mutationManifests"][0]
    have = {r["id"] for r in m["mutations"]}
    m["mutations"].extend(r for r in rows if r["id"] not in have)
    m["mutations"].sort(key=lambda r: r["id"])
    open(path, "w", encoding="utf-8").write(json.dumps(d, ensure_ascii=False, indent=2) + ("\n" if text.endswith("\n") else ""))
    print(path, len(m["mutations"]))
