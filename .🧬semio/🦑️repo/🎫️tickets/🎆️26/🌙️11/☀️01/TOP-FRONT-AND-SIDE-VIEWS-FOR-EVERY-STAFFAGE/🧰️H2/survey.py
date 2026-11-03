import json,sys,os
os.chdir(r'C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage')
import glob
for a in sys.argv[1:]:
    p=glob.glob(f'🗂️catalog/*/{a}/*.json')[0]
    d=json.load(open(p,encoding='utf-8'))['asset']
    print(a,json.dumps(d['dimensions']),json.dumps(d['presentation']),'bones',len(d['rig']['bones']),'poses',[p['id'] for p in d['poses']],'clips',[c['id'] for c in d['clips']])
    for k,r in d['representations'].items():
        g=r.get('geometry') or {}
        print('  ',k,r['status'],{v:len(s) for v,s in g.items()})
