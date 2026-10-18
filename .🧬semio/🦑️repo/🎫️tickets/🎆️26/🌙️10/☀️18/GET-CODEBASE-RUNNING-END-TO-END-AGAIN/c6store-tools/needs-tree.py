"""🔎 C6STORE reader impact: for every reader file that names a split file (hits list), the string literals of the reader that occur in the ORIGINAL text of the split file but no longer in its live root (they moved into children). usage: needs-tree.py"""
import re, os, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio/"
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
pat2tag = {}
for tag, rel in ft.items():
    parts = rel.split("/")
    pat2tag.setdefault("/".join(parts[-4:]), []).append(tag)
live = {}
for tag, rel in ft.items():
    if tag == "sunitm": continue
    live[tag] = open(R + rel, encoding="utf-8").read()
orig = {tag: open(f"{HERE}/orig/{tag}.rs", encoding="utf-8").read() for tag in ft if tag != "sunitm"}
LIT = re.compile(r'"((?:[^"\\\n]|\\.)*)"|`((?:[^`\\]|\\.)*)`|\'((?:[^\'\\\n]|\\.)*)\'')
hits = collections.defaultdict(set)
for l in open("/Users/ueli/.cache/c6store/readers-hits.txt", encoding="utf-8"):
    f, ln, p, ctx = l.rstrip("\n").split("\t")
    if not f.endswith((".ts", ".tsx", ".py", ".mjs")): continue
    for tag in pat2tag.get(p, []):
        hits[f].add(tag)
for f, tags in sorted(hits.items()):
    text = open(R + f, encoding="utf-8").read()
    lits = set()
    for m in LIT.finditer(text):
        s = next(g for g in m.groups() if g is not None)
        s = s.replace("\\n", "\n").replace("\\\"", '"').replace("\\'", "'").replace("\\`", "`")
        if len(s) >= 8: lits.add(s)
    for tag in sorted(tags):
        moved = sorted(s for s in lits if s in orig[tag] and s not in live[tag])
        print(f"{f.split('/')[-3]}/{f.split('/')[-2]} -> {tag}: {len(moved)} moved literal(s)")
        for s in moved[:6]:
            print("      ", repr(s[:130]))
