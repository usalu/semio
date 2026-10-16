"""Phase 2: assemble world crates from the decomposed stage tree."""
import sys, os, re, collections, json, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wplan import *
from usetree import parse_use_item
from oinf_rs import *

WCFG = '#[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]'
CRATE_PREFIX = 'semio_framework_os_infinite_world_'

def crate_ident(cid): return CRATE_PREFIX + cid.replace('-', '_')
def crate_pkg(cid): return 'semio-framework-os-infinite-world-' + cid

def collect_imports(mods):
    """name -> list of dict(attrs, path, local, alias) from every world `use` item (non-test)."""
    imp = collections.defaultdict(list)
    for rel, m in mods.items():
        if not rel.startswith('🌍️world'): continue
        for it in m.items:
            if it.kind != 'use': continue
            attrs, vis, ents = parse_use_item(it.text)
            for e in ents:
                if e['glob']: continue
                path = e['path']
                if path[0] in ('super', 'crate', 'self'): continue
                local = e['local']
                imp[local].append(dict(attrs=tuple(attrs), vis=vis, path=path, local=local, src=rel))
    return imp

def top_names(m, idxs):
    out = {}
    for idx in idxs:
        it = m.items[idx]
        if it.name and it.kind in ('fn','struct','enum','union','trait','type','const','static','macro_rules'):
            out[it.name] = it.kind
    return out


def testkit_cfg(text):
    def fix(m):
        inner = m.group(1)
        new = re.sub(r'(?<![A-Za-z0-9_"=])test(?![A-Za-z0-9_"])', 'any(test, feature = "testkit")', inner)
        return '#[cfg(' + new + ')]'
    return re.sub(r'#\[cfg\(((?:[^\[\]]|\[[^\]]*\])*)\)\]', fix, text)

def strip_inner_docs(chunk):
    return '\n'.join(l for l in chunk.split('\n') if not l.lstrip().startswith('//!'))

class Crate:
    pass

def assemble(P):
    mods = P['mods']; nodes = P['nodes']
    ic = dict(P['item_crate'])
    # macro_call placement: with the type they name
    tycrate = {}
    for (rel, idx), c in ic.items():
        it = mods[rel].items[idx]
        if it.kind in ('struct','enum','union','type','trait') and it.name: tycrate.setdefault(it.name, c)
    for (rel, idx), c in list(ic.items()):
        it = mods[rel].items[idx]
        if it.kind == 'macro_call':
            tys = [t for t in sorted(it.plain) if t in tycrate]
            cs = {tycrate[t] for t in tys}
            assert len(cs) == 1, (it.name, tys, cs)
            ic[(rel, idx)] = cs.pop()
    crates = []
    root_order = {rel: i for i, rel in enumerate(sorted(mods, key=lambda r: (r != '🌍️world/🦀️.rs', r)))}
    for ci, (cid, bands, folder) in enumerate(CRATES):
        c = Crate(); c.idx = ci; c.id = cid; c.folder = folder; c.items = []
        for (rel, idx), k in ic.items():
            if k == ci: c.items.append((rel, idx))
        c.items.sort(key=lambda x: (root_order[x[0]], x[1]))
        crates.append(c)
    return crates, ic


def chunk_text(m, idx):
    it = m.items[idx]
    t = m.text[it.full_start:it.full_end]
    t = strip_inner_docs(t)
    return testkit_cfg(t)

def normalize_vis(t):
    """pub(super)/pub(in crate::...) -> pub(crate) (crate-root items cannot use super)."""
    t = re.sub(r'\bpub\(super\)', 'pub(crate)', t)
    t = re.sub(r'\bpub\(in crate::[A-Za-z0-9_:]+\)', 'pub(crate)', t)
    return t

def used_idents(m, idxs):
    plain = set(); dots = set(); fields = set(); labels = set(); assoc = set()
    for idx in idxs:
        it = m.items[idx]
        plain |= it.plain; dots |= it.dots; fields |= it.fields
        for (a, b) in it.assoc: assoc.add(b); plain.add(a)
        labels |= set(re.findall(r'\b([a-z_][a-z0-9_]*)\s*:(?!:)', m.text[it.start:it.end]))
    return plain, dots, fields, labels, assoc

def emit_use(entries):
    """entries: list of dict(attrs, path, local) -> list of use lines grouped per (attrs, parent path)."""
    groups = collections.OrderedDict()
    for e in entries:
        key = (e['attrs'], tuple(e['path'][:-1]))
        groups.setdefault(key, [])
        leaf = e['path'][-1]
        item = leaf if e['local'] == leaf else f"{leaf} as {e['local']}"
        if item not in groups[key]: groups[key].append(item)
    lines = []
    for (attrs, parent), leaves in groups.items():
        for a in attrs: lines.append(a)
        pp = '::'.join(parent)
        leaves = sorted(leaves, key=lambda s: s.lower())
        if len(leaves) == 1:
            lines.append(f"use {pp}::{leaves[0]};" if pp else f"use {leaves[0]};")
        else:
            body = ', '.join(leaves)
            lines.append(f"use {pp}::{{{body}}};")
    return lines


def struct_field_spans(text, it):
    """fields of a brace struct: list of dict(name, vis_a, vis_b, name_a). Tuple structs: positional dict(name=None)."""
    toks = [t for t in lex(text[it.start:it.end]) if t.k != 'comment']
    off = it.start
    # find the body opener after the struct name
    i = 0
    while toks[i].s != 'struct': i += 1
    i += 2
    # skip generics / where until '{' or '(' or ';'
    d = 0
    while True:
        t = toks[i]
        if t.k == 'p' and t.s == '<': d += 1
        elif t.k == 'p' and t.s == '>' and toks[i-1].s != '-': d -= 1
        elif d == 0 and t.k == 'p' and t.s in '{(;': break
        i += 1
    kind = toks[i].s
    fields = []
    if kind == ';': return 'unit', fields
    close = '}' if kind == '{' else ')'
    depth = 0; j = i + 1; start_field = True
    dd = 0
    while True:
        t = toks[j]
        if start_field:
            # skip attrs
            while toks[j].s == '#':
                q = j + 1; dq = 0
                while True:
                    if toks[q].s == '[': dq += 1
                    elif toks[q].s == ']':
                        dq -= 1
                        if dq == 0: break
                    q += 1
                j = q + 1
            t = toks[j]
            if t.k == 'p' and t.s == close: break
            vis_a = vis_b = None
            if t.s == 'pub' and t.k == 'id':
                vis_a = off + t.a; j += 1
                if toks[j].s == '(' and toks[j].k == 'p':
                    q = j; dq = 0
                    while True:
                        if toks[q].s == '(': dq += 1
                        elif toks[q].s == ')':
                            dq -= 1
                            if dq == 0: break
                        q += 1
                    vis_b = off + toks[q].b; j = q + 1
                else:
                    vis_b = off + toks[j-1].b
                t = toks[j]
            name = None; name_a = off + t.a
            if kind == '{':
                name = t.s; j += 1
                assert toks[j].s == ':', (text[it.start:it.start+80], t.s)
            fields.append(dict(name=name, vis_a=vis_a, vis_b=vis_b, name_a=name_a))
            start_field = False
            continue
        if t.k == 'p':
            if t.s in '([{<': dd += 1
            elif t.s in ')]}' and dd > 0: dd -= 1
            elif t.s == '>' and toks[j-1].s != '-' and dd > 0: dd -= 1
            elif t.s == ',' and dd == 0: start_field = True
            elif t.s == close and dd == 0: break
            elif t.s in ')]}' and dd == 0: break
        j += 1
    return kind, fields

def rewrite_vis(chunk, ext_names, ext_members):
    """Return chunk with visibilities widened/normalized. Expects exactly one non-trivia item."""
    items, _ = chunk_file(chunk)
    real = [it for it in items if it.kind not in ('attr-only',)]
    assert len(real) == 1, [(i.kind, i.name) for i in items]
    it = real[0]
    edits = []
    def retarget(vis, vis_a, vis_b, head_a, name, widen):
        if widen:
            if vis == 'pub': return
            if vis_a is None: edits.append((head_a, head_a, 'pub '))
            else: edits.append((vis_a, vis_b, 'pub'))
        elif vis and vis != 'pub' and vis != 'pub(crate)':
            edits.append((vis_a, vis_b, 'pub(crate)'))
    if it.kind in ('fn','struct','enum','union','trait','type','const','static'):
        retarget(it.vis, it.vis_a, it.vis_b, it.head_a, it.name, it.name in ext_names)
        if it.kind == 'struct':
            kind, fields = struct_field_spans(chunk, it)
            for f in fields:
                visT = chunk[f['vis_a']:f['vis_b']] if f['vis_a'] is not None else ''
                if kind == '{':
                    widen = f['name'] in ext_members or it.name in ext_names and False
                else:
                    widen = it.name in ext_names
                retarget(visT, f['vis_a'], f['vis_b'], f['name_a'], f['name'], widen)
    elif it.kind == 'impl':
        bs = chunk.index('{', it.start)
        inner = chunk[bs + 1: it.end - 1]
        sub, _ = chunk_file(inner)
        if it.impl_trait is None:
            for s in sub:
                if s.kind in ('fn','const','type') and s.name:
                    o = bs + 1
                    va = s.vis_a + o if s.vis_a is not None else None
                    vb = s.vis_b + o if s.vis_b is not None else None
                    retarget(s.vis, va, vb, s.head_a + o, s.name, s.name in ext_members)
    out = chunk
    for a, b, r in sorted(edits, reverse=True):
        out = out[:a] + r + out[b:]
    return out


TITLES = {
 'model': ('🗃️', 'World3d data model: scene records, resource slots and registries, interaction ids, and the tool run trace layer.'),
 'components': ('🧩️', 'World3d state components: the per-concern state structs the host composes, with the types each one owns.'),
 'cursors': ('🧲️', 'World3d interaction cursors: ray, object and component picking, marquee selection and gumball gestures.'),
 'scene': ('🧱️', 'World3d scene resources and asset io: pools, chunking, LOD grid, environment, terrain, placeholder meshes and asset requests.'),
 'bridge': ('🌉️', 'World3d scene bridge: dynamic retirement, face overlays, snapshot apply and the guest scene bridge.'),
 'interaction': ('🧵️', 'World3d interaction: pointer, wheel and gesture entry points and the interaction authority.'),
 'host': ('🎛️', 'World3d host: the composed `World3dState`, frame sync and render orchestration.'),
}

LAST_DEFCFG = {}
PRUNE = {}
try:
    PRUNE = json.load(open('/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf/prune.json', encoding='utf-8'))
except Exception:
    PRUNE = {}

def build_crates(P, extra_used=None):
    mods = P['mods']
    crates, ic = assemble(P)
    imports = collect_imports(mods)
    # defs per crate
    byname = {}
    defcfg = {}
    for c in crates:
        c.defs = {}
        for rel, idx in c.items:
            it = mods[rel].items[idx]
            if it.name and it.kind in ('fn','struct','enum','union','trait','type','const','static','macro_rules'):
                if it.name in byname and byname[it.name] != c.idx:
                    c.defs[it.name] = it.kind   # duplicates across crates are reported below
                    print('DUP top-level name across crates', it.name, byname[it.name], c.idx)
                byname[it.name] = c.idx
                c.defs[it.name] = it.kind
                defcfg.setdefault(it.name, [])
                defcfg[it.name].append(tuple(testkit_cfg(a) for a in it.cfg))
    crates[0].defs['tool_run_trace'] = 'mod'
    byname['tool_run_trace'] = 0
    # used idents per crate
    for c in crates:
        c.plain = set(); c.dots = set(); c.fields = set(); c.labels = set(); c.assoc = set()
        per = collections.defaultdict(list)
        for rel, idx in c.items: per[rel].append(idx)
        for rel, idxs in per.items():
            p, d, f, l, a = used_idents(mods[rel], idxs)
            c.plain |= p; c.dots |= d; c.fields |= f; c.labels |= l; c.assoc |= a
    # ext usage (other crates + extra e.g. tests)
    for c in crates:
        c.ext_names = set(); c.ext_members = set()
        for k in crates:
            if k is c: continue
            c.ext_names |= k.plain
            c.ext_members |= k.plain | k.dots | k.fields | k.labels | k.assoc
        if extra_used:
            c.ext_names |= extra_used['plain']
            c.ext_members |= extra_used['plain'] | extra_used['dots'] | extra_used['fields'] | extra_used['labels'] | extra_used['assoc']
    # body text + imports
    for c in crates:
        c.body = []
        for rel, idx in c.items:
            t = chunk_text(mods[rel], idx)
            t = normalize_vis(t)
            it = mods[rel].items[idx]
            if it.kind not in ('macro_call',):
                t = rewrite_vis(t, c.ext_names, c.ext_members) if it.kind in ('fn','struct','enum','union','trait','type','const','static','impl') else t
            c.body.append(t.strip('\n'))
        cross = collections.defaultdict(list); ext = []
        seen_ext = set()
        for nm in sorted(c.plain):
            if nm in c.defs: continue
            if nm in byname and byname[nm] < c.idx:
                cross[byname[nm]].append(nm)
            elif nm in imports:
                for e in imports[nm]:
                    key = (e['attrs'], tuple(e['path']), e['local'])
                    if key in seen_ext: continue
                    seen_ext.add(key)
                    ext.append(dict(attrs=tuple(testkit_cfg(a) for a in e['attrs']), path=e['path'], local=e['local']))
        pr = set(PRUNE.get(c.folder + '/🦀️.rs', ()))
        for k in list(cross): cross[k] = [n for n in cross[k] if n not in pr]
        ext = [e for e in ext if e['local'] not in pr]
        c.cross = {k: v for k, v in cross.items() if v}
        c.ext_use = ext
        c.cross_cfg = {nm: defcfg.get(nm, [()]) for lst in c.cross.values() for nm in lst}
    global LAST_DEFCFG
    LAST_DEFCFG = defcfg
    return crates, ic, imports


def header_text(c):
    emoji, desc = TITLES[c.id]
    lines = [f"//! {emoji} {desc}", "//!", f"//! Crate `{crate_pkg(c.id)}`; a downward producer of the infinite world family (wasi-excluded except tests).", "", "#![cfg(any(test, not(all(target_arch = \"wasm32\", target_env = \"p2\"))))]", ""]
    uses = []
    for k in sorted(c.cross):
        ident = crate_ident(CRATES[k][0])
        groups = collections.OrderedDict()
        for nm in sorted(set(c.cross[k]), key=lambda s: s.lower()):
            variants = c.cross_cfg.get(nm, [()])
            # a name defined under several cfg variants is imported under the union of them
            keyset = tuple(sorted({v for v in variants}))
            if len(keyset) > 1 or (len(keyset) == 1 and any(len(x) > 1 for x in keyset)):
                # complex: import under 'any(...)' of the variants' joined predicate is not expressible simply; import each variant separately
                for v in keyset:
                    groups.setdefault(v, []).append(nm)
            else:
                groups.setdefault(keyset[0], []).append(nm)
        for attrs, names in groups.items():
            for a in attrs: uses.append(a)
            uses.append(f"use {ident}::{{{', '.join(names)}}};" if len(names) > 1 else f"use {ident}::{names[0]};")
    uses += emit_use(c.ext_use)
    out = lines + uses + [""]
    if c.id == 'model':
        out += ['#[path = "⏯️tool-run-trace/🦀️.rs"]', 'pub mod tool_run_trace;', ""]
    return '\n'.join(out)

def write_crates(crates, outroot):
    """outroot = stage tree root (the infinite dir copy). Writes lib roots, removes old world child module sources."""
    w = outroot + '/🌍️world'
    # remove old child module dirs (rs files only; keep tests/fixtures/etc)
    old = ['🧹️lifecycle','🧵️interaction','🧱️scene','🖌️overlay','🌉️bridge','🎛️frame','🖱️pointer','📡️assets']
    for d in old:
        shutil.rmtree(f"{w}/{d}", ignore_errors=True)
    # move tool-run-trace into model node
    for c in crates:
        os.makedirs(f"{w}/{c.folder}", exist_ok=True)
    if os.path.isdir(f"{w}/⏯️tool-run-trace"):
        shutil.move(f"{w}/⏯️tool-run-trace", f"{w}/{crates[0].folder}/⏯️tool-run-trace")
        V = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world"
        base = f"{w}/{crates[0].folder}/⏯️tool-run-trace"
        for dp, dn, fn in os.walk(base):
            for f in fn:
                if not f.endswith('.rs'): continue
                p = os.path.join(dp, f)
                rel = os.path.relpath(dp, base)
                old_dir = os.path.normpath(f"{V}/⏯️tool-run-trace/{rel}")
                new_dir = os.path.normpath(f"{V}/{crates[0].folder}/⏯️tool-run-trace/{rel}")
                t = open(p, encoding='utf-8').read()
                def sub(m):
                    absp = os.path.normpath(os.path.join(old_dir, m.group(2)))
                    return f'{m.group(1)}"{os.path.relpath(absp, new_dir)}"'
                t2 = re.sub(r'(include_(?:str|bytes)!\(\s*)"([^"]+)"', sub, t)
                if t2 != t: open(p, 'w', encoding='utf-8').write(t2)
    for c in crates:
        text = header_text(c) + '\n' + '\n\n'.join(c.body) + '\n'
        c.text = text
        open(f"{w}/{c.folder}/🦀️.rs", 'w', encoding='utf-8').write(text)
