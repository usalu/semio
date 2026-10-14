#!/usr/bin/env python3
"""🧩️ Lists the top-level items of one Rust file with line spans, the names each defines and the top-level names it
references, then (with an assignment JSON {name-regex-or-line-range: group}) reports every cross-group reference so a
single file can be cut into crates along an acyclic order.

usage: 🧩️items.py <file.rs> [--assign assign.json] [--order g1,g2,...]
"""
import json, re, sys
sys.path.insert(0, __import__("os").path.dirname(__file__))

def strip(src):
    out, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i); j = n if j < 0 else j; out.append(" " * (j - i)); i = j
        elif src.startswith("/*", i):
            d, j = 1, i + 2
            while j < n and d:
                if src.startswith("/*", j): d += 1; j += 2
                elif src.startswith("*/", j): d -= 1; j += 2
                else: j += 1
            out.append(re.sub(r"[^\n]", " ", src[i:j])); i = j
        elif c == "r" and re.match(r'r#*"', src[i:i + 40]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = re.match(r'r(#*)"', src[i:]); close = '"' + m.group(1); j = src.find(close, i + len(m.group(0)))
            j = n if j < 0 else j + len(close); out.append(re.sub(r"[^\n]", " ", src[i:j])); i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"': j += 2 if src[j] == "\\" else 1
            j += 1; out.append('"' + re.sub(r"[^\n]", " ", src[i + 1:j - 1]) + '"'); i = j
        elif c == "'" and (re.match(r"'(\\.|[^\\'])'", src[i:i + 4]) or re.match(r"'\\(x..|u\{[0-9a-fA-F]+\})'", src[i:i + 12])):
            m = re.match(r"'(\\x..|\\u\{[0-9a-fA-F]+\}|\\.|[^\\'])'", src[i:]); out.append(" " * len(m.group(0))); i += len(m.group(0))
        else:
            out.append(c); i += 1
    return "".join(out)

ITEM = re.compile(r"(?m)^(?P<attrs>(?:#\[[^\n]*\]\s*\n)*)(?P<head>(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?P<kind>fn|struct|enum|trait|impl|const|static|type|mod|macro_rules!|use)\b[^\n]*)")

def items(path):
    raw = open(path, encoding="utf-8").read(); s = strip(raw)
    lines = s.split("\n"); offs = [0]
    for l in lines: offs.append(offs[-1] + len(l) + 1)
    def line_of(o):
        import bisect
        return bisect.bisect_right(offs, o)
    out = []; pos = 0
    for m in ITEM.finditer(s):
        if m.start() < pos: continue
        if m.start() > 0 and s[m.start() - 1] != "\n": continue
        start = m.start()
        k = m.end("head")
        j = m.start("head")
        depth = 0; end = None
        while j < len(s):
            ch = s[j]
            if ch in "({[": depth += 1
            elif ch in ")}]":
                depth -= 1
                if depth == 0 and ch == "}":
                    nxt = s[j + 1:j + 2]
                    end = j + 1
                    if m.group("kind") not in ("struct", "enum", "const", "static", "type", "use"):
                        break
                    rest = re.match(r"\s*;", s[j + 1:])
                    if rest: end = j + 1 + rest.end()
                    break
            elif ch == ";" and depth == 0:
                end = j + 1; break
            j += 1
        if end is None: end = len(s)
        head = m.group("head")
        kind = m.group("kind")
        names = []
        if kind == "impl":
            mm = re.search(r"impl(?:<[^{]*?>)?\s+(?:([A-Za-z_][\w:]*)(?:<[^{]*?>)?\s+for\s+)?([A-Za-z_][\w]*)", head)
            names = [f"impl {mm.group(1) + ' for ' if mm and mm.group(1) else ''}{mm.group(2) if mm else '?'}"]
        elif kind in ("use", "mod"):
            names = [head.strip()]
        else:
            mm = re.search(r"(?:fn|struct|enum|trait|const|static|type|macro_rules!)\s+([A-Za-z_]\w*)", head)
            names = [mm.group(1)] if mm else ["?"]
        body = s[start:end]
        out.append({"start": line_of(start), "end": line_of(end - 1), "kind": kind, "name": names[0], "pub": head.lstrip().startswith("pub"), "body": body})
        pos = end
    return out

def main():
    path = sys.argv[1]
    its = items(path)
    defined = {}
    for it in its:
        if it["kind"] not in ("impl", "use", "mod"): defined.setdefault(it["name"], it)
    assign = json.load(open(sys.argv[sys.argv.index("--assign") + 1])) if "--assign" in sys.argv else None
    order = sys.argv[sys.argv.index("--order") + 1].split(",") if "--order" in sys.argv else None
    def group(it):
        if not assign: return "-"
        for key, g in assign.items():
            if re.fullmatch(r"\d+-\d+", key):
                a, b = map(int, key.split("-"))
                if a <= it["start"] <= b: return g
        return "?"
    for it in its:
        it["group"] = group(it)
        refs = set()
        for n in defined:
            if n != it["name"] and re.search(r"(?<![\w:])" + re.escape(n) + r"\b", it["body"]): refs.add(n)
        it["refs"] = sorted(refs)
    if not assign:
        for it in its:
            print(f'{it["start"]:5d}-{it["end"]:5d} {"pub " if it["pub"] else "    "}{it["kind"]:6s} {it["name"]}  -> {",".join(it["refs"])[:200]}')
        return
    rank = {g: i for i, g in enumerate(order)} if order else {}
    bad = {}
    for it in its:
        for r in it["refs"]:
            d = defined[r]; g1, g2 = it["group"], group(d)
            if g1 != g2:
                tag = "UP" if rank and rank.get(g2, 99) > rank.get(g1, 99) else "down"
                key = (g1, g2, tag)
                bad.setdefault(key, set()).add(f'{it["name"]}@{it["start"]} -> {r}{"" if d["pub"] else " (private)"}')
    for (g1, g2, tag), v in sorted(bad.items()):
        print(f"== {tag} {g1} -> {g2} ({len(v)})")
        for x in sorted(v): print("   ", x)
    for g in (order or []):
        n = sum(it["end"] - it["start"] + 1 for it in its if it["group"] == g)
        print(f"{g}: {n} lines")
    print("unassigned:", [(it["start"], it["name"]) for it in its if it["group"] == "?"][:40])

main()
