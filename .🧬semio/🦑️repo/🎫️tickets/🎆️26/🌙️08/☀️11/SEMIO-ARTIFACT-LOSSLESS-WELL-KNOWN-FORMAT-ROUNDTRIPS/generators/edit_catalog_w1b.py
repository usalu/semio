#!/usr/bin/env python3
# 📎 W1b closer: catalog.json edit — adds semio + 7 format roster rows, DAG edges, owner row,
# retires the dead `neutral` field (zero script.ts readers, W1 confirmed), bumps counts.stdio_artifacts.
# Round-trip verified: json.load -> json.dumps(indent=2, ensure_ascii=False) reproduces the
# ORIGINAL file byte-for-byte when nothing changes (confirmed separately) — so the only diff
# lines this script produces are the intended ones.
import json
import collections

P = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json"

with open(P, encoding="utf-8") as f:
    d = json.load(f, object_pairs_hook=collections.OrderedDict)

# ---- retire `neutral` from all 28 existing rows ----
for row in d["stdio_roster"].values():
    row.pop("neutral", None)

# ---- bridged format ids (master plan §3 "Full import/export lattice"), in roster-key order,
# ---- with the 4 new binary formats appended (they don't have a prior roster position) ----
existing_order = list(d["stdio_roster"].keys())
bridged_existing = [
    "txt", "xml", "json", "csv", "md", "gltf", "obj", "stl", "ply", "las", "step", "ifc",
    "dwg", "dxf", "svg", "png", "jpg", "gif", "bmp", "tiff", "pdf", "docx", "pptx", "bcf",
]
assert all(k in existing_order for k in bridged_existing)
bridged = bridged_existing + ["mp4", "avi", "mp3", "wav"]
assert len(bridged) == 28, len(bridged)

new_rows = collections.OrderedDict([
    ("semio", {"dir": "🧿️semio", "mime": "application/vnd.semio", "ext": ".semio", "depends": bridged}),
    ("mp4", {"dir": "🎥️mp4", "mime": "video/mp4", "ext": ".mp4", "depends": ["binary"]}),
    ("avi", {"dir": "📼️avi", "mime": "video/x-msvideo", "ext": ".avi", "depends": ["binary"]}),
    ("mp3", {"dir": "🎵️mp3", "mime": "audio/mpeg", "ext": ".mp3", "depends": ["binary"]}),
    ("wav", {"dir": "🔊️wav", "mime": "audio/wav", "ext": ".wav", "depends": ["binary"]}),
    ("epw", {"dir": "🌦️epw", "mime": "text/plain", "ext": ".epw", "depends": ["txt"]}),
    ("tsv", {"dir": "📑️tsv", "mime": "text/tab-separated-values", "ext": ".tsv", "depends": ["txt"]}),
    ("html", {"dir": "🌐️html", "mime": "text/html", "ext": ".html", "depends": ["txt"]}),
])
for k, v in new_rows.items():
    assert k not in d["stdio_roster"], k
    d["stdio_roster"][k] = v

assert len(d["stdio_roster"]) == 36, len(d["stdio_roster"])

# ---- DAG edges: mirror `depends` exactly like every existing row already does ----
for k, v in new_rows.items():
    for dep in v["depends"]:
        d["stdio_dag_edges"].append({"from": k, "to": dep})

# ---- owners: one new row for semio itself (machine-checked io coverage capability
# ---- statement — mirrors the 54 domain-plugin owner rows' shape) ----
d["owners"].append({
    "plugin": "🗄️stdio",
    "artifact": "🧿️semio",
    "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio",
    "kind_id": "s.stdio.semio",
    "stdio_artifacts": bridged,
    "import": bridged,
    "export": bridged,
})

# ---- counts ----
d["counts"]["stdio_artifacts"] = 36
d["counts"]["curated_io_pairs_note"] = (
    "273 is hand-maintained (not mechanically recomputed by this file/script.ts) — left "
    "unchanged by W1b; W4 adds the semio+7-format owner/leaf pairs and should recompute this "
    "number then, not fabricate it here."
)

out = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
with open(P, "w", encoding="utf-8") as f:
    f.write(out)
print("wrote", len(out), "bytes;", len(d["stdio_roster"]), "roster rows,", len(d["owners"]), "owners,", len(d["stdio_dag_edges"]), "edges")
