"""Switches a check-outcome case to the new oracle and appends rows computed by it: python -I add-rows.py <code> <rows-file> where each line is `fixture<TAB>check-regex-or-id<TAB>pass|fail|any`."""
import importlib.util, json, pathlib, re, sys

code, rows_file = sys.argv[1:3]
root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
sub = next(p for p in root.iterdir() if p.name.endswith(code)) / "🏅️standards/🔖️1/🪆️subsets/✳️any"
case = next((sub / "🧪️tests").glob("🚦️check-outcome-*"))
spec = importlib.util.spec_from_file_location("o", sub / "🔮️oracles/🚦️check-outcome/🐍️.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
py = case / "🐍️.py"
text = py.read_text(encoding="utf8")
text = text.replace('"🔮️oracles" / "⚖️compliance" / "🐍️.py"', '"🔮️oracles" / "🚦️check-outcome" / "🐍️.py"')
text = re.sub(r"def outcomes\(steps, path\):.*?\n\n\ndef adapter", '''def outcomes(steps, path):
    answers = steps.run_oracle(ORACLE, ["--json"], json.load(open(path, encoding="utf8")))
    return {row["id"]: (steps.pass_or_fail(row["utilization"]), row["utilization"]) for row in answers["checks"]}


def adapter''', text, flags=re.S)
py.write_text(text, encoding="utf8")
feature = case / "🥒️.feature"
body = feature.read_text(encoding="utf8").rstrip("\n").split("\n")
existing = {tuple(c.strip() for c in l.strip().strip("|").split("|"))[1:3] for l in body if l.strip().startswith(("| pass", "| fail"))}
n = len([l for l in body if l.strip().startswith(("| pass", "| fail"))])
new = []
for line in open(rows_file, encoding="utf8"):
    if not line.strip():
        continue
    fixture, check, want = line.rstrip("\n").split("\t")
    shared = "shared://" + fixture
    rows = oracle.evaluate(json.load(open(sub / "🧪️tests/🧫️fixtures" / fixture, encoding="utf8")))
    for key in [k for k in rows if re.fullmatch(check, k)]:
        if (shared, key) in existing:
            continue
        u = rows[key]
        status = "pass" if u <= 1 else "fail"
        if want != "any" and want != status:
            continue
        if 0.95 <= u <= 1.02:
            print("near-limit skipped", key, u)
            continue
        n += 1
        new.append(f"      | {status}-v2-{n:02d} | {shared} | {key} | {status} | {u:.9g} | 1e-3 |")
        existing.add((shared, key))
feature.write_text("\n".join(body + new) + "\n", encoding="utf8")
print(len(new), "rows added to", case.name)
