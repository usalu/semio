#!/usr/bin/env python3
"""🔎️ Crate-boundary scans on the carved model: orphan/inherent-impl placement, and cross-crate use of non-pub fields and methods."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve
from carve import Model, impl_self_ident, inline_descendants, rel

PRIMS = {'Vec', 'String', 'Option', 'Result', 'Box', 'Arc', 'Rc', 'HashMap', 'BTreeMap', 'HashSet', 'BTreeSet', 'str', 'u8', 'u16', 'u32', 'u64', 'i64', 'i32', 'bool', 'usize', 'f64', 'f32', 'Self', 'Cow', 'Duration', 'Instant', 'PathBuf', 'Path', 'Pin', 'Mutex', 'RwLock'}


def impl_header(it):
    toks = [t for t in it.toks if t.k != 'comment']
    i = 0
    while i < len(toks) and toks[i].x != 'impl':
        i += 1
    i += 1
    if i < len(toks) and toks[i].x == '<':
        d = 0
        while i < len(toks):
            if toks[i].x == '<':
                d += 1
            elif toks[i].x == '>':
                d -= 1
                if d == 0:
                    i += 1
                    break
            i += 1
    j = i
    d = 0
    for_idx = None
    while j < len(toks):
        x = toks[j].x
        if x == '<':
            d += 1
        elif x == '>':
            d -= 1
        elif d == 0 and (x == '{' or (x == 'where' and toks[j].k == 'id')):
            break
        elif d == 0 and x == 'for' and toks[j].k == 'id':
            for_idx = j
        j += 1
    if for_idx is None:
        return None, toks[i:j]
    return toks[i:for_idx], toks[for_idx + 1:j]


def first_path(toks):
    segs = []
    k = 0
    while k < len(toks):
        t = toks[k]
        if t.k == 'id' and t.x not in ('dyn', 'mut', 'const', 'unsafe'):
            segs.append(t.x)
            k += 1
            while k + 2 < len(toks) + 0 and k + 1 < len(toks) and toks[k].x == ':' and toks[k + 1].x == ':' and k + 2 < len(toks) and toks[k + 2].k == 'id':
                segs.append(toks[k + 2].x)
                k += 3
            return segs
        k += 1
    return segs


def crate_of_res(model, res):
    if res is None or res.kind not in ('item', 'mod', 'variant') or res.mod is None:
        return None
    return model.crate_ident_of(res.mod.file_root())


def orphan_scan(model):
    out = []
    for ident, root in model.roots.items():
        for fm in hcmod.walk(root):
            if fm.inline or getattr(fm, 'missing', False):
                continue
            for S in [fm] + list(inline_descendants(fm)):
                for it in S.all_items:
                    if it.kind != 'impl':
                        continue
                    trait, selft = impl_header(it)
                    spath = first_path(selft)
                    if not spath:
                        continue
                    hcmod.CRATES.clear(); hcmod.CRATES.update(model.roots)
                    sres = hcmod.resolve(S, spath)
                    scrate = crate_of_res(model, sres)
                    if trait is None:
                        if scrate is not None and scrate != ident:
                            out.append(('inherent-impl-on-foreign-type', ident, rel(fm.file), '::'.join(spath), scrate))
                        elif scrate is None and spath[-1] not in ('Self',):
                            out.append(('inherent-impl-on-external-type', ident, rel(fm.file), '::'.join(spath), ''))
                    else:
                        tpath = first_path(trait)
                        tres = hcmod.resolve(S, tpath) if tpath else None
                        tcrate = crate_of_res(model, tres)
                        local_trait = tcrate == ident
                        local_self = scrate == ident
                        for tt in list(trait) + list(selft):
                            if tt.k == 'id' and tt.x not in PRIMS and tt.x not in ('for', 'dyn', 'mut', 'const'):
                                rr = hcmod.resolve(S, [tt.x])
                                if rr is not None and crate_of_res(model, rr) == ident:
                                    local_self = True
                        generic_self = len(spath) == 1 and len(spath[0]) == 1 and spath[0].isupper()
                        if not local_trait and not local_self and not generic_self:
                            out.append(('orphan-trait-impl', ident, rel(fm.file), '::'.join(tpath) + ' for ' + '::'.join(spath), f'trait={tcrate} self={scrate}'))
    return out


# --------------------------------------------------------------- member visibility scan
def _skip_attrs(toks, j):
    while toks[j].x == '#' and toks[j].k == 'p':
        d = 0
        j += 1
        while True:
            if toks[j].x == '[':
                d += 1
            elif toks[j].x == ']':
                d -= 1
                if d == 0:
                    j += 1
                    break
            j += 1
    return j


def _vis_at(toks, j):
    """Return (vis string, first index, index after) of a visibility starting at toks[j] (or empty at j)."""
    if toks[j].x == 'pub' and toks[j].k == 'id':
        k = j + 1
        if toks[k].x == '(':
            e = k
            while toks[e].x != ')':
                e += 1
            return 'pub' + ''.join(x.x for x in toks[k:e + 1]), j, e + 1
        return 'pub', j, j + 1
    return '', j, j


def struct_members(item):
    """Return list of (name, vis, vis_start_off, vis_end_off, name_off) for fields of a struct item."""
    toks = [t for t in item.toks if t.k != 'comment']
    out = []
    i = 0
    while i < len(toks) and toks[i].x != 'struct':
        i += 1
    i += 2
    if i < len(toks) and toks[i].x == '<':
        d = 0
        while i < len(toks):
            if toks[i].x == '<':
                d += 1
            elif toks[i].x == '>':
                d -= 1
                if d == 0:
                    i += 1
                    break
            i += 1
    while i < len(toks) and toks[i].x not in ('{', '(', ';'):
        i += 1
    if i >= len(toks):
        return out
    if toks[i].x == '{':
        named = True
    elif toks[i].x == '(':
        named = False
    else:
        return out
    i += 1
    depth = 1
    idx = 0
    field_start = True
    n = len(toks)
    while i < n and depth > 0:
        t = toks[i]
        if depth == 1 and field_start and (t.k == 'id' or t.x == '#' or (not named and t.x not in (')', ',') and t.k != 'comment')):
            j = _skip_attrs(toks, i)
            vis, vs, ve = _vis_at(toks, j)
            if named:
                nm_tok = toks[ve]
                out.append((nm_tok.x, vis, toks[vs].a if vis else nm_tok.a, toks[ve - 1].b if vis else nm_tok.a, nm_tok.a))
            else:
                out.append((str(idx), vis, toks[vs].a if vis else toks[ve].a, toks[ve - 1].b if vis else toks[ve].a, toks[ve].a))
                idx += 1
            field_start = False
            i = ve + 1 if named else ve
            continue
        if t.k == 'p':
            if t.x in '({[':
                depth += 1
            elif t.x in ')}]':
                depth -= 1
            elif t.x == '<':
                depth += 1
            elif t.x == '>' and not (i > 0 and toks[i - 1].x in ('-', '=')):
                depth -= 1
            elif t.x == ',' and depth == 1:
                field_start = True
        i += 1
    return out


def impl_methods(it):
    """Return (name, vis, vis_start_off, vis_end_off, fn_off) of fns/consts at depth 1 of an impl body."""
    toks = [t for t in it.toks if t.k != 'comment']
    out = []
    i = 0
    while i < len(toks) and toks[i].x != '{':
        i += 1
    depth = 0
    n = len(toks)
    k = i
    while k < n:
        t = toks[k]
        if t.x == '{' and t.k == 'p':
            depth += 1
        elif t.x == '}' and t.k == 'p':
            depth -= 1
        elif depth == 1 and t.k == 'id' and t.x in ('fn', 'const') and k + 1 < n and toks[k + 1].k == 'id' and toks[k + 1].x not in ('fn', 'unsafe', 'async', 'extern'):
            nm = toks[k + 1].x
            j = k - 1
            while j >= 0 and toks[j].x in ('async', 'unsafe', 'const', 'extern', 'default') and toks[j].k == 'id':
                j -= 1
            vis = ''
            vs = ve = toks[j + 1].a if j + 1 < n else t.a
            if j >= 0 and toks[j].x == 'pub' and toks[j].k == 'id':
                vis, vs, ve = 'pub', toks[j].a, toks[j].b
            elif j >= 0 and toks[j].x == ')':
                m = j
                while m >= 0 and toks[m].x != '(':
                    m -= 1
                if m >= 1 and toks[m - 1].x == 'pub':
                    vis = 'pub' + ''.join(x.x for x in toks[m:j + 1])
                    vs, ve = toks[m - 1].a, toks[j].b
            out.append((nm, vis, vs, ve, toks[k].a))
        k += 1
    return out


def member_scan(model, only_nontest_owner=True):
    """Report non-pub fields/methods of types defined in crate O that files of other crates use by name (receiver type is mentioned in the consumer crate)."""
    import regs
    owners = collections.defaultdict(list)
    for ident, root in model.roots.items():
        for fm in hcmod.walk(root):
            if fm.inline or getattr(fm, 'missing', False):
                continue
            if only_nontest_owner and regs.mod_is_test_only(fm):
                continue
            for S in [fm] + list(inline_descendants(fm)):
                for it in S.all_items:
                    if only_nontest_owner and any(not __import__('cfgeval').holds(c, test=False) for c in it.cfg):
                        continue
                    if it.kind == 'struct':
                        for nm, vis, vs, ve, no in struct_members(it):
                            if vis != 'pub':
                                owners[nm].append(dict(crate=ident, type=it.name, kind='field', vis=vis, file=fm.file, vs=vs, ve=ve, at=no))
                    elif it.kind == 'impl':
                        trait, selft = impl_header(it)
                        if trait is not None:
                            continue
                        sp = first_path(selft)
                        if not sp:
                            continue
                        for nm, vis, vs, ve, off in impl_methods(it):
                            if vis != 'pub':
                                owners[nm].append(dict(crate=ident, type=sp[-1], kind='method', vis=vis, file=fm.file, vs=vs, ve=ve, at=off))
    uses = collections.defaultdict(lambda: collections.defaultdict(set))
    typeuse = collections.defaultdict(set)
    ctoruse = collections.defaultdict(set)
    for ident, root in model.roots.items():
        for fm in hcmod.walk(root):
            if fm.inline or getattr(fm, 'missing', False):
                continue
            toks = fm.toks
            for i, t in enumerate(toks):
                if t.k == 'num' and i > 0 and toks[i - 1].x == '.' and t.x.isdigit() and t.x in owners:
                    uses[ident][t.x].add(fm.file)
                    continue
                if t.k != 'id':
                    continue
                typeuse[ident].add(t.x)
                if t.x[:1].isupper():
                    pv = toks[i - 1].x if i > 0 else ''
                    nx = toks[i + 1].x if i + 1 < len(toks) else ''
                    if nx == '(' or (pv == '(' and nx in (')', ',')):
                        ctoruse[ident].add(t.x)
                if t.x not in owners:
                    continue
                prev = toks[i - 1].x if i > 0 else ''
                nxt = toks[i + 1].x if i + 1 < len(toks) else ''
                if prev == '.' and toks[i - 1].k == 'p':
                    uses[ident][t.x].add(fm.file)
                elif nxt == ':' and (i + 2 >= len(toks) or toks[i + 2].x != ':') and prev in ('{', ','):
                    uses[ident][t.x].add(fm.file)
                elif prev == ':' and i > 1 and toks[i - 2].x == ':':
                    uses[ident][t.x].add(fm.file)
                elif prev in ('{', ',') and nxt in (',', '}'):
                    uses[ident][t.x].add(fm.file)
    findings = []
    for consumer, names in uses.items():
        for nm, files in names.items():
            for o in owners[nm]:
                if o['crate'] == consumer:
                    continue
                if o['type'] not in typeuse[consumer]:
                    continue
                findings.append(dict(o, name=nm, consumer=consumer, files=sorted(files)))
    for consumer, tys in ctoruse.items():
        for nm, os_ in owners.items():
            if not nm.isdigit():
                continue
            for o in os_:
                if o['crate'] != consumer and o['type'] in tys:
                    findings.append(dict(o, name=nm, consumer=consumer, files=['<ctor>']))
    return findings
