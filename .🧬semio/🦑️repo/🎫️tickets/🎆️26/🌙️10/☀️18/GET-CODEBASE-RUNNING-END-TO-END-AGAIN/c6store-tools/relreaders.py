"""🔎 C6STORE relative-path reader census: every string literal ending in `🦀️.rs` of a tracked TS law is resolved against the law's directory (exact match) and, when it names at least two path segments, by suffix against the split files. usage: relreaders.py"""
import os, re, subprocess, collections
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio/"
ft = {t: p for t, p in (l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8")) if t != "sunitm"}
rel2tag = {p: t for t, p in ft.items()}
files = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "grep", "-l", "🦀️.rs", "--", "*.ts", "*.tsx", "*.mjs", "*.py", ":!.🧬semio"], cwd=R, capture_output=True, text=True).stdout.split("\n")
LIT = re.compile(r'(["\'`])((?:(?!\1)[^\\\n]|\\.)*🦀️\.rs)\1')
out = collections.defaultdict(list)
for f in files:
    if not f or not os.path.isfile(R + f): continue
    d = os.path.dirname(f)
    for ln, line in enumerate(open(R + f, encoding="utf-8").read().split("\n"), 1):
        for m in LIT.finditer(line):
            lit = m.group(2)
            cands = []
            if lit.startswith("."):
                cands.append(os.path.normpath(os.path.join(d, lit)))
            elif "/" in lit and not lit.startswith("/"):
                cands.append(os.path.normpath(lit))
            for c in cands:
                if c in rel2tag:
                    out[f].append((ln, rel2tag[c], "exact", lit)); break
            else:
                segs = [s for s in lit.split("/") if s not in ("..", ".")]
                if len(segs) >= 2:
                    suf = "/".join(segs)
                    for p, t in rel2tag.items():
                        if p.endswith("/" + suf) and (t not in ("str",) or len(segs) <= 2):
                            out[f].append((ln, t, "suffix", lit)); break
for f, rows in sorted(out.items()):
    for ln, t, kind, lit in rows:
        print(f"{'/'.join(f.split('/')[-3:])[:70]}:{ln} [{t}/{kind}] {lit[-70:]}")
