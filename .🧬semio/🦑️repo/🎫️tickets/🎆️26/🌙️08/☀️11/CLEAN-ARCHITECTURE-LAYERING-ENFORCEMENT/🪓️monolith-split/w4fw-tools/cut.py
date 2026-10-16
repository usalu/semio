import os, sys, re, json, collections, posixpath
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from load import *
from imports import EXTERN
WHOLE_FILE = {'onboarding': 'onboarding', 'ta_actions': 'time-travel-actions', 'tr_actions': 'tool-run-actions'}
KEPT = {'interaction_gesture', 'interaction_schema', 'k_clipboard', 'k_content', 'k_message', 'k_cold', 'm_utilinit', 'platform'}
NOISE = {'_', 'bytes', 'Part', 'absent'}
FALSE_EDGES = {('kernel-media-jobs', 'Platform'), ('abi', 'Event')}
ROOT_MAP = {'IconName': 'ui_wgpu::wgpu::IconName', 'ArtifactDialect': 'semio_framework_artifact_reference::ArtifactDialect'}
FILE_ORDER = {
 'manifest-media': ['m_vocab', 'm_view', 'm_media'],
 'manifest-contribution': ['m_plugin', 'm_surface', 'm_space', 'm_canonical'],
 'manifest-actions': ['manifest', 'm_actions', 'm_argschema', 'm_utilinit'],
 'manifest-app-surface': ['m_surface', 'm_view', 'm_actions'],
 'manifest-plugin': ['m_plugin', 'm_view'],
 'manifest-product-actions': ['m_vocab', 'm_actions'],
 'action-bus': ['action_bus', 'platform'],
}
def snake(pkg): return pkg.replace('-', '_')
def rel_from(dir_, target):
    return os.path.relpath(target, dir_)
class Cut:
    def __init__(self, R):
        self.R = R
        self.files = load(R)
        self.ents = entities(self.files)
        self.tables = {}
        for k, f in self.files.items():
            t = {}
            for pub, items in file_imports(f.text):
                for path, alias in items:
                    if alias in ('*', '_'): continue
                    pth = '::'.join(p for p in path if p != '')
                    rel = pth.split('::')[0] in ('crate', 'super', 'self')
                    if alias in t and not (t[alias].split('::')[0] in ('crate', 'super', 'self')) and rel: continue
                    t[alias] = pth
            self.tables[k] = t
        self.name2ents = collections.defaultdict(list)
        for key, e in self.ents.items():
            if e['kind'] not in ('mod',) and e['cluster'] and not e['name'].startswith('<') and e['file'] not in KEPT: self.name2ents[e['name']].append(key)
        self.cluster_units = collections.defaultdict(list)
        self.kept_tokens = collections.defaultdict(set)
        self.kept_local = collections.defaultdict(set)
        self.cluster_names = collections.defaultdict(set)
        self.global_alias = {}
        for k, t in self.tables.items():
            if k == 'root': continue
            for a, pth in t.items():
                if pth.split('::')[0] in ('crate','super','self') or pth.split('::')[0] in self.tables: continue
                self.global_alias.setdefault(a, pth)
        self.kept_names = collections.defaultdict(set)
        self.widen = set()
        self.assign_units()
    def assign_units(self):
        for key, e in self.ents.items():
            c = e['cluster']
            if c in (None, 'UNASSIGNED'):
                if c == 'UNASSIGNED': raise SystemExit(f"UNASSIGNED {key}")
                continue
            if e['file'] in KEPT:
                self.kept_tokens[c] |= e['tokens']
                if not e['name'].startswith('<'): self.kept_local[c].add(e['name'])
                continue
            for u in e['units']: self.cluster_units[c].append(u)
            if not e['name'].startswith('<'): self.cluster_names[c].add(e['name'])
        for k, f in self.files.items():
            if k in KEPT: continue
            for u in f.units:
                if u.kind != 'use' or u.test: continue
                head = u.text.split('use ', 1)[0]
                body = re.sub(r'//[^\n]*', '', u.text)
                if not re.search(r'(^|\n)\s*pub(\([^)]*\))?\s+use\b', body): continue
                c = self.assign_use(k, u, body)
                if c:
                    self.cluster_units[c].append(u)
                    for pub, items in file_imports(u.text):
                        if pub:
                            for path, alias in items:
                                if alias != '*': self.cluster_names[c].add(alias)
    def assign_use(self, k, u, body):
        if k == 'kernel' or k.startswith('k_'):
            if re.search(r'replication::\{[^}]*\bActorId\b', body) or 'replication::HybridLogicalTimestamp' in body: return 'kernel-identity'
            if 'semio_framework_ui_contract::{PresenceUpdate' in body or 'UiPatch' in body and 'ui_contract' in body: return 'kernel-ui'
            return 'kernel'
        if k == 'abi': return 'abi'
        if k.startswith('interaction'): return 'interaction'
        if k in WHOLE_FILE: return WHOLE_FILE[k]
        if k == 'manifest':
            return 'manifest-actions' if 'initial_window_utility' in body else None
        return None
    def owner_of(self, name, family_hint, current):
        cands = [x for x in self.name2ents.get(name, []) if self.ents[x]['cluster'] != current]
        if not cands: return None
        if len(cands) > 1:
            fam = [x for x in cands if FAMILY(x[0]) == family_hint]
            cands = fam or cands
        return self.ents[cands[0]]['cluster']
    def order_key(self, c, u):
        order = FILE_ORDER.get(c)
        idx = order.index(u.file) if order and u.file in order else (len(order) if order else list(SRC).index(u.file))
        return (idx, u.start)
    def tokens_of(self, units):
        s = set()
        for u in units: s |= u.tokens
        return s

PATH_RE = re.compile(r'(include_str!|include_bytes!|include!)\(\s*"([^"]+)"\s*\)')
ATTR_PATH_RE = re.compile(r'#\[path\s*=\s*"([^"]+)"\s*\]')
def fix_paths(text, old_dir, new_dir):
    if old_dir == new_dir: return text
    def fix(rel):
        if rel.startswith('/') : return rel
        ab = os.path.normpath(os.path.join(old_dir, rel))
        return os.path.relpath(ab, new_dir)
    text = PATH_RE.sub(lambda m: f'{m.group(1)}("{fix(m.group(2))}")', text)
    text = ATTR_PATH_RE.sub(lambda m: f'#[path = "{fix(m.group(1))}"]', text)
    return text
KQ = re.compile(r'(?<![A-Za-z0-9_:])kernel::(?=[A-Za-z_])')
def unit_text(u, cut, cluster_root_dir):
    lines = u.text.split('\n')
    lines = [l for l in lines if not MARK.match(l)]
    t = '\n'.join(lines)
    k = u.file
    if k == 'manifest' or k.startswith('m_'):
        t = KQ.sub('', t)
    if u.kind != 'mod':
        t = re.sub(r'\bpub\(super\)', 'pub(crate)', t)
        t = re.sub(r'\bpub\(crate\)(\s+(?:async\s+)?fn\s+(?:with_schema|plain_string)\b)', r'pub\1', t)
    t = re.sub(r'(?<![A-Za-z0-9_:])crate::(?:(?:ui|manifest|kernel)::)+([A-Za-z_])', r'\1', t)
    if k == 'kernel': t = re.sub(r'(?<![A-Za-z0-9_:])super::ViewModel', 'ViewModel', t)
    t = t.replace('intrinsic_media::fingerprint', 'fingerprint')
    t = '\n'.join(l for l in t.split('\n') if 'mod intrinsic_media;' not in l)
    for (wf, wn) in cut.widen:
        if wf == u.file and (u.name == wn or u.target == wn):
            t = re.sub(r'(?m)^(\s*)(?:pub\(crate\)\s+|pub\s+)?((?:async\s+)?(?:const\s+)?(?:unsafe\s+)?(?:fn|struct|enum|trait|type|const|static)\s+' + re.escape(wn) + r'\b)', r'\1pub \2', t, count=1)
    t = fix_paths(t, os.path.dirname(os.path.join(cut.R, SRC[k])), os.path.join(cut.R, cluster_root_dir))
    return t
def region_lines(prev, cur):
    """markers to move from stack prev to cur"""
    out = []
    i = 0
    while i < len(prev) and i < len(cur) and prev[i] == cur[i]: i += 1
    for j in range(len(prev) - 1, i - 1, -1):
        line, name = prev[j]
        out.append(line.replace('#region', '#endregion', 1))
    for j in range(i, len(cur)): out.append(cur[j][0])
    return out
def build_body(cut, c, units):
    cdir = os.path.dirname(CLUSTERS[c]['root'])
    units = sorted(units, key=lambda u: cut.order_key(c, u))
    out = []; stack = ()
    for u in units:
        t = unit_text(u, cut, cdir)
        for m in region_lines(stack, u.regions): out.append(m)
        stack = u.regions
        out.append(t)
        out.append('')
    for m in region_lines(stack, ()): out.append(m)
    return '\n'.join(out), units
def header_imports(cut, c, units, text_tokens, extra_skip=()):
    names_local = cut.cluster_names[c] | cut.kept_local[c]
    imports = collections.OrderedDict()   # path -> alias/None
    cluster_use = collections.defaultdict(set)
    fams = {FAMILY(u.file) for u in units} or {'x'}
    fam = sorted(fams)[0]
    tables = [cut.tables[k] for k in sorted({u.file for u in units}, key=lambda k: list(SRC).index(k))]
    merged = {}
    for k in sorted({u.file for u in units}, key=lambda k: list(SRC).index(k)):
        merged.update(cut.tables[k])
        if k.startswith('m_') and k not in ('m_media', 'm_canonical', 'm_utilinit'): 
            for a, p in cut.tables['manifest'].items(): merged.setdefault(a, p)
        if k in ('m_canonical', 'm_utilinit'):
            for a, p in cut.tables['manifest'].items(): merged.setdefault(a, p)
    allalias = collections.defaultdict(set)
    for k, t in cut.tables.items():
        if k == 'root': continue
        for a, p in t.items():
            if p.split('::')[0] not in ('crate', 'super', 'self'): allalias[a].add((FAMILY(k), p))
    ext = set(EXTERN)
    for tk in sorted(text_tokens):
        if tk in names_local or tk in NOISE or (c, tk) in FALSE_EDGES or tk in extra_skip: continue
        o = cut.owner_of(tk, fam, c)
        if o:
            cluster_use[o].add(tk); continue
        path = merged.get(tk)
        if path is not None and path.split('::')[0] in ('crate', 'super', 'self') and tk not in ROOT_MAP: path = None
        if path is None:
            cands = allalias.get(tk)
            if cands:
                same = [p for f_, p in cands if f_ == fam]
                path = sorted(same or [p for f_, p in cands])[0]
        if path is None and tk in ROOT_MAP: path = ROOT_MAP[tk]
        if path is None: continue
        if path.startswith(('crate::', 'super::', 'self::')) or path.split('::')[0] in ('crate', 'super', 'self'):
            if tk in ROOT_MAP: path = ROOT_MAP[tk]
            else: continue
        root = path.split('::')[0]
        if root in cut.tables: continue
        if root not in ext and not root.startswith('semio_') and root not in ('ui_wgpu', 'replication', 'serde', 'serde_json', 'pack', 'std', 'core', 'alloc', 'base64'):
            continue
        imports[path] = tk
    for meth, tr, pth in (('to_value', 'ToValue', 'semio_framework_value::ToValue'), ('from_value', 'FromValue', 'semio_framework_value::FromValue')):
        if meth in text_tokens and tr not in imports.values() and tr not in names_local and tr not in extra_skip:
            imports[pth] = tr
    return imports, cluster_use

TEST_PREFIX = re.compile(r'(?<![A-Za-z0-9_:])(?:crate|super)::((?:(?:ui|manifest|kernel)::)*)')
USE_RE = re.compile(r'(?m)^([ \t]*)((?:pub )?)use\s+((?:crate|super)::(?:(?:ui|manifest|kernel)::)*)(\{.*?\}|[A-Za-z_]\w*(?:\s+as\s+\w+)?)\s*;', re.S)
def pkg_ident(c): return snake(CLUSTERS[c]['pkg'])
class TestRewriter:
    def __init__(self, cut): self.cut = cut; self.log = []
    def owner_path(self, name, c):
        cut = self.cut
        if name in cut.cluster_names[c] or name in cut.kept_names.get(c, ()): return 'crate'
        o = cut.owner_of(name, 'manifest' if c.startswith('manifest') else 'kernel', c)
        if o: return pkg_ident(o)
        return None
    def rewrite(self, text, c):
        cut = self.cut
        text = re.sub(r'(?m)^[ \t]*use\s+crate::ui::kernel\s*;\s*\n', '', text)
        text = KQ.sub('', text)
        def use_sub(m):
            indent, pub, prefix, tree = m.group(1), m.group(2), m.group(3), m.group(4)
            items = flatten(re.sub(r'//[^\n]*', '', tree).replace('\n', ' '))
            groups = collections.OrderedDict()
            for path, alias in items:
                if alias == '*':
                    self.log.append(('glob', c, tree)); return m.group(0)
                name = path[0]
                own = self.owner_path(name, c)
                if own is None:
                    if name in ROOT_MAP: dest = ROOT_MAP[name].rsplit('::', 1)[0]
                    elif name in cut.global_alias: dest = cut.global_alias[name].rsplit('::', 1)[0]
                    else:
                        self.log.append(('unresolved', c, name)); return m.group(0)
                else: dest = own
                last = '::'.join(path)
                groups.setdefault(dest, []).append(last + ('' if alias == path[-1] else f' as {alias}'))
            out = []
            for dest, names in groups.items():
                names = sorted(set(names))
                out.append(f"{indent}{pub}use {dest}::" + (names[0] if len(names) == 1 else '{' + ', '.join(names) + '}') + ';')
            return '\n'.join(out)
        text = USE_RE.sub(use_sub, text)
        def inline(m):
            name = m.group(2)
            own = self.owner_path(name, c)
            if own is None:
                if name in ROOT_MAP: return ROOT_MAP[name]
                if name in cut.global_alias: return cut.global_alias[name]
                self.log.append(('unresolved-inline', c, name)); return m.group(0)
            return f"{own}::{name}"
        text = re.sub(r'(?<![A-Za-z0-9_:])(?:crate|super)::((?:(?:ui|manifest|kernel)::)*)([A-Za-z_]\w*)', lambda m: inline(m) if m.group(2) not in ('self',) else m.group(0), text)
        if not re.search(r'use\s+super::\*\s*;', text):
            masked = mask(text)
            toks = set(TOK.findall(masked))
            have = set()
            for pub_, items in file_imports(text):
                for path, alias in items: have.add(alias)
            local = {m.group(1) for m in re.finditer(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn|struct|enum|trait|type|const|static|mod)\s+([A-Za-z_]\w*)', masked)}
            need = collections.defaultdict(set)
            for tk in sorted(toks - have - local):
                if tk in NOISE: continue
                if tk in cut.cluster_names[c]: continue
                o = cut.owner_of(tk, 'manifest' if c.startswith('manifest') else 'kernel', c)
                if o and not (c, tk) in FALSE_EDGES:
                    # only when used as a bare path head or call, never a field/variant guess
                    need[pkg_ident(o)].add(tk)
            if need:
                add = '\n'.join('use ' + dest + '::' + (sorted(v)[0] if len(v) == 1 else '{' + ', '.join(sorted(v)) + '}') + ';' for dest, v in need.items()) + '\n'
                lines = text.split('\n'); i = 0
                while i < len(lines) and (lines[i].startswith('//!') or lines[i].startswith('#![')): i += 1
                text = '\n'.join(lines[:i]) + ('\n' if i else '') + add + '\n'.join(lines[i:])
        return text
def is_foreign_super(text):
    return False

def group_imports(imports, cluster_use):
    lines = []
    for path, alias in sorted(imports.items()):
        last = path.split('::')[-1]
        lines.append(f"use {path};" if last == alias else f"use {path} as {alias};")
    ext = lines
    cl = []
    for o in sorted(cluster_use):
        names = sorted(cluster_use[o])
        cl.append(f"use {pkg_ident(o)}::" + (names[0] if len(names) == 1 else '{' + ', '.join(names) + '}') + ';')
    return ext, cl
def test_entries(cut, c):
    out = []
    for k, f in cut.files.items():
        if k in KEPT: continue
        for u in f.units:
            if u.test and u.kind == 'mod' and TESTS.get((k, u.name), 'NA') == c:
                m = ATTR_PATH_RE.search(u.attrs)
                if not m: raise SystemExit(f"test mount without path {k} {u.name}")
                ab = os.path.normpath(os.path.join(os.path.dirname(os.path.join(cut.R, SRC[k])), m.group(1)))
                out.append((RENAME_TESTS.get((k, u.name), u.name), ab, u))
    return out
def build_cluster(cut, c, rewriter):
    info = CLUSTERS[c]
    units = cut.cluster_units[c]
    body, units = build_body(cut, c, units)
    cdir = os.path.join(cut.R, os.path.dirname(info['root']))
    tokens = cut.tokens_of(units) | cut.kept_tokens[c]
    imports, cuse = header_imports(cut, c, units, tokens)
    tests = test_entries(cut, c)
    ttext = {}
    tokens_t = set()
    for name, ab, u in tests:
        new = rewriter.rewrite(open(ab, encoding='utf-8').read(), c)
        ttext[ab] = new
        tokens_t |= set(TOK.findall(mask(new)))
    provided = set(cut.cluster_names[c]) | set(imports.values()) | {n for v in cuse.values() for n in v}
    t_imports, t_cuse = header_imports(cut, c, units, tokens_t - provided)
    return dict(c=c, body=body, units=units, imports=imports, cuse=cuse, tests=tests, ttext=ttext, t_imports=t_imports, t_cuse=t_cuse, dir=cdir)
def render_root(cut, b):
    c = b['c']; info = CLUSTERS[c]
    ext, cl = group_imports(b['imports'], b['cuse'])
    lines = [f"//! {info['emoji']} {info['doc']}"]
    if ext or cl:
        lines.append('')
        lines += ext + cl
    lines.append('')
    lines.append(b['body'].rstrip('\n'))
    t_ext, t_cl = group_imports(b['t_imports'], b['t_cuse'])
    if t_ext or t_cl:
        lines.append('')
        for l in t_ext + t_cl: lines.append(f"#[cfg(test)]\n{l}")
    if any('observe_heap_allocations_on_this_thread' in t for t in b['ttext'].values()):
        lines.append('')
        lines.append('#[cfg(test)]\n#[global_allocator]\nstatic TEST_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;')
    stack = ()
    for name, ab, u in sorted(b['tests'], key=lambda x: (x[2].regions, x[2].start)):
        lines.append('')
        for m in region_lines(stack, u.regions): lines.append(m)
        stack = u.regions
        rel = os.path.relpath(ab, b['dir'])
        lines.append(f'#[cfg(test)]\n#[path = "{rel}"]\nmod {name};')
    for m in region_lines(stack, ()): lines.append(m)
    return '\n'.join(lines) + '\n'
if __name__ == '__main__':
    cut = Cut(sys.argv[1] if len(sys.argv) > 1 else REPO)
    rw = TestRewriter(cut)
    out = {}
    for c in CLUSTERS:
        b = build_cluster(cut, c, rw)
        out[c] = b
        deps = sorted(set(b['cuse']) | set(b['t_cuse']))
        print(c, len(render_root(cut, b).split('\n')), 'lib-deps', sorted(b['cuse']), 'test-extra', sorted(set(b['t_cuse']) - set(b['cuse'])))
    print(rw.log[:40])
