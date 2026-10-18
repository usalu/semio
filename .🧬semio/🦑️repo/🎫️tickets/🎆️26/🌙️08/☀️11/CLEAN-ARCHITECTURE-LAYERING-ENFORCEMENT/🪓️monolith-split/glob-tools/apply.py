import sys,os,shutil
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from plan import *
HERE=os.path.dirname(os.path.abspath(__file__))
os.makedirs(HERE+"/orig",exist_ok=True)
for key in sys.argv[1:]:
    d, rootp, text, out, warn, dups, t, toks = build(key)
    for w in warn: print("  !", key, w)
    if dups: print("DUPS", key, dups); sys.exit(1)
    cur = read(rootp)
    assert cur == text
    bk = HERE+"/orig/"+key+".rs"
    if not os.path.exists(bk): shutil.copy(rootp, bk)
    new = text
    for p, lines in sorted(out, key=lambda x: -x[0]["span"][0]):
        a, b = p["span"]
        rep = "\n".join(lines)
        if not rep:
            rep = ""
        new = new[:a] + rep + new[b:]
    open(rootp, "w", encoding="utf-8").write(new)
    print("applied", key, len(out), "globs;", text.count("\n"), "->", new.count("\n"), "lines")
