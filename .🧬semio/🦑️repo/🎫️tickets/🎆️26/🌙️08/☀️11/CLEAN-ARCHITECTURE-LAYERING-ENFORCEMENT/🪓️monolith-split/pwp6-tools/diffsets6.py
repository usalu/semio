import sys, json, re, os, collections
sys.path.insert(0, '.')
import importlib.util
spec = importlib.util.spec_from_file_location('oracle6', os.path.join(os.path.dirname(os.path.abspath(__file__)), 'oracle6.py'))
GEN = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated'
def parse(path, tag):
    sb = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/pwp6-sb-' + tag + '/'
    c = collections.Counter()
    for line in open(path, encoding='utf-8', errors='replace'):
        line = line.strip()
        if not line.startswith('{'): continue
        try: d = json.loads(line)
        except Exception: continue
        if d.get('level') != 'error' or str(d.get('message', '')).startswith('aborting due to'): continue
        code = (d.get('code') or {}).get('code') or ''
        sp = next((s for s in d.get('spans', []) if s.get('is_primary')), None)
        label = text = ''
        if sp:
            label = sp.get('label') or ''
            t = sp.get('text') or []
            if t:
                l = t[0]
                text = (l['text'][l['highlight_start'] - 1:l['highlight_end'] - 1] if len(t) == 1 else l['text'][l['highlight_start'] - 1:]).strip()[:80]
        msg = str(d.get('message', ''))
        c[(code, msg, text, label)] += 1
    return c
if __name__ == '__main__':
    tag, cfg = sys.argv[1], sys.argv[2]
    a = parse(GEN + '/pwp6-oracle-%s-A-%s.jsonl' % (tag, cfg), tag); b = parse(GEN + '/pwp6-oracle-%s-B-%s.jsonl' % (tag, cfg), tag)
    new = [k for k in a if k not in b]; old = [k for k in b if k not in a]
    print('A', sum(a.values()), 'B', sum(b.values()), 'keys A', len(a), 'B', len(b), 'only-new keys', len(new), 'only-old keys', len(old))
    for k in sorted(new): print('  +', a[k], k)
    for k in sorted(old): print('  -', b[k], k)
