import sys; sys.path.insert(0, '.')
from model6 import *
from fields6 import parse_fields, comp_of
COMP_ORDER = ['core', 'constructor', 'stores', 'roster', 'ingress', 'interaction', 'history', 'hostevent', 'txn', 'children', 'pending', 'tools', 'media', 'ingest', 'toolmachine', 'timetravel', 'toolrun', 'maintenance', 'close']

def load_model(files=INSTANCE_FILES):
    return {rel: parse_file(rel) for rel in files}

def method_records(model):
    recs = []
    for rel, f in model.items():
        R = f['R']; L = f['L']
        for pi, p in enumerate(f['pieces']):
            if p.kind != 'vcsimpl': continue
            for mi, m in enumerate(p.members):
                rec = dict(file=rel, piece=pi, idx=mi, name=m.name, vis=m.vis, trait=p.trait, a=m.a, e=m.e, start=m.start,
                           text='\n'.join(R[m.a:m.e]), body='\n'.join(L[m.start:m.e]), sig=m.sig)
                recs.append(rec)
    return recs

def analyse(model=None, tt_whole=False):
    model = model or load_model()
    fs = parse_fields(); fcomp = {f['name']: comp_of(f['name']) for f in fs}; fn = set(fcomp)
    recs = method_records(model)
    by = collections.defaultdict(list)
    for r in recs: by[r['name']].append(r)
    uniq = {n: v[0] for n, v in by.items() if len(v) == 1}
    rk = {c: i for i, c in enumerate(COMP_ORDER)}; USE = len(COMP_ORDER)
    for m in recs:
        m['lines'] = m['e'] - m['a']
        m['fields'] = sorted(set(re.findall(r'\bself\s*\.\s*(\w+)', m['body'])) & fn)
        m['calls'] = sorted(set(re.findall(r'\b(?:self|Self|VcsArtifactApp)\s*(?:\.|::)\s*(\w+)\s*(?:\(|::<)', m['body'])) | set(re.findall(r'\b(?:Self|VcsArtifactApp)\s*::\s*(\w+)\b(?!\s*(?:\(|::))', m['body'])))
        m['dcomps'] = sorted({fcomp[f] for f in m['fields']}, key=lambda c: rk[c])
        cs = set(m['dcomps']); base = max([rk[c] for c in cs], default=-1)
        if len(cs - {'core'}) >= 4: base = USE
        if m['trait'] == 'PluginApp': base = USE + 1
        m['rank'] = base
    ch = True
    while ch:
        ch = False
        for m in recs:
            for cn in m['calls']:
                t = uniq.get(cn)
                if t and t is not m and t['rank'] > m['rank']: m['rank'] = t['rank']; ch = True
    for m in recs:
        r = m['rank']; m['home'] = COMP_ORDER[r] if 0 <= r < USE else ('free' if r < 0 else 'usecase' if r == USE else 'pluginapp')
    return recs, fcomp, uniq, model

if __name__ == '__main__':
    recs, fcomp, uniq, model = analyse()
    tot = collections.Counter()
    for m in recs: tot[m['home']] += m['lines']
    print(len(recs), dict(tot.most_common()))
