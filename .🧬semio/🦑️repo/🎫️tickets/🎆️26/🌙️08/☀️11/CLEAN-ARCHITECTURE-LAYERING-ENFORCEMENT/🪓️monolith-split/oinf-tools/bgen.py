"""Assemble the board crates from the planned stage tree (item-level, name-keyed imports)."""
import sys, os, re, collections, json, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import oinf_rs
from oinf_rs import *
from usetree import parse_use_item
import wgen
from wgen import testkit_cfg, strip_inner_docs, normalize_vis, used_idents, emit_use, rewrite_vis, chunk_text as _unused
from bcrates import *

ALIAS = {'canvas': 'semio_framework_canvas'}
KERNEL_ALIAS = {'os_spr', 'os_dsl', 'os_store', 'os_vcs', 'os_pack'}
LOCAL_SEG = {'super', 'self', 'crate', 'infinite'}

def local_roots(): return oinf_rs.LOCAL_ROOTS

def module_cfgs(tree):
    out = {}
    for e in tree:
        f = e['file']
        if not f: continue
        cf = [c for c in e['cfgs'] if c != '#[cfg(test)]']
        out[f] = cf
    return out

def external_roots(mods):
    roots = set()
    for rel, m in mods.items():
        for it in m.items:
            if it.kind != 'use': continue
            attrs, vis, ents = parse_use_item(it.text)
            for e in ents:
                p = e['path']
                if p[0] not in local_roots(): roots.add(ALIAS.get(p[0], p[0]))
    return roots

def collect_imports(mods, ext_roots):
    """local name -> {canonical path tuple -> dict(attrs, mods)}."""
    imp = collections.defaultdict(dict)
    globs = collections.defaultdict(list)
    LR = local_roots()
    for rel, m in mods.items():
        if not rel.startswith('🎲️board'): continue
        for it in m.items:
            if it.kind != 'use': continue
            attrs, vis, ents = parse_use_item(it.text)
            attrs = [a for a in attrs if not a.startswith('#!')]
            for e in ents:
                p = list(e['path'])
                if e['glob']:
                    if p[0] not in LR and ALIAS.get(p[0], p[0]) in ext_roots:
                        globs[rel].append(dict(attrs=tuple(attrs), path=[ALIAS.get(p[0], p[0])] + p[1:]))
                    continue
                if p[0] in LR:
                    k = 0
                    while k < len(p) and p[k] in LR: k += 1
                    rest = p[k:]
                    if not rest: continue
                    if rest[0] in KERNEL_ALIAS: p = rest
                    elif ALIAS.get(rest[0], rest[0]) in ext_roots: p = rest
                    else: continue
                elif p[0] == 'crate': continue
                p = [ALIAS.get(p[0], p[0])] + p[1:]
                if p[0] in KERNEL_ALIAS: p = ['semio_framework_os_kernel'] + p
                key = tuple(p)
                d = imp[e['local']].setdefault(key, dict(attrs=None, mods=set()))
                d['attrs'] = set(attrs) if d['attrs'] is None else (d['attrs'] & set(attrs))
                d['mods'].add(rel)
    return imp, globs

PATH_FLAT = re.compile(r'(?<![A-Za-z0-9_])(?<!::)(?:crate::)?infinite::board::(?:[a-z_][a-z0-9_]*::)*')
MOD_CHAIN = None

def flatten_paths(t, modnames, imported=frozenset()):
    t = PATH_FLAT.sub('', t)
    t = re.sub(r'(?<![A-Za-z0-9_])(?<!::)crate::os_(spr|dsl|store|vcs|pack)::', lambda m: 'semio_framework_os_kernel::os_' + m.group(1) + '::', t)
    t = re.sub(r'(?<![A-Za-z0-9_])(?<!::)crate::', '', t)
    t = re.sub(r'(?<![A-Za-z0-9_])(?<!::)(?:(?:super|self)::)+', '', t)
    modnames = {x for x in modnames if x not in imported}
    if modnames:
        rx = re.compile(r'(?<![A-Za-z0-9_])(?<!::)(?:(?:' + '|'.join(sorted(map(re.escape, modnames))) + r')::)+(?=[A-Za-z_])')
        t = rx.sub('', t)
    return t

LAST_EFF_GLOBS = {}
class Crate: pass

def item_chunk(m, idx, modcfg, modnames, imported=frozenset()):
    it = m.items[idx]
    t = m.text[it.full_start:it.full_end]
    t = strip_inner_docs(t)
    t = re.sub(r'(?m)^[ \t]*#!\[.*\][ \t]*\n', '', t)
    t = testkit_cfg(t)
    t = re.sub(r'#\[cfg_attr\(test,', '#[cfg_attr(any(test, feature = "testkit"),', t)
    t = normalize_vis(t)
    t = flatten_paths(t, modnames, imported)
    return t, it

def plan_items(P, uc, ic):
    mods = P['mods']
    tycrate = {}
    for (rel, idx), c in ic.items():
        it = mods[rel].items[idx]
        if it.kind in ('struct', 'enum', 'union', 'type', 'trait') and it.name: tycrate.setdefault(it.name, c)
    ic = dict(ic)
    for (rel, idx), c in list(ic.items()):
        it = mods[rel].items[idx]
        if it.kind == 'macro_call':
            tys = [t for t in sorted(it.plain) if t in tycrate]
            cs = {tycrate[t] for t in tys}
            if len(cs) != 1: print('MACRO placement', rel[-40:], it.name, tys, cs)
            if cs: ic[(rel, idx)] = max(cs)
    return ic

def build(P, uc, ic, extra_used=None, tree=None):
    mods = {rel: m for rel, m in P['mods'].items() if rel.startswith('🎲️board')}
    LR = local_roots()
    modnames = {x for x in LR if x not in LOCAL_SEG}
    ext_roots = external_roots(P['mods'])
    imports, globs = collect_imports(P['mods'], ext_roots)
    ic = plan_items(P, uc, ic)
    import modtree
    mp = {}
    for e in (tree or []):
        if e['file']: mp[e['file'][len(modtree.INFROOT):]] = tuple(e['modpath'])
    eff_globs = {}
    for rel in mods:
        pth = mp.get(rel, ())
        acc = []
        for r2, gl2 in globs.items():
            p2 = mp.get(r2, ())
            if p2 and pth[:len(p2)] == p2: acc += gl2
        eff_globs[rel] = acc
    global LAST_EFF_GLOBS
    LAST_EFF_GLOBS = eff_globs
    mcfg = module_cfgs(tree) if tree else {}
    crates = []
    for ci, (cid, bands, folder) in enumerate(CRATES):
        c = Crate(); c.idx = ci; c.id = cid; c.folder = folder; c.items = []
        for (rel, idx), k in ic.items():
            if k == ci and rel in mods: c.items.append((rel, idx))
        c.items.sort(key=lambda x: (x[0], x[1]))
        crates.append(c)
    byname = {}; defcfg = {}
    for c in crates:
        c.defs = {}
        for rel, idx in c.items:
            it = mods[rel].items[idx]
            if it.name and it.name != '_' and it.kind in ('fn', 'struct', 'enum', 'union', 'trait', 'type', 'const', 'static', 'macro_rules'):
                if it.name in byname and byname[it.name] != c.idx: print('DUP top-level name across crates', it.name, byname[it.name], c.idx)
                byname[it.name] = c.idx; c.defs[it.name] = it.kind
                defcfg.setdefault(it.name, []).append(tuple(testkit_cfg(a) for a in (list(it.cfg) + [x for x in mcfg.get(P['abs'].get(rel), []) if x not in it.cfg])))
    for c in crates:
        c.plain = set(); c.dots = set(); c.fields = set(); c.labels = set(); c.assoc = set()
        per = collections.defaultdict(list)
        for rel, idx in c.items: per[rel].append(idx)
        for rel, idxs in per.items():
            p, d, f, l, a = used_idents(mods[rel], idxs)
            c.plain |= p; c.dots |= d; c.fields |= f; c.labels |= l; c.assoc |= a
    for c in crates:
        c.ext_names = set(); c.ext_members = set()
        for k in crates:
            if k is c: continue
            c.ext_names |= k.plain
            c.ext_members |= k.plain | k.dots | k.fields | k.labels | k.assoc
        if extra_used:
            c.ext_names |= extra_used['plain']
            c.ext_members |= extra_used['plain'] | extra_used['dots'] | extra_used['fields'] | extra_used['labels'] | extra_used['assoc']
    upward = []
    for c in crates:
        c.body = []
        c.items = [(rel, idx) for rel, idx in c.items if not (mods[rel].items[idx].kind == 'macro_call' and mods[rel].items[idx].name == 'include')]
        for rel, idx in c.items:
            t, it = item_chunk(mods[rel], idx, mcfg, modnames, frozenset(imports))
            fpath = None
            if it.kind not in ('macro_call',):
                if it.kind in ('fn', 'struct', 'enum', 'union', 'trait', 'type', 'const', 'static', 'impl'):
                    t = rewrite_vis(t, c.ext_names, c.ext_members)
            mc = mcfg.get(P['abs'].get(rel), []) if 'abs' in P else []
            if mc: t = ''.join(a + '\n' for a in mc) + t.lstrip('\n')
            c.body.append(t.strip('\n'))
        cross = collections.defaultdict(list); ext = []; seen = set()
        for nm in sorted(c.plain):
            if nm in c.defs: continue
            if nm in byname and byname[nm] < c.idx:
                cross[byname[nm]].append(nm)
            elif nm in byname and byname[nm] > c.idx:
                upward.append((c.id, nm, CRATES[byname[nm]][0]))
            elif nm in imports:
                alts = imports[nm]
                if len(alts) > 1:
                    score = {}
                    used_mods = {rel for rel, idx in c.items}
                    for pth, d in alts.items(): score[pth] = len(d['mods'] & used_mods)
                    ordered = sorted(alts, key=lambda p: (-score[p], p))
                    keep = [ordered[0]] + [p for p in ordered[1:] if nm in ('FromValue', 'ToValue') and score[p] > 0]
                else: keep = list(alts)
                for pth in keep:
                    d = alts[pth]
                    key = (tuple(sorted(d['attrs'])), pth, nm)
                    if key in seen: continue
                    seen.add(key)
                    ext.append(dict(attrs=tuple(testkit_cfg(a) for a in sorted(d['attrs'])), path=list(pth), local=nm))
        c.cross = {k: v for k, v in cross.items() if v}
        gl = []; gseen = set()
        for rel in sorted({r for r, i in c.items}):
            for g in eff_globs.get(rel, []):
                key = (tuple(g['attrs']), tuple(g['path']))
                if key in gseen: continue
                gseen.add(key); gl.append(g)
        c.glob_use = gl
        c.ext_use = ext
        c.cross_cfg = {nm: defcfg.get(nm, [()]) for lst in c.cross.values() for nm in lst}
    return crates, ic, imports, defcfg, upward

def header_text(c):
    emoji, desc = TITLES[c.id]
    lines = [f"//! {emoji} {desc}", "//!", f"//! Crate `{crate_pkg(c.id)}`; a downward producer of the infinite board family.", ""]
    if c.id.startswith('normal-') or c.id.startswith('dag-'):
        lines += ['#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]', '#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]', ""]
    uses = []
    for k in sorted(c.cross):
        ident = crate_ident(CRATES[k][0])
        groups = collections.OrderedDict()
        for nm in sorted(set(c.cross[k]), key=lambda s: s.lower()):
            variants = c.cross_cfg.get(nm, [()])
            keyset = tuple(sorted({v for v in variants}))
            if len(keyset) > 1 or (len(keyset) == 1 and any(len(x) > 1 for x in keyset)):
                for v in keyset: groups.setdefault(v, []).append(nm)
            else: groups.setdefault(keyset[0], []).append(nm)
        for attrs, names in groups.items():
            for a in attrs: uses.append(a)
            uses.append(f"use {ident}::{{{', '.join(names)}}};" if len(names) > 1 else f"use {ident}::{names[0]};")
    uses += emit_use(c.ext_use)
    for g in c.glob_use:
        uses += [testkit_cfg(a) for a in g['attrs']]
        uses.append('use ' + '::'.join(g['path']) + '::*;')
    return '\n'.join(lines + uses + [""])

def write_crates(crates, outroot):
    for c in crates:
        d = f"{outroot}/{c.folder}"
        os.makedirs(d, exist_ok=True)
        text = header_text(c) + '\n' + '\n\n'.join(c.body) + '\n'
        c.text = text
        open(f"{d}/🦀️.rs", 'w', encoding='utf-8').write(text)
