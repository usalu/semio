"""📖 c6fw reader impact: for every reader file of a split root (readers2.py), the Rust-looking string literals of that file that occur in the ORIGINAL root text but no longer in the new root text (they moved to a child). usage: readers3.py stage|live <tag>..."""
import os, re, subprocess, sys, json
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio"
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
mode = sys.argv[1]
tags = sys.argv[2:]
def js_strings(src):
    out = []
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        if c == '/' and src.startswith('//', i):
            j = src.find('\n', i)
            i = n if j < 0 else j
            continue
        if c == '/' and src.startswith('/*', i):
            j = src.find('*/', i + 2)
            i = n if j < 0 else j + 2
            continue
        if c in '"\'`':
            q = c
            j = i + 1
            buf = []
            while j < n and src[j] != q:
                if src[j] == '\\' and j + 1 < n:
                    nx = src[j + 1]
                    buf.append({'n': '\n', 't': '\t', 'r': '\r'}.get(nx, nx))
                    j += 2
                    continue
                if q != '`' and src[j] == '\n':
                    break
                buf.append(src[j])
                j += 1
            out.append(''.join(buf))
            i = j + 1
            continue
        i += 1
    return out


def unesc(s):
    return s.replace('\\"', '"').replace("\\n", "\n").replace("\\\\", "\\")
out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-co", "--exclude-standard", "--", "*.ts", "*.tsx", "*.mjs", "*.cjs", "*.js", "*.py", "*.json", "*.jsonc", "*.sh", ":!.🧬semio", ":!*Cargo.lock"], cwd=R, capture_output=True, text=True).stdout.split("\n")
targets = {os.path.normpath(os.path.join(R, ft[t])): t for t in tags}
LIT = re.compile(r'["`\']([^"`\'\n]*\.rs)["`\']')
STR = re.compile(r'"((?:[^"\\\n]|\\.){6,})"')
readers = {}
for rel in out:
    if not rel or "node_modules" in rel:
        continue
    p = os.path.join(R, rel)
    try:
        t = open(p, encoding="utf-8", errors="ignore").read()
    except Exception:
        continue
    if ".rs" not in t:
        continue
    d = os.path.dirname(p)
    for m in LIT.finditer(t):
        s = m.group(1)
        c = os.path.normpath(os.path.join(d, s)) if s.startswith(".") else os.path.normpath(os.path.join(R, s))
        if c in targets:
            readers.setdefault((rel, targets[c]), None)
for (rel, tag), _ in sorted(readers.items()):
    orig = open(f"{HERE}/orig/{tag}.rs", encoding="utf-8").read()
    base = f"/Users/ueli/.cache/c6fw/stage/{tag}" if mode == "stage" else os.path.dirname(os.path.join(R, ft[tag]))
    new_root = open(base + "/🦀️.rs", encoding="utf-8").read()
    kids = "\n".join(open(os.path.join(dp, "🦀️.rs"), encoding="utf-8").read() for dp in [os.path.join(base, x) for x in sorted(os.listdir(base)) if os.path.isdir(os.path.join(base, x))] if os.path.exists(os.path.join(dp, "🦀️.rs")))
    t = open(os.path.join(R, rel), encoding="utf-8", errors="ignore").read()
    moved = []
    for s in js_strings(t):
        if len(s) < 6 or not re.search(r"[(){}:;=<>.]|fn |impl |struct |pub ", s):
            continue
        if s in orig and s not in new_root:
            moved.append(s)
    print(f"=== {rel[-100:]} [{tag}] moved-literals {len(moved)}")
    for s in moved[:12]:
        print("      ", repr(s[:100]), "-> child" if s in kids else "-> GONE")
