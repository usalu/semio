"""EN 1998 wave 3d: appends the Gherkin rows of the isolation, pier base and liquefaction hazard checks to the EN 1998 check-outcome feature; the expected status and utilisation come from the independent oracle. Idempotent (rows are keyed by their id).
Usage: /home/deck/semio/.venv/bin/python -I 4-feature-rows.py"""
import json, pathlib, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any")
FEATURE = ROOT / "🧪️tests/🚦️check-outcome-en1998-1/🥒️.feature"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
FIXTURES = ["isolation-pass", "isolation-fail", "hazard-pass", "hazard-fail"]
WANTED = ("isolation.iso-1.", ".base.pb-1.", "fd-1.layer.", "fd-1.spreading", "fd-1.pile.liquefied", "fd-1.profile.shearModulus")
text = FEATURE.read_text(encoding="utf8")
seen, lines = set(), []
for name in FIXTURES:
    document = json.load(open(ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome" / name / "🔣️.json", encoding="utf8"))
    answers = json.loads(subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(document), capture_output=True, text=True, check=True).stdout)["checks"]
    for r in answers:
        if not any(w in r["id"] for w in WANTED) or (name, r["id"]) in seen:
            continue
        seen.add((name, r["id"]))
        if name.startswith("hazard") and "isolation" in r["id"] or name.startswith("isolation") and not ("isolation" in r["id"] or ".base." in r["id"]):
            continue
        number = len(lines) + 1
        row_id = f"{'fail' if r['status'] == 'fail' else 'pass'}-v31-{number:02d}"
        line = f"      | {row_id} | shared://🚦️check-outcome/{name}/🔣️.json | {r['id']} | {r['status']} | {r['utilization']:.9g} | 1e-2 |"
        if f"| {r['id']} |" in text and f"/{name}/🔣️.json | {r['id']} |" in text:
            continue
        lines.append(line)
if lines:
    FEATURE.write_text(text.rstrip("\n") + "\n" + "\n".join(lines) + "\n", encoding="utf8")
print(len(lines), "rows appended")
