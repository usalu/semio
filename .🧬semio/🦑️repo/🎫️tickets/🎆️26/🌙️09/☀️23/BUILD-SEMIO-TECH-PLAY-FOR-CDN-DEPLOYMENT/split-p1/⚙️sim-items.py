#!/usr/bin/env python3
"""⚙️ Top-level item spans of one Rust file: (name, kind, start, end) with leading docs/attributes included, from a
comment- and literal-stripped copy (brace matching on code only). Shared helper of the sim split (lane P1).
"""
import re, os
HERE = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(HERE, "🕸️engine-graph.py"), encoding="utf-8").read().split("mods = {")[0])
HEAD = re.compile(r"(?:pub(?:\([^)]*\))?\s+)?(?:(?:const|async|unsafe)\s+)*(fn|struct|enum|const|static|type|trait|impl|mod|use|macro_rules!|union|extern crate)\b")
def items(raw):
    s = strip(raw); n = len(s); out = []; i = 0
    lines_start = [0] + [m.end() for m in re.finditer(r"\n", raw)]
    while i < n:
        m = re.compile(r"\S").search(s, i)
        if not m: break
        i = m.start()
        if s.startswith("#[", i) or s.startswith("#![", i):
            j = i; depth = 0
            while j < n:
                if s[j] == "[": depth += 1
                elif s[j] == "]":
                    depth -= 1
                    if depth == 0: break
                j += 1
            i = j + 1; continue
        h = HEAD.match(s, i)
        if not h: raise SystemExit(f"unparsed top-level text at offset {i}: {raw[i:i+80]!r}")
        kind = h.group(1)
        j = h.end(); depth = 0
        while j < n:
            c = s[j]
            if c in "([": depth += 1
            elif c in ")]": depth -= 1
            elif c == "{":
                if depth == 0:
                    d = 0
                    while j < n:
                        if s[j] == "{": d += 1
                        elif s[j] == "}":
                            d -= 1
                            if d == 0: break
                        j += 1
                    end = j + 1
                    if kind in ("const", "static", "type", "use") and end < n:
                        k = s.find(";", end); end = k + 1 if k >= 0 and s[end:k].strip() == "" else end
                    break
                depth += 1
            elif c == "}": depth -= 1
            elif c == ";" and depth == 0:
                end = j + 1; break
            j += 1
        text = raw[i:end]
        if kind == "impl":
            g = re.match(r"impl(?:<[^{]*?>)?\s+(?:[^{]*?\s+for\s+)?([A-Za-z_][A-Za-z0-9_:]*)", re.sub(r"\s+", " ", s[i:end][:300]).replace("pub ", ""))
            name = g.group(1).split("::")[-1] if g else "?"
        elif kind == "use":
            name = "use"
        else:
            g = re.match(r".*?\b" + re.escape(kind.rstrip("!")) + r"!?\s+([A-Za-z_][A-Za-z0-9_]*)", s[i:end][:400], re.S)
            name = g.group(1) if g else "?"
        # back up over docs/attributes/blank lines directly above (already consumed attributes are part of start)
        start = i
        k = raw.rfind("\n", 0, start) + 1
        while True:
            prev_end = k - 1
            if prev_end <= 0: break
            p = raw.rfind("\n", 0, prev_end) + 1
            line = raw[p:prev_end].strip()
            if line.startswith("///") or line.startswith("#["):
                k = p
            else:
                break
        out.append((name, kind, k, end))
        i = end
    return out
