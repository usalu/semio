import json,sys
slug=sys.argv[1]
d=json.load(open(f"🗑️generated/drafts/humans/{slug}.json"))
print(d.keys(), d['report'].keys())
fl={}
for f in d['report']['flags']:
    fl.setdefault(f.get('solid') or f.get('id'),[]).append(f['severity'][:3]+':'+f['code'])
def r(x):
    if isinstance(x,float): return round(x,3)
    if isinstance(x,list): return [r(i) for i in x]
    if isinstance(x,dict): return {k:r(v) for k,v in x.items()}
    return x
for s in d['volume']['solids']:
    g={k:r(v) for k,v in s.items() if k not in('id','bone','kind','material','role','tone','region','regions')}
    t=json.dumps(g,separators=(',',':'))
    print(s['id'],s['bone'],s['kind'],s['material'],s['role'],s['tone'],s.get('region') or s.get('regions'),t[:230],fl.get(s['id'],''))
