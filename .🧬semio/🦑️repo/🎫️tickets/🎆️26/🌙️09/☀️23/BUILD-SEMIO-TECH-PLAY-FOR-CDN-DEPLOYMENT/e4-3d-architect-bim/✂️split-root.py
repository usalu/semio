#!/usr/bin/env python3
"""✂️ Splits one artifact crate root's inline `#[path]` module tree into per-crate roots (wave E4 crate cut).

The spec names, per target crate, the root file to write, its header (crate docs plus extern aliases) and the module
subtrees it owns, each given as a module path (`standards::v1::subsets::any::schema::mutations`) or a top-level module
(`editor`). Every owned subtree is copied verbatim out of the source root, wrapped in the `#[path = "."]` skeleton of
its ancestors, and every `#[path = "P"]` inside it is rebased from the source root's directory onto the new root's
directory, so the module paths stay identical and the cut is a pure crate-prefix change. Subtrees named in `drop` are
removed from the source root afterwards (the source root keeps everything else).

usage: ✂️split-root.py <spec.json> [--dry]
spec.json: {"source": <repo-relative root file>, "crates": [{"root": <repo-relative file>, "header": <text>,
            "own": [<module path>...], "exclude": [<module path>...]}], "drop": [<module path>...]}
"""
import json, os, re, sys

REPO = "/Users/ueli/Documents/semio"


def match_brace(s, i):
    d = 0
    while i < len(s):
        if s[i] == "{": d += 1
        elif s[i] == "}":
            d -= 1
            if d == 0: return i
        i += 1
    raise ValueError("unbalanced")


def find_mod(s, a, b, name):
    """Returns (start incl. leading attributes, end incl. closing brace or semicolon) of `pub mod name` directly in [a,b)."""
    depth = 0; i = a
    pat = re.compile(r"(?:pub(?:\([^)]*\))?\s+)?mod\s+" + re.escape(name) + r"\s*([{;])")
    while i < b:
        c = s[i]
        if c == "{": depth += 1; i += 1; continue
        if c == "}": depth -= 1; i += 1; continue
        if depth == 0:
            m = pat.match(s, i)
            if m and (i == 0 or not (s[i - 1].isalnum() or s[i - 1] == "_")):
                start = i
                while True:
                    k = s.rfind("\n", 0, start - 1)
                    line = s[k + 1:start].strip()
                    if line.startswith("#[") or line.startswith("///"):
                        start = k + 1
                    else:
                        break
                start = s.rfind("\n", 0, start) + 1
                end = match_brace(s, m.end() - 1) + 1 if m.group(1) == "{" else m.end()
                return start, end
        i += 1
    return None


def locate(s, path):
    a, b = 0, len(s)
    span = None
    for seg in path.split("::"):
        span = find_mod(s, a, b, seg)
        if span is None: raise KeyError(path)
        body = s.index("{", s.index("mod " + seg, span[0]))
        a, b = body + 1, span[1] - 1
    return span


def rebase(text, src_dir, dst_dir):
    def fix(m):
        p = m.group(1)
        if p == ".": return m.group(0)
        return '#[path = "' + os.path.relpath(os.path.normpath(os.path.join(src_dir, p)), dst_dir) + '"]'
    return re.sub(r'#\[path\s*=\s*"([^"]*)"\]', fix, text)


def main():
    spec = json.load(open(sys.argv[1], encoding="utf-8")); dry = "--dry" in sys.argv
    src_path = os.path.join(REPO, spec["source"]); src = open(src_path, encoding="utf-8").read(); src_dir = os.path.dirname(src_path)
    for crate in spec["crates"]:
        dst_path = os.path.join(REPO, crate["root"]); dst_dir = os.path.dirname(dst_path)
        tree = {}
        for path in crate["own"]:
            a, b = locate(src, path)
            text = src[a:b]
            for ex in crate.get("exclude", []):
                if ex.startswith(path + "::"):
                    rel = ex[len(path) + 2:]
                    inner = text
                    ea, eb = locate(inner, path.split("::")[-1] + "::" + rel)
                    text = inner[:ea] + inner[eb:]
            tree[path] = rebase(text, src_dir, dst_dir)
        out = []

        def emit(prefix, depth):
            ind = "    " * depth
            children = sorted({p[len(prefix):].split("::")[0] for p in tree if p.startswith(prefix) and p != prefix.rstrip(":")} - {""})
            for child in children:
                full = prefix + child
                if full in tree:
                    out.append("\n".join((ind + l if l.strip() else l) for l in dedent(tree[full]).split("\n")))
                else:
                    out.append(f'{ind}#[path = "."]\n{ind}pub mod {child} {{')
                    emit(full + "::", depth + 1)
                    out.append(f"{ind}}}")
        emit("", 0)
        body = crate["header"].rstrip("\n") + "\n\n" + "\n".join(out).rstrip("\n") + "\n"
        print(f"== {crate['root']}: {body.count(chr(10))} lines")
        if not dry:
            os.makedirs(dst_dir, exist_ok=True)
            open(dst_path, "w", encoding="utf-8").write(body)
    for path in spec.get("drop", []):
        a, b = locate(src, path)
        src = src[:a].rstrip(" ") + src[b:].lstrip("\n")
    if not dry: open(src_path, "w", encoding="utf-8").write(src)


def dedent(text):
    lines = text.split("\n")
    cut = min((len(l) - len(l.lstrip()) for l in lines if l.strip()), default=0)
    return "\n".join(l[cut:] for l in lines)


if __name__ == "__main__":
    main()
