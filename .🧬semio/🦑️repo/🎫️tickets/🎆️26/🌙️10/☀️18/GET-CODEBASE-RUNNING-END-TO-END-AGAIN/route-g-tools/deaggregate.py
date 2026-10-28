"""Removes every bare (side-effect) import of another test file so each test file runs in its own process; scans the given files against the test files of the whole library. Usage: deaggregate.py <library-relative files.txt> <scan-prefix> [--apply]"""
import os, re, sys
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
lib = [l.strip() for l in open(sys.argv[1], encoding="utf8")]
known = set(os.path.normpath(os.path.join(L, f)) for f in lib)
prefix = sys.argv[2]
scan = [os.path.join(L, f) for f in lib if f.startswith(prefix)]
bare = re.compile(r"(^|[;{}])([ \t]*)import\s*([\"'])([^\"']+)\3[ \t]*;?[ \t]*\n?", re.M)
apply = "--apply" in sys.argv
total = 0
for f in scan:
    text = open(f, encoding="utf8").read(); out = []; last = 0; n = 0
    for m in bare.finditer(text):
        spec = m.group(4)
        if not spec.startswith("."): continue
        if os.path.normpath(os.path.join(os.path.dirname(f), spec)) not in known: continue
        out.append(text[last:m.start()] + m.group(1)); last = m.end(); n += 1
    if not n: continue
    out.append(text[last:]); new = "".join(out)
    lines = new.split("\n"); head = 0
    while head < len(lines) and (lines[head].startswith("import") or lines[head].strip() == ""): head += 1
    new = "\n".join([l for l in lines[:head] if l.strip() != ""] + lines[head:])
    total += n
    print(n, f.replace(L + "/", ""))
    if apply: open(f, "w", encoding="utf8").write(new)
print("removed", total)
