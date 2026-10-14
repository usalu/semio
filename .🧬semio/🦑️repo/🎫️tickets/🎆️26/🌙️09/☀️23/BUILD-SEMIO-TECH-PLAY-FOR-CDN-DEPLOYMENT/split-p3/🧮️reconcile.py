#!/usr/bin/env python3
"""🧮️ Recomputes the dependency tables of every package of one Cargo workspace from what its sources name.

For each member manifest the compiled sources are the `#[path]` trees of its `[lib]` and `[[test]]` roots. An extern
crate counts as named by lib code when a non-test file (outside `🧪️tests`, `#[cfg(test)]` spans and `[[test]]` trees)
names it, otherwise by test code. `[dependencies]` gets every crate lib code names, `[dev-dependencies]` every crate
only tests name (plus a lib dependency a test needs with extra features, as before). Rows keep their existing spec
(features, renames, paths); a crate with no known row gets `{ workspace = true }` when the workspace declares it.
`extern crate X as alias` aliases of the root resolve to X. Family normal-dependency cycles are reported.

usage: 🧮️reconcile.py <workspace Cargo.toml> plan|apply [--only <package-substring>]
"""
import json, os, re, sys, collections

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ns = {}
exec(compile(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], "module-graph", "exec"), ns)
cut = {"re": re}
src = open(os.path.join(HERE, "✂️crate-cut.py"), encoding="utf-8").read()
exec(compile(src[src.index("def strip(src):"):src.index('census["strip"] = strip')], "strip", "exec"), cut)
ns["strip"] = cut["strip"]
Crate, strip = ns["Crate"], cut["strip"]
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
ALIASES = {"dsl": "semio_framework_os_kernel", "protocol": "semio_framework_os_kernel", "store": "semio_framework_os_kernel", "vcs": "semio_framework_os_kernel"}
TABLES = ["dependencies", "dev-dependencies", "target.'cfg(not(target_arch = \"wasm32\"))'.dev-dependencies"]


def section(text, table):
    return re.search(r"(?ms)^\[" + re.escape(table) + r"\]\s*\n(.*?)(?=^\[|\Z)", text)


def rows(text, table):
    out = collections.OrderedDict()
    sec = section(text, table)
    if not sec:
        return out
    for line in sec.group(1).splitlines():
        m = re.match(r'^([A-Za-z0-9_\-"]+)\s*=\s*(.+)$', line.strip())
        if m:
            key = m.group(1).strip('"')
            pkg = re.search(r'package\s*=\s*"([^"]+)"', m.group(2))
            out[key] = (m.group(2).strip(), (pkg.group(1) if pkg else key))
    return out


def test_spans(text):
    out = []
    for m in re.finditer(r"#\[cfg\(\s*(?:all\s*\(\s*)?test\b[^\]]*\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?(?:mod\s+" + IDENT + r"\s*|fn\s+[^{;]*|impl\b[^{;]*)\{", text):
        depth, j = 0, m.end() - 1
        while j < len(text):
            if text[j] == "{":
                depth += 1
            elif text[j] == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        out.append((m.start(), j))
    for m in re.finditer(r"#\[cfg\(\s*(?:all\s*\(\s*)?test\b[^\]]*\)\]\s*(?:pub(?:\s*\([^)]*\))?\s+)?use\b[^;]*;", text):
        out.append((m.start(), m.end()))
    return out


def named(path, is_test, aliases):
    text = strip(open(path, encoding="utf-8").read())
    spans = [] if is_test else test_spans(text)
    lib, test = set(), set()
    for m in re.finditer(r"(?<![\w:$.])(" + IDENT + r")\s*(?:::|!)|\buse\s+(?:::)?(" + IDENT + r")\b|\bextern\s+crate\s+(" + IDENT + r")", text):
        name = m.group(1) or m.group(2) or m.group(3)
        name = aliases.get(name, name)
        (test if is_test or any(a <= m.start() < b for a, b in spans) else lib).add(name)
    return lib, test


def main():
    ws_manifest = os.path.abspath(sys.argv[1])
    mode = sys.argv[2]
    only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else None
    ws_dir = os.path.dirname(ws_manifest)
    ws = open(ws_manifest, encoding="utf-8").read()
    members = re.search(r"(?s)members = \[(.*?)\]", ws).group(1)
    members = re.findall(r'"([^"]+)"', members)
    ws_deps = set(re.findall(r"(?m)^\[workspace\.dependencies\.([A-Za-z0-9_\-]+)\]", ws))
    sec = section(ws, "workspace.dependencies")
    if sec:
        ws_deps |= set(rows(ws, "workspace.dependencies"))
    manifests = {m: os.path.join(ws_dir, m, "Cargo.toml") for m in members}
    known = {}
    for m, path in manifests.items():
        text = open(path, encoding="utf-8").read()
        for table in TABLES:
            for key, (spec, pkg) in rows(text, table).items():
                rel = re.sub(r'path\s*=\s*"([^"]+)"', lambda mm: f'path = "{os.path.normpath(os.path.join(os.path.dirname(path), mm.group(1)))}"', spec)
                known.setdefault((table, key), rel)
                known.setdefault(("any", key), rel)
    family = {}
    for m, path in manifests.items():
        text = open(path, encoding="utf-8").read()
        name = re.search(r'(?m)^name\s*=\s*"([^"]+)"', text).group(1)
        family[name.replace("-", "_")] = name
    graph = collections.defaultdict(set)
    plans = {}
    for m, path in sorted(manifests.items()):
        text = open(path, encoding="utf-8").read()
        name = re.search(r'(?m)^name\s*=\s*"([^"]+)"', text).group(1)
        if only and only not in name:
            continue
        pkg_dir = os.path.dirname(path)
        lib_root = re.search(r'(?ms)^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"', text)
        lib_root = os.path.normpath(os.path.join(pkg_dir, lib_root.group(1) if lib_root else "src/lib.rs"))
        test_roots = [os.path.normpath(os.path.join(pkg_dir, p)) for p in re.findall(r'(?ms)^\[\[test\]\][^\[]*?^path\s*=\s*"([^"]+)"', text)]
        lib_files = Crate(lib_root).files
        root_text = open(lib_root, encoding="utf-8").read()
        aliases = dict(ALIASES)
        for a in re.finditer(r"extern\s+crate\s+(" + IDENT + r")\s+as\s+(" + IDENT + r")", root_text):
            aliases[a.group(2)] = a.group(1)
        lib, test = set(), set()
        crate = Crate(lib_root)
        for f in lib_files:
            is_test = f in crate.test_files or "🧪️tests" in os.path.relpath(f, os.path.dirname(lib_root))
            l, t = named(f, is_test, aliases)
            lib |= l
            test |= t
        for root in test_roots:
            if not os.path.exists(root):
                continue
            for f in Crate(root).files:
                l, t = named(f, True, aliases)
                test |= l | t
        current = {table: rows(text, table) for table in TABLES}
        by_ident = {}
        for table in TABLES:
            for key, (spec, pkg) in current[table].items():
                by_ident.setdefault(key.replace("-", "_"), []).append((table, key, spec, pkg))
        candidates = set()
        for (table, key) in known:
            candidates.add(key)
        candidates |= set(ws_deps) | set(family.values())
        new = {t: collections.OrderedDict() for t in TABLES}
        for table in TABLES:
            for key, (spec, pkg) in current[table].items():
                if key not in family.values() and pkg not in family.values() and "procedural-generation" not in pkg:
                    new[table][key] = spec
        for key in sorted(candidates):
            ident = key.replace("-", "_")
            if ident == name.replace("-", "_"):
                continue
            pkg_of = {k: p for (t, k), sp in known.items() for p in [re.search(r'package\s*=\s*"([^"]+)"', sp)] if p}
            in_lib = ident in lib
            in_test = ident in test
            if not (in_lib or in_test):
                continue
            existing = by_ident.get(ident, [])

            def spec_for(table):
                for t, k, s, p in existing:
                    if t == table:
                        return k, s
                if (table, key) in known:
                    s = known[(table, key)]
                elif ("any", key) in known:
                    s = known[("any", key)]
                elif key in ws_deps:
                    s = "{ workspace = true }"
                else:
                    return None
                s = re.sub(r'path\s*=\s*"([^"]+)"', lambda mm: f'path = "{os.path.relpath(mm.group(1), pkg_dir)}"', s)
                s = re.sub(r",?\s*optional\s*=\s*true", "", s)
                return key, s
            if in_lib:
                ks = spec_for("dependencies")
                if ks is None:
                    print(f"UNKNOWN {name}: {key}")
                    continue
                k, s = ks
                if "features" in s and existing and all(t != "dependencies" for t, *_ in existing):
                    s = re.sub(r",?\s*features\s*=\s*\[[^\]]*\]", "", s)
                new["dependencies"][k] = s
                if key in family.values():
                    graph[name].add(key)
                for t, k2, s2, p in existing:
                    if t != "dependencies" and "features" in s2:
                        new[t][k2] = s2
            elif in_test:
                tdev = [e for e in existing if e[0] == TABLES[2]]
                if tdev:
                    new[TABLES[2]][tdev[0][1]] = tdev[0][2]
                    continue
                ks = spec_for("dev-dependencies")
                if ks is None:
                    print(f"UNKNOWN {name}: {key} (test)")
                    continue
                new["dev-dependencies"][ks[0]] = ks[1]
        for k in list(new["dev-dependencies"]):
            if k in new["dependencies"] and "features" not in new["dev-dependencies"][k]:
                del new["dev-dependencies"][k]
        plans[path] = (text, new, current)
    out = {}
    for path, (text, new, current) in plans.items():
        body = text
        for table in TABLES:
            rows_text = "\n".join(f"{k} = {v}" for k, v in new[table].items())
            sec = section(body, table)
            if sec:
                if rows_text:
                    body = body[:sec.start(1)] + rows_text + "\n\n" + body[sec.end(1):].lstrip("\n")
                else:
                    head = body.rfind("[" + table + "]", 0, sec.start(1))
                    body = body[:head] + body[sec.end(1):].lstrip("\n")
            elif rows_text:
                anchor = re.search(r"(?m)^\[\[test\]\]", body)
                block = f"[{table}]\n{rows_text}\n\n"
                body = body[:anchor.start()] + block + body[anchor.start():] if anchor else body.rstrip("\n") + "\n\n" + block
        body = re.sub(r"\n{3,}", "\n\n", body)
        if body != text:
            out[path] = body
            added = {t: sorted(set(new[t]) - set(current[t])) for t in TABLES}
            removed = {t: sorted(set(current[t]) - set(new[t])) for t in TABLES}
            print(f"{os.path.relpath(path, REPO)}: +{ {k: v for k, v in added.items() if v} } -{ {k: v for k, v in removed.items() if v} }")
    index, low, stack, on, sccs, counter = {}, {}, [], set(), [], [0]
    sys.setrecursionlimit(10000)

    def strong(v):
        index[v] = low[v] = counter[0]; counter[0] += 1; stack.append(v); on.add(v)
        for w in graph.get(v, ()):
            if w not in index:
                strong(w); low[v] = min(low[v], low[w])
            elif w in on:
                low[v] = min(low[v], index[w])
        if low[v] == index[v]:
            comp = []
            while True:
                w = stack.pop(); on.discard(w); comp.append(w)
                if w == v:
                    break
            if len(comp) > 1:
                sccs.append(comp)
    for v in list(graph):
        if v not in index:
            strong(v)
    for comp in sccs:
        print("CYCLE " + " ".join(sorted(comp)))
    if mode == "apply":
        for path, body in out.items():
            open(path, "w", encoding="utf-8").write(body)
        print(f"applied {len(out)}")


main()
