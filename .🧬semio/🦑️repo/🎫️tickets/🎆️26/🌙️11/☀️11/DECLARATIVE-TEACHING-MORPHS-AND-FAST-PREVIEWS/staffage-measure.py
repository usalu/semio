"""Measure staffage cast cost without a render: cache footprint per cast, warm resolve cost (cast_key, index, provenance), frame parse cost.
Run: python -I staffage-measure.py [out.json]  (cwd = repo root, venv python with manim, PYTHONUTF8=1)"""
import importlib
import json
import statistics
import sys
import time
from pathlib import Path

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
casts = S._manifest()["casts"]
rows = []
t0 = time.perf_counter()
S._source_digest.cache_clear()
digest = S._source_digest()
digest_ms = (time.perf_counter() - t0) * 1000
for name, entry in casts.items():
    t = time.perf_counter()
    key = S.cast_key(entry)
    key_ms = (time.perf_counter() - t) * 1000
    folder = S._cache_root() / key / name
    if not (folder / S._INDEX).is_file():
        rows.append({"cast": name, "cached": False, "key_ms": key_ms})
        continue
    t = time.perf_counter()
    index = S._index(folder)
    index_ms = (time.perf_counter() - t) * 1000
    names = list(dict.fromkeys(index["frames"]))
    sizes = [(folder / n).stat().st_size for n in names]
    t = time.perf_counter()
    S._slot_count(folder)
    slot_ms = (time.perf_counter() - t) * 1000
    t = time.perf_counter()
    parsed = S._parse_frame((folder / names[0]).read_text(encoding="utf-8"), index)
    parse_ms = (time.perf_counter() - t) * 1000
    t = time.perf_counter()
    for n in names[:20]:
        S._parse_frame((folder / n).read_text(encoding="utf-8"), index)
    parse20_ms = (time.perf_counter() - t) * 1000
    rows.append({"cast": name, "cached": True, "view": entry["view"], "viewpoint": entry["viewpoint"], "scale": entry["scale"], "kind": "clip" if "clip" in entry else "pose",
                 "frames": len(index["frames"]), "unique": len(names), "bytes": sum(sizes), "paths_first": len(parsed.spans), "points_first": int(len(parsed.points)),
                 "key_ms": round(key_ms, 3), "index_ms": round(index_ms, 3), "slot_ms": round(slot_ms, 2), "parse_first_ms": round(parse_ms, 2), "parse_first20_ms": round(parse20_ms, 2)})
cached = [r for r in rows if r["cached"]]
summary = {"casts": len(rows), "cached": len(cached), "source_digest_ms": round(digest_ms, 1),
           "cast_key_ms_median": round(statistics.median(r["key_ms"] for r in rows), 3),
           "bytes_total": sum(r["bytes"] for r in cached), "frames_total": sum(r["frames"] for r in cached), "unique_total": sum(r["unique"] for r in cached),
           "clips": sum(r["kind"] == "clip" for r in cached),
           "slot_ms_total": round(sum(r["slot_ms"] for r in cached), 1),
           "parse_first_ms_median": round(statistics.median(r["parse_first_ms"] for r in cached), 2),
           "paths_first_median": statistics.median(r["paths_first"] for r in cached),
           "paths_first_max": max(r["paths_first"] for r in cached)}
out = {"summary": summary, "top_bytes": sorted(cached, key=lambda r: -r["bytes"])[:8], "top_unique": sorted(cached, key=lambda r: -r["unique"])[:8], "rows": rows}
Path(sys.argv[1] if len(sys.argv) > 1 else "staffage-measure.json").write_text(json.dumps(out, ensure_ascii=False, indent=1), encoding="utf-8")
print(json.dumps(summary, indent=1))
for r in out["top_bytes"][:5]:
    print(r["cast"], r["frames"], r["unique"], r["bytes"], r["paths_first"], r["slot_ms"])
