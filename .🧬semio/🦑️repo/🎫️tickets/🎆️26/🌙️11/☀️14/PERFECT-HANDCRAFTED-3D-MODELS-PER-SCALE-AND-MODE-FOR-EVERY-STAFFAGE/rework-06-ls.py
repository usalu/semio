"""🔎 Compact listing of one volume list: python3 rework-06-ls.py <category/slug> <scale> <body|thermal|structure> [bone-substring]"""
import json,sys
a,s,l=sys.argv[1:4]; f=sys.argv[4] if len(sys.argv)>4 else ''
v=json.load(open('volumes/%s.json'%a))['volumes'][s][l]
def r(x):
    return [round(q,3) for q in x] if isinstance(x,list) else x
for x in v:
    if 'of' in x and 'kind' not in x:
        (not f) and print("OF",x["of"],{k:x[k] for k in x if k!="of"}); continue
    if f and f not in x['bone']: continue
    k=x['kind']
    if k=='loft': e='n%d %s>%s rx %s ry %s'%(len(x['sections']),r(x['sections'][0]['at']),r(x['sections'][-1]['at']),r([q['rx'] for q in x['sections']]),r([q['ry'] for q in x['sections']]))
    elif k=='capsule': e='%s>%s r %s %s'%(r(x['p0']),r(x['p1']),x['r0'],x['r1'])
    elif k=='box': e=str(x['size'])
    elif k=='ellipsoid': e=str(x['radii'])
    else: e='n%d %s %s..%s'%(len(x['profile']),x.get('plane',x.get('axis')),x.get('from'),x.get('to'))
    t=x.get('transform'); 
    m={kk:x[kk] for kk in ('region','layer','member','part','weights','repeat','slender','texture') if kk in x}
    print(x['id'],'|',x['bone'],k,x['role'],x['material'],e[:260],('T'+json.dumps(t)) if t else '',m if m else '')
