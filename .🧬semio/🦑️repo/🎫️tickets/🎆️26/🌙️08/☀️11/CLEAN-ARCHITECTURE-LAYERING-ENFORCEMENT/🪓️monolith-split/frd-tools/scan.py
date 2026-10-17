import os, re, sys, json, collections
ROOT = "/Users/ueli/Documents/semio"
F = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split")
G = os.path.join(F, "🗑️generated")
files = []
for name in ("frd-tracked-rs.txt", "frd-untracked-rs.txt"):
    with open(os.path.join(G, name), encoding="utf-8") as fh:
        files += [l.rstrip("\n") for l in fh if l.strip()]
files = sorted(set(files))
rows = []
for rel in files:
    p = os.path.join(ROOT, rel)
    try:
        t = open(p, encoding="utf-8").read()
    except (FileNotFoundError, IsADirectoryError):
        continue
    if "CARGO_MANIFEST_DIR" in t or "runtime_fixture" in t:
        rows.append((rel, t))
print(len(rows), "files with CARGO_MANIFEST_DIR or runtime_fixture", file=sys.stderr)
json.dump([r[0] for r in rows], open(os.path.join(G, "frd-files.json"), "w"), ensure_ascii=False)
fnre = re.compile(r'^([ \t]*)((?:pub(?:\([^)]*\))? )?(?:const )?fn ([A-Za-z0-9_]+)\s*\([^)]*\)[^{;]*)\{', re.M)
shapes = collections.Counter(); where = collections.defaultdict(list)
for rel, t in rows:
    for m in fnre.finditer(t):
        # find body end by brace matching (approximate, ignoring strings)
        i = m.end(); depth = 1
        while i < len(t) and depth:
            c = t[i]
            if c == '{': depth += 1
            elif c == '}': depth -= 1
            i += 1
        body = t[m.end():i-1]
        if "CARGO_MANIFEST_DIR" in body:
            sig = re.sub(r'\s+', ' ', m.group(2).strip())
            norm = re.sub(r'\s+', ' ', body.strip())
            key = (m.group(3), sig, norm[:400])
            shapes[key] += 1
            where[key].append(rel)
out = []
for key, n in shapes.most_common():
    out.append({"count": n, "name": key[0], "sig": key[1], "body": key[2], "sample": where[key][0]})
json.dump(out, open(os.path.join(G, "frd-shapes.json"), "w"), ensure_ascii=False, indent=1)
for o in out[:60]:
    print(o["count"], o["name"], "|", o["sig"][:80], "|", o["body"][:230])
print(len(out), "shapes")
