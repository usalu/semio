"""🧩️ quiet16: derives the anchored hunks of every modified (non-new) test file of a staged batch from its base and new bytes, each hunk widened by context lines until its old text is unique in the base, proves base + hunks == new, and writes edit_<batch>_tests.json for the rebase route of edit_<batch>.py.
usage: python3 -I make_tests_edits.py <batch>"""
import difflib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
batch = sys.argv[1]; stage = os.path.join(HERE, "staged", batch)
manifest = json.load(open(os.path.join(stage, "manifest.json"), encoding="utf8")); out = {}
for rel, row in manifest.items():
    if row["base"] is None or "🧪️tests" not in rel: continue
    A = open(os.path.join(stage, "base", row["file"]), encoding="utf8").read(); B = open(os.path.join(stage, "files", row["file"]), encoding="utf8").read()
    a, b, pairs = A.split("\n"), B.split("\n"), []
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        if tag == "equal": continue
        c = 0
        while True:
            lo, hi = max(0, i1 - c - (1 if i1 == i2 else 0)), min(len(a), i2 + c)
            old, new = "\n".join(a[lo:hi]), "\n".join(b[j1 - (i1 - lo):j2 + (hi - i2)])
            if old and A.count(old) == 1: break
            c += 1
        pairs.append([old, new])
    text = A
    for old, new in pairs: text = text.replace(old, new)
    if text != B: sys.exit("hunks do not rebuild " + rel)
    out[rel] = pairs
json.dump(out, open(os.path.join(HERE, "edit_%s_tests.json" % batch), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("hunks", {rel[-40:]: len(pairs) for rel, pairs in out.items()})
