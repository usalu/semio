"""🏷️ Renames mutation kind strings to the derive's to_kebab(variant) for one artifact (content and folder names)."""
import os, re, sys, subprocess
T = os.path.dirname(os.path.abspath(__file__))
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
art = sys.argv[1]
out = subprocess.run([sys.executable, "-I", f"{T}/scan.py"], capture_output=True, text=True).stdout
m = {}
for l in out.splitlines():
    a, _, o, n = l.split("\t")
    if art in a: m[o] = n
if not m: sys.exit("nothing")
rx = re.compile(r"(?<![A-Za-z0-9_-])(" + "|".join(re.escape(k) for k in sorted(m, key=len, reverse=True)) + r")(?![A-Za-z0-9_-])")
files = 0
for d, ds, fs in os.walk(ROOT):
    ds[:] = [x for x in ds if x not in ("target", "node_modules", ".git")]
    for f in fs:
        p = os.path.join(d, f)
        if os.path.islink(p): continue
        try: t = open(p, encoding="utf-8").read()
        except Exception: continue
        n = rx.sub(lambda g: m[g.group(1)], t)
        if n != t: open(p, "w", encoding="utf-8").write(n); files += 1
ren = 0
for d, ds, fs in os.walk(ROOT, topdown=False):
    for x in ds + fs:
        for o, n in m.items():
            if x.endswith(o) and (len(x) == len(o) or not re.match(r"[A-Za-z0-9_-]", x[-len(o)-1])):
                os.rename(os.path.join(d, x), os.path.join(d, x[:-len(o)] + n)); ren += 1; break
print(art, len(m), "files", files, "renamed", ren)
