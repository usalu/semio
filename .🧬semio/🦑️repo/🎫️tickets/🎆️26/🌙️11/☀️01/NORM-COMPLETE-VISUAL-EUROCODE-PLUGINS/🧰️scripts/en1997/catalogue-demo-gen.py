"""Generates the committed EN 1997 full-catalogue demo document `🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json`: one seeded case per registry entry in registry order, mirroring `design_case::seed` (number and choice/text examples of the vocabulary). Run with python3 -I."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import json, pathlib, re
ROOT = pathlib.Path("/home/deck/semio")
ART = next(ROOT.glob("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any"))
CAT = ART / "🧬️schema/💡️inferences/⚖️checks/📇️catalogue"
OUT = ART / "🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json"
GROUPS = ["factors", "earth-pressure", "foundations", "settlement", "piles", "walls-and-anchors", "slopes", "investigation", "hydraulics", "seepage", "wall-sections", "embankments"]
folders = {p.name.split("️", 1)[-1] if False else re.sub(r"^[^a-z]+", "", p.name): p for p in CAT.iterdir() if p.is_dir()}
vocab = json.loads((CAT / "📚️inputs/🔣️.json").read_text(encoding="utf-8"))["inputs"]
pat = re.compile(r'Entry \{ id: "([^"]+)", part: "[^"]*", clause: "[^"]*", en: "((?:[^"\\]|\\.)*)", de: "((?:[^"\\]|\\.)*)", inputs: &\[([^\]]*)\]')
def spec(entry_id, key):
    rows = [r for r in vocab if r["key"] == key]
    scoped = [r for r in rows if entry_id in [s.strip() for s in r.get("scope", "").split(",") if s.strip()]]
    return (scoped or [r for r in rows if not r.get("scope")] or rows or [None])[0]
cases = []
for g in GROUPS:
    text = (folders[g] / "🦀️.rs").read_text(encoding="utf-8")
    for m in pat.finditer(text):
        eid, en, de, inputs = m.group(1), json.loads('"%s"' % m.group(2)), json.loads('"%s"' % m.group(3)), re.findall(r'"([^"]+)"', m.group(4))
        nums, txts = {}, {}
        for key in inputs:
            r = spec(eid, key)
            if not r or "example" not in r:
                continue
            if r["kind"] == "number" and isinstance(r["example"], (int, float)):
                nums[key] = float(r["example"])
            elif r["kind"] in ("choice", "text") and isinstance(r["example"], str):
                txts[key] = r["example"]
        cases.append({"id": "case-" + "".join(c if c.isascii() and c.isalnum() else "-" for c in eid.lower()), "labelEn": en, "labelDe": de, "entryId": eid,
                      "numbers": [{"id": k, "value": nums[k]} for k in sorted(nums)], "texts": [{"id": k, "value": txts[k]} for k in sorted(txts)]})
doc = {"structureId": "geo-full-catalogue-demo", "geotechnicalCategory": 2, "designSituation": "bsP", "designApproach": "da2", "annex": "De", "groundwaterLevel": 4.0, "investigationDepth": 12.0,
       "layers": [], "footings": [], "piles": [], "retainingWalls": [], "slopes": [], "upliftCases": [], "anchors": [], "embeddedWalls": [], "pileGroups": [], "labTests": [], "layerWaters": [], "placements": [], "cases": cases}
OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(len(cases), "cases")
