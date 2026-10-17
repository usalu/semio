"""Emit the subset of a module's private `use` declarations that a test file mentions, as explicit use lines for a wrapper module.
usage: useimports.py <module-file> <test-file> [--skip-crate]  (names resolved by whole-word occurrence, comments/strings masked)"""
import re, sys
sys.path.insert(0, __import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from ftcstub import mask


def flatten(tree, prefix=""):
    tree = tree.strip()
    parts, cur, depth = [], "", 0
    for ch in tree:
        if ch == "{": depth += 1
        if ch == "}": depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur); cur = ""
        else:
            cur += ch
    parts.append(cur)
    if len(parts) > 1:
        out = []
        for p in parts:
            if p.strip(): out += flatten(p, prefix)
        return out
    t = parts[0].strip()
    m = re.match(r"^(.*?)::\s*\{(.*)\}$", t, re.S)
    if m:
        return flatten(m.group(2), prefix + m.group(1).strip() + "::")
    return [prefix + t]


def declared(text):
    m = mask(text)
    out = []
    for um in re.finditer(r"^use\s+([^;]+);", m, re.M | re.S):
        for full in flatten(re.sub(r"\s+", " ", text[um.start(1):um.end(1)])):
            full = full.strip()
            if full.endswith("::*") or full == "":
                continue
            alias = None
            am = re.match(r"^(.*)\s+as\s+(\w+)$", full)
            if am:
                full, alias = am.group(1).strip(), am.group(2)
            name = alias or full.split("::")[-1]
            if name == "self":
                name = full.split("::")[-2]
                full = full[:-6]
            out.append((name, full, alias))
    return out


if __name__ == "__main__":
    mod = open(sys.argv[1], encoding="utf-8").read()
    tst = open(sys.argv[2], encoding="utf-8").read()
    used = set(re.findall(r"(?<![\w$])([A-Za-z_]\w*)", mask(tst)))
    own = {n for n, _, _ in declared(tst)}
    lines = []
    for name, full, alias in declared(mod):
        if name in used and name not in own:
            lines.append(f"use {full}{' as ' + alias if alias else ''};")
    print("\n".join(sorted(set(lines))))
