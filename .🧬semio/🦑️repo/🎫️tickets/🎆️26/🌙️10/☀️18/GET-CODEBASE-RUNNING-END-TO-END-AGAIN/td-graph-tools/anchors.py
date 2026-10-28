"""⚓ [td-graph] anchored edits between two versions of a file, replayable on a drifted base.
usage: anchors.py make <base-file> <new-file> <out.json>   (changed spans by a line diff refined by a character diff; each group of spans becomes {old,new} with the shortest unique context, >= 24 chars each side)
       anchors.py apply <anchors.json> <target-file>        (every `old` must occur exactly once, or its `new` already once = applied; else exit 2 and nothing is written; keeps the file mode)"""
import difflib, json, os, stat, sys
def spans(x, y):
    xl, yl = x.splitlines(True), y.splitlines(True)
    xo, yo = [0], [0]
    for l in xl: xo.append(xo[-1] + len(l))
    for l in yl: yo.append(yo[-1] + len(l))
    out = []
    for tag, a1, a2, b1, b2 in difflib.SequenceMatcher(None, xl, yl, autojunk=False).get_opcodes():
        if tag == "equal": continue
        bx, by = x[xo[a1]:xo[a2]], y[yo[b1]:yo[b2]]
        ops = difflib.SequenceMatcher(None, bx, by, autojunk=False).get_opcodes() if len(bx) * len(by) < 2e8 else [("replace", 0, len(bx), 0, len(by))]
        out += [(xo[a1] + i1, xo[a1] + i2, by[j1:j2]) for t, i1, i2, j1, j2 in ops if t != "equal"]
    return out
def window(x, a, b):
    for pad in (24, 48, 96, 192, 384, 768, 1536, 4096, 16384):
        s, e = max(0, a - pad), min(len(x), b + pad)
        if x.count(x[s:e]) == 1: return s, e
    sys.exit(f"no unique anchor at {a}")
def make(base, new, out):
    x, y = open(base, encoding="utf8").read(), open(new, encoding="utf8").read()
    groups = []
    for s, e, rep in spans(x, y):
        a, b = window(x, s, e)
        groups.append({"at": a, "end": b, "spans": [(s, e, rep)]})
        while len(groups) > 1 and groups[-1]["at"] < groups[-2]["end"]:
            q = groups.pop(); g = groups[-1]; g["spans"] += q["spans"]; g["at"], g["end"] = window(x, min(g["at"], q["at"]), max(g["end"], q["end"]))
    rows = []
    for g in groups:
        text = x[g["at"]:g["end"]]
        for s, e, rep in sorted(g["spans"], reverse=True): text = text[:s - g["at"]] + rep + text[e - g["at"]:]
        rows.append({"old": x[g["at"]:g["end"]], "new": text})
    if splice(x, rows) != y: sys.exit("anchors do not reproduce the new file")
    json.dump(rows, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=0)
    print(out, len(rows), "anchors")
def splice(text, rows):
    hits = []
    for row in rows:
        if text.count(row["old"]) == 1: hits.append((text.find(row["old"]), row))
        elif text.count(row["new"]) != 1: sys.exit(f"DRIFT anchor x{text.count(row['old'])}: {row['old'][:90]!r}")
    for at, row in sorted(hits, key=lambda h: h[0], reverse=True): text = text[:at] + row["new"] + text[at + len(row["old"]):]
    return text
def apply(anchors, target):
    rows, text = json.load(open(anchors, encoding="utf8")), open(target, encoding="utf8").read()
    text = splice(text, rows)
    mode = stat.S_IMODE(os.stat(target).st_mode)
    with open(target, "w", encoding="utf8") as f: f.write(text)
    os.chmod(target, mode)
if __name__ == "__main__":
    {"make": lambda: make(*sys.argv[2:5]), "apply": lambda: apply(*sys.argv[2:4])}[sys.argv[1]]()
