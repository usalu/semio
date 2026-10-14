#!/usr/bin/env python3
"""🕸️ Plans a family cut: per candidate crate its lib/test unit size (from a `📋️unit-files.ts` dump, embedded inputs
included) and every inter-crate reference edge, split into lib edges and test-only edges, plus strongly connected
components of the lib graph. Uses the module resolution of `✂️family-cut.py` with the same config.

usage: 🕸️family-graph.py <config.json> <files.tsv> [--edges N] [--crate <package>]
"""
import collections, importlib.util, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("family_cut", os.path.join(HERE, "✂️family-cut.py"))
fc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fc)


def main():
    config_path, tsv = sys.argv[1], sys.argv[2]
    show = int(sys.argv[sys.argv.index("--edges") + 1]) if "--edges" in sys.argv else 6
    only = sys.argv[sys.argv.index("--crate") + 1] if "--crate" in sys.argv else None
    cut = fc.Cut(config_path)
    tests = set()
    for file in cut.tests:
        for mod in cut.tree.walk(cut.tree.files[file]):
            if mod.kind == "file":
                tests.add(mod.file)
    unit = {}
    for line in open(tsv, encoding="utf-8"):
        kind_unit, kind, lines, size, path = line.rstrip("\n").split("\t")
        unit.setdefault(path, {})[kind_unit] = (kind, int(lines), int(size))
    embeds = collections.defaultdict(set)
    for file in cut.tree.files:
        for m in re.finditer(r'include_(?:str|bytes)!\s*\(\s*"([^"]+)"', cut.tree.raw[file]):
            embeds[file].add(os.path.normpath(os.path.join(os.path.dirname(file), m.group(1))))
    size = collections.defaultdict(lambda: {"lib": [0, 0, 0], "test": [0, 0, 0]})
    seen = collections.defaultdict(lambda: {"lib": set(), "test": set()})
    repo = fc.REPO
    for file, mod in cut.tree.files.items():
        crate = cut.crate_of[file] if file not in tests else "[[test]] " + os.path.relpath(file, cut.artifact)
        rel = os.path.relpath(file, repo)
        row = unit.get(rel, {})
        is_test = mod.test_gated or file in tests
        for u in (["test"] if is_test else ["lib", "test"]):
            entry = row.get(u) or row.get("test") or ("source", 0, os.path.getsize(file))
            for path, (kind, lines, nbytes) in [(rel, entry)] + [(os.path.relpath(e, repo), unit.get(os.path.relpath(e, repo), {}).get("test", ("embedded", 0, os.path.getsize(e) if os.path.exists(e) else 0))) for e in embeds[file]]:
                if path in seen[crate][u]:
                    continue
                seen[crate][u].add(path)
                size[crate][u][0] += 1
                size[crate][u][1] += lines
                size[crate][u][2] += nbytes
    edges = collections.defaultdict(lambda: collections.defaultdict(list))
    for file, mod in cut.tree.files.items():
        own = cut.crate_of[file] if file not in tests else "[[test]]"
        text = cut.tree.text[file]
        spans = fc.test_spans(text)
        use_spans = [(u.start(), u.end()) for u in re.finditer(r"\buse\s+[^;]*;", text)]
        moved = [(u.start(), u.end()) for u in re.finditer(r"(?s)//#region 🚚️Moved.*?//#endregion 🚚️Moved", cut.tree.raw[file])]
        for m in re.finditer(r"(?<![\w:$])(?P<path>(?:crate|super|self)(?:" + fc.SEP + r"(?:" + fc.IDENT + r"|super))+)", text):
            if any(a <= m.start() < b for a, b in use_spans):
                continue
            segs = [x for x in re.split(fc.SEP, m.group("path")) if x]
            hit = cut.target(file, m.start(), segs)
            if not hit:
                continue
            crate = hit[0]
            if crate == own or crate == fc.EXTERNAL:
                continue
            test = mod.test_gated or file in tests or any(a <= m.start() < b for a, b in spans)
            edges[(own, crate)]["test" if test else "lib"].append((os.path.relpath(file, cut.artifact), "::".join(hit[2])))
        for m in re.finditer(r"\buse\s+(?:crate|super|self)\s*::", text):
            if any(a <= m.start() < b for a, b in moved):
                continue
            j, depth = m.end(), 0
            while j < len(text) and not (text[j] == ";" and depth == 0):
                depth += text[j] == "{"
                depth -= text[j] == "}"
                j += 1
            body = text[m.start() + 4:j].strip()
            for segs, alias in fc.leaves([], body):
                path = [s for s in segs if s not in ("*", "self")] if segs[-1] in ("*", "self") else segs
                hit = cut.target(file, m.start(), path) if len(path) > 1 or path[0] != "crate" else None
                if not hit or hit[0] == own or hit[0] == fc.EXTERNAL:
                    continue
                test = mod.test_gated or file in tests or any(a <= m.start() < b for a, b in spans)
                edges[(own, hit[0])]["test" if test else "lib"].append((os.path.relpath(file, cut.artifact), "::".join(hit[2])))
    print("| crate | lib files | lib lines | lib bytes | test files | test lines | test bytes |")
    print("|---|---:|---:|---:|---:|---:|---:|")
    for crate in list(cut.crates) + sorted(k for k in size if k.startswith("[[test]]")):
        l, t = size[crate]["lib"], size[crate]["test"]
        flag = " OVER" if l[0] > 32 or l[1] > 4000 or l[2] > 262144 or t[0] > 32 or t[1] > 4000 or t[2] > 262144 else ""
        print(f"| {crate}{flag} | {l[0]} | {l[1]} | {l[2]} | {t[0]} | {t[1]} | {t[2]} |")
    print("\n## edges")
    graph = collections.defaultdict(set)
    for (a, b), kinds in sorted(edges.items()):
        if only and only not in (a, b):
            continue
        print(f"{a} -> {b}: lib {len(kinds['lib'])} test {len(kinds['test'])}")
        if kinds["lib"]:
            graph[a].add(b)
        for kind in ("lib", "test"):
            for f, p in sorted(set(kinds[kind]))[:show]:
                print(f"    {kind} {f} :: {p}")
    index, low, stack, on, sccs, counter = {}, {}, [], set(), [], [0]

    def strong(v):
        index[v] = low[v] = counter[0]
        counter[0] += 1
        stack.append(v)
        on.add(v)
        for w in graph[v]:
            if w not in index:
                strong(w)
                low[v] = min(low[v], low[w])
            elif w in on:
                low[v] = min(low[v], index[w])
        if low[v] == index[v]:
            comp = []
            while True:
                w = stack.pop()
                on.discard(w)
                comp.append(w)
                if w == v:
                    break
            if len(comp) > 1:
                sccs.append(comp)

    for v in list(graph):
        if v not in index:
            strong(v)
    print("\n## lib cycles")
    for comp in sccs:
        print(" <-> ".join(sorted(comp)))


main()
