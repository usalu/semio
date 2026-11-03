"""EN 1993-1-10 Table 2.1 feature rows (v18): fixtures derived from the committed showcase (member-b1 S355 J2) and their check-outcome rows from the independent oracle (registry table lookup/interpolation of `check-outcome/🐍️.py`). `--write` writes the fixtures and the rows."""
import copy, json, pathlib, re, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIX = ROOT / "🧪️tests/🧫️fixtures"
DOCS = FIX / "📄️oracle-documents"
FEATURE = ROOT / "🧪️tests/🚦️check-outcome-en1993-1/🥒️.feature"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
showcase = json.load(open(FIX / "🎪️showcase/🔣️.json"))


def thick(d, tf):
    next(x for x in d["sections"] if x["id"] == "sec-heb240")["tf"] = tf
    return d


def ratio(d, value, tf):
    next(x for x in d["members"] if x["id"] == "member-b1")["toughnessStressRatio"] = value
    return thick(d, tf)


FIXTURES = {
    "toughness-fail": thick(copy.deepcopy(showcase), 0.07),
    "toughness-stress-pass": ratio(copy.deepcopy(showcase), 0.5, 0.04),
    "toughness-stress-fail": ratio(copy.deepcopy(showcase), 0.5, 0.06),
}
for doc, t_ed in ((FIXTURES["toughness-stress-pass"], -30.0), (FIXTURES["toughness-stress-fail"], -30.0)):
    next(x for x in doc["members"] if x["id"] == "member-b1")["designTemperature"] = t_ed
text = FEATURE.read_text(encoding="utf8")
n = max(int(x) for x in re.findall(r"\|\s*(?:pass|fail)-(?:v\d+-)?(\d+)\s*\|", text))
rows = []
for name, doc in FIXTURES.items():
    if "--write" in sys.argv:
        (DOCS / name).mkdir(exist_ok=True)
        (DOCS / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    out = json.loads(subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True, check=True).stdout)
    c = next(c for c in out["checks"] if c["id"] == "en1993.1-10.t.member-b1")
    status = c["status"]
    n += 1
    print(name, c["id"], status, round(c["utilization"], 4))
    if f"| shared://📄️oracle-documents/{name}/🔣️.json | {c['id']} |" not in text:
        rows.append(f"      | {'fail' if status == 'fail' else 'pass'}-v2-{n:02d} | shared://📄️oracle-documents/{name}/🔣️.json | {c['id']} | {status} | {c['utilization']:.9g} | 1e-3 |")
print(len(rows), "new rows")
if "--write" in sys.argv and rows:
    FEATURE.write_text(text.rstrip("\n") + "\n" + "\n".join(rows) + "\n", encoding="utf8")
