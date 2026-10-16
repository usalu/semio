# 🧬️ WP4 T1: ShellState -> 15 component structs + composer, tier-1 methods into component impls with component parameters.
# usage: t1.py <post-S0 tree> <out tree>      (both sandbox dirs; nothing in the repo is touched)
import re, sys, os, json, collections, shutil, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from t1lib import *
from units import *
from t1rewrite import *
from t1extra import *
import assign

TIER1_CRATES = [c for c in assign.RANK[:assign.RANK.index('shell-commands') + 1]]
FOLDER2CRATE = {}
for m in re.finditer(r"^\| \d+ \| `(shell-[a-z-]+)` \| `([^`]+)` \|", plan, re.M):
    FOLDER2CRATE[m.group(2).replace('shell-', '', 1)] = m.group(1)

def crate_of(path):
    m = re.search(r'/📥️staging/([^/]+)/', path)
    return FOLDER2CRATE.get(m.group(1)) if m else None

def parse_state_struct(text):
    src = Src(text)
    for it in top_items(src):
        if it['kind'] == 'struct' and re.match(r'pub struct ShellState\b', it['head']):
            first = it['first']; last = it['last']; break
    else: raise SystemExit('no ShellState struct')
    fields = []; i = first + 1; trivia = []
    # leading trivia of the struct itself
    s = first
    while s > 0 and src.lines[s-1].lstrip().startswith(('///', '#[')): s -= 1
    pending = []
    while i < last:
        l = src.lines[i]; bl = src.bl[i]
        if l.strip() == '': i += 1; continue
        if bl.strip() == '' or bl.lstrip().startswith('#['):
            pending.append(l)
            # attribute may span lines: consume until brackets balanced
            if bl.lstrip().startswith('#[') and bl.count('[') > bl.count(']'):
                d = bl.count('[') - bl.count(']')
                while d > 0:
                    i += 1; pending.append(src.lines[i]); d += src.bl[i].count('[') - src.bl[i].count(']')
            i += 1; continue
        m = re.match(r'^    (?:pub(?:\([^)]*\))?\s+)?([A-Za-z_]\w*)\s*:', bl)
        assert m, (i, l)
        j = i; d = 0
        while True:
            d += src.bl[j].count('<') + src.bl[j].count('(') + src.bl[j].count('[') + src.bl[j].count('{') - src.bl[j].count('>') - src.bl[j].count(')') - src.bl[j].count(']') - src.bl[j].count('}') + src.bl[j].count('->')
            if d <= 0 and src.bl[j].rstrip().endswith(','): break
            j += 1
        decl = src.lines[i:j + 1]
        fields.append(dict(name=m.group(1), lines=pending + decl)); pending = []
        i = j + 1
    return src, s, last, fields

def widen_pub(lines):
    out = []
    for l in lines:
        m = re.match(r'^(    )(?:pub(?:\([^)]*\))?\s+)?([A-Za-z_]\w*)(\s*:.*)$', l)
        if m and not l.lstrip().startswith(('#', '/')): out.append(m.group(1) + 'pub ' + m.group(2) + m.group(3))
        else: out.append(l)
    return out

COMP_DOC = {
 'ChromeState': 'transient chrome: errors, screen metrics, fullscreen, the per-frame build and present cursors',
 'SurfaceState': 'the surfaces the shell hosts: world3d, node graph, tiled map, icon render and board2d states with their retirement ledger',
 'TutorialState': 'the tutorial runtime and its deferred document operations',
 'SettleState': 'the settle lane: deferred actions, owed refresh scope, the pump and pending shell I/O',
 'PluginState': 'plugins, extensions, the host capability, panel-run ledger, faults and the app catalogue',
 'HistoryState': 'history cursor, entries, time travel, reprojection and the remembered local folders',
 'PrefsState': 'appearance, locale, terminology, driver and theme drafts, merge policy and open conflicts',
 'DirectoryState': 'identity, the directory client, agent bridge, inference port, hub workspace, space administration and presence',
 'SyncState': 'the document sync backbone, channel, status, link, hub documents, reseed and auto check-in',
 'DocumentState': 'the active session, per-window documents, spawned apps and document identities',
 'OpeningState': 'URI history, pending opens, import transfers, document services and the document opening pipeline',
 'WindowState': 'active window, engagement and measures chrome, window topology journal and per-window overrides',
 'ShellDockState': 'the dock model, anchors, drags, splits, layouts and window content geometry',
 'ShellInputState': 'scroll, overlay, selection, context menu, tree drag, widget maps and the retained input geometry',
 'CommandState': 'search and find palette state and staged command arguments',
}

COMP_EMOJI = {'ChromeState': '🎚️', 'SurfaceState': '🫧️', 'TutorialState': '🎓️', 'SettleState': '🫀️', 'PluginState': '🧲️', 'HistoryState': '⏪️', 'PrefsState': '🎛️', 'DirectoryState': '📒️', 'SyncState': '🔃️', 'DocumentState': '📄️', 'OpeningState': '📂️', 'WindowState': '🔲️', 'ShellDockState': '⚓️', 'ShellInputState': '🕹️', 'CommandState': '🎹️'}

def component_files(fields):
    byname = {f['name']: f for f in fields}
    out = {}
    for c in COMPS['components']:
        S = c['struct']
        missing = [n for n in c['fields'] if n not in byname]
        assert not missing, (S, missing)
        lines = [f'/// {COMP_EMOJI[S]} `{S}`: {COMP_DOC[S]}.', '///', '/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `' + FIELD_OF[S] + '`.']
        lines.append(f'pub struct {S} {{')
        for n in c['fields']:
            lines += widen_pub(byname[n]['lines'])
        lines.append('}')
        out[S] = lines
    return out

def composer_struct(old_start):
    lines = ['/// 🐙️ The shell state: exactly the fifteen component structs, which own the fields and the methods of the former god object.']
    lines.append('pub struct ShellState {')
    for c in COMPS['components']:
        lines.append(f'    pub {FIELD_OF[c["struct"]]}: {c["struct"]},')
    lines.append('}')
    return lines

def main(tree_in, tree_out):
    root = os.path.realpath(tree_out)
    assert root.startswith('/private/tmp/') or root.startswith('/tmp/')
    if os.path.exists(root): shutil.rmtree(root)
    shutil.copytree(tree_in, root)
    report = []
    # --- parent: composer
    ppath = root + '/🦀️.rs'
    ptext = open(ppath, encoding='utf-8').read()
    psrc, s, last, fields = parse_state_struct(ptext)
    comp_lines = component_files(fields)
    new_p = psrc.lines[:s] + composer_struct(s) + psrc.lines[last + 1:]
    ptext2 = '\n'.join(new_p)
    # component struct files per crate (tier-1)
    crate_structs = collections.OrderedDict()
    for c in COMPS['components']: crate_structs.setdefault(c['crate'], []).append(c['struct'])
    mounts = []
    for crate, structs in crate_structs.items():
        folder = CRATE_FOLDER_T1(crate)
        rel = f'📥️staging/{folder}/🧬️component/🦀️.rs'
        emoji = folder[:2]
        body = []
        for S in structs:
            body += comp_lines[S] + ['']
        text = '\n'.join([f'//! {emoji} `{crate}` component structs: {", ".join("`"+x+"`" for x in structs)} (RENDERER-WP4 step T1).', '//!', '//! Fields are cut from the former `ShellState` in declaration order and keep their names.', '', 'use super::*;', ''] + body).rstrip() + '\n'
        p = os.path.join(root, rel); os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, 'w', encoding='utf-8').write(text)
        mods = 'staged_' + re.sub(r'[^a-z0-9]+', '_', crate.replace('shell-', '')) + '__component'
        mounts.append((rel, mods))
    block = ['//#region 🧬️RendererWp4Components']
    for rel, mods in mounts:
        block += [f'#[path = "{rel}"]', f'mod {mods};', f'pub use {mods}::*;']
    block.append('//#endregion 🧬️RendererWp4Components')
    anchor = '//#region 📥️RendererWp4Staging'
    assert ptext2.count(anchor) == 1
    ptext2 = ptext2.replace(anchor, '\n'.join(block) + '\n\n' + anchor)
    i0 = ptext2.index('pub(crate) const SHELL_SOURCE')
    i1 = ptext2.index('\n);', i0)
    inc = ''.join(f',\n    include_str!("{rel}")' for rel, mods in mounts)
    ptext2 = ptext2[:i1] + inc + ptext2[i1:]
    open(ppath, 'w', encoding='utf-8').write(ptext2)
    stats = rewrite_staging(root, report)
    pt0 = open(ppath, encoding='utf-8').read()
    for path in FREE_MODS:
        rel = os.path.relpath(path, root)
        m = re.search(r'#\[path = "' + re.escape(rel) + r'"\]\nmod (\w+);\n(pub(?:\([a-z]+\))? use \w+::\*;\n)?', pt0)
        assert m, rel
        if not m.group(2):
            pt0 = pt0[:m.end()] + f'pub(crate) use {m.group(1)}::*;\n' + pt0[m.end():]
    open(ppath, 'w', encoding='utf-8').write(pt0)
    for path in sorted(glob.glob(root + '/*/🦀️.rs') + [x for x in glob.glob(root + '/📥️staging/*/*/🦀️.rs') if '/🧬️component/' not in x]):
        n = rewrite_free_defs_in_file(path, report)
        if n: stats['freefn:' + os.path.basename(os.path.dirname(path))[:6]] += n
    # Drop for ShellState lives in the parent
    pt = open(ppath, encoding='utf-8').read()
    psrc2 = Src(pt)
    for it in top_items(psrc2):
        if it['kind'] == 'impl' and re.match(r'impl\s+Drop\s+for\s+ShellState', it['head']):
            for mm in top_items(psrc2, it['first'] + 1, it['last'], 4):
                raw = '\n'.join(psrc2.lines[mm['first']:mm['last'] + 1])
                new_raw = rewrite_fn(raw, 'drop', 'usecase', None, report, 'self', False)
                pl = psrc2.lines[:mm['first']] + new_raw.split('\n') + psrc2.lines[mm['last'] + 1:]
                open(ppath, 'w', encoding='utf-8').write('\n'.join(pl))
                stats['drop'] += 1
    for path in sorted(glob.glob(root + '/**/🦀️.rs', recursive=True)):
        if not os.path.isfile(path): continue
        tx = open(path, encoding='utf-8').read()
        bl = blank(tx)
        edits = []
        for m in re.finditer(r'(?<![\w:])ShellState\s*::\s*([A-Za-z_]\w*)\s*\(', bl):
            x = m.group(1)
            if x in TIER1 and PLAN1[x].get('recv') != 'free' and PLAN1[x]['recv'] is None:
                edits.append((m.start(), m.start(1), '' if PLAN1[x]['struct'] is None else PLAN1[x]['struct'] + '::'))
            elif x in TIER1 and PLAN1[x].get('recv') in ('mut', 'ref', 'value'):
                report.append((os.path.relpath(path, root), f'ShellState::{x} used as UFCS'))
        if edits:
            for s, e, r in sorted(edits, reverse=True): tx = tx[:s] + r + tx[e:]
            open(path, 'w', encoding='utf-8').write(tx); stats['assoc-paths'] += len(edits)
    print('members', dict(stats))
    return root, report


from s0 import regions_of, transition

FREE_MODS = []

def rewrite_staging(root, report):
    stats = collections.Counter()
    for path in sorted(glob.glob(root + '/📥️staging/*/*/🦀️.rs')):
        if '/🧬️component/' in path: continue
        crate = crate_of(path)
        text = open(path, encoding='utf-8').read()
        src, units = split_file(path)
        snap, allr = regions_of(src)
        lines = list(src.lines)
        free_out = []
        for u in reversed([x for x in units if x.state_impl]):
            tier1 = crate in TIER1_CRATES
            if tier1:
                header = lines[u.first:u.code + 1]
                assert header[-1].strip() == 'impl ShellState {', header[-1]
                blocks = []; cur_S = None; body = []; inner_cur = []
                def close_block():
                    nonlocal body, inner_cur
                    inner_cur = transition(inner_cur, [], body)
                    body.append('}')
                    blocks.extend(body); blocks.append('')
                    body = []
                first_member = True
                for m in u.members:
                    S = PLAN1[m.name]['struct']
                    raw = '\n'.join(lines[m.first:m.last + 1])
                    if S is None:
                        new_raw = rewrite_member(raw, m.name, 'tier1', None, report)
                        free_out.append(m.lead + [(l[4:] if l.startswith('    ') else l) for l in new_raw.split('\n')] + m.trail)
                        stats['assoc-free'] += 1
                        continue
                    new_raw = rewrite_member(raw, m.name, 'tier1', S, report)
                    stats['tier1'] += 1
                    if S != cur_S:
                        if cur_S is not None: close_block()
                        h = list(header); h[-1] = f'impl {S} {{'
                        body = list(h); cur_S = S; inner_cur = []; first_member = True
                    inner = [r for r in snap[m.first] if r.idx >= u.code]
                    if not first_member: body.append('')
                    first_member = False
                    inner_cur = transition(inner_cur, inner, body)
                    body += m.lead + new_raw.split('\n') + m.trail
                if cur_S is not None: close_block()
                while blocks and blocks[-1] == '': blocks.pop()
                lines[u.first:u.last + 1] = blocks
            else:
                for m in reversed(u.members):
                    raw = '\n'.join(lines[m.first:m.last + 1])
                    mode = 'usecase'
                    new_raw = rewrite_member(raw, m.name, mode, None, report) if m.name != 'new' else regroup_new(raw, report)
                    stats['usecase'] += 1
                    lines[m.first:m.last + 1] = new_raw.split('\n')
        if free_out:
            while lines and lines[-1] == '': lines.pop()
            for blk in reversed(free_out):
                lines += [''] + blk
            if path not in FREE_MODS: FREE_MODS.append(path)
        open(path, 'w', encoding='utf-8').write('\n'.join(lines) + '\n')
    return stats

def CRATE_FOLDER_T1(crate):
    return assign_folder[crate]

assign_folder = {}
for m in re.finditer(r"^\| \d+ \| `(shell-[a-z-]+)` \| `([^`]+)` \|", plan, re.M):
    assign_folder[m.group(1)] = m.group(2).replace('shell-', '', 1)

if __name__ == '__main__':
    root, report = main(sys.argv[1], sys.argv[2])
    print('composer + components written to', root)
    import collections as C
    print(len(report), 'reports')
    c = C.Counter(r[1].split(' ')[0] + ' ' + r[1].split(' ')[1] if len(r[1].split(' '))>1 else r[1] for r in report)
    print(c.most_common(20))
    json.dump(report, open(GEN + '/rwp4-t1-report.json', 'w'), ensure_ascii=False, indent=0)
