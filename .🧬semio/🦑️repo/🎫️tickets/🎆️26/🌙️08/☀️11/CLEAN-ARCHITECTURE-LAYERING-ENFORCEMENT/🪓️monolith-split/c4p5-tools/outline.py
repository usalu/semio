"""📑 C4P5 outline: top-level items (column 0) of a Rust file with line ranges. usage: outline.py <file>"""
import re, sys
lines = open(sys.argv[1], encoding="utf-8").read().split("\n")
pat = re.compile(r'^(?:#\[[^\n]*\]\s*)*(pub(?:\([^)]*\))?\s+)?(async\s+)?(unsafe\s+)?(fn|struct|enum|trait|impl|const|static|type|mod|use|macro_rules!|union)\b\s*([A-Za-z_0-9<>:\' ,\[\]&()+=*!?-]*)')
out = []
i = 0
n = len(lines)
for i, l in enumerate(lines):
    if l.startswith("//#region") or l.startswith("//#endregion"):
        out.append((i + 1, "REGION", l[2:60]))
    elif l and not l[0].isspace() and l[0] not in "}/)]":
        m = pat.match(l)
        if m and m.group(4) != "use":
            out.append((i + 1, m.group(4), l[:110]))
        elif l.startswith("#[") or l.startswith("#!["):
            out.append((i + 1, "ATTR", l[:90]))
for ln, k, t in out:
    print(f"{ln:5} {k:8} {t}")
