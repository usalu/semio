#!/usr/bin/env python3
"""🪝 Applies anchored replacements to a root: `anchored.py <edits.py> <root> [--check]`. The edits module defines EDITS = [(rel, old, new), ...]; every `old` must occur exactly once (a 4th element "all" replaces every occurrence, at least one) in the file at the time it is applied (edits apply in order, a later edit may anchor on an earlier one's text); file modes are kept. --check only reports whether each anchor is unique."""
import os, runpy, stat, sys
def main():
    edits, root, check = runpy.run_path(sys.argv[1])["EDITS"], os.path.abspath(sys.argv[2]), "--check" in sys.argv
    texts, bad = {}, 0
    for rel, old, new, *mode in edits:
        path = os.path.join(root, rel)
        text = texts.get(rel)
        if text is None: text = texts[rel] = open(path, encoding="utf-8").read()
        count = text.count(old)
        if mode == ["all"] and count:
            texts[rel] = text.replace(old, new); continue
        if count != 1: print("ANCHOR %d× %s :: %s" % (count, rel.split("/")[-2] + "/" + rel.split("/")[-1], old[:90].replace("\n", "⏎"))); bad += 1; continue
        texts[rel] = text.replace(old, new, 1)
    if bad: sys.exit(2)
    if check: print("anchors unique:", len(edits)); return
    for rel, text in texts.items():
        path = os.path.join(root, rel); mode = stat.S_IMODE(os.stat(path).st_mode)
        with open(path, "w", encoding="utf-8", newline="") as handle: handle.write(text)
        os.chmod(path, mode)
    print("applied", len(edits), "edits to", len(texts), "files")
main()
