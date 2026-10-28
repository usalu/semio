#!/usr/bin/env python3
"""🏷️ Writes batches/names.json: files and folders that share their emoji with a sibling (or have none) get their own handpicked emoji, and every reference follows. python3 -I names_spec.py"""
import json, os, subprocess
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.isdir(os.path.join(ROOT, "🧰️framework")) and os.path.dirname(ROOT) != ROOT: ROOT = os.path.dirname(ROOT)
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
P = "🧰️framework/🔨️modules/🏃️process/"
renames = [
    (P + "🧪️testing/🟨️bun/🪦️orphans/🧪️tests/👶️fixture/📜️holder.ts", "🤲️holder.ts"),
    (P + "🧪️testing/🟨️bun/🪦️orphans/🧪️tests/👶️fixture/📜️leaver.ts", "🚪️leaver.ts"),
    (P + "🧪️testing/🟨️bun/🪦️orphans/🧪️tests/👶️fixture/📜️limit.ts", "📏️limit.ts"),
    (P + "🧪️testing/🟨️bun/🪦️orphans/🧪️tests/👶️fixture/📜️runner.ts", "🏃️runner.ts"),
    (L + "⚡️caching/🦀️cargo/🗃️unit-cache/🎰️jobserver/🧪️tests/🧫️fixtures/🐍️realise.py", "🪄️realise.py"),
    (L + "🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🧫️fixtures/📼️g5.json", "🎞️g5.json"),
    (L + "🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🧫️fixtures/📼️g29.json", "📹️g29.json"),
    (L + "🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🧫️fixtures/📼️g34.json", "🎥️g34.json"),
]
dirs = [
    (P + "📦️artifacts/🏗️native-build/🕸️units/🧪️tests/🔁️plan-stability", "⚓️plan-stability"),
    (L + "🏗️build/👁️observation/🧪️tests/🧾️row-sequence", "🪜️row-sequence"),
    (L + "🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🧪️tests/🦀️source-inputs-memo", "🧠️source-inputs-memo"),
]
G = L + "🏗️build/🕸️graph/🕰️generations/🧪️tests/🧱️index-chunks/"
moves, manual = [], []
def reference_files(segment, exclude):
    out = subprocess.run(["rg", "-l", "-F", "--glob", "!node_modules", "--", segment, "🧰️framework"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    return [f for f in out if f and f not in exclude and not f.startswith(exclude_dirs)]
exclude_dirs = tuple(old + "/" for old, _ in dirs)
for old, new_name in renames:
    new = os.path.join(os.path.dirname(old), new_name)
    moves.append({"from": old, "to": new})
    for ref in reference_files(os.path.basename(old), {old}):
        manual.append({"rel": ref, "old": os.path.basename(old), "new": new_name, "count": "any"})
for old, new_name in dirs:
    new = os.path.join(os.path.dirname(old), new_name)
    for base, _, names in os.walk(os.path.join(ROOT, old)):
        for name in names:
            rel = os.path.relpath(os.path.join(base, name), ROOT)
            moves.append({"from": rel, "to": new + rel[len(old):]})
    for ref in reference_files(os.path.basename(old), set()):
        if not ref.startswith(old + "/"): manual.append({"rel": ref, "old": os.path.basename(old), "new": new_name, "count": "any"})
moves.append({"from": G + "legacy.ts", "to": G + "🗜️flat-reference.ts"})
manual.append({"rel": G + "🗜️flat-reference.ts", "old": "legacyCreateFactIndexV1", "new": "flatReferenceFactIndexV1", "count": "any"})
manual.append({"rel": G + "🟦️.ts", "old": "legacyCreateFactIndexV1", "new": "flatReferenceFactIndexV1", "count": "any"})
manual.append({"rel": G + "🟦️.ts", "old": "\"./legacy.ts\"", "new": "\"./🗜️flat-reference.ts\"", "count": 1})
files = sorted({m["rel"] for m in manual if m["rel"] not in {mv["to"] for mv in moves}})
spec = {"title": "🏷️ Staged batch `cp-names` (lane compliance): fixtures and test folders that shared their emoji with a sibling (or had none) get their own handpicked emoji; the references follow; the flat reference model of the index chunks no longer calls itself legacy.", "files": files, "manual": manual, "moves": moves, "emoji": [], "verify": []}
json.dump(spec, open(os.path.join(HERE, "batches", "names.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(moves), "moves", len(files), "referencing files")
for f in files: print("  ", f[-90:])
