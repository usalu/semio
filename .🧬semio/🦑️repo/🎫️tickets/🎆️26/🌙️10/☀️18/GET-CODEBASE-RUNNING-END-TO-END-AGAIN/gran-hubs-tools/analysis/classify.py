"""🏷️ Per-hub dependent classification from census-<hub>.json: share of dependents that are pure facade users (every prod path resolves to a producer crate, none to the hub's own definitions), mixed, own-only, unresolved. `python3 classify.py`."""
from hubenv import *
def classify(h):
    d=json.load(open(OUT+'/census-%s.json'%h));R_=d['resolved'];ID=d['ident']
    out={}
    for dd,v in d['deps'].items():
        if not v['prod']:
            out[dd]='test' if v['test'] else 'macro' if v['macros'] else 'none';continue
        own=oth=unk=False
        ps=[p for p in v['prod'] if p!=''] or ['']
        for p in ps:
            r=R_[p]
            if r['kind']=='unresolved' or (r['kind']=='crate' and r['owner']==ID) or p in('','*','self'): unk=True
            elif r['owner']==ID and r['kind'] in('def','mod'): own=True
            else: oth=True
        out[dd]=('own' if own else '')+('|other' if oth else '')+('|unk' if unk else '')
    return out
if __name__=='__main__':
    for h in ['os-kernel','umbrella','plugin','job']:
        c=collections.Counter(classify(h).values());print(h,dict(c))
