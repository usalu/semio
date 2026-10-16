# ⚙️ Flip engine: rewrites `crate::`/`super::` paths of the files of a crate that becomes a real crate, using item-level name resolution over the staged tree.
import os, re, collections, sys
from common import *
from rs import blank, expand_use
from resolve import World
import layout as LY
import model

pt = model.pt
SEG = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
PATH_START = re.compile(r"(?<![A-Za-z0-9_$:])(crate|super)::")
USE_STMT = re.compile(r"(?m)^([ \t]*)((?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*)((?:pub(?:\([^)]*\))?[ \t]+)?)use[ \t]+((?:crate|super|self)::[^;]*);")
EXT_ALIASES = ("store", "protocol", "dsl", "vcs", "pack")


def inline_mod_spans(clean):
    spans = []
    for m in re.finditer(r"\bmod\s+[A-Za-z_]\w*\s*\{", clean):
        depth, j = 1, m.end()
        while j < len(clean) and depth:
            depth += (clean[j] == "{") - (clean[j] == "}")
            j += 1
        spans.append((m.end(), j))
    return spans


def in_spans(pos, spans):
    return any(a <= pos < b for a, b in spans)


def parse_path(clean, i):
    segs, j = [], i
    while True:
        m = SEG.match(clean, j)
        if not m:
            break
        segs.append(m.group(0))
        j = m.end()
        if clean.startswith("::", j) and SEG.match(clean, j + 2):
            j += 2
            continue
        break
    return segs, j


def expand_group_uses(text):
    clean = blank(text)
    out, last = [], 0
    for m in USE_STMT.finditer(clean):
        body = m.group(4)
        if "{" not in body and "*" not in body:
            continue
        first = re.match(r"(?:crate|super|self)::(\w+)", body)
        if first and first.group(1) in EXT_ALIASES:
            continue
        body_text = text[m.start(4):m.end(4)]
        lines = []
        for path, alias, glob in expand_use(body_text):
            p = "::".join(path)
            attrs = text[m.start(2):m.end(2)]
            if glob:
                lines.append(f"{m.group(1)}{attrs}{m.group(3)}use {p}::*;")
            else:
                a = f" as {alias}" if alias else ""
                lines.append(f"{m.group(1)}{attrs}{m.group(3)}use {p}{a};")
        out.append(text[last:m.start()])
        out.append("\n".join(l.replace("\n", " ") for l in lines))
        last = m.end()
    out.append(text[last:])
    return "".join(out)


class Engine:
    def __init__(self, world, layout, real, keys):
        """real: crate keys that are real crates after the flip (including the ones flipped now); keys: crates being flipped now"""
        self.w, self.L, self.real, self.keys = world, layout, set(real), set(keys)
        self.log = []
        self.unresolved = []
        self.widen = collections.defaultdict(set)
        self.root_ns = world.root_ns

    # ---------------------------------------------------------------- lookup helpers
    def lookup_root(self, name):
        for r in self.root_ns:
            res = self.w.lookup(r, name, fallback=False)
            if res:
                return res
        return self.w.global_def(name)

    def module_def(self, seg):
        return self.w.MM.get(seg)

    def descend(self, start, segs):
        """resolve `segs` below module file `start`; -> (kind, file, name, tail)"""
        cur = start
        for i, s in enumerate(segs):
            ch = {n: c for n, c, m in self.w.T.children(cur) if n != "<include>"}
            if s in ch:
                cur = ch[s]
                continue
            res = self.w.lookup(cur, s, fallback=(i == len(segs) - 1))
            if res is None:
                return ("unresolved", cur, s, segs[i + 1:])
            if res[0] == "mod":
                cur = res[1]
                continue
            if res[0] == "ext":
                return ("ext", None, "::".join(res[1]), segs[i + 1:])
            if res[0] == "inline":
                r2 = self.w.inline_lookup(res[1], res[2], segs[i + 1:], set(), 0)
                if r2 is None:
                    return ("unresolved", res[1], res[2], segs[i + 1:])
                if r2[0] == "def":
                    return ("def", r2[1], segs[i + 1] if len(segs) > i + 1 else s, segs[i + 2:])
                if r2[0] == "ext":
                    return ("ext", None, "::".join(r2[1]), [])
                if r2[0] == "mod":
                    return ("mod", r2[1], None, [])
                return ("unresolved", res[1], res[2], segs[i + 1:])
            return ("def", res[1], s, segs[i + 1:])
        return ("mod", cur, None, [])

    def crate_descend(self, lib, path):
        r = self.w.crate_root_rel(lib)
        if r:
            return self.descend(r, path)
        return ("ext", None, "::".join([lib] + path), [])

    def resolve_shim(self, m, rest):
        if not rest:
            return ("shim", None, None, [])
        name = rest[0]
        for u in m["uses"]:
            if u["glob"] or not u["path"]:
                continue
            if (u["alias"] or u["path"][-1]) == name:
                if u["path"][0].startswith("semio_framework_plugin_"):
                    return self.crate_descend(u["path"][0], u["path"][1:] + rest[1:])
                return ("ext", None, "::".join(u["path"] + rest[1:]), [])
        for u in m["uses"]:
            if u["glob"] and u["path"] and u["path"][0].startswith("semio_framework_plugin_"):
                res = self.crate_descend(u["path"][0], u["path"][1:] + rest)
                if res[0] in ("def", "mod", "ext"):
                    return res
        return ("unresolved", None, name, rest[1:])

    def hub_unit(self, m, rest):
        if m["kind"] == "file":
            return self.descend(m["file"], rest)
        if m["kind"] == "extern":
            return self.crate_descend(m["lib"], rest)
        if m["kind"] == "reexport":
            return self.crate_descend(m["lib"], m["path"] + rest)
        return self.resolve_shim(m, rest)

    def resolve(self, kw, segs, from_file):
        if kw == "super":
            p = self.w.parent_module(from_file)
            if p is None:
                return ("unresolved", None, "super", segs)
            if p in self.root_ns:
                return self.resolve("crate", segs, from_file)
            return self.descend(p, segs)
        if kw != "crate":
            return ("unresolved", None, kw, segs)
        if segs and segs[0] == "component" and len(segs) > 1:
            segs = segs[1:]
        s0 = segs[0]
        m = self.module_def(s0)
        if m is not None:
            return self.hub_unit(m, segs[1:])
        res = self.lookup_root(s0)
        if res is None:
            return ("unresolved", None, s0, segs[1:])
        if res[0] == "def":
            return ("def", res[1], s0, segs[1:])
        if res[0] == "mod":
            return self.descend(res[1], segs[1:])
        if res[0] == "inline":
            r2 = self.w.inline_lookup(res[1], res[2], segs[1:], set(), 0)
            if r2 is None:
                return ("unresolved", res[1], s0, segs[1:])
            if r2[0] == "def":
                return ("def", r2[1], segs[1], segs[2:])
            if r2[0] == "ext":
                return ("ext", None, "::".join(r2[1]), [])
            if r2[0] == "mod":
                return ("mod", r2[1], None, [])
            return ("unresolved", res[1], s0, segs[1:])
        return ("ext", None, "::".join(res[1]), segs[1:])

    # ---------------------------------------------------------------- rendering
    def render(self, kind, file, name, tail, cur):
        """-> new path text or None when the target is not a real crate (yet)"""
        if kind == "ext":
            return name + ("::" + "::".join(tail) if tail else "")
        if kind in ("unresolved", "shim") or file is None:
            return None
        if file in self.w.ext_of:
            lib, mp = self.w.ext_of[file]
            return "::".join([lib] + mp + ([name] if kind == "def" else []) + tail)
        j = self.L.owner.get(file) or model.file_owner(file)
        if j not in self.real and j != cur:
            return None
        if j.startswith(("TOP", "UNMAPPED")):
            return None
        mp = self.L.modpath(j, file) if j in self.L.crates else None
        if mp is None:
            return None
        if kind == "mod" and file in self.L.mname and not mp:
            mp = [self.L.mname[file]]
        base = "crate" if j == cur else libname(j)
        parts = [base] + mp
        if kind == "def":
            parts.append(name)
        return "::".join(parts + tail)


def libname(k):
    return PFX + k.replace("-", "_")


SKIP_TOK = set("Self self crate super std core alloc String Vec Option Result Box Arc Rc Some None Ok Err Default Clone Copy Debug Send Sync Sized Drop Fn FnMut FnOnce Into From TryFrom TryInto ToString PartialEq Eq Ord PartialOrd Hash Iterator IntoIterator Extend AsRef AsMut ToOwned Borrow bool char str u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 as break const continue else enum extern false fn for if impl in let loop match mod move mut pub ref return static struct trait true type unsafe use where while async await dyn union macro_rules".split())


def toks_used(text):
    clean = blank(text)
    out = set()
    for m in re.finditer(r"(?<![A-Za-z0-9_$.])([A-Za-z_][A-Za-z0-9_]*)(?![A-Za-z0-9_]*!)", clean):
        n = m.group(1)
        j = m.end()
        nxt = clean[j:j + 3].lstrip()
        if nxt.startswith(":") and not nxt.startswith("::"):
            continue
        out.add(n)
    for m in re.finditer(r"(?<![A-Za-z0-9_$.])([A-Za-z_][A-Za-z0-9_]*)!", clean):
        out.add(m.group(1))
    return out


def local_names(text):
    from index import FileInfo
    clean = blank(text)
    names = set()
    for m in re.finditer(r"(?m)^[ \t]*(?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*(?:pub(?:\([^)]*\))?[ \t]+)?(?:(?:async|unsafe|const|extern(?:[ \t]+\"[^\"]*\")?)[ \t]+)*(?:fn|struct|enum|trait|type|const|static|mod|union|macro_rules!)[ \t]+(?:mut[ \t]+)?([A-Za-z_][A-Za-z0-9_]*)", clean):
        names.add(m.group(1))
    for m in re.finditer(r"(?m)^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?use[ \t]+([^;]*);", clean):
        for path, alias, glob in expand_use(m.group(1)):
            if not glob and path:
                names.add(alias or path[-1])
    return names


GATE_RX = re.compile(r"cfg\((?:any\(|all\()?[^)]*(?:\btest\b|artifact-app-testing|media-turn-testing|reserved-producer-testing|original-actor-context-testing)")


def gate_at(text, clean, pos):
    """'gated' when the statement at pos (or the top-level item enclosing it) carries a test / testing-feature cfg"""
    ls = text.rfind("\n", 0, pos) + 1
    window = text[max(0, ls - 400):ls]
    lines = window.split("\n")
    prev = []
    for l in reversed(lines[:-1]):
        if l.strip().startswith(("#[", "//")) or l.strip() == "":
            prev.append(l)
            if len(prev) > 6: break
        else:
            break
    if GATE_RX.search("\n".join(prev)) or GATE_RX.search(text[ls:pos + 1]):
        return "gated"
    dm = 0
    start = 0
    i = pos
    depth = clean[:pos].count("{") - clean[:pos].count("}")
    if depth > 0:
        k = pos
        d = depth
        while k > 0 and d > 0:
            k -= 1
            if clean[k] == "}": d += 1
            elif clean[k] == "{": d -= 1
        ls2 = text.rfind("\n", 0, k) + 1
        window = text[max(0, ls2 - 400):k]
        if GATE_RX.search("\n".join(window.split("\n")[-6:])):
            return "gated"
    return "ungated"


def _rewrite(self, text, rel, cur):
    self.cur = cur
    text = expand_group_uses(text)
    clean = blank(text)
    spans = inline_mod_spans(clean)
    out, last = [], 0
    for m in PATH_START.finditer(clean):
        if m.start() < last:
            continue
        kw = m.group(1)
        if kw == "super" and in_spans(m.start(), spans):
            continue
        j = m.end()
        if j < len(clean) and clean[j] == "{":
            continue
        segs, end = parse_path(clean, j)
        if not segs:
            continue
        if segs[0] in EXT_ALIASES and kw == "crate":
            continue
        res = self.resolve(kw, segs, rel)
        new = self.render(res[0], res[1], res[2], res[3], cur)
        if new is None:
            self.unresolved.append((rel, kw + "::" + "::".join(segs), res[0], res[1], gate_at(text, clean, m.start())))
            continue
        out.append(text[last:m.start()])
        tail_ch = clean[end:end + 40].lstrip()
        ls = clean.rfind("\n", 0, m.start()) + 1
        in_use = re.match(r"\s*(?:pub(?:\([^)]*\))?\s+)?use\s+$", clean[ls:m.start()]) is not None and tail_ch.startswith(";")
        if in_use and not tail_ch.startswith("::") and segs[-1] != new.split("::")[-1] and segs[-1] not in ("self",):
            new = new + " as " + segs[-1]
        out.append(new)
        last = end
    out.append(text[last:])
    return "".join(out)


def _explode(self, text, rel, parent, cur):
    """`use super::*;` of a file whose parent module changed -> explicit imports resolved in the old parent"""
    if not re.search(r"(?m)^[ \t]*use super::\*;", blank(text)):
        return text
    body = re.sub(r"(?m)^[ \t]*use super::\*;[ \t]*\n", "", text)
    have = local_names(body)
    lines, seen = [], set()
    for t in sorted(toks_used(body)):
        if t in SKIP_TOK or t in have:
            continue
        res = self.w.lookup(parent, t, fallback=False)
        if res is None:
            continue
        if res[0] == "ext":
            path = "::".join(res[1])
            if path.split("::")[-1] != t:
                path += f" as {t}"
            stmt = f"use {path};"
        else:
            kind = "def" if res[0] == "def" else "mod"
            new = self.render(kind, res[1], t, [], cur)
            if new is None:
                self.unresolved.append((rel, "super::*:" + t, "unrendered", res[1]))
                continue
            if res[1] == rel:
                continue
            stmt = f"use {new};"
        if stmt not in seen:
            seen.add(stmt); lines.append(stmt)
    first_code = 0
    ls = body.split("\n")
    i = 0
    while i < len(ls) and (ls[i].startswith("//!") or ls[i].strip() == ""):
        i += 1
    return "\n".join(ls[:i] + lines + ls[i:])


Engine.rewrite = _rewrite
Engine.explode = _explode
