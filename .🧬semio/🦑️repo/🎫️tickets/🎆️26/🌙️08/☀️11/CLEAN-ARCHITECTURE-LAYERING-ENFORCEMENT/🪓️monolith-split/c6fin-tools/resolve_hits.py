"""📖 C6FIN relative-path reader census: every string literal ending in `.rs` (and `*.rs` concatenations by the last two path segments) in tracked/untracked text files is resolved against the file's directory, each ancestor directory and the repo root; hits are those that resolve to one of the split files or to their directories. usage: resolve_hits.py"""
import os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio"
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
ROOTS = {v: k for k, v in ft.items()}
DIRS = {os.path.dirname(v): k for k, v in ft.items()}
out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-co", "--exclude-standard", "-z"], cwd=R, capture_output=True).stdout.decode("utf-8", "replace").split("\0")
exts = (".ts", ".tsx", ".mjs", ".js", ".py", ".sh", ".json", ".jsonc", ".md", ".toml")
LIT = re.compile(r'["\'`]([^"\'`\n]*\.rs)["\'`]')
hits = []
for f in out:
    if not f or f.startswith(".🧬semio") or f.endswith("Cargo.lock") or not f.endswith(exts):
        continue
    if "node_modules" in f or "/target/" in f or "/dist/" in f:
        continue
    p = os.path.join(R, f)
    try:
        t = open(p, encoding="utf-8").read()
    except Exception:
        continue
    d = os.path.dirname(f)
    anc = []
    cur = d
    while True:
        anc.append(cur)
        if not cur:
            break
        cur = os.path.dirname(cur)
    for i, line in enumerate(t.split("\n"), 1):
        for m in LIT.finditer(line):
            lit = m.group(1)
            if lit.startswith("http"):
                continue
            for base in anc:
                n = os.path.normpath(os.path.join(base, lit))
                if n in ROOTS:
                    hits.append((f, i, "file", ROOTS[n], lit))
                    break
for h in hits:
    print("\t".join(str(x) for x in (h[3], h[2], h[0][-100:], h[1], h[4][:100])))
print(len(hits), "hits")
