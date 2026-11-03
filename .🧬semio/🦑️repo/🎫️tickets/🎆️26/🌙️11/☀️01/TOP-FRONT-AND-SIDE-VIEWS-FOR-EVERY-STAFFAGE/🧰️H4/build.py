import sys

sys.path.insert(0, __file__.rsplit("\\", 1)[0])
from lib import *
from std import front_std, plan_std
from assets_std import CONFIGS
import special
import text
CUSTOM = {"wheelchair-user": (special.wheel_front, special.wheel_plan, special.WHEEL), "sleeping-person": (special.sleep_front, special.sleep_plan, special.SLEEP)}

TEXT = {}


def run(aid):
    doc = load(aid)
    a = doc["asset"]
    A = CONFIGS.get(aid) or CUSTOM[aid][2]
    ff, pf = (front_std, plan_std) if aid in CONFIGS else CUSTOM[aid][:2]
    for sc, rep in a["representations"].items():
        if rep["status"] != "authored":
            continue
        rep["geometry"]["front"] = ff(A, a, rep, sc)
        rep["geometry"]["plan"] = pf(A, a, rep, sc)
        for key, (en, de) in text.texts(aid, sc).items():
            d = rep["decisions"][key]
            for lang, add, marks in (("en", en, (" Front view", " Front and plan:", " Plan:")), ("de", de, (" Vorderansicht", " Vorderansicht und Grundriss:", " Grundriss:"))):
                cur = d[lang]
                cuts = [cur.find(m) for m in marks if cur.find(m) >= 0]
                if cuts:
                    cur = cur[:min(cuts)]
                d[lang] = cur.rstrip() + " " + add
    a["presentation"].pop("viewpoints", None)
    save(aid, doc)
    print(aid, {sc: (len(r["geometry"]["front"]), len(r["geometry"]["plan"])) for sc, r in a["representations"].items() if r["status"] == "authored"})


if __name__ == "__main__":
    for aid in sys.argv[1:]:
        run(aid)
