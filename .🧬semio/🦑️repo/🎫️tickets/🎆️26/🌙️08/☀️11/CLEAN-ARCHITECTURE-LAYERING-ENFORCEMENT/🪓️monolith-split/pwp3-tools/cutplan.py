"""🧮️ Closure planner for one lift: seeds (definition names) -> the items that must travel with them (trait impls whose local anchor type moves, private helpers only the moved items use) and the references that would become back edges. usage: python3 cutplan.py <file.rs> <seed,seed,...> [--stay name,name]"""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from rsitems import split_items
import plan_tools as pt
IDENT = re.compile(r'[A-Za-z_]\w*')
DEFKINDS = ('fn', 'struct', 'enum', 'union', 'trait', 'type', 'static', 'const')

def itemize(text):
    out = []
    for it in split_items(text):
        if it.kind in ('marker', 'innerdoc'): continue
        head = it.text
        i = 0; lines = it.text.split('\n')
        while i < len(lines) and (not lines[i].strip() or lines[i].lstrip().startswith(('#[', '///', '//'))): i += 1
        # attribute continuation lines are rare; fall back to first code line
        code = '\n'.join(lines[i:]) if i < len(lines) else it.text
        first = lines[i] if i < len(lines) else ''
        d = dict(it=it, kind=it.kind, first=first.strip(), defname=None, target=None, trait=None, targs=set())
        if it.kind in DEFKINDS:
            m = re.search(r'\b(?:' + '|'.join(DEFKINDS) + r')\s+([A-Za-z_]\w*)', first)
            d['defname'] = m.group(1) if m else None
        elif it.kind == 'thread_local!':
            d['defname'] = None
            d['statics'] = re.findall(r'static\s+([A-Z_0-9]+)', it.text)
        elif it.kind == 'impl':
            tgt, tr = pt.impl_target(first if '{' in first or True else it.text)
            d['target'], d['trait'] = tgt, tr
            hdr = re.split(r'\{', code, 1)[0]
            d['hdr'] = hdr
            d['targs'] = set(IDENT.findall(hdr))
        d['toks'] = pt.tokens(pt.strip(it.text))
        d['ids'] = set(IDENT.findall(pt.strip(it.text)))
        out.append(d)
    return out

def plan(path, seeds, stay=()):
    text = open(path, encoding='utf8').read()
    items = itemize(text)
    defs = {}
    for d in items:
        for n in ([d['defname']] if d['defname'] else []) + d.get('statics', []): defs[n] = d
    moved = set(); queue = []
    for s in seeds:
        if s not in defs: raise SystemExit('seed not found: ' + s)
        moved.add(id(defs[s]))
    changed = True
    reason = {}
    while changed:
        changed = False
        movednames = {n for n, d in defs.items() if id(d) in moved}
        for d in items:
            if id(d) in moved: continue
            if d['kind'] == 'impl':
                anchors = {n for n in d['targs'] if n in defs and id(defs[n]) is not id(d)}
                tgt_moved = d['target'] in movednames
                if tgt_moved or (d['trait'] in movednames and d['trait']):
                    moved.add(id(d)); reason[id(d)] = 'impl of ' + str(d['target']); changed = True
                elif any(n in movednames for n in anchors) and d['target'] not in defs:
                    moved.add(id(d)); reason[id(d)] = 'foreign-self impl with moved arg ' + str(sorted(anchors & movednames)); changed = True
        usedby = {}
        for n, d in defs.items():
            if id(d) in moved or n in stay: continue
            if d['kind'] not in ('fn', 'const', 'static', 'type'): continue
            users = [e for e in items if e is not d and n in e['toks'] | e['ids']]
            if users and all(id(u) in moved for u in users):
                moved.add(id(d)); reason[id(d)] = 'helper only used by moved'; changed = True
    return items, defs, moved, reason

if __name__ == '__main__':
    path = sys.argv[1]; seeds = [s for s in sys.argv[2].split(',') if s]
    stay = set(sys.argv[sys.argv.index('--stay') + 1].split(',')) if '--stay' in sys.argv else set()
    items, defs, moved, reason = plan(path, seeds, stay)
    mv_lines = 0
    print('MOVED:')
    for d in items:
        if id(d) in moved:
            n = d['it'].end - d['it'].start + 1; mv_lines += n
            tag = 'seed' if d['defname'] in seeds else reason.get(id(d), '?')
            print(f"  {d['it'].start+1:4d}-{d['it'].end+1:4d} {d['kind']:6s} {(d['defname'] or d['first'])[:60]:60s} [{tag}]")
    print('moved lines', mv_lines)
    movednames = {n for n, d in defs.items() if id(d) in moved}
    print('BACK REFS (moved -> staying defs):')
    for d in items:
        if id(d) not in moved: continue
        for n, t in defs.items():
            if id(t) in moved or n in ('Self',): continue
            if n in d['toks'] and n not in pt.GENERIC and len(n) > 2:
                print(f"  {(d['defname'] or d['first'])[:40]} -> {n} ({t['kind']} @{t['it'].start+1})")
