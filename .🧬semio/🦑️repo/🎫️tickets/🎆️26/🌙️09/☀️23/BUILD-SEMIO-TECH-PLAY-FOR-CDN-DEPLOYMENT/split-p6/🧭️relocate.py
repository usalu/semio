#!/usr/bin/env python3
"""🧭️ Relocates one `#[path]`-tree artifact crate into a family of crates whose roots are owner `🦀️.rs` files.

Every module of the monolith is assigned to a crate and a new module path by longest-prefix rules on its normalized
module path (`component` segments dropped). Each path the sources write (`use` trees, `crate::`/`super::`/`self::`
paths, root-relative and module-relative paths) is resolved against the ORIGINAL module tree (definitions, `use`
bindings and globs followed to the defining module) and re-rendered for its new home: `crate::<new path>` inside the
same crate, `<crate ident>::<new path>` across crates. Relative paths whose meaning survives are kept verbatim. Mounts
are regenerated: a mount stays in its file when parent and child share a crate and position, otherwise it is removed
and re-emitted in the owning crate's root mount region. Cross-crate re-exports (facades) are dropped; consumers name
the owner. `pub(crate)`/`pub(super)` widen to `pub` in every relocated file. Top-level items can be moved between files
first (`moves`), so resolution already sees them at their destination.

usage: 🧭️relocate.py <config.json> plan|apply [--verbose]
config: {"root": monolith root (repo-relative), "crate": monolith crate ident, "consumers": [repo-relative roots],
  "crates": {name: {"ident": rust ident, "root": owner 🦀️.rs (repo-relative), "header": [root lines], "doc": "//! .."}},
  "rules": [[normalized module prefix ("" = root), crate, new prefix]...],
  "files": [[repo-relative file regex, crate, new module path]...],   (file overrides, first match)
  "moves": [{"from": file, "to": file, "items": [name regex...], "doc": "//! .. (for created files)"}],
  "aggregator": crate name of the monolith root}
"""
import collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
SEP = r"\s*::\s*"


def strip(src):
    """🧹️ Blanks comments and string/char literal bodies, keeping every byte offset (path attribute values kept)."""
    out, i, n = [], 0, len(src)
    blank = lambda text: re.sub(r"[^\n]", " ", text)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            j = n if j < 0 else j
            out.append(" " * (j - i))
            i = j
        elif src.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif src.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            out.append(blank(src[i:j]))
            i = j
        elif c in "rb" and re.match(r'(?:r|br|b?r#+)"|r#*"', src[i:i + 40]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")) and re.match(r'b?r(#*)"', src[i:]):
            m = re.match(r'b?r(#*)"', src[i:])
            close = '"' + m.group(1)
            j = src.find(close, i + len(m.group(0)))
            j = n if j < 0 else j + len(close)
            head = len(m.group(0))
            out.append(src[i:i + head] + blank(src[i + head:j - len(close)]) + close)
            i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            j += 1
            keep = re.search(r"#\[path\s*=\s*$", "".join(out[-12:]))
            out.append(src[i:j] if keep else '"' + blank(src[i + 1:j - 1]) + '"')
            i = j
        elif c == "'" and (re.match(r"'(\\.|[^\\'])'", src[i:i + 4]) or re.match(r"'\\(x..|u\{[0-9a-fA-F]+\})'", src[i:i + 12])):
            m = re.match(r"'(\\x..|\\u\{[0-9a-fA-F]+\}|\\.|[^\\'])'", src[i:])
            out.append(" " * len(m.group(0)))
            i += len(m.group(0))
        else:
            out.append(c)
            i += 1
    result = "".join(out)
    assert len(result) == len(src)
    return result


def close_of(text, open_index):
    depth, j = 0, open_index
    while j < len(text):
        if text[j] == "{":
            depth += 1
        elif text[j] == "}":
            depth -= 1
            if depth == 0:
                return j
        j += 1
    raise SystemExit("unbalanced braces")


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
    """🍃️ Flattens a use tree into (segments, alias) leaves; `*` stays a final segment, `self` folds into its prefix."""
    body = body.strip()
    m = re.match(r"^((?:" + IDENT + SEP + r")*)\{(.*)\}$", body, re.S)
    if m:
        pre = prefix + [x for x in re.split(SEP, m.group(1)) if x]
        out = []
        for part in split_top(m.group(2)):
            out += leaves(pre, part)
        return out
    alias = re.search(r"\s+as\s+(" + IDENT + r")$", body)
    core = re.sub(r"\s+as\s+" + IDENT + r"$", "", body)
    segs = prefix + [x for x in re.split(SEP, core) if x]
    if segs and segs[-1] == "self":
        segs = segs[:-1]
        if alias is None and segs:
            return [(segs, segs[-1], True)]
    return [(segs, alias.group(1) if alias else None, False)]


ITEM_HEAD = re.compile(r"(?P<vis>pub(?:\s*\([^)]*\))?\s+)?(?:(?:const|async|unsafe|extern\s+\"[^\"]*\"|default)\s+)*(?P<kind>fn|struct|enum|trait|type|const|static|union|macro_rules!)\s*(?P<name>" + IDENT + r")")


class Module:
    def __init__(self, path, file, start, end, parent, component, cfg, decl):
        self.path, self.file, self.start, self.end, self.parent = path, file, start, end, parent
        self.component, self.cfg, self.decl = component, cfg, decl
        self.defs, self.uses, self.children = {}, [], {}


class Tree:
    """🌳️ The monolith's module tree: modules by actual path, files, definitions and use bindings per module."""

    def __init__(self, root):
        self.root = root
        self.mods, self.files, self.text, self.raw = {}, {}, {}, {}
        self.load_file(root, (), None, None, None)

    def read(self, file):
        if file not in self.raw:
            self.raw[file] = open(file, encoding="utf-8").read()
            self.text[file] = strip(self.raw[file])
        return self.text[file]

    def load_file(self, file, path, parent, cfg, decl):
        file = os.path.normpath(file)
        text = self.read(file)
        module = Module(path, file, 0, len(text), parent, path[-1:] == ("component",), cfg, decl)
        self.mods[path] = module
        self.files.setdefault(file, path)
        if parent is not None:
            self.mods[parent].children[path[-1]] = path
        self.scan(module, file, 0, len(text), os.path.dirname(file), True)

    def scan(self, module, file, start, end, directory, file_scope):
        text = self.text[file]
        pos, pending_path, pending_cfg, attr_start = start, None, None, None
        token = re.compile(r"#\s*\[\s*path\s*=\s*\"(?P<path>[^\"]+)\"\s*\]|#\s*\[\s*cfg\s*\((?P<cfg>(?:[^()]|\([^()]*(?:\([^()]*\)[^()]*)*\))*)\)\s*\]|#!?\s*\[|(?P<vis>\bpub(?:\s*\([^)]*\))?\s+)?\bmod\s+(?P<mod>" + IDENT + r")\s*(?P<kind>[;{])|[{};]")
        while pos < end:
            m = token.search(text, pos, end)
            if not m:
                break
            t = m.group(0)
            if m.group("path") is not None:
                pending_path = m.group("path")
                attr_start = m.start() if attr_start is None else attr_start
                pos = m.end()
                continue
            if m.group("cfg") is not None:
                pending_cfg = m.group("cfg")
                attr_start = m.start() if attr_start is None else attr_start
                pos = m.end()
                continue
            if t.startswith("#"):
                close = text.find("]", m.end())
                depth, j = 1, m.end()
                while j < end and depth:
                    if text[j] == "[":
                        depth += 1
                    elif text[j] == "]":
                        depth -= 1
                    j += 1
                attr_start = m.start() if attr_start is None else attr_start
                pos = j
                continue
            if m.group("mod"):
                name = m.group("mod")
                decl_start = attr_start if attr_start is not None else m.start()
                if m.group("vis") is None and attr_start is None:
                    decl_start = m.start()
                child = module.path + (name,)
                if m.group("kind") == "{":
                    close = close_of(text, m.end() - 1)
                    sub = os.path.normpath(os.path.join(directory, pending_path)) if pending_path else os.path.join(directory, name)
                    inner = Module(child, file, m.end(), close, module.path, False, pending_cfg, (file, decl_start, close + 1))
                    inner.vis = (m.group("vis") or "").strip()
                    self.mods[child] = inner
                    module.children[name] = child
                    self.scan(inner, file, m.end(), close, sub, False)
                    pos = close + 1
                else:
                    if pending_path:
                        target = os.path.normpath(os.path.join(os.path.dirname(file) if file_scope else directory, pending_path))
                    else:
                        target = os.path.normpath(os.path.join(directory, name + ".rs"))
                    if os.path.exists(target):
                        self.load_file(target, child, module.path, pending_cfg, (file, decl_start, m.end()))
                        self.mods[child].vis = (m.group("vis") or "").strip()
                    pos = m.end()
                pending_path, pending_cfg, attr_start = None, None, None
                continue
            if t == "{":
                pos = close_of(text, m.start()) + 1
            else:
                pos = m.end()
            pending_path, pending_cfg, attr_start = None, None, None

    def index(self):
        """📇️ Definitions and use bindings at each module's top level."""
        self.macro_exports = {}
        for module in self.mods.values():
            text = self.text[module.file]
            for (a, b) in self.top_level(module):
                chunk = text[a:b]
                head = re.sub(r"^(\s*#!?\s*\[[^\]]*\])*\s*", "", chunk)
                m = ITEM_HEAD.match(head)
                if m:
                    module.defs.setdefault(m.group("name"), (m.group("kind"), a))
                    if m.group("kind") == "macro_rules!" and re.search(r"#\s*\[\s*macro_export", chunk[:chunk.find("macro_rules")]):
                        self.macro_exports[m.group("name")] = module.path
                    continue
                u = re.match(r"(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+(?P<body>[^;]*);", head, re.S)
                if u:
                    for segs, alias, self_leaf in leaves([], u.group("body")):
                        if segs and segs[-1] == "*":
                            module.uses.append(("*", segs[:-1], bool(u.group("vis")), a))
                        elif segs:
                            module.uses.append((alias or segs[-1], segs, bool(u.group("vis")), a))
                    continue
                e = re.match(r"extern\s+crate\s+(" + IDENT + r")(?:\s+as\s+(" + IDENT + r"))?\s*;", head)
                if e:
                    module.uses.append((e.group(2) or e.group(1), ["::" + e.group(1)], False, a))
                    continue
                k = re.match(r"(?:" + IDENT + SEP + r")*(" + IDENT + r")\s*!\s*([\(\[\{])", head)
                if k and k.group(1) != "macro_rules":
                    args = head[k.end():]
                    first = re.match(r"\s*(?:pub(?:\s*\([^)]*\))?\s+)?(" + IDENT + r")", args)
                    if first and first.group(1) not in ("pub", "struct", "enum", "fn", "type", "impl", "const", "static", "trait", "mod", "use", "crate", "self", "super"):
                        module.defs.setdefault(first.group(1), ("macro-def", a))
                    for d in re.finditer(r"\b(?:struct|enum|fn|type|trait|const|static|mod)\s+(" + IDENT + r")", args):
                        module.defs.setdefault(d.group(1), ("macro-def", a))
            for name, child in module.children.items():
                module.defs.setdefault(name, ("mod", None))

    def top_level(self, module):
        """🧱️ Spans of the module's top-level items (inline child modules skipped as one item)."""
        text = self.text[module.file]
        pos, end, spans = module.start, module.end, []
        while pos < end:
            m = re.compile(r"\S").search(text, pos, end)
            if not m:
                break
            start = j = m.start()
            depth = 0
            while j < end:
                c = text[j]
                if c in "([":
                    depth += 1
                elif c in ")]":
                    depth -= 1
                elif c == "{":
                    j = close_of(text, j)
                    if depth == 0:
                        k = re.compile(r"[ \t]*").match(text, j + 1).end()
                        if k < end and text[k] == ";":
                            j = k
                        j += 1
                        break
                elif c == ";" and depth == 0:
                    j += 1
                    break
                j += 1
            spans.append((start, j))
            pos = j
        return spans

    def parent_of(self, path):
        return self.mods[path].parent if path in self.mods else path[:-1]

    def resolve_name(self, path, name, seen=None):
        """🔎️ (kind, module path, name) for a name bound in module `path`: ("def"|"mod", where, name) or ("ext", segs)."""
        seen = set() if seen is None else seen
        key = (path, name)
        if key in seen or path not in self.mods:
            return None
        seen.add(key)
        try:
            return self.resolve_bound(path, name, seen)
        finally:
            seen.discard(key)

    def resolve_bound(self, path, name, seen):
        module = self.mods[path]
        if path == () and name in self.macro_exports and name not in module.defs:
            return ("macro", self.macro_exports[name], name, [])
        if name in module.defs:
            kind = module.defs[name][0]
            if kind == "mod":
                return ("mod", module.children[name], None)
            return ("def", path, name)
        for bound, segs, _vis, _at in module.uses:
            if bound == name:
                return self.resolve_path(path, segs, seen)
        for bound, segs, _vis, _at in module.uses:
            if bound == "*":
                target = self.resolve_path(path, segs, seen)
                if target and target[0] == "mod":
                    found = self.resolve_name(target[1], name, seen)
                    if found:
                        return found
        return None

    def resolve_path(self, path, segs, seen=None):
        """🧭️ Resolves segments written inside module `path`: ("mod", p, rest) / ("def", p, name, rest) / ("ext", segs)."""
        seen = set() if seen is None else seen
        if not segs:
            return ("mod", path, [])
        first = segs[0]
        if first.startswith("::"):
            return ("ext", [first[2:]] + segs[1:])
        if first == "crate":
            cur, i = (), 1
        elif first in ("super", "self"):
            cur, i = path, 0
            while i < len(segs) and segs[i] in ("super", "self"):
                if segs[i] == "super":
                    cur = self.real_parent(cur)
                i += 1
        else:
            found = self.resolve_name(path, first, seen)
            if found is None:
                return ("ext", segs)
            if found[0] == "ext":
                return ("ext", found[1] + segs[1:])
            if found[0] in ("def", "macro"):
                return (found[0], found[1], found[2], segs[1:])
            cur, i = found[1], 1
        while i < len(segs):
            seg = segs[i]
            if seg in ("super",):
                cur = self.real_parent(cur)
                i += 1
                continue
            found = self.resolve_name(cur, seg, seen)
            if found is None:
                return None
            if found[0] == "ext":
                return ("ext", found[1] + segs[i + 1:])
            if found[0] in ("def", "macro"):
                return (found[0], found[1], found[2], segs[i + 1:])
            cur = found[1]
            i += 1
        return ("mod", cur, [])

    def real_parent(self, path):
        parent = self.parent_of(path)
        return parent if parent is not None else ()


TRANSPARENT = set()


def normalized(path):
    """🧽️ Drops transparent segments: private child modules their parent glob re-exports (`mod component; pub use component::*;`)."""
    return tuple(seg for k, seg in enumerate(path) if path[:k + 1] not in TRANSPARENT)


def mark_transparent(tree):
    for path, module in tree.mods.items():
        if not path or getattr(module, "vis", "") or module.parent is None:
            continue
        parent = tree.mods[module.parent]
        if any(bound == "*" and segs in ([path[-1]], ["self", path[-1]]) for bound, segs, _vis, _at in parent.uses):
            TRANSPARENT.add(path)


class Plan:
    def __init__(self, config):
        self.config = config
        self.root = os.path.join(REPO, config["root"])
        self.tree = Tree(self.root)
        self.tree.index()
        mark_transparent(self.tree)
        self.rules = sorted(([tuple(p.split("::")) if p else (), c, tuple(n.split("::")) if n else ()] for p, c, n in config["rules"]), key=lambda r: -len(r[0]))
        self.file_rules = [(re.compile(r), c, tuple(n.split("::")) if n else ()) for r, c, n in config.get("files", [])]
        self.idents = {name: spec["ident"] for name, spec in config["crates"].items()}
        self.aggregator = config["aggregator"]
        self.crate_roots = {name: os.path.normpath(os.path.join(REPO, spec["root"])) for name, spec in config["crates"].items()}
        for crate, file in self.crate_roots.items():
            if file not in self.tree.files:
                path = ("@" + crate,)
                module = Module(path, file, 0, 0, (), False, None, None)
                self.tree.mods[path] = module
                self.tree.files[file] = path
                self.tree.raw[file] = self.tree.text[file] = ""
                self.tree.mods[()].children[path[0]] = path
                self.tree.mods[()].defs[path[0]] = ("mod", None)
        self.moves = self.select_moves(config.get("moves", []))
        self.place = {}
        for path, module in self.tree.mods.items():
            self.place[path] = self.locate(path)

    def select_moves(self, moves):
        """🚚️ Item spans to move (with docs, impls and attached macro invocations); re-homes their definitions."""
        out = []
        tree = self.tree
        for move in moves:
            src_file = os.path.normpath(os.path.join(REPO, move["from"]))
            dst_file = os.path.normpath(os.path.join(REPO, move["to"]))
            src, dst = tree.files[src_file], tree.files[dst_file]
            rules = [re.compile(r) for r in move["items"]]
            raw, text = tree.raw[src_file], tree.text[src_file]
            chosen, names = [], set()
            spans = tree.top_level(tree.mods[src])
            kinds = []
            for a, b in spans:
                head = re.sub(r"^(\s*#!?\s*\[[^\]]*\])*\s*", "", text[a:b])
                m = ITEM_HEAD.match(head)
                if m:
                    kinds.append(("def", m.group("name")))
                    continue
                i = re.match(r"(?:unsafe\s+)?impl\b\s*(?:<(?:[^<>]|<[^<>]*>)*>)?\s*([^{]*)", head)
                if i:
                    target = re.split(r"\s+for\s+", i.group(1))[-1]
                    ident = re.findall(IDENT, re.sub(r"<.*", "", target.strip().lstrip("&")))
                    kinds.append(("impl", ident[-1] if ident else "?"))
                    continue
                k = re.match(r"(?:" + IDENT + SEP + r")*" + IDENT + r"\s*!\s*[\(\[\{]\s*(" + IDENT + r")?", head)
                kinds.append(("macro", k.group(1) if k and k.group(1) else "?") if k else ("other", "?"))
            for (a, b), (kind, name) in zip(spans, kinds):
                if kind == "def" and any(r.fullmatch(name) for r in rules):
                    names.add(name)
            for (a, b), (kind, name) in zip(spans, kinds):
                if name in names:
                    start, stop = item_extent(raw, text, a, b)
                    chosen.append((start, stop, kind, name))
            for name in names:
                entry = tree.mods[src].defs.pop(name, None)
                if entry:
                    tree.mods[dst].defs[name] = entry
                    tree.mods[src].uses.append((name, ["crate"] + list(dst) + [name], False, -1))
            out.append({"src": src, "dst": dst, "src_file": src_file, "dst_file": dst_file, "spans": chosen, "names": names})
            missing = [r.pattern for r in rules if not any(r.fullmatch(n) for n in names)]
            if missing:
                print(f"MOVE {move['from']} -> {move['to']}: no item matches {missing}")
        return out

    def rel(self, file):
        return os.path.relpath(file, REPO)

    def locate(self, path):
        """📍️ (crate, new module path) of an actual module path."""
        module = self.tree.mods[path]
        if module.file != self.tree.root and self.tree.files.get(module.file) == path:
            for rule, crate, new in self.file_rules:
                if rule.search(self.rel(module.file)):
                    return crate, new
            parent = module.parent
            while parent:
                pmod = self.tree.mods[parent]
                if pmod.file != self.tree.root and self.tree.files.get(pmod.file) == parent and any(rule.search(self.rel(pmod.file)) for rule, _c, _n in self.file_rules):
                    crate, new = self.place.get(parent) or self.locate(parent)
                    return crate, tuple(new) + normalized(path[len(parent):])
                parent = pmod.parent
        if module.file != self.tree.root and self.tree.files.get(module.file) != path:
            host = self.tree.files[module.file]
            crate, new = self.place.get(host) or self.locate(host)
            return crate, new + normalized(path[len(host):])
        if path and path[0].startswith("@"):
            return path[0][1:], tuple(path[1:])
        norm = normalized(path)
        for prefix, crate, new in self.rules:
            if norm[:len(prefix)] == prefix:
                return crate, new + norm[len(prefix):]
        raise SystemExit(f"unplaced module {'::'.join(path)}")

    def file_crate(self, file):
        return self.place[self.tree.files[file]][0]

    def render_target(self, target, from_crate):
        """🖊️ New absolute text for a resolved target, or None when external/unresolved."""
        if target is None or target[0] == "ext":
            return None
        if target[0] == "mod":
            crate, new = self.place[target[1]]
            segs = list(new) + list(target[2])
        elif target[0] == "macro":
            crate = self.place[target[1]][0]
            segs = [target[2]] + list(target[3])
        else:
            crate, new = self.place[target[1]]
            segs = list(new) + [target[2]] + list(target[3])
        head = "crate" if crate == from_crate else self.idents[crate]
        return crate, head, segs




USE_STMT = re.compile(r"(?m)^(?P<attrs>(?:[ \t]*#\s*\[[^\n]*\][ \t]*\n)*)(?P<indent>[ \t]*(?:#\s*\[[^\]\n]*\][ \t]*)*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+(?P<body>[^;]*);")
REL_PATH = re.compile(r"(?<![\w:$#])(?P<path>(?:crate|super|self)(?:" + SEP + r"(?:" + IDENT + r"|super))+)")
NAME_PATH = re.compile(r"(?<![\w:$#.'])(?P<path>" + IDENT + r"(?:" + SEP + IDENT + r")+)")
DOC_LINE = re.compile(r"^[ \t]*(///|//(?!#)|#\s*\[)")


class Rewriter:
    """✍️ Computes every file's rewritten text, the regenerated mounts and the crate dependency graph."""

    def __init__(self, plan):
        self.plan, self.tree = plan, plan.tree
        self.inline = collections.defaultdict(list)
        for path, module in self.tree.mods.items():
            if self.tree.files.get(module.file) != path:
                self.inline[module.file].append((module.start, module.end, path))
        self.edges = collections.defaultdict(collections.Counter)
        self.detail = collections.defaultdict(set)
        self.test_edges = collections.defaultdict(collections.Counter)
        self.test_detail = collections.defaultdict(set)
        self.dropping = False
        self.blanked = {}
        self.unresolved = collections.Counter()
        self.header_uses = collections.defaultdict(list)
        self.moved = {}
        self.home = {}

    def context(self, file, offset):
        best = self.tree.files[file]
        width = None
        for a, b, path in self.inline[file]:
            if a <= offset < b and (width is None or b - a < width):
                best, width = path, b - a
        return best

    def placed(self, ctx):
        return self.plan.place[ctx]

    def reinterpret(self, segs, home_crate, home_path, ctx):
        """🔁️ What the written segments mean once the file sits at its new module path: (crate, segs) or None."""
        if segs[0] == "crate":
            return home_crate, list(segs[1:])
        if segs[0] in ("super", "self"):
            cur, i = list(home_path), 0
            while i < len(segs) and segs[i] in ("super", "self"):
                if segs[i] == "super":
                    if not cur:
                        return None
                    cur.pop()
                i += 1
            return home_crate, cur + list(segs[i:])
        found = self.tree.mods[ctx].defs.get(segs[0])
        if found and found[0] == "mod":
            return home_crate, list(home_path) + list(segs)
        return None

    def new_target(self, ctx, segs):
        target = self.tree.resolve_path(ctx, segs)
        if target is None:
            return None, None
        if target[0] == "ext":
            return target, None
        return target, self.plan.render_target(target, None)

    def render(self, rendered, home_crate):
        crate, _head, segs = rendered
        head = "crate" if crate == home_crate else self.plan.idents[crate]
        return crate, head, segs

    def leaf_text(self, segs, alias, glob, ctx, home, self_leaf=False):
        """🍃️ Rewritten leaf (segments text, alias, glob) or None to keep; [] to drop."""
        home_crate, home_path = home
        target, rendered = self.new_target(ctx, segs)
        if target is None:
            if segs[0] in ("crate", "super", "self"):
                self.unresolved[("::".join(segs), os.path.relpath(self.current, REPO))] += 1
            return None
        if target[0] == "ext":
            if segs[0] in ("super", "self", "crate"):
                lead = []
                k = 0
                while k < len(segs) and segs[k] in ("super", "self", "crate"):
                    lead.append(segs[k])
                    k += 1
                old_mod = self.tree.resolve_path(ctx, lead + [])
                if old_mod and old_mod[0] == "mod" and self.placed(old_mod[1]) == (home_crate, tuple(self.reinterpret(lead, home_crate, home_path, ctx)[1])):
                    return None
                return ("::".join(target[1]), alias, glob)
            return None
        if glob and target[0] == "mod":
            target = ("mod", self.facade_target(target[1]), [])
            rendered = self.plan.render_target(target, None)
        crate, head, new = self.render(rendered, home_crate)
        if crate != home_crate and not self.dropping:
            testy = self.is_test(ctx)
            (self.test_edges if testy else self.edges)[home_crate][crate] += 1
            (self.test_detail if testy else self.detail)[(home_crate, crate)].add((os.path.relpath(self.current, REPO), "::".join(new) + "  <=  " + "::".join(segs)))
        again = self.reinterpret(segs, home_crate, home_path, ctx)
        if again is not None and again == (crate, list(new)):
            return None
        if glob:
            if (crate, tuple(new)) == (home_crate, tuple(home_path)):
                return []
            return [("::".join([head] + list(new)), None, True)]
        if target[0] == "mod" and not new and crate != home_crate:
            return (head, alias or segs[-1], False)
        if target[0] == "mod" and not new:
            return ("crate", alias or segs[-1], False) if alias or segs[-1] != "crate" else None
        return ("::".join([head] + list(new)), alias if alias else (segs[-1] if self_leaf or (new and new[-1] != segs[-1]) else None), False)

    def facade_target(self, path, depth=0):
        """🪞️ Follows a module that only glob re-exports one other module (a removed shim) to the module it mirrors."""
        module = self.tree.mods.get(path)
        if module is None or depth > 8:
            return path
        real = [name for name, (kind, _a) in module.defs.items()]
        globs = [segs for bound, segs, vis, _a in module.uses if bound == "*" and vis]
        if not real and len(globs) == 1 and len(module.uses) == 1:
            found = self.tree.resolve_path(path, globs[0])
            if found and found[0] == "mod":
                return self.facade_target(found[1], depth + 1)
        return path

    def glob_groups(self, target, home_crate):
        """🌐️ Distinct new defining modules of every name a glob of `target` brings."""
        if target[0] != "mod":
            return []
        names = self.names(target[1], set())
        groups = []
        for name in sorted(names):
            found = self.tree.resolve_name(target[1], name)
            if not found or found[0] == "ext":
                continue
            where = found[1] if found[0] == "def" else self.tree.parent_of(found[1])
            if where is None:
                continue
            crate, path = self.plan.place[where]
            if (crate, tuple(path)) not in groups:
                groups.append((crate, tuple(path)))
        if not groups:
            groups.append(tuple(self.plan.place[target[1]][:1]) + (tuple(self.plan.place[target[1]][1]),))
        return groups

    def names(self, path, seen):
        if path in seen or path not in self.tree.mods:
            return set()
        seen.add(path)
        module = self.tree.mods[path]
        out = set(module.defs) | {bound for bound, *_ in module.uses if bound != "*"}
        for bound, segs, _vis, _at in module.uses:
            if bound == "*":
                found = self.tree.resolve_path(path, segs)
                if found and found[0] == "mod":
                    out |= self.names(found[1], seen)
        return out

    def is_test(self, ctx):
        path = ctx
        while path is not None:
            module = self.tree.mods.get(path)
            if module is None:
                return False
            if module.cfg and re.search(r"\btest\b", module.cfg) and not re.search(r"not\s*\(\s*test", module.cfg):
                return True
            if "🧪️tests" in module.file and self.tree.files.get(module.file) == path:
                return True
            path = module.parent
        return False

    def file_home(self, file, offset):
        ctx = self.context(file, offset)
        for move in self.plan.moves:
            if move["src_file"] == file and any(a <= offset < b for a, b, _k, _n in move["spans"]):
                crate, path = self.placed(move["dst"])
                return ctx, (crate, tuple(path))
        crate, path = self.placed(ctx)
        return ctx, (crate, tuple(path))

    def rewrite_file(self, file):
        """🔁️ (start, end, replacement) edits of one relocated file."""
        self.current = file
        raw, text = self.tree.raw[file], self.tree.text[file]
        edits, covered = [], []
        file_crate = self.plan.file_crate(file)
        for m in USE_STMT.finditer(text):
            ctx, home = self.file_home(file, m.start("body"))
            module_level = self.is_module_level(file, m.start())
            leaves_out, changed, private = [], False, []
            for segs, alias, self_leaf in leaves([], m.group("body")):
                glob = bool(segs) and segs[-1] == "*"
                core = segs[:-1] if glob else segs
                if not core:
                    leaves_out.append(None)
                    continue
                pre = self.tree.resolve_path(ctx, core) if m.group("vis") and module_level and m.group("vis").strip() == "pub" else None
                if pre and pre[0] == "ext" and file == self.tree.root:
                    leaves_out.append(None)
                    changed = True
                    continue
                self.dropping = bool(pre and pre[0] in ("def", "mod", "macro") and self.plan.place[pre[1]][0] != home[0])
                if not m.group("vis") and not glob and self.unused(file, m, alias or core[-1], ctx, core):
                    leaves_out.append(None)
                    changed = True
                    self.dropping = False
                    continue
                if glob and not m.group("vis"):
                    refined = self.refine_glob(file, m, ctx, core, home)
                    if refined is not None:
                        changed = True
                        for path_text, new_alias in refined:
                            leaves_out.append((path_text, new_alias, None))
                        self.dropping = False
                        continue
                result = self.leaf_text(core, alias, glob, ctx, home, self_leaf)
                self.dropping = False
                if m.group("vis") and module_level and result is not None:
                    target = self.tree.resolve_path(ctx, core)
                    if target and target[0] in ("def", "mod", "macro") and self.plan.place[target[1]][0] != home[0] and m.group("vis").strip() == "pub":
                        bound = alias or core[-1]
                        if self.mentions(bound, self.outside(file, m), target[0] == "mod") or self.mentions(bound, self.children_text(file), target[0] == "mod"):
                            rendered = self.plan.render_target(target, None)
                            r_crate, r_head, r_segs = self.render(rendered, home[0])
                            self.edges[home[0]][r_crate] += 1
                            self.detail[(home[0], r_crate)].add((os.path.relpath(self.current, REPO), "::".join(r_segs) + "  <=  facade " + "::".join(core)))
                            private.append(("::".join([r_head] + list(r_segs)) if r_segs else r_head, alias if alias else (None if r_segs and r_segs[-1] == bound else bound)))
                        result, changed = [], True
                    elif target and target[0] == "ext" and file == self.tree.root:
                        result, changed = [], True
                if result is None:
                    leaves_out.append(("::".join(core) + ("::*" if glob else ""), alias, None))
                else:
                    changed = True
                    for item in (result if isinstance(result, list) else [result]):
                        path_text, new_alias, new_glob = item
                        leaves_out.append((path_text + ("::*" if new_glob else ""), new_alias, None))
            covered.append((m.start(), m.end()))
            if not changed:
                continue
            kept = [leaf for leaf in leaves_out if leaf is not None]
            lines = [f"{m.group('attrs')}{m.group('indent')}use {path_text}{' as ' + alias if alias else ''};" for path_text, alias in private]
            groups = collections.OrderedDict()
            for path_text, alias, _ in kept:
                cfg = ""
                if path_text.startswith("#[cfg(test)] "):
                    cfg, path_text = "#[cfg(test)] ", path_text[len("#[cfg(test)] "):]
                head, _, last = path_text.rpartition("::")
                groups.setdefault((cfg, head), []).append(last + (f" as {alias}" if alias and alias != last else ""))
            for (cfg, head), tails in groups.items():
                body = f"{head}::{tails[0]}" if len(tails) == 1 and head else (f"{head}::{{{', '.join(tails)}}}" if head else tails[0])
                lines.append(f"{m.group('attrs')}{m.group('indent')}{cfg}{m.group('vis') or ''}use {body};")
            edits.append((m.start(), m.end(), "\n".join(lines)))
        for m in REL_PATH.finditer(text):
            if any(a <= m.start() < b for a, b in covered):
                continue
            segs = [x for x in re.split(SEP, m.group("path")) if x]
            ctx, home = self.file_home(file, m.start())
            result = self.leaf_text(segs, None, False, ctx, home)
            if result is None or result == []:
                continue
            path_text = result[0]
            edits.append((m.start(), m.end(), path_text))
        for m in NAME_PATH.finditer(text):
            if any(a <= m.start() < b for a, b in covered) or any(a <= m.start() < b for a, b, _ in edits):
                continue
            segs = [x for x in re.split(SEP, m.group("path")) if x]
            if segs[0] in ("crate", "super", "self", "Self"):
                continue
            ctx, home = self.file_home(file, m.start())
            found = self.tree.mods[ctx].defs.get(segs[0])
            if not (found and found[0] == "mod") and not (file == self.tree.root and ctx == ()):
                continue
            if file == self.tree.root and ctx == () and segs[0] not in self.tree.mods[()].children:
                continue
            result = self.leaf_text(segs, None, False, ctx, home)
            if result is None or result == []:
                continue
            edits.append((m.start(), m.end(), result[0]))
        return sorted(edits)

    def outside(self, file, m):
        """🙈️ The file's stripped text without any `use` statement and without the spans moved away from it."""
        key = file
        if key not in self.blanked:
            text = list(self.tree.text[file])
            for u in USE_STMT.finditer(self.tree.text[file]):
                text[u.start():u.end()] = " " * (u.end() - u.start())
            for move in self.plan.moves:
                if move["src_file"] == file:
                    for a, b, _k, _n in move["spans"]:
                        text[a:b] = " " * (b - a)
            if file == self.tree.root:
                for path, module in self.tree.mods.items():
                    if module.decl and module.decl[0] == file and module.parent == ():
                        a, b = module.decl[1], module.decl[2]
                        text[a:b] = " " * (b - a)
            self.blanked[key] = "".join(text)
        if m is not None:
            for move in self.plan.moves:
                if move["src_file"] == file:
                    for a, b, _k, _n in move["spans"]:
                        if a <= m.start() < b:
                            span = list(self.tree.text[file][a:b])
                            for u in USE_STMT.finditer(self.tree.text[file], a, b):
                                span[u.start() - a:u.end() - a] = " " * (u.end() - u.start())
                            return "".join(span)
        return self.blanked[key]

    @staticmethod
    def mentions(name, text, is_module):
        if is_module:
            return re.search(r"(?<![\w.:])" + re.escape(name) + r"\s*::", text) is not None
        return re.search(r"(?<![\w.:])" + re.escape(name) + r"(?![\w])(?!\s*:(?!:))", text) is not None or re.search(r"(?<![\w.:])" + re.escape(name) + r"\s*::", text) is not None

    def children_text(self, file, tests=None):
        """👶️ Stripped text of the file modules mounted by this file (all, only test ones, or only lib ones)."""
        path = self.tree.files[file]
        out = []
        for child_path, module in self.tree.mods.items():
            if self.tree.files.get(module.file) == child_path and module.file != file and module.parent == path:
                if (tests is None or self.is_test(child_path) == tests) and re.search(r"\buse\s+super\s*::\s*\*", self.tree.text[module.file]):
                    out.append(self.tree.text[module.file])
        return "\n".join(out)

    def unused(self, file, m, bound, ctx, core):
        """🗑️ True for an import of an internal non-trait item whose name its file and child files never mention."""
        target = self.tree.resolve_path(ctx, core)
        if not target or target[0] not in ("def", "mod"):
            return False
        if target[0] == "def":
            kind = self.tree.mods[target[1]].defs.get(target[2], ("?",))[0]
            if kind not in ("fn", "struct", "enum", "const", "static", "type", "union"):
                return False
        is_module = target[0] == "mod"
        if self.mentions(bound, self.outside(file, m), is_module) or self.mentions(bound, self.children_text(file), is_module):
            return False
        return True

    def refine_glob(self, file, m, ctx, core, home):
        """🔍️ Replaces a glob whose module now spans other crates (or the aggregator root) by explicit imports of the names
        the file uses; None keeps the ordinary glob handling."""
        target = self.tree.resolve_path(ctx, core)
        if not target or target[0] != "mod":
            return None
        facade = self.facade_target(target[1])
        crate = self.plan.place[facade][0]
        if crate == home[0] and facade != ():
            return None
        names = self.names(facade, set())
        lib_text = self.outside(file, m) + "\n" + self.children_text(file, False)
        test_text = self.children_text(file, True)
        used, test_only = set(), set()
        for name in names:
            found = self.tree.resolve_name(facade, name)
            if found and self.mentions(name, lib_text, found[0] == "mod"):
                used.add(name)
            elif found and self.mentions(name, test_text, found[0] == "mod"):
                used.add(name)
                test_only.add(name)
        own = set(self.tree.mods[ctx].defs)
        out = []
        for name in sorted(used - own):
            found = self.tree.resolve_name(facade, name)
            if not found:
                continue
            if found[0] == "ext":
                continue
            if found[0] == "mod":
                rendered = self.plan.render_target(("mod", found[1], []), None)
            else:
                rendered = self.plan.render_target((found[0], found[1], found[2], []), None)
            n_crate, head, segs = self.render(rendered, home[0])
            if (n_crate, tuple(segs[:-1])) == (home[0], tuple(home[1])):
                continue
            if n_crate != home[0] and not self.is_test(ctx) and name not in test_only:
                self.edges[home[0]][n_crate] += 1
                self.detail[(home[0], n_crate)].add((os.path.relpath(self.current, REPO), "::".join(segs) + "  <=  glob " + "::".join(core)))
            elif n_crate != home[0]:
                self.test_edges[home[0]][n_crate] += 1
                self.test_detail[(home[0], n_crate)].add((os.path.relpath(self.current, REPO), "::".join(segs) + "  <=  glob " + "::".join(core)))
            prefix = "#[cfg(test)] " if name in test_only and not self.is_test(ctx) else ""
            if not segs:
                out.append((prefix + head, name))
            else:
                out.append((prefix + "::".join([head] + segs), None if segs[-1] == name else name))
        return out

    def is_module_level(self, file, offset):
        ctx = self.context(file, offset)
        module = self.tree.mods[ctx]
        return any(a == offset or (a <= offset < b and not self.tree.text[file][a:offset].strip()) for a, b in self.tree.top_level(module))


def apply_edits(raw, edits):
    out, last = [], 0
    for a, b, rep in sorted(edits, key=lambda e: (e[0], e[1])):
        if a < last:
            continue
        out.append(raw[last:a])
        out.append(rep)
        last = b
    out.append(raw[last:])
    return "".join(out)


VIS = re.compile(r"\bpub\s*\(\s*(?:crate|super|in\s+[^)]*)\s*\)")


def item_extent(raw, text, start, end):
    """📏️ Extends an item span back over its attached doc comments and attributes (no blank line in between)."""
    line_start = raw.rfind("\n", 0, start) + 1
    while line_start > 0:
        prev_start = raw.rfind("\n", 0, line_start - 1) + 1
        line = raw[prev_start:line_start - 1]
        if line.strip() and DOC_LINE.match(line) and not line.strip().startswith("//#"):
            line_start = prev_start
        else:
            break
    stop = end
    if raw[stop:stop + 1] == "\n":
        stop += 1
    return line_start, stop


class Relocation:
    """🚚️ Applies the plan: item moves, path rewrites, mount regeneration, facade removal, visibility, consumers."""

    def __init__(self, plan):
        self.plan, self.tree = plan, plan.tree
        self.rw = Rewriter(plan)
        self.extra = collections.defaultdict(list)
        self.removals = collections.defaultdict(list)
        self.created = {}

    def file_edits(self):
        edits = {}
        for file in self.tree.files:
            if self.plan.file_crate(file) is None:
                continue
            found = self.rw.rewrite_file(file)
            text = self.tree.text[file]
            for m in VIS.finditer(text):
                found.append((m.start(), m.end(), "pub"))
            edits[file] = found
        return edits

    def mounts(self):
        """🪢️ Decides kept vs regenerated mounts; fills removals and generated mount tables."""
        generated = collections.defaultdict(list)
        root = self.tree.root
        for path, module in self.tree.mods.items():
            if module.decl is None:
                continue
            decl_file, a, b = module.decl
            is_file = self.tree.files.get(module.file) == path
            if not is_file:
                if decl_file == root and self.tree.real_parent(path) == ():
                    self.removals[root].append((a, b))
                continue
            crate, new = self.plan.place[path]
            if self.plan.crate_roots.get(crate) == module.file:
                if decl_file != root:
                    self.removals[decl_file].append((a, b))
                continue
            ctx = self.rw.context(decl_file, a)
            keep = decl_file != root and path not in TRANSPARENT and self.plan.place[ctx] == (crate, tuple(new[:-1])) and new and new[-1] == path[-1]
            if keep:
                continue
            if decl_file != root:
                self.removals[decl_file].append((a, b))
                if path in TRANSPARENT:
                    text = self.tree.text[decl_file]
                    g = re.compile(r"\bpub\s+use\s+" + path[-1] + r"\s*::\s*\*\s*;").search(text, b)
                    if g:
                        self.removals[decl_file].append((g.start(), g.end()))
            generated[crate].append((tuple(new), module.file, module.cfg, getattr(module, "vis", "") or ("pub" if path in TRANSPARENT else "")))
        return generated

    def place_mounts(self, generated):
        """🌳️ Hosts each generated mount in the deepest file module of its crate that prefixes it."""
        by_place = {}
        for path, module in self.tree.mods.items():
            if self.tree.files.get(module.file) == path:
                by_place[self.plan.place[path]] = module.file
        for crate, root_file in self.plan.crate_roots.items():
            by_place[(crate, ())] = root_file
        hosted = collections.defaultdict(list)
        for crate, mounts in generated.items():
            for new, file, cfg, vis in mounts:
                k = len(new) - 1
                while k > 0 and (crate, tuple(new[:k])) not in by_place:
                    k -= 1
                host = by_place[(crate, tuple(new[:k]))]
                hosted[host].append((tuple(new[k:]), file, cfg, vis))
        return hosted

    def emit_mounts(self, host, mounts):
        base = os.path.dirname(host)
        tree = {}
        for rel, file, cfg, vis in sorted(mounts):
            node = tree
            for seg in rel[:-1]:
                node = node.setdefault(("node", seg), {})
            node[("leaf", rel[-1])] = (file, cfg, vis)

        def walk(node, depth):
            pad = "    " * depth
            lines = []
            for (kind, name), value in sorted(node.items(), key=lambda kv: (kv[0][1], kv[0][0])):
                if kind == "leaf":
                    file, cfg, vis = value
                    if cfg:
                        lines.append(f"{pad}#[cfg({cfg})]")
                    lines.append(f'{pad}#[path = "{os.path.relpath(file, base)}"]')
                    lines.append(f"{pad}{vis + ' ' if vis else ''}mod {name};")
                else:
                    lines.append(f'{pad}#[path = "."]')
                    lines.append(f"{pad}pub mod {name} {{")
                    lines += walk(value, depth + 1)
                    lines.append(f"{pad}}}")
            return lines

        return "\n".join(["//#region 🪢️TaxonomyMounts"] + walk(tree, 0) + ["//#endregion 🪢️TaxonomyMounts"])

    def bare_imports(self, text, ctx, home, exclude, own_names):
        """📥️ `use` lines a moved block needs for names its old module had in scope."""
        lines = []
        idents = set(re.findall(r"(?<![\w.:])(" + IDENT + r")\b(?!\s*!)", text)) - exclude - own_names
        module = self.tree.mods[ctx]
        bound = {name: segs for name, segs, _vis, _at in module.uses if name != "*"}
        for name in sorted(idents):
            if name in module.defs and module.defs[name][0] not in ("mod",):
                target = ("def", ctx, name, [])
            elif name in module.defs:
                target = ("mod", module.children[name], [])
            elif name in bound:
                target = self.tree.resolve_path(ctx, bound[name])
                if target and target[0] == "ext":
                    lines.append(f"use {'::'.join(bound[name])}{'' if bound[name][-1] == name else ' as ' + name};")
                    continue
            else:
                continue
            if not target or target[0] == "ext":
                continue
            rendered = self.plan.render_target(target, None)
            crate, head, segs = self.rw.render(rendered, home[0])
            if (crate, tuple(segs[:-1])) == (home[0], tuple(home[1])) and target[0] != "mod":
                continue
            if target[0] == "mod" and not segs:
                lines.append(f"use {head} as {name};")
            else:
                lines.append(f"use {'::'.join([head] + list(segs))}{'' if segs and segs[-1] == name else ' as ' + name};")
        return lines

    def build(self):
        edits = self.file_edits()
        generated = self.mounts()
        hosted = self.place_mounts(generated)
        out = {}
        appended = collections.defaultdict(list)
        imports = collections.defaultdict(list)
        for move in self.plan.moves:
            file = move["src_file"]
            raw, text = self.tree.raw[file], self.tree.text[file]
            dst_home = (self.plan.place[move["dst"]][0], tuple(self.plan.place[move["dst"]][1]))
            src_home = (self.plan.place[move["src"]][0], tuple(self.plan.place[move["src"]][1]))
            dst_names = set(self.tree.mods[move["dst"]].defs) | {n for n, *_ in self.tree.mods[move["dst"]].uses}
            for a, b, _kind, _name in sorted(move["spans"]):
                inner = [(x - a, y - a, r) for x, y, r in edits[file] if a <= x < b]
                edits[file] = [e for e in edits[file] if not (a <= e[0] < b)] + [(a, b, "")]
                block = apply_edits(raw[a:b], inner)
                appended[move["dst_file"]].append(block.rstrip("\n"))
                imports[move["dst_file"]] += self.bare_imports(text[a:b], move["src"], dst_home, move["names"], dst_names)
            rest = "".join(ch if not any(a <= i < b for a, b, _k, _n in move["spans"]) else " " for i, ch in enumerate(text))
            used = set(re.findall(r"(?<![\w.:])(" + IDENT + r")\b", rest)) & move["names"]
            for name in sorted(used):
                rendered = self.plan.render_target(("def", move["dst"], name, []), None)
                crate, head, segs = self.rw.render(rendered, src_home[0])
                imports[file].append(f"use {'::'.join([head] + list(segs))};")
        for file in self.tree.files:
            raw = self.tree.raw[file]
            found = list(edits.get(file, []))
            for a, b in self.removals.get(file, []):
                stop = b + 1 if raw[b:b + 1] == "\n" else b
                found = [e for e in found if not (a <= e[0] < stop)] + [(a, stop, "")]
            text = re.sub(r"\n{4,}", "\n\n\n", apply_edits(raw, found))
            if appended.get(file):
                text = text.rstrip("\n") + "\n\n" + "\n\n".join(appended[file]) + "\n"
            out[file] = text
        for crate, root_file in self.plan.crate_roots.items():
            if root_file not in out:
                spec = self.plan.config["crates"][crate]
                out[root_file] = spec.get("doc", "//! 🧩️ " + crate) + "\n"
        for file, extra in appended.items():
            if not self.tree.raw.get(file):
                spec = next((self.plan.config["crates"][c] for c, r in self.plan.crate_roots.items() if r == file), {})
                out[file] = spec.get("doc", "//! 🧩️") + "\n\n" + "\n\n".join(extra) + "\n"
        for file, lines in imports.items():
            lines = list(dict.fromkeys(lines))
            text = out[file]
            m = re.match(r"((?:[ \t]*//![^\n]*\n)*(?:[ \t]*#!\[[^\n]*\n|[ \t]*\n)*)", text)
            out[file] = text[:m.end()] + "\n".join(lines) + "\n" + text[m.end():]
        for host, mounts in hosted.items():
            out[host] = out[host].rstrip("\n") + "\n\n" + self.emit_mounts(host, mounts) + "\n"
        for crate, root_file in self.plan.crate_roots.items():
            if not out[root_file].lstrip().startswith("//!"):
                out[root_file] = self.plan.config["crates"][crate].get("doc", "//! 🧩️ " + crate) + "\n\n" + out[root_file].lstrip("\n")
        for crate, root_file in self.plan.crate_roots.items():
            header = self.plan.config["crates"][crate].get("header", [])
            if header:
                text = out[root_file]
                missing = [line for line in header if line and line not in text]
                if missing:
                    m = re.match(r"((?:[ \t]*//![^\n]*\n)*)", text)
                    out[root_file] = text[:m.end()] + "\n" + "\n".join(missing) + "\n" + text[m.end():]
        return out

    def consumers(self):
        ident = self.plan.config["crate"]
        found = {}
        pattern = re.compile(r"(?<![\w:])" + re.escape(ident) + r"(?P<rest>(?:" + SEP + IDENT + r")+)")
        for root in self.plan.config.get("consumers", []):
            for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, root)):
                dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated", ".git")]
                for name in filenames:
                    if not name.endswith(".rs"):
                        continue
                    path = os.path.normpath(os.path.join(dirpath, name))
                    if path in self.tree.files:
                        continue
                    raw = open(path, encoding="utf-8").read()
                    if ident not in raw:
                        continue
                    text = strip(raw)
                    edits, used, covered = [], collections.Counter(), []
                    for m in USE_STMT.finditer(text):
                        body = m.group("body").strip()
                        if not re.match(re.escape(ident) + r"\b", body):
                            continue
                        lines = []
                        for segs, alias, self_leaf in leaves([], body):
                            glob = segs[-1] == "*"
                            core = (segs[:-1] if glob else segs)[1:]
                            target = self.tree.resolve_path((), core) if core else ("mod", (), [])
                            rendered = self.plan.render_target(target, None) if target and target[0] != "ext" else None
                            if target and target[0] == "ext":
                                text_path = "::".join(target[1])
                            elif rendered:
                                crate, _h, new = rendered
                                used[crate] += 1
                                text_path = "::".join([self.plan.idents[crate]] + list(new))
                                if glob and target[0] == "mod":
                                    groups = self.rw.glob_groups(target, None)
                                    lines += [f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {'::'.join([self.plan.idents[c]] + list(p))}::*;" for c, p in groups]
                                    for c, _p in groups:
                                        used[c] += 1
                                    continue
                            else:
                                text_path = "::".join(segs[:-1] if glob else segs)
                            tail = "::*" if glob else (f" as {alias}" if alias else (f" as {segs[-1]}" if rendered and not rendered[2] else ""))
                            lines.append(f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {text_path}{tail};")
                        edits.append((m.start(), m.end(), "\n".join(lines)))
                        covered.append((m.start(), m.end()))
                    for m in pattern.finditer(text):
                        if any(a <= m.start() < b for a, b in covered):
                            continue
                        segs = [x for x in re.split(SEP, m.group("rest")) if x]
                        target = self.tree.resolve_path((), segs)
                        if not target:
                            continue
                        if target[0] == "ext":
                            edits.append((m.start(), m.end(), "::".join(target[1])))
                            continue
                        crate, _h, new = self.plan.render_target(target, None)
                        used[crate] += 1
                        edits.append((m.start(), m.end(), "::".join([self.plan.idents[crate]] + list(new))))
                    found[path] = (apply_edits(raw, edits), dict(used))
        return found


def main():
    config = json.load(open(sys.argv[1], encoding="utf-8"))
    mode = sys.argv[2]
    plan = Plan(config)
    relocation = Relocation(plan)
    out = relocation.build()
    consumers = relocation.consumers()
    rw = relocation.rw
    crates = collections.defaultdict(list)
    for file, path in plan.tree.files.items():
        crates[plan.place[path][0]].append(file)
    for crate in sorted(crates):
        print(f"crate {crate}: {len(crates[crate])} files -> {dict(rw.edges.get(crate, {}))}  tests-> {dict(rw.test_edges.get(crate, {}))}")
        if "--verbose" in sys.argv:
            for file in sorted(crates[crate]):
                print(f"   {'::'.join(plan.place[plan.tree.files[file]][1]) or '<root>'}\t{plan.rel(file)}")
    graph = {c: set(e) for c, e in rw.edges.items()}
    state, stack, cycles = {}, [], []
    def visit(node):
        state[node] = 1
        stack.append(node)
        for nxt in sorted(graph.get(node, ())):
            if state.get(nxt) == 1:
                cycles.append(stack[stack.index(nxt):] + [nxt])
            elif not state.get(nxt):
                visit(nxt)
        stack.pop()
        state[node] = 2
    for node in sorted(graph):
        if not state.get(node):
            visit(node)
    for cycle in cycles:
        print("CYCLE " + " -> ".join(cycle))
        for a, b in zip(cycle, cycle[1:]):
            rows = sorted(rw.detail[(a, b)])
            for file, path in rows[:int(os.environ.get("SHOW", "6"))]:
                print(f"    {a}->{b}: {path}   [{file.split('/🪆️subsets/')[-1] if '/🪆️subsets/' in file else file[-70:]}]")
    for spec in filter(None, os.environ.get("EDGE", "").split(",")):
        a, b = spec.split(">")
        for file, path in sorted(rw.detail[(a, b)]):
            print(f"EDGE {a}->{b}: {path}   [{file.split('/🪆️subsets/')[-1] if '/🪆️subsets/' in file else file[-80:]}]")
        for file, path in sorted(rw.test_detail[(a, b)]):
            print(f"TEST-EDGE {a}->{b}: {path}   [{file.split('/🪆️subsets/')[-1] if '/🪆️subsets/' in file else file[-80:]}]")
    for (path, file), count in sorted(rw.unresolved.items()):
        print(f"UNRESOLVED {path} x{count} in {file}")
    for path, (_text, used) in sorted(consumers.items()):
        print(f"consumer {json.dumps(used, sort_keys=True)} {plan.rel(path)}")
    layout = {"crates": {}, "edges": {c: dict(e) for c, e in rw.edges.items()}, "test_edges": {c: dict(e) for c, e in rw.test_edges.items()}}
    for crate, files in crates.items():
        lib = sorted(plan.rel(f) for f in files if not rw.is_test(plan.tree.files[f]))
        tests = sorted(plan.rel(f) for f in files if rw.is_test(plan.tree.files[f]))
        root = plan.rel(plan.crate_roots[crate])
        if root not in lib:
            lib.insert(0, root)
        layout["crates"][crate] = {"ident": plan.idents[crate], "root": root, "lib": lib, "tests": tests}
    json.dump(layout, open(os.path.join(HERE, "🗑️generated", os.path.basename(sys.argv[1]).replace(".json", ".layout.json")), "w"), indent=1, ensure_ascii=False)
    changed = [f for f, t in out.items() if not os.path.exists(f) or open(f, encoding="utf-8").read() != t]
    print(f"files changed: {len(changed)} consumers: {len(consumers)}")
    if mode == "apply":
        for file in changed:
            os.makedirs(os.path.dirname(file), exist_ok=True)
            open(file, "w", encoding="utf-8").write(out[file])
        for path, (text, _used) in consumers.items():
            open(path, "w", encoding="utf-8").write(text)
        json.dump({plan.rel(p): u for p, (_t, u) in consumers.items()}, open(os.path.join(HERE, "🗑️generated", os.path.basename(sys.argv[1]).replace(".json", ".consumers.json")), "w"), indent=1, ensure_ascii=False)
        json.dump({c: dict(e) for c, e in rw.edges.items()}, open(os.path.join(HERE, "🗑️generated", os.path.basename(sys.argv[1]).replace(".json", ".edges.json")), "w"), indent=1, ensure_ascii=False)
    elif mode == "diff":
        import difflib
        outdir = os.path.join(HERE, "🗑️generated", "preview")
        for file in changed:
            old = open(file, encoding="utf-8").read().splitlines(keepends=True) if os.path.exists(file) else []
            sys.stdout.writelines(difflib.unified_diff(old, out[file].splitlines(keepends=True), plan.rel(file), plan.rel(file), n=1))


if __name__ == "__main__":
    main()
