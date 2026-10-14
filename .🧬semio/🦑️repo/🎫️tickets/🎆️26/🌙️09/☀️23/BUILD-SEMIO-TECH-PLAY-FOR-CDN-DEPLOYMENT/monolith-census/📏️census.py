#!/usr/bin/env python3.12
"""📏️ Monolith census: mounted Rust closure per Cargo package, path-dep graph, SCCs, upward edges."""

import collections
import os
import re
import sys
import time
import tomllib

REPO = "/Users/ueli/Documents/semio"
TICKET = os.path.join(REPO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT")
OUT = os.path.join(TICKET, "🗑️generated/monolith-census")
REPORT = os.path.join(TICKET, "📓️2026-10-14-monolith-census.md")
TICKET_ROOT = ".🧬semio"
FRAMEWORK = "🧰️framework"
HUB = "🌎️hub"
EXT = "✏️s"
PLUGINS = "🔌️plugins"
ARTIFACTS = "🗿️artifacts"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
PRUNE_NAMES = {".git", "target", "node_modules", "\U0001F5D1️generated"}
PRUNE_PATHS = {os.path.join(REPO, ".🧬semio/🦑️repo/", "⚡️cache")}
DEP_KEYS = ("dependencies", "build-dependencies")
AREA_ORDER = ["sdk", "framework", "hub", "artifact", "plugin-other", "ext-other", "tools", "ticket"]
AREA_LABEL = {
    "sdk": "Plugin SDK (`🛍️products/💻️os/🔨️modules/🔌️plugin`)",
    "framework": "Other framework",
    "hub": "Hub (`🌎️hub`)",
    "artifact": "Plugin artifact crates (`✏️s/🔌️plugins/*/🗿️artifacts/*`)",
    "plugin-other": "Plugin other (`✏️s/🔌️plugins/*`, non-artifact)",
    "ext-other": "Extensions, modules, dev, tests (`✏️s`, other)",
    "tools": "Tools (other top-level)",
    "ticket": "Ticket copies (scanned, excluded from canonical lists)",
}
RANK = {"sdk": 0, "framework": 0, "artifact": 1, "plugin-other": 1, "ext-other": 1, "hub": 2}
TOKEN_RE = re.compile(
    r"(?P<ws>\s+)"
    r"|(?P<lc>//[^\n]*)"
    r"|(?P<bc>/\*)"
    r"|(?P<raw>[bc]?r#*\")"
    r"|(?P<str>[bc]?\"(?:[^\"\\]|\\.)*\")"
    r"|(?P<chr>b?'(?:\\(?:u\{[^}\n]*\}|x[0-9A-Fa-f]{2}|.)|[^\\'\n])')"
    r"|(?P<life>'[^\W\d]\w*)"
    r"|(?P<id>[^\W\d]\w*)"
    r"|(?P<num>\d[\w.]*)"
    r"|(?P<p>.)",
    re.S,
)


def block_end(text, i):
    depth = 1
    n = len(text)
    while depth and i < n:
        a = text.find("/*", i)
        b = text.find("*/", i)
        if b < 0:
            return n
        if 0 <= a < b:
            depth += 1
            i = a + 2
        else:
            depth -= 1
            i = b + 2
    return i


def lex(text):
    toks = []
    code = set()
    pos = 0
    line = 1
    n = len(text)
    while pos < n:
        m = TOKEN_RE.match(text, pos)
        kind = m.lastgroup
        end = m.end()
        value = m.group(0)
        if kind == "bc":
            end = block_end(text, pos + 2)
            value = ""
        elif kind == "raw":
            h = m.group("raw").count("#")
            closing = '"' + "#" * h
            stop = text.find(closing, end)
            if stop < 0:
                stop = n
                end = n
            else:
                value = text[end:stop]
                end = stop + len(closing)
            kind = "str"
        elif kind == "str":
            body = value[value.index('"') + 1:-1]
            value = body.replace("\\\\", "\\")
        nl = text.count("\n", pos, end)
        if kind not in ("ws", "lc", "bc"):
            code.update(range(line, line + nl + 1))
            if kind == "str":
                toks.append(("str", value))
            elif kind in ("id", "p"):
                toks.append((kind, value))
            else:
                toks.append((kind, ""))
        line += nl
        pos = end
    return toks, code


def scan_events(toks):
    events = []
    stack = []
    depth = 0
    pending = None
    i = 0
    n = len(toks)
    while i < n:
        kind, val = toks[i]
        if kind == "p" and val == "#":
            j = i + 1
            if j < n and toks[j] == ("p", "!"):
                j += 1
            if j < n and toks[j] == ("p", "["):
                level = 0
                k = j
                attr = []
                while k < n:
                    t = toks[k]
                    if t == ("p", "["):
                        level += 1
                    elif t == ("p", "]"):
                        level -= 1
                        if level == 0:
                            break
                    attr.append(t)
                    k += 1
                for a in range(len(attr) - 2):
                    if attr[a] == ("id", "path") and attr[a + 1] == ("p", "=") and attr[a + 2][0] == "str":
                        pending = attr[a + 2][1]
                i = k + 1
                continue
        if kind == "id" and val == "mod" and i + 2 < n and toks[i + 1][0] == "id":
            name = toks[i + 1][1]
            nxt = toks[i + 2]
            if nxt == ("p", ";"):
                events.append(("mod", name, pending, tuple(s[0] for s in stack)))
                pending = None
                i += 3
                continue
            if nxt == ("p", "{"):
                depth += 1
                stack.append((name, depth))
                pending = None
                i += 3
                continue
        if (kind == "id" and val == "include" and i + 4 < n and toks[i + 1] == ("p", "!")
                and toks[i + 2] == ("p", "(") and toks[i + 3][0] == "str" and toks[i + 4] == ("p", ")")):
            events.append(("include", toks[i + 3][1], tuple(s[0] for s in stack)))
            i += 5
            continue
        if kind == "p":
            if val == "{":
                depth += 1
                pending = None
            elif val == "}":
                if stack and stack[-1][1] == depth:
                    stack.pop()
                depth -= 1
                pending = None
            elif val == ";":
                pending = None
        i += 1
    return events


CACHE = {}


def scan_source(path):
    info = CACHE.get(path)
    if info is None:
        with open(path, "rb") as f:
            raw = f.read()
        text = raw.decode("utf-8", errors="replace")
        lines = text.count("\n") + (0 if (not text or text.endswith("\n")) else 1)
        toks, code = lex(text)
        info = {"lines": lines, "active": len(code), "bytes": len(raw), "events": scan_events(toks)}
        CACHE[path] = info
    return info


def closure(roots):
    files = {}
    missing = []
    queue = collections.deque((r, os.path.dirname(r), None) for r in roots)
    while queue:
        path, mdir, parent = queue.popleft()
        if path in files:
            continue
        info = scan_source(path)
        files[path] = {"lines": info["lines"], "active": info["active"], "bytes": info["bytes"],
                       "parent": parent, "mdir": mdir}
        pdir = os.path.dirname(path)
        for ev in info["events"]:
            if ev[0] == "mod":
                _, name, pa, inl = ev
                base = os.path.join(mdir, *inl)
                if pa is not None:
                    base_p = base if inl else pdir
                    target = os.path.normpath(os.path.join(base_p, pa))
                    cands = [target]
                    child = os.path.dirname(target)
                else:
                    cands = [os.path.join(base, name + ".rs"), os.path.join(base, name, "mod.rs")]
                    child = os.path.join(base, name)
                hit = next((c for c in cands if os.path.isfile(c)), None)
                if hit is None:
                    missing.append((path, os.path.normpath(cands[0]), "mod"))
                else:
                    queue.append((os.path.normpath(hit), child, path))
            else:
                _, rel, inl = ev
                target = os.path.normpath(os.path.join(pdir, rel))
                if os.path.isfile(target):
                    queue.append((target, os.path.join(mdir, *inl), path))
                else:
                    missing.append((path, target, "include"))
    return files, missing


def package_roots(d, data):
    targets = []
    lib = data.get("lib")
    if isinstance(lib, dict) and "path" in lib:
        targets.append(os.path.join(d, lib["path"]))
    elif lib is not None or os.path.isfile(os.path.join(d, "src", "lib.rs")):
        targets.append(os.path.join(d, "src", "lib.rs"))
    for b in data.get("bin", []) or []:
        if isinstance(b, dict):
            if "path" in b:
                targets.append(os.path.join(d, b["path"]))
            else:
                targets.append(os.path.join(d, "src", "bin", str(b.get("name", "")) + ".rs"))
    if data["package"].get("autobins", True):
        main = os.path.join(d, "src", "main.rs")
        if os.path.isfile(main):
            targets.append(main)
        bindir = os.path.join(d, "src", "bin")
        if os.path.isdir(bindir):
            for e in sorted(os.listdir(bindir)):
                full = os.path.join(bindir, e)
                if e.endswith(".rs") and os.path.isfile(full):
                    targets.append(full)
                elif os.path.isfile(os.path.join(full, "main.rs")):
                    targets.append(os.path.join(full, "main.rs"))
    roots, missing = [], []
    for t in targets:
        t = os.path.normpath(t)
        if os.path.isfile(t):
            if t not in roots:
                roots.append(t)
        else:
            missing.append(t)
    return roots, missing


def dep_specs(data):
    tables = [data.get(k, {}) or {} for k in DEP_KEYS]
    for tgt in (data.get("target", {}) or {}).values():
        if isinstance(tgt, dict):
            tables += [tgt.get(k, {}) or {} for k in DEP_KEYS]
    for t in tables:
        for name, spec in t.items():
            yield name, spec


def area_of(rel):
    parts = rel.split("/")
    if parts[0].startswith(TICKET_ROOT):
        return "ticket"
    if rel == SDK or rel.startswith(SDK + "/"):
        return "sdk"
    if parts[0] == FRAMEWORK:
        return "framework"
    if parts[0] == HUB:
        return "hub"
    if parts[0] == EXT:
        if len(parts) > 4 and parts[1] == PLUGINS and parts[3] == ARTIFACTS:
            return "artifact"
        if len(parts) > 2 and parts[1] == PLUGINS:
            return "plugin-other"
        return "ext-other"
    return "tools"


def iter_files(root):
    for dp, dns, fns in os.walk(root, onerror=lambda e: None):
        keep = []
        for d in dns:
            full = os.path.join(dp, d)
            if d in PRUNE_NAMES or full in PRUNE_PATHS:
                continue
            keep.append(d)
        dns[:] = keep
        for f in fns:
            yield os.path.join(dp, f)


def tarjan(nodes, adj):
    index, low, onstack, stack, sccs = {}, {}, set(), [], []
    counter = 0
    for v in nodes:
        if v in index:
            continue
        index[v] = low[v] = counter
        counter += 1
        stack.append(v)
        onstack.add(v)
        work = [(v, iter(adj.get(v, ())))]
        while work:
            node, it = work[-1]
            advanced = False
            for w in it:
                if w not in index:
                    index[w] = low[w] = counter
                    counter += 1
                    stack.append(w)
                    onstack.add(w)
                    work.append((w, iter(adj.get(w, ()))))
                    advanced = True
                    break
                if w in onstack:
                    low[node] = min(low[node], index[w])
            if advanced:
                continue
            work.pop()
            if work:
                parent = work[-1][0]
                low[parent] = min(low[parent], low[node])
            if low[node] == index[node]:
                comp = []
                while True:
                    w = stack.pop()
                    onstack.discard(w)
                    comp.append(w)
                    if w == node:
                        break
                sccs.append(comp)
    return sccs


def write_tsv(path, header, rows):
    with open(path, "w", encoding="utf-8") as f:
        f.write("\t".join(header) + "\n")
        for r in rows:
            f.write("\t".join(str(x).replace("\t", " ") for x in r) + "\n")


def kib(b):
    return f"{b / 1024:,.0f}"


def main():
    t0 = time.time()
    for c in (FRAMEWORK, HUB, EXT, SDK, TICKET_ROOT):
        if not os.path.exists(os.path.join(REPO, c)):
            sys.exit(f"missing constant path: {c}")
    os.makedirs(OUT, exist_ok=True)

    all_files = list(iter_files(REPO))
    rs_files = sorted(f for f in all_files if f.endswith(".rs") and os.path.isfile(f))
    manifests_paths = [f for f in all_files if os.path.basename(f) == "Cargo.toml"]
    print(f"walk: {len(all_files)} files, {len(rs_files)} .rs, {len(manifests_paths)} Cargo.toml in {time.time() - t0:.1f}s", flush=True)

    manifests, parse_errors = {}, []
    for f in manifests_paths:
        d = os.path.dirname(f)
        try:
            with open(f, "rb") as fh:
                manifests[d] = tomllib.load(fh)
        except Exception as e:
            parse_errors.append((os.path.relpath(f, REPO), str(e)))

    ws_deps = {d: (data["workspace"].get("dependencies", {}) or {}) for d, data in manifests.items() if "workspace" in data}

    def nearest_workspace(d):
        cur = d
        while True:
            if cur in ws_deps:
                return cur
            parent = os.path.dirname(cur)
            if parent == cur:
                return None
            cur = parent

    pkg_dirs = [d for d, data in manifests.items() if "package" in data]
    pkg_set = set(pkg_dirs)
    print(f"packages: {len(pkg_dirs)} (parse errors: {len(parse_errors)})", flush=True)

    pk = {}
    unresolved_deps = []
    for d in pkg_dirs:
        data = manifests[d]
        rel = os.path.relpath(d, REPO)
        scope = "ticket" if rel.startswith(TICKET_ROOT) else "canonical"
        area = area_of(rel)
        roots, missing_roots = package_roots(d, data)
        files, missing = closure(roots)
        outside = sum(1 for p in files if not p.startswith(d + os.sep))
        deps = set()
        for name, spec in dep_specs(data):
            target = None
            if isinstance(spec, dict) and "path" in spec:
                target = os.path.normpath(os.path.join(d, spec["path"]))
            elif isinstance(spec, dict) and spec.get("workspace") is True:
                wsd = nearest_workspace(d)
                w = ws_deps.get(wsd, {}).get(name) if wsd else None
                if isinstance(w, dict) and "path" in w:
                    target = os.path.normpath(os.path.join(wsd, w["path"]))
            if target is None:
                continue
            if target in pkg_set:
                deps.add(target)
            else:
                unresolved_deps.append((rel, name, os.path.relpath(target, REPO)))
        pk[d] = {
            "name": data["package"].get("name", os.path.basename(d)), "rel": rel, "scope": scope, "area": area,
            "roots": roots, "missing_roots": missing_roots, "files": files, "missing": missing,
            "lines": sum(v["lines"] for v in files.values()), "active": sum(v["active"] for v in files.values()),
            "bytes": sum(v["bytes"] for v in files.values()), "nfiles": len(files), "outside": outside,
            "deps": deps, "dependents": set(), "dependents_t": 0,
        }
    print(f"closures done in {time.time() - t0:.1f}s; cache {len(CACHE)} sources", flush=True)

    for d, p in pk.items():
        for t in p["deps"]:
            pk[t]["dependents"].add(d)

    rev_t = {}
    for d in pk:
        seen = set()
        frontier = collections.deque(pk[d]["dependents"])
        while frontier:
            x = frontier.popleft()
            if x in seen:
                continue
            seen.add(x)
            frontier.extend(pk[x]["dependents"])
        rev_t[d] = len(seen)
    for d in pk:
        pk[d]["dependents_t"] = rev_t[d]

    adj = {d: p["deps"] for d, p in pk.items()}
    sccs = tarjan(list(pk), adj)
    cyc = [c for c in sccs if len(c) > 1]
    cyc_canon = [c for c in cyc if all(pk[x]["scope"] == "canonical" for x in c)]

    upward = []
    for d, p in pk.items():
        if p["scope"] != "canonical" or p["area"] not in RANK:
            continue
        for t in p["deps"]:
            q = pk[t]
            if q["area"] in RANK and RANK[p["area"]] < RANK[q["area"]]:
                upward.append((p["area"], q["area"], p["name"], p["rel"], q["name"], q["rel"]))
    upward.sort()

    canon = [p for p in pk.values() if p["scope"] == "canonical"]
    tick = [p for p in pk.values() if p["scope"] == "ticket"]
    uniq = {}
    for p in canon:
        for f, v in p["files"].items():
            uniq[f] = v
    canon_rs = [f for f in rs_files if not os.path.relpath(f, REPO).startswith(TICKET_ROOT)]
    ticket_rs = [f for f in rs_files if os.path.relpath(f, REPO).startswith(TICKET_ROOT)]
    unmounted = [f for f in canon_rs if f not in uniq]
    mounted_by = collections.Counter()
    for p in canon:
        for f in p["files"]:
            mounted_by[f] += 1

    rs_rows = []
    for f in rs_files:
        info = scan_source(f)
        rel = os.path.relpath(f, REPO)
        rs_rows.append((rel, "ticket" if rel.startswith(TICKET_ROOT) else "canonical", area_of(rel),
                        info["lines"], info["active"], info["bytes"], mounted_by.get(f, 0)))
    rs_rows.sort(key=lambda r: (-r[3], r[0]))

    pk_rows = []
    for d, p in sorted(pk.items(), key=lambda kv: (-kv[1]["lines"], kv[1]["rel"])):
        pk_rows.append((p["scope"], p["area"], p["name"], p["rel"], len(p["roots"]), p["lines"], p["active"],
                        p["nfiles"], p["bytes"], p["outside"], len(p["missing"]), len(p["deps"]),
                        len(p["dependents"]), p["dependents_t"]))
    write_tsv(os.path.join(OUT, "packages.tsv"),
              ["scope", "area", "name", "dir", "roots", "lines", "active", "files", "bytes", "outside_files",
               "missing_mods", "deps_out", "dependents_direct", "dependents_transitive"], pk_rows)
    mf_rows = []
    for p in canon:
        for f, v in sorted(p["files"].items()):
            mf_rows.append((p["rel"], os.path.relpath(f, REPO), v["lines"], v["active"], v["bytes"],
                            os.path.relpath(v["parent"], REPO) if v["parent"] else "(root)"))
    write_tsv(os.path.join(OUT, "mounted-files-canonical.tsv"),
              ["package_dir", "file", "lines", "active", "bytes", "mounted_from"], mf_rows)
    write_tsv(os.path.join(OUT, "rs-sources.tsv"),
              ["file", "scope", "area", "lines", "active", "bytes", "mounted_by_canonical"], rs_rows)
    edge_rows = [(pk[d]["rel"], pk[t]["rel"], pk[d]["area"], pk[t]["area"]) for d in pk for t in pk[d]["deps"]]
    write_tsv(os.path.join(OUT, "edges.tsv"), ["from_dir", "to_dir", "from_area", "to_area"], sorted(edge_rows))
    miss_rows = []
    for p in pk.values():
        for src, tgt, kind in p["missing"]:
            miss_rows.append((p["rel"], os.path.relpath(src, REPO), os.path.relpath(tgt, REPO), kind))
        for r in p["missing_roots"]:
            miss_rows.append((p["rel"], "(root)", os.path.relpath(r, REPO), "root"))
    write_tsv(os.path.join(OUT, "unresolved.tsv"), ["package_dir", "declared_in", "target", "kind"], sorted(miss_rows))
    write_tsv(os.path.join(OUT, "unresolved-deps.tsv"), ["package_dir", "dep", "target_dir"], sorted(unresolved_deps))
    write_tsv(os.path.join(OUT, "parse-errors.tsv"), ["manifest", "error"], parse_errors)

    lines_out = []
    w = lines_out.append

    def gt(key, t):
        return sum(1 for p in canon if p[key] > t)

    total_canon_lines = sum(p["lines"] for p in canon)
    uniq_lines = sum(v["lines"] for v in uniq.values())
    uniq_active = sum(v["active"] for v in uniq.values())
    w("# 📏️ Monolith Census (2026-10-14)")
    w("")
    w("Read-only census of every Cargo package in `/Users/ueli/Documents/semio`. Generated by `monolith-census/📏️census.py`; raw TSV/JSON under `🗑️generated/monolith-census/`.")
    w("")
    w("**Method and caveats**")
    w("")
    w("- Roots: the lib root (`[lib] path`, else `src/lib.rs`) plus every bin root (`[[bin]]`, `src/main.rs`, `src/bin/*`). `build.rs`, tests, examples, benches are not roots.")
    w("- Closure: `mod x;` (`x.rs` or `x/mod.rs`), `#[path]` (also inside `cfg_attr`), and `include!` of `.rs`. Inline modules and mod-rs / non-mod-rs directory rules are followed. `cfg` is ignored, so this is an upper bound.")
    w("- Lines = physical lines. Active = lines containing code (not blank, not comment-only; block comments and multi-line strings handled).")
    w("- Graph: `[dependencies]` and `[build-dependencies]` (plus target-specific) path deps, with `workspace = true` resolved from the nearest workspace root. Dev-dependencies are excluded.")
    w("- Ticket copies under `.🧬semio/` are scanned but kept out of canonical totals and top lists. Canonical = everything else.")
    w("- Closures overlap (shared files count in each package), so totals are given both as sums and as unique files.")
    w("")
    w("## 1. Totals")
    w("")
    w(f"- Cargo packages (`[package]`): **{len(canon)}** canonical, {len(tick)} ticket copies. Parse errors: {len(parse_errors)}.")
    w(f"- Canonical packages with no resolvable root: {sum(1 for p in canon if not p['roots'])}. Root paths declared but missing: {sum(len(p['missing_roots']) for p in canon)}.")
    w(f"- Mounted lines, summed over canonical packages (overlap counted): **{total_canon_lines:,}**.")
    w(f"- Unique canonical mounted files: **{len(uniq):,}** with {uniq_lines:,} lines ({uniq_active:,} active).")
    w(f"- All canonical `.rs` files: {len(canon_rs):,}. Not mounted by any canonical package: {len(unmounted):,} ({sum(scan_source(f)['lines'] for f in unmounted):,} lines).")
    w(f"- Ticket `.rs` files: {len(ticket_rs):,} ({sum(scan_source(f)['lines'] for f in ticket_rs):,} lines).")
    w(f"- Unresolved `mod`/`include` targets in canonical closures: {sum(len(p['missing']) for p in canon)} (mostly cfg-gated or generated files).")
    w("")
    w("## 2. Distribution (canonical packages)")
    w("")
    w("| Threshold | Mounted lines | Active lines |")
    w("|---|---:|---:|")
    for t in (4000, 15000, 50000):
        w(f"| > {t:,} | {gt('lines', t)} | {gt('active', t)} |")
    w("")
    w("Fleet per-unit ceiling (4,000 active lines / 262,144 bytes / 32 files), violations among canonical packages:")
    w("")
    w(f"- active > 4,000: {sum(1 for p in canon if p['active'] > 4000)}")
    w(f"- bytes > 262,144: {sum(1 for p in canon if p['bytes'] > 262144)}")
    w(f"- files > 32: {sum(1 for p in canon if p['nfiles'] > 32)}")
    w("")
    w("## 3. Top 60 canonical packages by mounted lines, grouped by area")
    w("")
    top60 = sorted(canon, key=lambda p: (-p["lines"], p["rel"]))[:60]
    rank_of = {id(p): i + 1 for i, p in enumerate(top60)}
    for area in AREA_ORDER:
        if area == "ticket":
            continue
        rows = [p for p in top60 if p["area"] == area]
        if not rows:
            continue
        w(f"**{AREA_LABEL[area]}**")
        w("")
        w("| # | Package | Lines | Active | Files | KiB | Directory |")
        w("|---:|---|---:|---:|---:|---:|---|")
        for p in rows:
            w(f"| {rank_of[id(p)]} | `{p['name']}` | {p['lines']:,} | {p['active']:,} | {p['nfiles']} | {kib(p['bytes'])} | `{p['rel']}` |")
        w("")
    w("## 4. Largest mounted files of the top 30 monoliths (split candidates)")
    w("")
    for i, p in enumerate(sorted(canon, key=lambda p: (-p["lines"], p["rel"]))[:30]):
        top_files = sorted(p["files"].items(), key=lambda kv: (-kv[1]["lines"], kv[0]))[:5]
        pdir = os.path.join(REPO, p["rel"])
        parts = "; ".join(f"`{os.path.relpath(f, pdir)}` {v['lines']:,}" for f, v in top_files)
        w(f"{i + 1}. `{p['name']}` ({p['lines']:,} lines): {parts}")
    w("")
    w("## 5. Dependency cycles (SCCs with more than one package)")
    w("")
    w(f"- SCCs total with >1 member: {len(cyc)} (canonical-only: {len(cyc_canon)}).")
    for c in cyc_canon:
        w(f"- Canonical cycle ({len(c)}): " + ", ".join(f"`{pk[x]['name']}`" for x in sorted(c, key=lambda x: pk[x]['rel'])))
    for c in cyc:
        if c not in cyc_canon:
            w(f"- Ticket-involved cycle ({len(c)}): " + ", ".join(f"`{pk[x]['rel']}`" for x in sorted(c, key=lambda x: pk[x]['rel'])[:4]) + " ...")
    if not cyc:
        w("- None.")
    w("")
    w("## 6. Upward edges (lower layer depends on higher layer)")
    w("")
    w("Layer rank: framework / plugin SDK = 0, `✏️s` plugins and extensions = 1, hub = 2. A normal edge goes from higher rank to lower rank.")
    w("")
    kinds = collections.Counter(f"{a} -> {b}" for a, b, *_ in upward)
    w(f"- Upward edges: {len(upward)} " + ("(" + ", ".join(f"{k}: {v}" for k, v in sorted(kinds.items())) + ")" if kinds else ""))
    for a, b, name, rel, tname, trel in upward[:60]:
        w(f"- `{name}` ({a}) -> `{tname}` ({b}); `{rel}` -> `{trel}`")
    if len(upward) > 60:
        w(f"- ... {len(upward) - 60} more in `🗑️generated/monolith-census/edges.tsv`.")
    if not upward:
        w("- None.")
    w("")
    w("## 7. Packages with the most dependents (canonical)")
    w("")
    w("| Package | Direct dependents | Transitive dependents | Area | Directory |")
    w("|---|---:|---:|---|---|")
    for p in sorted(canon, key=lambda p: (-p["dependents_t"], -len(p["dependents"]), p["rel"]))[:15]:
        w(f"| `{p['name']}` | {len(p['dependents'])} | {p['dependents_t']} | {p['area']} | `{p['rel']}` |")
    w("")
    w("## 8. Single Rust files over 4,000 lines (canonical, top 50)")
    w("")
    w("| Lines | Active | KiB | Mounted by | Area | File |")
    w("|---:|---:|---:|---:|---|---|")
    big = [r for r in rs_rows if r[1] == "canonical" and r[3] > 4000]
    for r in big[:50]:
        w(f"| {r[3]:,} | {r[4]:,} | {kib(r[5])} | {r[6]} | {r[2]} | `{r[0]}` |")
    w("")
    w(f"Canonical files over 4,000 lines: {len(big)}. Ticket-copy files over 4,000 lines: {sum(1 for r in rs_rows if r[1] == 'ticket' and r[3] > 4000)} (not listed).")
    w("")
    w("## 9. Unresolved declarations and parse issues")
    w("")
    w(f"- Unresolved `mod`/`include` in canonical closures: {sum(len(p['missing']) for p in canon)}; full list in `🗑️generated/monolith-census/unresolved.tsv`.")
    w(f"- Unresolved path deps (target is not a package): {len(unresolved_deps)}; see `unresolved-deps.tsv`.")
    w(f"- Manifest parse errors: {len(parse_errors)}.")
    w("")

    with open(REPORT, "w", encoding="utf-8") as f:
        f.write("\n".join(lines_out) + "\n")
    summary = {
        "canonical_packages": len(canon), "ticket_packages": len(tick),
        "canonical_mounted_lines_sum": total_canon_lines, "unique_mounted_files": len(uniq),
        "unique_mounted_lines": uniq_lines, "cycles": len(cyc), "canonical_cycles": len(cyc_canon),
        "upward_edges": len(upward), "unresolved_mods": sum(len(p["missing"]) for p in canon),
        "seconds": round(time.time() - t0, 1),
    }
    print(summary, flush=True)
    print(f"report: {REPORT}", flush=True)


if __name__ == "__main__":
    main()
