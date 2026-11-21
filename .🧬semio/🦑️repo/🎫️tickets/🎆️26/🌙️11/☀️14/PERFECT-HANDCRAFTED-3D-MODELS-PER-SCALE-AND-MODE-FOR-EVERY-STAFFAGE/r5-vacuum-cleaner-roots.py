"""🌱 Round 5 vacuum cleaner: the root corrections outside `volumes`, so the drawings print over the whole preset bands (rule valid-range-gap): in the front and plan views of 1:50 the hose lies in the symmetry plane along the 35 mm wand, and its true 36 mm left 0.5 mm slivers beside the wand that fall below 0.2 mm on paper at 1:20; those two views now draw the hose 34 mm, inside the wand, as a draughtsman generalises a half-millimetre step (the side view keeps the true 36 mm). The 1:50 band starts again at 1:20 and the 1:500 band now reaches 1:1000. Re-reads the catalog file right before writing and replaces only the `representations` member, in the file's own formatting.

Usage: python3 r5-vacuum-cleaner-roots.py [--apply]
"""
import copy
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/vacuum-cleaner/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r5-vacuum-cleaner", "cand", "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "appliances", "vacuum-cleaner.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])
R_VIEW = 0.017
NOTE = {"en": " In the front view and the plan the hose lies along the 35 mm wand; it is drawn 34 mm there, inside the wand, because its true 0.5 mm step beside the wand would print as a sliver below 0.2 mm.", "de": " In Vorderansicht und Grundriss liegt der Schlauch längs des 35-mm-Rohrs; er ist dort 34 mm gezeichnet, innerhalb des Rohrs, weil seine wahre Stufe von 0,5 mm neben dem Rohr als Splitter unter 0,2 mm drucken würde."}


def representations(reps):
    """🖊️ The corrected representations: ranges and the hose width of the front view and plan of 1:50."""
    r = copy.deepcopy(reps)
    r["50"]["validRange"]["minDenominator"] = 20
    r["500"]["validRange"]["maxDenominator"] = 1000
    for view in ("front", "plan"):
        for s in r["50"]["geometry"][view]:
            if s["id"].startswith("hose-") and s["id"].endswith("-tube"):
                s["r0"] = s["r1"] = R_VIEW
                s["solid"]["z0"], s["solid"]["z1"] = -R_VIEW, R_VIEW
    thin = r["50"]["decisions"]["thinElements"]
    for lang in ("en", "de"):
        if NOTE[lang].strip() not in thin[lang]:
            thin[lang] += NOTE[lang]
    return r


def main():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    want = representations(doc["asset"]["representations"])
    cand = copy.deepcopy(doc)
    cand["asset"]["representations"] = want
    cand["asset"]["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(cand, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("candidate", CANDIDATE)
    if "--apply" not in sys.argv:
        return
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    start, end = member_range(text, "    ", "representations")
    text = text[:start] + "    " + json.dumps("representations") + ": " + json.dumps(want, indent=2, ensure_ascii=False).replace("\n", "\n    ") + text[end:]
    got = json.loads(text)["asset"]
    if any(got[k] != have[k] for k in have if k != "representations") or got["representations"] != want or list(got) != list(have):
        raise SystemExit("the edited text changes more than the representations")
    open(CATALOG, "w", encoding="utf-8").write(text)
    print("written", CATALOG)


if __name__ == "__main__":
    main()
