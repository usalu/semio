import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
ga = m.genetic()
gen = ga["generations"][0]
print(gen["tournaments"])
lookup = {i["id"]: i for i in gen["population"]}
for ch in gen["children"]:
    cross = {k: lookup[ch["genes"][k]]["design"][k] for k in ("wwr", "overhang")}
    e = m.evaluate({**ch["design"], **cross})["electricity"]
    print(ch["id"], ch["from"], "cross", {k: round(v, 3) for k, v in cross.items()}, round(e, 1), "child", round(ch["electricity"], 1),
          {k: round(v, 4) for k, v in ch["mutation"].items()})
for k, g in enumerate(ga["generations"][:-1]):
    lk = {i["id"]: i for i in g["population"]}
    for ch in g["children"]:
        cross = {a: lk[ch["genes"][a]]["design"][a] for a in ("wwr", "overhang")}
        e = m.evaluate({**ch["design"], **cross})["electricity"]
        if any(ch["mutation"].values()):
            print(k, ch["id"], round(e, 1), "->", round(ch["electricity"], 1), "better" if ch["electricity"] < e else "worse")
print(m.load_study()["algorithms"])
