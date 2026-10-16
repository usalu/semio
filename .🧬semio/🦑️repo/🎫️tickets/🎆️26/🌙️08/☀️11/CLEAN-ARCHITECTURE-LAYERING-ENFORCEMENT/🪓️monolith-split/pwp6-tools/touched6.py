import sys; sys.path.insert(0, '.')
from common6 import *
F = GEN + '/pwp6-touched.json'
def load():
    return json.load(open(F, encoding='utf-8')) if os.path.exists(F) else dict(created=[], modified=[], deleted=[])
def merge(created=(), modified=(), deleted=()):
    t = load(); c, m, d = set(t['created']), set(t['modified']), set(t['deleted'])
    for x in created:
        if x in d: d.discard(x); m.add(x)       # deleted earlier, written again: a modification of an original
        elif x not in m: c.add(x)
    for x in modified:
        if x not in c: m.add(x)
    for x in deleted:
        if x in c: c.discard(x)                 # created and deleted by me: no trace
        else: m.discard(x); d.add(x)
    json.dump(dict(created=sorted(c), modified=sorted(m), deleted=sorted(d)), open(F, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
