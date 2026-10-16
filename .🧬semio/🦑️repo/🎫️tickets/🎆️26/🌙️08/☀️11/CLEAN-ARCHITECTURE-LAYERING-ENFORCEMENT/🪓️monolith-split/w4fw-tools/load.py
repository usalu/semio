import os, sys, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import SourceFile, TOK, MARK
from lex import mask
from cfg import *
from assign import *
from imports import flatten, file_imports
def load(R):
    files = {k: SourceFile(k, os.path.join(R, p)) for k, p in SRC.items()}
    return files
def entities(files):
    """returns dict key=(file,name) -> dict(units=[...], cluster) and unit->entity map; test units separate"""
    ents = collections.OrderedDict()
    last = None
    for k, f in files.items():
        last = None
        for u in f.units:
            if (u.test and u.kind in ('mod', 'use')) or u.kind in ('use', 'comment'): continue
            if u.kind in ('fn','struct','enum','type','trait','static','mod','union','const','macro_rules!') and u.name:
                if u.name == '_' and last: ents[last]['units'].append(u); continue
                key = (k, u.name)
                e = ents.setdefault(key, dict(file=k, name=u.name, kind=u.kind, vis=u.vis, units=[]))
                e['units'].append(u); last = key
    byname = collections.defaultdict(list)
    for key in ents: byname[key[1]].append(key)
    extra = []
    for k, f in files.items():
        for u in f.units:
            if u.kind not in ('impl', 'macro-call'): continue
            cands = [x for x in byname.get(u.target, []) if x[0] == k] or byname.get(u.target, [])
            if not cands:
                head = re.sub(r'\s+', ' ', u.text.split('{')[0])
                htoks = [t for t in TOK.findall(mask(head)) if t not in ('impl', 'for', 'where', 'pub', 'Fn', 'Option', 'Vec', 'String', 'Result', 'Self')]
                for t in htoks:
                    c2 = [x for x in byname.get(t, []) if x[0] == k]
                    if c2: cands = c2; break
            if cands:
                ents[cands[0]]['units'].append(u)
            else:
                key = (k, f"<{u.kind}:{u.target}@{u.start}>")
                ents[key] = dict(file=k, name=key[1], kind=u.kind, vis='', units=[u])
    for key, e in ents.items():
        e['units'].sort(key=lambda u: (u.file, u.start))
        e['cluster'] = assign_entity(e['file'], e['name'] if not e['name'].startswith('<') else e['units'][0].target or '', e['kind'])
        for u in e['units']:
            for rl, rn in u.regions:
                if rn in REGION_CLUSTER and e['kind'] != 'mod': e['cluster'] = REGION_CLUSTER[rn]
        e['lines'] = sum(u.lines for u in e['units'])
        e['tokens'] = set().union(*[u.tokens for u in e['units']])
    return ents
if __name__ == '__main__':
    R = sys.argv[1] if len(sys.argv) > 1 else REPO
    files = load(R); ents = entities(files)
    c = collections.defaultdict(lambda: [0, 0])
    for key, e in ents.items():
        c[e['cluster']][0] += 1; c[e['cluster']][1] += e['lines']
        if e['cluster'] in ('UNASSIGNED', None) and e['kind'] != 'mod': print('UNASSIGNED', key)
    for k, v in sorted(c.items(), key=lambda x: str(x[0])): print(k, v)
