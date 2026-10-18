"""🔁 Reports duplicate top-level `use` imports (same bound name imported twice, E0252) in files changed versus HEAD under the plugin and dev trees.
usage: dupuse.py [--fix]   (--fix deletes the later exact-duplicate single-name `use` line only)"""
import re, subprocess, sys, os, collections
R = "/Users/ueli/Documents/semio"
fix = "--fix" in sys.argv
files = subprocess.run(["git", "-c", "core.fsmonitor=false", "diff", "--name-only", "HEAD", "--", "✏️s/🔌️plugins", "✏️s/🧑‍💻dev"], cwd=R, capture_output=True, text=True).stdout.split("\n")
files = [f for f in files if f.endswith(".rs")]
def flatten(prefix, body):
    out = []; depth = 0; cur = ""; parts = []
    for ch in body:
        if ch == "{": depth += 1
        if ch == "}": depth -= 1
        if ch == "," and depth == 0: parts.append(cur); cur = ""
        else: cur += ch
    if cur.strip(): parts.append(cur)
    for p in parts:
        p = p.strip()
        if not p: continue
        m = re.match(r"^(.*?)::\{(.*)\}$", p, re.S)
        if m: out += flatten(prefix + m.group(1) + "::", m.group(2))
        else: out.append(prefix + p)
    return out
bad = 0
for f in files:
    try: L = open(os.path.join(R, f), encoding="utf-8").read().split("\n")
    except Exception: continue
    seen = collections.defaultdict(list)
    for i, l in enumerate(L):
        m = re.match(r"^use (.*);$", l)
        if not m or (i > 0 and L[i - 1].lstrip().startswith("#[")): continue
        body = m.group(1)
        if body.endswith("::*") and "{" not in body: continue
        items = flatten("", "{" + body + "}") if "{" in body else [body]
        for it in items:
            if it.endswith("::*") or it.endswith(" as _") or it.endswith("::self"): continue
            name = it.split(" as ")[-1].split("::")[-1].strip()
            seen[name].append((i, it))
    for name, occ in seen.items():
        paths = collections.Counter(o[1] for o in occ)
        occ = [o for o in occ if paths[o[1]] > 1]
        if len(occ) > 1:
            bad += 1
            print(f.split("artifacts/")[-1][-70:], name, [o[0] + 1 for o in occ], [o[1][:60] for o in occ][:2])
            if fix:
                for i, it in occ[1:]:
                    if re.match(r"^use [A-Za-z0-9_:]+;$", L[i]) and L[i] == f"use {it};": L[i] = None
    if fix and None in L:
        open(os.path.join(R, f), "w", encoding="utf-8").write("\n".join(l for l in L if l is not None))
print("duplicates", bad)
