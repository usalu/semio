#!/usr/bin/env python3
"""✂️ Cuts one `#[path]`-tree artifact monolith into a family of owner-rooted crates (split lane P7).

Every family crate is rooted at its owner directory's `🦀️.rs` (`<owner>/📦️packages/🦀️rust/Cargo.toml`, `path =
"../../🦀️.rs"`), so its modules are re-rooted: the owner's old module path becomes the crate root and every file the
config assigns to the crate keeps its path below that root (a file outside the owner subtree keeps its full old path).
The monolith root stays the aggregator. Every `crate::`/`super::`/`self::` path (and every root-relative path of the
monolith root) is resolved against the old module tree, through the root's facades and re-exports, and rewritten to
the owning crate; relative paths that stay valid are kept. External `<monolith>::…` paths are retargeted the same way.
Selected files become `[[test]]` targets of a family crate. Nothing is re-exported.

usage: ✂️family-cut.py <config.json> plan|apply
config: {"artifact": dir, "crate": monolith ident, "consumers": [repo roots],
         "crates": [{"package": name, "owner": artifact-relative dir, "claims": [regex on artifact-relative file], "doc": "//! ...",
                     "description": str, "create_root": "//! doc of a new crate root file", "module": "old::module::path of a new root"}],
         "tests": [{"package": name, "file": artifact-relative file, "name": ident}],
         "header": [crate-root attribute and extern lines], "drop": [regex on artifact-relative file to unmount],
         "moves": [{"from": "crate::a", "to": "crate::b", "items": [names]}], "moves_files": [`🚚️move-items.py` records]}
"""
import collections, json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
SEP = r"\s*::\s*"


def strip(src):
    """🧹️ Blanks comments, string, raw-string and char literals while keeping every byte offset; `#[path]` strings stay."""
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
        elif c in "rb" and re.match(r'(?:br|r)#*"', src[i:i + 40]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = re.match(r'(?:br|r)(#*)"', src[i:])
            close = '"' + m.group(1)
            j = src.find(close, i + len(m.group(0)))
            j = n if j < 0 else j + len(close)
            out.append(blank(src[i:j]))
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


def close_of(text, i):
    """🔚️ Index of the bracket closing the one at `i` (stripped text)."""
    pairs = {"{": "}", "(": ")", "[": "]"}
    stack = [pairs[text[i]]]
    j = i + 1
    while stack:
        ch = text[j]
        if ch in pairs:
            stack.append(pairs[ch])
        elif ch in ")]}":
            stack.pop()
        j += 1
    return j - 1


def skip_item(text, i, end):
    """⏭️ End (exclusive) of the item starting at `i`: its `;` or the `}` of its first top-level block."""
    j = i
    while j < end:
        ch = text[j]
        if ch == ";":
            return j + 1
        if ch in "([":
            j = close_of(text, j) + 1
            continue
        if ch == "{":
            return close_of(text, j) + 1
        if ch == "}":
            return j
        j += 1
    return end


MODDECL = re.compile(r"(?P<vis>pub(?:\s*\([^)]*\))?\s+)?mod\s+(?P<name>" + IDENT + r")\s*(?P<kind>[;{])")
GLOB = re.compile(r"pub\s+use\s+component\s*::\s*\*\s*;")
PATHATTR = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')


class Mod:
    def __init__(self, path, name, kind, parent, file, decl_file=None, decl_span=None, vis="", attrs=None, dir=None):
        self.path, self.name, self.kind, self.parent, self.file = path, name, kind, parent, file
        self.decl_file, self.decl_span, self.vis, self.attrs, self.dir = decl_file, decl_span, vis, attrs or [], dir
        self.children = collections.OrderedDict()
        self.glob = None
        self.in_file = None

    def cfg(self):
        return [a for a in self.attrs if a.startswith("#[cfg")]

    def test(self):
        return any(re.search(r"\btest\b", a) and not re.search(r"not\s*\(\s*test", a) for a in self.cfg())


class Tree:
    """🌳️ The monolith's module tree: file and inline modules, their declarations and the files' stripped text."""

    def __init__(self, root, inside=None):
        self.root_file = os.path.normpath(root)
        self.inside = os.path.normpath(inside) + os.sep if inside else None
        self.text, self.raw, self.files = {}, {}, {}
        self.root = Mod((), "", "file", None, self.root_file, dir=os.path.dirname(self.root_file))
        self.load(self.root, False)

    def load(self, mod, test):
        file = mod.file
        if file in self.files:
            raise SystemExit(f"file mounted twice: {file}")
        self.files[file] = mod
        mod.test_gated = test
        self.raw[file] = open(file, encoding="utf-8").read()
        self.text[file] = strip(self.raw[file])
        self.body(mod, file, 0, len(self.text[file]), mod.dir, test)

    def body(self, mod, file, start, end, directory, test):
        text = self.text[file]
        i, attrs, attrs_start = start, [], None
        while i < end:
            while i < end and text[i] in " \t\r\n":
                i += 1
            if i >= end:
                break
            if text.startswith("#![", i):
                i = close_of(text, i + 2) + 1
                continue
            if text.startswith("#[", i):
                j = close_of(text, i + 1)
                attrs.append(text[i:j + 1])
                attrs_start = i if attrs_start is None else attrs_start
                i = j + 1
                continue
            m = MODDECL.match(text, i)
            if m:
                name, vis, kind = m.group("name"), (m.group("vis") or "").strip(), m.group("kind")
                pattr = next((PATHATTR.match(a).group(1) for a in attrs if PATHATTR.match(a)), None)
                rest = [a for a in attrs if not PATHATTR.match(a)]
                decl_start = attrs_start if attrs_start is not None else m.start()
                child_test = test or any(re.search(r"\btest\b", a) and not re.search(r"not\s*\(\s*test", a) for a in rest if a.startswith("#[cfg"))
                if kind == "{":
                    close = close_of(text, m.end() - 1)
                    sub = os.path.normpath(os.path.join(directory, pattr)) if pattr else os.path.join(directory, name)
                    child = Mod(mod.path + (name,), name, "inline", mod, None, file, (decl_start, close + 1), vis, rest, sub)
                    child.in_file = file
                    child.test_gated = child_test
                    mod.children[name] = child
                    self.body(child, file, m.end(), close, sub, child_test)
                    i = close + 1
                else:
                    if pattr is None:
                        raise SystemExit(f"mod {name} without #[path] in {file}")
                    target = os.path.normpath(os.path.join(directory, pattr))
                    child = Mod(mod.path + (name,), name, "file", mod, target, file, (decl_start, m.end()), vis, rest, os.path.dirname(target))
                    mod.children[name] = child
                    if self.inside and not target.startswith(self.inside):
                        child.external = True
                        child.test_gated = child_test
                    elif os.path.exists(target):
                        self.load(child, child_test)
                    else:
                        raise SystemExit(f"missing module file {target}")
                    i = m.end()
                attrs, attrs_start = [], None
                continue
            g = GLOB.match(text, i)
            if g:
                mod.glob = (attrs_start if attrs_start is not None else g.start(), g.end(), [a for a in attrs if a.startswith("#[cfg")])
                i = g.end()
                attrs, attrs_start = [], None
                continue
            i = skip_item(text, i, end)
            attrs, attrs_start = [], None

    def walk(self, mod=None):
        mod = mod or self.root
        yield mod
        for child in mod.children.values():
            yield from self.walk(child)

    def resolve(self, segs):
        """🧭️ Deepest module of an absolute path (through private `component` globs) and the remaining segments."""
        node, i = self.root, 0
        while i < len(segs):
            s = segs[i]
            child = node.children.get(s)
            if child is None and node.glob and "component" in node.children:
                child = node.children["component"].children.get(s)
            if child is None:
                break
            node, i = child, i + 1
        return node, list(segs[i:])


def leaves(prefix, body):
    m = re.match(r"^((?:" + IDENT + SEP + r")*)\{(.*)\}$", body.strip(), re.S)
    if m:
        pre = prefix + [x for x in re.split(SEP, m.group(1)) if x]
        out = []
        for part in split_top(m.group(2)):
            out += leaves(pre, part)
        return out
    alias = re.search(r"\s+as\s+(" + IDENT + r")$", body.strip())
    core = re.sub(r"\s+as\s+" + IDENT + r"$", "", body.strip())
    return [(prefix + [x for x in re.split(SEP, core) if x], alias.group(1) if alias else None)]


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


def inline_spans(text):
    spans = []
    for m in re.finditer(r"(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+(" + IDENT + r")\s*\{", text):
        spans.append((m.end(), close_of(text, m.end() - 1), m.group(1)))
    return spans


def test_spans(text):
    out = []
    for m in re.finditer(r"#\[cfg\(\s*(?:all\([^)]*\btest\b[^)]*\)|test)\s*\)\]", text):
        j = m.end()
        while True:
            k = j
            while k < len(text) and text[k] in " \t\r\n":
                k += 1
            if text.startswith("#[", k):
                j = close_of(text, k + 1) + 1
                continue
            break
        out.append((m.start(), skip_item(text, k, len(text))))
    return out


class Cut:
    def __init__(self, config_path):
        self.config = json.load(open(config_path, encoding="utf-8"))
        self.artifact = os.path.join(REPO, self.config["artifact"])
        self.tree = Tree(os.path.join(self.artifact, "🦀️.rs"), self.artifact)
        self.ident = self.config["crate"]
        self.package = self.ident.replace("_", "-")
        self.crates = collections.OrderedDict()
        self.crates[self.package] = {"package": self.package, "owner": "", "claims": [], "aggregator": True}
        for spec in self.config["crates"]:
            self.crates[spec["package"]] = dict(spec)
        self.idents = {name: name.replace("-", "_") for name in self.crates}
        self.tests = {os.path.normpath(os.path.join(self.artifact, t["file"])): t for t in self.config.get("tests", [])}
        self.drop = [re.compile(r) for r in self.config.get("drop", [])]
        self.assign()
        self.remap = {}
        for name, spec in self.crates.items():
            for rule in [re.compile(r) for r in spec.get("flatten", [])]:
                for file, mod in self.tree.files.items():
                    if self.crate_of[file] == name and rule.search(self.rel(file)):
                        self.remap[tuple(mod.path)] = (name, (mod.name,))
        self.aliases = self.derive_aliases()
        base = os.path.dirname(os.path.abspath(config_path))
        moves = list(self.config.get("moves", []))
        for name in self.config.get("moves_files", []):
            moves += json.load(open(os.path.join(base, name), encoding="utf-8"))
        for move in moves:
            src = [s for s in move["from"].split("::") if s != "crate"]
            dst = [s for s in move["to"].split("::") if s != "crate"]
            for item in move["items"]:
                self.aliases.append((tuple(src + [item]), tuple(dst + [item]), "item"))
        self.aliases.sort(key=lambda a: -len(a[0]))
        self.problems = []

    def rel(self, path):
        return os.path.relpath(path, self.artifact)

    def assign(self):
        """🏷️ File -> crate: first matching claim, else the deepest owner directory containing the file."""
        self.crate_of, self.owner_file = {}, {}
        for name, spec in self.crates.items():
            self.owner_file[name] = os.path.normpath(os.path.join(self.artifact, spec["owner"], "🦀️.rs"))
        claims = [(re.compile(r), name) for name, spec in self.crates.items() for r in spec.get("claims", [])]
        for file, mod in self.tree.files.items():
            rel = self.rel(file)
            hit = next((name for rule, name in claims if rule.search(rel)), None)
            if hit is None:
                best = None
                for name, spec in self.crates.items():
                    owner = spec["owner"]
                    if owner == "" or rel == owner or rel.startswith(owner.rstrip("/") + "/"):
                        if best is None or len(owner) > len(self.crates[best]["owner"]):
                            best = name
                hit = best
            self.crate_of[file] = hit
        self.owner_path = {}
        for name, file in self.owner_file.items():
            spec = self.crates[name]
            if spec.get("create_root"):
                if os.path.exists(file) and file in self.tree.files:
                    raise SystemExit(f"new root of {name} already mounted: {file}")
                self.owner_path[name] = tuple(spec["module"].split("::"))
                continue
            if file not in self.tree.files:
                raise SystemExit(f"owner file of {name} is not mounted: {file}")
            if self.crate_of[file] != name:
                raise SystemExit(f"owner file of {name} is claimed by {self.crate_of[file]}")
            mod = self.tree.files[file]
            self.owner_path[name] = mod.parent.path if mod.name == "component" and mod.parent and mod.parent.glob else mod.path

    def derive_aliases(self):
        """🪞️ Every `pub use` re-export of the monolith as a path alias: root facades (`pub mod x { pub use …::*; }`), root
        and module item re-exports and globs. Re-exports of a module's own inline submodules and of `component` stay."""
        aliases = []
        root = self.tree.root
        self.reexports = []

        def target_of(segs, base):
            if segs[0] == "crate":
                return segs[1:]
            if segs[0] in ("super", "self"):
                b, k = list(base), 0
                while k < len(segs) and segs[k] in ("super", "self"):
                    if segs[k] == "super":
                        if not b:
                            return None
                        b.pop()
                    k += 1
                return b + segs[k:]
            return segs

        for file, mod in self.tree.files.items():
            text = self.tree.text[file]
            is_root = file == self.tree.root_file
            for m in re.finditer(r"(?m)^(?P<indent>[ \t]*)pub\s+use\s+(?P<body>[^;]*);", text):
                base = self.module_at(file, m.start())
                public = self.public(base)
                head = re.match(IDENT, m.group("body").strip())
                if not head:
                    continue
                if not is_root and head.group(0) not in ("crate", "super", "self"):
                    continue
                rows = []
                for segs, alias in leaves([], m.group("body")):
                    if segs and segs[0] == "component":
                        continue
                    glob = segs[-1] == "*"
                    path = target_of([x for x in segs if x != "*"], base)
                    if not path:
                        continue
                    if is_root and path[0] not in root.children and head.group(0) not in ("crate", "super", "self"):
                        continue
                    node, rest = self.tree.resolve(path)
                    if node.in_file == file and not rest and node.kind == "inline" and tuple(node.path[:-1]) == tuple(base):
                        continue
                    if glob:
                        if public and is_root:
                            aliases.append((tuple(public), tuple(path), "facade"))
                        else:
                            for item in self.item_names(node):
                                aliases.append((tuple(public) + (item,), tuple(path) + (item,), "glob"))
                    else:
                        name = alias or path[-1]
                        aliases.append((tuple(public) + (name,), tuple(path), "item"))
                    rows.append((segs, alias, path))
                if rows and not is_root:
                    self.reexports.append((file, m.start(), m.end(), m.group("indent"), rows))
        aliases.sort(key=lambda a: -len(a[0]))
        return aliases

    def public(self, segs):
        """🌐️ An absolute module path without the private `component` segments its parents glob re-export."""
        out, node = [], self.tree.root
        for seg in segs:
            child = node.children.get(seg) if node is not None else None
            if seg == "component" and node is not None and node.glob:
                node = child
                continue
            out.append(seg)
            node = child
        return out

    def item_names(self, node):
        file = node.file if node.kind == "file" else (node.children["component"].file if "component" in node.children else None)
        if file is None:
            return []
        text = self.tree.text[file]
        names = re.findall(r"(?m)^pub\s+(?:async\s+|const\s+(?=fn)|unsafe\s+)*(?:fn|struct|enum|trait|type|const|static|mod|union)\s+(" + IDENT + r")", text)
        return sorted(set(names))

    def module_at(self, file, offset):
        """📍️ Old absolute module path at an offset of a file (inline modules included)."""
        mod = self.tree.files[file]
        base = list(mod.path)
        for a, b, name in inline_spans(self.tree.text[file]):
            if a <= offset < b:
                base.append(name)
        return base

    def unalias(self, segs):
        for _ in range(6):
            for alias, target, kind in self.aliases:
                if tuple(segs[:len(alias)]) == alias and (kind != "item" or len(segs) >= len(alias)):
                    segs = list(target) + list(segs[len(alias):])
                    break
            else:
                return segs
        return segs

    def home(self, node, rest):
        """🏠️ Crate that owns an item path ending at `node` (inline modules resolve to their component's file)."""
        if node.kind == "file":
            return self.crate_of[node.file] if node.file in self.crate_of else self.crate_of[node.decl_file]
        if "component" in node.children and node.glob:
            if rest or True:
                return self.crate_of[node.children["component"].file]
        if node.in_file and node.in_file != self.tree.root_file:
            return self.crate_of[node.in_file]
        crates = {self.crate_of[m.file] for m in self.tree.walk(node) if m.kind == "file"}
        if len(crates) == 1:
            return crates.pop()
        if node.in_file == self.tree.root_file and not rest:
            return None
        return self.package

    def new_path(self, crate, segs):
        """🧭️ Re-rooted module path of an old absolute path inside `crate` (flattened files mount at the crate root)."""
        p = self.owner_path[crate]
        segs = list(segs)
        for key, (owner, value) in self.remap.items():
            if owner == crate and tuple(segs[:len(key)]) == key:
                return list(value) + segs[len(key):]
        if tuple(segs[:len(p)]) == tuple(p):
            tail = segs[len(p):]
            owner_mod = self.tree.files.get(self.owner_file[crate])
            if tail and tail[0] == "component" and owner_mod is not None and owner_mod.name == "component":
                tail = tail[1:]
            return tail
        return segs

    def target(self, file, offset, segs):
        """🎯️ (crate, new segments) of a path written in `file` at `offset`; None when not a monolith path."""
        base = self.module_at(file, offset)
        if segs[0] == "crate":
            absolute = segs[1:]
        elif segs[0] in ("super", "self"):
            b, k = list(base), 0
            while k < len(segs) and segs[k] in ("super", "self"):
                if segs[k] == "super":
                    if not b:
                        return None
                    b.pop()
                k += 1
            absolute = b + segs[k:]
        else:
            absolute = segs
        absolute = self.unalias(absolute)
        node, rest = self.tree.resolve(absolute)
        crate = self.home(node, rest)
        if crate is None:
            return None
        return crate, self.new_path(crate, list(node.path) + rest), absolute

    def relative_ok(self, file, offset, segs, crate, new):
        """♻️ True when the relative path `segs` written in `file` still names `new` inside `crate`."""
        if segs[0] not in ("super", "self") or self.crate_of[file] != crate:
            return False
        base = self.new_path(crate, self.module_at(file, offset))
        k = 0
        b = list(base)
        while k < len(segs) and segs[k] in ("super", "self"):
            if segs[k] == "super":
                if not b:
                    return False
                b.pop()
            k += 1
        return b + segs[k:] == list(new)

    def spell(self, own, crate, new, test_root=False):
        head = "crate" if crate == own and not test_root else self.idents[crate]
        segs = [s for s in new if s != "component"] if head != "crate" else list(new)
        return head, segs

    def rewrite_file(self, file, own, test_root=False):
        raw, text = self.tree.raw[file], self.tree.text[file]
        edits, covered = [], []
        root_names = set(self.tree.root.children) | {a[0][0] for a in self.aliases if len(a[0]) == 1}
        use_re = re.compile(r"(?m)^(?P<attrs>(?:[ \t]*#\[[^\n]*\]\n)*)(?P<indent>[ \t]*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+(?P<body>[^;]*);")
        is_root = file == self.tree.root_file
        for m in use_re.finditer(text):
            body = m.group("body").strip()
            head = re.match(IDENT, body)
            if not head or head.group(0) not in ("crate", "super", "self") and not (is_root and head.group(0) in root_names):
                continue
            rows, changed = [], False
            for segs, alias in leaves([], body):
                glob = segs[-1] == "*"
                path = [s for s in segs if s != "*"]
                selfish = path and path[-1] == "self"
                if selfish:
                    path = path[:-1]
                hit = self.target(file, m.start("body"), path) if path else None
                if hit is None:
                    rows.append(("crate" if path[0] == "crate" else None, path, alias, glob, segs))
                    continue
                crate, new, absolute = hit
                if test_root or crate != own or (path[0] == "crate" and new != path[1:]) or (path[0] in ("super", "self") and not self.relative_ok(file, m.start("body"), path, crate, new)) or (is_root and path[0] not in ("crate", "super", "self") and crate != own):
                    head_name, shown = self.spell(own, crate, new, test_root)
                    rows.append((head_name, shown, alias, glob, segs))
                    changed = True
                else:
                    rows.append((None, path, alias, glob, segs))
                if crate != own and self.crates[crate].get("aggregator") and not test_root:
                    self.problems.append(f"UPWARD {self.rel(file)} -> aggregator via {'::'.join(absolute)}")
            if not changed:
                covered.append((m.start(), m.end()))
                continue
            groups = collections.OrderedDict()
            for head_name, shown, alias, glob, segs in rows:
                if head_name is None:
                    key, path = shown[0], shown[1:]
                else:
                    key, path = head_name, shown
                tail = "*" if glob else None
                groups.setdefault(key, []).append((path, alias, tail))
            lines = []
            for key, items in groups.items():
                parts = []
                for path, alias, tail in items:
                    p = "::".join(path + ([tail] if tail else []))
                    if not p:
                        p = "self"
                    parts.append(p + (f" as {alias}" if alias else ""))
                if parts == ["self"]:
                    lines.append(f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {key};")
                elif parts and all(p.startswith("self as ") for p in parts) and len(parts) == 1:
                    lines.append(f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {key} as {parts[0][8:]};")
                else:
                    tree = parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"
                    lines.append(f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {key}::{tree};")
            edits.append((m.start(), m.end(), "\n".join(lines)))
            covered.append((m.start(), m.end()))
        path_re = re.compile(r"(?<![\w:$])(?P<path>(?:crate|super|self)(?:" + SEP + r"(?:" + IDENT + r"|super))+)")
        candidates = list(path_re.finditer(text))
        if is_root:
            candidates += [m for m in re.finditer(r"(?<![\w:$.])(?P<path>" + IDENT + r"(?:" + SEP + IDENT + r")+)", text) if re.match(IDENT, m.group("path")).group(0) in root_names and re.match(IDENT, m.group("path")).group(0) not in ("crate", "super", "self")]
        decl_spans = [mod.decl_span for mod in self.tree.walk() if mod.decl_file == file and mod.decl_span]
        for m in sorted(candidates, key=lambda m: m.start()):
            if any(a <= m.start() < b for a, b in covered):
                continue
            segs = [x for x in re.split(SEP, m.group("path")) if x]
            hit = self.target(file, m.start(), segs)
            if hit is None:
                continue
            crate, new, absolute = hit
            keep = crate == own and not test_root and ((segs[0] == "crate" and new == segs[1:]) or (segs[0] in ("super", "self") and self.relative_ok(file, m.start(), segs, crate, new)) or (is_root and segs[0] not in ("crate", "super", "self") and new == segs))
            if keep:
                continue
            if crate != own and self.crates[crate].get("aggregator") and not test_root:
                self.problems.append(f"UPWARD {self.rel(file)}:{raw.count(chr(10), 0, m.start()) + 1} -> aggregator via {'::'.join(absolute)}")
            head_name, shown = self.spell(own, crate, new, test_root)
            edits.append((m.start(), m.end(), "::".join([head_name] + shown)))
            covered.append((m.start(), m.end()))
        return sorted(edits)

    def emit_tree(self, crate, root_dir, files, indent=0):
        """🌲️ Mount tree of `files` (file modules of `crate`) below the crate root, in the monolith's `#[path = "."]` style."""
        nodes = collections.OrderedDict()
        for mod in files:
            new = self.new_path(crate, mod.path)
            if not new:
                continue
            cur = nodes
            for seg in new[:-1]:
                cur = cur.setdefault(seg, collections.OrderedDict())
                cur.setdefault("\0mods", [])
            cur.setdefault("\0leaves", []).append((new[-1], mod))
        old_inline = {}
        for mod in self.tree.walk():
            if mod.kind == "inline" and mod.in_file == self.tree.root_file:
                old_inline[tuple(self.new_path(crate, mod.path))] = mod

        def render(node, prefix, depth):
            pad = "    " * depth
            lines = []
            for name, mod in node.get("\0leaves", []):
                rel = os.path.relpath(mod.file, root_dir)
                lines += [pad + a for a in mod.attrs]
                lines.append(f'{pad}#[path = "{rel}"]')
                lines.append(f"{pad}{(mod.vis + ' ') if mod.vis else ''}mod {name};")
                if name == "component" and mod.parent and mod.parent.glob:
                    lines += [pad + a for a in mod.parent.glob[2]]
                    lines.append(f"{pad}pub use component::*;")
            for name, child in node.items():
                if name.startswith("\0"):
                    continue
                inline = old_inline.get(tuple(prefix + [name]))
                attrs = inline.attrs if inline else []
                vis = inline.vis if inline else "pub"
                lines += [pad + a for a in attrs]
                lines.append(f'{pad}#[path = "."]')
                lines.append(f"{pad}{(vis + ' ') if vis else ''}mod {name} {{")
                lines += render(child, prefix + [name], depth + 1)
                lines.append(f"{pad}}}")
            return lines

        return render(nodes, [], indent)

    def plan(self):
        self.rewrites, self.roots, self.removals = {}, {}, collections.defaultdict(list)
        self.dropped_reexports = []
        for file, mod in self.tree.files.items():
            if file == self.tree.root_file:
                continue
            parent_file = mod.decl_file
            if file in self.tests or any(r.search(self.rel(file)) for r in self.drop):
                if parent_file != self.tree.root_file:
                    self.removals[parent_file].append(mod.decl_span)
                continue
            if parent_file != self.tree.root_file and self.crate_of[parent_file] != self.crate_of[file]:
                self.removals[parent_file].append(mod.decl_span)
                self.relocated = getattr(self, "relocated", set())
                self.relocated.add(file)
        relocated = getattr(self, "relocated", set())
        test_descendants = set()
        for file in self.tests:
            for mod in self.tree.walk(self.tree.files[file]):
                if mod.kind == "file":
                    test_descendants.add(mod.file)
        for file, mod in self.tree.files.items():
            own = self.crate_of[file]
            if file in test_descendants:
                root_test = next(t for t in self.tests if mod.path[:len(self.tree.files[t].path)] == self.tree.files[t].path)
                edits = self.rewrite_file(file, self.tests[root_test]["package"], test_root=True)
            else:
                edits = self.rewrite_file(file, own)
            for rfile, rstart, rend, indent, rows in self.reexports:
                if rfile != file:
                    continue
                kept = []
                for segs, alias, path in rows:
                    hit = self.target(file, rstart, [x for x in segs if x != "*"])
                    if hit and hit[0] == own and file not in test_descendants:
                        kept.append((segs, alias))
                if len(kept) == len(rows):
                    continue
                raw = self.tree.raw[file]
                end = rend + (1 if raw[rend:rend + 1] == "\n" else 0)
                rendered = "".join(f"{indent}pub use {'::'.join(segs)}{' as ' + alias if alias else ''};\n" for segs, alias in kept)
                edits = [e for e in edits if not (rstart <= e[0] < end)] + [(rstart, end, rendered)]
                self.dropped_reexports.append((self.rel(file), [("::".join(segs), alias) for segs, alias, _ in rows if (segs, alias) not in kept]))
            for m in re.finditer(r"(?m)^[ \t]*//#region 🚚️Moved\n(?:.*\n)*?[ \t]*//#endregion 🚚️Moved[ \t]*\n", self.tree.raw[file]):
                edits = [e for e in edits if not (m.start() <= e[0] < m.end())] + [(m.start(), m.end(), "")]
            for span in self.removals.get(file, []):
                end = span[1] + (1 if self.tree.raw[file][span[1]:span[1] + 1] == "\n" else 0)
                start = self.tree.raw[file].rfind("\n", 0, span[0]) + 1
                if self.tree.raw[file][start:span[0]].strip():
                    start = span[0]
                edits = [e for e in edits if not (start <= e[0] < end)] + [(start, end, "")]
            self.rewrites[file] = sorted(edits)
        for crate in self.crates:
            files = [m for m in self.tree.files.values() if self.crate_of[m.file] == crate and m.file not in test_descendants and m.file != self.owner_file[crate] and not any(r.search(self.rel(m.file)) for r in self.drop)]
            mounted = [m for m in files if (m.decl_file == self.tree.root_file and not (crate == self.package and m.parent is self.tree.root)) or m.file in relocated]
            self.roots[crate] = (self.owner_file[crate], mounted)

    def root_edits(self):
        """🧱️ Replaces the monolith root's mount trees with the aggregator's remaining tree and drops facades."""
        file = self.tree.root_file
        raw, text = self.tree.raw[file], self.tree.text[file]
        edits = []
        for mod in self.tree.root.children.values():
            if mod.kind == "inline" and mod.decl_span:
                start = raw.rfind("\n", 0, mod.decl_span[0]) + 1
                end = mod.decl_span[1] + (1 if raw[mod.decl_span[1]:mod.decl_span[1] + 1] == "\n" else 0)
                edits.append((start, end, ""))
            elif mod.kind == "file" and self.crate_of[mod.file] != self.package or (mod.kind == "file" and mod.file in self.tests):
                start = raw.rfind("\n", 0, mod.decl_span[0]) + 1
                end = mod.decl_span[1] + (1 if raw[mod.decl_span[1]:mod.decl_span[1] + 1] == "\n" else 0)
                edits.append((start, end, ""))
        for m in re.finditer(r"(?m)^[ \t]*pub\s+use\s+(?P<body>[^;]*);[ \t]*\n?", text):
            if any(a <= m.start() < b for a, b, _ in edits):
                continue
            dests = set()
            for segs, alias in leaves([], m.group("body")):
                path = [s for s in segs if s != "*"]
                hit = self.target(file, m.start("body"), path) if path else None
                if hit:
                    dests.add(hit[0])
            if dests and dests != {self.package}:
                edits.append((m.start(), m.end(), ""))
        return edits

    def apply_text(self, raw, edits):
        out, last = [], 0
        for a, b, rep in sorted(edits, key=lambda e: (e[0], -e[1])):
            if a < last:
                if b <= last:
                    continue
                raise SystemExit(f"overlapping edits at {a} < {last}")
            out.append(raw[last:a])
            out.append(rep)
            last = b
        out.append(raw[last:])
        return "".join(out)

    def crate_root_text(self, crate, rewritten):
        owner, files = self.roots[crate]
        root_dir = os.path.dirname(owner)
        tree = self.emit_tree(crate, root_dir, files)
        if crate == self.package:
            return rewritten.rstrip("\n") + "\n\n" + ("\n".join(tree) + "\n" if tree else "")
        header = list(self.config.get("header", []))
        used = "\n".join(self.tree.text[m.file] for m in self.tree.files.values() if self.crate_of[m.file] == crate)
        for alias, target in self.config.get("aliases", {}).items():
            if re.search(r"(?<![\w.$])(?:::)?" + re.escape(alias) + r"\s*(?:::|\))", used):
                header.append(f"extern crate {target} as {alias};")
        doc = self.crates[crate].get("doc")
        lines = rewritten.split("\n")
        k = 0
        while k < len(lines) and lines[k].startswith("//!"):
            k += 1
        docs = lines[:k] if k else ([doc] if doc else [])
        body = lines[k:]
        while body and not body[0].strip():
            body = body[1:]
        inner = []
        while body and (body[0].startswith("#![") or not body[0].strip()):
            if body[0].strip():
                inner.append(body[0])
            body = body[1:]
        block = docs + [""] + inner + header + [""] + (["//#region 🧩️Modules"] + tree + ["//#endregion 🧩️Modules", ""] if tree else [])
        return "\n".join(block + body)

    def externals(self):
        """🔗️ External consumer files naming the monolith: rewritten text and the family crates they use."""
        roots = self.config.get("consumers", [])
        out = subprocess.run(["git", "grep", "-l", "-e", self.ident, "--", *[r + "/**/*.rs" for r in roots]], cwd=REPO, capture_output=True, text=True).stdout.split("\n")
        found = {}
        pattern = re.compile(r"(?<![\w:])" + re.escape(self.ident) + r"(?P<rest>(?:" + SEP + IDENT + r")*)")
        group = re.compile(r"(?m)^(?P<attrs>(?:[ \t]*#\[[^\n]*\]\n)*)(?P<indent>[ \t]*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+" + re.escape(self.ident) + r"(?P<body>\s*::\s*[^;]*);")
        for relpath in out:
            if not relpath:
                continue
            path = os.path.join(REPO, relpath)
            if path in self.tree.files or not os.path.exists(path):
                continue
            raw = open(path, encoding="utf-8").read()
            text = strip(raw)
            edits, used, covered = [], collections.Counter(), []
            for m in group.finditer(text):
                body = m.group("body").strip()[2:].strip()
                groups = collections.OrderedDict()
                for segs, alias in leaves([], body):
                    glob = segs and segs[-1] == "*"
                    path_segs = [s for s in segs if s != "*"]
                    absolute = self.unalias(path_segs)
                    node, rest = self.tree.resolve(absolute)
                    crate = self.home(node, rest) or self.package
                    new = [s for s in self.new_path(crate, list(node.path) + rest) if s != "component"]
                    used[crate] += 1
                    groups.setdefault(self.idents[crate], []).append("::".join(new + (["*"] if glob else [])) + (f" as {alias}" if alias else ""))
                lines = []
                for head, parts in groups.items():
                    parts = [p if p else "self" for p in parts]
                    tree = parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"
                    if tree == "self":
                        lines.append(f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {head};")
                    elif tree.startswith("self as "):
                        lines.append(f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {head} as {tree[8:]};")
                    else:
                        lines.append(f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {head}::{tree};")
                edits.append((m.start(), m.end(), "\n".join(lines)))
                covered.append((m.start(), m.end()))
            for m in pattern.finditer(text):
                if any(a <= m.start() < b for a, b in covered):
                    continue
                segs = [x for x in re.split(SEP, m.group("rest")) if x]
                if not segs:
                    used[self.package] += 1
                    continue
                absolute = self.unalias(segs)
                node, rest = self.tree.resolve(absolute)
                crate = self.home(node, rest) or self.package
                new = [s for s in self.new_path(crate, list(node.path) + rest) if s != "component"]
                used[crate] += 1
                if crate != self.package or new != segs:
                    edits.append((m.start(), m.end(), "::".join([self.idents[crate]] + new)))
            found[path] = (self.apply_text(raw, edits) if edits else raw, dict(used))
        return found

    def report(self):
        print(f"# monolith {self.rel(self.tree.root_file)}: {len(self.tree.files)} files, {len(self.aliases)} aliases")
        counts = collections.Counter(self.crate_of.values())
        for crate in self.crates:
            print(f"crate {crate}: {counts[crate]} files, owner path {'::'.join(self.owner_path[crate]) or '<root>'}")
        for alias, target, kind in self.aliases[:400]:
            print(f"alias {kind} {'::'.join(alias)} -> {'::'.join(target)}")


def main():
    config_path, mode = sys.argv[1], sys.argv[2]
    cut = Cut(config_path)
    cut.report()
    cut.plan()
    outputs = {}
    for file in cut.tree.files:
        raw = cut.tree.raw[file]
        edits = list(cut.rewrites.get(file, []))
        if file == cut.tree.root_file:
            extra = cut.root_edits()
            edits = [e for e in edits if not any(a <= e[0] < b for a, b, _ in extra)] + extra
        try:
            text = cut.apply_text(raw, edits)
        except SystemExit as error:
            raise SystemExit(f"{error} in {cut.rel(file)}: {[e for e in sorted(edits) if e[0] < 800][:8]}")
        crate = cut.crate_of[file]
        if file == cut.owner_file.get(crate):
            text = cut.crate_root_text(crate, text)
        if text != raw:
            outputs[file] = text
    for crate, spec in cut.crates.items():
        if spec.get("create_root"):
            outputs[cut.owner_file[crate]] = cut.crate_root_text(crate, spec["create_root"] + "\n")
    externals = cut.externals()
    for problem in sorted(set(cut.problems)):
        print(problem)
    usage = {}
    for path, (text, used) in sorted(externals.items()):
        print(f"consumer {json.dumps(used, sort_keys=True)} {os.path.relpath(path, REPO)}")
        usage[os.path.relpath(path, REPO)] = used
        if text != open(path, encoding="utf-8").read():
            outputs[path] = text
    test_files = set()
    for t in cut.tests:
        for mod in cut.tree.walk(cut.tree.files[t]):
            if mod.kind == "file":
                test_files.add(mod.file)
    family = {}
    for crate in cut.crates:
        family[crate] = {"owner": cut.crates[crate]["owner"], "root": cut.rel(cut.owner_file[crate]), "lib": [], "test": [], "targets": []}
    for file, crate in cut.crate_of.items():
        if file in test_files or any(r.search(cut.rel(file)) for r in cut.drop):
            continue
        family[crate]["test" if cut.tree.files[file].test_gated and file != cut.owner_file[crate] else "lib"].append(cut.rel(file))
    for path, t in cut.tests.items():
        family[t["package"]]["targets"].append({"name": t["name"], "file": t["file"], "files": [cut.rel(m.file) for m in cut.tree.walk(cut.tree.files[path]) if m.kind == "file"]})
    cache = os.path.join(REPO, ".🧬semio/🦑️repo/⚡️cache/p7")
    os.makedirs(cache, exist_ok=True)
    stem = os.path.basename(config_path).rsplit(".", 1)[0]
    json.dump({"family": family, "consumers": usage, "dropped_reexports": cut.dropped_reexports}, open(os.path.join(os.path.dirname(os.path.abspath(config_path)), "🗺️plan.json"), "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    print(f"rewritten={len(outputs)} crates={len(cut.crates)} tests={len(cut.tests)}")
    if mode == "apply":
        for path, text in outputs.items():
            os.makedirs(os.path.dirname(path), exist_ok=True)
        for path, text in outputs.items():
            open(path, "w", encoding="utf-8").write(text)
    elif mode == "diff":
        for path, text in outputs.items():
            out_path = os.path.join(cache, "diff", stem, os.path.relpath(path, REPO))
            os.makedirs(os.path.dirname(out_path), exist_ok=True)
            open(out_path, "w", encoding="utf-8").write(text)


if __name__ == "__main__":
    main()
