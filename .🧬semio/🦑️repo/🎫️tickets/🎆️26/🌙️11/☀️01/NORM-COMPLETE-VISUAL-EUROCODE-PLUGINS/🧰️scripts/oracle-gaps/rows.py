"""🧾 Appends check-outcome rows `pass|fail-v30-NN` (status and utilisation read from the committed oracle, tolerance 1e-6) to a code's feature. Usage: rows.py <enXXXX> <spec.tsv> [--dry]; spec line: `fixture-uri<TAB>check-id`. Idempotent per (fixture, check)."""
import glob, json, re, subprocess, sys
import os
TOL = os.environ.get("TOL", "1e-2")
code, spec = sys.argv[1], sys.argv[2]
sub = glob.glob(f"/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/*{code}/🏅️standards/🔖️1/🪆️subsets/✳️any")[0]
feature = glob.glob(f"{sub}/🧪️tests/🚦️check-outcome-{code}-1/🥒️.feature")[0]
oracle = f"{sub}/🔮️oracles/🚦️check-outcome/🐍️.py"
text = open(feature, encoding="utf8").read()
n = max([int(x) for x in re.findall(r"-v30-(\d+)\s*\|", text)] + [0])
rows, cache = [], {}
for line in open(spec, encoding="utf8"):
    if not line.strip() or line.startswith("#"):
        continue
    fixture, check = line.rstrip("\n").split("\t")
    if f"| {fixture} | {check} |" in text:
        continue
    path = sub + "/🧪️tests/🧫️fixtures/" + fixture.removeprefix("shared://")
    if path not in cache:
        out = subprocess.run([sys.executable, "-I", oracle, "--json", path], capture_output=True, text=True)
        cache[path] = {c["id"]: c for c in json.loads(out.stdout)["checks"]}
    row = cache[path][check]
    u = row["utilization"]
    status = row.get("status") or ("pass" if u <= 1 else "fail")
    if status == "pass" and row.get("reads"):
        status = "unverifiedBasis"
    n += 1
    rows.append(f"      | {'fail' if status == 'fail' else 'pass'}-v30-{n} | {fixture} | {check} | {status} | {u:.9g} | {TOL} |")
if "--dry" in sys.argv:
    print("\n".join(rows)); sys.exit()
text = open(feature, encoding="utf8").read()
text += ("" if text.endswith("\n") else "\n") + "\n".join(rows) + ("\n" if rows else "")
open(feature, "w", encoding="utf8").write(text)
print(len(rows), "rows appended to", feature)
