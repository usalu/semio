"""Verifies (1) test-name multisets original vs current for every snapshotted .rs, (2) sha256 of every fixture of tc1-sites-before.tsv, (3) rustfmt parse of every touched .rs."""
import collections, hashlib, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = "/Users/ueli/Documents/semio"
GEN = os.path.join(HERE, "..", "🗑️generated")
pat = re.compile(r'#\[(?:test|[\w:]*async_test)\]\s*(?:#\[[^\]]*\]\s*)*(?:async\s+)?fn\s+(\w+)')
bad = 0; total = 0; files = []
for line in open(f"{HERE}/orig/index.tsv"):
    key, path = line.rstrip("\n").split("\t")
    if not path.endswith(".rs"): continue
    a = pat.findall(open(f"{HERE}/orig/{key}.rs", encoding="utf-8").read())
    b = pat.findall(open(path, encoding="utf-8").read())
    total += len(a); files.append(path)
    if collections.Counter(a) != collections.Counter(b):
        bad += 1; print("TEST NAME DIFF", path[-80:], collections.Counter(a) - collections.Counter(b), collections.Counter(b) - collections.Counter(a))
print("test-name multisets: files", len(files), "tests before", total, "files differing", bad)
shabad = 0; n = 0
for row in open(f"{GEN}/tc1-sites-before.tsv"):
    c = row.rstrip("\n").split("\t")
    if len(c) < 7 or not c[5]: continue
    art = c[0].split("/🔮️oracles")[0].split("/🏅️standards")[0]
    art = c[0][:c[0].index("/", c[0].index("/🗿️artifacts/") + 14)]
    tgt = os.path.join(ROOT, art, c[3]); n += 1
    if hashlib.sha256(open(tgt, "rb").read()).hexdigest() != c[5]: shabad += 1; print("SHA DIFF", tgt)
print("fixture sha256: sites", n, "differing", shabad)
rf = 0
for p in files:
    r = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout", p], capture_output=True, text=True)
    if r.returncode != 0 and "error" in r.stderr.lower().split("\n")[0:1][0:1].__str__().lower() or "error:" in r.stderr:
        rf += 1; print("RUSTFMT", p[-70:], r.stderr[:300])
print("rustfmt parse: files", len(files), "failing", rf)
