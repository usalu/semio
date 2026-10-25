import difflib, os, re, sys
R = "/Users/ueli/Documents/semio"
C = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/dbg-d"
O = C + "/🗑️generated/dbgv-d"
CTX = 60
LOG = re.compile(r'^\s*(?:await\s+)?(?:console\.(?:log|debug|error|warn|info)|process\.(?:stderr|stdout)\.write|println!|eprintln!)\s*\(?')
out = []
nontrivial = 0
for line in open(G + "/orig/map.txt", encoding="utf-8"):
    line = line.rstrip("\n")
    if not line:
        continue
    n, f = line.split(" ", 1)
    p = R + "/" + f
    a = open(G + f"/orig/{n}.bak", encoding="utf-8", errors="replace").read()
    raw = a
    a = a.replace("[DEBUG] ", "[TRACE] ").replace("\\[DEBUG\\]", "\\[TRACE\\]")
    b = open(p, encoding="utf-8", errors="replace").read()
    if raw == b:
        continue
    if a == b:
        out.append(f"#### {n} {f}\n  (pure [DEBUG]->[TRACE] rename)")
        continue
    al, bl = a.split("\n"), b.split("\n")
    sm = difflib.SequenceMatcher(None, al, bl, autojunk=False)
    items = []
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal":
            continue
        A = "\n".join(al[i1:i2]); B = "\n".join(bl[j1:j2])
        cm = difflib.SequenceMatcher(None, A, B, autojunk=False)
        for t, a1, a2, b1, b2 in cm.get_opcodes():
            if t == "equal":
                continue
            d = A[a1:a2]; ins = B[b1:b2]
            pre = A[max(0, a1 - CTX):a1].replace("\n", "\\n"); post = A[a2:a2 + CTX].replace("\n", "\\n")
            items.append((i1 + 1, d, ins, pre, post))
    out.append(f"#### {n} {f}")
    for ln, d, ins, pre, post in items:
        kind = "N"
        if ins == "" and re.match(r'^\s*(?:await\s+)?(?:console\.(?:log|debug|error|warn|info)|process\.(?:stderr|stdout)\.write|println!|eprintln!)\s*[\(]', d) and d.rstrip().endswith((");", ")", ");\n")) :
            kind = "L"
        if d == "DEBUG" and ins in ("TRACE",):
            kind = "T"
        elif re.fullmatch(r"\s*", d) and re.fullmatch(r"\s*", ins):
            kind = "W"
        out.append(f"  L{ln} [{kind}] -{d!r} +{ins!r} | …{pre} ⟦⟧ {post}…" if len(d) < 400 and len(ins) < 400 else f"  L{ln} [{kind}] -(len {len(d)}) {d[:200]!r}…{d[-120:]!r} +{ins[:200]!r}")
open(O + "/wdiff.txt", "w", encoding="utf-8").write("\n".join(out) + "\n")
print(len(out))
