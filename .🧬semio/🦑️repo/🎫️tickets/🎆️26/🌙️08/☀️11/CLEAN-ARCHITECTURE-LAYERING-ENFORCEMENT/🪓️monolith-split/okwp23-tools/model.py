# 🧮 Item-to-crate model of the WP2/WP3 carve: the planner's final assignment (cof), matched to the live files by item start line; file -> crate rollups.
import json, os, re, sys, functools, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools')
from okrs import items
R = '/Users/ueli/Documents/semio'
M = R + '/🧰️framework/🛍️products/💻️os/🔨️modules'
S = M + '/🏪️store'
GEN = HERE + '/../🗑️generated'

@functools.lru_cache(maxsize=None)
def cof():
    return json.load(open(GEN + '/okwp23-cof.json'))

def short(f): return f.replace(M + '/', 'M/')

OVERRIDE = {}   # (file, item name) -> crate key; filled by assign.py
OVERRIDE_RX = []   # (file, compiled regex over the code of an item, crate key): also assigns macro-invocation items
FILE_OVERRIDE = {}   # file -> crate key (files the planner left unassigned or mis-assigned); filled by assign.py

class FileModel:
    def __init__(self, path):
        self.path = path
        self.src = open(path, encoding='utf8').read()
        self.items = items(self.src)
        self.lines = self.src.split('\n')
        theirs = cof()['items'].get(path, [])
        by_start = {}
        for t in theirs: by_start.setdefault(t['start'], []).append(t)
        for it in self.items:
            it['sl'] = self.src.count('\n', 0, it['start']) + 1
            it['el'] = self.src.count('\n', 0, it['end']) + 1
            it['crate'] = None
            best = 0
            for x in theirs:
                ov = min(it['el'], x['end']) - max(it['sl'], x['start']) + 1
                if ov > best and (x['kind'] == it['kind'] or ov * 2 >= (it['el'] - it['sl'] + 1)):
                    best = ov; it['crate'] = x['crate']
        for it in self.items:
            ov = OVERRIDE.get((path, it['name']))
            if ov: it['crate'] = ov
            code = None
            for (rf, rx, rc) in OVERRIDE_RX:
                if rf != path: continue
                if code is None: code = re.sub(r'(?m)^\s*//.*$', '', it['text'])
                if rx.search(code):
                    it['crate'] = rc
                    if it['kind'] is None: it['forced'] = True; it['name'] = it['name'] or ('macro ' + re.sub(r'\s+', ' ', code.strip())[:50])
        self.crates = collections.Counter()
        for it in self.items:
            if it['crate']: self.crates[it['crate']] += it['el'] - it['sl'] + 1
    def main_crate(self):
        return self.crates.most_common(1)[0][0] if self.crates else None

@functools.lru_cache(maxsize=None)
def file_model(path): return FileModel(path)

def file_crates():
    """file -> main crate for every file known to the planner"""
    out = {}
    for f in cof()['items']:
        if os.path.exists(f): out[f] = file_model(f).main_crate()
    return out

if __name__ == '__main__':
    miss = 0
    for f in sorted(cof()['items']):
        if not os.path.exists(f): print('gone', f); continue
        fm = file_model(f)
        unassigned = [it for it in fm.items if it['kind'] in ('fn', 'struct', 'enum', 'impl', 'trait', 'const', 'static', 'type', 'union', 'macro_rules') and not it['crate']]
        if unassigned:
            miss += len(unassigned)
            print(short(f)[:80], 'unassigned', [(i['sl'], i['kind'], (i['name'] or '')[:30]) for i in unassigned[:4]])
    print('unassigned items', miss)


def crate_of_file(f, w=None):
    """crate key for any file of the kernel world: planner files by their main crate, other files (tests, fixtures) by the item that mounts them in the host file (else the host's crate)."""
    import ns
    w = w or ns.world()
    if f in FILE_OVERRIDE: return FILE_OVERRIDE[f]
    if f in cof()['items']: return file_model(f).main_crate()
    m = w.files.get(f)
    if not m or not m.parent: return None
    P = m.parent
    pf = P.file
    if pf in cof()['items'] and os.path.exists(pf):
        fm = file_model(pf)
        for it in fm.items:
            if it['kind'] == 'mod' and it['name'] == m.name and it['crate']: return it['crate']
    return crate_of_file(pf, w)
