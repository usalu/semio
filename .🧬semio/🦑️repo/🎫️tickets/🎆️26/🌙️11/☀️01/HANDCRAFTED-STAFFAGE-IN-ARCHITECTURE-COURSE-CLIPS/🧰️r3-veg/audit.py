import json,glob,os,sys
root=sys.argv[1]
rows=[]
for f in sorted(glob.glob(root+'/🗂️catalog/🌳️vegetation/*/🔣️.json')):
    a=json.load(open(f,encoding='utf8'))['asset']
    r=a['representations']
    sc={k+('' if v['status']=='authored' else '!'):tuple([len(v.get('geometry',{}).get(w,[])) for w in ('elevation','front','plan')]) for k,v in r.items()}
    leaf=[x for x in a['thermal']['regions'] if x['model']=='leaf']
    lai=[x['leaf']['leafAreaIndex'] for x in leaf]
    gs=[x['leaf']['stomatalConductanceMaxMS'] for x in leaf]
    d=a['dimensions']
    em=[e['kind'] for e in a['thermal']['emitters']]
    print(a['id'],os.path.getsize(f)//1024,'KB',d,'sc',sc,'LAI',lai,'gs',gs,'em',em,'poses',len(a['poses']),'clips',[c['id'] for c in a['clips']],'sym',len(a['symbols']))
