"""osm split engine: cuts one Rust module file into cohesive child files, regenerates precise imports, widens only
what crosses a child boundary to pub(super) (the original module scope), and rebuilds the parent as explicit re-exports."""
import collections, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from osmlib import *
from rstok import *

import json
OVERRIDES = {}
_ovp = os.path.join(os.path.dirname(os.path.abspath(__file__)), "overrides.json")
if os.path.exists(_ovp):
    OVERRIDES = json.load(open(_ovp, encoding="utf-8"))

DEF_KW = {"fn", "struct", "enum", "trait", "const", "static", "type", "union", "mod"}
WIDEN_KW = {"fn", "struct", "enum", "trait", "const", "static", "type", "union"}
QUALIFIERS = {"async", "const", "unsafe", "default", "extern"}


class Def:
    def __init__(self, name, kw, vis, attrs, ctx, item=None, reexport=False):
        self.name, self.kw, self.vis, self.attrs, self.ctx, self.item, self.reexport = name, kw, vis, attrs, ctx, item, reexport

    @property
    def cfg(self):
        return tuple(a for a in self.attrs if a.startswith("#[cfg"))


class Ctx:
    def __init__(self, fspec, key, path, mod=None, dirname=None, doc=None, body=None, qualified_only=False):
        self.fspec, self.key, self.path, self.mod, self.dirname, self.doc = fspec, key, path, mod, dirname, doc
        self.body, self.qualified_only = body or [], qualified_only
        self.defs, self.items, self.idents, self.widened = {}, [], set(), []
        self.leaves, self.sibs = [], []

    @property
    def text(self):
        return "\n".join(self.body)


class FileSpec:
    def __init__(self, label, rel, path, children, root_keep, inherit=None, trait_methods=None, mod_attrs=None):
        self.label, self.rel, self.path = label, rel, path
        self.children, self.root_keep = children, root_keep
        self.inherit = inherit
        self.trait_methods = trait_methods or {}
        self.mod_attrs = mod_attrs or {}
        self.ctxs, self.pool, self.root = [], {}, None
        self.root_head = []
        self.no_trait = {}
        self.root_replace = []


def rel_path(frm, to):
    k = 0
    while k < len(frm) and k < len(to) and frm[k] == to[k]:
        k += 1
    return "::".join(["super"] * (len(frm) - k) + list(to[k:]))


def region_stacks(lines):
    stack, out = [], []
    for i, l in enumerate(lines):
        out.append(list(stack))
        s = l.strip()
        if s.startswith("//#region"):
            stack.append(s[len("//#region"):].strip())
        elif s.startswith("//#endregion"):
            nm = s[len("//#endregion"):].strip()
            assert stack, f"unbalanced endregion line {i + 1}"
            if nm:
                assert stack[-1].split(" — ")[0].strip() == nm.split(" — ")[0].strip(), f"region mismatch line {i + 1}: {stack[-1]!r} vs {nm!r}"
            stack.pop()
    assert not stack, f"unclosed regions {stack}"
    return out


def cut_ranges(lines, stacks, ranges, dedent=0, skip=frozenset()):
    out, ostack = [], []
    for (a, b) in ranges:
        entry = stacks[a - 1]
        k = 0
        while k < len(ostack) and k < len(entry) and ostack[k] == entry[k]:
            k += 1
        while len(ostack) > k:
            out.append(f"//#endregion {ostack.pop()}")
        for nm in entry[k:]:
            out.append(f"//#region {nm}")
            ostack.append(nm)
        for i in range(a - 1, b):
            if (i + 1) in skip:
                continue
            l = lines[i]
            s = l.strip()
            if s.startswith("//#region"):
                ostack.append(s[len("//#region"):].strip())
            elif s.startswith("//#endregion"):
                if ostack:
                    ostack.pop()
            if dedent and l.startswith(" " * dedent):
                l = l[dedent:]
            out.append(l)
    while ostack:
        out.append(f"//#endregion {ostack.pop()}")
    while out and out[-1].strip() == "":
        out.pop()
    while out and out[0].strip() == "":
        out.pop(0)
    collapsed = []
    for l in out:
        if l.strip() == "" and collapsed and collapsed[-1].strip() == "":
            continue
        collapsed.append(l)
    return collapsed


def fix_relative_paths(body):
    rx = re.compile(r'(#\[path\s*=\s*"|include_(?:str|bytes)!\(\s*")([^"]+)(")')
    out, n = [], [0]

    def sub(m):
        if m.group(2).startswith("/"):
            return m.group(0)
        n[0] += 1
        return m.group(1) + "../" + m.group(2) + m.group(3)

    for l in body:
        out.append(rx.sub(sub, l))
    return out, n[0]


def load(fspec, text):
    lines = lines_of(text)
    items = outline(text)
    stacks = region_stacks(lines)
    pool, pool_lines = collections.OrderedDict(), set()
    for it in items:
        if it["kw"] == "use" and it["vis"] == "":
            txt = "\n".join(lines[it["core"]:it["end"] + 1])
            cfg = tuple(a for a in it["attrs"] if a.startswith("#[cfg"))
            other = [a for a in it["attrs"] if not a.startswith("#[cfg")]
            assert not other, f"use with attribute {other}"
            for path, bound, glob in parse_use(txt):
                if glob and path == ("super",):
                    fspec.super_glob = True
                    continue
                assert not glob, f"glob import {path}"
                assert path[0] != "super", "super:: use path"
                assert bound not in pool, f"duplicate import {bound}"
                pool[bound] = (path, cfg)
            for i in range(it["start"], it["end"] + 1):
                pool_lines.add(i + 1)
    fspec.pool, fspec.pool_lines = pool, pool_lines
    fspec.lines, fspec.stacks, fspec.items = lines, stacks, items


def make_ctxs(fspec):
    lines = fspec.lines
    claimed = collections.defaultdict(list)
    ctxs = []
    for ch in fspec.children:
        if ch.get("promote"):
            a, b = ch["promote"]
            rngs = [(a, b)]
            body = cut_ranges(lines, fspec.stacks, rngs, dedent=4, skip=fspec.pool_lines)
        else:
            rngs = ch["ranges"]
            body = cut_ranges(lines, fspec.stacks, rngs, skip=fspec.pool_lines)
        for (a, b) in rngs + list(ch.get("consume", [])):
            for i in range(a, b + 1):
                if i not in fspec.pool_lines:
                    claimed[i].append(ch["mod"])
        body, nfix = fix_relative_paths(body)
        c = Ctx(fspec, f"{fspec.label}:{ch['mod']}", fspec.path + (ch["mod"],), mod=ch["mod"], dirname=ch["dir"], doc=ch["doc"], body=body, qualified_only=bool(ch.get("promote")))
        c.fixed_paths = nfix
        c.promoted = ch.get("promote")
        ctxs.append(c)
    segs = []
    for seg in fspec.root_keep:
        if seg == "@MODS":
            segs.append("@MODS")
            continue
        a, b = seg
        for i in range(a, b + 1):
            if i not in fspec.pool_lines:
                claimed[i].append("<root>")
        segs.append(cut_ranges(lines, fspec.stacks, [seg], skip=fspec.pool_lines))
    fspec.root_segments = segs
    rootbody = []
    for sg in segs:
        if sg != "@MODS":
            rootbody += sg + [""]
    root = Ctx(fspec, f"{fspec.label}:<root>", fspec.path, body=rootbody)
    fspec.root = root
    fspec.ctxs = [root] + ctxs
    for i, l in enumerate(lines, 1):
        st = l.strip()
        if i in fspec.pool_lines:
            continue
        if st == "" or st.startswith("//#region") or st.startswith("//#endregion"):
            continue
        assert len(claimed.get(i, [])) == 1, f"{fspec.label}: line {i} claimed by {claimed.get(i)}: {l[:80]}"
    return fspec.ctxs


def analyze(ctx):
    ctx.items = outline(ctx.text) if ctx.body else []
    ctx.defs = {}
    ctx.local_imports = set()
    ctx.idents = idents_of(ctx.text)
    ctx.scope_idents = scope_idents_of(ctx.text)
    ctx.calls = method_calls_of(ctx.text)
    for it in ctx.items:
        kw = it["kw"]
        if kw in DEF_KW and it["name"]:
            assert it["name"] not in ctx.defs, f"{ctx.key}: duplicate def {it['name']}"
            ctx.defs[it["name"]] = Def(it["name"], kw, it["vis"], it["attrs"], ctx, item=it)
        elif kw == "use" and not it["vis"]:
            txt = "\n".join(ctx.body[it["core"]:it["end"] + 1])
            for path, bound, glob in parse_use(txt):
                ctx.local_imports.add(bound)
        elif kw == "use" and it["vis"]:
            txt = "\n".join(ctx.body[it["core"]:it["end"] + 1])
            for path, bound, glob in parse_use(txt):
                assert not glob
                ctx.defs[bound] = Def(bound, "use", it["vis"], it["attrs"], ctx, item=it, reexport=True)


def trait_method_table(files):
    tab = {}
    for f in files:
        tab.update(f.trait_methods)
    for f in files:
        for c in f.ctxs:
            for it in c.items:
                if it["kw"] == "trait" and it["name"]:
                    toks = tokenize("\n".join(c.body[it["core"]:it["end"] + 1]))
                    meths, d = set(), 0
                    for i, (k, v, s, e) in enumerate(toks):
                        if k == "p" and v in "{([":
                            d += 1
                        elif k == "p" and v in "})]":
                            d -= 1
                        elif k == "id" and v == "fn" and d == 1 and i + 1 < len(toks):
                            meths.add(toks[i + 1][1])
                    tab[it["name"]] = sorted(meths)
    return tab


def _adj_gt(toks, i):
    return toks[i][1] == ">" and i > 0 and toks[i - 1][1] == "-" and toks[i - 1][3] == toks[i][2]


def _skip_angle(toks, i):
    d = 0
    while i < len(toks):
        k, v, s, e = toks[i]
        if k == "p" and v == "<":
            d += 1
        elif k == "p" and v == ">" and not _adj_gt(toks, i):
            d -= 1
            if d == 0:
                return i + 1
        i += 1
    return i


def _has_vis_before(toks, j):
    if j - 1 >= 0 and toks[j - 1][0] == "id" and toks[j - 1][1] == "pub":
        return True
    if j - 1 >= 0 and toks[j - 1][1] == ")":
        q, dd = j - 1, 0
        while q >= 0:
            if toks[q][1] == ")":
                dd += 1
            elif toks[q][1] == "(":
                dd -= 1
                if dd == 0:
                    break
            q -= 1
        return q - 1 >= 0 and toks[q - 1][0] == "id" and toks[q - 1][1] == "pub"
    return False


def struct_field_edits(text):
    toks = tokenize(text)
    n = len(toks)
    i = 0
    while i < n and not (toks[i][0] == "id" and toks[i][1] == "struct"):
        i += 1
    i += 2
    if i < n and toks[i][0] == "p" and toks[i][1] == "<":
        i = _skip_angle(toks, i)
    while i < n and not (toks[i][0] == "p" and toks[i][1] in "{(;"):
        i += 1
    if i >= n or toks[i][1] == ";":
        return [], False
    close = "}" if toks[i][1] == "{" else ")"
    tuple_mode = close == ")"
    i += 1
    idx, res = 0, []
    while i < n and not (toks[i][0] == "p" and toks[i][1] == close):
        while toks[i][0] == "p" and toks[i][1] == "#":
            j, d = i + 1, 0
            while True:
                if toks[j][1] == "[":
                    d += 1
                elif toks[j][1] == "]":
                    d -= 1
                    if d == 0:
                        break
                j += 1
            i = j + 1
        start_i = i
        has_vis = False
        if toks[i][0] == "id" and toks[i][1] == "pub":
            has_vis = True
            i += 1
            if toks[i][1] == "(":
                d = 0
                while True:
                    if toks[i][1] == "(":
                        d += 1
                    elif toks[i][1] == ")":
                        d -= 1
                        if d == 0:
                            break
                    i += 1
                i += 1
        name = str(idx) if tuple_mode else toks[i][1]
        res.append((toks[start_i][2], name, has_vis))
        d = 0
        while i < n:
            k, v, s, e = toks[i]
            if k == "p" and v in "{([":
                d += 1
            elif k == "p" and v in "})]":
                if d == 0:
                    break
                d -= 1
            elif k == "p" and v == "<":
                d += 1
            elif k == "p" and v == ">" and not _adj_gt(toks, i):
                d -= 1
            elif k == "p" and v == "," and d == 0:
                break
            i += 1
        idx += 1
        if i < n and toks[i][1] == ",":
            i += 1
    return res, tuple_mode


def impl_member_edits(text):
    toks = tokenize(text)
    n = len(toks)
    i = 0
    while i < n and not (toks[i][0] == "id" and toks[i][1] == "impl"):
        i += 1
    i += 1
    if i < n and toks[i][0] == "p" and toks[i][1] == "<":
        i = _skip_angle(toks, i)
    is_trait, d = False, 0
    while i < n:
        k, v, s, e = toks[i]
        if k == "p" and v == "<":
            d += 1
        elif k == "p" and v == ">" and not _adj_gt(toks, i):
            d -= 1
        elif k == "id" and v == "for" and d == 0:
            is_trait = True
        elif k == "p" and v == "{" and d == 0:
            break
        i += 1
    if is_trait:
        return None
    res, depth = [], 1
    i += 1
    while i < n and depth > 0:
        k, v, s, e = toks[i]
        if k == "p" and v in "{([":
            depth += 1
        elif k == "p" and v in "})]":
            depth -= 1
        elif depth == 1 and k == "id" and v == "fn" and i + 1 < n and toks[i + 1][0] == "id":
            j = i
            while j - 1 >= 0 and ((toks[j - 1][0] == "id" and toks[j - 1][1] in QUALIFIERS) or toks[j - 1][0] == "str"):
                j -= 1
            res.append((toks[j][2], toks[i + 1][1], _has_vis_before(toks, j)))
        elif depth == 1 and k == "id" and v == "const" and i + 2 < n and toks[i + 1][0] == "id" and toks[i + 1][1] != "fn" and toks[i + 2][1] == ":":
            res.append((toks[i][2], toks[i + 1][1], _has_vis_before(toks, i)))
        i += 1
    return res


def widen(ctx, allctxs):
    other_ctxs = [c for c in allctxs if c is not ctx]
    others = set()
    for c in other_ctxs:
        others |= c.idents
    for it in ctx.items:
        kw, name = it["kw"], it["name"]
        lo, hi = it["core"], it["end"]
        if kw in WIDEN_KW and name and it["vis"] == "" and name in others:
            m = re.match(r"^(\s*)(.*)$", ctx.body[lo])
            ctx.body[lo] = m.group(1) + "pub(super) " + m.group(2)
            ctx.widened.append(("item", kw, name))
        if kw == "struct":
            blk = "\n".join(ctx.body[lo:hi + 1])
            edits, is_tuple = struct_field_edits(blk)
            ins = []
            for (off, fname, has_vis) in edits:
                if has_vis:
                    continue
                if is_tuple:
                    uses = any(name in c.idents and (re.search(r"\b" + re.escape(name) + r"\s*\(", c.text) or re.search(r"\.\d\b", c.text)) for c in other_ctxs)
                else:
                    uses = fname in others
                if uses:
                    ins.append((off, fname))
            if ins:
                for off, fname in sorted(ins, reverse=True):
                    blk = blk[:off] + "pub(super) " + blk[off:]
                    ctx.widened.append(("field", name, fname))
                ctx.body[lo:hi + 1] = blk.split("\n")
        elif kw == "impl":
            blk = "\n".join(ctx.body[lo:hi + 1])
            edits = impl_member_edits(blk)
            if not edits:
                continue
            ins = [(off, mname) for (off, mname, has_vis) in edits if not has_vis and mname in others]
            if ins:
                for off, mname in sorted(ins, reverse=True):
                    blk = blk[:off] + "pub(super) " + blk[off:]
                    ctx.widened.append(("method", it["head"].strip()[:50], mname))
                ctx.body[lo:hi + 1] = blk.split("\n")


def attr_prefix(cfg):
    return "".join(a + "\n" for a in cfg)


def render_leaves(leafs):
    groups = collections.OrderedDict()
    for path, bound, cfg in sorted(leafs, key=lambda x: (x[2], x[0][:-1], x[0][-1], x[1])):
        groups.setdefault((cfg, path[:-1]), []).append((path[-1], bound))
    out = []
    for (cfg, parent), names in groups.items():
        names = sorted(set(names))
        pre = attr_prefix(cfg)
        nm = lambda a: a[0] if a[0] == a[1] else f"{a[0]} as {a[1]}"
        if len(names) == 1:
            out.append(f"{pre}use {'::'.join(parent + (names[0][0],))}{'' if names[0][0] == names[0][1] else ' as ' + names[0][1]};")
        else:
            one = f"{pre}use {'::'.join(parent)}::{{{', '.join(nm(a) for a in names)}}};"
            if len(one) <= 190:
                out.append(one)
            else:
                out.append(f"{pre}use {'::'.join(parent)}::{{\n" + "\n".join(f"    {nm(a)}," for a in names) + "\n};")
    return out


def render_use_group(entries, vis=""):
    """entries: [(relpath, name, cfg)] -> grouped use statements with optional visibility."""
    groups = collections.OrderedDict()
    for rp, name, cfg in sorted(entries, key=lambda x: (x[2], x[0], x[1])):
        groups.setdefault((cfg, rp), []).append(name)
    out = []
    v = (vis + " ") if vis else ""
    for (cfg, rp), names in groups.items():
        names = sorted(set(names))
        pre = attr_prefix(cfg)
        if len(names) == 1:
            out.append(f"{pre}{v}use {rp}::{names[0]};")
        else:
            one = f"{pre}{v}use {rp}::{{{', '.join(names)}}};"
            out.append(one if len(one) <= 190 else f"{pre}{v}use {rp}::{{\n" + "\n".join(f"    {n}," for n in names) + "\n};")
    return out


def resolve_imports(ctx, fspec, tmeth):
    leaves, sibs, via_trait = {}, [], []
    idents = set(ctx.scope_idents)
    for tname, meths in tmeth.items():
        if tname not in idents and any(m in ctx.calls for m in meths) and tname not in fspec.no_trait.get(ctx.mod, ()):
            idents.add(tname)
            via_trait.append(tname)
    chain = [fspec] + ([fspec.inherit] if fspec.inherit else [])
    for n in sorted(idents):
        if n in ctx.defs or n in ctx.local_imports:
            continue
        for f in chain:
            sib = None
            for c in f.ctxs:
                if c is ctx:
                    continue
                if n in c.defs and not c.qualified_only:
                    assert sib is None, f"{n} defined twice in {f.label}"
                    sib = c.defs[n]
            if sib is not None:
                sibs.append((rel_path(ctx.path, sib.ctx.path), n, sib.cfg))
                break
            if n in f.pool:
                path, cfg = f.pool[n]
                leaves[n] = (path, n, cfg)
                break
            qm = [c for c in f.ctxs if c.qualified_only and c.mod == n and c is not ctx]
            if qm:
                sibs.append((rel_path(ctx.path, qm[0].path[:-1]) or "self", n, ()))
                break
    ov = OVERRIDES.get(ctx.key, {})
    drop, cfgt = set(ov.get("drop", [])), set(ov.get("cfg_test", []))
    leaves = {k: v for k, v in leaves.items() if k not in drop}
    sibs = [x for x in sibs if x[1] not in drop]
    cfg_test = ("#[cfg(test)]",)
    leaf_list = [(p, b, (cfg_test if b in cfgt and cfg_test[0] not in c else c)) for (p, b, c) in leaves.values()]
    sibs = [(rp, n, (cfg_test if n in cfgt and cfg_test[0] not in c else c)) for (rp, n, c) in sibs]
    ctx.leaves, ctx.sibs, ctx.via_trait = leaf_list, sibs, via_trait


def child_text(ctx, doc_override=None):
    head = ["//! " + d if d else "//!" for d in ctx.doc.split("\n")] + [""]
    imp = render_leaves(ctx.leaves) + render_use_group(ctx.sibs)
    out = head + (imp + [""] if imp else []) + ctx.body
    return "\n".join(out) + "\n"


def exports_of(ctx):
    pub, crate = [], []
    if ctx.qualified_only:
        return pub, crate
    ov = OVERRIDES.get(f"{ctx.fspec.label}:<exports>", {})
    drop, cfgt = set(ov.get("drop", [])), set(ov.get("cfg_test", []))
    for d in ctx.defs.values():
        if d.kw == "mod":
            continue
        cfg = d.cfg
        if d.vis == "pub":
            pub.append((ctx.mod, d.name, cfg))
        elif d.vis == "pub(crate)":
            if d.name in drop:
                continue
            if d.name in cfgt and "#[cfg(test)]" not in cfg:
                cfg = cfg + ("#[cfg(test)]",)
            crate.append((ctx.mod, d.name, cfg))
    return pub, crate


def root_block(fspec):
    root = fspec.root
    lines = []
    imp = render_leaves(root.leaves) + render_use_group(root.sibs)
    if imp:
        lines += imp + [""]
    for c in fspec.ctxs[1:]:
        for a in fspec.mod_attrs.get(c.mod, []):
            lines.append(a)
        lines.append(f'#[path = "{c.dirname}/🦀️.rs"]')
        lines.append(f"mod {c.mod};")
    pub_all, crate_all = [], []
    for c in fspec.ctxs[1:]:
        p, k = exports_of(c)
        pub_all += p
        crate_all += k
    names = collections.Counter(n for _, n, _ in pub_all + crate_all)
    dup = [n for n, k in names.items() if k > 1]
    assert not dup, f"duplicate export names {dup}"
    ex = render_use_group(pub_all, "pub") + render_use_group(crate_all, "pub(crate)")
    if ex:
        lines += [""] + ex
    return lines


def root_text(fspec):
    out = list(fspec.root_head)
    if out:
        out.append("")
    for sg in fspec.root_segments:
        if sg == "@MODS":
            while out and out[-1] == "":
                out.pop()
            out += [""] + root_block(fspec) + [""]
        else:
            out += sg + [""]
    while out and out[-1] == "":
        out.pop()
    text = "\n".join(out) + "\n"
    for a, b in fspec.root_replace:
        assert text.count(a) == 1, f"root replace anchor not unique: {a!r}"
        text = text.replace(a, b)
    return text


def emit_all(files, outroot, log):
    written = {}
    for f in files:
        base = os.path.dirname(f.rel)
        for c in f.ctxs[1:]:
            written[os.path.join(base, c.dirname, "🦀️.rs")] = child_text(c)
        written[f.rel] = root_text(f)
    for rel, text in written.items():
        p = os.path.join(outroot, rel)
        write(p, text)
    return written


def run_group(files, outroot, srcroot=None, log=print):
    for f in files:
        load(f, read(os.path.join(srcroot or ROOT, f.rel)))
        make_ctxs(f)
    allctxs = [c for f in files for c in f.ctxs]
    for c in allctxs:
        analyze(c)
    for c in allctxs:
        widen(c, allctxs)
    for c in allctxs:
        analyze(c)
    tmeth = trait_method_table(files)
    for f in files:
        for c in f.ctxs:
            resolve_imports(c, f, tmeth)
    return emit_all(files, outroot, log)
