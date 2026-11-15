"""🔢 Count notation variants of a symbol family across the energy tree."""
import collections
import os
import re
import sys

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy"
SKIP = {".venv", "site-packages", "__pycache__", ".ruff_cache"}
PAT = re.compile(open(sys.argv[1], encoding="utf-8").read().strip())
counts = collections.Counter()
where = collections.defaultdict(set)
for d, ds, fs in os.walk(ROOT):
    ds[:] = [x for x in ds if x not in SKIP]
    for f in fs:
        if os.path.splitext(f)[1] not in {".py", ".md", ".json"}:
            continue
        path = os.path.join(d, f)
        try:
            text = open(path, encoding="utf-8").read()
        except Exception:
            continue
        for m in PAT.finditer(text):
            counts[m.group(0)] += 1
            where[m.group(0)].add(os.path.relpath(path, ROOT)[:70])
sys.stdout.reconfigure(encoding="utf-8")
for key, n in counts.most_common():
    print(n, repr(key), sorted(where[key])[:3])
