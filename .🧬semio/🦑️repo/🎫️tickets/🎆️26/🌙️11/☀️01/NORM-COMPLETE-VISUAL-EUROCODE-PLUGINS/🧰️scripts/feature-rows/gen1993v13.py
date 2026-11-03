"""EN 1993 feature-rows fixtures for the joint plates and foundation blocks of v13/v14 (fin, web and flange cover plates, base plate cbend/tbend/mjrd/cone), derived from the committed showcase document, and their check-outcome rows from the committed oracle. Run: .venv/bin/python -I gen1993v13.py [--write]"""
import copy, json, pathlib, re, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIX = ROOT / "🧪️tests/🧫️fixtures"
DOCS = FIX / "📄️oracle-documents"
FEATURE = ROOT / "🧪️tests/🚦️check-outcome-en1993-1/🥒️.feature"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
showcase = json.load(open(FIX / "🎪️showcase/🔣️.json"))


def overload(d):
    by = lambda coll, i: next(x for x in d[coll] if x["id"] == i)
    for a in by("joints", "joint-j1")["actions"]:
        a["shear"] *= 8.0
    by("jointPlates", "plate-fin")["plateThickness"] = 0.006
    sp = by("joints", "joint-sp")
    sp["momentDemand"] *= 3.0
    for a in sp["actions"]:
        a["shear"] *= 4.0
    by("jointPlates", "plate-web")["plateThickness"] = 0.004
    bp = by("joints", "joint-bp")
    bp["baseMoment"] *= 10.0
    bp["anchorLength"] = 0.0
    for a in bp["actions"]:
        a["shear"] *= 5.0
    by("foundationBlocks", "block-bp")["anchorEmbedment"] = 0.1
    return d


def remedy_weld_grout(d):
    by = lambda coll, i: next(x for x in d[coll] if x["id"] == i)
    for a in by("joints", "joint-j1")["actions"]:
        a["shear"] *= 12.0
    by("jointPlates", "plate-fin")["weldThroat"] = 0.003
    block = by("foundationBlocks", "block-bp")
    block["groutFck"] = 0.1 * block["concreteFck"]
    block["groutThickness"] = 0.05
    return d


def remedy_mjrd(d):
    next(x for x in d["joints"] if x["id"] == "joint-bp")["baseMoment"] *= 3.0
    return d


FIXTURES = {"plates-pass": showcase, "plates-fail": overload(copy.deepcopy(showcase)), "plates-fail-weld-grout": remedy_weld_grout(copy.deepcopy(showcase)), "plates-fail-mjrd": remedy_mjrd(copy.deepcopy(showcase))}
EXPECT = {
    "plates-pass": ["en1993.1-8.3.6.plate.plate-fin", "en1993.1-8.3.10.plateblock.plate-fin", "en1993.1-8.4.5.plateweld.plate-fin", "en1993.1-8.3.6.plate.plate-flange", "en1993.1-1.6.2.3.platenet.plate-flange", "en1993.1-8.3.6.plate.plate-web", "en1993.1-8.6.2.grout.block-bp", "en1993.1-8.6.2.bearing.block-bp", "en1993.1-8.6.2.anchort.block-bp", "en1993.1-8.6.2.anchorv.block-bp", "en1993.1-8.6.2.cbend.block-bp", "en1993.1-8.6.2.tbend.block-bp", "en1993.1-8.6.2.mjrd.block-bp", "en1993.1-8.6.2.cone.block-bp"],
    "plates-fail": ["en1993.1-8.3.6.plate.plate-fin", "en1993.1-8.3.10.plateblock.plate-fin", "en1993.1-8.3.6.plate.plate-flange", "en1993.1-1.6.2.3.platenet.plate-flange", "en1993.1-8.3.6.plate.plate-web", "en1993.1-8.3.10.plateblock.plate-web", "en1993.1-8.6.2.bearing.block-bp", "en1993.1-8.6.2.cbend.block-bp", "en1993.1-8.6.2.anchort.block-bp", "en1993.1-8.6.2.anchorv.block-bp", "en1993.1-8.6.2.cone.block-bp"],
    "plates-fail-weld-grout": ["en1993.1-8.3.6.plate.plate-fin", "en1993.1-8.4.5.plateweld.plate-fin", "en1993.1-8.6.2.grout.block-bp"],
    "plates-fail-mjrd": ["en1993.1-8.6.2.anchort.block-bp", "en1993.1-8.6.2.tbend.block-bp", "en1993.1-8.6.2.mjrd.block-bp", "en1993.1-8.6.2.cone.block-bp"],
}

text = FEATURE.read_text(encoding="utf8")
n = max(int(x) for x in re.findall(r"\|\s*(?:pass|fail)-(?:v\d+-)?(\d+)\s*\|", text))
rows = []
for name, doc in FIXTURES.items():
    if "--write" in sys.argv:
        (DOCS / name).mkdir(exist_ok=True)
        (DOCS / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    out = json.loads(subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True, check=True).stdout)
    answers = {c["id"]: c for c in out["checks"]}
    for check in EXPECT[name]:
        c = answers[check]
        status = c.get("status") or ("pass" if c["utilization"] <= 1 else "fail")
        n += 1
        print(name, check, status, round(c["utilization"], 4))
        if f"| shared://📄️oracle-documents/{name}/🔣️.json | {check} |" not in text:
            rows.append(f"      | {'fail' if status == 'fail' else 'pass'}-v2-{n:02d} | shared://📄️oracle-documents/{name}/🔣️.json | {check} | {status} | {c['utilization']:.9g} | 1e-3 |")
print(len(rows), "new rows")
if "--write" in sys.argv and rows:
    FEATURE.write_text(text.rstrip("\n") + "\n" + "\n".join(rows) + "\n", encoding="utf8")
