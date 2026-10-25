"""python3 moved.py <diff.txt> : resolves the changed `a/<alias>/<file>` inputs of a diff-execlogs report through every workspace's sources.json"""
import json, glob, re, sys
W = "/Users/ueli/Documents/semio/.cache/b/p/m/bazel/workspaces"
alias = {}
for f in glob.glob(W + "/*/*/sources.json"):
    try: d = json.load(open(f, encoding="utf-8"))
    except Exception: continue
    def walk(x):
        if isinstance(x, dict):
            for k, v in x.items():
                if re.fullmatch(r"[0-9a-f]{10}", k) and isinstance(v, str): alias[k] = v
                else: walk(v)
        elif isinstance(x, list):
            for v in x: walk(v)
    walk(d)
seen = set()
for line in open(sys.argv[1], encoding="utf-8"):
    m = re.match(r"\s+changed a/([0-9a-f]{10})/(.*)", line)
    if m and (m.group(1), m.group(2)) not in seen:
        seen.add((m.group(1), m.group(2))); print(alias.get(m.group(1), "?" + m.group(1)) + "/" + m.group(2).encode("latin1", "ignore").decode("utf-8", "ignore"))
