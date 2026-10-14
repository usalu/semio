#!/usr/bin/env python3
"""🔗️ Item-level cross-file references of one `#[path]`-tree crate: every `crate::`/`super::`/`self::` path (use leaves and
inline paths) resolved to (owning file, first item segment). Prints TSV `src-file  T|-  target-file  item  count`, paths
relative to the crate root directory, test-gated sources marked `T`.

usage: 🔗️item-refs.py <crate-root.rs> [--from <regex>] [--to <regex>]
"""
import collections, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ns = {}
exec(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], ns)
Crate, flatten, inline_spans, test_spans = ns["Crate"], ns["flatten"], ns["inline_spans"], ns["test_spans"]
USE_INLINE = re.compile(r"\b(crate|super|self)((?:\s*::\s*(?:[A-Za-z_][A-Za-z0-9_]*|super))+)(\s*::\s*\{)?")


def resolve(crate, segs):
    """🧭️ Longest module prefix (through private `component` glob mounts) → (file, rest)."""
    cur, k = (), 0
    while k < len(segs):
        if cur + (segs[k],) in crate.mods:
            cur = cur + (segs[k],)
        elif cur + ("component", segs[k]) in crate.mods:
            cur = cur + ("component", segs[k])
        else:
            break
        k += 1
    target = crate.mods.get(cur + ("component",)) if cur + ("component",) in crate.mods else crate.mods.get(cur, crate.root)
    return target, segs[k] if k < len(segs) else "<module>"


def main():
    root = sys.argv[1]
    want_from = re.compile(sys.argv[sys.argv.index("--from") + 1]) if "--from" in sys.argv else None
    want_to = re.compile(sys.argv[sys.argv.index("--to") + 1]) if "--to" in sys.argv else None
    crate = Crate(root)
    rel = lambda f: os.path.relpath(f, crate.base)
    rows = collections.Counter()
    for file, modpath in crate.files.items():
        s = crate.text[file]
        spans = inline_spans(s)
        tests = test_spans(s)
        is_test = file in crate.test_files or "🧪️tests" in rel(file)

        def base_at(off):
            base = list(modpath)
            for a, b, name in spans:
                if a <= off < b:
                    base.append(name)
            return base

        paths, covered = [], []
        for m in re.finditer(r"\buse\s+(crate|super|self)\s*::", s):
            j, d, k = m.end(), 0, m.end()
            while k < len(s):
                if s[k] == "{": d += 1
                elif s[k] == "}": d -= 1
                elif s[k] == ";" and d == 0: break
                k += 1
            covered.append((m.start(), k))
            for p in flatten([m.group(1)], s[j:k]):
                paths.append((m.start(), p))
        for m in USE_INLINE.finditer(s):
            if m.group(3) or any(a <= m.start() < b for a, b in covered):
                continue
            paths.append((m.start(), [x for x in re.split(r"\s*::\s*", (m.group(1) + m.group(2)).strip()) if x]))
        for off, segs in paths:
            if segs[0] == "crate":
                absolute = segs[1:]
            else:
                base, k = base_at(off), 0
                while k < len(segs) and segs[k] in ("super", "self"):
                    if segs[k] == "super" and base:
                        base.pop()
                    k += 1
                absolute = base + segs[k:]
            target, item = resolve(crate, tuple(absolute))
            if target == file:
                continue
            gated = is_test or any(a <= off < b for a, b in tests)
            if want_from and not want_from.search(rel(file)):
                continue
            if want_to and not want_to.search(rel(target)):
                continue
            rows[(rel(file), "T" if gated else "-", rel(target), item)] += 1
    for (src, t, dst, item), n in sorted(rows.items()):
        print(f"{src}\t{t}\t{dst}\t{item}\t{n}")


main()
