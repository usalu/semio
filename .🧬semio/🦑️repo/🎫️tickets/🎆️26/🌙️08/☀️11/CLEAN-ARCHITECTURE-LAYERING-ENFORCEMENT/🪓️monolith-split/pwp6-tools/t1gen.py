import sys; sys.path.insert(0, '.')
import t2run, rw6
from t2run import *
from t1lit import parse_literal, find_literal
from fields6 import parse_fields, comp_of
from s0apply import doc_emoji_pool, SLUG

COMP_DOC = {
 'constructor': 'Store-constructor state: the receipts, units and lane of the four original catalogue constructors.',
 'stores': 'The stores of the instance: document, config, draft, presence, transient, interaction and window owner registries with their one-item publication factories.',
 'roster': 'The peer roster: this actor\'s colour, the adopted peer presence and the roster publication admission state.',
 'ingress': 'Action ingress: the parser slots that retain unused original arguments and faults until their paid close.',
 'interaction': 'Local interaction: the framework-owned live query, topology authority, hover and snapshot read returns.',
 'history': 'Command history: the session command log, the cached history view, backfill marks and the supersede ledger.',
 'host_events': 'Host events: the utilities, window roster and typed operations a host event dispatch reached.',
 'txn': 'Transactions: the single pending foreign transaction and its proposal.',
 'child_composition': 'Child composition: the live owned children, their private groups, content roots and retirement registries.',
 'pending': 'Pending reserved work: the reserved commits, render trees, configuration commands and their failure custody.',
 'tools': 'Tool operations: typed operation outboxes, tool jobs and registrations, cancellations and latest-wins commands.',
 'media': 'Media transfers: the media exports, downloads, closures and snapshot retirements of the instance.',
 'ingest': 'Envelope jobs: envelope ingress and decode, store replacement and document archive loads.',
 'maintenance': 'Maintenance cursors: the rotation positions of the live-maintenance ladder.',
 'close': 'Close cursors: the positions and drained flags of the close ladder.',
}
COMP_DIR = ('🧱️', 'components')

def comp_module_path(cf):
    crate = COMPONENTS[cf][1]
    return t2run.S.stage_dir(crate) + '/' + COMP_DIR[0] + COMP_DIR[1] + '/🦀️.rs' if False else None

CTOR_FILE = '🧵️instance/📥️staging/🚪️ingress/🦀️.rs'
PARENT = '🧵️instance/🦀️.rs'
STRUCT_KEEP_FIRST = ['app', 'mounted_policy']
STRUCT_KEEP_LAST = ['tool_runs', 'time_travel', 'tool_machines']

def field_type(decl):
    d = decl.strip().rstrip(',').strip()
    m = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:\s*(.*)$', d, re.S)
    return m.group(1), m.group(2).strip()

def fn_locals(text, fn_name, lit_start):
    i = text.index('fn ' + fn_name)
    seg = text[i:lit_start]
    M = mask(seg)
    params = re.search(r'fn\s+\w+(?:<[^>]*>)?\s*\((.*?)\)\s*->', M, re.S).group(1)
    names = set(re.findall(r'(?:^|,)\s*(?:mut\s+)?(\w+)\s*:', params))
    for m in re.finditer(r'\blet\s+(?:mut\s+)?(\w+)\b', M): names.add(m.group(1))
    for m in re.finditer(r'\blet\s*\(([^)]*)\)', M):
        for x in re.findall(r'(?:mut\s+)?(\w+)', m.group(1)): names.add(x)
    return names

def uses_local(init, locals_):
    M = mask(init)
    for m in re.finditer(r'(?<![\w.])(?<![^:]::)([A-Za-z_]\w*)', M):
        if m.group(1) in locals_ and not M[m.end():m.end() + 2] == '::' and not re.match(r'\s*(::|\()', M[m.end():m.end()+3]) : return m.group(1)
    return None

def plan_constructor(ctor_text):
    ents, ob, cb = parse_literal(ctor_text)
    locals_ = fn_locals(ctor_text, 'with_registry_on_bus', ctor_text.index('let mut this = Self {'))
    return ents, ob, cb, locals_

def build_components(ctor_text):
    fields = parse_fields(); ft = field_table()
    ents, ob, cb, locals_ = plan_constructor(ctor_text)
    ent = {e['name']: e for e in ents}
    by = collections.OrderedDict((cf, []) for cf in COMPONENTS)
    for f in fields:
        cf = ft[f['name']]['cf']
        if cf: by[cf].append(f)
    cg = build_comp_generics()
    comp = {}
    hoists = []
    for cf, fs in by.items():
        params = []; inits = []; structlines = []
        for f in fs:
            fname, ftype = field_type(f['decl'])
            e = ent[fname]
            init = e['init'] if e['init'] is not None else fname
            attrs = e['attrs']
            dl = f['decl'].strip().rstrip(',')
            # struct field text: docs + cfg attrs + `pub name: type,`
            docs = [l for l in f['raw'] if l.strip().startswith('///') or l.strip().startswith('//')]
            cfgs = [l.strip() for l in f['raw'] if l.strip().startswith('#[')]
            body = re.sub(r'^(pub(\([^)]*\))?\s+)?', '', dl.strip())
            structlines += [d.rstrip() for d in docs] + ['    ' + c for c in cfgs] + ['    pub ' + body + ',']
            if attrs: pass
            use = uses_local(init, locals_)
            if re.fullmatch(r'\w+', init) and init in locals_:
                params.append((fname, ftype, init)); inits.append((fname, None, attrs))
            elif use or '.await' in init:
                hoists.append((fname, init, attrs, use))
                if fname in locals_ and fname != init: raise SystemExit('hoist name clash ' + fname)
                params.append((fname, ftype, fname)); inits.append((fname, None, attrs))
            else:
                inits.append((fname, init, attrs))
        comp[cf] = dict(params=params, inits=inits, structlines=structlines, generics=cg[cf])
    return comp, hoists, ents, ob, cb, locals_

def component_file_text(crate, cfs, comp, w, emojis):
    lines = ['//! ' + next(emojis) + ' The instance components of `' + crate + '`: ' + ', '.join(COMPONENTS[c][0] for c in cfs) + ' (WP6, plan section 7.3).', 'use super::*;', '']
    for cf in cfs:
        c = comp[cf]; name = COMPONENTS[cf][0]; g = c['generics']
        bounds = {'A': 'A: ArtifactApp', 'M': 'M: SpaceMember + MemberFactory'}
        sgen = '<' + ', '.join(bounds[x] for x in g) + '>' if g else ''
        igen = '<' + ', '.join({'A': 'A: ArtifactApp', 'M': "M: SpaceMember + MemberFactory + 'static"}[x] for x in g) + '>' if g else ''
        tgen = '<' + ', '.join(g) + '>' if g else ''
        lines += ['/// ' + next(emojis) + ' ' + COMP_DOC[cf], 'pub struct ' + name + sgen + ' {'] + c['structlines'] + ['}', '']
        plist = ', '.join(n + ': ' + t for n, t, _ in c['params'])
        lines += ['impl' + igen + ' ' + name + tgen + ' {', '    /// ' + next(emojis) + ' Builds the component from the parts the composer constructs first; every other field starts at its construction value.', '    pub fn new(' + plist + ') -> Self {', '        Self {']
        for fname, init, attrs in c['inits']:
            for a in (attrs.split('\n') if attrs else []): lines.append('            ' + a.strip())
            lines.append('            ' + (fname + ',' if init is None else fname + ': ' + init + ','))
        lines += ['        }', '    }', '}', '']
    while lines and not lines[-1].strip(): lines.pop()
    return '\n'.join(lines) + '\n'

CRATE_COMPS = collections.OrderedDict()
for _cf, (_n, _crate, _k) in COMPONENTS.items(): CRATE_COMPS.setdefault(_crate, []).append(_cf)
CRATE_SHORT = {'instance-stores': 'stores', 'instance-history': 'history', 'instance-children': 'children', 'instance-jobs': 'jobs', 'instance-docs': 'docs', 'instance-ladders': 'ladders'}
CRATE_ROOT_ = {'instance-stores': '🧵️instance/🏪️stores', 'instance-history': '🧵️instance/📜️history', 'instance-children': '🧵️instance/🪆️children', 'instance-jobs': '🧵️instance/👷️jobs', 'instance-docs': '🧵️instance/📄️docs', 'instance-ladders': '🪜️maintenance-ladder'}

def comp_file_path(crate): return CRATE_ROOT_[crate] + '/📥️staging/🧱️components/🦀️.rs'

def struct_text(comp, ents, w):
    lines_out = []
    # composer struct: kept fields first
    ent = {e['name']: e for e in ents}
    ftab = {f['name']: f for f in parse_fields()}
    sdecl = lambda n: re.sub(r'^(pub(\([^)]*\))?\s+)?', '', ftab[n]['decl'].strip())
    out = ['pub struct VcsArtifactApp<A: ArtifactApp, M: SpaceMember + MemberFactory = NoMembers> {']
    def emit_kept(n):
        f = ftab[n]
        for l in f['raw']:
            if l.strip().startswith(('///', '#[', '//')): out.append(l.rstrip())
        out.append('    pub(crate) ' + sdecl(n))
    for n in STRUCT_KEEP_FIRST: emit_kept(n)
    for cf in COMPONENTS:
        name = COMPONENTS[cf][0]; g = comp[cf]['generics']
        out.append('    /// ' + next(EMO) + ' ' + COMP_DOC[cf])
        out.append('    pub(crate) ' + cf + ': ' + name + ('<' + ', '.join(g) + '>' if g else '') + ',')
    for n in STRUCT_KEEP_LAST: emit_kept(n)
    out.append('}')
    return out

def literal_text(comp, hoists, ents):
    ent = {e['name']: e for e in ents}
    out = []
    for h in hoists:
        out.append('        let ' + h[0] + ' = ' + h[1] + ';')
    out.append('        let mut this = Self {')
    for n in STRUCT_KEEP_FIRST: out.append('            ' + ent[n]['raw'] + ',')
    for cf in COMPONENTS:
        name = COMPONENTS[cf][0]
        args = ', '.join(p[2] for p in comp[cf]['params'])
        out.append('            ' + cf + ': ' + name + '::new(' + args + '),')
    for n in STRUCT_KEEP_LAST: out.append('            ' + ent[n]['raw'] + ',')
    out.append('        };')
    return out

EMO = None
def init_emoji():
    global EMO
    EMO = iter(doc_emoji_pool()[8:])      # the first emojis serve the staging files of step S0

def apply_all():
    init_emoji()
    nt, w, stats, model, table = t2run.convert_all()
    ctor = nt[CTOR_FILE]
    comp, hoists, ents, ob, cb, locals_ = build_components(ctor)
    # constructor literal
    start_line = ctor.rfind('\n', 0, ctor.index('let mut this = Self {')) + 1
    M = mask(ctor); end = cb
    end_stmt = ctor.index(';', cb) + 1
    new_lit = '\n'.join(literal_text(comp, hoists, ents))
    ctor2 = ctor[:start_line] + new_lit + ctor[end_stmt:]
    # `this` receiver in the constructor tail
    fnstart = ctor2.index('pub async fn with_registry_on_bus')
    tail_start = ctor2.index('};', ctor2.index('let mut this = Self {')) + 2
    fn_end = rw6.match_close(mask(ctor2), ctor2.index('{', ctor2.index(')', ctor2.index('pub async fn with_registry_on_bus'))), '{', '}') if False else None
    tail = ctor2[tail_start:]
    # rewrite only the statements after the literal up to the end of the function: the function ends with `this\n    }`
    m_end = re.search(r'\n        this\n    }', tail)
    body_tail = tail[:m_end.start()] if m_end else tail
    rest = tail[m_end.start():] if m_end else ''
    body_tail2 = rw6.rewrite(body_tail, w, 'composer', recv='this')
    ctor2 = ctor2[:tail_start] + body_tail2 + rest
    nt[CTOR_FILE] = ctor2
    # parent: struct + mounts
    parent = nt[PARENT]
    pm = parse_file(PARENT)  # for the struct span: use a text based search
    i0 = parent.index('pub struct VcsArtifactApp')
    # struct starts at i0 (docs above are kept); ends at the matching brace
    ob_ = parent.index('{', parent.index('>', i0 + 30)) if False else parent.index(' {\n', i0)
    Mp = mask(parent)
    cbs = rw6.match_close(Mp, ob_ + 1, '{', '}')
    new_struct = '\n'.join(struct_text(comp, ents, w))
    parent = parent[:i0] + new_struct + parent[cbs + 1:]
    # component module mounts
    mounts = []; uses = []
    comp_texts = {}
    for crate, cfs in CRATE_COMPS.items():
        path = comp_file_path(crate)
        comp_texts[path] = component_file_text(crate, cfs, comp, w, EMO)
        mod = CRATE_SHORT[crate] + '_components'
        rel = os.path.relpath(PLUG + path, PLUG + '🧵️instance')
        mounts += ['#[path = "' + rel + '"]', 'mod ' + mod + ';']
        uses.append('pub(crate) use ' + mod + '::{' + ', '.join(COMPONENTS[c][0] for c in cfs) + '};')
    marker = '//#endregion 📥️StagedInstanceMounts'
    assert parent.count(marker) == 1
    parent = parent.replace(marker, '\n'.join(mounts + uses) + '\n' + marker)
    nt[PARENT] = parent
    nt.update(comp_texts)
    return nt, w, stats, comp, hoists

if __name__ == '__main__':
    nt, w, stats, comp, hoists = apply_all()
    print(stats, len(nt), 'files; flags', w.flags)
    print(nt[PARENT][nt[PARENT].index('pub struct VcsArtifactApp'):][:3500])
