#!/usr/bin/env python3
"""🧪️ Moves white-box test modules of a `#[path]`-tree monolith into its root test region (pre-cut, compile-atomic).

For every entry the `#[cfg(test)] #[path] mod` declaration is cut from its parent file and re-declared in the crate
root (after `anchor`) under its new name. Inside the test file every `super::`/`super::super::` path is rewritten to
the absolute `crate::` path it meant (nested inline modules of the test file keep their own `super` chain), the
parent's private `use` leaves the test names are imported explicitly, and every parent item the test names that is
not `pub` becomes `pub`, so the module keeps compiling from the root and, later, from an integration target.
`renames` rewrite literal path prefixes in every crate file afterwards.

usage: 🧪️test-region.py <spec.json> plan|apply
spec: {"artifact": dir, "anchor": text in the root, "tests": [{"parent": file, "test": file, "name": ident}],
       "renames": {"crate::old": "crate::new"}}
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ns = {}
exec(compile(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], "module-graph", "exec"), ns)
Crate, strip, flatten = ns["Crate"], ns["strip"], ns["flatten"]
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
ITEM = re.compile(r"(?m)^(pub(?:\s*\([^)]*\))?\s+)?((?:async\s+|const\s+|unsafe\s+)*)(fn|const|static|struct|enum|type|trait|mod)\s+(" + IDENT + r")")


def modpath_of(crate, path):
    mp = crate.files[os.path.normpath(path)]
    return mp[:-1] if mp and mp[-1] == "component" else mp


def absolutize(text, base, own):
    """🧭️ Rewrites `super::` chains: `own` is the test module's original path below `base`; paths into the test module itself re-root at its new root-level name."""
    raw = strip(text)
    spans = []
    for m in re.finditer(r"(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+(" + IDENT + r")\s*\{", raw):
        depth, j = 0, m.end() - 1
        while j < len(raw):
            depth += raw[j] == "{"
            depth -= raw[j] == "}"
            if raw[j] == "}" and depth == 0:
                break
            j += 1
        spans.append((m.end(), j, m.group(1)))

    def scope(offset):
        path = list(own)
        for a, b, name in sorted(spans):
            if a <= offset < b:
                path.append(name)
        return path
    out, last = [], 0
    for m in re.finditer(r"(?<![\w:])(?:super\s*::\s*)+", raw):
        chain = m.group(0).count("super")
        here = scope(m.start())
        target = here[:len(here) - chain]
        if len(target) >= len(own):
            target = [own[-1]] + target[len(own):]
        out.append(text[last:m.start()])
        out.append("crate::" + "".join(s + "::" for s in target))
        last = m.end()
    out.append(text[last:])
    return re.sub(r"crate::(\s*\*)", r"crate::\1", "".join(out))


def parent_uses(raw, base):
    leaves = {}
    for m in re.finditer(r"(?m)^use\s+([^;]+);", strip(raw)):
        body = m.group(1).strip()
        for leaf in flatten([], body):
            if not leaf:
                continue
            alias = None
            part = re.search(r"(" + IDENT + r")\s+as\s+(" + IDENT + r")", body)
            name = leaf[-1]
            if leaf[0] == "self":
                leaf = ["crate"] + list(base) + leaf[1:]
            elif leaf[0] == "super":
                ups = 0
                while ups < len(leaf) and leaf[ups] == "super":
                    ups += 1
                leaf = ["crate"] + list(base[:len(base) - ups]) + leaf[ups:]
            leaves.setdefault(name, "::".join(leaf))
    for m in re.finditer(r"(?m)^use\s+([^;{]*?)(" + IDENT + r")\s+as\s+(" + IDENT + r")\s*;", strip(raw)):
        leaves[m.group(3)] = f"{m.group(1)}{m.group(2)} as {m.group(3)}"
    return leaves


def main():
    spec = json.load(open(sys.argv[1], encoding="utf-8"))
    mode = sys.argv[2]
    art = os.path.join(REPO, spec["artifact"])
    root = os.path.join(art, "🦀️.rs")
    crate = Crate(root)
    files = {}

    def get(p):
        p = os.path.normpath(p)
        if p not in files:
            files[p] = open(p, encoding="utf-8").read()
        return p
    mounts = []
    for t in spec["tests"]:
        parent = get(os.path.join(art, t["parent"]))
        test = get(os.path.join(art, t["test"]))
        base = modpath_of(crate, parent)
        rel = os.path.relpath(test, os.path.dirname(parent))
        decl = re.search(r"#\[cfg\(test\)\]\s*\n#\[path\s*=\s*\"(?:\./)?" + re.escape(rel) + r"\"\]\s*\n(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+" + IDENT + r"\s*;\n", files[parent])
        if not decl:
            sys.exit(f"no mount of {t['test']} in {t['parent']}")
        files[parent] = files[parent][:decl.start()] + files[parent][decl.end():]
        text = files[test]
        text = absolutize(text, base, base + (t["name"],))
        uses = parent_uses(files[parent], base)
        defined = set(m.group(4) for m in ITEM.finditer(strip(files[parent])))
        idents = set(re.findall(r"\b(" + IDENT + r")\b", strip(text)))
        own_items = set(m.group(4) for m in ITEM.finditer(strip(text)))
        own_uses = set(re.findall(r"\b(" + IDENT + r")\s*[,};]", " ".join(re.findall(r"(?m)^\s*use\s+[^;]+;", strip(text)))))
        glob = re.search(r"use\s+crate::" + re.escape("::".join(base)) + r"::\*\s*;", text) if base else None
        if glob:
            add = sorted(f"use {uses[n]};" for n in (idents & set(uses)) - own_items - own_uses - defined if not uses[n].endswith("::*"))
            if add:
                head = re.match(r"(?s)((?://![^\n]*\n)*)", text).end()
                text = text[:head] + "\n".join(add) + "\n" + text[head:]
            used = idents & defined
            body = files[parent]
            for m in sorted(ITEM.finditer(strip(body)), key=lambda m: -m.start()):
                vis, name = m.group(1), m.group(4)
                if name in used and (not vis or vis.strip() != "pub") and m.group(3) != "mod":
                    line_start = body.rfind("\n", 0, m.start()) + 1
                    if body[line_start:m.start()].strip():
                        continue
                    body = body[:m.start()] + "pub " + body[m.start() + len(vis or ""):]
                    print(f"pub {t['parent']}: {name}")
            files[parent] = body
        files[test] = text
        r = os.path.relpath(test, art)
        mounts.append(f"#[cfg(test)]\n#[path = \"{r}\"]\n{'pub(crate) ' if t.get('shared') else ''}mod {t['name']};\n")
    rp = get(root)
    anchor = spec["anchor"]
    if files[rp].count(anchor) != 1:
        sys.exit("anchor must match once in the root")
    i = files[rp].index(anchor) + len(anchor)
    files[rp] = files[rp][:i] + "".join("\n" + m for m in mounts) + files[rp][i:]
    for old, new in spec.get("renames", {}).items():
        for f in list(crate.files):
            p = get(f)
            files[p] = files[p].replace(old, new)
    changed = 0
    for p, text in files.items():
        if text != open(p, encoding="utf-8").read():
            changed += 1
            print(f"{'write' if mode == 'apply' else 'would write'} {os.path.relpath(p, art)}")
            if mode == "apply":
                open(p, "w", encoding="utf-8").write(text)
    print(f"{changed} files")


main()
