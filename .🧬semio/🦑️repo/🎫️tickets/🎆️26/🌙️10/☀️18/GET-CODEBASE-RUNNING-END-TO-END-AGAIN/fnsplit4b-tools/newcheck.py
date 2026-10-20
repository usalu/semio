"""🧪 Shape check of new split files: usage newcheck.py <file>...  (repo-relative or absolute). Reports path bytes (relative to the repo root), line count, docstring emoji equals the folder emoji, longest function in the file, and duplicate fn doc emoji inside one file."""
import os, re, sys
ROOT = "/Users/ueli/Documents/semio/"
bad = 0
for f in sys.argv[1:]:
    rel = f[len(ROOT):] if f.startswith(ROOT) else f
    text = open(ROOT + rel, encoding="utf8").read()
    lines = text.count("\n") + 1
    size = len(rel.encode("utf-16-le")) // 2
    comp = max(len(c.encode("utf8")) for c in rel.split("/"))
    folder = os.path.basename(os.path.dirname(rel))
    fe = folder.split("️")[0] if "️" in folder else folder[:1]
    m = re.match(r"^(?://!|/\*\*)\s*(\S+)", text)
    head = m.group(1) if m else ""
    docs = re.findall(r"^[ \t]*(?:///|/\*\*)[ \t]*(\S+)", text, re.M)
    docs = [d.replace("️", "") for d in docs]
    dup = sorted({d for d in docs if docs.count(d) > 1})
    probs = []
    if size > 255: probs.append("path %d UTF-16 units" % size)
    if comp > 255: probs.append("component %d bytes" % comp)
    if rel.endswith(("/🦀️.rs", "/🟦️.ts")) and head.replace("️", "") != fe.replace("️", "") and folder != "":
        probs.append("head emoji %r != folder %r" % (head, fe))
    if dup: probs.append("duplicate doc emoji %s" % dup)
    bad += bool(probs)
    print("%4d lines %3d bytes  %s %s" % (lines, size, rel.split("/")[-2][:40], "; ".join(probs) or "ok"))
print("problems:", bad)
