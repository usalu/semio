import sys, os, re, collections
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/applied/"
SNAP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/orig/🌉️mcp/"
NEW = S + "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/"
REAL = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/"
def lines(root, files=None):
    c = collections.Counter()
    for dp, dn, fn in os.walk(root):
        for f in fn:
            if f.endswith(".rs") and not f.endswith(".orig"):
                p = os.path.join(dp, f)
                for ln in open(p, encoding="utf-8").read().split("\n"):
                    t = ln.strip()
                    if t: c[t] += 1
    return c
orig = lines(SNAP)
new = lines(NEW)
# new tree = staged overlay + untouched real files: compute using real tree lines for files not in staged
c2 = collections.Counter()
staged = set()
for dp, dn, fn in os.walk(NEW):
    for f in fn:
        if f.endswith(".rs"): staged.add(os.path.relpath(os.path.join(dp, f), NEW))
for dp, dn, fn in os.walk(REAL):
    for f in fn:
        if f.endswith(".rs") and not f.endswith(".orig"):
            rel = os.path.relpath(os.path.join(dp, f), REAL)
            if rel in staged or "generated" in rel: continue
            for ln in open(os.path.join(dp, f), encoding="utf-8").read().split("\n"):
                t = ln.strip()
                if t: c2[t] += 1
new = new + c2
only_o = orig - new; only_n = new - orig
def classify(t):
    if t.startswith("use ") or t.startswith("pub use ") or re.match(r'#\[cfg.*\]$', t): return "use/cfg"
    if t.startswith("#[path") or re.match(r'(pub )?mod \w+;', t): return "mount"
    if t.startswith("//!"): return "moddoc"
    if t.startswith("//#"): return "region"
    return "OTHER"
for name, d in (("only-orig", only_o), ("only-new", only_n)):
    cc = collections.Counter(); ex = collections.defaultdict(list)
    for t, n in d.items():
        k = classify(t); cc[k] += n
        if k == "OTHER": ex[k].append((n, t[:130]))
    print(name, dict(cc))
    for n, t in sorted(ex["OTHER"], reverse=True)[:25]: print("   ", n, t)

def norm(t):
    t = re.sub(r'\b(?:crate|super|self)::(?:\w+::)*', '', t)
    t = re.sub(r'\b[a-z_]+::(?=[A-Za-z_])', '', t)
    t = re.sub(r'pub\(\w+(?: \w+::\w+)?\) ?', '', t)
    t = re.sub(r'\bpub ', '', t)
    t = re.sub(r'\s+', ' ', t)
    t = re.sub(r'#\[cfg\(test\)\]', '', t)
    return t.strip()
no = collections.Counter(); nn = collections.Counter()
for t, n in only_o.items():
    if classify(t) == "OTHER": no[norm(t)] += n
for t, n in only_n.items():
    if classify(t) == "OTHER": nn[norm(t)] += n
lost = no - nn; gained = nn - no
print("UNMATCHED orig-only after normalization:", sum(lost.values()))
for t, n in sorted(lost.items(), key=lambda kv: -kv[1])[:40]: print("   -", n, t[:150])
print("UNMATCHED new-only after normalization:", sum(gained.values()))
for t, n in sorted(gained.items(), key=lambda kv: -kv[1])[:25]: print("   +", n, t[:150])
