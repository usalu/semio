"""Dump the variants, criteria and index results the index scene depends on."""
import importlib
import json
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
out = []
study = model.load_study()
out.append(json.dumps(study["weights"], ensure_ascii=False))
for c in model.criteria():
    out.append(json.dumps(c, ensure_ascii=False))
for v in model.variants(extra=True):
    out.append(json.dumps({"id": v["id"], "title": v["title"], "design": v["design"], "extra": v.get("extra"),
                           "intensity": v["intensity"], "cost": v["cost"], "daylight": v["daylight"], "carbon": v["carbon"]}, ensure_ascii=False))
for name, weights, ids in (("balanced", study["weights"]["balanced"], None), ("cost_heavy", study["weights"]["cost_heavy"], None),
                           ("balanced6", study["weights"]["balanced"], [v["id"] for v in model.variants(extra=True)])):
    out.append(name)
    for row in model.index(weights, ids):
        out.append(json.dumps({"id": row["id"], "raw": row["raw"], "n": {k: round(x, 3) for k, x in row["normalized"].items()},
                               "score": round(row["score"], 3), "rank": row["rank"]}, ensure_ascii=False))
reweighed = model.reweigh(study["weights"]["balanced"], "cost", 0.6)
out.append("reweigh 0.6 " + json.dumps(reweighed))
out.append("cost_heavy set " + json.dumps(study["weights"]["cost_heavy"]))
Path(sys.argv[1]).write_text("\n".join(out), encoding="utf-8")
