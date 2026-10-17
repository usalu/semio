import os, re, sys
# 🧪️ fixtests.py <kept-manifest> <part-manifest>... : moves the case mounts of a split family's [[test]] target files to the part crates that own the leaves (header copied, mount paths rebased); drops stanzas that end up empty.
R = "/Users/ueli/Documents/semio"
PATH_RX = re.compile(r'#\[path\s*=\s*"([^"]*)"\]')
kept = os.path.join(R, sys.argv[1])
parts = [os.path.join(R, p) for p in sys.argv[2:]]
def stanzas(txt):
    return list(re.finditer(r'\[\[test\]\]\n(?:[^\n\[][^\n]*\n)+', txt))
def leafdirs_of(manifest):
    mt = open(manifest).read()
    lp = re.search(r'\[lib\][^\[]*?path = "([^"]+)"', mt).group(1)
    root = os.path.normpath(os.path.join(os.path.dirname(manifest), lp))
    txt = open(root).read()
    paths = [os.path.normpath(os.path.join(os.path.dirname(root), m.group(1))) for m in PATH_RX.finditer(txt) if m.group(1) != "."]
    dirs = set()
    for p in paths:
        if os.path.basename(p) == "🦀️.rs" and "🔺️diff" not in p and "↩️inverse" not in p and "🦠️mutation" not in p:
            dirs.add(os.path.dirname(p))
    return dirs
kmt = open(kept).read()
kst = stanzas(kmt)
for st in kst:
    name = re.search(r'name = "([^"]+)"', st.group(0)).group(1)
    rel = re.search(r'path = "([^"]+)"', st.group(0)).group(1)
    tf = os.path.normpath(os.path.join(os.path.dirname(kept), rel))
    lines = open(tf).read().split("\n")
    first = next(i for i, l in enumerate(lines) if l.startswith("#[path"))
    header = lines[:first]
    body = lines[first:]
    blocks = []
    i = 0
    while i < len(body):
        if not body[i].strip():
            i += 1
            continue
        assert body[i].startswith("#[path") and re.match(r"(pub )?mod \w+;", body[i + 1]), body[i:i + 2]
        blocks.append(body[i:i + 2])
        i += 2
    remaining = list(blocks)
    for pm in parts:
        owned = leafdirs_of(pm)
        mine = []
        for b in remaining:
            tgt = os.path.normpath(os.path.join(os.path.dirname(tf), PATH_RX.search(b[0]).group(1)))
            if any(tgt.startswith(d + os.sep) for d in owned):
                mine.append(b)
        remaining = [b for b in remaining if b not in mine]
        ptxt = open(pm).read()
        pst = [s for s in stanzas(ptxt) if re.search(r'name = "%s"' % re.escape(name), s.group(0))]
        assert len(pst) == 1, (pm, name)
        prel = re.search(r'path = "([^"]+)"', pst[0].group(0)).group(1)
        if not mine:
            ptxt = ptxt.replace(pst[0].group(0), "")
            ptxt = re.sub(r"\n{3,}", "\n\n", ptxt).rstrip("\n") + "\n"
            open(pm, "w").write(ptxt)
            print(f"{os.path.basename(os.path.dirname(os.path.dirname(os.path.dirname(pm))))}: stanza {name} dropped (no cases)")
            continue
        ptf = os.path.normpath(os.path.join(os.path.dirname(pm), prel))
        os.makedirs(os.path.dirname(ptf), exist_ok=True)
        out = []
        for b in mine:
            tgt = os.path.normpath(os.path.join(os.path.dirname(tf), PATH_RX.search(b[0]).group(1)))
            out += ['#[path = "%s"]' % os.path.relpath(tgt, os.path.dirname(ptf)), b[1]]
        hdr = list(header)
        open(ptf, "w").write("\n".join(hdr).rstrip("\n") + "\n\n" + "\n".join(out) + "\n")
        print(f"{name}: {len(mine)} cases -> {os.path.relpath(ptf, R)[-60:]}")
    open(tf, "w").write("\n".join(header).rstrip("\n") + "\n\n" + "\n".join(l for b in remaining for l in b) + "\n")
    print(f"{name}: {len(remaining)} cases stay in {os.path.relpath(tf, R)[-60:]}")
