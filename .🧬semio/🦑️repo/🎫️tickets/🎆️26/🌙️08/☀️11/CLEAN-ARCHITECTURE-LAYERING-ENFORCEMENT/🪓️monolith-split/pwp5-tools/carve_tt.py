"""⏪️ S0 cut of `⏪️time-travel/🦀️.rs` (3,456 lines) along the plan: the root keeps Limits, Outcome, Ledger, StoreState and Supersessions (crate `time-travel`); the panel regions become two piece files of crate `time-travel-panel`; the Driver region and the history view model close over `command` go to the lift file `🎮️command/🛬️lifted/⏪️time-travel/🦀️.rs`; `time_travel_arg_generation` stays in crate `time-travel` (it names the history-edit generation argument; it reads the generic `tool_run_arg_u64` downward). Every moved item is mounted back with `#[path] mod x; pub use x::*;` so no module path of the monolith changes. usage: python3 carve_tt.py [--plan]"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import lib5 as L, cutplan as cp, mv, plan_tools as pt
from rsitems import split_items
TT = L.PLUG + '/⏪️time-travel/🦀️.rs'
CMD_LIFT = L.PLUG + '/🎮️command/🛬️lifted/⏪️time-travel/🦀️.rs'
PANEL_DIR = L.PLUG + '/⏪️time-travel/🪟️panel'
PIECES = {'panel_pointer': ('🔢️pointer', 'pointer', '🔢️ The draft editor input rows of the history panel: RFC 6901 pointer reads and edits of the draft value, variant switching, snaps, options, coercion, selection values and entity names.'),
          'panel_sections': ('🖼️sections', 'sections', '🖼️ The history panel sections: row refusals of the mutation controls, the band, reprojection and editor sections, control rows, the input row builder and the panel limits.')}
REGION_DEST = {'Limits': 'tt', 'Outcome': 'tt', 'Ledger': 'tt', 'StoreState': 'tt', 'Supersessions': 'tt', 'PanelView': 'panel_sections', 'Pointer': 'panel_pointer', 'Driver': 'cmd', 'Panel': 'panel_sections'}
CMD_SEEDS = ['HISTORY_ROW_MUTATION_ROWS', 'history_mutation_entry', 'HistoryPanelText', 'MutationRowRefusals', 'history_severity_tone', 'history_mutation_description']
TO_PANEL = ['TIME_TRAVEL_ENTITY_NAME_VISITS', 'TIME_TRAVEL_SHORT_ID_CHARS']
TO_TT = ['time_travel_selection_value', 'time_travel_arg_generation']
TO_TOOLRUN = []
RANK = {'cmd': 0, 'panel_pointer': 1, 'panel_sections': 1, 'toolrun': 2, 'tt': 3}
CHILDREN = ['⏪️time-travel/♻️retirement/🦀️.rs', '⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs', '⏪️time-travel/🧵️instance/🦀️.rs']

def plan(text):
    items = split_items(text)
    covered = set()
    for it in items:
        for i in range(it.start, it.end + 1): covered.add(i)
    lines = text.split('\n')
    miss = [i for i, l in enumerate(lines) if l.strip() and i not in covered]
    assert not miss, ('uncovered lines', miss[:5])
    cps = {d['it'].start: d for d in cp.itemize(text)}
    stack = []; region_of = {}; assign = {}
    for i, it in enumerate(items):
        if it.kind == 'marker':
            m = re.match(r'//#(end)?region\s+(.*)$', it.text.strip()); n = m.group(2).strip()
            if m.group(1): stack.pop()
            else: stack.append(n)
            continue
        top = stack[0].replace('🔖️', '') if stack else None
        region_of[i] = top
        assign[i] = REGION_DEST[top] if top else 'tt'
    defs = {}
    for i, it in enumerate(items):
        d = cps.get(it.start)
        if d is None: continue
        for n in ([d['defname']] if d['defname'] else []) + d.get('statics', []): defs[n] = (i, d)
    for n in CMD_SEEDS + TO_PANEL + TO_TT + TO_TOOLRUN:
        assert n in defs, n
    for n in TO_PANEL: assign[defs[n][0]] = 'panel_sections'
    for n in TO_TT: assign[defs[n][0]] = 'tt'
    for n in TO_TOOLRUN: assign[defs[n][0]] = 'toolrun'
    moved = {defs[n][0] for n in CMD_SEEDS}
    seeds = set(moved)
    changed = True
    while changed:
        changed = False
        names = {n for n, (i, d) in defs.items() if i in moved}
        for i, it in enumerate(items):
            d = cps.get(it.start)
            if d is None or i in moved: continue
            if d['kind'] == 'impl' and d['target'] in names:
                moved.add(i); changed = True; continue
            if assign[i] not in ('panel_pointer', 'panel_sections'): continue
            nm = d['defname']
            if not nm or nm in pt.GENERIC: continue
            for j in list(moved):
                dj = cps.get(items[j].start)
                if dj and nm in (dj['toks'] | dj['ids']):
                    moved.add(i); changed = True; break
    for i in moved:
        if assign[i] in ('tt',) and i not in seeds: raise SystemExit('closure reached a time-travel item: ' + str(cps[items[i].start]['defname']))
        assign[i] = 'cmd'
    for i, it in enumerate(items):
        d = cps.get(it.start)
        if d and d['kind'] == 'impl' and d['target'] in defs and defs[d['target']][0] != i:
            td = assign[defs[d['target']][0]]
            if assign[i] != td: assign[i] = td
    return items, cps, defs, assign, region_of

def rank_check(items, cps, defs, assign):
    bad = []
    for i, it in enumerate(items):
        d = cps.get(it.start)
        if d is None: continue
        me = assign[i]
        for n, (j, dj) in defs.items():
            if j == i or n in pt.GENERIC or len(n) < 3: continue
            if n in d['toks'] and RANK.get(assign[j], 3) > RANK.get(me, 3): bad.append((d['defname'] or d['first'][:40], me, n, assign[j]))
    return bad

def ids_of(text):
    t = re.sub(r'//[^\n]*', '', text)
    return set(re.findall(r'[A-Za-z_]\w*', t)), set(re.findall(r'\.\s*([A-Za-z_]\w*)', t)), set(re.findall(r'\b([A-Za-z_]\w*)\s*:(?!:)', t))

def widen_for(body, others):
    ids, dots, inits = ids_of(others)
    priv = mv.names_private(body)
    names = {n for n, k in priv.items() if (n in ids if k == 'item' else n in dots if k == 'impl' else (n in dots or n in inits))}
    return mv.widen(body, names), sorted(names)

def build(dry=False):
    text = L.rd(TT)
    items, cps, defs, assign, region_of = plan(text)
    bad = rank_check(items, cps, defs, assign)
    dests = ['tt', 'cmd', 'panel_pointer', 'panel_sections', 'toolrun']
    out = L.assemble(items, assign, dests)
    texts = {d: L.render(out[d]) if out[d] else '' for d in dests}
    counts = {d: sum(1 for k, v in out[d] if k == 'item') for d in dests}
    lines_of = {d: sum(v.end - v.start + 1 for k, v in out[d] if k == 'item') for d in dests}
    children = '\n'.join(L.rd(L.PLUG + '/' + c) for c in CHILDREN)
    report = dict(counts=counts, lines=lines_of, rank_violations=bad, moved_to={})
    for i, a in assign.items():
        if a != REGION_DEST.get(region_of.get(i), 'tt') and (cps.get(items[i].start) or {}).get('defname'):
            report['moved_to'][cps[items[i].start]['defname']] = a
    return text, items, assign, texts, counts, lines_of, children, report

def main():
    text, items, assign, texts, counts, lines_of, children, report = build()
    print(json.dumps(report, ensure_ascii=False, indent=1))
    if '--plan' in sys.argv: return
    names = {}
    wide = {}
    for d in ('cmd', 'panel_pointer', 'panel_sections'):
        others = children + '\n' + '\n'.join(texts[x] for x in texts if x != d)
        body = texts[d]
        body = re.sub(r'\bpub\(super\)', 'pub(crate)', body)
        texts[d], wide[d] = widen_for(body, others)
    print('widened', {d: wide[d] for d in wide})
    mounts = []
    for d, (folder, mod, doc) in PIECES.items():
        mounts.append(f'#[path = "🪟️panel/{folder}/🦀️.rs"]\nmod panel_{mod};\npub use panel_{mod}::*;\n')
        head = f'//! {doc}\n\nuse super::*;\n\n'
        L.write(PANEL_DIR + '/' + folder + '/🦀️.rs', head + texts[d])
    rel = os.path.relpath(CMD_LIFT, os.path.dirname(TT))
    assert len(os.path.relpath(CMD_LIFT, L.ROOT).encode()) <= 240
    mounts.append(f'#[path = "{rel}"]\nmod lifted_command;\npub use lifted_command::*;\n')
    mount_text = '\n'.join(mounts)
    root = texts['tt']
    wide['tt'] = []
    anchor = '//#region 🔖️Limits'
    assert root.count(anchor) == 1
    root = root.replace(anchor, mount_text + '\n' + anchor, 1)
    if counts['cmd']:
        hdr = (f'//! ⏪️ Lift file: {counts["cmd"]} item(s), about {lines_of["cmd"]} lines, of `⏪️time-travel/🦀️.rs` that belong to crate `command` (WP3): the history view vocabulary the instance command routes read (mutation pages, member histories and view stamps) together with the history wire row, its panel view model and the panel texts.\n'
               '//!\n//! Cut by WP5; WP3 wires this file with one `mod` line in the crate root at the flip. Until `command` exists, `⏪️time-travel/🦀️.rs` mounts it as a child module and re-exports it with `pub use`.\n\nuse super::*;\n\n')
        L.write(CMD_LIFT, hdr + texts['cmd'])
    L.write(TT, root)
    json.dump(dict(report=report, wide=wide, toolrun=texts['toolrun']), open(L.HERE + '/../🗑️generated/pwp5-carve-tt.json', 'w'), ensure_ascii=False, indent=1)

if __name__ == '__main__': main()
