# 🔎 Production files of the build and caching libraries that no test file reaches through relative imports or path strings: reach.py
import os, re, subprocess, collections, posixpath
ROOT = "/Users/ueli/Documents/semio"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
N = lambda s: s.replace("️", "")
raw = subprocess.run(["git", "-c", "core.quotepath=off", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "🧰️framework"], cwd=ROOT, capture_output=True).stdout.decode().split("\0")
files = {N(f): os.path.join(ROOT, f) for f in raw if f and f.endswith((".ts", ".mjs", ".mts", ".js", ".cjs", ".py", ".json", ".sh"))}
bybase = collections.defaultdict(list)
for f in files:
    bybase[posixpath.basename(f)].append(f)
STR = re.compile(r"""["'`]([^"'`\n]*(?:\.ts|\.mjs|\.mts|\.js|\.json|\.py)|\.{1,2}/[^"'`\n]*)["'`]""")
cache = {}
def refs(f):
    if f in cache: return cache[f]
    text = N(open(files[f], encoding="utf8", errors="replace").read())
    out = set()
    d = posixpath.dirname(f)
    for m in STR.finditer(text):
        s = m.group(1)
        if "${" in s: continue
        if s.startswith("."):
            p = posixpath.normpath(posixpath.join(d, s))
            for cand in (p, p + ".ts", p + ".mjs", p + "/🟦.ts", p + "/🟨.mjs"):
                if cand in files: out.add(cand)
    cache[f] = out
    return out
seen = set(); stack = [f for f in files if "🧪tests" in f.split("/") or "🧫fixtures" in f.split("/")]
seen.update(stack)
while stack:
    f = stack.pop()
    if f not in files: continue
    for g in refs(f):
        if g not in seen: seen.add(g); stack.append(g)
LIBN = N(LIB)
res = []
for f in files:
    if f.startswith(LIBN + "/🏗build/") or f.startswith(LIBN + "/⚡caching/"):
        if f.split("/")[-1] in ("🟦.ts", "🟨.mjs", "📜script.ts") and "🧪tests" not in f.split("/") and f not in seen:
            res.append((len(open(files[f], encoding="utf8", errors="replace").read().splitlines()), f[len(LIBN) + 1:]))
res.sort(reverse=True)
for n, f in res: print(n, f)
print(len(res))
