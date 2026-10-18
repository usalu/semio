"""📖 c6fw relative text-reader census: every ts/tsx/mjs/cjs/js/py/json/sh file (tracked or untracked, ticket tree excluded) whose string literals resolve (new URL(rel, import.meta.url), resolve(dir, rel), join, or an absolute repo-relative suffix) to one of the split Rust roots. usage: readers2.py <tag>..."""
import os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio"
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
targets = {os.path.normpath(os.path.join(R, ft[t])): t for t in (sys.argv[1:] or ft)}
out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-co", "--exclude-standard", "--", "*.ts", "*.tsx", "*.mjs", "*.cjs", "*.js", "*.py", "*.json", "*.jsonc", "*.sh", "*.toml", ":!.🧬semio", ":!*Cargo.lock"], cwd=R, capture_output=True, text=True).stdout.split("\n")
LIT = re.compile(r'["`\']([^"`\'\n]*\.rs)["`\']')
hits = {}
for rel in out:
    if not rel or "node_modules" in rel or "/target/" in rel:
        continue
    p = os.path.join(R, rel)
    try:
        t = open(p, encoding="utf-8", errors="ignore").read()
    except Exception:
        continue
    if "🦀️.rs" not in t and ".rs" not in t:
        continue
    d = os.path.dirname(p)
    for ln, line in enumerate(t.split("\n"), 1):
        for m in LIT.finditer(line):
            s = m.group(1)
            cands = set()
            if s.startswith("."):
                cands.add(os.path.normpath(os.path.join(d, s)))
            if s.startswith("/"):
                cands.add(os.path.normpath(s))
            cands.add(os.path.normpath(os.path.join(R, s)))
            for c in cands:
                if c in targets:
                    hits.setdefault(targets[c], []).append((rel, ln, s))
            for tp, tag in targets.items():
                tr = tp[len(R) + 1:]
                if len(s) > 14 and not s.startswith(".") and tr.endswith(s.lstrip("/")) and s.count("/") >= 2:
                    hits.setdefault(tag, []).append((rel, ln, s))
for tag in (sys.argv[1:] or ft):
    h = sorted(set(hits.get(tag, [])))
    print(f"=== {tag}: {len(h)} reader literals")
    for rel, ln, s in h:
        print("   ", rel[-120:], ln, s[-80:])
