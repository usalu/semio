"""Writes the append_rows spec for the check families of one code that lack a pass or a fail row: picks per family and missing status the check farthest from the limit band (pass <= 0.9, fail >= 1.1) over all check-outcome fixtures. Usage: spec_families.py <enXXXX> <instance-regex> <out.tsv> [--report]"""
import glob, json, math, re, subprocess, sys

code, instance, out = sys.argv[1], re.compile(sys.argv[2]), sys.argv[3]
sub = glob.glob(f"/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/*{code}/🏅️standards/🔖️1/🪆️subsets/✳️any")[0]
feature = open(glob.glob(sub + "/🧪️tests/🚦️check-outcome-*/🥒️.feature")[0], encoding="utf8").read()
fam = lambda i: instance.sub("", i)
have = {}
for status, check in re.findall(r"\|\s*(pass|fail)-\d+\s*\|\s*\S+\s*\|\s*(\S+)\s*\|", feature):
    have.setdefault(fam(check), set()).add(status)
cand = {}
for path in sorted(glob.glob(sub + "/🧪️tests/🧫️fixtures/🚦️check-outcome/*/🔣️.json")):
    run = subprocess.run(["/home/deck/semio/.venv/bin/python", "-I", sub + "/🔮️oracles/🚦️check-outcome/🐍️.py", path], capture_output=True, text=True)
    for c in json.loads(run.stdout)["checks"]:
        u = c["utilization"]
        status = "fail" if u > 1 else "pass"
        if (status == "pass" and 0 <= u <= 0.9 or status == "fail" and 1.1 <= u < 1e7) and not math.isnan(u):
            cand.setdefault((fam(c["id"]), status), []).append((u, path, c["id"]))
lines, missing = [], []
families = {f for f, _ in cand} | set(have)
for f in sorted(families):
    for status in ("pass", "fail"):
        if status in have.get(f, ()):
            continue
        options = cand.get((f, status))
        if not options:
            missing.append((f, status))
            continue
        u, path, cid = max(options, key=lambda o: (("purlin" in o[2] or "bldg" in o[2]), o[0] if status == "pass" else -o[0]))
        lines.append(f"shared://{path.split('/🧫️fixtures/')[1]}\t{cid}")
open(out, "w", encoding="utf8").write("\n".join(lines) + "\n")
print(len(lines), "rows planned; no fixture for:", missing)
