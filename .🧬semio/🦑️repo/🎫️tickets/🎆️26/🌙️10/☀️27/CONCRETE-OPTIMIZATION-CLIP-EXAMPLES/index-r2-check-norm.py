import importlib
import sys
from decimal import ROUND_HALF_UP, Decimal
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
order = [c["id"] for c in m.criteria()]
rows = m.index(m.load_study()["weights"]["balanced"])


def r(value, digits):
    return Decimal(repr(float(value))).quantize(Decimal(1).scaleb(-digits), rounding=ROUND_HALF_UP)


for c in order:
    key = next(x["key"] for x in m.criteria() if x["id"] == c)
    direction = next(x["direction"] for x in m.criteria() if x["id"] == c)
    for digits in (2, 3):
        values = {v["id"]: float(r(v[key], digits)) for v in m.variants()}
        lo, hi = min(values.values()), max(values.values())
        bad = []
        for row in rows:
            n = (values[row["id"]] - lo) / (hi - lo)
            n = 1 - n if direction == "max" else n
            if r(n, 2) != r(row["normalized"][c], 2):
                bad.append((row["id"], float(r(n, 2)), float(r(row["normalized"][c], 2))))
        print(c, digits, "bad" if bad else "ok", bad)
