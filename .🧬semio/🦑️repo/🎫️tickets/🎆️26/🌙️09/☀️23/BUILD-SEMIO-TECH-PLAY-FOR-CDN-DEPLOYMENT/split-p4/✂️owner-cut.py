#!/usr/bin/env python3
"""✂️ Cuts one `#[path]`-tree crate into owner-rooted crates (lane P4).

Every new crate is `<owner>/📦️packages/🦀️rust` with its root at an existing (or new) `<owner>/🦀️.rs`; module paths inside a
crate are the old module paths minus the crate's `module` prefix. Every path that names an item (`crate::`, `super::`, `self::`,
the old root's bare module paths, and external `<old crate>::` paths in consumer files) is resolved to the item's DEFINITION
(re-export facades are followed, never kept) and rendered from the crate that now owns that definition. Mounts are moved: a
file whose mounter stays in another crate is mounted from its own crate's root tree, and the stale declaration is deleted.
Items referenced across crates are widened to `pub`. `plan` prints the crate graph, cycles with sample references, and the
unresolved paths; `apply` writes.

usage: ✂️owner-cut.py <config.json> plan|apply [--edges N]
config: {"root": old crate root (repo-rel), "crate": old crate ident,
         "crates": [{"package", "module": "a::b" (old normalized prefix, "" = root), "root": repo-rel root file,
                     "files": [regex on repo-rel path, ...], "new_root": optional doc line when the root file is created}],
         "aggregator": package that keeps every unclaimed file (the old crate), "header": [lines], "consumers": [repo-rel dirs]}
"""
import collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
_ns = {}
exec(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], _ns)
Crate = _ns["Crate"]
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
SEP = r"\s*::\s*"


def strip(src):
    """🧹️ Blanks comments and literals keeping every offset (length-preserving)."""
    out, i, n = [], 0, len(src)
    blank = lambda text: re.sub(r"[^\n]", " ", text)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            j = n if j < 0 else j
            out.append(" " * (j - i)); i = j
        elif src.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith("/*", j): depth, j = depth + 1, j + 2
                elif src.startswith("*/", j): depth, j = depth - 1, j + 2
                else: j += 1
            out.append(blank(src[i:j])); i = j
        elif c in "rb" and re.match(r'(?:b?r)#*"', src[i:i + 40]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = re.match(r'(b?r)(#*)"', src[i:])
            close = '"' + m.group(2)
            j = src.find(close, i + len(m.group(0)))
            j = n if j < 0 else j + len(close)
            out.append(src[i:i + len(m.group(0))] + blank(src[i + len(m.group(0)):j - len(close)]) + close); i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            j += 1
            keep = re.search(r"#\[path\s*=\s*$", "".join(out[-12:]))
            out.append(src[i:j] if keep else '"' + blank(src[i + 1:j - 1]) + '"'); i = j
        elif c == "'" and (re.match(r"'(\\.|[^\\'])'", src[i:i + 4]) or re.match(r"'\\(x..|u\{[0-9a-fA-F]+\})'", src[i:i + 12])):
            m = re.match(r"'(\\x..|\\u\{[0-9a-fA-F]+\}|\\.|[^\\'])'", src[i:])
            out.append(" " * len(m.group(0))); i += len(m.group(0))
        else:
            out.append(c); i += 1
    result = "".join(out)
    assert len(result) == len(src)
    return result


_ns["strip"] = strip


def depth_map(s):
    """📏️ Brace depth before every offset."""
    depth, out = 0, [0] * (len(s) + 1)
    for i, c in enumerate(s):
        out[i] = depth
        if c == "{": depth += 1
        elif c == "}": depth -= 1
    out[len(s)] = depth
    return out


def inline_mods(s):
    """🧱️ Inline `mod name {` spans: (body_start, body_end, name, decl_start)."""
    spans = []
    for m in re.finditer(r"(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+(" + IDENT + r")\s*\{", s):
        depth, j = 0, m.end() - 1
        while j < len(s):
            if s[j] == "{": depth += 1
            elif s[j] == "}":
                depth -= 1
                if depth == 0: break
            j += 1
        spans.append((m.end(), j, m.group(1), m.start()))
    return spans


def split_top(text):
    parts, depth, cur = [], 0, ""
    for ch in text:
        if ch == "{": depth += 1
        if ch == "}": depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur); cur = ""
        else:
            cur += ch
    if cur.strip(): parts.append(cur)
    return [p.strip() for p in parts if p.strip()]


def use_leaves(prefix, body):
    """🍃️ Flattens one use tree into (segments, alias, glob)."""
    body = body.strip()
    m = re.match(r"^((?:" + IDENT + SEP + r")*)\{(.*)\}$", body, re.S)
    if m:
        pre = prefix + [x for x in re.split(SEP, m.group(1)) if x]
        out = []
        for part in split_top(m.group(2)):
            out += use_leaves(pre, part)
        return out
    alias = re.search(r"\s+as\s+(" + IDENT + r")$", body)
    core = re.sub(r"\s+as\s+" + IDENT + r"$", "", body)
    segs = prefix + [x for x in re.split(SEP, core) if x]
    glob = bool(segs) and segs[-1] == "*"
    if glob: segs = segs[:-1]
    return [(segs, alias.group(1) if alias else None, glob)]


ITEM = re.compile(r"(?m)^[ \t]*(?:#\[[^\n]*\][ \t]*)*(?P<vis>pub(?:\s*\([^)]*\))?\s+)?(?:(?:const|async|unsafe|extern\s+\"C\")\s+)*(?P<kind>fn|struct|enum|trait|type|const|static|mod|union|macro_rules!)\s*(?P<name>" + IDENT + r")")
PUBUSE = re.compile(r"(?m)^[ \t]*(?:#\[[^\n]*\][ \t]*)*(?P<vis>pub(?:\s*\([^)]*\))?)\s+use\s+(?P<body>[^;]*);")
USE_STMT = re.compile(r"(?P<vis>pub(?:\s*\([^)]*\))?\s+)?\buse\s+(?P<body>(?:::\s*)?" + IDENT + r"(?:" + SEP + r"(?:" + IDENT + r"|\*|\{))?[^;]*);")
INLINE_PATH = re.compile(r"(?<![\w:$])(?P<path>(?:crate|super|self)(?:" + SEP + r"(?:" + IDENT + r"|super))+)")


class Loc:
    """📍️ A resolved target: normalized old module, item name (None = the module itself), trailing segments, defining file."""

    def __init__(self, module, item=None, rest=(), file=None, external=None, via_reexport=False, unresolved=False):
        self.module, self.item, self.rest, self.file = tuple(module), item, list(rest), file
        self.external, self.via_reexport, self.unresolved = external, via_reexport, unresolved


class Old:
    """🏛️ The old crate: module tree, definitions and re-exports per normalized module."""

    def __init__(self, root, moved=None):
        self.moved = moved or {}
        self.crate = Crate(os.path.join(REPO, root))
        self.raw = {f: open(f, encoding="utf-8").read() for f in self.crate.files}
        self.text = {f: strip(self.raw[f]) for f in self.crate.files}
        self.norm_files = collections.defaultdict(list)
        for f, mp in self.crate.files.items():
            self.norm_files[self.norm(mp)].append(f)
        self.defs = collections.defaultdict(dict)
        self.globs = collections.defaultdict(list)
        for f, mp in self.crate.files.items():
            self.index(f, mp)
        for mp in list(self.crate.mods):
            n = self.norm(mp)
            if n:
                self.add_def(n[:-1], n[-1], ("mod", self.module_file(n), "mod"))

    def add_def(self, mod, name, entry):
        entries = self.defs[mod].setdefault(name, [])
        if entry[0] == "def" and entry[2] == "mod":
            entry = ("mod", entry[1], "mod")
        if entry not in entries:
            entries.append(entry)

    def entry(self, module, name, final):
        """🔎️ The entry `name` names in `module`: a value/type item wins over a module of the same name on a final segment."""
        entries = self.defs.get(module, {}).get(name, [])
        items = [e for e in entries if e[0] != "mod"]
        mods = [e for e in entries if e[0] == "mod"]
        if final and items:
            return items[0]
        return (mods or items or [None])[0]

    @staticmethod
    def norm(mp):
        return tuple(x for x in mp if x != "component")

    def module_file(self, norm):
        """📄️ The file whose top level IS module `norm` (its component file, else the file declaring it)."""
        for raw, f in self.crate.mods.items():
            if self.norm(raw) == norm and raw and raw[-1] == "component":
                return f
        for raw, f in self.crate.mods.items():
            if self.norm(raw) == norm and self.crate.files.get(f) == raw:
                return f
        for raw, f in self.crate.mods.items():
            if self.norm(raw) == norm:
                return f
        return self.crate.root

    def index(self, f, mp):
        s = self.text[f]
        base = self.norm(mp)
        spans = inline_mods(s)
        depth = depth_map(s)

        def module_at(pos):
            enclosing = sorted((sp for sp in spans if sp[0] <= pos < sp[1]), key=lambda sp: sp[0])
            return enclosing

        macro_blocks = []
        for m in re.finditer(r"(?m)^[ \t]*(?:" + IDENT + SEP + r")*" + IDENT + r"!\s*\{", s):
            if depth[m.start()] == len(module_at(m.start())):
                macro_blocks.append(m.end() - 1)
        for m in ITEM.finditer(s):
            pos = m.start("kind")
            enclosing = module_at(pos)
            inside_macro = any(b < pos and depth[b] == len(enclosing) and depth[pos] == len(enclosing) + 1 and "}" not in s[b + 1:pos].replace("{", "").replace("}", "}") for b in macro_blocks) if depth[pos] == len(enclosing) + 1 else False
            if depth[pos] != len(enclosing) and not inside_macro:
                continue
            mod = base + tuple(sp[2] for sp in enclosing)
            if m.group("kind") == "macro_rules!" and "#[macro_export]" in s[max(0, m.start() - 200):m.start("kind")]:
                self.add_def((), m.group("name"), ("def", f, "macro_export"))
                continue
            self.add_def(mod, m.group("name"), ("def", f, m.group("kind")))
        for m in PUBUSE.finditer(s):
            pos = m.start("vis")
            enclosing = module_at(pos)
            if depth[pos] != len(enclosing):
                continue
            mod = base + tuple(sp[2] for sp in enclosing)
            raw_mod = tuple(mp) + tuple(sp[2] for sp in enclosing)
            for segs, alias, glob in use_leaves([], m.group("body")):
                if not segs:
                    continue
                if glob:
                    if segs != ["component"]:
                        self.globs[mod].append((segs, raw_mod, f))
                    continue
                name = alias or segs[-1]
                if segs[-1] == "self":
                    segs = segs[:-1]
                    name = alias or segs[-1]
                self.add_def(mod, name, ("use", f, segs, raw_mod))

    def walk(self, segs):
        cur, k = (), 0
        mods = self.crate.mods
        while k < len(segs):
            if cur + (segs[k],) in mods:
                cur = cur + (segs[k],)
            elif cur + ("component", segs[k]) in mods:
                cur = cur + ("component", segs[k])
            else:
                break
            k += 1
        return cur, k

    def absolute(self, segs, module):
        """🧭️ `crate::`/`super::`/`self::` segments → absolute normalized segments, from the RAW `module` (with `component` hops)."""
        if not segs:
            return []
        if segs[0] == "crate":
            return [x for x in segs[1:]]
        if segs[0] in ("super", "self"):
            base, k = list(module), 0
            while k < len(segs) and segs[k] in ("super", "self"):
                if segs[k] == "super":
                    if not base:
                        return None
                    base.pop()
                k += 1
            return [x for x in base if x != "component"] + list(segs[k:])
        return None

    def locate(self, segs, seen=0):
        if seen > 12:
            return Loc((), segs[0] if segs else None, segs[1:], unresolved=True)
        raw, k = self.walk(tuple(segs))
        if k and k == len(segs):
            parent = self.norm(raw)[:-1]
            final = self.entry(parent, segs[-1], True)
            if final is not None and final[0] != "mod":
                raw, k = self.walk(tuple(segs[:-1]))
        module = self.norm(raw)
        rest = list(segs[k:])
        if not rest:
            return Loc(module, None, [], self.module_file(module))
        name = rest[0]
        entry = self.entry(module, name, len(rest) == 1)
        if entry is None and name in self.moved.get(module, {}):
            found = self.locate(list(self.moved[module][name]) + rest, seen + 1)
            found.via_reexport = True
            return found
        if entry is None:
            for gsegs, gmod, gfile in self.globs.get(module, []):
                target = self.absolute(gsegs, gmod)
                if target is None:
                    continue
                found = self.locate(target + rest, seen + 1)
                if not found.unresolved:
                    found.via_reexport = True
                    return found
            return Loc(module, name, rest[1:], self.module_file(module), unresolved=True)
        if entry[0] in ("def", "mod"):
            return Loc(module, name, rest[1:], entry[1])
        _, f, usegs, umod = entry
        target = self.absolute(usegs, umod)
        if target is None:
            if usegs and usegs[0] not in ("crate", "super", "self"):
                head = usegs[0]
                if head in self.root_modules():
                    target = list(usegs)
                else:
                    return Loc(module, name, rest[1:], f, external=list(usegs) + rest[1:], via_reexport=True)
            else:
                return Loc(module, name, rest[1:], f, unresolved=True)
        found = self.locate(target + rest[1:], seen + 1)
        found.via_reexport = True
        return found

    def root_modules(self):
        return {raw[0] for raw in self.crate.mods if len(raw) == 1}


class Cut:
    def __init__(self, config):
        self.config = config
        moved = collections.defaultdict(dict)
        for path in config.get("moves", []):
            spec = json.load(open(os.path.join(REPO, path), encoding="utf-8"))
            for move in spec["moves"]:
                for name in move["select"]:
                    if not name.startswith(("re:", "region:")) and not name.startswith(("macro#", "item#")):
                        moved[tuple(x for x in move.get("from_module", "").split("::") if x)][name] = tuple(x for x in move["to_module"].split("::") if x)
        self.old = Old(config["root"], moved)
        self.old_root = self.old.crate.root
        self.crates = {c["package"]: c for c in config["crates"]}
        self.aggregator = config["aggregator"]
        self.crates.setdefault(self.aggregator, {"package": self.aggregator, "module": "", "root": config["root"], "files": []})
        for c in self.crates.values():
            c["mod"] = tuple(x for x in c["module"].split("::") if x)
            c["root_abs"] = os.path.normpath(os.path.join(REPO, c["root"]))
            c["rules"] = [re.compile(r) for r in c["files"]]
        self.ident = {name: name.replace("-", "_") for name in self.crates}
        self.ident[self.aggregator] = config["crate"]
        self.assign = {}
        for f in self.old.crate.files:
            rel = os.path.relpath(f, REPO)
            owner = next((name for name, c in self.crates.items() if c["root_abs"] == f), None)
            if owner is None:
                owner = next((name for name, c in self.crates.items() if any(r.search(rel) for r in c["rules"])), self.aggregator)
            self.assign[f] = owner
        self.refs = collections.defaultdict(collections.Counter)
        self.test_refs = collections.defaultdict(collections.Counter)
        self.samples = collections.defaultdict(list)
        self.unresolved = []
        self.need_pub = collections.defaultdict(set)
        self.facades = []
        self.generated_names = set()
        self.keep_root = set(config.get("keep_root", ["dsl", "protocol", "store", "vcs", "value_derive"]))
        self.facade_files = {os.path.normpath(os.path.join(REPO, p)) for p in config.get("facade_files", [])} | {self.old_root}
        self.facade_import_files = {os.path.normpath(os.path.join(REPO, p)) for p in config.get("facade_import_files", [])}
        self.errors = []

    def crate_of(self, loc, self_crate=None):
        if loc.file is None:
            return self.aggregator
        if loc.item is None and tuple(loc.module) == () and self_crate is not None:
            return self_crate
        if loc.item is None:
            own = [f for f, raw in self.old.crate.files.items() if raw and raw[-1] == "component" and self.old.norm(raw) == tuple(loc.module)]
            if own:
                return self.assign.get(own[0], self.aggregator)
            if self.old.crate.files.get(loc.file) != tuple(self.old.crate.mods.get(tuple(loc.module), ())) or loc.file == self.old_root:
                votes = collections.Counter(self.assign[f] for f, raw in self.old.crate.files.items() if self.old.norm(raw)[:len(loc.module)] == tuple(loc.module) and len(self.old.norm(raw)) > len(loc.module))
                if votes:
                    return votes.most_common(1)[0][0]
        return self.assign.get(loc.file, self.aggregator)

    def root_base(self, crate):
        """🌿️ The raw module the crate root file IS (its `component` hop dropped), or None for a created root."""
        root = self.crates[crate]["root_abs"]
        if root not in self.old.crate.files:
            return None
        raw = tuple(self.old.crate.files[root])
        return raw[:-1] if raw and raw[-1] == "component" else raw

    def newmod(self, module, crate):
        prefix = self.crates[crate]["mod"]
        base = self.root_base(crate)
        if base is not None:
            norm_base = self.old.norm(base)
            if tuple(module[:len(norm_base)]) == norm_base and len(norm_base) >= len(prefix):
                return tuple(module[len(norm_base):])
        if tuple(module[:len(prefix)]) != prefix:
            self.errors.append(f"module {'::'.join(module)} outside {crate} prefix {'::'.join(prefix)}")
            return tuple(module)
        return tuple(module[len(prefix):])

    def render(self, loc, self_crate, src_file, line):
        if loc.external is not None:
            return "::".join(loc.external), None
        if loc.item is None and tuple(loc.module) == () and self_crate in self.crates:
            return "crate", self_crate
        target = self.crate_of(loc, self_crate if self_crate in self.crates else None)
        if loc.item is not None and loc.file is not None and loc.file in self.old.crate.files:
            defining_module = loc.module
        else:
            defining_module = loc.module
        is_macro = loc.item is not None and any(e[0] == "def" and e[2] == "macro_export" for e in self.old.defs.get((), {}).get(loc.item, [])) and loc.module == ()
        segs = ([] if is_macro else list(self.newmod(defining_module, target))) + ([loc.item] if loc.item is not None else []) + list(loc.rest)
        head = "crate" if target == self_crate else self.ident[target]
        if target != self_crate:
            is_test = src_file in self.old.crate.test_files or "🧪️tests" in src_file
            (self.test_refs if is_test else self.refs)[self_crate][target] += 1
            if len(self.samples[(self_crate, target)]) < 400:
                self.samples[(self_crate, target)].append((os.path.relpath(src_file, REPO), line, "::".join(list(defining_module) + ([loc.item] if loc.item else [])), "T" if (src_file in self.old.crate.test_files or "🧪️tests" in src_file) else "-"))
            if loc.item is not None and loc.file is not None:
                self.need_pub[loc.file].add((defining_module, loc.item))
        return head + ("::" + "::".join(segs) if segs else ""), target

    def file_module(self, f, offset, spans):
        mod = list(self.old.crate.files[f])
        for a, b, name, _ in sorted(spans, key=lambda sp: sp[0]):
            if a <= offset < b:
                mod.append(name)
        return mod

    def new_raw(self, f, crate):
        """🧬️ The file's raw module path inside its new crate (the root file is `()`)."""
        raw = tuple(self.old.crate.files[f])
        root = self.crates[crate]["root_abs"]
        root_raw = tuple(self.old.crate.files.get(root, ()))
        if f == root:
            return ()
        base = self.root_base(crate)
        if base is not None and raw[:len(base)] == base and len(base) >= len(self.crates[crate]["mod"]):
            rest = raw[len(base):]
            return rest[1:] if rest[:1] == ("component",) else rest
        prefix = self.crates[crate]["mod"]
        if raw[:len(prefix)] == prefix:
            return raw[len(prefix):]
        self.errors.append(f"file {os.path.relpath(f, REPO)} raw {'::'.join(raw)} outside {crate}")
        return raw

    def new_depth(self, f, crate, offset, spans):
        """📐️ Depth of the enclosing module inside its new crate (how many `super` a relative path may climb)."""
        return len(self.new_raw(f, crate)) + sum(1 for a, b, _, _ in spans if a <= offset < b)

    def rewrite_file(self, f, crate):
        raw, s = self.old.raw[f], self.old.text[f]
        spans = inline_mods(s)
        edits, covered = [], []
        line_of = lambda off: raw.count("\n", 0, off) + 1
        is_old_root = f == self.old_root
        root_mods = self.old.root_modules()

        def resolve(segs, offset):
            module = self.file_module(f, offset, spans)
            if segs[0] in ("crate", "super", "self"):
                absolute = self.old.absolute(segs, module)
                if absolute is None:
                    return None
                if absolute and absolute[0] in self.keep_root:
                    return None
                loc = self.old.locate(absolute)
                return self.generated(loc)
            if is_old_root and segs[0] in root_mods:
                return self.old.locate(segs)
            return None

        def keep_relative(segs, loc, offset):
            if segs[0] not in ("super", "self") or loc.via_reexport or loc.external is not None:
                return False
            if self.crate_of(loc) != crate:
                return False
            supers = sum(1 for x in segs if x == "super")
            return supers <= self.new_depth(f, crate, offset, spans)

        dead = self.dead_aliases(f, s, spans)
        for m in USE_STMT.finditer(s):
            body = m.group("body")
            head = re.match(r"(?:::\s*)?(" + IDENT + r")", body).group(1)
            if head not in ("crate", "super", "self") and not (is_old_root and head in root_mods):
                continue
            leaves = use_leaves([], body)
            if not m.group("vis") and dead:
                kept = [leaf for leaf in leaves if not (leaf[0] and not leaf[2] and (leaf[1] or leaf[0][-1]) in dead)]
                if len(kept) != len(leaves):
                    leaves = kept
                    if not leaves:
                        start = raw.rfind("\n", 0, m.start()) + 1
                        end = m.end() + (1 if raw[m.end():m.end() + 1] == "\n" else 0)
                        edits.append((start, end, ""))
                        covered.append((m.start(), m.end()))
                        continue
                    body = "{" + ", ".join("::".join(sg) + (f" as {al}" if al else "") for sg, al, gl in leaves) + "}"
                    forced = True
                else:
                    forced = False
            else:
                forced = False
            if m.group("vis") and m.group("vis").strip().startswith("pub") and self.is_facade(f, crate, leaves, m.start("body")):
                start = raw.rfind("\n", 0, m.start()) + 1
                end = m.end() + (1 if raw[m.end():m.end() + 1] == "\n" else 0)
                doc = re.search(r"(?:[ \t]*///[^\n]*\n)+$", raw[:start])
                edits.append((doc.start() if doc else start, end, ""))
                covered.append((m.start(), m.end()))
                self.facades.append((os.path.relpath(f, REPO), line_of(m.start()), body.strip()[:80]))
                continue
            rendered, changed = [], forced
            for segs, alias, glob in leaves:
                if not segs:
                    rendered.append(None); continue
                is_self = segs[-1] == "self"
                core = segs[:-1] if is_self else segs
                loc = resolve(core, m.start("body"))
                if loc is None or loc.unresolved:
                    if loc is not None and loc.unresolved:
                        self.unresolved.append((os.path.relpath(f, REPO), line_of(m.start()), "::".join(core)))
                    rendered.append(("::".join(core), alias, glob, core[-1] if core else None)); continue
                if keep_relative(core, loc, m.start("body")) or (core[0] == "crate" and False):
                    rendered.append(("::".join(core), alias, glob, core[-1])); continue
                path, _ = self.render(loc, crate, f, line_of(m.start()))
                changed = True
                rendered.append((path, alias, glob, core[-1]))
            covered.append((m.start(), m.end()))
            if not changed:
                continue
            own_module = "crate" + "".join("::" + x for x in self.old.norm(self.new_raw(f, crate)) + tuple(sp[2] for sp in spans if sp[0] <= m.start() < sp[1]))
            parts = []
            for item in rendered:
                if item is None:
                    continue
                path, alias, glob, original = item
                if not glob and not alias and path.rpartition("::")[0] == own_module and not m.group("vis"):
                    continue
                if glob:
                    parts.append(path + "::*")
                elif alias:
                    parts.append(f"{path} as {alias}")
                elif original is not None and path.split("::")[-1] != original and original not in ("crate", "super", "self"):
                    parts.append(f"{path} as {original}")
                else:
                    parts.append(path)
            parts = list(dict.fromkeys(parts))
            if not parts:
                start = raw.rfind("\n", 0, m.start()) + 1
                end = m.end() + (1 if raw[m.end():m.end() + 1] == "\n" else 0)
                edits.append((start, end, ""))
                continue
            if m.group("vis") and f in self.facade_import_files and all(item is None or item[0].split("::")[0] != "crate" for item in rendered):
                edits.append((m.start("vis"), m.end("vis"), ""))
            if len(parts) == 1:
                tree = parts[0]
            else:
                groups = collections.OrderedDict()
                for p in parts:
                    prefix, _, last = p.rpartition("::") if "::" in p.split(" as ")[0] else ("", "", p)
                    if " as " in p:
                        base, alias_part = p.split(" as ")
                        prefix, _, last = base.rpartition("::")
                        last = f"{last} as {alias_part}"
                    groups.setdefault(prefix, []).append(last)
                trees = [(f"{prefix}::{{{', '.join(items)}}}" if len(items) > 1 else f"{prefix}::{items[0]}") if prefix else items[0] for prefix, items in groups.items()]
                tree = trees[0] if len(trees) == 1 else "{" + ", ".join(trees) + "}"
            edits.append((m.start("body"), m.end("body"), tree))
        for m in INLINE_PATH.finditer(s):
            if any(a <= m.start() < b for a, b in covered):
                continue
            segs = [x for x in re.split(SEP, m.group("path")) if x]
            loc = resolve(segs, m.start())
            if loc is None:
                continue
            if loc.unresolved:
                self.unresolved.append((os.path.relpath(f, REPO), line_of(m.start()), "::".join(segs)))
                continue
            if keep_relative(segs, loc, m.start()):
                continue
            path, _ = self.render(loc, crate, f, line_of(m.start()))
            if path != m.group("path"):
                edits.append((m.start(), m.end(), path))
        aliases = {}
        for m in USE_STMT.finditer(s):
            body = m.group("body")
            head = re.match(r"(?:::\s*)?(" + IDENT + r")", body).group(1)
            if head not in ("crate", "super", "self"):
                continue
            for segs, alias, glob in use_leaves([], body):
                if glob or not segs:
                    continue
                core = segs[:-1] if segs[-1] == "self" else segs
                absolute = self.old.absolute(core, self.file_module(f, m.start(), spans))
                if absolute is None:
                    continue
                walked, k = self.old.walk(tuple(absolute))
                if k == len(absolute) and absolute:
                    aliases[alias or core[-1]] = absolute
        if aliases:
            for m in re.finditer(r"(?<![\w:$])(?P<path>(?:" + "|".join(re.escape(a) for a in sorted(aliases)) + r")(?:" + SEP + IDENT + r")+)", s):
                if any(a <= m.start() < b for a, b in covered) or any(a <= m.start() < b for a, b, _ in edits):
                    continue
                segs = [x for x in re.split(SEP, m.group("path")) if x]
                base = aliases[segs[0]]
                loc = self.old.locate(list(base) + segs[1:])
                if loc.unresolved or loc.item is None:
                    continue
                module_loc = self.old.locate(list(base))
                if not loc.via_reexport and self.crate_of(loc) == self.crate_of(module_loc):
                    continue
                path, _ = self.render(loc, crate, f, line_of(m.start()))
                edits.append((m.start(), m.end(), path))
        if is_old_root:
            for m in re.finditer(r"(?<![\w:$])(?P<path>(?:" + "|".join(sorted(root_mods)) + r")(?:" + SEP + IDENT + r")+)", s):
                if any(a <= m.start() < b for a, b in covered) or any(a <= m.start() < b for a, b in edits_spans(edits)):
                    continue
                if self.in_mount_tree(m.start()):
                    continue
                segs = [x for x in re.split(SEP, m.group("path")) if x]
                loc = self.old.locate(segs)
                if loc.unresolved:
                    continue
                path, target = self.render(loc, crate, f, line_of(m.start()))
                if target != crate:
                    edits.append((m.start(), m.end(), path))
        return sorted(edits)

    def generated(self, loc):
        """🏭️ A name no source declares but whose module file exists is macro-generated there."""
        if loc.unresolved and loc.item is not None and loc.file is not None and loc.module != ():
            self.generated_names.add(("::".join(loc.module), loc.item))
            loc.unresolved = False
        return loc

    def dead_aliases(self, f, s, spans):
        """💀️ Module aliases imported by `use` whose every use resolves to an item now owned by another crate than the module."""
        aliases = {}
        for m in USE_STMT.finditer(s):
            body = m.group("body")
            head = re.match(r"(?:::\s*)?(" + IDENT + r")", body).group(1)
            if head not in ("crate", "super", "self") or m.group("vis"):
                continue
            for segs, alias, glob in use_leaves([], body):
                if glob or not segs:
                    continue
                core = segs[:-1] if segs[-1] == "self" else segs
                absolute = self.old.absolute(core, self.file_module(f, m.start(), spans))
                if absolute is None:
                    continue
                walked, k = self.old.walk(tuple(absolute))
                if k == len(absolute) and absolute:
                    aliases[alias or core[-1]] = absolute
        dead = set()
        for name, base in aliases.items():
            uses = [u for u in re.finditer(r"(?<![\w:$.])" + re.escape(name) + r"((?:" + SEP + IDENT + r")+)", s) if not re.match(r"\s*use\b", s[s.rfind("\n", 0, u.start()) + 1:u.start()]) and "use " not in s[s.rfind("\n", 0, u.start()) + 1:u.start()]]
            if not uses:
                continue
            module_crate = self.crate_of(self.old.locate(list(base)))
            moved_all = True
            for u in uses:
                segs = [x for x in re.split(SEP, u.group(1)) if x]
                loc = self.old.locate(list(base) + segs)
                if loc.unresolved or loc.item is None or (not loc.via_reexport and self.crate_of(loc) == module_crate):
                    moved_all = False
                    break
            if moved_all:
                dead.add(name)
        return dead

    def is_facade(self, f, crate, leaves, offset):
        """🪞️ A `pub use` whose every leaf names an item another crate now owns (deleted, never kept)."""
        if f not in self.facade_files:
            return False
        module = list(self.old.crate.files[f])
        for segs, alias, glob in leaves:
            if glob or not segs or segs[0] not in ("crate", "super", "self") and not (f == self.old_root and segs[0] in self.old.root_modules()):
                return False
            absolute = self.old.absolute(segs, module) if segs[0] in ("crate", "super", "self") else list(segs)
            if absolute is None:
                return False
            loc = self.old.locate(absolute)
            if loc.unresolved or self.crate_of(loc) == crate:
                return False
        return True

    def in_mount_tree(self, offset):
        return any(a <= offset < b for a, b in self.root_tree_spans)

    def consumer_edits(self, path, raw, s):
        old_ident = self.config["crate"]
        edits, covered = [], []
        line_of = lambda off: raw.count("\n", 0, off) + 1
        for m in re.finditer(r"(?P<vis>pub(?:\s*\([^)]*\))?\s+)?\buse\s+(?P<body>" + re.escape(old_ident) + r"\b[^;]*);", s):
            leaves = use_leaves([], m.group("body"))
            parts = []
            for segs, alias, glob in leaves:
                core = segs[1:]
                is_self = core and core[-1] == "self"
                if is_self:
                    core = core[:-1]
                if not core:
                    parts.append(old_ident + (f" as {alias}" if alias else "") if not glob else old_ident + "::*")
                    continue
                loc = self.generated(self.old.locate(core))
                if loc.unresolved:
                    self.unresolved.append((os.path.relpath(path, REPO), line_of(m.start()), "::".join(segs)))
                    rendered = "::".join(segs)
                else:
                    rendered, _ = self.render(loc, "<consumer>", path, line_of(m.start()))
                if glob:
                    parts.append(rendered + "::*")
                elif alias:
                    parts.append(f"{rendered} as {alias}")
                elif rendered.split("::")[-1] != core[-1]:
                    parts.append(f"{rendered} as {core[-1]}")
                else:
                    parts.append(rendered)
            parts = list(dict.fromkeys(parts))
            tree = parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"
            edits.append((m.start("body"), m.end("body"), tree))
            covered.append((m.start(), m.end()))
        for m in re.finditer(r"(?<![\w:$])" + re.escape(old_ident) + r"(?P<rest>(?:" + SEP + IDENT + r")+)", s):
            if any(a <= m.start() < b for a, b in covered):
                continue
            segs = [x for x in re.split(SEP, m.group("rest")) if x]
            loc = self.generated(self.old.locate(segs))
            if loc.unresolved:
                self.unresolved.append((os.path.relpath(path, REPO), line_of(m.start()), "::".join(segs)))
                continue
            rendered, _ = self.render(loc, "<consumer>", path, line_of(m.start()))
            edits.append((m.start(), m.end(), rendered))
        return sorted(edits)

    def mounts(self):
        """🗺️ child file → (mounter file, declaration span in mounter raw text or None for the old root tree)."""
        out = {}
        pattern = re.compile(r'(?P<decl>(?:#\[[^\]\n]*\]\s*)*#\[path\s*=\s*"(?P<path>[^"]+)"\]\s*(?:#\[[^\]\n]*\]\s*)*(?P<vis>pub(?:\s*\([^)]*\))?\s+)?mod\s+(?P<name>' + IDENT + r')\s*;)')
        for f in self.old.crate.files:
            if f == self.old_root:
                continue
            s = self.old.text[f]
            for m in pattern.finditer(s):
                target = os.path.normpath(os.path.join(os.path.dirname(f), m.group("path")))
                if target in self.old.crate.files:
                    start = m.start("decl")
                    end = m.end("decl")
                    if self.old.raw[f][end:end + 1] == "\n":
                        end += 1
                    out[target] = (f, (start, end))
        return out


def edits_spans(edits):
    return [(a, b) for a, b, _ in edits]


def apply_edits(raw, edits):
    out, last = [], 0
    for a, b, rep in sorted(edits):
        if a < last:
            raise SystemExit(f"overlapping edits at {a}")
        out.append(raw[last:a]); out.append(rep); last = b
    out.append(raw[last:])
    return "".join(out)


TOKENS = re.compile(r'#\[path\s*=\s*"(?P<path>[^"]+)"\]|#\[cfg\((?P<cfg>[^\]]*)\)\]|(?P<vis>pub(?:\s*\([^)]*\))?\s+)?mod\s+(?P<mod>' + IDENT + r')\s*(?P<kind>[;{])|pub\s+use\s+component\s*::\s*\*\s*;|\}')


def root_tree(root_file, raw):
    """🌳️ The old root's `#[path = "."]` inline trees: [(span, node)], node = ("node", name, vis, cfg, children) | ("mount", name, vis, cfg, file) | ("glob", cfg)."""
    text = strip(raw)
    base = os.path.dirname(root_file)
    trees = []
    for m in re.finditer(r"#\[path\s*=\s*\"\.\"\]\s*(?:#\[[^\]]*\]\s*)*(?P<vis>pub(?:\s*\([^)]*\))?\s+)?mod\s+(" + IDENT + r")\s*\{", text):
        if any(a <= m.start() < b for (a, b), _ in trees):
            continue
        depth, j = 0, m.end() - 1
        while True:
            if text[j] == "{": depth += 1
            elif text[j] == "}":
                depth -= 1
                if depth == 0: break
            j += 1
        start = text.rfind("\n", 0, m.start()) + 1
        pre = text[start:m.start()]
        cfgm = re.search(r"#\[cfg\(([^\]]*)\)\]", text[max(0, start - 200):m.start()])
        trees.append(((start, j + 1), ("node", m.group(2), (m.group("vis") or "").strip() or "pub", None, parse_tree(text, m.end(), j, base))))
    return trees


def parse_tree(text, start, end, directory):
    children, pending, cfg = [], None, None
    pos = start
    while pos < end:
        t = TOKENS.search(text, pos, end)
        if not t:
            break
        pos = t.end()
        if t.group("path") is not None:
            pending = t.group("path")
        elif t.group("cfg") is not None:
            cfg = t.group("cfg")
        elif t.group("mod"):
            name, vis = t.group("mod"), (t.group("vis") or "").strip()
            if t.group("kind") == "{":
                depth, j = 0, t.end() - 1
                while True:
                    if text[j] == "{": depth += 1
                    elif text[j] == "}":
                        depth -= 1
                        if depth == 0: break
                    j += 1
                sub = os.path.normpath(os.path.join(directory, pending)) if pending else os.path.join(directory, name)
                children.append(("node", name, vis, cfg, parse_tree(text, t.end(), j, sub)))
                pos = j + 1
            else:
                children.append(("mount", name, vis, cfg, os.path.normpath(os.path.join(directory, pending)) if pending else os.path.join(directory, name + ".rs")))
            pending, cfg = None, None
        elif t.group(0).startswith("pub"):
            children.append(("glob", cfg)); cfg = None
    return children


def emit_tree(children, keep, root_dir, indent):
    """🌱️ Re-emits the parts of an inline tree that lead to a kept file (paths relative to `root_dir`)."""
    pad = "    " * indent
    lines, previous = [], None
    for child in children:
        if child[0] == "glob":
            if previous is not None and previous[0] == "mount" and previous[1] == "component" and previous[4] in keep:
                if child[1]:
                    lines.append(f"{pad}#[cfg({child[1]})]")
                lines.append(f"{pad}pub use component::*;")
            continue
        previous = child
        if not leads(child, keep):
            continue
        if child[0] == "mount":
            _, name, vis, cfg, path = child
            if cfg:
                lines.append(f"{pad}#[cfg({cfg})]")
            lines.append(f'{pad}#[path = "{os.path.relpath(path, root_dir)}"]')
            lines.append(f"{pad}{(vis + ' ') if vis else ''}mod {name};")
        else:
            _, name, vis, cfg, sub = child
            if cfg:
                lines.append(f"{pad}#[cfg({cfg})]")
            lines.append(f'{pad}#[path = "."]')
            lines.append(f"{pad}{(vis + ' ') if vis else ''}mod {name} {{")
            lines += emit_tree(sub, keep, root_dir, indent + 1)
            lines.append(f"{pad}}}")
    return lines


def mount_info(trees, mounts, raw_texts, f):
    """🏷️ (cfg, visibility) of the old declaration that mounted `f`."""
    def walk(children):
        for child in children:
            if child[0] == "mount" and child[4] == f:
                return child[3], child[2]
            if child[0] == "node":
                found = walk(child[4])
                if found:
                    return found
        return None
    found = walk([node for _, node in trees])
    if found:
        return found
    if f in mounts:
        mounter, (a, b) = mounts[f]
        decl = raw_texts[mounter][a:b]
        cfg = re.search(r"#\[cfg\(([^\]]*)\)\]", decl)
        vis = re.search(r"(pub(?:\s*\([^)]*\))?)\s+mod\s", decl)
        return (cfg.group(1) if cfg else None), (vis.group(1) if vis else "")
    return None, "pub"


def emit_paths(cut, name, files, trees, mounts, root_dir):
    """🌳️ Mounts `files` from the crate root at their new module paths: inline `#[path = "."]` nodes for every
    intermediate module, a `component` file as `mod component; pub use component::*;` inside its node."""
    tree = {"nodes": {}, "mounts": []}
    for f in files:
        raw = list(cut.new_raw(f, name))
        component = bool(raw) and raw[-1] == "component"
        path = [x for x in (raw[:-1] if component else raw) if x != "component"]
        if not path and not component:
            cut.errors.append(f"unmountable root-level file {os.path.relpath(f, REPO)} in {name}")
            continue
        node = tree
        for seg in (path if component else path[:-1]):
            node = node["nodes"].setdefault(seg, {"nodes": {}, "mounts": []})
        cfg, vis = mount_info(trees, mounts, cut.old.raw, f)
        node["mounts"].append(("component" if component else path[-1], f, cfg, vis, component))
    def emit(node, indent):
        pad = "    " * indent
        lines = []
        for mname, f, cfg, vis, component in sorted(node["mounts"], key=lambda m: (not m[4], m[0])):
            if cfg:
                lines.append(f"{pad}#[cfg({cfg})]")
            lines.append(f'{pad}#[path = "{os.path.relpath(f, root_dir)}"]')
            if component:
                lines.append(f"{pad}mod component;")
                if cfg:
                    lines.append(f"{pad}#[cfg({cfg})]")
                lines.append(f"{pad}pub use component::*;")
            else:
                lines.append(f"{pad}{(vis + ' ') if vis else ''}mod {mname};")
        for seg, child in sorted(node["nodes"].items()):
            lines.append(f'{pad}#[path = "."]')
            lines.append(f"{pad}pub mod {seg} {{")
            lines += emit(child, indent + 1)
            lines.append(f"{pad}}}")
        return lines
    return emit(tree, 0)


def emit_spliced(children, splice, keep, root_dir, root_file):
    """🪢️ Emits `children`, flattening the nodes on `splice` (the path from the crate prefix to the root file's own module):
    their children land at the current level, and the root file's own `component` mount and glob are dropped."""
    if not splice:
        return emit_tree([c for c in children if not (c[0] == "mount" and c[4] == root_file)], keep, root_dir, 0)
    lines = []
    for child in children:
        if child[0] == "node" and child[1] == splice[0]:
            lines += emit_spliced(child[4], splice[1:], keep, root_dir, root_file)
        elif child[0] != "glob" and leads(child, keep):
            lines += emit_tree([child], keep, root_dir, 0)
    return lines


def leads(node, keep):
    if node[0] == "mount":
        return node[4] in keep
    if node[0] == "node":
        return any(leads(child, keep) for child in node[4])
    return False


def subtree(trees, prefix):
    """🎯️ The children of the inline node at raw module path `prefix` (with `component` hops)."""
    children = [node for _, node in trees]
    for seg in prefix:
        nxt = next((c for c in children if c[0] == "node" and c[1] == seg), None)
        if nxt is None:
            return None
        children = nxt[4]
    return children


def main():
    config_path, mode = sys.argv[1], sys.argv[2]
    show = int(sys.argv[sys.argv.index("--edges") + 1]) if "--edges" in sys.argv else 6
    config = json.load(open(config_path, encoding="utf-8"))
    cut = Cut(config)
    old = cut.old
    root_raw = old.raw[cut.old_root]
    trees = root_tree(cut.old_root, root_raw)
    cut.root_tree_spans = [span for span, _ in trees]
    mounts = cut.mounts()
    by_crate = collections.defaultdict(list)
    for f, c in cut.assign.items():
        by_crate[c].append(f)
    rewrites = {}
    for f, c in cut.assign.items():
        edits = cut.rewrite_file(f, c)
        if edits:
            rewrites[f] = edits
    consumers = {}
    for root in config.get("consumers", []):
        for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, root)):
            dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated", ".git")]
            for name in filenames:
                if not name.endswith(".rs"):
                    continue
                path = os.path.normpath(os.path.join(dirpath, name))
                if path in old.crate.files:
                    continue
                raw = open(path, encoding="utf-8").read()
                if config["crate"] not in raw:
                    continue
                edits = cut.consumer_edits(path, raw, strip(raw))
                if edits:
                    consumers[path] = (raw, edits)
    graph = cut.refs
    print("## crates")
    for name in sorted(by_crate):
        files = by_crate[name]
        lines = sum(old.raw[f].count("\n") + 1 for f in files)
        print(f"{name}: {len(files)} files, {lines} lines (all, incl tests)")
    print("\n## crate edges (production)")
    for a in sorted(graph):
        for b, n in sorted(graph[a].items()):
            print(f"{a} -> {b}: {n}")
    print("\n## test-only edges")
    for a in sorted(cut.test_refs):
        for b, n in sorted(cut.test_refs[a].items()):
            if b not in graph.get(a, {}):
                print(f"{a} -> {b}: {n}")
    order, state, cycles = [], {}, []

    def visit(node, stack):
        state[node] = 1
        for nxt in graph.get(node, {}):
            if nxt == "<consumer>":
                continue
            if state.get(nxt) == 1:
                cycles.append(stack[stack.index(nxt):] + [nxt])
            elif not state.get(nxt):
                visit(nxt, stack + [nxt])
        state[node] = 2

    for node in sorted(by_crate):
        if not state.get(node):
            visit(node, [node])
    print("\n## cycles")
    for cyc in cycles:
        print(" -> ".join(cyc))
        for a, b in zip(cyc, cyc[1:]):
            seen = collections.Counter(x[2] for x in cut.samples[(a, b)] if x[3] == "-")
            print(f"   {a} -> {b}: " + ", ".join(f"{k}×{v}" for k, v in seen.most_common(show)))
    if "--dump" in sys.argv:
        json.dump({f"{a}>{b}": rows for (a, b), rows in cut.samples.items()}, open(sys.argv[sys.argv.index("--dump") + 1], "w"), ensure_ascii=False, indent=0)
    if "--why" in sys.argv:
        a, b = sys.argv[sys.argv.index("--why") + 1].split(">")
        for row in cut.samples[(a, b)]:
            print("   ", row[3], row[0].split("✳️any/")[-1], row[1], row[2])
    print(f"\n## unresolved ({len(cut.unresolved)})")
    for row in sorted(set(cut.unresolved))[:60]:
        print("  ", row)
    print(f"\n## generated names ({len(cut.generated_names)})")
    for row in sorted(cut.generated_names)[:40]:
        print("  ", row)
    print(f"\n## facades deleted ({len(cut.facades)})")
    for row in cut.facades[:40]:
        print("  ", row)
    print(f"\n## errors ({len(set(cut.errors))})")
    for e in sorted(set(cut.errors))[:40]:
        print("  ", e)
    if mode != "apply":
        return
    if cycles:
        raise SystemExit("refused: cycles")
    write_all(cut, config, trees, mounts, by_crate, rewrites, consumers)


def widen(raw, s, names):
    """🔓️ Makes the module-level declarations of `names` `pub`."""
    edits = []
    for m in ITEM.finditer(s):
        if m.group("name") not in names:
            continue
        vis = m.group("vis")
        if vis and vis.strip() == "pub":
            continue
        if vis:
            edits.append((m.start("vis"), m.end("vis"), "pub "))
        else:
            edits.append((m.start("kind"), m.start("kind"), "pub "))
    return edits


def write_all(cut, config, trees, mounts, by_crate, rewrites, consumers):
    old = cut.old
    extra = collections.defaultdict(list)
    for f, (mounter, span) in mounts.items():
        if cut.assign[f] != cut.assign[mounter] or f == cut.crates[cut.assign[f]]["root_abs"]:
            extra[mounter].append((span[0], span[1], ""))
    for f, items in cut.need_pub.items():
        names = {item for _, item in items}
        extra[f] += widen(old.raw[f], old.text[f], names)
    for f, c in cut.assign.items():
        if c == cut.aggregator:
            continue
        for m in re.finditer(r"\bpub\(crate\)\s", old.raw[f]):
            if old.text[f][m.start():m.start() + 10] == old.raw[f][m.start():m.start() + 10]:
                extra[f].append((m.start(), m.end(), "pub "))
    texts = {}
    for f in old.crate.files:
        edits = rewrites.get(f, []) + extra.get(f, [])
        if f == cut.old_root:
            edits = [e for e in edits if not any(a <= e[0] < b for a, b in cut.root_tree_spans)]
            for (a, b), node in trees:
                keep = {x for x in by_crate[cut.aggregator] if x not in mounts or cut.assign[mounts[x][0]] != cut.aggregator}
                kept = "\n".join(emit_tree([node], keep, os.path.dirname(cut.old_root), 0))
                edits.append((a, b, kept))
        dedup = []
        for e in sorted(edits, key=lambda e: (e[0], -(e[1] - e[0]))):
            if dedup and e[0] < dedup[-1][1]:
                prev = dedup[-1]
                if e[1] <= prev[1] and e[0] >= prev[0]:
                    if prev[2] == "" or e[0] == e[1] == prev[0]:
                        continue
                    if e[0] == e[1]:
                        continue
                    print(f"OVERLAP {os.path.relpath(f, REPO)}: {prev[:2]} {prev[2][:60]!r} vs {e[:2]} {e[2][:60]!r}")
                    continue
                print(f"OVERLAP {os.path.relpath(f, REPO)}: {prev[:2]} {prev[2][:60]!r} vs {e[:2]} {e[2][:60]!r}")
                continue
            dedup.append(e)
        texts[f] = apply_edits(old.raw[f], dedup) if dedup else old.raw[f]
    for name, crate in cut.crates.items():
        if name == cut.aggregator:
            continue
        root = crate["root_abs"]
        files = set(by_crate[name])
        need = {f for f in files if f != root and (f not in mounts or cut.assign[mounts[f][0]] != name or mounts[f][0] == root and False)}
        need = {f for f in need if f not in mounts or cut.assign[mounts[f][0]] != name}
        body = emit_paths(cut, name, sorted(need), trees, mounts, os.path.dirname(root))
        header = config["header"]
        text = texts.get(root)
        if text is None and os.path.exists(root):
            text = open(root, encoding="utf-8").read()
        if text is None:
            text = (crate.get("new_root") or f"//! {name}") + "\n"
        lines = text.split("\n")
        at = 0
        while at < len(lines) and (lines[at].startswith("//!") or lines[at].startswith("#![") or not lines[at].strip()):
            at += 1
        lines[at:at] = header + [""]
        text = "\n".join(lines).rstrip("\n") + "\n"
        if body:
            text += "\n//#region 🌳️Modules\n" + "\n".join(body) + "\n//#endregion 🌳️Modules\n"
        texts[root] = text
        os.makedirs(os.path.dirname(root), exist_ok=True)
    for f, text in texts.items():
        if text != old.raw.get(f):
            open(f, "w", encoding="utf-8").write(text)
    for path, (raw, edits) in consumers.items():
        open(path, "w", encoding="utf-8").write(apply_edits(raw, edits))
    usage = collections.defaultdict(set)
    for (a, b), rows in cut.samples.items():
        usage[a].add(b)
    out = os.path.join(REPO, ".🧬semio/🦑️repo/⚡️cache/p4", config.get("name", "cut") + ".usage.json")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    json.dump({a: sorted(b) for a, b in usage.items()}, open(out, "w"), indent=1, sort_keys=True)
    print(f"written: {len([1 for f, t in texts.items() if t != old.raw.get(f)])} crate files, {len(consumers)} consumer files; usage {out}")


if __name__ == "__main__":
    main()
