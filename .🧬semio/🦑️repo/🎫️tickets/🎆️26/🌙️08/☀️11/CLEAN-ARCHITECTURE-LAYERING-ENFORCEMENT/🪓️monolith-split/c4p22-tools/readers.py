"""🔎 c4p22: list every file under the three artifact families that names one of the 7 split roots (include_str!, #[path], include!, relative string literals).
usage: readers.py"""
import os, re, subprocess
R = "/Users/ueli/Documents/semio/"
A = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
here = os.path.dirname(os.path.abspath(__file__))
roots = {os.path.normpath(R + l.strip()) for l in open(here + "/files.txt") if l.strip()}
fams = [A + "/🗽️obj", A + "/🧊️gltf", A + "/🧱️ply"]
out = subprocess.run(["git", "-C", R, "-c", "core.quotepath=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--"] + fams, capture_output=True, text=True).stdout
files = [f for f in out.split("\0") if f]
rx = re.compile(r'"((?:\.\.?/)[^"\n]*?)"|`((?:\.\.?/)[^`\n]*?)`|\'((?:\.\.?/)[^\'\n]*?)\'')
hits = 0
for rel in files:
    if not rel.endswith((".rs", ".ts", ".tsx", ".json", ".md", ".semio", ".feature", ".toml", ".jsonc", ".proto", ".graphql")):
        continue
    p = R + rel
    try:
        t = open(p, encoding="utf-8").read()
    except Exception:
        continue
    for i, line in enumerate(t.split("\n"), 1):
        for m in rx.finditer(line):
            lit = next(g for g in m.groups() if g)
            tgt = os.path.normpath(os.path.join(os.path.dirname(p), lit))
            if tgt in roots:
                hits += 1
                print(f"{rel[-90:]}:{i}: {line.strip()[:200]}")
print("HITS", hits, "files scanned", len(files))
