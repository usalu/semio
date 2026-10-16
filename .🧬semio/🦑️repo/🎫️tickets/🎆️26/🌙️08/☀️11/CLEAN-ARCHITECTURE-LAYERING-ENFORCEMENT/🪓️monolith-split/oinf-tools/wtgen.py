"""Phase 3: split the world tests along crate lines into per-crate integration test targets."""
import sys, os, re, collections, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wgen import *
from model import INF

VIRT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite"
LIMIT = 3000
TEST_DIRS = ['🔬️unit','📜️journal-sequences','📇️surface-verbs','🖱️pointer-gestures','🧲️scene-input-residency','🌉️bridge-empty-build']
PART_EMOJI = ['🔬️', '🔭️', '🧪️', '🧮️', '🧰️']

class TItem:
    pass

def load_pool():
    pool = {}   # (dir, idx) -> TItem
    byfile = {}
    for d in TEST_DIRS:
        p = f"{INF}/🌍️world/🧪️tests/{d}/🦀️.rs"
        text = open(p, encoding='utf-8').read()
        items, _ = chunk_file(text)
        byfile[d] = (p, text, items)
        for idx, it in enumerate(items):
            t = TItem(); t.dir = d; t.idx = idx; t.it = it; t.text = text[it.full_start:it.full_end]
            t.lines = text.count('\n', it.full_start, it.full_end)
            t.is_test = any(a.startswith('#[test') for a in it.attrs)
            pool[(d, idx)] = t
    return pool, byfile

def file_imports(byfile):
    """imports declared by the test files themselves (non-super)."""
    imp = collections.defaultdict(list)
    for d, (p, text, items) in byfile.items():
        for it in items:
            if it.kind != 'use': continue
            attrs, vis, ents = parse_use_item(it.text)
            for e in ents:
                if e['glob'] or e['path'][0] in ('super', 'crate', 'self'): continue
                imp[e['local']].append(dict(attrs=tuple(attrs), vis=vis, path=e['path'], local=e['local'], src=d))
    return imp

def test_used_sets(pool):
    plain = set(); dots = set(); fields = set(); labels = set(); assoc = set()
    for t in pool.values():
        it = t.it
        plain |= it.plain; dots |= it.dots; fields |= it.fields
        for (a, b) in it.assoc: assoc.add(b); plain.add(a)
        labels |= set(re.findall(r'\b([a-z_][a-z0-9_]*)\s*:(?!:)', t.text))
    return dict(plain=plain, dots=dots, fields=fields, labels=labels, assoc=assoc)

def classify(pool, byfile, wdef):
    local = {}
    for (d, idx), t in pool.items():
        if t.it.name and t.it.kind in ('fn','struct','enum','const','static','type','trait'): local.setdefault(d, {})[t.it.name] = (d, idx)
    unit_names = local.get('🔬️unit', {})
    def resolve(d, name):
        if name in local.get(d, {}): return local[d][name]
        if name in unit_names: return unit_names[name]
        return None
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
    for key in pool: t = pool[key]; t.crate = crate_of(key)
    return resolve, deps

def closure(keys, deps):
    seen = set(); stack = list(keys)
    while stack:
        k = stack.pop()
        if k in seen: continue
        seen.add(k)
        stack.extend(deps[k])
    return seen


def rerel(text, old_dir, new_dir):
    def sub(m):
        rel = m.group(2)
        absp = os.path.normpath(os.path.join(old_dir, rel))
        return f'{m.group(1)}"{os.path.relpath(absp, new_dir)}"'
    return re.sub(r'(include_(?:str|bytes)!\(\s*)"([^"]+)"', sub, text)

def build_tests(P, crates, imports, defcfg):
    pool, byfile = load_pool()
    wdef = {}
    for c in crates:
        for nm in c.defs: wdef[nm] = c.idx
    resolve, deps = classify(pool, byfile, wdef)
    fimp = file_imports(byfile)
    # special items: Drop impl for World3dState in tests, WORLD_SOURCE consts
    special_drop = [k for k, t in pool.items() if t.it.kind == 'impl' and t.it.impl_ty == 'World3dState' and t.it.impl_trait == 'Drop']
    roots = collections.defaultdict(list)    # (crate, dir) -> test keys
    for k, t in pool.items():
        if t.is_test: roots[(t.crate, k[0])].append(k)
    reach_all = closure([k for k, t in pool.items() if t.is_test], deps)
    unreached = [k for k, t in pool.items() if k not in reach_all and t.it.kind not in ('use', 'mod') and k not in special_drop]
    targets = []
    for (c, d), keys in sorted(roots.items()):
        keys.sort(key=lambda k: k[1])
        parts = []; cur = []
        for k in keys:
            trial = cur + [k]
            lines = sum(pool[x].lines for x in closure(trial, deps) if x not in special_drop)
            if cur and lines > LIMIT:
                parts.append(cur); cur = [k]
            else: cur = trial
        if cur: parts.append(cur)
        for pi, part in enumerate(parts):
            targets.append(dict(crate=c, dir=d, part=pi, nparts=len(parts), tests=part))
    return dict(pool=pool, byfile=byfile, deps=deps, imports_by_file=fimp, targets=targets, unreached=unreached, special_drop=special_drop, resolve=resolve)

if __name__ == '__main__':
    P = plan(); crates, ic, imports = build_crates(P)
    defcfg = {}
    T = build_tests(P, crates, imports, defcfg)
    pool = T['pool']
    print('targets', len(T['targets']))
    tot = 0
    for tg in T['targets']:
        cl = [x for x in closure(tg['tests'], T['deps']) if x not in T['special_drop']]
        lines = sum(pool[x].lines for x in cl)
        tot += lines
        print(CRATES[tg['crate']][0], tg['dir'], tg['part'], len(tg['tests']), 'tests', lines, 'lines', f"(helpers {sum(pool[x].lines for x in cl if not pool[x].is_test)})")
    print('total with duplication', tot, 'unreached', [(pool[k].dir, pool[k].it.kind, pool[k].it.name) for k in T['unreached']][:20])


READ_HELPERS = """/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.
fn read_static(path_from_manifest: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path_from_manifest);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_str())
}

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(read_static(path));
    }
    Box::leak(joined.into_boxed_str())
}
"""

INC_RE = re.compile(r'include_str!\(\s*"([^"]+)"\s*\)')

def to_runtime(texts, new_dir_v, pkg_dir_v):
    """convert include_str! (relative to new_dir_v) into run-time reads relative to the manifest dir; returns (texts, used)."""
    def mrel(rel):
        absp = os.path.normpath(os.path.join(new_dir_v, rel))
        return os.path.relpath(absp, pkg_dir_v)
    const_names = []
    out = []
    used = False
    for t in texts:
        m = re.match(r'(?s)(.*?)(?:pub(?:\([^)]*\))?\s+)?const\s+([A-Z0-9_]+)\s*:\s*&(?:\'static\s+)?str\s*=\s*(.*?);\s*$', t)
        if m and 'include_str!' in m.group(3):
            lead, name, rhs = m.group(1), m.group(2), m.group(3)
            paths = [mrel(x) for x in INC_RE.findall(rhs)]
            if rhs.lstrip().startswith('concat!'):
                body = 'read_joined(&[' + ', '.join(f'"{x}"' for x in paths) + '])'
            else:
                body = f'read_static("{paths[0]}")'
            const_names.append(name)
            t = f"{lead}#[allow(non_snake_case)]\nfn {name}() -> &'static str {{\n    {body}\n}}"
            used = True
        else:
            if 'include_str!' in t:
                t = INC_RE.sub(lambda mm: f'read_static("{mrel(mm.group(1))}")', t); used = True
        out.append(t)
    if const_names:
        rx = re.compile(r'(?<![A-Za-z0-9_])(' + '|'.join(map(re.escape, const_names)) + r')(?![A-Za-z0-9_(])')
        res = []
        for t in out:
            if re.search(r'fn (' + '|'.join(map(re.escape, const_names)) + r')\(\)', t):
                head, sep, tail = t.partition('{')
                res.append(head + sep + rx.sub(r'\1()', tail))
            else:
                res.append(rx.sub(r'\1()', t))
        out = res
    return out, used


def target_dirname(tg):
    base = tg['dir']
    if tg['part'] == 0: return base
    emoji = PART_EMOJI[tg['part']]
    return f"{emoji}unit-{tg['part'] + 1}"

def world_source_const(crates):
    return None

def gen_target(T, tg, crates, imports, defcfg, wdef, outroot):
    pool = T['pool']; deps = T['deps']
    c = crates[tg['crate']]
    keep = set(T['special_drop']) if tg.get('keep_special') else set()
    keys = sorted((k for k in closure(list(tg['tests']) + list(tg.get('extra', [])), deps) if k not in T['special_drop'] or k in keep), key=lambda k: ((0 if k[0] == '🔬️unit' else 1, TEST_DIRS.index(k[0])), k[1]))
    new_dir = tg.get('newdir') or f"{outroot}/🌍️world/{c.folder}/🧪️tests/{target_dirname(tg)}"
    bodies = []; plain = set(); dirs_used = set()
    for k in keys:
        t = pool[k]; dirs_used.add(k[0])
        old_dir = f"{VIRT}/🌍️world/🧪️tests/{k[0]}"
        txt = strip_inner_docs(t.text)
        txt = normalize_vis(txt)
        txt = txt.replace('crate::os_spr', 'semio_framework_os_kernel::os_spr')
        txt = rerel(txt, old_dir, f"{VIRT}/🌍️world/{c.folder}/🧪️tests/{os.path.basename(new_dir)}" if not tg.get('newdir') else f"{VIRT}/🌍️world/{c.folder}/🧰️testkit")
        if t.it.kind == 'const' and t.it.name == 'WORLD_SOURCE':
            vnew = f"{VIRT}/🌍️world/{c.folder}/🧪️tests/{os.path.basename(new_dir)}"
            paths = [f"{os.path.relpath(f'{VIRT}/🌍️world/{cc.folder}/🦀️.rs', vnew)}" for cc in crates]
            body = ',\n'.join(f'    include_str!("{p}")' for p in paths)
            txt = f"const WORLD_SOURCE: &str = concat!(\n{body}\n);"
        bodies.append(txt.strip('\n'))
        plain |= set(t.it.plain) | {a for (a, b) in t.it.assoc}
    # imports
    cross = collections.defaultdict(list); ext = []; seen = set()
    localnames = {pool[k].it.name for k in keys if pool[k].it.name}
    for nm in sorted(plain):
        if nm in localnames: continue
        if nm in wdef and wdef[nm] <= tg['crate']:
            cross[wdef[nm]].append(nm)
        else:
            srcs = list(imports.get(nm, []))
            for d in dirs_used: srcs += [e for e in T['imports_by_file'].get(nm, []) if e['src'] == d]
            for e in srcs:
                key = (e['attrs'], tuple(e['path']), e['local'])
                if key in seen: continue
                seen.add(key)
                ext.append(dict(attrs=tuple(e['attrs']), path=e['path'], local=e['local']))
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
    pkg_v = f"{VIRT}/🌍️world/{c.folder}/📦️packages/🦀️rust"
    newdir_v = f"{VIRT}/🌍️world/{c.folder}/🧰️testkit" if tg.get('newdir') else f"{VIRT}/🌍️world/{c.folder}/🧪️tests/{os.path.basename(new_dir)}"
    bodies, used_rt = to_runtime(bodies, newdir_v, pkg_v)
    if used_rt: bodies.insert(0, READ_HELPERS.strip('\n'))
    head = tg.get('head') or f"//! 🧪️ {c.id} tests ({target_dirname(tg)}): compiled against the `testkit` surface of the world crates.\n\n"
    text = head + '\n'.join(uses) + '\n\n' + '\n\n'.join(bodies) + '\n'
    os.makedirs(new_dir, exist_ok=True)
    open(new_dir + '/🦀️.rs', 'w', encoding='utf-8').write(text)
    return new_dir + '/🦀️.rs', text
