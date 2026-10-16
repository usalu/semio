"""Board tests: rewrite to the decomposed host API, classify by crate, split into per-crate integration targets."""
import sys, os, re, collections, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bxfile, bgen, bcrates
from bcrates import *
from oinf_rs import *
from usetree import parse_use_item
import wgen, wtgen
from wgen import emit_use, strip_inner_docs, normalize_vis, testkit_cfg
from wtgen import closure, rerel, to_runtime, READ_HELPERS

SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
VIRT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite"
LIMIT = 3000
PART_EMOJI = ['🔬️', '🔭️', '🧪️', '🧮️', '🧰️']
B = '🎲️board/'
NR = B + '🔌️ports/➡️directed/➕️normal/'
DR = B + '🔌️ports/➡️directed/🕸️dag/'
# (rel of the original test file, target dir name, host type, receivers)
TEST_FILES = [
 (B + '🧪️tests/🔬️unit/🦀️.rs', '🔬️engine-unit', None, []),
 (B + '🔌️ports/➡️directed/🧪️tests/🔬️quadrant/🦀️.rs', '📐️quadrant', None, []),
 (B + '🔌️ports/↔️undirected/🧪️tests/🔬️unit/🦀️.rs', '↔️undirected-ports', None, []),
 (B + '➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs', '↔️undirected-normal', None, []),
 (B + '➕️normal/➡️directed/🧪️tests/🔬️unit/🦀️.rs', '➡️directed-normal', None, []),
 (B + '🧬️schema/💡️inferences/📐️layout/🕸️layered/🧪️tests/🔬️tidy-tree/🦀️.rs', '🌳️tidy-tree', None, []),
 (B + '🚪️io/📝️text/🎯️dag-input/📊️progress/🧪️tests/🦀️.rs', '📊️progress', None, []),
 (NR + '🧪️tests/🔬️board-host-standalone/🦀️.rs', '🔬️board-host', 'BoardHost', ['host']),
 (DR + '🧪️tests/🔬️unit/🦀️.rs', '🔬️dag-unit', 'DagHost', ['host']),
 (DR + '🧪️tests/🔗️wire-edit/🦀️.rs', '🔗️wire-edit', 'DagHost', ['host']),
 (DR + '🧪️tests/🧪️node-graph-edit-rows/🦀️.rs', '🛠️edit-rows', 'DagHost', ['host', 'fits', 'oversized']),
 (DR + '🎛️input/🧪️tests/🦀️.rs', '🎛️input', 'DagHost', ['host']),
]

EXCLUDE = {'board_property_retirement_credits'}

class TItem: pass

def load_pool(src_root, meta, modnames, hostfix=None):
    pool = {}; byfile = {}
    for rel, tag, host, recv in TEST_FILES:
        p = f"{src_root}/{rel}"
        text = open(p, encoding='utf-8').read()
        haz = []
        if host:
            import bxinfer
            bxinfer.FILE_EXTRA[(rel, host)] = set(recv)
            text, plan, haz, flags, bad = bxinfer.rewrite_file(text, rel, meta, bxfile.specs())
            assert not bad, (rel, bad)
        items, _ = chunk_file(text)
        byfile[tag] = (rel, text, items, haz)
        for idx, it in enumerate(items):
            if it.name in EXCLUDE: continue
            t = TItem(); t.tag = tag; t.idx = idx; t.it = it; t.text = text[it.full_start:it.full_end]
            t.lines = text.count('\n', it.full_start, it.full_end)
            t.is_test = any(a.startswith('#[test') for a in it.attrs)
            pool[(tag, idx)] = t
    return pool, byfile

def file_imports(byfile, ext_roots):
    imp = collections.defaultdict(list)
    LR = bgen.local_roots()
    for tag, (rel, text, items, haz) in byfile.items():
        for it in items:
            if it.kind != 'use': continue
            attrs, vis, ents = parse_use_item(it.text)
            attrs = [a for a in attrs if not a.startswith('#!')]
            for e in ents:
                p = list(e['path'])
                if e['glob']: continue
                if p[0] in LR:
                    k = 0
                    while k < len(p) and p[k] in LR: k += 1
                    rest = p[k:]
                    if rest and (rest[0] in bgen.KERNEL_ALIAS or bgen.ALIAS.get(rest[0], rest[0]) in ext_roots): p = rest
                    else: continue
                elif p[0] == 'crate': continue
                p = [bgen.ALIAS.get(p[0], p[0])] + p[1:]
                if p[0] in bgen.KERNEL_ALIAS: p = ['semio_framework_os_kernel'] + p
                imp[e['local']].append(dict(attrs=tuple(attrs), path=p, local=e['local'], src=tag))
    return imp

def test_used_sets(pool):
    plain = set(); dots = set(); fields = set(); labels = set(); assoc = set()
    for t in pool.values():
        it = t.it
        plain |= it.plain; dots |= it.dots; fields |= it.fields
        for (a, b) in it.assoc: assoc.add(b); plain.add(a)
        labels |= set(re.findall(r'\b([a-z_][a-z0-9_]*)\s*:(?!:)', t.text))
    return dict(plain=plain, dots=dots, fields=fields, labels=labels, assoc=assoc)

def classify(pool, wdef):
    local = {}
    for (tag, idx), t in pool.items():
        if t.it.name and t.it.kind in ('fn', 'struct', 'enum', 'const', 'static', 'type', 'trait', 'macro_rules'): local.setdefault(tag, {})[t.it.name] = (tag, idx)
    def resolve(tag, name):
        return local.get(tag, {}).get(name)
    memo = {}
    def crate_of(key, stack=()):
        if key in memo: return memo[key]
        memo[key] = 0
        t = pool[key]; c = 0
        names = set(t.it.plain) | {a for (a, b) in t.it.assoc}
        for idn in names:
            r = resolve(key[0], idn)
            if r is not None and r != key and r not in stack: c = max(c, crate_of(r, stack + (key,)))
            elif r is None and idn in wdef: c = max(c, wdef[idn])
        memo[key] = c
        return c
    deps = {}
    for key, t in pool.items():
        names = set(t.it.plain) | {a for (a, b) in t.it.assoc}
        deps[key] = [resolve(key[0], n) for n in names if resolve(key[0], n) is not None and resolve(key[0], n) != key]
    for key in pool: pool[key].crate = crate_of(key)
    return deps

def build_tests(src_root, meta, crates, imports, defcfg, ext_roots, modnames):
    pool, byfile = load_pool(src_root, meta, modnames)
    wdef = {}
    for c in crates:
        for nm in c.defs: wdef[nm] = c.idx
    deps = classify(pool, wdef)
    fimp = file_imports(byfile, ext_roots)
    roots = collections.defaultdict(list)
    for k, t in pool.items():
        if t.is_test: roots[(t.crate, k[0])].append(k)
    reach_all = closure([k for k, t in pool.items() if t.is_test], deps)
    unreached = [k for k, t in pool.items() if k not in reach_all and t.it.kind not in ('use', 'mod', 'attr-only', 'macro_call')]
    targets = []
    for (c, d), keys in sorted(roots.items()):
        keys.sort(key=lambda k: k[1])
        parts = []; cur = []
        for k in keys:
            trial = cur + [k]
            lines = sum(pool[x].lines for x in closure(trial, deps))
            if cur and lines > LIMIT: parts.append(cur); cur = [k]
            else: cur = trial
        if cur: parts.append(cur)
        for pi, part in enumerate(parts): targets.append(dict(crate=c, dir=d, part=pi, nparts=len(parts), tests=part))
    return dict(pool=pool, byfile=byfile, deps=deps, imports_by_file=fimp, targets=targets, unreached=unreached)

def target_dirname(tg):
    if tg['part'] == 0: return tg['dir']
    return f"{PART_EMOJI[tg['part']]}{tg['dir'].split('️', 1)[-1]}-{tg['part'] + 1}"

def gen_target(T, tg, crates, imports, defcfg, wdef, outroot, modnames, src_files):
    pool = T['pool']; deps = T['deps']
    c = crates[tg['crate']]
    keys = sorted(closure(list(tg['tests']), deps), key=lambda k: (k[0], k[1]))
    dname = target_dirname(tg)
    new_dir = f"{outroot}/{c.folder}/🧪️tests/{dname}"
    bodies = []; plain = set(); dirs_used = set()
    for k in keys:
        t = pool[k]; dirs_used.add(k[0])
        rel = T['byfile'][k[0]][0]
        old_dir = os.path.dirname(f"{VIRT}/{rel}")
        txt = strip_inner_docs(t.text)
        txt = re.sub(r'(?m)^[ \t]*#!\[.*\][ \t]*\n', '', txt)
        txt = normalize_vis(txt)
        txt = bgen.flatten_paths(txt, modnames, frozenset(imports))
        txt = txt.replace('crate::os_spr', 'semio_framework_os_kernel::os_spr')
        txt = rerel(txt, old_dir, f"{VIRT}/{c.folder}/🧪️tests/{dname}")
        bodies.append(txt.strip('\n'))
        plain |= set(t.it.plain) | {a for (a, b) in t.it.assoc}
    cross = collections.defaultdict(list); ext = []; seen = set()
    localnames = {pool[k].it.name for k in keys if pool[k].it.name}
    for nm in sorted(plain):
        if nm in localnames or nm == '_': continue
        if nm in wdef and wdef[nm] <= tg['crate']:
            cross[wdef[nm]].append(nm)
        else:
            srcs = []
            for d in dirs_used: srcs += [e for e in T['imports_by_file'].get(nm, []) if e['src'] == d]
            if not srcs and nm in imports:
                alts = imports[nm]
                best = sorted(alts, key=lambda p: (-len(alts[p]['mods']), p))[0]
                srcs = [dict(attrs=tuple(sorted(alts[best]['attrs'])), path=list(best), local=nm)]
            for e in srcs:
                key = (e['attrs'], tuple(e['path']), e['local'])
                if key in seen: continue
                seen.add(key); ext.append(dict(attrs=tuple(e['attrs']), path=e['path'], local=e['local']))
    uses = []
    for k in sorted(cross):
        ident = crate_ident(CRATES[k][0])
        groups = collections.OrderedDict()
        for nm in sorted(set(cross[k]), key=lambda s: s.lower()):
            variants = defcfg.get(nm, [()])
            keyset = tuple(sorted({v for v in variants}))
            if len(keyset) > 1:
                for v in keyset: groups.setdefault(v, []).append(nm)
            else: groups.setdefault(keyset[0], []).append(nm)
        for attrs, names in groups.items():
            uses += list(attrs)
            uses.append(f"use {ident}::{{{', '.join(names)}}};" if len(names) > 1 else f"use {ident}::{names[0]};")
    uses += emit_use(ext)
    gl = []; gseen = set()
    for d in sorted(dirs_used):
        r = T['byfile'][d][0]
        cand = r[:r.index('/🧪️tests/')] + '/🦀️.rs'
        for g in bgen.LAST_EFF_GLOBS.get(cand, []):
            key = (tuple(g['attrs']), tuple(g['path']))
            if key in gseen: continue
            gseen.add(key); gl.append(g)
    for g in gl:
        uses += [testkit_cfg(a) for a in g['attrs']]
        uses.append('use ' + '::'.join(g['path']) + '::*;')
    pkg_v = f"{VIRT}/{c.folder}/📦️packages/🦀️rust"
    bodies, used_rt = to_runtime(bodies, f"{VIRT}/{c.folder}/🧪️tests/{dname}", pkg_v)
    fill_lib = f"{VIRT}/{next(x for x in bcrates.CRATES if x[0] == 'normal-fill')[2]}/🦀️.rs"
    fill_rel = os.path.relpath(fill_lib, pkg_v)
    bodies = [re.sub(r'read_joined\(&\[[^\]]*\]\)', f'read_static("{fill_rel}")', b) if 'fn BOARD_HOST_SOURCE()' in b else b for b in bodies]
    helpers = READ_HELPERS.strip('\n')
    if not any('read_joined(' in b for b in bodies):
        helpers = helpers.split('\n\n///', 1)[0]
    if used_rt: bodies.insert(0, helpers)
    head = f"//! 🧪️ {c.id} tests ({dname}): compiled against the `testkit` surface of the board crates.\n\n"
    text = head + '\n'.join(uses) + '\n\n' + '\n\n'.join(bodies) + '\n'
    os.makedirs(new_dir, exist_ok=True)
    open(new_dir + '/🦀️.rs', 'w', encoding='utf-8').write(text)
    return new_dir + '/🦀️.rs', text
