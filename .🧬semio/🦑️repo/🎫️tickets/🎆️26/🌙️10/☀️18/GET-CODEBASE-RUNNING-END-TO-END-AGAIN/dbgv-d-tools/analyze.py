import difflib, re, sys, glob
R = "/Users/ueli/Documents/semio/"
C = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/"
G = C + "🗑️generated/dbg-d/"
changed = {}
paths = {}
for line in open(G + "orig/map.txt", encoding="utf-8"):
    n, f = line.rstrip("\n").split(" ", 1)
    try:
        b = open(R + f, encoding="utf-8", errors="replace").read().split("\n")
    except OSError:
        continue
    a = open(G + f"orig/{n}.bak", encoding="utf-8", errors="replace").read().replace("[DEBUG] ", "[TRACE] ").split("\n")
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    lines = set()
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal":
            continue
        for j in range(max(1, j1 - 0), j2 + 2):
            lines.add(j)
    changed[R + f] = lines
    paths[R + f] = n
frame = re.compile(r"\((/Users/ueli/Documents/semio/[^)]+?):(\d+):(\d+)\)")
frame2 = re.compile(r"at (/Users/ueli/Documents/semio/\S+?):(\d+):(\d+)")
for log in sys.argv[1:]:
    text = open(log, encoding="utf-8", errors="replace").read()
    hits = {}
    for m in list(frame.finditer(text)) + list(frame2.finditer(text)):
        p, ln = m.group(1), int(m.group(2))
        if p in changed:
            key = (paths[p], p, ln)
            hits[key] = hits.get(key, 0) + 1
    print("==", log[-60:])
    for k, v in sorted(hits.items()):
        near = k[2] in changed[k[1]]
        print("  frame in changed file", (k[0], k[1][len(R):][-60:], k[2]), "x", v, "ON-CHANGED-LINE" if near else "not a changed line")
