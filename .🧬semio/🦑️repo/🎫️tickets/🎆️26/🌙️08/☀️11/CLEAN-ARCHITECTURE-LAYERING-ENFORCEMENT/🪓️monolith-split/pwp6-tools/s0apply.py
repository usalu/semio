import sys; sys.path.insert(0, '.')
from s0plan import *
import s0plan as SP
SLUG = {
 '🧵️instance/🦀️.rs': ('🧵️', 'instance-root'), '🧵️instance/🚪️ingress/🦀️.rs': ('🚪️', 'ingress'), '🧵️instance/📨️dispatch/🦀️.rs': ('📨️', 'dispatch'), '🧵️instance/🖼️render/🦀️.rs': ('🖼️', 'render'),
 '🧵️instance/🧷️command-routes/🦀️.rs': ('🧷️', 'command-routes'), '🧵️instance/🧾️typed-operations/🦀️.rs': ('🧾️', 'typed-operations'), '🧵️instance/🔌️plugin-app-impl/🦀️.rs': ('🔌️', 'plugin-app-impl'),
 '🧵️instance/🛠️tool-machine/🦀️.rs': ('🛠️', 'tool-machine'), '🧵️instance/🤝️group/🦀️.rs': ('🤝️', 'group'), '🧵️instance/🔄️full-operation/🦀️.rs': ('🔄️', 'full-operation'), '🧵️instance/⏳️operation-progress/🦀️.rs': ('⏳️', 'operation-progress'),
 '🪜️close-ladder/🦀️.rs': ('🔒️', 'close-ladder'), '🪜️envelope-ladder/🦀️.rs': ('🗝️', 'envelope-ladder'), '🪜️maintenance-ladder/🦀️.rs': ('🪜️', 'maintenance-ladder'),
 '⏪️time-travel/🧵️instance/🦀️.rs': ('⏪️', 'time-travel'), '⏯️tool-run/🧵️instance/🦀️.rs': ('⏯️', 'tool-run'),
 '🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs': ('🔁️', 'document-replacement'), '🎮️command/🎟️lease/🦀️.rs': ('🎟️', 'lease'), '♻️lifecycle/🏪️stores/🦀️.rs': ('🏪️', 'lifecycle-stores'),
 '🎮️command/📥️ingress/🦀️.rs': ('📮️', 'action-ingress'), '🪆️child/👁️read/🧪️test-view/🦀️.rs': ('🪆️', 'child-test-view')}
MODULE_ROOTED = {'🎮️command/📥️ingress/🦀️.rs'}   # the file is the module `action_ingress` of the facade root, not a child of the instance parent
ALLOWED_OUTSIDE_WIDEN = True

def src_for(rel): return PLUG + rel

def vis_of(line):
    m = re.match(r'(\s*)((?:pub(?:\([^)]*\))?\s+)?)(.*)$', line)
    return m.group(1), m.group(2).strip(), m.group(3)
def widen_line(line):
    ind, vis, rest = vis_of(line)
    if vis in ('pub', 'pub(crate)'): return line
    return ind + 'pub(crate) ' + rest

def struct_fields_widen(lines):
    """widen private named/tuple fields of the struct whose head starts in lines (first non-attr line)"""
    text = '\n'.join(lines)
    m = mask(text)
    i = m.find('struct ')
    if i < 0: return lines
    j = i
    # find body opener: '{' or '(' directly after generics
    k = j + 7
    while k < len(m) and (m[k].isalnum() or m[k] == '_'): k += 1
    # skip generics
    while k < len(m) and m[k].isspace(): k += 1
    if k < len(m) and m[k] == '<':
        d = 0
        while k < len(m):
            if m[k] == '<': d += 1
            elif m[k] == '>' and m[k - 1] != '-': d -= 1
            k += 1
            if d == 0: break
    while k < len(m) and m[k] not in '{(;' :
        # where clause etc
        if m.startswith('where', k):
            while k < len(m) and m[k] not in '{;': k += 1
            break
        k += 1
    if k >= len(m) or m[k] == ';': return lines
    open_, close_ = m[k], '}' if m[k] == '{' else ')'
    depth = 0; e = k
    while e < len(m):
        if m[e] in '({[': depth += 1
        elif m[e] in ')}]' : depth -= 1
        elif m[e] == '<': pass
        e += 1
        if depth == 0: break
    body_start, body_end = k + 1, e - 1
    # split top-level fields by commas respecting <>() [] {}
    out = text[:body_start]; seg_start = body_start; d = 0; angle = 0; prev = ''
    pieces = []
    for p in range(body_start, body_end):
        c = m[p]
        if c in '([{': d += 1
        elif c in ')]}': d -= 1
        elif c == '<': angle += 1
        elif c == '>' and prev != '-' and prev != '=': angle -= 1
        prev = c
        if c == ',' and d == 0 and angle == 0:
            pieces.append((seg_start, p + 1)); seg_start = p + 1
    pieces.append((seg_start, body_end))
    res = text[:body_start]
    for (a, b) in pieces:
        seg = text[a:b]
        sm = mask(seg)
        if not sm.strip(): res += seg; continue
        # leading attrs/docs
        mm = re.match(r'((?:\s*(?:#\[[^\]]*\]|///[^\n]*))*\s*)(.*)$', seg, re.S)
        lead, rest = mm.group(1), mm.group(2)
        if re.match(r'pub(?:\([^)]*\))?\s', rest): res += seg
        else: res += lead + 'pub(crate) ' + rest
    res += text[body_end:]
    return res.split('\n')

def snake(s): return re.sub(r'[^a-z0-9]+', '_', s.lower()).strip('_')

class Target:
    def __init__(self, f, crate): self.file = f; self.crate = crate; self.pieces = []; self.path = None; self.inplace = False; self.mod = None

DOC_EMOJI = list('📪📭📫🗄🧲🔏🔧📌📂📒📀🪙🗜⛓✉🎴🧿🪬🔩⚖🧴🪞🪟🪠🧽🧹🧺🪣🧼🪥🪒🧻🪤🥁🪘🎷🎺🪗🎸🎻🪕🎹🎼🏺🛖🏕🛶🪁🪂🛟🪐🔭🔬🩻🧫🧪🩺💉🩹🦠🧬🪸🪷🪻🪼🫎🫏🫚🫛🪭🪮🪯🪇🪈🪏🪉🫟🫧🪩🪪🪫🪬🫙🫘🪹🪺')
def doc_emoji_pool():
    used = set(json.load(open(GEN + '/pwp6-emoji-used.json'))['doc'])
    out = []
    for e in DOC_EMOJI:
        if any(u.startswith(e) for u in used): continue
        if e + '\ufe0f' not in out: out.append(e + '\ufe0f')
    return out

def stage_dir(crate): return CRATE_ROOT[crate] + '/📥️staging'

def build(P):
    recs, items, model = P['recs'], P['items'], P['model']
    mcrate = {(m['file'], m['piece'], m['idx']): m['crate'] for m in recs}
    icrate = {(it['file'], it['piece']): it['crate'] for it in items}
    targets = collections.OrderedDict()
    def tgt(f, c):
        k = (f, c)
        if k not in targets: targets[k] = Target(f, c)
        return targets[k]
    for rel, f in model.items():
        for pi, p in enumerate(f['pieces']):
            if p.kind == 'vcsimpl':
                by = collections.OrderedDict()
                for mi, m in enumerate(p.members): by.setdefault(mcrate[(rel, pi, mi)], []).append(mi)
                for c, mis in by.items(): tgt(rel, c).pieces.append(('impl', pi, mis))
            elif p.itkind == 'mod':
                if rel == '🧵️instance/🦀️.rs': continue
                dc = SP.ROOT_FILE.get(rel) or SP.IN_PLACE_EXTRA.get(rel) or FILE_USECASE[rel]
                tgt(rel, dc).pieces.append(('item', pi, None))
            else: tgt(rel, icrate[(rel, pi)]).pieces.append(('item', pi, None))
    return targets

def target_path(t):
    f, c = t.file, t.crate
    if SP.ROOT_FILE.get(f) == c or SP.IN_PLACE_EXTRA.get(f) == c:
        t.inplace = True; t.path = f; return
    emo, kebab = SLUG[f]
    t.path = stage_dir(c) + '/' + emo + kebab + '/🦀️.rs'
    t.mod = SHORT[c] + '_' + snake(kebab)

def main(dry=True):
    P = SP.plan()
    targets = build(P)
    for t in targets.values(): target_path(t)
    model = P['model']
    pool = doc_emoji_pool()
    rows = []
    for (f, c), t in targets.items():
        nl = 0
        for kind, pi, mis in t.pieces:
            p = model[f]['pieces'][pi]
            nl += (sum(p.members[i].e - p.members[i].a for i in mis) if kind == 'impl' else p.e - p.s)
        rows.append((t.crate, t.file, t.path, nl, t.inplace))
        print(c.ljust(18), ('IN-PLACE ' if t.inplace else 'staging  '), str(nl).rjust(5), f.split('/')[-2][:18].ljust(18), len(t.path.encode('utf-8')))
    print(len(targets), 'targets')
    return P, targets

if __name__ == '__main__':
    main()

TREE_OF = {'⏪️time-travel/🧵️instance/🦀️.rs': 'tt', '⏯️tool-run/🧵️instance/🦀️.rs': 'tr', '🎮️command/📥️ingress/🦀️.rs': 'ai'}
def old_tree(f): return TREE_OF.get(f, 'instance')
def new_tree(t): return old_tree(t.file) if t.inplace and t.file in TREE_OF else ('ai' if t.file in MODULE_ROOTED else 'instance')

FN_HEAD = re.compile(r'^(\s*)((?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+))')

def first_code_line(R, s, e):
    k = s
    while k < e and R[k].strip().startswith(('///', '#[', '//')) or (k < e and not R[k].strip()): k += 1
    return k

def impl_members(R, L, p):
    """named fn members at depth 1 of a (non-vcs) impl piece: (name, line_index)"""
    out = []; depth = 0
    for j in range(p.s, p.e):
        l = L[j]; before = depth
        depth = before + l.count('{') - l.count('}')
        if before == 1:
            m = FN_HEAD.match(l)
            if m: out.append((m.group(3), j))
    return out

def analyse_refs(P, targets):
    """returns: widen sets per (file, piece, member) based on references from other targets"""
    model = P['model']
    # defined names
    meth_def = collections.defaultdict(list)   # name -> [(target, kind, ref)]
    item_def = collections.defaultdict(list)
    for t in targets.values():
        f = model[t.file]
        for kind, pi, mis in t.pieces:
            p = f['pieces'][pi]
            if kind == 'impl':
                for mi in mis: meth_def[p.members[mi].name].append((t, 'vcs', (pi, mi)))
            else:
                if p.itkind == 'impl':
                    for nm, j in impl_members(f['R'], f['L'], p): meth_def[nm].append((t, 'assoc', (pi, j)))
                else:
                    nm = re.match(r'[A-Za-z_]\w*', p.name or '')
                    if nm: item_def[nm.group(0)].append((t, p.itkind, pi))
    # text per target (masked)
    ttext = {}
    for t in targets.values():
        f = model[t.file]; chunks = []
        for kind, pi, mis in t.pieces:
            p = f['pieces'][pi]
            if kind == 'impl': chunks += ['\n'.join(f['L'][p.members[mi].a:p.members[mi].e]) for mi in mis]
            else: chunks.append('\n'.join(f['L'][p.s:p.e]))
        ttext[(t.file, t.crate)] = '\n'.join(chunks)
    # extra (non-staged) texts that may reference: the whole rest of plugin tree is handled separately (they only see pub(crate)+)
    return meth_def, item_def, ttext


def expand_use(stmt):
    """`use a::b::{c, d as e, f::g};` -> {name: full path text}"""
    s = re.sub(r'\s+', ' ', stmt.strip().rstrip(';'))
    s = re.sub(r'^(?:pub(?:\([^)]*\))?\s+)?use\s+', '', s)
    out = {}
    def rec(prefix, body):
        body = body.strip()
        if '{' not in body:
            if body.endswith('*') or body == 'self': return
            m = re.match(r'(.*?)(?:\s+as\s+(\w+))?$', body)
            path = (prefix + m.group(1)).strip(); name = m.group(2) or path.split('::')[-1]
            out[name] = path + (' as ' + m.group(2) if m.group(2) else '')
            return
        i = body.index('{'); pre = prefix + body[:i]
        j = body.rindex('}'); inner = body[i + 1:j]
        d = 0; cur = ''; parts = []
        for ch in inner:
            if ch == '{': d += 1
            elif ch == '}': d -= 1
            if ch == ',' and d == 0: parts.append(cur); cur = ''
            else: cur += ch
        parts.append(cur)
        for pt_ in parts:
            if pt_.strip(): rec(pre, pt_)
    rec('', s)
    return out

def parent_use_map(parent_rel):
    raw = open(PLUG + parent_rel, encoding='utf-8').read(); M = mask(raw)
    out = {}
    for m in re.finditer(r'(?m)^(?:pub(?:\([^)]*\))?\s+)?use\s[^;]*;', M):
        out.update(expand_use(raw[m.start():m.end()]))
    return out

TURN_PARENT = {'⏪️time-travel/🧵️instance/🦀️.rs': '⏪️time-travel/🦀️.rs', '⏯️tool-run/🧵️instance/🦀️.rs': '⏯️tool-run/🦀️.rs'}

def rebase_paths(seg, old_dir, new_dir):
    out = []
    for l in seg:
        m = re.search(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', l)
        if m:
            absp = os.path.normpath(os.path.join(old_dir, m.group(1)))
            l = l[:m.start(1)] + os.path.relpath(absp, new_dir) + l[m.end(1):]
        out.append(l)
    return out

def balance_regions(lines):
    stack = []
    for l in lines:
        s = l.strip()
        m = re.match(r'//#region\s+(.*)$', s)
        if m: stack.append(m.group(1)); continue
        m = re.match(r'//#endregion\s+(.*)$', s)
        if m and stack and stack[-1] == m.group(1): stack.pop()
    out = list(lines)
    while out and not out[-1].strip(): out.pop()
    for name in reversed(stack): out.append('//#endregion ' + name)
    return out

def name_ref(text, name, call):
    if call: return re.search(r'(?<![\w])' + re.escape(name) + r'\s*(?:\(|::<)', text) is not None or re.search(r'(?:\.|::)' + re.escape(name) + r'\b(?!\s*:(?!:))', text) is not None
    return re.search(r'(?<![\w])(?<![^.]\.)' + re.escape(name) + r'\b', text) is not None

def emit_all(P, targets):
    model = P['model']
    meth_def, item_def, ttext = analyse_refs(P, targets)
    pool = doc_emoji_pool(); pi_ = iter(pool)
    texts = {}; widen_log = []; exports = {}; defs = {}
    for key, t in targets.items():
        f = model[t.file]; R = f['R']; L = f['L']
        out = []
        nt = new_tree(t)
        def other_refs(name, call, vis='', defining=key):
            scope_all = vis != '' or t.file in SP.PARENT_TEXT
            return [k for k, tx in ttext.items() if k != defining and (scope_all or k[0] == t.file or (t.file in SP.PARENT_TEXT)) and name_ref(tx, name, call)]
        for kind, pi, mis in t.pieces:
            p = f['pieces'][pi]
            gap = [g for g in p.gap if g.strip()]
            if kind == 'impl':
                out_seg = list(p.head_lines)
                for mi in mis:
                    m = p.members[mi]
                    seg = list(R[m.a:m.e])
                    while seg and not seg[-1].strip(): seg.pop()
                    sig_i = m.start - m.a
                    ind, vis, rest = vis_of(seg[sig_i])
                    refs = other_refs(m.name, True, vis)
                    tree_move = (old_tree(t.file) != nt)
                    need = False
                    if p.trait: refs = []; tree_move = False
                    if vis == '' and refs: need = True
                    if vis in ('pub(super)',) and (tree_move or any(new_tree(targets[r]) != nt for r in refs if r in targets)): need = True
                    if need:
                        seg[sig_i] = widen_line(seg[sig_i]); widen_log.append((t.file, t.crate, 'method', m.name, vis or 'private', 'pub(crate)'))
                    out_seg += seg; out_seg.append('')
                while out_seg and not out_seg[-1].strip(): out_seg.pop()
                out_seg.append(R[p.close_idx])
                out += gap + out_seg + ['']
            else:
                seg = list(R[p.s:p.e])
                if p.itkind == 'impl' and p.trait: pass
                elif p.itkind == 'impl':
                    for nm, j in impl_members(R, L, p):
                        ind, vis, rest = vis_of(R[j])
                        refs = other_refs(nm, True, vis)
                        tree_move = (old_tree(t.file) != nt)
                        if (vis == '' and refs) or (vis == 'pub(super)' and tree_move):
                            seg[j - p.s] = widen_line(seg[j - p.s]); widen_log.append((t.file, t.crate, 'assoc', nm, vis or 'private', 'pub(crate)'))
                elif p.itkind == 'mod':
                    if not t.inplace: seg = rebase_paths(seg, os.path.dirname(PLUG + t.file), os.path.dirname(PLUG + t.path))
                else:
                    fi = first_code_line(R, p.s, p.e) - p.s
                    ind, vis, rest = vis_of(seg[fi])
                    nm = re.match(r'[A-Za-z_]\w*', p.name or '')
                    nm = nm.group(0) if nm else None
                    if nm: defs.setdefault(key, {})[nm] = (p.itkind, vis)
                    if nm:
                        call = False
                        refs = other_refs(nm, call, vis)
                        tree_move = (old_tree(t.file) != nt)
                        need = (vis == '' and refs) or (vis == 'pub(super)' and tree_move)
                        if need:
                            seg[fi] = widen_line(seg[fi]); widen_log.append((t.file, t.crate, p.itkind, nm, vis or 'private', 'pub(crate)'))
                            defs[key][nm] = (p.itkind, 'pub(crate)')
                            if p.itkind == 'struct' and nm != 'VcsArtifactApp': seg = struct_fields_widen(seg)
                        elif p.itkind == 'struct' and refs and nm != 'VcsArtifactApp' and not t.inplace:
                            seg = struct_fields_widen(seg)
                out += gap + seg + ['']
        while out and not out[-1].strip(): out.pop()
        # header
        if t.inplace or t.file in MODULE_ROOTED:
            header = list(f['header'])
        else:
            emo = next(pi_)
            what = SHORT[t.crate].replace('_', '-')
            orig_doc = [l for l in f['header'] if l.lstrip().startswith('//!')]
            header = ['//! ' + emo + ' Staged slice of the `' + SLUG[t.file][1] + '` band of the mounted artifact instance: its `' + t.crate + '` members (WP6 step S0 of the instance family).'] + (['//!'] + orig_doc if orig_doc else []) + ['use super::*;']
            ex = [l for l in f['header'] if l.strip() and l.strip() != 'use super::*;' and not l.lstrip().startswith(('//!', '///'))]
            if t.file == '🧵️instance/🦀️.rs': ex = []
            if t.file in TURN_PARENT:
                glob_ = 'time_travel' if t.file.startswith('⏪') else 'tool_run'
                have = set()
                exj = '\n'.join(ex)
                for stm in re.finditer(r'(?m)^[ \t]*(?:#\[[^\]]*\][ \t]*)?(?:pub(?:\([^)]*\))?\s+)?use\s[^;]*;', mask(exj)):
                    have |= set(expand_use(exj[stm.start():stm.end()]).keys())
                body_txt = '\n'.join(out)
                body_m = mask(body_txt)
                pm = parent_use_map(TURN_PARENT[t.file])
                add = []
                for nm_, path_ in sorted(pm.items()):
                    if nm_ in have or not re.search(r'(?<![\w])(?<![^.]\.)' + re.escape(nm_) + r'\b', body_m): continue
                    add.append('use ' + path_ + ';')
                ex = ['use crate::tool_jobs::' + glob_ + '::*;'] + ex + add
            header += ex
        texts[key] = (header, balance_regions(out))
    return texts, widen_log, defs, ttext

SIBLING_FIX = [(re.compile(r'(?<![\w:])envelope_ladder::'), '')]

CRATE_ORDER = ['instance-stores', 'instance-history', 'instance-children', 'instance-jobs', 'instance-docs', 'instance-commands', 'instance-typed-ops', 'instance-render', 'instance-ladders', 'time-travel-turns', 'tool-run-turns', 'instance-app']

def lossless_report(P, targets, out):
    model = P['model']
    orig = collections.Counter(); new = collections.Counter()
    norm = lambda l: re.sub(r'\bpub(?:\((?:crate|super|in [^)]*)\))?\s+', '', l.rstrip())
    for rel, f in model.items():
        for l in f['R']:
            if l.strip(): orig[norm(l)] += 1
    for path, tx in out.items():
        for l in tx.split('\n'):
            if l.strip(): new[norm(l)] += 1
    lost = orig - new; added = new - orig
    return lost, added

