"""🧾 Runs the EN 1991 catalogue oracle on every catalogue feature row and compares with the hand-checked expected value of the row."""
import pathlib, subprocess, sys
B = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any")
rows = []
inp = False
for line in (B / "🧪️tests/🏋️en1991-catalogue/🥒️.feature").read_text(encoding="utf8").splitlines():
    line = line.strip()
    if line.startswith("Examples:"): inp = True; continue
    if not inp or not line.startswith("|"): continue
    c = [x.strip() for x in line.strip("|").split("|")]
    if c[0] == "id": continue
    rows.append(c)
stdin = ""
for i, c in enumerate(rows):
    toks = [("s:" + p.replace(":", "=", 1)) if "=" not in p else (("v:" if "," in p else "n:") + p) for p in c[3].split(";") if p]
    import re, collections
    vec = collections.defaultdict(list)
    plain = []
    for tk in toks:
        m = re.match(r"n:(\w+)\.(\d+)=(.*)", tk)
        (vec[m.group(1)].append(m.group(3)) if m else plain.append(tk))
    toks = plain + ["v:%s=%s" % (k, ",".join(v)) for k, v in vec.items()]
    stdin += "|".join([c[1], str(i), "annex=" + c[2]] + toks) + "\n"
res = subprocess.run([sys.executable, "-I", str(B / "🔮️oracles/⚖️compliance/🐍️.py"), "--catalogue"], input=stdin, capture_output=True, text=True)
print(res.stderr[-3000:])
got = {}
for l in res.stdout.splitlines():
    e, i, k, v = l.split("|"); got[(int(i), k)] = float(v)
bad = miss = ok = 0
for i, c in enumerate(rows):
    want, tol = float(c[5]), float(c[6])
    if (i, c[4]) not in got: miss += 1; continue
    g = got[(i, c[4])]
    if abs(g - want) > tol * max(abs(want), 1e-12): bad += 1; print("MISMATCH", c[0], c[1], c[4], "oracle", g, "feature", want)
    else: ok += 1
print("rows", len(rows), "ok", ok, "mismatch", bad, "no-oracle", miss, "exit", res.returncode)
