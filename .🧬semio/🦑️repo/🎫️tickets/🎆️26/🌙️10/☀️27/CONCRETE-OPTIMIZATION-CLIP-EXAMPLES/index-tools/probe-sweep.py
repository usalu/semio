"""Probe the sweep: winners along the cost weight, exact switch shares, close-call bands, cost cap candidates."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
study = model.load_study()
base = study["weights"]["balanced"]
ids = [v["id"] for v in model.variants()]


def winner(share):
    rows = model.index(model.reweigh(base, "cost", share / 100), ids)
    return min(rows, key=lambda r: r["score"])["id"]


previous = winner(0)
print("start", previous)
for k in range(1, 4001):
    share = k * 0.025
    current = winner(share)
    if current != previous:
        print("switch", previous, "->", current, "at", round(share, 3))
        previous = current
for share in (0, 2.5, 5, 10, 11, 12, 40, 100):
    rows = sorted(model.index(model.reweigh(base, "cost", share / 100), ids), key=lambda r: r["score"])
    print(share, [(r["id"], round(r["score"], 3)) for r in rows[:3]])
for cap in (21000, 21500):
    print("cap", cap, [(v["id"], v["cost"]) for v in model.variants() if v["cost"] <= cap])
