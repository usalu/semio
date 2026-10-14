#!/usr/bin/env python3
"""🪚️ Extracts module subtrees of a `#[path]`-tree monolith crate into a new crate rooted at an existing owner file.

Bottom-up and compile-atomic: the extracted subtree may only name framework crates and crates extracted before it, so
every `crate::`/`super::`/`self::` path that leaves the subtree is refused (printed as UPWARD) unless `--force`.

* The owner file (`<from>::component` of the first move with `"to": []`) becomes the crate root: the crate header
  (inner attributes, the `extern crate … as dsl/protocol/store/vcs` aliases the subtree uses) is inserted after its
  inner doc comment and the monolith root's inline mount tree below `<from>` is appended, re-based on the owner
  directory. Other moves are mounted as inline module chains at their `"to"` path.
* Paths: every path into a moved subtree is rewritten absolutely: `crate::<new path>` inside the new crate (relative
  paths that still resolve are kept), `<new crate>::<new path>` in the remaining monolith and in external consumers.
  Re-exports are followed; `pub use` facades of moved items are deleted, never re-pointed; private `component`
  segments are dropped. `use` groups that mix targets are split leaf by leaf, keeping attributes and visibility.
* `#[cfg(feature = "<gate>")]` attributes in moved files are evaluated as enabled (the gate dissolves into the crate
  boundary).
* Manifests: the new package manifest names exactly the crates its sources name (tests -> dev-dependencies), the
  workspace root gets the member and the dependency row, the monolith gets the dependency.

usage: 🪚️extract.py <spec.json> plan|apply [--force]
spec: {"artifact": dir, "monolith": crate ident, "package": name, "owner": dir relative to artifact,
       "moves": [{"from": "a::b", "to": ""}], "exclude": ["a::b::c"], "gate": "component-app-assembly",
       "consumers": [repo-relative roots], "description": "...", "monolith_optional": false}
"""
import collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ns = {}
exec(compile(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], "module-graph", "exec"), ns)
cut = {"re": re}
src = open(os.path.join(HERE, "✂️crate-cut.py"), encoding="utf-8").read()
exec(compile(src[src.index("def strip(src):"):src.index('census["strip"] = strip')], "crate-cut-strip", "exec"), cut)
strip = cut["strip"]
ns["strip"] = strip
Crate, flatten_paths = ns["Crate"], ns["flatten"]
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
SEP = r"\s*::\s*"
ALIASES = {"dsl": "semio_framework_os_kernel", "protocol": "semio_framework_os_kernel", "store": "semio_framework_os_kernel", "vcs": "semio_framework_os_kernel"}


def segs_of(text):
    return [x for x in re.split(SEP, text.strip()) if x]


def mod_tuple(text):
    return tuple(segs_of(text)) if text else ()


def split_top(text):
    parts, depth, cur = [], 0, ""
    for ch in text:
        if ch == "{":
            depth += 1
        if ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    return [p.strip() for p in parts if p.strip()]


def leaves(prefix, body):
    m = re.match(r"^((?:" + IDENT + SEP + r")*)\{(.*)\}$", body.strip(), re.S)
    if m:
        pre = prefix + segs_of(m.group(1).rstrip(": \n"))
        out = []
        for part in split_top(m.group(2)):
            out += leaves(pre, part)
        return out
    alias = re.search(r"\s+as\s+(" + IDENT + r")$", body.strip())
    core = re.sub(r"\s+as\s+" + IDENT + r"$", "", body.strip())
    return [(prefix + segs_of(core), alias.group(1) if alias else None)]


def inline_spans(text):
    spans = []
    for m in re.finditer(r"(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+(" + IDENT + r")\s*\{", text):
        depth, j = 0, m.end() - 1
        while j < len(text):
            if text[j] == "{":
                depth += 1
            elif text[j] == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        spans.append((m.end(), j, m.group(1)))
    return spans


def test_spans(text):
    out = []
    for m in re.finditer(r"#\[cfg\(\s*(?:all\s*\(\s*)?test\b[^\]]*\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?(?:mod\s+" + IDENT + r"\s*|fn\s+[^{;]*)\{", text):
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
    for m in re.finditer(r"#\[cfg\(\s*(?:all\s*\(\s*)?test\b[^\]]*\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?(?:use\b[^;]*|mod\s+" + IDENT + r"\s*);", text):
        out.append((m.start(), m.end()))
    return out


class Model:
    """🧠️ The monolith's module tree, re-exports and the move mapping."""

    def __init__(self, spec):
        self.spec = spec
        self.artifact = os.path.join(REPO, spec["artifact"])
        self.crate = Crate(os.path.join(self.artifact, "🦀️.rs"))
        self.ident = spec["monolith"]
        self.new_ident = spec["package"].replace("-", "_")
        self.moves = [(mod_tuple(m["from"]), mod_tuple(m["to"])) for m in spec["moves"]]
        self.excludes = [mod_tuple(e) for e in spec.get("exclude", [])]
        self.raw = {f: open(f, encoding="utf-8").read() for f in self.crate.files}
        self.reexports = {}
        self.build_reexports()
        self.moved_files = {f for f, mp in self.crate.files.items() if self.is_moved(mp)}
        self.shared_files = set()
        decl = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
        for f in self.crate.files:
            if f in self.moved_files:
                continue
            for m in decl.finditer(self.crate.text[f]):
                target = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
                if target in self.moved_files and "🧪️tests" in os.path.relpath(target, self.artifact):
                    self.shared_files.add(target)
        self.self_alias = False

    def is_moved(self, mp):
        mp = tuple(x for x in mp if x != "component") if not any(a and a[-1] == "component" for a, _ in self.moves) else tuple(mp)
        return any(mp[:len(a)] == a for a, _ in self.moves) and not any(mp[:len(e)] == e for e in self.excludes)

    def owner_component(self, frm):
        return frm + ("component",) in self.crate.mods and self.crate.mods[frm + ("component",)] != self.crate.mods.get(frm)

    def new_path(self, mp):
        """🧭️ New module path (inside the new crate) of an old absolute path, `component` segments dropped."""
        mp = tuple(x for x in mp if x != "component")
        best = max((m for m in self.moves if mp[:len(m[0])] == m[0]), key=lambda m: len(m[0]))
        rest = mp[len(best[0]):]
        return tuple(x for x in best[1] + rest if x != "component")

    def build_reexports(self):
        for f, modpath in self.crate.files.items():
            s = self.crate.text[f]
            mods = [tuple(modpath)] + ([tuple(modpath[:-1])] if modpath and modpath[-1] == "component" else [])
            spans = inline_spans(s)
            for m in re.finditer(r"\bpub\s+use\s+((?:crate|super|self)\s*::[^;]*);", s):
                base = list(modpath) + [name for a, b, name in spans if a <= m.start() < b]
                for leaf, alias in leaves([], m.group(1)):
                    if not leaf or leaf[-1] == "*":
                        continue
                    if leaf[-1] == "self":
                        leaf = leaf[:-1]
                    target = self.absolute(leaf, base)
                    if target is None:
                        continue
                    for mp in mods:
                        self.reexports[(mp + tuple(name for a, b, name in spans if a <= m.start() < b), alias or target[-1])] = tuple(target)

    @staticmethod
    def absolute(segs, base):
        if not segs:
            return None
        if segs[0] == "crate":
            return list(segs[1:])
        base = list(base)
        k = 0
        while k < len(segs) and segs[k] in ("super", "self"):
            if segs[k] == "super":
                if not base:
                    return None
                base.pop()
            k += 1
        return base + list(segs[k:])

    def canonical(self, mp, depth=0):
        """🔁️ Follows re-exports so that a path names its defining module."""
        mp = tuple(mp)
        for k in range(len(mp), -1, -1):
            p = mp[:k]
            if p in self.crate.mods or k == 0:
                if k < len(mp) and (p, mp[k]) in self.reexports and depth < 10:
                    return self.canonical(self.reexports[(p, mp[k])] + mp[k + 1:], depth + 1)
                if k < len(mp) and p + ("component",) in self.crate.mods and (p + ("component",), mp[k]) in self.reexports and depth < 10:
                    return self.canonical(self.reexports[(p + ("component",), mp[k])] + mp[k + 1:], depth + 1)
                return mp
        return mp


def child_heads(crate, f, spans):
    """🌿️ Names of the child modules a relative path in file `f` may start with (file module and its inline modules)."""
    bases = [tuple(crate.files[f])] + [tuple(crate.files[f]) + (name,) for a, b, name in spans]
    heads = {mp[-1] for mp in crate.mods for base in bases if len(mp) == len(base) + 1 and mp[:len(base)] == base and mp[-1] != "component"}
    for base in bases:
        if base + ("component",) in crate.mods:
            heads |= {mp[-1] for mp in crate.mods if len(mp) == len(base) + 2 and mp[:len(base) + 1] == base + ("component",)}
    return sorted(h for h in heads if h not in ("crate", "super", "self"))


def file_base(model, f, spans, offset):
    return list(model.crate.files[f]) + [name for a, b, name in spans if a <= offset < b]


def rewrite_file(model, f, text, raw, own_new, report):
    """🔁️ (start, end, replacement) edits for one monolith file (moved or remaining)."""
    spans = inline_spans(text)
    tspans = test_spans(text)
    edits, covered = [], []
    moved_file = f in model.moved_files

    def decide(segs, offset, written):
        if segs and segs[0] == model.ident:
            segs = ["crate"] + list(segs[1:])
            written = segs
        base = file_base(model, f, spans, offset)
        target = model.absolute(segs, base)
        if target is None:
            return None
        target = model.canonical(target)
        if model.is_moved(target):
            newp = model.new_path(target)
            if f in model.shared_files:
                model.self_alias = True
                return (model.new_ident, newp)
            if moved_file:
                if written[0] != "crate":
                    new_base = list(model.new_path(base)) if model.is_moved(base) else None
                    rel = model.absolute(written, new_base) if new_base is not None else None
                    if rel is not None and tuple(x for x in rel if x != "component") == newp:
                        return None
                return ("crate", newp)
            return (model.new_ident, newp)
        if moved_file:
            in_test = f in model.crate.test_files or "🧪️tests" in os.path.relpath(f, model.artifact) or any(a <= offset < b for a, b in tspans)
            if in_test and model.spec.get("test_upward") == "monolith":
                report["test_upward"].append((os.path.relpath(f, REPO), raw.count("\n", 0, offset) + 1, "::".join(target)))
                return (model.ident, tuple(x for x in target if x != "component"))
            report["upward"].append((os.path.relpath(f, REPO), raw.count("\n", 0, offset) + 1, "::".join(target)))
        return None

    heads_alt = "|".join(["crate", "super", "self", re.escape(model.ident)] + [re.escape(h) for h in child_heads(model.crate, f, spans)])
    use_re = re.compile(r"(?m)^(?P<attrs>(?:[ \t]*#\[[^\n]*\]\n)*)(?P<indent>[ \t]*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+(?P<body>(?:" + heads_alt + r")\b(?!\s*!)[^;]*);")
    for m in use_re.finditer(text):
        flat = leaves([], m.group("body"))
        rows, changed = [], False
        for segs, alias in flat:
            glob = bool(segs) and segs[-1] == "*"
            path = [s for s in segs if s != "*"]
            selfleaf = bool(path) and path[-1] == "self"
            if selfleaf:
                path = path[:-1]
            d = decide(path, m.start("body"), path)
            if d is not None:
                changed = True
            rows.append((path, glob, alias, d, selfleaf))
        if not changed:
            covered.append((m.start(), m.end()))
            continue
        lines = []
        is_pub = bool(m.group("vis")) and not moved_file
        for path, glob, alias, d, selfleaf in rows:
            if d is not None and d[0] == model.new_ident and is_pub and not glob:
                report["facades"].append((os.path.relpath(f, REPO), "::".join(path)))
                continue
            shown = (d[0] + "::" + "::".join(d[1])) if d is not None else "::".join(path)
            if d is not None and not d[1]:
                shown = d[0]
            tail = "::*" if glob else (f" as {alias}" if alias else "")
            if selfleaf and not alias and d is None:
                shown = "::".join(path)
            attrs = m.group("attrs")
            lines.append(f"{attrs}{m.group('indent')}{m.group('vis') or ''}use {shown}{tail};")
        start = m.start()
        edits.append((start, m.end(), "\n".join(lines)))
        covered.append((m.start(), m.end()))
    path_re = re.compile(r"(?<![\w:$.])(?P<path>(?:" + heads_alt + r")(?:" + SEP + r"(?:" + IDENT + r"|super))+)")
    for m in path_re.finditer(text):
        if any(a <= m.start() < b for a, b in covered):
            continue
        segs = segs_of(m.group("path"))
        d = decide(segs, m.start(), segs)
        if d is None:
            continue
        edits.append((m.start(), m.end(), d[0] + ("::" + "::".join(d[1]) if d[1] else "")))
    if f == model.crate.root:
        heads = sorted({mp[0] for mp in model.crate.mods if mp})
        root_re = re.compile(r"(?<![\w:$])(?P<path>(?:" + "|".join(map(re.escape, heads)) + r")(?:" + SEP + IDENT + r")+)")
        for m in root_re.finditer(text):
            if any(a <= m.start() < b for a, b in covered) or any(a <= m.start() < b for a, b, _ in spans):
                continue
            if text[max(0, m.start() - 4):m.start()].strip().endswith("mod"):
                continue
            segs = segs_of(m.group("path"))
            target = model.canonical(segs)
            if model.is_moved(target):
                newp = model.new_path(target)
                edits.append((m.start(), m.end(), model.new_ident + ("::" + "::".join(newp) if newp else "")))
    return sorted(set(edits))


def apply_edits(raw, edits):
    out, last = [], 0
    for a, b, rep in sorted(edits):
        if a < last:
            raise SystemExit(f"overlapping edits at {a}")
        out.append(raw[last:a])
        out.append(rep)
        last = b
    out.append(raw[last:])
    return "".join(out)


TOK = re.compile(r"#\[path\s*=\s*\"(?P<path>[^\"]+)\"\]|#\[cfg\((?P<cfg>[^\]]*)\)\]|#\[(?P<attr>[^\]]*)\]|(?P<vis>pub(?:\s*\([^)]*\))?\s+)?mod\s+(?P<mod>" + IDENT + r")\s*(?P<kind>[;{])|(?P<glob>pub\s+use\s+component\s*::\s*\*\s*;)|(?P<close>\})")


def parse_mounts(text, start, end, directory, file_dir):
    """🌳️ Nodes of one module body: ("mount", name, vis, attrs, file, span) | ("node", name, vis, attrs, children, span, dir) | ("glob", attrs, span)."""
    children, attrs, attr_start, pending = [], [], None, None
    pos = start
    while pos < end:
        t = TOK.search(text, pos, end)
        if not t:
            break
        if attr_start is None and (t.group("path") or t.group("cfg") is not None or t.group("attr") is not None):
            between = text[pos:t.start()]
            attr_start = t.start()
        pos = t.end()
        if t.group("path") is not None:
            pending = t.group("path")
            attrs.append(("path", t.group("path")))
        elif t.group("cfg") is not None:
            attrs.append(("cfg", t.group("cfg")))
        elif t.group("attr") is not None:
            attrs.append(("attr", t.group("attr")))
        elif t.group("mod"):
            name, vis = t.group("mod"), (t.group("vis") or "").strip()
            s0 = attr_start if attr_start is not None else t.start()
            if t.group("kind") == "{":
                depth, j = 0, t.end() - 1
                while True:
                    if text[j] == "{":
                        depth += 1
                    elif text[j] == "}":
                        depth -= 1
                        if depth == 0:
                            break
                    j += 1
                sub = os.path.normpath(os.path.join(directory, pending)) if pending else os.path.join(directory, name)
                children.append(("node", name, vis, [a for a in attrs if a[0] != "path"], parse_mounts(text, t.end(), j, sub, file_dir), (s0, j + 1), sub))
                pos = j + 1
            else:
                base = directory if pending else directory
                target = os.path.normpath(os.path.join(base, pending)) if pending else os.path.join(directory, name + ".rs")
                children.append(("mount", name, vis, [a for a in attrs if a[0] != "path"], target, (s0, t.end())))
            attrs, attr_start, pending = [], None, None
        elif t.group("glob"):
            children.append(("glob", attrs, (attr_start if attr_start is not None else t.start(), t.end())))
            attrs, attr_start, pending = [], None, None
        elif t.group("close"):
            attrs, attr_start, pending = [], None, None
    return children


def find_node(children, segs):
    node = None
    for seg in segs:
        node = next((c for c in children if c[0] == "node" and c[1] == seg), None)
        if node is None:
            return None
        children = node[4]
    return node


def render(children, root_dir, indent, gate, keep=lambda c: True):
    pad = "    " * indent
    lines, previous = [], None
    for child in children:
        if child[0] == "glob":
            if previous is not None and previous[0] == "mount" and previous[1] == "component" and keep(previous):
                lines.append(f"{pad}pub use component::*;")
            continue
        if not keep(child):
            previous = child
            continue
        attrs = [a for a in child[3] if not (a[0] == "cfg" and gate and a[1].strip() == f'feature = "{gate}"')]
        for kind, value in attrs:
            lines.append(f"{pad}#[cfg({value})]" if kind == "cfg" else f"{pad}#[{value}]")
        if child[0] == "mount":
            lines.append(f'{pad}#[path = "{os.path.relpath(child[4], root_dir)}"]')
            lines.append(f"{pad}{(child[2] + ' ') if child[2] else ''}mod {child[1]};")
        else:
            inner = render(child[4], root_dir, indent + 1, gate, keep)
            if not inner:
                lines = lines[:len(lines) - len(attrs)]
                previous = child
                continue
            lines += [f'{pad}#[path = "."]', f"{pad}{child[2] or 'pub'} mod {child[1]} {{"] + inner + [f"{pad}}}"]
        previous = child
    return lines


def ungate(text, gate):
    """🚪️ Evaluates `feature = "<gate>"` as enabled in one moved file's cfg attributes."""
    if not gate:
        return text
    g = re.escape(f'feature = "{gate}"')
    text = re.sub(r"(?m)^[ \t]*#\[cfg\(\s*" + g + r"\s*\)\]\s*\n", "", text)
    text = re.sub(r"#\[cfg\(\s*" + g + r"\s*\)\]\s*", "", text)
    text = re.sub(r"#\[cfg\(\s*all\s*\(\s*test\s*,\s*" + g + r"\s*\)\s*\)\]", "#[cfg(test)]", text)
    text = re.sub(r"#\[cfg\(\s*all\s*\(\s*" + g + r"\s*,\s*test\s*\)\s*\)\]", "#[cfg(test)]", text)
    text = re.sub(r"#\[cfg\(\s*all\s*\(\s*" + g + r"\s*,\s*(not\s*\(\s*target_arch\s*=\s*\"wasm32\"\s*\))\s*\)\s*\)\]", r"#[cfg(\1)]", text)
    if re.search(g, text):
        print(f"WARN gate {gate} remains in a moved file")
    return text


def toml_section(text, table):
    return re.search(r"(?ms)^\[" + re.escape(table) + r"\]\s*$(.*?)(?=^\[|\Z)", text)


def dep_table(text, table):
    """📦️ {extern ident: (key, spec text)} of one dependency table (inline and dotted forms)."""
    out = {}
    sec = toml_section(text, table)
    if sec:
        for m in re.finditer(r"(?m)^([A-Za-z0-9_\-\"]+)\s*=\s*(.+)$", sec.group(1)):
            key = m.group(1).strip('"')
            spec = m.group(2).strip()
            pkg = re.search(r'package\s*=\s*"([^"]+)"', spec)
            out[key.replace("-", "_")] = (key, spec, pkg.group(1) if pkg else key)
    for m in re.finditer(r"(?ms)^\[" + re.escape(table) + r"\.([A-Za-z0-9_\-]+)\]\s*$(.*?)(?=^\[|\Z)", text):
        key, body = m.group(1), m.group(2)
        pkg = re.search(r'package\s*=\s*"([^"]+)"', body)
        spec = "{ " + ", ".join(l.strip().replace(" = ", " = ") for l in body.strip().splitlines() if l.strip() and not l.strip().startswith("#")) + " }"
        out[key.replace("-", "_")] = (key, spec, pkg.group(1) if pkg else key)
    return out


def retarget_spec(spec, old_dir, new_dir):
    def fix(m):
        absolute = os.path.normpath(os.path.join(old_dir, m.group(1)))
        return f'path = "{os.path.relpath(absolute, new_dir)}"'
    spec = re.sub(r'path\s*=\s*"([^"]+)"', fix, spec)
    spec = re.sub(r",?\s*optional\s*=\s*true", "", spec)
    spec = re.sub(r"\{\s*,", "{", spec)
    return spec


def usage(model, files, gate):
    """🔎️ Extern idents named by lib code and by test-only code of the moved files."""
    lib, test = set(), set()
    for f in files:
        text = ungate(strip(model.raw[f]), gate)
        tspans = test_spans(text)
        is_test = f in model.crate.test_files or "🧪️tests" in os.path.relpath(f, model.artifact)
        for m in re.finditer(r"(?<![\w:$.])(" + IDENT + r")\s*(?:::|!)", text):
            name = m.group(1)
            (test if is_test or any(a <= m.start() < b for a, b in tspans) else lib).add(name)
        for m in re.finditer(r"\buse\s+(?:::)?(" + IDENT + r")\b", text):
            (test if is_test or any(a <= m.start() < b for a, b in tspans) else lib).add(m.group(1))
        for m in re.finditer(r"#\[(?:derive\(([^\]]*)\)|(" + IDENT + r")::)", text):
            for name in re.findall(r"(" + IDENT + r")::", m.group(0)):
                (test if is_test else lib).add(name)
    return lib, test - lib


def main():
    spec_path, mode = sys.argv[1], sys.argv[2]
    force = "--force" in sys.argv
    spec = json.load(open(spec_path, encoding="utf-8"))
    model = Model(spec)
    gate = spec.get("gate")
    crate = model.crate
    owner_dir = os.path.join(model.artifact, spec["owner"])
    first = next((m for m in model.moves if m[1] == ()), None)
    if first and model.owner_component(first[0]):
        root_file = crate.mods[first[0] + ("component",)]
    elif first and first[0] in crate.mods and crate.mods[first[0]] != crate.mods.get(first[0][:-1]) and crate.mods[first[0]] != crate.root:
        root_file = crate.mods[first[0]]
    else:
        root_file = os.path.join(owner_dir, "🦀️.rs")
    if os.path.dirname(root_file) != os.path.normpath(owner_dir):
        raise SystemExit(f"owner dir {owner_dir} is not the root file's dir {root_file}")
    report = {"upward": [], "facades": [], "test_upward": []}
    writes = {}
    for f in sorted(crate.files):
        raw, text = model.raw[f], strip(model.raw[f])
        edits = rewrite_file(model, f, text, raw, f in model.moved_files, report)
        if edits:
            try:
                writes[f] = apply_edits(raw, edits)
            except SystemExit:
                print(f"OVERLAP in {os.path.relpath(f, REPO)}: {[(a, b, raw[a:b][:60], r[:60]) for a, b, r in edits][:12]}")
                raise
    # mount surgery in the monolith root and in remaining declaring files
    root_raw = writes.get(crate.root, model.raw[crate.root])
    root_text = strip(root_raw)
    tree = parse_mounts(root_text, 0, len(root_text), os.path.dirname(crate.root), os.path.dirname(crate.root))
    removals = []

    def moved_mount(child, prefix):
        mp = prefix + (child[1],)
        return model.is_moved(mp)

    def collect(children, prefix):
        for child in children:
            if child[0] == "glob":
                continue
            mp = prefix + (child[1],)
            if child[0] == "mount" and model.is_moved(mp):
                removals.append(child[5])
            elif child[0] == "node":
                if model.is_moved(mp) and not any(e[:len(mp)] == mp for e in model.excludes):
                    removals.append(child[5])
                else:
                    collect(child[4], mp)
    collect(tree, ())
    for child in tree:
        pass
    # globs following a removed component mount
    for child_list_prefix in []:
        pass
    edits = []
    for a, b in removals:
        s = root_raw.rfind("\n", 0, a) + 1
        e = b + (1 if root_raw[b:b + 1] == "\n" else 0)
        edits.append((s, e, ""))
    globs = []

    def drop_globs(children, prefix):
        for i, child in enumerate(children):
            if child[0] == "glob" and i > 0 and children[i - 1][0] == "mount" and children[i - 1][1] == "component" and model.is_moved(prefix + ("component",)):
                a, b = child[2]
                s = root_raw.rfind("\n", 0, a) + 1
                e = b + (1 if root_raw[b:b + 1] == "\n" else 0)
                edits.append((s, e, ""))
            elif child[0] == "node":
                drop_globs(child[4], prefix + (child[1],))
    drop_globs(tree, ())
    edits = sorted(set(edits))
    merged = []
    for e in edits:
        if merged and e[0] < merged[-1][1]:
            merged[-1] = (merged[-1][0], max(merged[-1][1], e[1]), "")
        else:
            merged.append(e)
    root_new = apply_edits(root_raw, merged)
    root_new = re.sub(r"\n(\s*)#\[path = \"\.\"\]\n\s*pub mod (" + IDENT + r") \{\s*\}\n", "\n", root_new)
    for _ in range(6):
        root_new = re.sub(r"(?m)^([ \t]*)#\[path = \"\.\"\]\n[ \t]*(?:pub )?mod " + IDENT + r" \{\s*\}\n", "", root_new)
    writes[crate.root] = root_new
    # nested declarations of moved files inside remaining (non-root) files
    nested = []
    decl = re.compile(r'(?m)^[ \t]*(?:#\[cfg\([^\]]*\)\]\s*)?#\[path\s*=\s*"(?P<path>[^"]+)"\]\s*(?:#\[cfg\((?P<cfg>[^\]]*)\)\]\s*)?(?P<vis>pub(?:\s*\([^)]*\))?\s+)?mod\s+(?P<name>' + IDENT + r')\s*;[ \t]*\n?')
    for f in crate.files:
        if f in model.moved_files or f == crate.root:
            continue
        body = writes.get(f, model.raw[f])
        text = strip(body)
        cut_edits = []
        for m in decl.finditer(text):
            target = os.path.normpath(os.path.join(os.path.dirname(f), m.group("path")))
            if target in model.moved_files and target not in model.shared_files:
                cut_edits.append((m.start(), m.end(), ""))
                nested.append((crate.files[target], target, (m.group("vis") or "").strip(), m.group("cfg")))
        if cut_edits:
            writes[f] = apply_edits(body, cut_edits)
    # remaining children declared inside moved files stay in the monolith at their module path
    orphans = []
    for f in sorted(model.moved_files):
        if f in model.shared_files:
            continue
        body = writes.get(f, model.raw[f])
        text = strip(body)
        cut_edits = []
        for m in decl.finditer(text):
            target = os.path.normpath(os.path.join(os.path.dirname(f), m.group("path")))
            if target in crate.files and target not in model.moved_files:
                cut_edits.append((m.start(), m.end(), ""))
                orphans.append((tuple(x for x in crate.files[target] if x != "component"), target, (m.group("vis") or "").strip(), m.group("cfg")))
        if cut_edits:
            writes[f] = apply_edits(body, cut_edits)
    if orphans:
        root_dir_mono = os.path.dirname(crate.root)
        lines = []
        for mp, target, vis, cfg in orphans:
            chain = mp[:-1]
            block = [("    " * i) + '#[path = "."]\n' + ("    " * i) + f"pub mod {seg} {{" for i, seg in enumerate(chain)]
            pad = "    " * len(chain)
            if cfg:
                block.append(f"{pad}#[cfg({cfg})]")
            block.append(f'{pad}#[path = "{os.path.relpath(target, root_dir_mono)}"]')
            block.append(f"{pad}{vis + ' ' if vis else ''}mod {mp[-1]};")
            block += [("    " * i) + "}" for i in reversed(range(len(chain)))]
            lines += block
            print(f"ORPHAN kept in monolith at {'::'.join(mp)}")
        writes[crate.root] = writes[crate.root].rstrip("\n") + "\n\n" + "\n".join(lines) + "\n"
    # new crate root
    root_dir = owner_dir
    mounts = []
    def prune(children, prefix):
        out = []
        for c in children:
            if c[0] == "node":
                mp = prefix + (c[1],)
                if any(e == mp for e in model.excludes):
                    continue
                c = c[:4] + (prune(c[4], mp),) + c[5:]
            elif c[0] == "mount" and any(e == prefix + (c[1],) for e in model.excludes):
                continue
            out.append(c)
        return out

    for frm, to in model.moves:
        node = find_node(tree, frm)
        if node is None:
            parent = find_node(tree, frm[:-1]) if len(frm) > 1 else ("node", "", "", [], tree)
            mount = next((c for c in (parent[4] if parent else []) if c[0] == "mount" and c[1] == frm[-1]), None)
            if mount is not None and to:
                chain = to[:-1]
                lines = [("    " * i) + f'#[path = "."]\n' + ("    " * i) + f"pub mod {seg} {{" for i, seg in enumerate(chain)]
                pad = "    " * len(chain)
                for kind, value in mount[3]:
                    lines.append(f"{pad}#[cfg({value})]" if kind == "cfg" else f"{pad}#[{value}]")
                lines.append(f'{pad}#[path = "{os.path.relpath(mount[4], root_dir)}"]')
                lines.append(f"{pad}{mount[2] + ' ' if mount[2] else ''}mod {to[-1]};")
                lines += [("    " * i) + "}" for i in reversed(range(len(chain)))]
                mounts += lines
            continue
        node = node[:4] + (prune(node[4], frm),) + node[5:]
        keep = lambda c, frm=frm: True
        body = [c for c in node[4] if not (c[0] == "mount" and c[1] == "component" and to == () and frm == first[0])]
        body = [c for i, c in enumerate(body)]
        if to == () and frm == first[0]:
            inner = render([c for c in node[4] if not (c[0] == "mount" and c[1] == "component") and not (c[0] == "glob")], root_dir, 0, gate)
            mounts += inner
        else:
            inner = render(node[4], root_dir, len(to), gate)
            chain_open = [("    " * i) + f'#[path = "."]\n' + ("    " * i) + f"pub mod {seg} {{" for i, seg in enumerate(to)]
            mounts += chain_open + inner + [("    " * i) + "}" for i in reversed(range(len(to)))]
    for mp, target, vis, cfg in nested:
        if target == root_file:
            continue
        newp = model.new_path(mp)
        chain = newp[:-1]
        lines = [("    " * i) + f'#[path = "."]\n' + ("    " * i) + f"pub mod {seg} {{" for i, seg in enumerate(chain)]
        pad = "    " * len(chain)
        if cfg and not (gate and cfg.strip() == f'feature = "{gate}"'):
            lines.append(f"{pad}#[cfg({cfg})]")
        lines.append(f'{pad}#[path = "{os.path.relpath(target, root_dir)}"]')
        lines.append(f"{pad}{vis + ' ' if vis else ''}mod {newp[-1]};")
        lines += [("    " * i) + "}" for i in reversed(range(len(chain)))]
        mounts += lines
    moved_sources = sorted(model.moved_files)
    lib_names, test_names = usage(model, moved_sources, gate)
    aliases = sorted(a for a in ALIASES if a in lib_names or a in test_names)
    header = [f"extern crate {ALIASES[a]} as {a};" for a in aliases] + ([f"extern crate self as {model.new_ident};"] if model.self_alias else [])
    for f in sorted(model.shared_files):
        print(f"SHARED {os.path.relpath(f, REPO)}")
    if root_file in model.raw:
        base = writes.get(root_file, model.raw[root_file])
        m = re.match(r"((?:[ \t]*//![^\n]*\n)*)", base)
        doc = m.group(1)
        rest = base[len(doc):]
        inner = "#![allow(clippy::result_large_err)]\n"
        root_text_new = doc + inner + ("\n".join(header) + "\n" if header else "") + rest.rstrip("\n") + ("\n\n" + "\n".join(mounts) + "\n" if mounts else "\n")
    else:
        doc = f"//! {spec.get('doc', '🧩️ ' + spec['package'])}\n"
        root_text_new = doc + ("\n".join(header) + "\n" if header else "") + "\n" + "\n".join(mounts) + "\n"
    writes[root_file] = root_text_new
    for f in moved_sources:
        if f in writes:
            writes[f] = ungate(writes[f], gate)
        elif gate and re.search(re.escape(f'feature = "{gate}"'), model.raw[f]):
            writes[f] = ungate(model.raw[f], gate)
    # external consumers
    consumers = {}
    group = re.compile(r"(?m)^(?P<attrs>(?:[ \t]*#\[[^\n]*\]\n)*)(?P<indent>[ \t]*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+" + re.escape(model.ident) + r"(?P<body>\s*::\s*[^;]*\{[^;]*);")
    single = re.compile(r"(?<![\w:])" + re.escape(model.ident) + r"(?P<rest>(?:" + SEP + IDENT + r")+)")
    for root in spec.get("consumers", []):
        for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, root)):
            dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated", ".git") and not d.startswith(".")]
            for name in filenames:
                if not name.endswith(".rs"):
                    continue
                path = os.path.normpath(os.path.join(dirpath, name))
                if path in crate.files:
                    continue
                raw = open(path, encoding="utf-8").read()
                if model.ident not in raw:
                    continue
                text = strip(raw)
                edits, covered = [], []
                for m in group.finditer(text):
                    body = m.group("body").strip()[2:].strip()
                    rows, changed = [], False
                    for segs, alias in leaves([], body):
                        glob = bool(segs) and segs[-1] == "*"
                        p = [s for s in segs if s != "*"]
                        target = model.canonical(p)
                        if model.is_moved(target):
                            changed = True
                            rows.append((model.new_ident, list(model.new_path(target)), glob, alias))
                        else:
                            rows.append((model.ident, p, glob, alias))
                    if changed:
                        lines = [f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {head}{'::' + '::'.join(p) if p else ''}{'::*' if glob else (f' as {alias}' if alias else '')};" for head, p, glob, alias in rows]
                        edits.append((m.start(), m.end(), "\n".join(lines)))
                    covered.append((m.start(), m.end()))
                for m in single.finditer(text):
                    if any(a <= m.start() < b for a, b in covered):
                        continue
                    target = model.canonical(segs_of(m.group("rest")))
                    if model.is_moved(target):
                        newp = model.new_path(target)
                        edits.append((m.start(), m.end(), model.new_ident + ("::" + "::".join(newp) if newp else "")))
                if edits:
                    consumers[path] = apply_edits(raw, edits)
    for f, line, target in sorted(set(report["upward"])):
        print(f"UPWARD {f}:{line} {target}")
    for f, path in sorted(set(report["facades"])):
        print(f"FACADE-DROPPED {f} {path}")
    # manifests
    mono_manifest = os.path.join(model.artifact, "📦️packages/🦀️rust/Cargo.toml")
    mono_text = open(mono_manifest, encoding="utf-8").read()
    deps = dep_table(mono_text, "dependencies")
    dev = dep_table(mono_text, "dev-dependencies")
    target_dev = dep_table(mono_text, "target.'cfg(not(target_arch = \"wasm32\"))'.dev-dependencies")
    pkg_dir = os.path.join(owner_dir, "📦️packages/🦀️rust")
    mono_dir = os.path.dirname(mono_manifest)
    named_lib = {ALIASES.get(n, n) for n in lib_names}
    named_test = {ALIASES.get(n, n) for n in test_names} - named_lib
    extra = spec.get("extra_deps", {})
    lib_rows, dev_rows, tdev_rows = [], [], []
    for ident, (key, dspec, pkg) in sorted(deps.items()):
        if ident in named_lib or ident in extra:
            lib_rows.append(f"{key} = {retarget_spec(dspec, mono_dir, pkg_dir)}")
        elif ident in named_test and ident not in dev and ident not in target_dev:
            dev_rows.append(f"{key} = {retarget_spec(dspec, mono_dir, pkg_dir)}")
    for ident, (key, dspec, pkg) in sorted(dev.items()):
        if ident in named_test or (ident in named_lib and "features" in dspec and ident in deps):
            dev_rows.append(f"{key} = {retarget_spec(dspec, mono_dir, pkg_dir)}")
    for ident, (key, dspec, pkg) in sorted(target_dev.items()):
        if ident in named_test or ident in named_lib:
            tdev_rows.append(f"{key} = {retarget_spec(dspec, mono_dir, pkg_dir)}")
    for name, row in extra.items():
        if not any(r.startswith(name + " =") for r in lib_rows):
            lib_rows.append(f"{name} = {row}")
    ws_root = model.artifact
    ws_manifest = os.path.join(ws_root, "Cargo.toml")
    rel_ws = os.path.relpath(ws_root, pkg_dir)
    manifest = [
        "[package]", f'workspace = "{rel_ws}"', f'name = "{spec["package"]}"', "version.workspace = true", "edition.workspace = true",
        "rust-version.workspace = true", f'description = "{spec.get("description", spec["package"])}"', "",
        "[package.metadata.semio]", 'role = "library"', "", "[lints]", "workspace = true", "", "[lib]",
        f'path = "{os.path.relpath(root_file, pkg_dir)}"', "", "[dependencies]"] + sorted(set(lib_rows))
    if dev_rows:
        manifest += ["", "[dev-dependencies]"] + sorted(set(dev_rows))
    if tdev_rows:
        manifest += ["", "[target.'cfg(not(target_arch = \"wasm32\"))'.dev-dependencies]"] + sorted(set(tdev_rows))
    if report["test_upward"]:
        mono_pkg = re.search(r'(?m)^name\s*=\s*"([^"]+)"', mono_text).group(1)
        row = f'{mono_pkg} = {{ path = "{os.path.relpath(mono_dir, pkg_dir)}" }}'
        if "[dev-dependencies]" in manifest:
            i = manifest.index("[dev-dependencies]")
            manifest.insert(i + 1, row)
        else:
            manifest += ["", "[dev-dependencies]", row]
        for f, line, target in sorted(set(report["test_upward"])):
            print(f"TEST-UPWARD {f}:{line} {target}")
    manifest_text = "\n".join(manifest) + "\n"
    print(f"== root {os.path.relpath(root_file, REPO)}  mounts={len(mounts)} header={aliases}")
    print(f"== moved files {len(moved_sources)}; rewritten monolith files {len([f for f in writes if f in crate.files])}; consumers {len(consumers)}")
    for path in sorted(consumers):
        print(f"consumer {os.path.relpath(path, REPO)}")
    print("== manifest\n" + manifest_text)
    if report["upward"] and not force:
        print("REFUSED: upward references")
        if mode == "apply":
            raise SystemExit(1)
    if mode != "apply":
        return
    for path, text in writes.items():
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(text)
    for path, text in consumers.items():
        open(path, "w", encoding="utf-8").write(text)
    os.makedirs(pkg_dir, exist_ok=True)
    open(os.path.join(pkg_dir, "Cargo.toml"), "w", encoding="utf-8").write(manifest_text)
    ws = open(ws_manifest, encoding="utf-8").read()
    member = os.path.relpath(pkg_dir, ws_root)
    if f'"{member}"' not in ws:
        ws = re.sub(r"(?s)(\[workspace\]\s*\nresolver = \"2\"\s*\nmembers = \[\n)", lambda m: m.group(1) + f'    "{member}",\n', ws, count=1)
    row = f'[workspace.dependencies.{spec["package"]}]\npath = "{member}"\n'
    if f"[workspace.dependencies.{spec['package']}]" not in ws:
        ws = re.sub(r"(?m)^\[workspace\.lints\.rust\]", row + "\n[workspace.lints.rust]", ws, count=1)
    open(ws_manifest, "w", encoding="utf-8").write(ws)
    mono = open(mono_manifest, encoding="utf-8").read()
    if f"\n{spec['package']} =" not in mono:
        opt = ", optional = true" if spec.get("monolith_optional") else ""
        mono = re.sub(r"(?m)^\[dependencies\]\n", f"[dependencies]\n{spec['package']} = {{ workspace = true{opt} }}\n", mono, count=1)
        if spec.get("monolith_optional") and gate:
            mono = re.sub(r'(?m)^(' + re.escape(gate) + r'\s*=\s*\[)', r'\1"dep:' + spec["package"] + '", ', mono, count=1)
    open(mono_manifest, "w", encoding="utf-8").write(mono)
    print("applied")


if __name__ == "__main__":
    main()
