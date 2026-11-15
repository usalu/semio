import json,sys
d=json.load(open(f'🗑️generated/author10/{sys.argv[1]}.json'))
print('dims',d['dims'],d['presentation'])
for b in d['bones']:
    f=d['frames'][b['id']]
    print(f"{b['id']:14} par={str(b['parent']):10} len={b['lengthM']:.3f} sec={b.get('section')} m={b.get("mass")} o={[round(x,3) for x in f['o']]} u={[round(x,2) for x in f['u']]} v={[round(x,2) for x in f['v']]} w={[round(x,2) for x in f['w']]}")
for r in d['thermal']['regions']:
    print('R',r['id'],r['kind'],r['areaM2'],r.get('hidden'),[ (l['id'],l.get('thicknessM'),l.get('material')) for l in r['layers']], (r.get('body') or {}).get('segment'))
print('E',json.dumps(d['thermal']['emitters']))
print('S',json.dumps(d['supports']))
for s,v in d['scales'].items():
    if v['status']!='authored': print(s,v); continue
    print(s,'memberBones',v['memberBones'],'members',v['members'],'shapeBones',v['shapeBones'])
