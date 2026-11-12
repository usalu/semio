import json,sys
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/%s/🔣️.json"
a=json.load(open(S%sys.argv[1]))['asset']
print(list(a.keys()))
print('DIM',a['dimensions'])
print('RIG',json.dumps(a['rig'],ensure_ascii=False))
print('MAT',[ (m['id']) for m in a['materials']])
print('REG',[ (r['id']) for r in a['thermal']['regions']])
print('POSES',json.dumps(a.get('poses'),ensure_ascii=False)[:1500])
print('CLIPS',json.dumps(a.get('clips'),ensure_ascii=False)[:1200])
print('REPS',list(a['representations'].keys()))
g=a['representations']['50']['geometry']
def short(s):
    d={k:v for k,v in s.items() if k not in('title','label','text','description')}
    return json.dumps(d,ensure_ascii=False,separators=(',',':'))
for v in ('elevation','front','plan'):
    print('==',v)
    for s in g[v]: print(short(s))
for d in a['representations']:
    if d=='50':continue
    gg=a['representations'][d].get('geometry')
    if gg: print(d,{v:[s.get('id') for s in gg[v]] for v in gg})
