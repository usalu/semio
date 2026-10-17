"""🔬 Counts top-level arguments of retained-step API calls in tracked Rust files.

Usage: rsd.py <repo-root> <out-file>
"""
import re, subprocess, sys

ROOT = sys.argv[1]
OUT = sys.argv[2]
EXPECT = {"StepContext::new": 7, "StepBudget::new": 3, "StepBudget::from_duration": 4}
PAT = re.compile(r"\b(StepContext::new|StepBudget::new|StepBudget::from_duration)\s*\(")


def files():
    out = subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*.rs", ":!.🧬semio"], cwd=ROOT, capture_output=True).stdout.decode()
    return [f for f in out.split("\0") if f]


def split_args(src, i):
    depth = 0
    args = 0
    nonempty = False
    j = i
    in_str = None
    while j < len(src):
        c = src[j]
        if in_str:
            if c == "\\":
                j += 2
                continue
            if c == in_str:
                in_str = None
        elif c == '"':
            in_str = '"'
            nonempty = True
        elif c in "([{":
            depth += 1
            nonempty = True
        elif c in ")]}":
            if depth == 0:
                return args + (1 if nonempty else 0), j
            depth -= 1
        elif c == "<" and False:
            pass
        elif c == "," and depth == 0:
            args += 1
            nonempty = False
        elif not c.isspace():
            nonempty = True
        j += 1
    return None, j


rows = []
for f in files():
    try:
        src = open(f"{ROOT}/{f}", encoding="utf-8").read()
    except Exception:
        continue
    for m in PAT.finditer(src):
        n, _ = split_args(src, m.end())
        line = src.count("\n", 0, m.start()) + 1
        rows.append((f, line, m.group(1), n, EXPECT[m.group(1)]))
with open(OUT, "w") as o:
    for f, line, name, n, e in rows:
        o.write(f"{'BAD' if n != e else 'ok '}\t{name}\t{n}\t{f}:{line}\n")
print(len(rows), sum(1 for r in rows if r[3] != r[4]))
