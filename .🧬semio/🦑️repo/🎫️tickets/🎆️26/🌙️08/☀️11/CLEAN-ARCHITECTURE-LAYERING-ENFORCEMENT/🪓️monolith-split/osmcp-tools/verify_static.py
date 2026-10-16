import sys, os, re, collections, subprocess, json
sys.path.insert(0, ".")
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/applied/"
R = "/Users/ueli/Documents/semio/"
SNAP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/orig/"
MCPREL = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/"
rs = []
for dp, dn, fn in os.walk(S + MCPREL):
    for f in fn:
        if f.endswith(".rs"): rs.append(os.path.join(dp, f))
print("generated rs files:", len(rs))
# a. rustfmt parse
bad = 0
for p in rs:
    r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout"], input=open(p, encoding="utf-8").read(), capture_output=True, text=True)
    if r.returncode != 0:
        bad += 1; print("RUSTFMT FAIL", os.path.relpath(p, S)[-80:], r.stderr[:200].replace("\n", " "))
print("rustfmt failures:", bad)
# c. path resolution
miss = 0; maxlen = 0
for p in rs:
    t = open(p, encoding="utf-8").read()
    d = os.path.dirname(p)
    for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]', t):
        tgt = os.path.normpath(os.path.join(d, m.group(1)))
        real = tgt.replace(S, R) if tgt.startswith(S) else tgt
        if not (os.path.exists(tgt) or os.path.exists(tgt.replace(S, R))): miss += 1; print("MISSING path", os.path.relpath(p, S)[-70:], m.group(1)[:80])
    for m in re.finditer(r'(?:include!|include_str!|include_bytes!)\("([^"]+)"\)', t):
        tgt = os.path.normpath(os.path.join(d, m.group(1)))
        if not (os.path.exists(tgt) or os.path.exists(tgt.replace(S, R))): miss += 1; print("MISSING include", os.path.relpath(p, S)[-70:], m.group(1)[:80])
    maxlen = max(maxlen, len(os.path.relpath(p, S).encode()))
print("missing:", miss, "longest path bytes:", maxlen)
# e. region balance + presence
def regions(t):
    return re.findall(r'^\s*//#(end)?region[ \t]+(\S.*?)\s*$', t, re.M)
def bal(t):
    st = []
    for e, nm in regions(t):
        if e:
            if not st or st.pop() != nm: return False
        else: st.append(nm)
    return not st
unbal = [p for p in rs if not bal(open(p, encoding="utf-8").read())]
print("unbalanced files:", [os.path.relpath(p, S)[-60:] for p in unbal])
orig = collections.Counter(); new = collections.Counter()
for dp, dn, fn in os.walk(SNAP + MCPREL):
    for f in fn:
        if f.endswith(".rs"):
            for e, nm in regions(open(os.path.join(dp, f), encoding="utf-8").read()):
                if not e: orig[nm] += 1
for p in rs:
    for e, nm in regions(open(p, encoding="utf-8").read()):
        if not e: new[nm] += 1
print("region names missing in output:", [k for k in orig if k not in new][:20], "orig", sum(orig.values()), "new", sum(new.values()))
