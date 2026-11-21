"""🌱 Round 5 (E01 appliances): the root corrections outside `volumes` for gas-boiler, hot-water-cylinder, circulation-pump, mvhr-unit and decentralised-hrv-fan, each stated by hand in `EDITS`. Builds the candidate `🗑️generated/r5-<slug>/🔣️.json` (catalog asset with the edits and the volumes source) and, with `--apply`, writes only the changed top-level members (never `volumes`) into the catalog file in its own formatting, re-reading the file right before writing. Idempotent.

Usage: python3 r5-e01-roots.py <slug> [--apply]
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances"))
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])


def region(asset, rid):
    """🌡️ The thermal region with this id."""
    return next(r for r in asset["thermal"]["regions"] if r["id"] == rid)


def gas_boiler(a):
    """🔥 1:200 control band drawn 30 mm deep in front and plan (15 mm proud of the hood, 15 mm behind its face where the hood covers it) so it prints to the end of the range; connections area from the honest 1:50 tubes and nuts."""
    g = a["representations"]["200"]["geometry"]
    front = next(s for s in g["front"] if s["id"] == "panel")
    front["w"] = 0.03
    plan = next(s for s in g["plan"] if s["id"] == "panel")
    plan["h"] = 0.03
    region(a, "connections")["areaM2"] = 0.035


def mvhr_unit(a):
    """🌬️ 1:200 drawing valid to 1:210: its four 90 mm spigots stand 60 mm apart, a gap that falls under the 0.15 mm print minimum beyond that."""
    a["representations"]["200"]["validRange"]["maxDenominator"] = 210


EDITS = {"gas-boiler": gas_boiler, "mvhr-unit": mvhr_unit}


def main():
    """🚀 Builds the candidate and optionally applies it."""
    slug = sys.argv[1]
    path = os.path.join(CATALOG, slug, "🔣️.json")
    doc = json.load(open(path, encoding="utf-8"))
    EDITS.get(slug, lambda a: None)(doc["asset"])
    doc["asset"]["volumes"] = json.load(open(os.path.join(HERE, "volumes", "appliances", slug + ".json"), encoding="utf-8"))["volumes"]
    out = os.path.join(HERE, "🗑️generated", "r5-" + slug, "🔣️.json")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    json.dump(doc, open(out, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("candidate", out)
    if "--apply" not in sys.argv:
        return
    want = doc["asset"]
    text = open(path, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    changed = [key for key in have if key != "volumes" and have[key] != want[key]]
    for key in changed:
        start, end = member_range(text, "    ", key)
        text = text[:start] + "    " + json.dumps(key) + ": " + dump(want[key], 2) + text[end:]
    got = json.loads(text)["asset"]
    if list(got) != list(have) or any(got[key] != want[key] for key in want if key != "volumes") or got["volumes"] != have["volumes"]:
        raise SystemExit("the edited text does not match the candidate")
    open(path, "w", encoding="utf-8").write(text)
    print("written", path, "members:", ", ".join(changed) or "none")


if __name__ == "__main__":
    main()
