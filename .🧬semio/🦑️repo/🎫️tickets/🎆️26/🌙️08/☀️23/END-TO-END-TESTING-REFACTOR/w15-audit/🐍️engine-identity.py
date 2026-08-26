"""🔎 Reproduce the pre-refactor 📕️norm oracle identity claim: strip each old adapter's per-subset
data block, docstrings and comments, then hash what is left. Run from w15-work/old-adapters/."""
import re, hashlib, glob
h = {}
for p in sorted(glob.glob("*.py")):
    lines, out, skip = open(p, encoding="utf-8").read().splitlines(), [], False
    for l in lines:
        if l.strip().startswith("# region 🔖️Vocabulary"): skip = True; continue
        if l.strip().startswith("# endregion 🔖️Vocabulary"): skip = False; continue
        if not skip: out.append(l)
    body = re.sub(r'"""(?:.|\n)*?"""', '', "\n".join(out))
    body = re.sub(r'(?m)^\s*#.*$', '', body)
    body = re.sub(r'\s+', ' ', body).strip()
    h.setdefault(hashlib.sha256(body.encode()).hexdigest()[:16], []).append(p.replace('.py', ''))
for k, v in sorted(h.items(), key=lambda kv: -len(kv[1])): print(k, len(v), sorted(v))
print("distinct engines among the 15 OLD adapters:", len(h))
