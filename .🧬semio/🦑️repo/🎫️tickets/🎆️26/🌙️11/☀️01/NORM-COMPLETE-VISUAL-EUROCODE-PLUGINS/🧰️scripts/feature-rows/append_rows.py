"""Append check-outcome rows (pass/fail by the oracle utilisation) to a code's check-outcome feature. Usage: append_rows.py <enXXXX> <specfile>; each spec line: fixture-uri<TAB>check-id. Idempotent: rows already present (fixture and check) are skipped."""
import glob, json, re, subprocess, sys

code, spec = sys.argv[1], sys.argv[2]
sub = glob.glob(f"/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/*{code}/🏅️standards/🔖️1/🪆️subsets/✳️any")[0]
feature = f"{sub}/🧪️tests/🚦️check-outcome-{code}-1/🥒️.feature"
oracle = f"{sub}/🔮️oracles/🚦️check-outcome/🐍️.py"
text = open(feature, encoding="utf8").read()
n = max(int(x) for x in re.findall(r"\|\s*(?:pass|fail)-(\d+)\s*\|", text))
rows, cache = [], {}
for line in open(spec, encoding="utf8"):
    if not line.strip():
        continue
    fixture, check = line.rstrip("\n").split("\t")
    if f"| {fixture} | {check} |" in text:
        continue
    path = sub + "/🧪️tests/🧫️fixtures/" + fixture.removeprefix("shared://")
    if path not in cache:
        out = subprocess.run([sys.executable, "-I", oracle, path], capture_output=True, text=True)
        cache[path] = {c["id"]: c["utilization"] for c in json.loads(out.stdout)["checks"]}
    u = cache[path][check]
    n += 1
    status = "pass" if u <= 1 else "fail"
    rows.append(f"      | {status}-{n:02d} | {fixture} | {check} | {status} | {u:.9g} | 1e-2 |")
text = open(feature, encoding="utf8").read()
if not text.endswith("\n"):
    text += "\n"
open(feature, "w", encoding="utf8").write(text + "\n".join(rows) + ("\n" if rows else ""))
print(len(rows), "rows appended to", feature)
