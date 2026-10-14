#!/usr/bin/env python3
"""🏗️ Rehosts one `#[path]`-tree monolith crate into owner-rooted crates (brief law L6, P9 decision N1).

The monolith's module tree is parsed from its root (inline `#[path = "."]` scaffolds, `mod component; pub use
component::*;` file modules, alias modules made of glob re-exports, root re-exports, re-exports of external crates).
Every file is assigned to a target crate by the config rules; every target crate is rooted at its owner's `🦀️.rs` and
hosts its files at their logical module path below the owner's. Every `crate::`/`super::`/`self::`/root-relative path
is resolved to the item that defines it and rewritten to that item's new home (`crate::…` inside the same crate,
`<crate>::…` across crates, the external path for re-exported external items); a re-export hop inside the owning crate
is kept when the definition sits in a private module.

Before the rewrite, in memory: root items that lower crates name move into the `root_to` crate's owner file, configured
regions move into new files (`regions`), new owner files are created (`create`), and text transforms run (`transforms`).
After it: module declarations whose file changes crate or loses its scaffold are deleted and re-generated in the hosting
crate, the root scaffold and cross-crate `pub use` facades are deleted, imports of items defined in the importing module
are dropped, restricted visibilities become `pub(crate)` (or `pub` when another crate names the item), and every new
crate root gets the monolith's extern-crate header. `apply` writes the files and `🗑️generated/rehost-<crate>.json`
(per-crate edges, external crate idents and files) for the manifest step.

usage: 🏗️rehost.py <config.json> plan|apply
"""
import collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
_ns = {"__file__": os.path.join(HERE, "🦀️rust_items.py")}
exec(compile(open(_ns["__file__"], encoding="utf-8").read(), _ns["__file__"], "exec"), _ns)
strip, parse_scope, path_attr, is_test_gated, use_leaves, IDENT = (_ns[k] for k in ("strip", "parse_scope", "path_attr", "is_test_gated", "use_leaves", "IDENT"))
SEP = r"\s*::\s*"
OVERLAY, DELETED = {}, set()


def read(path):
    path = os.path.normpath(path)
    if path in OVERLAY:
        return OVERLAY[path]
    return open(path, encoding="utf-8").read()


def exists(path):
    path = os.path.normpath(path)
    return path in OVERLAY or (path not in DELETED and os.path.exists(path))


class Scope:
    """📦️ One module of the monolith: a file body or an inline `mod x { }` block."""

    def __init__(self, path, file, span, parent, decl, vis, test):
        self.path, self.file, self.span, self.parent, self.decl, self.vis, self.test = path, file, span, parent, decl, vis, test
        self.children, self.items, self.uses = {}, {}, []
        self.macro_text = ""
        self.is_file = span is None

    def __repr__(self):
        return "Scope(" + "::".join(self.path) + ")"


class Tree:
    """🌳️ The monolith's module tree with per-file raw and stripped text."""

    def __init__(self, root):
        self.root_file = os.path.normpath(root)
        self.raw, self.text, self.scopes, self.file_scope, self.items_of = {}, {}, [], {}, collections.defaultdict(list)
        self.root = None
        self.root = self.load(self.root_file, (), None, None, "", False)

    def load(self, file, path, parent, decl, vis, test):
        file = os.path.normpath(file)
        if file not in self.raw:
            self.raw[file] = read(file)
            self.text[file] = strip(self.raw[file])
        scope = Scope(path, file, None, parent, decl, vis, test)
        self.scopes.append(scope)
        self.file_scope.setdefault(file, scope)
        if self.root is None and parent is None:
            self.root = scope
        self.fill(scope, self.text[file], 0, len(self.text[file]), os.path.dirname(file))
        return scope

    def fill(self, scope, s, start, end, base):
        raw = self.raw[scope.file]
        for item in parse_scope(s, start, end):
            item.file = scope.file
            item.scope = scope
            self.items_of[scope.file].append(item)
            if item.kind == "mod":
                p = path_attr(raw, item.attrs)
                test = scope.test or is_test_gated(raw, item.attrs)
                if item.body:
                    nbase = os.path.normpath(os.path.join(base, p)) if p is not None else os.path.join(base, item.name)
                    child = Scope(scope.path + (item.name,), scope.file, item.body, scope, item, item.vis, test)
                    self.scopes.append(child)
                    scope.children[item.name] = child
                    self.fill(child, s, item.body[0], item.body[1], nbase)
                else:
                    target = os.path.normpath(os.path.join(base, p)) if p is not None else os.path.join(base, item.name + ".rs")
                    if not exists(target):
                        raise SystemExit(f"missing module file {target}")
                    scope.children[item.name] = self.load(target, scope.path + (item.name,), scope, item, item.vis, test)
            elif item.kind == "use":
                scope.uses.append((item, use_leaves(s[item.kw + 3:item.end - 1])))
            elif item.kind == "macro":
                text = s[item.start:item.end]
                scope.macro_text += " " + text
                if item.name.endswith("derive_artifact_facets"):
                    for m in re.finditer(r"(?:spec|builder|analyzer|composer)\s*:?\s+(" + IDENT + r")", text):
                        scope.items.setdefault(m.group(1), item)
            elif item.name and item.kind not in ("impl", "inner-attr"):
                scope.items.setdefault(item.name, item)
                if item.kind == "macro_rules" and re.search(r"#\[\s*macro_export", " ".join(raw[a:b] for a, b in item.attrs)) and self.root is not None:
                    item.exported = True
                    self.root.items.setdefault(item.name, item)


class Resolver:
    """🧭️ Resolves monolith paths to defining items, following child modules, explicit and glob re-exports."""

    def __init__(self, tree, crate_ident):
        self.tree, self.crate_ident = tree, crate_ident

    def lookup(self, scope, name, seen, include_private=True):
        key = (id(scope), name)
        if key in seen:
            return None
        seen = seen | {key}
        if name in scope.children:
            return ("mod", scope.children[name], None, [])
        if name in scope.items:
            it = scope.items[name]
            if it.kind == "macro_rules" and getattr(it, "scope", scope) is not scope:
                return ("item", it.scope, name, [])
            return ("item", scope, name, [])
        for item, leaves in scope.uses:
            if not include_private and not item.vis:
                continue
            for segs, alias in leaves:
                if not segs or segs[-1] == "*":
                    continue
                selfleaf = segs[-1] == "self"
                last = segs[-2] if selfleaf and len(segs) > 1 else segs[-1]
                if (alias or last) != name:
                    continue
                target = segs[:-1] if selfleaf else segs
                hit = self.resolve(scope, target, seen)
                if hit is None:
                    return ("ext", None, target, [(scope, name, item)])
                if hit[0] == "unresolved" or hit[4] < len(target):
                    if hit[0] == "ext":
                        return hit[:4]
                    continue
                return hit[0], hit[1], hit[2], hit[3] + [(scope, name, item)]
        for item, leaves in scope.uses:
            if not include_private and not item.vis:
                continue
            for segs, alias in leaves:
                if segs and segs[-1] == "*":
                    target = self.resolve(scope, segs[:-1], seen)
                    if target and target[0] == "mod":
                        hit = self.lookup(target[1], name, seen, include_private=False)
                        if hit:
                            return hit[0], hit[1], hit[2], hit[3] + [(scope, name, item)]
        if name[:1].isupper() and re.search(r"\b" + re.escape(name) + r"\b", scope.macro_text):
            return ("item", scope, name, [])
        return None

    def resolve(self, scope, segs, seen=frozenset()):
        """🧭️ (kind, scope, name, hops, consumed) for a path from `scope`; None for an external path."""
        if not segs:
            return ("mod", scope, None, [], 0)
        i, cur, head = 0, scope, segs[0]
        anchored = head in ("crate", "$crate", "super", "self") or head == self.crate_ident
        if head in ("crate", "$crate") or head == self.crate_ident:
            cur, i = self.tree.root, 1
        elif head == "super":
            while i < len(segs) and segs[i] == "super":
                cur, i = cur.parent, i + 1
        elif head == "self":
            i = 1
        hops, first = [], True
        if i == len(segs):
            return ("mod", cur, None, hops, i)
        while i < len(segs):
            name = segs[i]
            hit = self.lookup(cur, name, seen)
            if hit is None:
                return None if not anchored and first else ("unresolved", cur, name, hops, i)
            kind, s2, n2, h2 = hit
            hops = hops + h2
            first = False
            if kind == "ext":
                return ("ext", None, list(n2) + segs[i + 1:], hops, len(segs))
            if kind == "item":
                return ("item", s2, n2, hops, i + 1)
            cur = s2
            i += 1
        return ("mod", cur, None, hops, i)


def alias_target(resolver, scope):
    """🪞️ An inline module whose body is only glob re-exports stands for the module it re-exports."""
    if scope.is_file or scope.items or not scope.uses:
        return None
    globs = [segs for item, leaves in scope.uses for segs, alias in leaves if segs and segs[-1] == "*"]
    others = [segs for item, leaves in scope.uses for segs, alias in leaves if not (segs and segs[-1] == "*")]
    if len(globs) == 1 and not others:
        hit = resolver.resolve(scope, globs[0][:-1])
        if hit and hit[0] == "mod":
            return hit[1]
    return None


class Generated:
    """🧪️ A macro-generated item the parser cannot see, declared by config (`generated`: name -> crate key)."""

    def __init__(self, name, scope):
        self.kind, self.name, self.scope, self.exported, self.attrs, self.vis = "macro_rules", name, scope, True, [], "pub"


class Plan:
    """🗺️ Target crates, file hosting and module paths of one rehost run."""

    def __init__(self, cfg):
        self.cfg = cfg
        self.tree = Tree(os.path.join(REPO, cfg["root"]))
        self.res = Resolver(self.tree, cfg["crate"])
        self.crates = cfg["crates"]
        self.idents = {k: v["package"].replace("-", "_") for k, v in self.crates.items()}
        self.rules = [(re.compile(r), c, (m[0] if m else None)) for r, c, *m in cfg["rules"]]
        self.host, self.problems = {}, []
        self.assign()
        for name, key in cfg.get("generated", {}).items():
            owner = self.tree.file_scope.get(self.owner_file(key))
            if owner is not None:
                self.tree.root.items[name] = Generated(name, owner)

    def rel(self, file):
        return os.path.relpath(file, REPO)

    def owner_file(self, key):
        return os.path.normpath(os.path.join(REPO, self.crates[key]["owner"], "🦀️.rs"))

    def rule(self, file):
        rel = self.rel(file)
        for r, c, m in self.rules:
            if r.search(rel):
                return c, m
        return None, None

    def assign(self):
        owners = {self.owner_file(k): k for k in self.crates}
        for scope in self.tree.scopes:
            if not scope.is_file:
                continue
            key, explicit = self.rule(scope.file)
            if key is None:
                dh = self.host.get(id(self.tree.file_scope[scope.parent.file])) if scope.parent else None
                if dh is None:
                    self.problems.append(f"unassigned file {self.rel(scope.file)}")
                    continue
                self.host[id(scope)] = (dh[0], dh[1] + scope.path[len(self.tree.file_scope[scope.parent.file].path):])
                continue
            lg_path = tuple(x for x in scope.path if x != "component")
            if owners.get(scope.file) == key:
                path = ()
            elif explicit is not None:
                path = tuple(x for x in explicit.split("::") if x)
            else:
                prefix = tuple(x for x in (self.crates[key].get("prefix") or "").split("::") if x)
                if lg_path[:len(prefix)] != prefix:
                    self.problems.append(f"{self.rel(scope.file)}: {'::'.join(lg_path)} is not below the {key} prefix")
                    path = lg_path[-1:]
                else:
                    path = lg_path[len(prefix):]
            self.host[id(scope)] = (key, path)
        for scope in self.tree.scopes:
            if scope.is_file:
                continue
            fs = self.tree.file_scope[scope.file]
            if id(fs) in self.host and fs is not self.tree.root:
                key, path = self.host[id(fs)]
                self.host[id(scope)] = (key, path + scope.path[len(fs.path):])
            elif fs is self.tree.root and scope.parent is self.tree.root and "component" not in scope.children and scope.items:
                self.host[id(scope)] = ("agg" if "agg" in self.crates else None, scope.path)
        for _ in range(3):
            for scope in self.tree.scopes:
                if scope.is_file or id(scope) in self.host:
                    continue
                comp = scope.children.get("component")
                if comp is not None and id(comp) in self.host:
                    self.host[id(scope)] = self.host[id(comp)]
                    continue
                tgt = alias_target(self.res, scope)
                if tgt is not None and self.scope_host(tgt):
                    self.host[id(scope)] = self.scope_host(tgt)

    def scope_host(self, scope):
        for _ in range(8):
            if id(scope) in self.host:
                return self.host[id(scope)]
            comp = scope.children.get("component")
            if comp is not None:
                scope = comp
                continue
            tgt = alias_target(self.res, scope)
            if tgt is None:
                return None
            scope = tgt
        return None

    def file_host(self, file):
        return self.host.get(id(self.tree.file_scope[file]))

    def private_chain(self, scope, key):
        s = scope
        while s is not None:
            h = self.scope_host(s)
            if h is None or h[0] != key or not h[1]:
                return False
            lg = s.parent if s.path and s.path[-1] == "component" else s
            if lg.decl is not None and not lg.vis:
                return True
            s = lg.parent
        return False

    def new_path(self, ctx_file, kind, scope, name, hops):
        ctx = self.file_host(ctx_file) if ctx_file in self.tree.file_scope else ("<external>", ())
        if kind == "ext":
            return "::".join(name)
        target = self.scope_host(scope)
        if target is None or ctx is None or target[0] is None:
            return None
        key, path = target
        if kind == "item" and name and name in scope.items and scope.items[name].kind == "macro_rules" and getattr(scope.items[name], "exported", False):
            key, path = target[0], ()
        segs = list(path) + ([name] if name else [])
        if self.private_chain(scope, key):
            for hscope, hname, hitem in reversed(hops):
                h = self.scope_host(hscope)
                if h and h[0] == key and not self.private_chain(hscope, key):
                    segs = list(h[1]) + [hname]
                    break
        head = "crate" if ctx[0] == key else self.idents[key]
        return "::".join([head] + segs)


PATH_RE = re.compile(r"(?<![\w:$])(\$crate|crate|super|self|[A-Za-z_][A-Za-z0-9_]*)((?:\s*::\s*(?:[A-Za-z_][A-Za-z0-9_]*|super))+)")
VIS_IN_RE = re.compile(r"pub\s*\(\s*(?:in\s+[^)]*|super)\s*\)")


class Rewriter:
    """✍️ Collects per-file edits: path rewrites, use regrouping, declaration removal and generation, headers."""

    def __init__(self, plan):
        self.plan, self.tree, self.res = plan, plan.tree, plan.res
        self.edits = collections.defaultdict(list)
        self.removed = collections.defaultdict(list)
        self.report = {"unresolved": [], "unhosted": [], "dropped_reexports": [], "edges": collections.Counter(), "test_edges": collections.Counter(), "crossref": set()}

    def line(self, file, off):
        return self.tree.raw[file].count("\n", 0, off) + 1

    def in_removed(self, file, off):
        return any(a <= off < b for a, b in self.removed[file])

    def edge(self, file, scope, test, what=""):
        a, b = self.plan.file_host(file), self.plan.scope_host(scope) if scope is not None else None
        if a and b and b[0] and a[0] != b[0]:
            (self.report["test_edges"] if test else self.report["edges"])[(a[0], b[0])] += 1
            self.report.setdefault("samples", {}).setdefault((a[0], b[0], test), f"{self.plan.rel(file)}: {what}")

    def is_test(self, file, scope):
        s = scope
        while s is not None:
            if s.test:
                return True
            s = s.parent
        return self.tree.file_scope[file].test

    def scope_at(self, file, off):
        best = self.tree.file_scope[file]
        for sc in self.tree.scopes:
            if sc.file == file and sc.span and sc.span[0] <= off < sc.span[1]:
                if best.span is None or sc.span[0] >= best.span[0]:
                    best = sc
        return best

    def run(self):
        self.remove_declarations()
        for file in list(self.tree.raw):
            if self.plan.file_host(file) is None:
                continue
            self.rewrite_uses(file)
            self.rewrite_paths(file)
        upgrade = collections.defaultdict(set)
        for sid, name in self.report["crossref"]:
            upgrade[sid].add(name)
        for file in list(self.tree.raw):
            if self.plan.file_host(file) is not None:
                self.rewrite_visibility(file, upgrade)
        self.generate_declarations()
        self.headers()

    def remove_declarations(self):
        """🗑️ Root scaffold and every module declaration that the target layout re-generates."""
        root = self.tree.root
        for item in self.tree.items_of[root.file]:
            if item.scope is not root:
                continue
            if item.kind == "mod" or (item.kind == "use" and item.vis):
                self.removed[root.file].append((item.start, item.end))
        for scope in self.tree.scopes:
            if not scope.is_file or scope is root or scope.decl is None:
                continue
            decl, dfile = scope.decl, scope.decl.file
            if dfile == root.file:
                continue
            dh, fh, sh = self.plan.file_host(dfile), self.plan.host.get(id(scope)), self.plan.scope_host(decl.scope)
            preserved = dh and fh and sh and dh[0] == fh[0] == sh[0] and fh[1] == sh[1] + (decl.name,)
            if not preserved:
                self.removed[dfile].append((decl.start, decl.end))

    def rewrite_uses(self, file):
        for item in self.tree.items_of[file]:
            if item.kind != "use" or self.in_removed(file, item.start):
                continue
            scope = item.scope
            leaves = next(l for it, l in scope.uses if it is item)
            self.rewrite_use(file, scope, item, leaves)

    def rewrite_use(self, file, scope, item, leaves):
        plan, tree, res = self.plan, self.tree, self.res
        raw = tree.raw[file]
        ctx = plan.file_host(file)
        test = self.is_test(file, scope)
        out, changed = [], False
        for segs, alias in leaves:
            orig = "::".join(segs) + (f" as {alias}" if alias else "")
            head = segs[0] if segs else ""
            anchored = head in ("crate", "$crate", "super", "self") or head == plan.cfg["crate"]
            if not anchored and not (file == tree.root_file and (head in tree.root.children or head in tree.root.items)) and head not in scope.children:
                out.append((orig, None))
                continue
            glob, selfleaf = segs[-1] == "*", segs[-1] == "self"
            target = segs[:-1] if (glob or selfleaf) else segs
            r = res.resolve(scope, target)
            if r is None:
                out.append((orig, None))
                continue
            kind, dscope, name, hops, used = r
            if kind == "unresolved" or (kind != "ext" and used < len(target)):
                self.report["unresolved"].append(f"{plan.rel(file)}:{self.line(file, item.start)}: use {orig}")
                out.append((orig, None))
                continue
            if kind == "item" and not alias and dscope is scope:
                out.append((orig, "DROP"))
                changed = True
                continue
            new = plan.new_path(file, kind, dscope, name, hops)
            if new is None:
                self.report["unhosted"].append(f"{plan.rel(file)}:{self.line(file, item.start)}: use {orig}")
                out.append((orig, None))
                continue
            if kind == "item" and not alias and plan.scope_host(dscope) == plan.scope_host(scope) and plan.scope_host(dscope) is not None:
                out.append((orig, "DROP"))
                changed = True
                continue
            if item.vis and kind != "ext" and plan.scope_host(dscope) and ctx and plan.scope_host(dscope)[0] != ctx[0]:
                self.report["dropped_reexports"].append(f"{plan.rel(file)}: pub use {orig}")
                out.append((orig, "DROP"))
                changed = True
                continue
            if item.vis and kind == "ext" and file == tree.root_file:
                out.append((orig, "DROP"))
                changed = True
                continue
            if glob and kind == "mod" and plan.scope_host(dscope) == plan.scope_host(scope):
                out.append((orig, "DROP"))
                changed = True
                continue
            if kind != "ext":
                self.edge(file, dscope, test, orig)
                th = plan.scope_host(dscope)
                if th and ctx and th[0] != ctx[0] and kind == "item":
                    self.report["crossref"].add((id(dscope), name))
            keep_name = segs[-2] if selfleaf and len(segs) > 1 else segs[-1]
            if not glob and not alias and new.split("::")[-1] != keep_name:
                alias = keep_name
            text = new + ("::*" if glob else "") + (f" as {alias}" if alias else "")
            if re.sub(r"\s+", "", text) != re.sub(r"\s+", "", orig):
                changed = True
            out.append((orig, text))
        if not changed:
            return
        keep = [(o if t is None else t) for o, t in out if t != "DROP"]
        if not keep:
            self.edits[file].append((item.start, item.end, ""))
            return
        groups = collections.OrderedDict()
        for t in keep:
            base, _, alias = t.partition(" as ")
            parts = base.split("::")
            groups.setdefault("::".join(parts[:-1]), []).append(parts[-1] + (" as " + alias if alias else ""))
        lines = []
        for prefix, names in groups.items():
            names = list(dict.fromkeys(names))
            if not prefix:
                lines.extend(f"use {n};" for n in names)
            elif len(names) == 1:
                lines.append(f"use {prefix}::{names[0]};")
            else:
                lines.append(f"use {prefix}::{{{', '.join(names)}}};")
        attrs = raw[item.start:item.head]
        vis = (item.vis + " ") if item.vis else ""
        indent = re.search(r"[ \t]*$", raw[:item.start]).group(0)
        self.edits[file].append((item.start, item.end, ("\n" + indent).join(attrs + vis + line for line in lines)))

    def rewrite_paths(self, file):
        plan, tree, res = self.plan, self.tree, self.res
        s = tree.text[file]
        uses = [(it.start, it.end) for it in tree.items_of[file] if it.kind == "use"]
        for m in PATH_RE.finditer(s):
            off = m.start()
            if self.in_removed(file, off) or any(a <= off < b for a, b in uses):
                continue
            if re.search(r"pub\s*\(\s*in\s+$", s[max(0, off - 24):off]):
                continue
            segs = [x for x in re.split(SEP, m.group(0)) if x]
            head = segs[0]
            anchored = head in ("crate", "$crate", "super", "self") or head == plan.cfg["crate"]
            if not anchored and not (file == tree.root_file and (head in tree.root.children or head in tree.root.items)):
                continue
            sc = self.scope_at(file, off)
            r = res.resolve(sc, segs)
            if r is None:
                continue
            kind, dscope, name, hops, used = r
            if kind == "unresolved":
                self.report["unresolved"].append(f"{plan.rel(file)}:{self.line(file, off)}: {m.group(0)}")
                continue
            new = plan.new_path(file, kind, dscope, name, hops)
            if new is None:
                self.report["unhosted"].append(f"{plan.rel(file)}:{self.line(file, off)}: {m.group(0)}")
                continue
            if head == "$crate":
                new = new.replace("crate", "$crate", 1) if new.startswith("crate") else new
            text, pos, count = m.group(0), 0, 0
            for part in re.finditer(r"\$?[A-Za-z_][A-Za-z0-9_]*", text):
                count += 1
                if count == used:
                    pos = part.end()
                    break
            if kind != "ext":
                self.edge(file, dscope, self.is_test(file, sc), m.group(0))
                th, ctx = plan.scope_host(dscope), plan.file_host(file)
                if th and ctx and th[0] != ctx[0] and kind == "item":
                    self.report["crossref"].add((id(dscope), name))
            if new != re.sub(r"\s+", "", text[:pos]):
                self.edits[file].append((off, off + pos, new))

    def rewrite_visibility(self, file, upgrade):
        """🔓️ Restricted visibilities become `pub(crate)`; an item another crate names becomes `pub`."""
        raw, s = self.tree.raw[file], self.tree.text[file]
        targets = {}
        for item in self.tree.items_of[file]:
            if item.name and item.name in upgrade.get(id(item.scope), ()) and not self.in_removed(file, item.start):
                targets[item.head] = item
        done = set()
        for head, item in targets.items():
            if item.vis == "pub":
                continue
            m = re.match(r"pub\s*\([^)]*\)\s*", s[head:])
            self.edits[file].append((head, head + (m.end() if m else 0), "pub "))
            done.add(head)
        for m in VIS_IN_RE.finditer(s):
            if m.start() not in done and not self.in_removed(file, m.start()):
                self.edits[file].append((m.start(), m.end(), "pub(crate)"))

    def generate_declarations(self):
        """🪢️ Declares every hosted file whose declaration was removed, below its nearest hosted ancestor file."""
        plan, tree = self.plan, self.tree
        by_crate = collections.defaultdict(list)
        for scope in tree.scopes:
            if not scope.is_file or id(scope) not in plan.host:
                continue
            key, path = plan.host[id(scope)]
            if not path:
                continue
            decl = scope.decl
            if decl is not None and decl.file != tree.root.file and not self.in_removed(decl.file, decl.start):
                continue
            by_crate[key].append(scope)
        self.generated = collections.defaultdict(list)
        for key, scopes in by_crate.items():
            files = {plan.host[id(s)][1]: s.file for s in tree.scopes if s.is_file and id(s) in plan.host and plan.host[id(s)][0] == key}
            files.setdefault((), plan.owner_file(key))
            tree_nodes = collections.defaultdict(dict)
            for scope in sorted(scopes, key=lambda s: plan.host[id(s)][1]):
                path = plan.host[id(scope)][1]
                anc = max((p for p in files if p == path[:len(p)] and len(p) < len(path)), key=len)
                lg = scope.parent if scope.path and scope.path[-1] == "component" else scope
                vis = lg.vis if lg.decl is not None else "pub"
                attrs = [tree.raw[lg.decl.file][a:b] for a, b in lg.decl.attrs if not tree.raw[lg.decl.file][a:b].replace(" ", "").startswith("#[path")] if lg.decl is not None else []
                if lg.test and not any("cfg" in a and "test" in a for a in attrs):
                    attrs = ["#[cfg(test)]"] + attrs
                self.generated[files[anc]].append((path[len(anc):], scope.file, vis or "", attrs))
        for host_file, decls in self.generated.items():
            base = os.path.dirname(host_file)
            nested = {}
            for rel_path, file, vis, attrs in decls:
                node = nested
                for seg in rel_path[:-1]:
                    node = node.setdefault(("dir", seg), {})
                node[("file", rel_path[-1])] = (file, vis, attrs)
            lines = []
            def emit(node, indent):
                for (kind, name), value in sorted(node.items(), key=lambda kv: (kv[0][0] != "file", kv[0][1])):
                    pad = "    " * indent
                    if kind == "file":
                        file, vis, attrs = value
                        for a in attrs:
                            lines.append(pad + a)
                        lines.append(f'{pad}#[path = "{os.path.relpath(file, base)}"]')
                        lines.append(f"{pad}{(vis + ' ') if vis else ''}mod {name};")
                    else:
                        lines.append(f'{pad}#[path = "."]')
                        lines.append(f"{pad}pub mod {name} {{")
                        emit(value, indent + 1)
                        lines.append(pad + "}")
            emit(nested, 0)
            raw = tree.raw.get(host_file) if host_file in tree.raw else read(host_file) if exists(host_file) else ""
            text = "\n//#region 🪢️Mounts\n" + "\n".join(lines) + "\n//#endregion 🪢️Mounts\n"
            self.edits[host_file].append((len(raw), len(raw), text))

    def headers(self):
        plan = self.plan
        header = "\n".join(plan.cfg.get("header", []))
        for key in plan.crates:
            owner = plan.owner_file(key)
            if owner == self.tree.root_file:
                continue
            raw = self.tree.raw.get(owner) if owner in self.tree.raw else read(owner)
            m = re.match(r"(?:\s*//![^\n]*\n)*", raw)
            self.edits[owner].append((m.end(), m.end(), ("\n" if m.end() else "") + header + "\n"))


def external_files(plan):
    """🔭️ Every `.rs` file under the configured consumer roots that is not part of the monolith and names it."""
    ident = plan.cfg["crate"]
    found = []
    for root in plan.cfg.get("consumers", []):
        base = os.path.join(REPO, root)
        for dirpath, dirnames, filenames in os.walk(base):
            dirnames[:] = [d for d in dirnames if d not in ("target", "node_modules") and not d.startswith(".")]
            for name in filenames:
                path = os.path.normpath(os.path.join(dirpath, name))
                if not name.endswith(".rs") or path in plan.tree.file_scope:
                    continue
                if ident in read(path):
                    found.append(path)
    return sorted(set(found))


def rewrite_external(plan, file, edits, named):
    """🔭️ Retargets `<monolith>::…` paths and use trees in a consumer file to the crates that own the items."""
    ident = plan.cfg["crate"]
    raw = read(file)
    s = strip(raw)
    root = plan.tree.root
    uses = []
    def visit(start, end):
        for it in parse_scope(s, start, end):
            if it.kind == "use":
                uses.append(it)
            elif it.kind == "mod" and it.body:
                visit(*it.body)
    try:
        visit(0, len(s))
    except Exception as error:
        named.setdefault("_problems", []).append(f"{plan.rel(file)}: {error}")
        return
    spans = []
    for it in uses:
        leaves = use_leaves(s[it.kw + 3:it.end - 1])
        if not any(segs and segs[0] == ident for segs, alias in leaves):
            continue
        spans.append((it.start, it.end))
        out = []
        for segs, alias in leaves:
            if not segs or segs[0] != ident:
                out.append("::".join(segs) + (f" as {alias}" if alias else ""))
                continue
            glob, selfleaf = segs[-1] == "*", segs[-1] == "self"
            target = ["crate"] + (segs[1:-1] if (glob or selfleaf) else segs[1:])
            r = plan.res.resolve(root, target)
            new = plan.new_path(file, r[0], r[1], r[2], r[3]) if r and r[0] != "unresolved" and (r[0] == "ext" or r[4] == len(target)) else None
            if new is None:
                named.setdefault("_problems", []).append(f"{plan.rel(file)}: use {'::'.join(segs)}")
                out.append("::".join(segs) + (f" as {alias}" if alias else ""))
                continue
            named.setdefault(file, set()).add(new.split("::")[0])
            keep_name = segs[-2] if selfleaf and len(segs) > 1 else segs[-1]
            if not glob and not alias and new.split("::")[-1] != keep_name:
                alias = keep_name
            out.append(new + ("::*" if glob else "") + (f" as {alias}" if alias else ""))
        groups = collections.OrderedDict()
        for t in out:
            base, _, alias = t.partition(" as ")
            parts = base.split("::")
            groups.setdefault("::".join(parts[:-1]), []).append(parts[-1] + (" as " + alias if alias else ""))
        lines = []
        for p, n in groups.items():
            n = list(dict.fromkeys(n))
            lines.extend([f"use {x};" for x in n] if not p else [f"use {p}::{n[0]};"] if len(n) == 1 else [f"use {p}::{{{', '.join(n)}}};"])
        attrs, vis = raw[it.start:it.head], (it.vis + " ") if it.vis else ""
        indent = re.search(r"[ \t]*$", raw[:it.start]).group(0)
        edits.append((it.start, it.end, ("\n" + indent).join(attrs + vis + l for l in lines)))
    for m in re.finditer(r"(?<![\w:$])" + ident + r"((?:\s*::\s*[A-Za-z_][A-Za-z0-9_]*)+)", s):
        if any(a <= m.start() < b for a, b in spans):
            continue
        segs = ["crate"] + [x for x in re.split(SEP, m.group(1)) if x]
        r = plan.res.resolve(root, segs)
        if r is None or r[0] == "unresolved":
            named.setdefault("_problems", []).append(f"{plan.rel(file)}: {m.group(0)}")
            continue
        new = plan.new_path(file, r[0], r[1], r[2], r[3])
        if new is None:
            named.setdefault("_problems", []).append(f"{plan.rel(file)}: {m.group(0)} (unhosted)")
            continue
        text, pos, count = m.group(0), 0, 0
        for part in re.finditer(r"[A-Za-z_][A-Za-z0-9_]*", text):
            count += 1
            if count == r[4]:
                pos = part.end()
                break
        named.setdefault(file, set()).add(new.split("::")[0])
        edits.append((m.start(), m.start() + pos, new))


def tidy(text):
    """🧹️ Drops regions a move emptied and collapses the blank runs it left."""
    text = re.sub(r"//#region ([^\n]+)\n(?:[ \t]*\n)*//#endregion \1\n?", "", text)
    return re.sub(r"\n{3,}", "\n\n", text)


def apply_edits(raw, edits):
    edits = sorted(set(edits), key=lambda e: (e[0], e[1], 0 if e[2].lstrip().startswith(("#![", "extern crate")) else 1))
    out, pos = [], 0
    for a, b, t in edits:
        if a < pos:
            raise SystemExit(f"overlapping edit at {a} ({t[:60]!r})")
        out.append(raw[pos:a])
        out.append(t)
        pos = b
    out.append(raw[pos:])
    return "".join(out)


#region pre-passes
def transforms(cfg):
    """🔁️ Config text transforms on files (regex substitutions), creations and deletions, applied to the overlay."""
    for create in cfg.get("create", []):
        path = os.path.normpath(os.path.join(REPO, create["path"]))
        OVERLAY[path] = create["text"]
    for t in cfg.get("transforms", []):
        path = os.path.normpath(os.path.join(REPO, t["path"]))
        text = read(path)
        for pattern, repl in t["sub"]:
            text, n = re.subn(pattern, repl, text, flags=re.S | re.M)
            if n == 0 and not t.get("optional"):
                raise SystemExit(f"transform {pattern!r} matched nothing in {t['path']}")
        OVERLAY[path] = text
    for d in cfg.get("delete", []):
        DELETED.add(os.path.normpath(os.path.join(REPO, d)))


def item_text_span(raw, item):
    """📏️ Item span widened to its doc comments and the rest of its last line."""
    start = item.start
    while True:
        prev = raw.rfind("\n", 0, start - 1)
        line = raw[prev + 1:start].strip() if start > 0 else ""
        cand = raw[prev + 1:start - 1] if prev >= -1 else ""
        if re.match(r"^\s*///", cand) or re.match(r"^\s*#\[", cand):
            start = prev + 1
            continue
        break
    end = item.end
    nl = raw.find("\n", end)
    if nl >= 0 and not raw[end:nl].strip():
        end = nl + 1
    return start, end


def rebase(text, src_dir, dst_dir):
    """📎️ Re-relativizes `include_str!`/`include_bytes!` and `#[path]` literals of text moved from `src_dir` to `dst_dir`."""
    def fix(m):
        target = os.path.normpath(os.path.join(src_dir, m.group(2)))
        return m.group(1) + os.path.relpath(target, dst_dir) + m.group(3)
    text = re.sub(r'(include_(?:str|bytes)!\(\s*")([^"]+)("\s*\))', fix, text)
    return re.sub(r'(#\[path\s*=\s*")([^"]+)("\])', fix, text)


def move_regions(cfg):
    """🚚️ Moves `from_marker`..EOF of a file into a new file mounted below it, importing the names it used."""
    for region in cfg.get("regions", []):
        src = os.path.normpath(os.path.join(REPO, region["file"]))
        dst = os.path.normpath(os.path.join(REPO, region["to"]))
        raw = read(src)
        at = raw.index(region["from_marker"])
        moved, kept = rebase(raw[at:], os.path.dirname(src), os.path.dirname(dst)), raw[:at]
        stripped = strip(kept)
        names = set()
        for it in parse_scope(stripped, 0, len(stripped)):
            if it.kind == "use":
                for segs, alias in use_leaves(stripped[it.kw + 3:it.end - 1]):
                    if segs and segs[-1] not in ("*", "self"):
                        names.add(alias or segs[-1])
            elif it.name and it.kind not in ("impl", "mod", "macro", "inner-attr"):
                names.add(it.name)
        used = sorted(n for n in names if re.search(r"(?<![\w:])" + re.escape(n) + r"\b", strip(moved)))
        mod = region["mount"]
        rel = os.path.relpath(dst, os.path.dirname(src))
        imports = f"use super::{{{', '.join(used)}}};\n" if used else ""
        OVERLAY[dst] = region["doc"] + "\n\n" + imports + "\n" + moved
        OVERLAY[src] = kept.rstrip() + f'\n\n#[path = "{rel}"]\npub mod {mod};\npub use {mod}::*;\n'


def move_mods(cfg):
    """🚚️ Moves inline modules (with their `pub use <name>::*;`) into another file; the source keeps a temporary module
    re-export so old paths resolve, which the rewrite then retargets and deletes."""
    for spec in cfg.get("move_mods", []):
        src = os.path.normpath(os.path.join(REPO, spec["file"]))
        dst = os.path.normpath(os.path.join(REPO, spec["to"]))
        raw = read(src)
        st = strip(raw)
        items = parse_scope(st, 0, len(st))
        mod = next(it for it in items if it.kind == "mod" and it.name == spec["name"] and it.body)
        spans = [item_text_span(raw, mod)]
        glob = next((it for it in items if it.kind == "use" and re.fullmatch(r"use\s+" + spec["name"] + r"\s*::\s*\*\s*;", st[it.kw:it.end].strip())), None)
        if glob is not None:
            spans.append(item_text_span(raw, glob))
        moved = "".join(rebase(raw[a:b], os.path.dirname(src), os.path.dirname(dst)) for a, b in sorted(spans))
        kept = raw
        for a, b in sorted(spans, reverse=True):
            kept = kept[:a] + kept[b:]
        kept = kept.rstrip() + "\n" + spec["source_reexport"] + "\n"
        target = read(dst)
        for pattern in spec.get("drop_in_target", []):
            target = re.sub(pattern, "", target, flags=re.M)
        OVERLAY[src] = kept
        OVERLAY[dst] = target.rstrip() + "\n\n" + moved


def move_root_items(cfg):
    """🚚️ Moves root items named by lower crates into the `root_to` owner file (records, dialect, schema ids)."""
    target_key = cfg.get("root_to")
    if not target_key:
        return
    plan = Plan(cfg)
    rw = Rewriter(plan)
    root = plan.tree.root
    wanted = set()
    for file in plan.tree.raw:
        h = plan.file_host(file)
        if h is None or h[0] == "agg" or file == root.file:
            continue
        s = plan.tree.text[file]
        for m in PATH_RE.finditer(s):
            segs = [x for x in re.split(SEP, m.group(0)) if x]
            if segs[0] not in ("crate", "super", "self"):
                continue
            if rw.is_test(file, rw.scope_at(file, m.start())):
                continue
            r = plan.res.resolve(rw.scope_at(file, m.start()), segs)
            if r and r[0] == "item" and r[1] is root:
                wanted.add(r[2])
        for sc in plan.tree.scopes:
            if sc.file != file or rw.is_test(file, sc):
                continue
            for item, leaves in sc.uses:
                for segs, alias in leaves:
                    if segs and segs[0] in ("crate", "super", "self") and segs[-1] != "*":
                        r = plan.res.resolve(sc, segs[:-1] if segs[-1] == "self" else segs)
                        if r and r[0] == "item" and r[1] is root:
                            wanted.add(r[2])
    wanted |= set(cfg.get("root_to_extra", []))
    raw = plan.tree.raw[root.file]
    items = {it.name: it for it in plan.tree.items_of[root.file] if it.scope is root and it.name and it.kind not in ("mod", "use", "macro", "extern", "impl", "inner-attr")}
    stripped = plan.tree.text[root.file]
    changed = True
    while changed:
        changed = False
        for n in list(wanted):
            it = items.get(n)
            if it is None:
                continue
            body = stripped[it.start:it.end]
            for other in items:
                if other not in wanted and re.search(r"(?<![\w:])" + re.escape(other) + r"\b", body):
                    wanted.add(other)
                    changed = True
    impls = [it for it in plan.tree.items_of[root.file] if it.scope is root and it.kind == "impl" and any(re.search(r"\bfor\s+" + re.escape(n) + r"\b|^impl(?:<[^>]*>)?\s+" + re.escape(n) + r"\b", stripped[it.kw:it.end].split("{")[0].strip()) for n in wanted)]
    moving = sorted([items[n] for n in wanted if n in items] + impls, key=lambda it: it.start)
    if not moving:
        return
    pieces, spans = [], []
    for it in moving:
        a, b = item_text_span(raw, it)
        spans.append((a, b))
        pieces.append(raw[a:b].rstrip() + "\n")
    root_imports = {}
    for item, leaves in root.uses:
        for segs, alias in leaves:
            if segs and segs[-1] not in ("*", "self"):
                root_imports[alias or segs[-1]] = segs
    moved_text = rebase("\n".join(pieces), os.path.dirname(root.file), os.path.dirname(plan.owner_file(target_key)))
    needs = sorted(n for n in root_imports if re.search(r"(?<![\w:])" + re.escape(n) + r"\b", strip(moved_text)))
    dst = plan.owner_file(target_key)
    dst_logical = cfg["crates"][target_key]["prefix"]
    imports = "".join(f"use {'::'.join(root_imports[n])};\n" for n in needs)
    names = [it.name for it in moving if it.kind != "impl"]
    kept = raw
    for a, b in sorted(spans, reverse=True):
        kept = kept[:a] + kept[b:]
    kept = kept.rstrip() + f"\npub use crate::{dst_logical}::{{{', '.join(names)}}};\n"
    OVERLAY[root.file] = kept
    OVERLAY[dst] = read(dst).rstrip() + "\n\n" + imports + "\n" + moved_text
def drop_use_names(text, names):
    """✂️ Removes the given imported names from every `use` item of a file (an emptied item disappears)."""
    st = strip(text)
    edits = []
    for it in parse_scope(st, 0, len(st)):
        if it.kind != "use":
            continue
        leaves = use_leaves(st[it.kw + 3:it.end - 1])
        keep = [(segs, alias) for segs, alias in leaves if not (segs and segs[-1] in names and not alias)]
        if len(keep) == len(leaves):
            continue
        vis = (it.vis + " ") if it.vis else ""
        if not keep:
            edits.append((it.start, it.end, ""))
            continue
        groups = collections.OrderedDict()
        for segs, alias in keep:
            groups.setdefault("::".join(segs[:-1]), []).append(segs[-1] + (f" as {alias}" if alias else ""))
        lines = [f"use {p}::{n[0]};" if len(n) == 1 and p else f"use {p}::{{{', '.join(n)}}};" if p else f"use {n[0]};" for p, n in groups.items()]
        edits.append((it.start, it.end, "\n".join(vis + l for l in lines)))
    return apply_edits(text, edits)


def generic_leaves(cfg):
    """🧬️ Makes every mutation leaf generic over the dispatch enum (P9 N4): the `MutationKind` impl and the inverse take
    `Op: Mutation<S, Diff = D> + From<..>` for exactly the leaves the inverse emits, and the leaf stops naming the enum."""
    g = cfg.get("generic_leaves")
    if not g:
        return
    enum, snap = g["enum"], g["snapshot"]
    mroot = os.path.normpath(os.path.join(REPO, g["mutations_dir"]))
    src = read(os.path.join(mroot, "🦀️.rs"))
    variants = {v: (mod, ty) for v, mod, ty in re.findall(r"^\s+([A-Z][A-Za-z0-9]*)\(([a-z0-9_]+)(?:::mutation)?::([A-Za-z0-9]+)\),", src, re.M)}
    for leaf in g["leaves"]:
        d = os.path.join(mroot, leaf)
        inv_path, leaf_path = os.path.join(d, "↩️inverse", "🦀️.rs"), os.path.join(d, "🦀️.rs")
        bounds = []
        if exists(inv_path):
            inv = read(inv_path)
            for v in re.findall(r"\b" + enum + r"::([A-Z][A-Za-z0-9]*)\s*\(", inv):
                mod, ty = variants[v]
                b = f"From<{g['mutations_path']}::{mod}::{ty}>"
                if b not in bounds:
                    bounds.append(b)
            inv = re.sub(r"\b" + enum + r"::([A-Z][A-Za-z0-9]*)\s*\(", "Op::from(", inv)
            inv, n = re.subn(r"pub fn inverse\((\w+): &(\w+), (\w+): &(\w+)\) -> Result<Vec<" + enum + r">, semio_framework_value::ValueError>",
                             lambda m: f"pub fn inverse<Op>({m.group(1)}: &{m.group(2)}, {m.group(3)}: &{m.group(4)}) -> Result<Vec<Op>, semio_framework_value::ValueError>" + (f" where Op: {' + '.join(bounds)}" if bounds else ""), inv)
            if n != 1:
                raise SystemExit(f"inverse signature not found in {inv_path}")
            OVERLAY[os.path.normpath(inv_path)] = drop_use_names(inv, {enum})
        text = read(leaf_path)
        where = f"Op: protocol::Mutation<{snap}, Diff = {g['diff_path']}>" + "".join(" + " + b for b in bounds)
        text, n = re.subn(r"impl protocol::MutationKind<" + snap + ", " + enum + r"> for (\w+) \{", lambda m: f"impl<Op> protocol::MutationKind<{snap}, Op> for {m.group(1)} where {where} {{", text)
        if n != 1:
            raise SystemExit(f"MutationKind impl not found in {leaf_path}")
        text = text.replace(f"<{enum} as protocol::Mutation<{snap}>>::Diff", f"<Op as protocol::Mutation<{snap}>>::Diff").replace(f"Result<Vec<{enum}>,", "Result<Vec<Op>,")
        OVERLAY[os.path.normpath(leaf_path)] = drop_use_names(text, {enum})


def facet_leaves(cfg):
    """🍃️ Each schema facet owns its five leaves (`<FACET>_LEAVES`); the artifact descriptor assembles them (P9 N7)."""
    f = cfg.get("facet_leaves")
    if not f:
        return
    schema = os.path.normpath(os.path.join(REPO, f["schema_file"]))
    text = read(schema)
    for facet, (sub, const, modpath) in f["facets"].items():
        pattern = facet + r": semio_framework_schema_registry::FacetLeaves \{\s*rust: include_str!\(\"" + re.escape(sub) + r"/🦀️\.rs\"\),.*?\}"
        text, n = re.subn(pattern, f"{facet}: {modpath}::{const}", text, count=1, flags=re.S)
        if n != 1:
            raise SystemExit(f"facet {facet} leaves not found in {schema}")
        facet_file = os.path.join(os.path.dirname(schema), sub, "🦀️.rs")
        body = read(facet_file).rstrip()
        kinds = ", ".join(f'{k}: include_str!("{v}")' for k, v in (("rust", "🦀️.rs"), ("typescript", "🟦️.ts"), ("graphql", "🔗️.graphql"), ("json_schema", "🔣️.json"), ("proto", "🛰️.proto")))
        OVERLAY[os.path.normpath(facet_file)] = body + f"\n\n//#region 🍃️FacetLeaves\n/// 🍃️ The five handcrafted leaves of the `{facet}` schema facet.\npub const {const}: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves {{ {kinds} }};\n//#endregion 🍃️FacetLeaves\n"
    OVERLAY[schema] = text
#endregion pre-passes


def main():
    cfg = json.load(open(sys.argv[1], encoding="utf-8"))
    mode = sys.argv[2]
    original = set(Tree(os.path.join(REPO, cfg["root"])).raw)
    transforms(cfg)
    generic_leaves(cfg)
    facet_leaves(cfg)
    move_mods(cfg)
    move_regions(cfg)
    move_root_items(cfg)
    plan = Plan(cfg)
    if cfg.get("delete_unmounted"):
        artifact = os.path.dirname(os.path.join(REPO, cfg["root"]))
        for file in original - set(plan.tree.raw):
            if file.startswith(artifact + os.sep):
                DELETED.add(file)
    rw = Rewriter(plan)
    rw.run()
    rep = rw.report
    for p in plan.problems:
        print("PROBLEM", p)
    for k in ("unresolved", "unhosted"):
        print(f"## {k}: {len(rep[k])}")
        for line in rep[k][:80]:
            print("  ", line)
    print("## crate edges (normal)")
    for (a, b), n in sorted(rep["edges"].items()):
        print(f"  {a} -> {b}: {n}")
    print("## crate edges (test)")
    for (a, b), n in sorted(rep["test_edges"].items()):
        if (a, b) not in rep["edges"]:
            print(f"  {a} -> {b}: {n}   e.g. {rep['samples'].get((a, b, True), '')[-140:]}")
    print("## dropped re-exports:", len(rep["dropped_reexports"]))
    hosted = collections.Counter(plan.host[id(s)][0] for s in plan.tree.scopes if s.is_file and id(s) in plan.host)
    print("## files per crate", dict(hosted))
    named = {}
    for file in external_files(plan):
        rewrite_external(plan, file, rw.edits[file], named)
    for problem in named.get("_problems", [])[:60]:
        print("EXTERNAL", problem)
    files = {}
    for file in set(plan.tree.raw) | set(rw.edits):
        raw = plan.tree.raw.get(file) if file in plan.tree.raw else read(file) if exists(file) else ""
        edits = rw.edits.get(file, []) + [(a, b, "") for a, b in rw.removed.get(file, [])]
        new = apply_edits(raw, edits)
        if edits or file in OVERLAY:
            new = tidy(new)
        if new != (open(file, encoding="utf-8").read() if os.path.exists(file) else None):
            files[file] = new
    for file, text in OVERLAY.items():
        files.setdefault(file, text) if file not in files and text != (open(file, encoding="utf-8").read() if os.path.exists(file) else None) else None
    summary = {"edges": [[a, b, n] for (a, b), n in rep["edges"].items()], "test_edges": [[a, b, n] for (a, b), n in rep["test_edges"].items()],
               "files": {k: sorted(plan.rel(s.file) for s in plan.tree.scopes if s.is_file and plan.host.get(id(s), (None,))[0] == k) for k in plan.crates},
               "test_files": sorted(plan.rel(s.file) for s in plan.tree.scopes if s.is_file and rw.is_test(s.file, s)),
               "owners_created": [plan.rel(os.path.join(REPO, c["path"])) for c in cfg.get("create", [])],
               "crossref": len(rep["crossref"]),
               "consumers": {plan.rel(f): sorted(v) for f, v in named.items() if f != "_problems"}}
    out = os.path.join(HERE, "🗑️generated", "rehost-" + cfg["crate"] + ".json")
    json.dump(summary, open(out, "w"), ensure_ascii=False, indent=1)
    print(f"## {len(files)} files change, {len(DELETED)} deleted")
    if mode == "apply":
        for file, text in files.items():
            os.makedirs(os.path.dirname(file), exist_ok=True)
            open(file, "w", encoding="utf-8").write(text)
        for d in DELETED:
            if os.path.isdir(d):
                import shutil
                shutil.rmtree(d)
            elif os.path.exists(d):
                os.remove(d)
    elif mode == "diff":
        import difflib
        for file, text in sorted(files.items()):
            old = open(file, encoding="utf-8").read().splitlines() if os.path.exists(file) else []
            for line in difflib.unified_diff(old, text.splitlines(), plan.rel(file), plan.rel(file), lineterm="", n=1):
                print(line)


if __name__ == "__main__":
    main()
