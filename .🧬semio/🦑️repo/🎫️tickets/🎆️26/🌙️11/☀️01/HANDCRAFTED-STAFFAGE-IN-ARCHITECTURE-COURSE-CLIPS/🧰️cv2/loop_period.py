import importlib
import json

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
casts = S._manifest()["casts"]
rows, seen = [], set()
for cast, entry in casts.items():
    if "clip" not in entry:
        continue
    document = json.loads(S._catalog_bytes(entry["asset"]))
    clip = next(c for c in document["asset"]["clips"] if c["id"] == entry["clip"])
    fps = entry.get("fps", 15)
    scheduled = clip["durationS"] * fps
    count = round(scheduled)
    if clip["loop"] and count <= 600 and abs(scheduled - count) > 1e-9 and (entry["asset"], entry["clip"], fps) not in seen:
        seen.add((entry["asset"], entry["clip"], fps))
        rows.append((cast, clip["durationS"], fps, count, count / fps, count / fps - clip["durationS"]))
looping = sum(1 for e in casts.values() if "clip" in e)
print("clip casts:", looping, "| looping clips with a non-integer duration x fps:", len(rows))
for row in rows:
    print("  %s: clip %.4f s, fps %s, %d frames, loop period %.4f s, seam error %+.4f s" % row)
