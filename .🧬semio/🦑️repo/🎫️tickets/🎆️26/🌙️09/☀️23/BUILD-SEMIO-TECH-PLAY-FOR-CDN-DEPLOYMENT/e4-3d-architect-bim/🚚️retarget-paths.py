#!/usr/bin/env python3
"""🚚️ Retargets Rust paths after items or modules move (wave E4). Rewrites `use` trees leaf by leaf (a statement whose
leaves all keep their path stays byte-identical; a changed one is rebuilt as one statement, so attributes and visibility
still apply to every leaf) and every other path occurrence (expressions, types, doc links, `$crate` macro bodies).

A move is {"from": "<root>::<seg>::...", "to": "<root>::<seg>::...", "names": [..] | "*"} with full paths including the
root (`crate`, `$crate`, `semio_framework_3d`, ...). With a name list only `from::NAME` (and deeper) moves; with "*" the
whole module subtree moves (`from` itself becomes `to`). The longest `from` wins. A spec applies to the files under its
`files` entries (repo-relative directories or files); several specs may run in sequence.

usage: 🚚️retarget-paths.py <spec.json> [--dry]
spec.json: [{"files": [...], "exclude": [regex...], "moves": [...]}, ...]
"""
import json, os, re, sys

REPO = "/Users/ueli/Documents/semio"
SEG = r"(?:\$crate|[A-Za-z_][A-Za-z0-9_]*)"


def mask(src, comments=True):
    out = list(src); i, n = 0, len(src)
    def blank(a, b):
        for k in range(a, b):
            if out[k] != "\n": out[k] = " "
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i); j = n if j < 0 else j
            if comments: blank(i, j)
            i = j; continue
        if src.startswith("/*", i):
            d, j = 1, i + 2
            while j < n and d:
                if src.startswith("/*", j): d += 1; j += 2
                elif src.startswith("*/", j): d -= 1; j += 2
                else: j += 1
            if comments: blank(i, j)
            i = j; continue
        m = re.match(r'b?r(#*)"', src[i:i + 20]) if c in "rb" and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")) else None
        if m:
            end = src.find('"' + m.group(1), i + m.end()); end = n if end < 0 else end
            blank(i, end + 1 + len(m.group(1))); i = end + 1 + len(m.group(1)); continue
        if c == '"':
            j = i + 1
            while j < n and src[j] != '"': j += 2 if src[j] == "\\" else 1
            blank(i, j + 1); i = j + 1; continue
        if c == "'":
            m = re.match(r"'(\\.[^']*|[^'\\])'", src[i:i + 12])
            if m: blank(i, i + m.end()); i += m.end(); continue
        i += 1
    return "".join(out)


def split_top(t):
    parts, d, cur = [], 0, ""
    for ch in t:
        if ch == "{": d += 1
        elif ch == "}": d -= 1
        if ch == "," and d == 0: parts.append(cur); cur = ""
        else: cur += ch
    if cur.strip(): parts.append(cur)
    return parts


def leaves(prefix, body):
    body = body.strip()
    if not body: return []
    m = re.match(r"^((?:(?:::)?\s*" + SEG + r"\s*::\s*)*)\{(.*)\}\s*$", body, re.S)
    if m:
        pre = [x for x in re.split(r"\s*::\s*", m.group(1).strip()) if x]
        out = []
        for part in split_top(m.group(2)): out += leaves(prefix + pre, part)
        return out
    alias = None
    am = re.match(r"^(.*?)\s+as\s+(\w+)$", body, re.S)
    if am: body, alias = am.group(1).strip(), am.group(2)
    segs = [x for x in re.split(r"\s*::\s*", body) if x]
    if segs and segs[-1] == "self":
        segs = segs[:-1]
        if alias is None: alias = "self"
    return [(prefix + segs, alias)]


class Mover:
    def __init__(s, moves):
        s.moves = sorted(moves, key=lambda m: -len(m["from"].split("::")))
        for m in s.moves: m["f"] = m["from"].split("::"); m["t"] = m["to"].split("::")

    def path(s, segs):
        for m in s.moves:
            f = m["f"]
            if segs[:len(f)] != f: continue
            if m["names"] == "*": return m["t"] + segs[len(f):]
            if len(segs) > len(f) and segs[len(f)] in m["names"]: return m["t"] + segs[len(f):]
        return None


def render(leafs):
    def tree(items):
        groups, order = {}, []
        for segs, alias in items:
            if segs[0] not in groups: groups[segs[0]] = []; order.append(segs[0])
            groups[segs[0]].append((segs[1:], alias))
        parts = []
        for key in order:
            sub = groups[key]
            terminals = [alias for rest, alias in sub if not rest]
            nested = [(rest, alias) for rest, alias in sub if rest]
            if len(terminals) == 1 and not nested:
                alias = terminals[0]
                parts.append(key + ("" if alias in (None, "self") else " as " + alias)); continue
            inner = ["self" if alias in (None, "self") else "self as " + alias for alias in terminals]
            if nested: inner += tree(nested)
            parts.append(key + "::" + inner[0] if len(inner) == 1 else key + "::{" + ", ".join(inner) + "}")
        return parts
    parts = tree(leafs)
    return parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"


def rewrite(src, mover):
    ms = mask(src)
    edits = []
    spans = []
    for m in re.finditer(r"(?<![\w$])use\s+", ms):
        j = m.end(); d = 0; k = j
        while k < len(ms):
            if ms[k] == "{": d += 1
            elif ms[k] == "}": d -= 1
            elif ms[k] == ";" and d == 0: break
            k += 1
        if k >= len(ms): continue
        spans.append((m.start(), k + 1))
        body = src[j:k]
        if "::" not in body and "{" not in body: continue
        ls = leaves([], body)
        changed, out = False, []
        for segs, alias in ls:
            if segs and segs[-1] == "*":
                np = mover.path(segs[:-1]); np = np + ["*"] if np is not None else None
            else:
                np = mover.path(segs)
            if np is not None and np != segs: changed = True; out.append((np, alias))
            else: out.append((segs, alias))
        if changed:
            edits.append((j, k, render(out)))
    strings = mask(src, comments=False)
    def in_use(pos):
        for a, b in spans:
            if a <= pos < b: return True
        return False
    for m in re.finditer(r"(?<![\w$])(?<!::)((?:\$crate|[A-Za-z_][A-Za-z0-9_]*)(?:::[A-Za-z_][A-Za-z0-9_]*)+)", src):
        if in_use(m.start()) or strings[m.start()] == " ": continue
        segs = m.group(1).split("::")
        np = mover.path(segs)
        if np is None and segs[-1:] and False: continue
        if np is not None and np != segs:
            edits.append((m.start(1), m.end(1), "::".join(np)))
    if not edits: return src, 0
    edits.sort()
    out, cur = [], 0
    for a, b, t in edits:
        if a < cur: continue
        out.append(src[cur:a]); out.append(t); cur = b
    out.append(src[cur:])
    return "".join(out), len(edits)


def files_of(spec):
    exc = [re.compile(x) for x in spec.get("exclude", [])]
    for entry in spec["files"]:
        p = os.path.join(REPO, entry)
        if os.path.isfile(p): yield p; continue
        for root, ds, fs in os.walk(p):
            ds[:] = [d for d in ds if d not in ("target", "node_modules", "⚡️cache", "🎫️tickets") and not d.startswith(".")]
            for f in fs:
                if f.endswith(".rs"):
                    q = os.path.join(root, f)
                    if not any(e.search(os.path.relpath(q, REPO)) for e in exc): yield q


def main():
    specs = json.load(open(sys.argv[1], encoding="utf-8")); dry = "--dry" in sys.argv
    total = 0
    for spec in specs:
        mover = Mover(spec["moves"])
        for f in sorted(set(files_of(spec))):
            src = open(f, encoding="utf-8").read()
            new, n = rewrite(src, mover)
            if n:
                total += n
                print(f"{n:4} {os.path.relpath(f, REPO)}")
                if not dry: open(f, "w", encoding="utf-8").write(new)
    print(f"edits: {total}")


if __name__ == "__main__":
    main()
