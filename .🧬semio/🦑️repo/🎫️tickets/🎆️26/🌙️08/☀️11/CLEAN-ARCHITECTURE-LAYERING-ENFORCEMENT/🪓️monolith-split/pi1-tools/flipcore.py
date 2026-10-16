#!/usr/bin/env python3
"""🔧️ Flip planner: for a group of crates that stop being modules of the monolith, computes the new text of every file they own (explicit imports instead of inherited scopes, translated paths, foreign includes/mounts removed) and the new root of each crate. Dry run by default; `Plan.apply()` writes through ed.py."""
import collections, os, re, sys
from lib import *
from rs import blank
import rsparse as P
import modnode as M
import world as Wd
import layout as L
import translate as TR
import ed
import reloc as R

IDENT = TR.IDENT
ALIASES = ("store", "protocol", "dsl", "vcs", "pack")


def stmt_ids(clean, st):
    return set(IDENT.findall(clean[st.start:st.end]))


class Edit:
    __slots__ = ("lo", "hi", "text")

    def __init__(self, lo, hi, text):
        self.lo, self.hi, self.text = lo, hi, text


def widen_removal(text, lo, hi):
    """a removed statement takes its comment lines directly above and one trailing newline with it"""
    while True:
        j = text.rfind("\n", 0, lo - 1) if lo > 0 else -1
        line_start = j + 1
        line = text[line_start:lo].strip() if text[line_start:lo].strip() == "" else None
        prev_end = lo
        # find the previous line
        k = lo
        while k > 0 and text[k - 1] in " \t":
            k -= 1
        if k > 0 and text[k - 1] == "\n":
            ps = text.rfind("\n", 0, k - 1) + 1
            prev = text[ps:k - 1]
            if prev.lstrip().startswith("//") and not prev.lstrip().startswith(("//#", "//!")):
                lo = ps
                continue
        break
    while lo > 0 and text[lo - 1] in " \t":
        lo -= 1
    if hi < len(text) and text[hi] == "\n":
        hi += 1
    return lo, hi


class Plan:
    def __init__(self, group, roots, promote=None, W=None, extra_mounts=()):
        self.W = W or Wd.World()
        comp = self.W.mono.mods.get("component")
        for name, file in extra_mounts:
            M.attach(comp, name, file)
        self.group = list(group)
        self.gset = set(group)
        self.roots = roots                      # key -> root file (class B: the old module file; class A: stub path)
        self.roots_all = {k: REPO + "/" + Wd.O.CR[k]["dir"] + "/🦀️.rs" for k in group}
        self.LY = L.Layout(self.W, promote=promote or {}, roots=roots)
        self.rank = {k: v["rank"] for k, v in Wd.O.CR.items()}
        self.notes = collections.defaultdict(list)
        self.out = {}                           # file -> new text
        self.file_part = {}
        for node in M.walk(self.W.mono):
            for p in node.parts:
                if p.lo == 0 and p.hi == len(p.text):
                    self.file_part[p.file] = p
        self.mod_parts = collections.defaultdict(list)
        for node in M.walk(self.W.mono):
            for p in node.parts:
                a = self.LY.anchor(p)
                self.mod_parts[(a[0], tuple(a[1]))].append(p)
        self.removed = {}                       # (file) -> list of (lo, hi, reason)
        self.reloc = {}                         # (id(part), stmt start) -> Unit
        self.reloc_files = set()
        self.units = collections.defaultdict(list)
        self.bound = collections.defaultdict(set)
        self.moved_mounts = collections.defaultdict(list)     # K -> (name, vis, attrs, child file) removed foreign `mod x;`
        self.dropped_uses = collections.defaultdict(list)     # K -> (statement text, reason)
        import uo as _uo
        self.base_real = set(_uo.BASE_REAL)
        self._skip_files = set()
        self.host_exports = {}

    # ownership helpers -----------------------------------------------------------------------------------------
    def owner(self, part):
        return self.LY.part_owner(part)

    def allowed_crates(self):
        """crates a group crate may name: the already flipped ones and the group itself"""
        import uo as _u
        return set(_u.BASE_REAL) | set(_u.flipped_by_me()) | self.gset

    def outranks(self, k2, k):
        return k2 in self.rank and k in self.rank and self.rank[k2] > self.rank[k]

    def child_part_for_include(self, node, path_attr_file):
        return self.file_part.get(path_attr_file)

    # statement classification ----------------------------------------------------------------------------------
    def classify(self, K, node, part, st, clean):
        """'keep' | 'remove' | 'relocate' for a statement of part (inside node) in crate K"""
        if (id(part), st.start) in self.reloc:
            return "relocate"
        if st.kind == "other" and P.split_attrs(part.text, clean, st)[1] is not None and clean[P.split_attrs(part.text, clean, st)[1]:P.split_attrs(part.text, clean, st)[1] + 8] == "include!":
            m = M.INCLUDE.match(part.text, P.split_attrs(part.text, clean, st)[1])
            if m:
                inc = os.path.normpath(os.path.join(os.path.dirname(part.file), m.group(1)))
                ip = self.file_part.get(inc)
                if ip is not None and self.owner(ip) != K:
                    return "remove"
            return "keep"
        if st.kind == "mod" and not st.inline:
            child = node.mods.get(st.name)
            if child is not None and not getattr(child, "missing", False) and child.parts:
                if self.owner(child.parts[0]) != K:
                    return "remove"
            return "keep"
        if st.kind == "mod" and st.inline:
            child = node.mods.get(st.name)
            if child is not None and self.LY.content_owner(child) not in (None, K):
                return "remove"
            return "keep"
        return "keep"

    def precompute_reloc(self):
        """find cfg(test) units of the group that cannot stay in their crate (they name higher layers or facade fixtures)"""
        for node in M.walk(self.W.mono):
            for part in node.parts:
                K = self.owner(part)
                if K not in self.gset:
                    continue
                self._scan_units(K, node, part, part.lo, part.hi)
        # propagate: nested units inside relocated files are part of them
        return self.reloc

    def _scan_units(self, K, node, part, lo, hi):
        """all top-level cfg(test) statements of a part form one unit; when any of them names higher layers or facade fixtures the whole unit moves to a facade test host"""
        text, clean = part.text, part.clean
        gated = []
        for st in P.statements(text, clean, lo, hi):
            if st.kind == "use":
                continue
            if st.kind == "other":
                attrs, at = P.split_attrs(text, clean, st)
                if clean[at:at + 8] != "include!":
                    continue
            else:
                attrs = st.attrs
            if R.is_pure_test(attrs):
                gated.append(st)
            elif st.kind == "mod" and st.inline:
                ch = node.mods.get(st.name)
                if ch is not None:
                    self._scan_units(K, ch, ch.parts[0], st.body[0], st.body[1])
        if not gated:
            return
        files = set()
        seeds = []
        for st in gated:
            if st.kind == "other" or (st.kind == "mod" and not st.inline):
                for nd, pt in R.unit_ids(self, node, part, st):
                    files |= R.closure_files(self, [pt.file])
        extra_parts = [p for n in M.walk(self.W.mono) for p in n.parts if p.file in files]
        reasons = R.flagged_spans(self, K, node, part, gated) + R.flagged_parts(self, K, extra_parts)
        if not reasons:
            return
        u = R.Unit(K, node, part, gated[0], "stmts", files, None, sorted(set(reasons)))
        u.stmts = gated
        for st in gated:
            self.reloc[(id(part), st.start)] = u
        self.units[K].append(u)
        self.reloc_files |= files

    # per file --------------------------------------------------------------------------------------------------
    def kept_ids(self, K, node, part, lo, hi, depth_stmts=None):
        clean = part.clean
        ids = set()
        for st in P.statements(part.text, clean, lo, hi):
            if st.kind == "use":
                continue
            if self.classify(K, node, part, st, clean) in ("remove", "relocate"):
                continue
            if st.kind == "mod" and st.inline:
                child = node.mods.get(st.name)
                if child is not None:
                    ids |= self.kept_ids(K, child, child.parts[0], st.body[0], st.body[1])
                    continue
            ids |= stmt_ids(clean, st)
        return ids

    def declared(self, node, parts):
        names = set()
        for it in node.items:
            if it.part in parts:
                names.add(it.name)
        for u in node.uses:
            if u.part in parts and u.local:
                names.add(u.local)
        return names

    def module_info(self, K, key):
        """(ids used by the module and its nested modules, names the module declares, item names it declares)"""
        parts = [p for p in self.mod_parts[key] if p.file not in self.reloc_files]
        ids, decl, items = set(), set(), set()
        for k2, path in list(self.mod_parts):
            if k2 == key[0] and path[:len(key[1])] == key[1]:
                for p in self.mod_parts[(k2, path)]:
                    if p.file in self.reloc_files:
                        continue
                    ids |= self.kept_ids(K, p.node, p, p.lo, p.hi)
        for p in parts:
            decl |= self.declared(p.node, parts)
            items |= {it.name for it in p.node.items if it.part is p}
        return ids, decl, items

    def rewrite_file(self, file):
        part = self.file_part.get(file)
        if part is None:
            return None
        K = self.owner(part)
        if K not in self.gset:
            return None
        key = tuple(self.LY.anchor(part)[1])
        a = self.LY.anchor(part)
        modkey = (a[0], tuple(a[1]))
        tr = TR.Tr(self.W, self.LY, K, flipped=self.base_real, allowed=self.allowed_crates())
        ids, decl, items = self.module_info(K, modkey)
        text = part.text
        edits = []
        self.process(K, tr, part.node, part, 0, len(text), edits, ids, True, decl)
        return self.apply_edits(text, edits), tr.notes

    def process(self, K, tr, node, part, lo, hi, edits, ids, top, decl):
        mkey = self.LY.anchor(part)
        mkey = (mkey[0], tuple(mkey[1]))
        text, clean = part.text, part.clean
        stmts = P.statements(text, clean, lo, hi)
        skip = []
        header = []
        for st in stmts:
            if st.kind == "use":
                skip.append((st.start, st.end))
                newlines = self.use_text(K, tr, node, part, st, ids, top, decl, mkey)
                if newlines.strip() == "":
                    if st.vis or any(n[0] in ("UP-use", "unflipped-target", "use-dropped") for n in tr.notes[-6:]):
                        self.dropped_uses[K].append((text[st.start:st.end], rel(part.file)))
                    lo2, hi2 = widen_removal(text, st.start, st.end)
                    edits.append(Edit(lo2, hi2, ""))
                else:
                    edits.append(Edit(st.start, st.end, newlines))
                continue
            action = self.classify(K, node, part, st, clean)
            if action == "remove" and st.kind == "mod" and not st.inline:
                ch = node.mods.get(st.name)
                self.moved_mounts[K].append((st.name, st.vis, st.attrs, ch.parts[0].file if ch is not None and ch.parts else None, rel(part.file)))
            if action in ("remove", "relocate"):
                skip.append((st.start, st.end))
                lo2, hi2 = widen_removal(text, st.start, st.end)
                edits.append(Edit(lo2, hi2, ""))
                continue
            if st.kind == "mod" and st.inline:
                child = node.mods.get(st.name)
                if child is not None:
                    self.process(K, tr, child, child.parts[0], st.body[0], st.body[1], edits, ids | self.kept_ids(K, child, child.parts[0], st.body[0], st.body[1]), False, {it.name for it in child.items if it.part is child.parts[0]})
                    skip.append((st.body[0], st.body[1]))
                    continue
        aliases = set()
        for st in stmts:
            if st.kind == "use" and not P.is_test_cfg(st.attrs):
                for path, alias, glob in P.use_entries(st, clean):
                    if glob or not path:
                        continue
                    local = alias or (path[-1] if path[-1] != "self" else path[-2])
                    if local in ("self", "super", "crate"):
                        continue
                    r = self.W.resolve_path(node, path)
                    if r is not None and r.kind == "node":
                        aliases.add(local)
        for e in tr.body_edits(node, text, clean, lo, hi, skip, aliases):
            edits.append(Edit(*e))

    def use_text(self, K, tr, node, part, st, ids, top, decl=(), mkey=None):
        entries = P.use_entries(st, part.clean)
        out = []
        keep_verbatim = False
        for path, alias, glob in entries:
            if glob and not top and path == ["super"]:
                keep_verbatim = True
        if keep_verbatim and len(entries) == 1:
            return part.text[st.start:st.end]
        new_entries = []
        for path, alias, glob in entries:
            if glob and not top and path == ["super"]:
                new_entries.append((path, None, True))
                continue
            if glob:
                tgt = tr.W.resolve_path(node, path)
                if tgt is not None and tgt.kind == "node" and tgt.node.root().crate in (None, "FACADE"):
                    # internal monolith glob: explode into the names the module uses
                    for n, p, ga in tr.glob_names(tgt.node, ids):
                        if n in decl or self.W.bound_elsewhere(node, n, path):
                            continue
                        sk = p[0][len(TR.PFX):].replace("_", "-") if p[0].startswith(TR.PFX) else None
                        if sk and self.outranks(sk, K):
                            if n in ids:
                                tr.notes.append(("UP-glob", "::".join(p), n))
                            continue
                        new_entries.append((p, None if p[-1] == n else n, False, ga))
                    produced = any(not e_[2] for e_ in new_entries)
                    towner = self.LY.part_owner(tgt.node.parts[0]) if tgt.node.parts else None
                    if produced or towner in (K, None) or (towner in self.gset):
                        for e in tr.ext_globs(tgt.node):
                            if (e, None, True) not in new_entries:
                                new_entries.append((e, None, True))
                    continue
                if tgt is not None and tgt.kind in ("node", "crate"):
                    p = tr.emit(tgt)
                    if p is None:
                        continue
                    new_entries.append((p, None, True))
                    continue
                new_entries.append((path, None, True))
                continue
            tgt = tr.W.resolve_path(node, path)
            if tgt is None or tgt.kind in ("unresolved", "cycle"):
                tr.notes.append(("unresolved-use", "::".join(path), 0))
                new_entries.append((path, alias, False))
                continue
            p = tr.emit(tgt)
            if p is None:
                continue
            local = alias or (path[-1] if path[-1] != "self" else path[-2])
            site_k = None
            if p and p[0].startswith(TR.PFX):
                site_k = p[0][len(TR.PFX):].replace("_", "-")
            if site_k and self.outranks(site_k, K):
                if local in ids or st.vis:
                    tr.notes.append(("UP-use", "::".join(p), local))
                continue
            if not st.vis and local not in ids and local not in TR.TRAITS and local != "_":
                continue
            new_entries.append((p, local if p[-1] != local else None, False))
        for e_ in new_entries:
            p_, a_, g_ = e_[0], e_[1], e_[2]
            if not g_:
                self.bound[mkey].add(a_ or p_[-1])
        lines = TR.group_uses(new_entries, st.vis, st.attrs)
        return "\n".join(lines)

    @staticmethod
    def apply_edits(text, edits):
        edits = sorted(edits, key=lambda e: (e.lo, e.hi))
        out, last = [], 0
        for e in edits:
            if e.lo < last:
                raise RuntimeError(f"overlapping edits at {e.lo} (< {last}): {text[e.lo:e.lo+60]!r}")
            out.append(text[last:e.lo])
            out.append(e.text)
            last = e.hi
        out.append(text[last:])
        return "".join(out)

if __name__ == "__main__":
    pass


# ---------------------------------------------------------------------------------------------------------------
# assembly: promoted modules, crate roots, headers
# ---------------------------------------------------------------------------------------------------------------

def cfg_attrs(attrs):
    return re.findall(r"#\[\s*cfg\((?:[^\[\]]|\[[^\]]*\])*\)\s*\]", attrs or "")


def mount_name(self, primary):
    node = primary.node
    return self.LY.anchor(primary)[1][-1] if self.LY.anchor(primary)[1] else None


def needs_mount(self, K):
    """modules of crate K that sit at the crate root but are mounted (or included) by a file of another crate in the monolith"""
    out = []
    seen = set()
    for (k2, path), parts in self.mod_parts.items():
        if k2 != K or len(path) != 1:
            continue
        p0 = parts[0]
        node = p0.node
        if node.inline and p0 is node.parts[0] and any(q.host_part is p0 for q in parts):
            p0 = next(q for q in parts if q.host_part is p0)
        foreign = False
        attrs = ""
        if p0.host_part is not None:
            foreign = self.owner(p0.host_part) != K or (p0.host_part.node.inline and p0.host_part is p0.host_part.node.parts[0] and self.LY.content_owner(p0.host_part.node) != self.owner(p0.host_part))
            attrs = p0.include_attrs
        elif node.parent is not None and getattr(node, "mount_part", None) is not None and not node.inline:
            foreign = self.owner(node.mount_part) != K
            ms = getattr(node, "mount_stmt", None)
            attrs = ms[1].attrs if ms else ""
        elif node.inline:
            foreign = False
        if foreign and p0.file not in seen:
            seen.add(p0.file)
            out.append(dict(name=path[0], file=p0.file, attrs=cfg_attrs(attrs), key=(k2, path)))
    return out


def mount_lines(self, K, root_file):
    lines = []
    for m in self.needs_mount(K):
        rel_path = os.path.relpath(m["file"], os.path.dirname(root_file))
        pre = "".join(a + "\n" for a in m["attrs"])
        lines.append(f'{pre}#[path = "{rel_path}"]\npub mod {m["name"]};')
    return "\n".join(lines)


def inherited_header(self, K, key, tr):
    """explicit imports of the names a module uses that its old scope supplied from outside the module (host scope, removed or moved parts)"""
    parts = [p for p in self.mod_parts[key] if p.file not in self.reloc_files]
    full = [p for p in parts if p.lo == 0 and p.hi == len(p.text)]
    p0 = full[0] if full else parts[0]
    node = p0.node
    ids, decl, items = self.module_info(K, key)
    bound = self.bound.get(key, set())
    entries = []
    done = set()
    for name in sorted(ids - decl - bound):
        if name in Wd.PRIM or name in done:
            continue
        b = self.W.lookup(node, name)
        if not b:
            continue
        kind, bnode, obj = b[0]
        r = self.W.follow_any(b)
        if r is None:
            continue
        if r.kind == "item" and any(r.item.part is q for q in parts):
            continue
        p = tr.emit(r)
        if p is None:
            continue
        site_k = p[0][len(TR.PFX):].replace("_", "-") if p[0].startswith(TR.PFX) else None
        if site_k and self.outranks(site_k, K):
            tr.notes.append(("UP-inherited", "::".join(p), name))
            continue
        attrs = obj.attrs if kind == "use" else ""
        done.add(name)
        entries.append((cfg_attrs(attrs), p, name))
    lines = []
    for attrs, p, name in entries:
        local = None if p[-1] == name else name
        lines += TR.group_uses([(p, local, False)], "", "\n".join(attrs))
    if p0.host_part is not None or (node.inline and p0 is not node.parts[0]):
        for e in tr.ext_globs(node):
            lines.append("use " + "::".join(e) + "::*;")
    for t in sorted(tr.trait_names_in(node)):
        if t in done or t in decl or t in bound:
            continue
        b = self.W.lookup(node, t)
        if b:
            r = self.W.follow_any(b)
            p = tr.emit(r) if r is not None else None
            if p and not (r.kind == "item" and any(r.item.part is q for q in parts)):
                site_k = p[0][len(TR.PFX):].replace("_", "-") if p[0].startswith(TR.PFX) else None
                if not (site_k and self.outranks(site_k, K)):
                    lines += TR.group_uses([(p, None if p[-1] == t else t, False)], "", "")
    return "\n".join(lines)


def header_pos(text):
    """offset after the leading inner docs and inner attributes"""
    pos = 0
    for m in re.finditer(r"[^\n]*\n", text):
        line = m.group(0)
        if line.startswith("//!") or line.startswith("#![") or line.strip() == "":
            pos = m.end()
            continue
        break
    return pos


def alias_header(self, K, texts):
    used = set()
    for t in texts:
        c = blank(t)
        for a in ALIASES:
            if re.search(r"(?:crate::|(?<![A-Za-z0-9_:]))" + a + r"::", c) or re.search(r"\buse\s+" + a + r"\b", c) or re.search(r"\bextern\s+crate\s+\S+\s+as\s+" + a + r"\b", c):
                used.add(a)
    return used


Plan.needs_mount = needs_mount
Plan.mount_lines = mount_lines
Plan.inherited_header = inherited_header


def build(self, skip_files=()):
    """new text of every file of the group (rewritten bodies, headers, roots)"""
    self._skip_files = set(skip_files)
    files = {}
    notes = collections.defaultdict(list)
    owned = collections.defaultdict(list)
    self.precompute_reloc()
    for node in M.walk(self.W.mono):
        for p in node.parts:
            if p.lo == 0 and p.hi == len(p.text) and self.owner(p) in self.gset:
                owned[self.owner(p)].append(p)
    for K in self.group:
        for p in owned[K]:
            if p.file in skip_files or p.file in self.reloc_files:
                continue
            r = self.rewrite_file(p.file)
            if r is None:
                continue
            text, nts = r
            a = self.LY.anchor(p)
            key = (a[0], tuple(a[1]))
            tr = TR.Tr(self.W, self.LY, K)
            tr = TR.Tr(self.W, self.LY, K, flipped=self.base_real, allowed=self.allowed_crates())
            full = [q for q in self.mod_parts[key] if q.lo == 0 and q.hi == len(q.text) and q.file not in self.reloc_files]
            if full and full[0] is p:
                hdr = self.inherited_header(K, key, tr)
                if hdr.strip():
                    pos = header_pos(text)
                    text = text[:pos] + hdr + "\n" + text[pos:]
                nts = nts + tr.notes
            files[p.file] = text
            notes[K] += [(rel(p.file),) + tuple(n) for n in nts]
    self.files, self.fnotes = files, notes
    return files


Plan.build = build


# ---------------------------------------------------------------------------------------------------------------
# roots, manifests
# ---------------------------------------------------------------------------------------------------------------

def finalize(self, skip_mounts=(), skip_files=(), root_extra=None):
    """add crate roots (class A rewritten, class B extended) to the rewritten files; returns {file: text}"""
    files = dict(self.files)
    for f in skip_files:
        files.pop(f, None)
    out = dict(files)
    for K in self.group:
        root = self.roots_all[K]
        ktexts = [t for f, t in files.items() if self.LY.part_owner(self.file_part[f]) == K]
        aliases = alias_header(self, K, ktexts)
        mounts = []
        for m in self.needs_mount(K):
            if m["name"] in skip_mounts or m["file"] in skip_files or m["file"] in self.reloc_files:
                continue
            rel_path = os.path.relpath(m["file"], os.path.dirname(root))
            pre = "".join(a + "\n" for a in m["attrs"])
            mounts.append(f'{pre}#[path = "{rel_path}"]\npub mod {m["name"]};')
        ext = "".join(f"extern crate semio_framework_os_kernel as {a};\n" for a in ALIASES if a in aliases)
        if root in files:
            t = files[root]
            pos = header_pos(t)
            t = t[:pos] + ext + t[pos:]
            if mounts:
                t = t.rstrip("\n") + "\n\n" + "\n".join(mounts) + "\n"
            out[root] = t
        else:
            stub = read(root) if os.path.exists(root) else ""
            doc = "".join(l + "\n" for l in stub.split("\n") if l.startswith("//!"))
            doc = re.sub(r"//!\n//! 🧭️ WP\d skeleton[^\n]*\n", "", doc)
            kept = ""
            sclean = blank(stub)
            for st in P.statements(stub, sclean, 0, len(stub)):
                if st.kind == "mod" and not st.inline:
                    kept += stub[st.start:st.end] + "\n"
            out[root] = doc.rstrip("\n") + "\n\n" + ext + ("\n" + "\n".join(mounts) + "\n" if mounts else "") + (("\n" + kept) if kept else "")
        if root_extra and K in root_extra:
            out[root] = out[root].rstrip("\n") + "\n" + root_extra[K].strip("\n") + "\n"
    return out


Plan.finalize = finalize


# ---------------------------------------------------------------------------------------------------------------
# facade test hosts
# ---------------------------------------------------------------------------------------------------------------
HUB_OF_WP = {3: "👥️wp3-owners", 4: "⚛️wp4-reactor", 5: "⏯️wp5-features"}


def host_path(self, K):
    wp = Wd.O.CR[K]["wp"]
    return PLUG + "/🪝️mounts/" + HUB_OF_WP[wp] + "/🧪️test-hosts/" + Wd.O.CR[K]["emoji"] + K + "/🦀️.rs"


def host_module_name(self, K):
    return K.replace("-", "_") + "_test_hosts"


def unit_closure_parts(self, u):
    if u.kind == "stmts":
        return [p for n in M.walk(self.W.mono) for p in n.parts if p.file in u.files] + [u.part]
    if u.kind == "inline":
        seeds = R.unit_ids(self, u.node, u.part, u.st)
        return [p for n in M.walk(self.W.mono) for p in n.parts if p.node is seeds[0][0] or p.host_part is seeds[0][1]]
    return [p for n in M.walk(self.W.mono) for p in n.parts if p.file in u.files]


def host_text(self, K, flipped):
    units = self.units.get(K, [])
    if not units:
        return None
    hp = self.host_path(K)
    body = []
    n = 0
    for u in units:
        n += 1
        parts = self.unit_closure_parts(u)
        ids, decl = set(), set()
        if u.kind == "stmts":
            for s in u.stmts:
                ids |= set(IDENT.findall(u.part.clean[s.start:s.end]))
            for p in parts:
                if p is u.part:
                    continue
                for st in P.statements(p.text, p.clean, p.lo, p.hi):
                    if st.kind != "use":
                        ids |= set(IDENT.findall(p.clean[st.start:st.end]))
                decl |= self.declared(p.node, parts)
            decl |= {it.name for it in u.node.items if it.part is u.part and any(s.start <= it.stmt.start < s.end for s in u.stmts)}
        else:
          for p in parts:
            for st in P.statements(p.text, p.clean, p.lo, p.hi):
                if st.kind != "use":
                    ids |= set(IDENT.findall(p.clean[st.start:st.end]))
            decl |= self.declared(p.node, parts)
        tr = TR.Tr(self.W, self.LY, "FACADE", flipped=flipped)
        scope_node = u.node if u.kind in ("include", "stmts") else (R.unit_ids(self, u.node, u.part, u.st)[0][0])
        entries = []
        done = set()
        for name in sorted(ids - decl):
            if name in Wd.PRIM or name in done:
                continue
            b = self.W.lookup(scope_node, name)
            if not b:
                continue
            kind, bnode, obj = b[0]
            if kind == "item" and any(obj.part is q for q in parts):
                continue
            r = self.W.follow_any(b)
            p = tr.emit(r) if r is not None else None
            if p is None:
                continue
            attrs = obj.attrs if kind == "use" else ""
            done.add(name)
            entries.append((cfg_attrs(attrs), p, name))
        lines = []
        for attrs, p, name in entries:
            lines += TR.group_uses([(p, None if p[-1] == name else name, False)], "", "\n".join(attrs))
        for e in tr.ext_globs(scope_node):
            lines.append("use " + "::".join(e) + "::*;")
        for t in sorted(tr.trait_names_in(scope_node)):
            if t in done or t in decl:
                continue
            b = self.W.lookup(scope_node, t)
            if b:
                r = self.W.follow_any(b)
                p = tr.emit(r) if r is not None else None
                if p:
                    lines += TR.group_uses([(p, None if p[-1] == t else t, False)], "", "")
        if u.kind == "stmts":
            text = "\n".join(R.rebase_paths(u.part.text[s.start:s.end], os.path.dirname(u.part.file), os.path.dirname(hp)) for s in u.stmts)
        else:
            text = u.part.text[u.st.start:u.st.end]
            text = R.rebase_paths(text, os.path.dirname(u.part.file), os.path.dirname(hp))
        exported = sorted({it.name for p in parts for it in p.node.items if it.part is p and it.kind in ("fn", "struct", "enum", "const", "static", "type", "trait") and (it.vis or "").startswith("pub") and (u.kind != "stmts" or p is not u.part or any(s.start <= it.stmt.start < s.end for s in u.stmts))})
        self.host_exports[(K, f"{K.replace('-', '_')}_unit_{n}")] = exported
        body.append(f"#[cfg(test)]\npub(crate) mod {K.replace('-', '_')}_unit_{n} {{\n" + "".join("    " + l + "\n" for l in "\n".join(lines).split("\n") if l.strip()) + "    " + text.replace("\n", "\n    ") + "\n}\n")
    doc = f"//! 🧪️ Facade test host of `{K}`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.\n\n"
    return doc + "\n".join(body)


Plan.host_path = host_path
Plan.host_module_name = host_module_name
Plan.unit_closure_parts = unit_closure_parts
Plan.host_text = host_text


# ---------------------------------------------------------------------------------------------------------------
# production / test split of texts (dependency edges)
# ---------------------------------------------------------------------------------------------------------------

def part_is_test(self, p):
    if "/🧪️tests/" in p.file or "/🧪️testing/" in p.file:
        return True
    if p.host_part is not None:
        return R.is_pure_test(p.include_attrs) or self.part_is_test(p.host_part)
    ms = getattr(p.node, "mount_stmt", None)
    if ms is not None and not p.node.inline:
        if R.is_pure_test(ms[1].attrs):
            return True
        mp = getattr(p.node, "mount_part", None)
        return self.part_is_test(mp) if mp is not None else False
    return False


def split_prod_test(self, file, text):
    """(production text, test text) of a rewritten file"""
    part = self.file_part.get(file)
    if part is not None and self.part_is_test(part):
        return "", text
    clean = blank(text)
    prod, test = [], []
    for st in P.statements(text, clean, 0, len(text)):
        seg = text[st.start:st.end]
        (test if R.is_pure_test(st.attrs) or (st.kind == "other" and R.is_pure_test(P.split_attrs(text, clean, st)[0])) else prod).append(seg)
    return "\n".join(prod), "\n".join(test)


Plan.part_is_test = part_is_test
Plan.split_prod_test = split_prod_test


# ---------------------------------------------------------------------------------------------------------------
# consumers: mounts of group files from files outside the group
# ---------------------------------------------------------------------------------------------------------------

def foreign_mount_edits(self, skip=()):
    """{file: [(old statement text, new text, note)]} one-line re-points of `mod x;` units of files outside the group that mount a group-owned file; `skip` names are left to the spec"""
    out = collections.defaultdict(list)
    comp = self.W.mono.mods.get("component")
    for node in M.walk(self.W.mono):
        for part in node.parts:
            if self.owner(part) in self.gset or part.file in self.reloc_files:
                continue
            for st in P.statements(part.text, part.clean, part.lo, part.hi):
                if st.kind != "mod" or st.inline or st.name in skip:
                    continue
                child = node.mods.get(st.name)
                if child is None or getattr(child, "missing", False) or not child.parts:
                    continue
                cp = child.parts[0]
                Kc = self.owner(cp)
                if Kc not in self.gset or cp.file in self.reloc_files or cp.file in self._skip_files:
                    continue
                K2, path = self.LY.anchor(cp)
                lib = TR.PFX + K2.replace("-", "_")
                attrs = re.sub(r"#\[\s*path\s*=\s*\"[^\"]*\"\s*\]\s*", "", part.text[st.start:st.head_end] if False else st.attrs)
                attrs = attrs.strip()
                vis = st.vis
                hub_level = (node is comp)
                if hub_level and not path:
                    new = (attrs + "\n" if attrs else "") + (vis + " " if vis else "") + f"extern crate {lib} as {st.name};"
                else:
                    target = lib + "".join("::" + p for p in path)
                    new = (attrs + "\n" if attrs else "") + (vis + " " if vis else "") + f"use {target} as {st.name};"
                out[part.file].append((part.text[st.start:st.end], new, f"{st.name} -> {K2}{'/' + '/'.join(path) if path else ''}"))
    return out


Plan.foreign_mount_edits = foreign_mount_edits


# ---------------------------------------------------------------------------------------------------------------
# rehome: monolith files that stay in the facade but lose their parent module (their `super` changes)
# ---------------------------------------------------------------------------------------------------------------

def rehome(self, file):
    """new text of a facade-resident file whose parent module moved into a crate: its `use super::*` and `super::` paths name explicit targets (valid in the facade)"""
    part = self.file_part.get(file)
    if part is None:
        return None
    node = part.node
    text, clean = part.text, part.clean
    tr = TR.Tr(self.W, self.LY, "FACADE", flipped=self.base_real | set(self.group) | set(__import__("uo").flipped_by_me()))
    ids = set()
    for st in P.statements(text, clean, 0, len(text)):
        if st.kind != "use":
            ids |= set(IDENT.findall(clean[st.start:st.end]))
    stack = list(node.mods.values())
    while stack:
        ch = stack.pop()
        for p in ch.parts:
            for st in P.statements(p.text, p.clean, p.lo, p.hi):
                if st.kind != "use":
                    ids |= set(IDENT.findall(p.clean[st.start:st.end]))
        stack.extend(ch.mods.values())
    edits = []
    skip = []
    for st in P.statements(text, clean, 0, len(text)):
        if st.kind == "use":
            ents = P.use_entries(st, clean)
            if not any(p and p[0] == "super" for p, a, g in ents):
                continue
            skip.append((st.start, st.end))
            new = []
            for path, alias, glob in ents:
                if not (path and path[0] == "super"):
                    new.append((path, alias, glob))
                    continue
                tgt = self.W.resolve_path(node, path)
                if glob:
                    if tgt is not None and tgt.kind == "node" and tgt.node.root().crate in (None, "FACADE"):
                        for n, p, ga in tr.glob_names(tgt.node, ids):
                            if self.W.bound_elsewhere(node, n, path):
                                continue
                            new.append((p, None if p[-1] == n else n, False, ga))
                        for e in tr.ext_globs(tgt.node):
                            new.append((e, None, True))
                    elif tgt is not None and tgt.kind in ("node", "crate"):
                        p = tr.emit(tgt)
                        if p:
                            new.append((p, None, True))
                    continue
                if tgt is None or tgt.kind in ("unresolved", "cycle"):
                    new.append((path, alias, False))
                    continue
                p = tr.emit(tgt)
                if p is None:
                    continue
                local = alias or path[-1]
                new.append((p, local if p[-1] != local else None, False))
            lines = TR.group_uses(new, st.vis, st.attrs)
            edits.append(Edit(st.start, st.end, "\n".join(lines)))
        elif st.kind == "mod" and st.inline:
            skip.append((st.body[0], st.body[1]))
    for m in TR.BODY_PATH.finditer(clean):
        if m.group(1) != "super" or any(a <= m.start() < b for a, b in skip):
            continue
        segs = ["super"] + m.group(2).split("::")[1:]
        res = self.W.resolve_path(node, segs)
        if res is None or res.kind in ("unresolved", "cycle"):
            continue
        new = tr.emit(res)
        if new:
            edits.append(Edit(m.start(), m.end(), "::".join(new)))
    return self.apply_edits(text, edits), tr.notes


Plan.rehome = rehome
