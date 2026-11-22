"""📐 Section table of lofts: python3 rework-06-sec.py <category/slug> <scale> <list> <id> [<id>...]"""
import json,sys
a,s,l=sys.argv[1:4]
v=json.load(open('volumes/%s.json'%a))['volumes'][s][l]
for x in v:
    if x.get('id') in sys.argv[4:]:
        print(x['id'],x['bone'],x['kind'],{k:x[k] for k in x if k not in('sections','id','bone','kind')})
        for q in x.get('sections',[]):
            print('  ',[round(c,4) for c in q['at']],round(q['rx'],4),round(q['ry'],4),q.get('exponent'),q.get('rotation'))
