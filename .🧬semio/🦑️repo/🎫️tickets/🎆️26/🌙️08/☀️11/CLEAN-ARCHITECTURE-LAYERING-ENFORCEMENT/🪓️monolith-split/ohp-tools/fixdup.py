"""🔁 For every single-name `use` line this fleet added (not in HEAD), removes the HEAD import of the same name from the same scope window (±60 lines): the old import was the unresolved one, the new line is its repaired path; a braced old import loses just that name.
usage: fixdup.py [--apply]"""
import subprocess, re, os, sys, collections
R = "/Users/ueli/Documents/semio"
apply = "--apply" in sys.argv
files = [f for f in subprocess.run(["git", "-c", "core.fsmonitor=false", "diff", "--name-only", "HEAD", "--", "✏️s/🔌️plugins", "✏️s/🧑‍💻dev"], cwd=R, capture_output=True, text=True).stdout.split("\n") if f.endswith(".rs")]
tot = 0
for f in files:
    p = os.path.join(R, f)
    try: L = open(p, encoding="utf-8").read().split("\n")
    except Exception: continue
    head = subprocess.run(["git", "-c", "core.fsmonitor=false", "show", "HEAD:" + f], cwd=R, capture_output=True, text=True).stdout.split("\n")
    hc = collections.Counter(head)
    drop = set(); edits = {}
    for i, l in enumerate(L):
        m = re.match(r"^use ([A-Za-z0-9_:]+)::([A-Za-z0-9_]+);$", l)
        if not m or hc.get(l, 0) > 0: continue
        leaf = m.group(2)
        for j in range(max(0, i - 60), min(len(L), i + 60)):
            if j == i or hc.get(L[j], 0) == 0: continue
            w = L[j]
            ms = re.match(r"^use ([A-Za-z0-9_:]+)::" + re.escape(leaf) + r";$", w)
            if ms and ms.group(0) != l:
                drop.add(j); tot += 1; print(f.split("artifacts/")[-1][-55:], "drop", w[:100]); break
            mb = re.match(r"^use ([A-Za-z0-9_:]+)::\{([^}]*)\};$", w)
            if mb:
                names = [x.strip() for x in mb.group(2).split(",") if x.strip()]
                if leaf in names:
                    names.remove(leaf)
                    edits[j] = (f"use {mb.group(1)}::{{{', '.join(names)}}};" if len(names) > 1 else (f"use {mb.group(1)}::{names[0]};" if names else None))
                    tot += 1; print(f.split("artifacts/")[-1][-55:], "strip", leaf, "from", w[:90]); break
    if apply and (drop or edits):
        for j, v in edits.items(): L[j] = v
        L = [l for k, l in enumerate(L) if k not in drop and l is not None]
        open(p, "w", encoding="utf-8").write("\n".join(L))
print("duplicates", tot, "APPLIED" if apply else "dry")
