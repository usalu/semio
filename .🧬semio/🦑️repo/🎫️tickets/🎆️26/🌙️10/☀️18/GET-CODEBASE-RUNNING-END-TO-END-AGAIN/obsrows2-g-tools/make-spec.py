#!/usr/bin/env python3
"""[obsrows2-g] derives anchored edits (spec.json) from a base tree and a final tree.
usage: make-spec.py <base-root> <final-root> <out spec.json>      (file list: mine.json beside this script)
Per file: line-level opcodes, refined to character spans inside replaced blocks, each span widened with context until its `old` text is unique in the base and no span overlaps another."""
import difflib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
base_root, final_root, out = sys.argv[1:4]
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
files = json.load(open(HERE + "/mine.json", encoding="utf8"))

def spans(base, final):
    result = []
    a_lines, b_lines = base.splitlines(keepends=True), final.splitlines(keepends=True)
    offsets = [0]
    for line in a_lines: offsets.append(offsets[-1] + len(line))
    boffsets = [0]
    for line in b_lines: boffsets.append(boffsets[-1] + len(line))
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a_lines, b_lines, autojunk=False).get_opcodes():
        if tag == "equal": continue
        old, new = base[offsets[i1]:offsets[i2]], final[boffsets[j1]:boffsets[j2]]
        if tag == "replace" and len(old) < 400000 and len(new) < 400000:
            matcher = difflib.SequenceMatcher(None, old, new, autojunk=False)
            for t, x1, x2, y1, y2 in matcher.get_opcodes():
                if t != "equal": result.append((offsets[i1] + x1, offsets[i1] + x2, new[y1:y2]))
        else:
            result.append((offsets[i1], offsets[i2], new))
    return result

def widen(base, found, final, gap=8):
    merged = []
    for start, end, new in sorted(found):
        if merged and start <= merged[-1][1] + gap:
            ps, pe, pn = merged[-1]; merged[-1] = (ps, end, pn + base[pe:start] + new)
        else: merged.append((start, end, new))
    edits = []
    for start, end, new in merged:
        left, right = 0, 0
        while True:
            s, e = max(0, start - left), min(len(base), end + right)
            old = base[s:e]
            if old and base.count(old) == 1 and final.count(base[s:start] + new + base[end:e]) == 1: break
            if s == 0 and e == len(base): break
            left += 24; right += 24
        edits.append((s, e, base[s:start] + new + base[end:e]))
    for k in range(len(edits) - 1):
        if edits[k][1] > edits[k + 1][0]: return widen(base, found, final, gap * 4)
    return edits

spec = {"files": {}}
for rel in files:
    path = LIB + rel
    base = open(os.path.join(base_root, path), encoding="utf8").read()
    final = open(os.path.join(final_root, path), encoding="utf8").read()
    if base == final: print("unchanged", rel[-60:]); continue
    edits = widen(base, spans(base, final), final)
    spec["files"][path] = [{"old": base[s:e], "new": n} for s, e, n in edits]
    print("%3d edits  %s" % (len(edits), rel[-70:]))
json.dump(spec, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=1)
