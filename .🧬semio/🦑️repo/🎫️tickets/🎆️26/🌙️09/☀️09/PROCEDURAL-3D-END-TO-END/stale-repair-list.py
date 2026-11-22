import json,glob,os,sys
S='/Volumes/semio-store/v1/origins/*/segments/*.jsonl'
ev=[]
for f in glob.glob(S):
    for l in open(f,encoding='utf8'):
        l=l.strip()
        if not l: continue
        try: e=json.loads(l)
        except: continue
        p=e.get('path','')
        if p.startswith('.cache') or p.startswith('.git/') or '/.git/' in p: continue
        ev.append(e)
ev.sort(key=lambda e:e['hlc'])
cur={}
out=[]
LO,HI='1795309900000','1795311000000'
for e in ev:
    p=e['path']
    c=cur.get(p)
    if e['operation']=='put':
        if e['hlc'].startswith('17953') and e['hlc'].endswith('-g1-m04') and LO<=e['hlc'][:13]<=HI:
            b=e.get('base')
            cs=c['sha256'] if c and c['operation']=='put' else None
            if b!=cs:
                out.append(dict(path=p,hlc=e['hlc'],base=b,new=e['sha256'],head=cs,headhlc=c['hlc'] if c else None,headop=c['operation'] if c else None))
    cur[p]=e
json.dump(dict(out=out,cur={e['path']:{'sha':cur[e['path']].get('sha256'),'op':cur[e['path']]['operation'],'hlc':cur[e['path']]['hlc']} for e in out}),open(sys.argv[1],'w'),ensure_ascii=False,indent=1)
print(len(out),len(ev))
