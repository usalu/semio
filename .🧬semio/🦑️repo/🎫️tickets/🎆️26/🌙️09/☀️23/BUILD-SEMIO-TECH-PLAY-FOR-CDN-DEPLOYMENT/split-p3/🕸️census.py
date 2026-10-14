#!/usr/bin/env python3
"""🕸️ Partition census of one `#[path]`-tree artifact crate against the 4,000 lines / 262,144 bytes / 32 files unit ceiling.

Joins the canonical per-file closure (`📏️closure-dump.ts` JSON, lib and test cfg) with the module graph of the crate
(`🕸️module-graph.py` loader), attributes every embedded input to the file that includes it, folds files onto candidate
crates by ordered path rules, and prints per-crate lib/test unit sizes, the crate graph (lib edges and test edges), its
strongly connected components and the file edges that close every cycle.

usage: 🕸️census.py <partition.json> [--files CRATE] [--edges A B]
partition: {"artifact": dir, "closure": closure json, "rules": [[regex on artifact-relative path, crate]...]}
"""
import json, os, re, sys, collections

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ns = {}
src = open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0]
exec(compile(src, "module-graph", "exec"), ns)
Crate, test_spans, inline_spans, flatten = ns["Crate"], ns["test_spans"], ns["inline_spans"], ns["flatten"]
USE_RE = ns["USE_RE"]
CEIL = (4000, 262144, 32)
INCLUDE = re.compile(r'include_(?:str|bytes)!\s*\(\s*"((?:[^"\\]|\\.)*)"')


def unescape(text):
    return re.sub(r"\\u\{([0-9a-fA-F]+)\}", lambda m: chr(int(m.group(1), 16)), text).replace('\\"', '"').replace("\\\\", "\\")


REEXPORTS = {}


def build_reexports(crate):
    """🔁️ (module path, name) -> absolute target segments for every explicit `pub use crate::…` re-export."""
    for f, modpath in crate.files.items():
        s = crate.text[f]
        mods = [modpath] + ([modpath[:-1]] if modpath and modpath[-1] == "component" else [])
        for m in re.finditer(r"\bpub\s+use\s+(crate|super|self)\s*::", s):
            j = m.end(); d = 0; k = j
            while k < len(s):
                if s[k] == "{": d += 1
                elif s[k] == "}": d -= 1
                elif s[k] == ";" and d == 0: break
                k += 1
            body = s[j:k]
            for leaf in flatten([m.group(1)], body):
                alias = re.search(r"\bas\s+(\w+)\s*$", body) if "{" not in body else None
                if not leaf or leaf[-1] in ("*", "self"): continue
                segs = leaf
                if segs[0] == "crate": target = segs[1:]
                else:
                    base = list(modpath); k2 = 0
                    while k2 < len(segs) and segs[k2] in ("super", "self"):
                        if segs[k2] == "super" and base: base.pop()
                        k2 += 1
                    target = base + segs[k2:]
                for mp in mods:
                    REEXPORTS[(tuple(mp), target[-1])] = tuple(target)


def resolve(crate, mp, depth=0):
    for k in range(len(mp), -1, -1):
        p = mp[:k]
        if p in crate.mods or k == 0:
            if k < len(mp) and (p, mp[k]) in REEXPORTS and depth < 8:
                return resolve(crate, REEXPORTS[(p, mp[k])] + tuple(mp[k + 1:]), depth + 1)
            break
    return crate.resolve(mp)


def refs(crate, file):
    """🔗️ (target file, in_test_span) for every crate-relative path in one file."""
    s = crate.text[file]
    modpath = crate.files[file]
    spans = inline_spans(s)
    tests = test_spans(s)

    def base_at(off):
        base = list(modpath)
        for a, b, name in spans:
            if a <= off < b:
                base.append(name)
        return base

    paths = []
    for m in re.finditer(r"\buse\s+(crate|super|self)\s*::", s):
        j = m.end(); d = 0; k = j
        while k < len(s):
            if s[k] == "{": d += 1
            elif s[k] == "}": d -= 1
            elif s[k] == ";" and d == 0: break
            k += 1
        g = m.group(1) + "::" + s[j:k]
        head, rest = g.split("::", 1)
        for p in flatten([head], rest):
            paths.append((m.start(), p))
    for m in USE_RE.finditer(s):
        if m.group(3): continue
        paths.append((m.start(), [x for x in re.split(r"\s*::\s*", (m.group(1) + m.group(2)).strip()) if x]))
    if file == crate.root:
        for m in re.finditer(r"(?<![\w:])((?:standards|editor|viewer|examples|host|preview_eval|terminology|central_apply)(?:\s*::\s*[A-Za-z_][A-Za-z0-9_]*)+)", s):
            paths.append((m.start(), ["crate"] + [x for x in re.split(r"\s*::\s*", m.group(1)) if x]))
    out = set()
    for off, segs in paths:
        if not segs: continue
        if segs[0] == "crate":
            mp = tuple(segs[1:])
        else:
            base = base_at(off); k = 0
            while k < len(segs) and segs[k] in ("super", "self"):
                if segs[k] == "super" and base: base.pop()
                k += 1
            mp = tuple(base + segs[k:])
        target = resolve(crate, mp)
        if target != file:
            out.add((target, any(a <= off < b for a, b in tests), "::".join(mp)))
    return out


def load(partition_path):
    part = json.load(open(partition_path, encoding="utf-8"))
    base = os.path.dirname(os.path.abspath(partition_path))
    artifact = os.path.join(REPO, part["artifact"])
    closure = json.load(open(os.path.join(base, part["closure"]), encoding="utf-8"))
    crate = Crate(os.path.join(artifact, "🦀️.rs"))
    build_reexports(crate)
    rules = [(re.compile(r), c) for r, c in part["rules"]]
    rel = lambda f: os.path.relpath(f, artifact)

    def owner(f):
        r = rel(f)
        for rule, c in rules:
            if rule.search(r): return c
        return "?"

    lib = {os.path.join(REPO, i["path"]): i for i in closure["lib"]}
    test = {os.path.join(REPO, i["path"]): i for i in closure["test"]}
    embedder = {}
    for f in crate.files:
        raw = open(f, encoding="utf-8").read()
        for m in INCLUDE.finditer(raw):
            target = os.path.normpath(os.path.join(os.path.dirname(f), unescape(m.group(1))))
            embedder.setdefault(target, f)
    return part, artifact, crate, owner, lib, test, embedder, rel


def main():
    part, artifact, crate, owner, lib, test, embedder, rel = load(sys.argv[1])
    unit = collections.defaultdict(lambda: {"lib": [0, 0, 0], "test": [0, 0, 0]})
    members = collections.defaultdict(list)
    for kind, table in (("lib", lib), ("test", test)):
        for path, i in table.items():
            src_file = embedder.get(path, path) if i["kind"] == "embedded" else path
            if i["kind"] == "embedded" and path not in embedder:
                print(f"UNATTRIBUTED {kind} {os.path.relpath(path, REPO)}")
            c = owner(src_file)
            u = unit[c][kind]
            u[0] += i["lines"]; u[1] += i["bytes"]; u[2] += 1
            if kind == "test":
                members[c].append((i["lines"], i["bytes"], "L" if path in lib else "T", i["kind"][0], rel(path)))
    graph = {"lib": collections.defaultdict(collections.Counter), "test": collections.defaultdict(collections.Counter)}
    examples = collections.defaultdict(list)
    for f in crate.files:
        a = owner(f)
        ftest = f not in lib
        for g, in_test, text in refs(crate, f):
            b = owner(g)
            if a == b: continue
            kind = "test" if (ftest or in_test or g not in lib) else "lib"
            graph[kind][a][b] += 1
            examples[(a, b)].append((kind, rel(f), rel(g), text))
    names = sorted(unit)
    def cell(u):
        flag = "OVER" if u[0] > CEIL[0] or u[1] > CEIL[1] or u[2] > CEIL[2] else "ok  "
        return f"{flag} {u[0]:6d}L {u[1]:8d}B {u[2]:4d}F"
    print(f"{'crate':40s} {'lib':30s} {'test':30s} lib-deps / test-only-deps")
    for c in names:
        ld = sorted(graph["lib"][c]); td = sorted(set(graph["test"][c]) - set(ld))
        print(f"{c:40s} {cell(unit[c]['lib'])}  {cell(unit[c]['test'])}  {ld} / {td}")
    for kind in ("lib", "test"):
        edges = {a: set(bs) for a, bs in graph["lib"].items()}
        if kind == "test":
            for a, bs in graph["test"].items(): edges.setdefault(a, set()).update(bs)
        index, low, stack, on, sccs, counter = {}, {}, [], set(), [], [0]
        sys.setrecursionlimit(10000)
        def strong(v):
            index[v] = low[v] = counter[0]; counter[0] += 1; stack.append(v); on.add(v)
            for w in edges.get(v, ()):
                if w not in index: strong(w); low[v] = min(low[v], low[w])
                elif w in on: low[v] = min(low[v], index[w])
            if low[v] == index[v]:
                comp = []
                while True:
                    w = stack.pop(); on.discard(w); comp.append(w)
                    if w == v: break
                if len(comp) > 1: sccs.append(sorted(comp))
        for v in sorted(set(edges) | {b for bs in edges.values() for b in bs}):
            if v not in index: strong(v)
        for comp in sccs:
            print(f"\nCYCLE[{kind}] {comp}")
            for a in comp:
                for b in comp:
                    if b in edges.get(a, ()):
                        ex = [e for e in examples[(a, b)] if kind == "test" or e[0] == "lib"]
                        print(f"  {a} -> {b}: {len(ex)}  e.g. {ex[:3]}")
    if "--files" in sys.argv:
        c = sys.argv[sys.argv.index("--files") + 1]
        for row in sorted(members[c], reverse=True): print(*row, sep="\t")
    if "--dump" in sys.argv:
        json.dump({f"{a}|{b}": sorted(set(v)) for (a, b), v in examples.items()}, open(sys.argv[sys.argv.index("--dump") + 1], "w"), ensure_ascii=False)
    if "--edges" in sys.argv:
        i = sys.argv.index("--edges"); a, b = sys.argv[i + 1], sys.argv[i + 2]
        for e in sorted(set(examples[(a, b)])): print(*e, sep="\t")


main()
