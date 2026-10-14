#!/usr/bin/env python3
"""🚚️ Rewrites paths to items that leave a crate root: `<head>::NAME` (inline, in `use` groups and single uses) becomes
`<target path>::NAME` for every NAME in the map. `head` is `crate` inside the crate and the crate ident outside it.
Use groups are flattened and regrouped per target; attributes before a `use` are repeated on each emitted line.

usage: 🚚️relocate-root.py <map.json> <head> <file>...
map: {"NAME": "<path without NAME>", ...}   e.g. {"FemNode": "crate::standards::v1::subsets::any::schema::structure"}
     (for an external head, a target starting with `crate::` is re-headed to that head)
"""
import json, re, sys
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
def split_top(text):
    parts, depth, cur = [], 0, ""
    for ch in text:
        if ch == "{": depth += 1
        if ch == "}": depth -= 1
        if ch == "," and depth == 0: parts.append(cur); cur = ""
        else: cur += ch
    if cur.strip(): parts.append(cur)
    return [p.strip() for p in parts if p.strip()]
def leaves(prefix, body):
    m = re.match(r"^((?:" + IDENT + r"\s*::\s*)*)\{(.*)\}$", body.strip(), re.S)
    if m:
        pre = prefix + [x for x in re.split(r"\s*::\s*", m.group(1)) if x]
        out = []
        for part in split_top(m.group(2)): out += leaves(pre, part)
        return out
    alias = re.search(r"\s+as\s+(" + IDENT + r")$", body.strip())
    core = re.sub(r"\s+as\s+" + IDENT + r"$", "", body.strip())
    return [(prefix + [x for x in re.split(r"\s*::\s*", core) if x], alias.group(1) if alias else None)]
mp = json.load(open(sys.argv[1], encoding="utf-8")); head = sys.argv[2]
def target(name):
    t = mp[name]
    if "::" not in head and head != "crate" and t.startswith("crate::"): t = head + t[5:]
    return t
for path in sys.argv[3:]:
    s = open(path, encoding="utf-8").read(); orig = s
    use = re.compile(r"(?m)^(?P<attrs>(?:[ \t]*#\[[^\n]*\]\n)*)(?P<indent>[ \t]*)(?P<vis>pub(?:\([^)]*\))?\s+)?use\s+" + re.escape(head) + r"::(?P<body>[^;]*);")
    def g(m):
        flat = leaves([], m.group("body"))
        if not any(segs and segs[0] in mp for segs, _ in flat): return m.group(0)
        rows = {}
        for segs, alias in flat:
            if segs and segs[0] in mp:
                pre, rest = target(segs[0]), segs
            else:
                pre, rest = head, segs
            rows.setdefault(pre, []).append("::".join(rest) + (f" as {alias}" if alias else ""))
        lines = []
        for pre, items in rows.items():
            body = items[0] if len(items) == 1 else "{" + ", ".join(items) + "}"
            lines.append(f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {pre}::{body};")
        return "\n".join(lines)
    s = use.sub(g, s)
    inline = re.compile(r"(?<![\w:])" + re.escape(head) + r"::(" + "|".join(sorted(map(re.escape, mp), key=len, reverse=True)) + r")\b(?!\s*::\s*\{)")
    s = inline.sub(lambda m: target(m.group(1)) + "::" + m.group(1), s)
    if s != orig:
        open(path, "w", encoding="utf-8").write(s); print("relocated", path)
