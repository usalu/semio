"""🚦 Re-derives the status column of a code's check-outcome rows from the committed oracle (kernel rule: a pass citing an unverified NDP is unverifiedBasis). Usage: rederive_status.py <enXXXX> [--write]"""
import glob, json, re, subprocess, sys
code, write = sys.argv[1], "--write" in sys.argv
sub = glob.glob(f"/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/*{code}/🏅️standards/🔖️1/🪆️subsets/✳️any")[0]
path = glob.glob(f"{sub}/🧪️tests/🚦️check-outcome-{code}-1/🥒️.feature")[0]
lines = open(path, encoding="utf8").read().split("\n")
pattern = re.compile(r"^(\s*\|\s*)((?:pass|fail)-\S+)(\s*\|\s*)shared://(\S+)(\s*\|\s*)(\S+)(\s*\|\s*)(\S+)(\s*\|.*)$")
cache, changed = {}, 0
for n, line in enumerate(lines):
    m = pattern.match(line)
    if not m:
        continue
    fx, chk, st = m.group(4), m.group(6), m.group(8)
    p = f"{sub}/🧪️tests/🧫️fixtures/{fx}"
    if p not in cache:
        r = subprocess.run([sys.executable, "-I", f"{sub}/🔮️oracles/🚦️check-outcome/🐍️.py", p], capture_output=True, text=True)
        cache[p] = {c["id"]: c for c in json.loads(r.stdout)["checks"]}
    c = cache[p]
    if chk not in c or st not in ("pass", "unverifiedBasis"):
        continue
    status = c[chk].get("status") or ("pass" if c[chk]["utilization"] <= 1 else "fail")
    if status != st and status in ("pass", "unverifiedBasis"):
        lines[n] = f"{m.group(1)}{m.group(2)}{m.group(3)}shared://{fx}{m.group(5)}{chk}{m.group(7)}{status}{m.group(9)}"
        changed += 1
        print("ROW", m.group(2), chk, st, "->", status)
if write and changed:
    open(path, "w", encoding="utf8").write("\n".join(lines))
print(code, changed, "rows changed")
