"""✂️ Item mover: cuts named column-0 items (with docs, attributes and impl blocks of named types) out of one Rust file and appends them to another; checks the cut for dangling references."""
import re, sys
from model import *

def select(raw, names, impls=True):
    its = items(raw); chosen = []; lines = raw.split('\n')
    for it in its:
        nm = item_name(it)
        if it['kind'] == 'thread_local!':
            inner = set(re.findall(r'static\s+(\w+)\s*:', '\n'.join(lines[it['start']:it['end'] + 1])))
            if inner & names: chosen.append(dict(it, tl=inner & names))
            continue
        if nm in names and it['kind'] != 'impl': chosen.append(it)
        elif impls and it['kind'] == 'impl':
            head = it['name']
            m = re.match(r'impl(?:<[^>]*>)?\s+(?:[\w:<>, ]+\s+for\s+)?([A-Za-z_]\w*)', head)
            if m and m.group(1) in names: chosen.append(it)
    return its, chosen

def ref_report(raw, names):
    """For the file text: (moved items' references to remaining top-level names, remaining items' references to moved names)."""
    its, chosen = select(raw, names)
    ids = {id(c) for c in chosen}
    lines = raw.split('\n')
    def body(it): return strip('\n'.join(lines[it['start']:it['end'] + 1]))
    rem_defs = {item_name(i) for i in its if id(i) not in ids and item_name(i) and i['kind'] not in ('use', 'impl')}
    moved_defs = {item_name(i) for i in chosen if item_name(i)}
    tok = lambda s: set(re.findall(r'(?<![\w.])([A-Za-z_]\w*)', s))
    m2r = set(); r2m = set()
    for i in chosen: m2r |= tok(body(i)) & rem_defs
    for i in its:
        if id(i) not in ids and i['kind'] != 'use': r2m |= tok(body(i)) & moved_defs
    return sorted(m2r - moved_defs), sorted(r2m)

def cut(path, names, dst_path=None, region=None, dry=False):
    raw = open(path, encoding='utf8').read()
    its, chosen = select(raw, names)
    found = {item_name(c) for c in chosen if item_name(c)}
    for c in chosen:
        found |= c.get('tl', set())
    missing = set(names) - found
    assert not missing, ('missing', missing)
    lines = raw.split('\n')
    spans = sorted((c['start'], c['end']) for c in chosen)
    text = '\n\n'.join('\n'.join(lines[s:e + 1]) for s, e in spans)
    keep = []; skip = set()
    for s, e in spans: skip |= set(range(s, e + 1))
    for i, l in enumerate(lines):
        if i not in skip: keep.append(l)
    out = re.sub(r'\n{3,}', '\n\n', '\n'.join(keep))
    if dry: return text, out
    open(path, 'w', encoding='utf8').write(out)
    if dst_path:
        d = open(dst_path, encoding='utf8').read().rstrip('\n')
        block = text if not region else '\n//#region %s\n\n%s\n\n//#endregion %s' % (region, text, region)
        open(dst_path, 'w', encoding='utf8').write(d + '\n\n' + block + '\n')
    return text, out

if __name__ == '__main__':
    path = sys.argv[1]; names = set(sys.argv[2:])
    raw = open(path, encoding='utf8').read()
    m2r, r2m = ref_report(raw, names)
    print('moved items reference remaining:', m2r)
    print('remaining items reference moved:', r2m)
