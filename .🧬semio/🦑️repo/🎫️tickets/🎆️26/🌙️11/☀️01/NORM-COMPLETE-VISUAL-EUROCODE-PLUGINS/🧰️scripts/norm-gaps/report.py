import json, re, os, sys
from collections import Counter
SPEC = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/📐️specs"
FILES = {"en1990": "en1990-en1991-target.md", "en1991": "en1990-en1991-target.md", "en1992": "en1992-target.md",
         "en1993": "en1993-target.md", "en1994": "en1994-en1999-target.md", "en1995": "en1995-en1996-target.md",
         "en1996": "en1995-en1996-target.md", "en1997": "en1997-en1998-target.md", "en1998": "en1997-en1998-target.md",
         "en1999": "en1994-en1999-target.md"}
o = json.load(open(sys.argv[1], encoding="utf-8"))
lines = {}
def lineno(code, id_):
    f = FILES[code]
    if f not in lines:
        lines[f] = open(os.path.join(SPEC, f), encoding="utf-8").read().splitlines()
    L = lines[f]
    for pat in [r"^\|\s*`?" + re.escape(id_) + r"`?\s*\|", r"(?<![A-Za-z0-9.\-])" + re.escape(id_) + r"(?![A-Za-z0-9\-])"]:
        for i, l in enumerate(L):
            if re.search(pat, l):
                return "%s:%d" % (f, i + 1)
    return "%s:?" % f
mode = sys.argv[2]
for code in o:
    for r in o[code]["items"]:
        if mode == "all" and r["status"] in ("missing", "partial", "blocked-unverified"):
            print(code, r["id"], r["clause"][:40], r["status"], lineno(code, r["id"]), "|", r["en"][:150].replace("\n", " "), "|", (r["notes"] or [""])[0])
