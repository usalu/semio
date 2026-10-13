#!/usr/bin/env python3
"""🧹️ Exact import blocks for the decomposed `semio-framework-plugin` modules (wave E2, step 1).

Every decomposed module starts with the private import block of the old `app` module (its preamble) plus the
crate imports it needs. This tool keeps that block exact across every build configuration:

- `flatten`  rebuilds each module's leading private `use` block as one leaf per line: the original preamble of
             `app` (read from the committed root, so no import is ever guessed) plus every crate-local leaf the
             module already imports (keeping its gate). Each line carries a `//⟨statement⟩` tag naming the
             original statement, so docs and grouping survive.
- `regate L` turns every leaf the native build `L` reports unused into a leaf gated with the test-support
             configuration (`any(test, feature = "artifact-app-testing")`).
- `prune L`  deletes every leaf the test build `L` reports unused (it compiles every gated leaf).
- `group`    re-emits the block statement by statement (original order and docs), one statement per gate.

Logs are cargo `--message-format=json` outputs. Usage: imports.py flatten|regate LOG|prune LOG|group
"""
import importlib.util, json, os, re, subprocess, sys
from collections import OrderedDict, defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rust_items", os.path.join(os.path.dirname(HERE), "e1-os-kernel", "🧱️rust_items.py"))
ri = importlib.util.module_from_spec(spec); spec.loader.exec_module(ri)
REPO = "/Users/ueli/Documents/semio"
PLUGIN = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
DEST = json.load(open(os.path.join(HERE, "🗂️destinations.json"), encoding="utf-8"))
FILES = [os.path.join(PLUGIN, d["dir"], "🦀️.rs") for d in DEST["destinations"]]
DEST_MODULE = {os.path.join(PLUGIN, d["dir"], "🦀️.rs"): d["module"] for d in DEST["destinations"]}
GATE = 'any(test, feature = "artifact-app-testing")'
BASE_COMMIT = "c73888a85bb"
TAG = re.compile(r"\s*//⟨([^⟩]*)⟩\s*$")


def parse_tree(s):
    """🌳️ Flattens one use tree into `(path, alias)` leaves (`alias` None, an identifier or `_`)."""
    s = re.sub(r"\s+", " ", s).strip()
    out = []

    def split_top(body):
        depth, cur, parts = 0, "", []
        for ch in body:
            if ch == "{": depth += 1
            if ch == "}": depth -= 1
            if ch == "," and depth == 0: parts.append(cur); cur = ""
            else: cur += ch
        if cur.strip(): parts.append(cur)
        return [p.strip() for p in parts if p.strip()]

    def walk(prefix, t):
        t = t.strip()
        if t.startswith("{") and t.endswith("}"):
            for p in split_top(t[1:-1]): walk(prefix, p)
            return
        m = re.match(r"^((?:[\w]+\s*::\s*)*)\{(.*)\}$", t, re.S)
        if m:
            pre = [x.strip() for x in m.group(1).split("::") if x.strip()]
            for p in split_top(m.group(2)): walk(prefix + pre, p)
            return
        m = re.match(r"^(.*?)\s+as\s+(\w+)$", t)
        alias = None
        if m: t, alias = m.group(1).strip(), m.group(2)
        segs = [x.strip() for x in t.split("::") if x.strip()]
        if segs == ["self"]: segs = []
        if segs and segs[-1] == "self": segs = segs[:-1]
        out.append(("::".join(prefix + segs), alias))
    walk([], s)
    return out


def bound(path, alias):
    if alias == "_" or path.endswith("*"): return None
    return alias or path.split("::")[-1]


def leaf_line(path, alias, gate, tag):
    body = f"use {path}" + (f" as {alias}" if alias else "") + ";"
    return (f"#[cfg({gate})] " if gate else "") + body + f" //⟨{tag}⟩"


def header_items(raw):
    cl = ri.clean(raw)
    its, _ = ri.items(raw, cl, 0, len(cl))
    head = []
    for it in its:
        if it["kind"] != "use" or re.match(r"\s*pub", cl[it["head"]:it["end"]]): break
        head.append(it)
    return head, cl


def statement_of(raw, cl, it):
    attrs = raw[it["start"]:it["head"]]
    gate = None
    for a in re.findall(r"#\[\s*cfg\((.*?)\)\s*\]", attrs, re.S):
        gate = re.sub(r"\s+", " ", a.strip())
    docs = [l.strip() for l in attrs.split("\n") if l.strip().startswith("//") and not TAG.search(l)]
    body = re.sub(r"\s+", " ", cl[it["head"]:it["end"]].strip())
    m = re.match(r"use\s+(.*);$", body)
    return gate, docs, m.group(1) if m else None


def original_preamble():
    raw = subprocess.run(["git", "show", f"{BASE_COMMIT}:{os.path.relpath(PLUGIN, REPO)}/🦀️.rs"], cwd=REPO, capture_output=True, text=True).stdout
    cl = ri.clean(raw)
    top, _ = ri.items(raw, cl, 0, len(cl))
    app = next(it for it in top if it["kind"] == "mod" and it["name"] == "app")
    ob = cl.index("{", app["head"]); cb = ri.skip_group(cl, ob) - 1
    items, _ = ri.items(raw, cl, ob + 1, cb)
    local = {it["name"] for it in items if it["kind"] == "mod"}
    stmts = []
    for it in items:
        if it["kind"] != "use": continue
        body = re.sub(r"\s+", " ", cl[it["head"]:it["end"]].strip())
        m = re.match(r"(?:pub(?:\s*\([^)]*\))?\s+)?use\s+(.*);$", body)
        first = re.split(r"::|\{", m.group(1).strip())[0].strip()
        if first in local or first == "self": continue
        attrs = raw[it["start"]:it["head"]]
        gate = None
        for a in re.findall(r"#\[\s*cfg\((.*?)\)\s*\]", attrs, re.S): gate = re.sub(r"\s+", " ", a.strip())
        docs = [l.strip() for l in attrs.split("\n") if l.strip().startswith("//") and not re.match(r"//\s*#(?:end)?region", l.strip()) and not l.strip().startswith("//!")]
        allow = "#[allow(unused_imports)]" in attrs
        stmts.append({"gate": gate, "docs": docs, "leaves": parse_tree(m.group(1)), "allow": allow})
    inline = [(ri.skip_group(cl, cl.index("{", it["head"])) if "{" in cl[it["head"]:it["end"]] else it["end"], it["start"]) for it in items if it["kind"] == "mod"]
    for it in items:
        if it["kind"] != "macro_call" or it["name"] != "include": continue
        lit = re.search(r'include!\(\s*"([^"]+)"', raw[it["head"]:it["end"]])
        if not lit: continue
        attrs = raw[it["start"]:it["head"]]
        igate = None
        for a in re.findall(r"#\[\s*cfg\((.*?)\)\s*\]", attrs, re.S): igate = re.sub(r"\s+", " ", a.strip())
        inc = os.path.join(PLUGIN, lit.group(1))
        iraw = open(inc, encoding="utf-8").read(); icl = ri.clean(iraw)
        iits, _ = ri.items(iraw, icl, 0, len(icl))
        for jt in iits:
            if jt["kind"] != "use": continue
            body = re.sub(r"\s+", " ", icl[jt["head"]:jt["end"]].strip())
            mm = re.match(r"use\s+(.*);$", body)
            if not mm: continue
            leaves = [(p_, a_) for p_, a_ in parse_tree(mm.group(1)) if re.match(r"^(std|core|alloc|semio_\w+|store|protocol|dsl|vcs|serde\w*|ui_wgpu|pack)\b", p_)]
            if leaves:
                stmts.append({"gate": igate, "docs": [], "leaves": leaves, "allow": False})
    return stmts


def owner_index():
    """🗂️ Crate item name -> its owning module path; a component-level definition wins over nested ones and
    definitions win over re-exports. Names generated by the crate's own macros are indexed from their invocations."""
    spec2 = importlib.util.spec_from_file_location("facet_imports", os.path.join(HERE, "🧭️facet-imports.py"))
    fi = importlib.util.module_from_spec(spec2); spec2.loader.exec_module(fi)
    fi.walk(f"{PLUGIN}/📦️packages/🦀️rust/🦀️.rs", "crate", False, f"{PLUGIN}/📦️packages/🦀️rust")
    cands = defaultdict(set)
    for mod in fi.modules:
        segs = mod.path.split("::")
        if len(segs) < 3 or segs[2] in ("app", "plugin_app_close_prelude"): continue
        raw, cl = fi.load(mod.file)
        base = cl[mod.rng[0]:mod.rng[0] + 1] == "{"
        its, _ = ri.items(raw, cl, mod.rng[0] + (1 if base else 0), mod.rng[1])
        path = "crate::" + "::".join(segs[2:])
        depth = len(segs)
        for it in its:
            if it["name"] and it["kind"] in ("struct", "enum", "trait", "type", "fn", "const", "static", "union", "mod"):
                cands[it["name"]].add((0 if depth == 3 else 1, 0, path))
            if it["kind"] == "macro_call" and it["name"] == "framework_reserved_job":
                for nm in re.findall(r"\b([A-Z]\w+)\b", cl[it["head"]:it["end"]])[:2]: cands[nm].add((0 if depth == 3 else 1, 0, path))
            if it["kind"] == "macro_call" and it["name"] == "dyn_enum_close":
                for nm in re.findall(r"enum\s+(\w+)", cl[it["head"]:it["end"]]): cands[nm].add((0 if depth == 3 else 1, 0, path))
            if it["kind"] == "use" and re.match(r"\s*pub", cl[it["head"]:it["end"]]):
                mm = re.match(r"(?:pub(?:\s*\([^)]*\))?\s+)?use\s+(.*);$", re.sub(r"\s+", " ", cl[it["head"]:it["end"]].strip()))
                if mm:
                    for p_, a_ in parse_tree(mm.group(1)):
                        b_ = bound(p_, a_)
                        if b_: cands[b_].add((0 if depth == 3 else 1, 1, path))
    out = {}
    for name, cs in cands.items():
        best = min(cs)
        top = [c for c in cs if c[:2] == best[:2]]
        if len({c[2] for c in top}) == 1: out[name] = best[2]
    return out


def normalize_crate_leaf(path, alias, owners):
    segs = path.split("::")
    if segs[0] != "crate": return path
    rest = segs[1:]
    if rest and rest[0] == "component": rest = rest[1:]
    if rest and rest[0] in ("app", "plugin_app_close_prelude") and len(rest) >= 2:
        rest = rest[1:]
        if rest[0] in owners: return owners[rest[0]] + "::" + "::".join(rest)
        return None
    if len(rest) == 1 and rest[0] in owners: return owners[rest[0]] + "::" + rest[0]
    if len(rest) == 1: return None
    return "crate::" + "::".join(rest)


def flatten():
    pre = original_preamble()
    meta = {}
    for f in FILES:
        raw = open(f, encoding="utf-8").read()
        head, cl = header_items(raw)
        if not head:
            start = end = len(raw.split("\n", 1)[0]) + 1
        else:
            start, end = head[0]["start"], head[-1]["end"]
        lines, stm = [], OrderedDict()
        taken = set(re.findall(r"^(?:pub(?:\([^)]*\))?\s+)?(?:const\s+|async\s+|unsafe\s+)*(?:fn|struct|enum|trait|type|const|static|mod|union)\s+(\w+)", cl, re.M))
        for im in re.finditer(r'^include!\(\s*"([^"]+)"\s*\);', raw, re.M):
            inc = os.path.normpath(os.path.join(os.path.dirname(f), im.group(1)))
            if not os.path.exists(inc): continue
            iraw = open(inc, encoding="utf-8").read(); icl = ri.clean(iraw)
            for jt in ri.items(iraw, icl, 0, len(icl))[0]:
                if jt["kind"] != "use": continue
                mm = re.match(r"(?:pub(?:\s*\([^)]*\))?\s+)?use\s+(.*);$", re.sub(r"\s+", " ", icl[jt["head"]:jt["end"]].strip()))
                if mm: taken |= {bound(p_, a_) for p_, a_ in parse_tree(mm.group(1))}
            taken |= set(re.findall(r"^(?:pub(?:\([^)]*\))?\s+)?(?:const\s+|async\s+|unsafe\s+)*(?:fn|struct|enum|trait|type|const|static|mod|union)\s+(\w+)", icl, re.M))
        bound_pre = set()
        for i, s in enumerate(pre):
            tag = f"p{i}"
            stm[tag] = {"docs": s["docs"], "allow": s["allow"]}
            for path, alias in s["leaves"]:
                b_ = bound(path, alias)
                if b_ and b_ in taken: continue
                if b_: bound_pre.add(b_)
                lines.append(leaf_line(path, alias, s["gate"], tag))
        k = 0
        seen = set()
        owners = flatten.owners
        for it in head:
            gate, docs, tree = statement_of(raw, cl, it)
            if tree is None: continue
            for path, alias in parse_tree(tree):
                if not re.match(r"^(crate|super|self)\b", path): continue
                if path.startswith("crate"):
                    path = normalize_crate_leaf(path, alias, owners)
                    if path is None: continue
                    if path.rsplit("::", 1)[0] == "crate::" + DEST_MODULE[f]: continue
                if bound(path, alias) in bound_pre: continue
                key = (path, alias, gate)
                if key in seen: continue
                seen.add(key)
                tag = f"c{k}"; k += 1
                stm[tag] = {"docs": [], "allow": False}
                lines.append(leaf_line(path, alias, gate, tag))
        inner = []
        for l in raw.split("\n"):
            if l.startswith("//!") or l.startswith("#!["): inner.append(l)
            else: break
        head_txt = "\n".join(inner)
        rest = raw[end:].lstrip("\n")
        if not head: rest = raw[len(head_txt):].lstrip("\n")
        new = head_txt + "\n" + "\n".join(lines) + "\n\n" + rest
        open(f, "w", encoding="utf-8").write(new)
        meta[os.path.relpath(f, PLUGIN)] = stm
    json.dump(meta, open(os.path.join(REPO, ".🧬semio/🦑️repo/⚡️cache/e2/imports-meta.json"), "w"), ensure_ascii=False, indent=1)
    print("flattened", len(FILES), "modules")


def unused_lines(log):
    hits = defaultdict(set)
    for line in open(log):
        try: j = json.loads(line)
        except ValueError: continue
        if j.get("reason") != "compiler-message": continue
        m = j["message"]
        if m["level"] != "warning" or (m.get("code") or {}).get("code") != "unused_imports": continue
        for s in m["spans"]:
            f = os.path.normpath(os.path.join(REPO, s["file_name"]))
            if f in FILES: hits[f].add(s["line_start"])
    return hits


def edit(log, action):
    n = 0
    for f, lines in unused_lines(log).items():
        src = open(f, encoding="utf-8").read().split("\n")
        for ln in sorted(lines, reverse=True):
            l = src[ln - 1]
            if not TAG.search(l): continue
            if action == "regate":
                if l.startswith("#[cfg("): continue
                src[ln - 1] = f"#[cfg({GATE})] " + l
            else:
                src[ln - 1] = None
            n += 1
        open(f, "w", encoding="utf-8").write("\n".join(x for x in src if x is not None))
    print(action, n, "leaves")


def render_tree(paths):
    tree = OrderedDict()
    for path, alias in paths:
        segs = path.split("::")
        node = tree
        for s_ in segs[:-1]: node = node.setdefault(s_, OrderedDict())
        node.setdefault(("leaf", segs[-1], alias), None)

    def emit(node):
        parts = []
        for k, v in node.items():
            if isinstance(k, tuple):
                parts.append(k[1] + (f" as {k[2]}" if k[2] else ""))
            else:
                inner = emit(v)
                parts.append(f"{k}::{inner[0]}" if len(inner) == 1 else f"{k}::{{{', '.join(inner)}}}")
        return parts
    top = emit(tree)
    return top[0] if len(top) == 1 else "{" + ", ".join(top) + "}"


def group():
    meta = json.load(open(os.path.join(REPO, ".🧬semio/🦑️repo/⚡️cache/e2/imports-meta.json"), encoding="utf-8"))
    for f in FILES:
        rel = os.path.relpath(f, PLUGIN)
        src = open(f, encoding="utf-8").read().split("\n")
        idx = [i for i, l in enumerate(src) if TAG.search(l)]
        if not idx: continue
        stm = OrderedDict()
        for i in idx:
            l = src[i]
            tag = TAG.search(l).group(1)
            l = TAG.sub("", l)
            gm = re.match(r"#\[cfg\((.*)\)\]\s*(use .*)$", l)
            gate, body = (gm.group(1), gm.group(2)) if gm else (None, l.strip())
            m = re.match(r"use\s+(.*?)(?:\s+as\s+(\w+))?;$", body)
            stm.setdefault(tag, OrderedDict()).setdefault(gate, []).append((m.group(1), m.group(2)))
        out = []
        for tag, gates in stm.items():
            info = meta.get(rel, {}).get(tag, {"docs": [], "allow": False})
            out.extend(info["docs"])
            for gate, paths in gates.items():
                prefix = (f"#[cfg({gate})]\n" if gate else "") + ("#[allow(unused_imports)]\n" if info["allow"] else "")
                out.append(prefix + f"use {render_tree(paths)};")
        a, b = idx[0], idx[-1]
        new = src[:a] + out + src[b + 1:]
        open(f, "w", encoding="utf-8").write("\n".join(new))
    print("grouped")


if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "flatten":
        flatten.owners = owner_index()
        flatten()
    elif cmd in ("regate", "prune"): edit(sys.argv[2], cmd)
    elif cmd == "group": group()
