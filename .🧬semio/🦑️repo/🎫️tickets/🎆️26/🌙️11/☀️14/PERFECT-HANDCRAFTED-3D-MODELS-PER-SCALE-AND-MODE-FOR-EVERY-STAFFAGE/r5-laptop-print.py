"""🖨️ Round 5, appliances/laptop: the two print findings that refuse the catalog asset fixed at the root. 1:50 plan: the hinge barrel lies under the lid shell, only a 1 mm sliver stayed visible (0.187 mm at 1:20), so the hidden barrel shape is removed from the plan and the decision text follows. 1:200: the 6.5 mm lid is 0.199 mm at 1:350, with the 0.18 mm body pen the band can reach 1:320 (6.5 mm / 320 + 0.18 = 0.200 mm), so it ends there (the 1:500 drawing is unavailable, nothing else starts there).

Usage: python3 r5-laptop-print.py candidate   writes 🗑️generated/r5-laptop/cand/🔣️.json
       python3 r5-laptop-print.py apply       writes the candidate's representations into the catalog file
@see ./r5-brief.md
@see ./r3-laptop-draw.py
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/laptop/🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "appliances", "laptop.json")
CANDIDATE = os.path.join(HERE, "🗑️generated", "r5-laptop", "cand", "🔣️.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])

MAX_200 = 320
TEXT_50 = [
    ("and the lid seen from above as a band behind the hinge barrel.", "and the lid seen from above as a band over the hinge barrel, which it hides."),
    ("sowie der Deckel von oben als Band hinter der Scharnierwalze.", "sowie der Deckel von oben als Band über der Scharnierwalze, die er verdeckt."),
]


def candidate():
    """🧪 The catalog asset with the hidden plan barrel removed at 1:50, the 1:200 band ending at 1:320 and the source volumes."""
    doc = json.load(open(CATALOG, encoding="utf-8"))
    asset = doc["asset"]
    rep50 = asset["representations"]["50"]
    rep50["geometry"]["plan"] = [s for s in rep50["geometry"]["plan"] if s["id"] != "plan-hinge"]
    raw = json.dumps(rep50["decisions"], ensure_ascii=False)
    for old, new in TEXT_50:
        if old not in raw:
            print("[r5] text not found:", old[:60])
        raw = raw.replace(old, new)
    rep50["decisions"] = json.loads(raw)
    asset["representations"]["200"]["validRange"]["maxDenominator"] = MAX_200
    asset["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(doc, open(CANDIDATE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("[r5] candidate", CANDIDATE)


def apply():
    """📝 The candidate's representations into the catalog file in its own formatting."""
    want = json.load(open(CANDIDATE, encoding="utf-8"))["asset"]
    text = open(CATALOG, encoding="utf-8").read()
    start, end = member_range(text, "    ", "representations")
    text = text[:start] + "    " + json.dumps("representations") + ": " + json.dumps(want["representations"], indent=2, ensure_ascii=False).replace("\n", "\n    ") + text[end:]
    if json.loads(text)["asset"]["representations"] != want["representations"]:
        raise SystemExit("the edited text does not match the candidate")
    open(CATALOG, "w", encoding="utf-8").write(text)
    print("[r5] written representations")


{"candidate": candidate, "apply": apply}[sys.argv[1]]()
