import json,sys,os,hashlib,subprocess,tempfile
G=sys.argv[1]; ROOT='/Users/ueli/Documents/semio'; B='/Volumes/semio-store/v1/blobs'
d=json.load(open(G+'/stale.json'))['out']
d=[x for x in d if x['head'] is not None]
def blob(s):
    p=f'{B}/{s[:2]}/{s}'
    return open(p,'rb').read() if os.path.exists(p) else None
def sha(b): return hashlib.sha256(b).hexdigest()
res=[]
for x in d:
    p=x['path']; f=os.path.join(ROOT,p); r=dict(path=p,base=x['base'][:8],new=x['new'][:8],head=x['head'][:8])
    if not os.path.exists(f): r['action']='missing-current'; res.append(r); continue
    cur=open(f,'rb').read(); cs=sha(cur); r['cur']=cs[:8]
    if x['head']==x['new']: r['action']='already-recovered'; r['note']='fleet head == m04 new'; res.append(r); continue
    bb,nb,hb=blob(x['base']),blob(x['new']),blob(x['head'])
    if None in (bb,nb,hb): r['action']='conflict'; r['note']='blob missing '+str([k for k,v in zip('bnh',(bb,nb,hb)) if v is None]); res.append(r); continue
    td=tempfile.mkdtemp(dir=G)
    for n,b in (('cur',cur),('head',hb),('base',bb),('new',nb)): open(f'{td}/{n}','wb').write(b)
    if cs==x['head']: args=['head','base','new']; r['mode']='apply-m04-edit-on-head'
    else:
        args=['cur','base','head']; r['mode']='apply-fleet-delta-on-current'
        nl=set(nb.decode('utf8','replace').splitlines())-set(bb.decode('utf8','replace').splitlines()); cl=set(cur.decode('utf8','replace').splitlines())
        r['m04_edit_present']=nl<=cl
    pr=subprocess.run(['git','merge-file','-p']+[f'{td}/{a}' for a in args],capture_output=True)
    if pr.returncode==0:
        if pr.stdout==cur: r['action']='already-recovered'
        else: r['action']='merged'; open(f'{td}/out','wb').write(pr.stdout); r['merged_sha']=sha(pr.stdout); r['td']=td
    else: r['action']='conflict'; r['note']=f'git merge-file rc={pr.returncode}'
    res.append(r)
json.dump(res,open(G+'/plan.json','w'),ensure_ascii=False,indent=1)
import collections; print(collections.Counter(r['action'] for r in res))
for r in res:
    if r['action'] not in('merged',): print(r['action'],r.get('note'),r['path'][-60:])
