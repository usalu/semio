import pickle,json,collections,re,datetime
rows=pickle.load(open('p14/rows.pkl','rb'))
by=collections.defaultdict(lambda:collections.defaultdict(list))
for r in rows: by[r['root']][r['plat']].append(r)
def walk(a,b,path,out):
    if type(a)!=type(b): out.append((path,repr(a)[:150],repr(b)[:150]));return
    if isinstance(a,dict):
        for k in sorted(set(a)|set(b)):
            if k not in a: out.append((path+'/'+k,'<absent>',repr(b[k])[:150]))
            elif k not in b: out.append((path+'/'+k,repr(a[k])[:150],'<absent>'))
            else: walk(a[k],b[k],path+'/'+k,out)
    elif isinstance(a,list):
        if len(a)!=len(b): out.append((path,'len%d'%len(a),'len%d'%len(b)))
        for x,y in zip(a,b): walk(x,y,path+'[]',out)
    elif a!=b: out.append((path,repr(a)[:150],repr(b)[:150]))
cats=collections.Counter();shown=0;gaps=[]
for root,v in by.items():
    if 'darwin-arm64' in v and 'linux-x64' in v:
        a=max(v['darwin-arm64'],key=lambda r:r['t']);b=max(v['linux-x64'],key=lambda r:r['t'])
        gaps.append(abs(a['t']-b['t'])/3.6e6)
        A=json.loads(a['img']);B=json.loads(b['img'])
        # compare recipes by sorted values ignoring their hash keys
        ra=sorted(json.dumps(x,sort_keys=True) for x in (A.get('closureRecipes') or {}).values())
        rb=sorted(json.dumps(x,sort_keys=True) for x in (B.get('closureRecipes') or {}).values())
        if ra!=rb and len(ra)==len(rb):
            for x,y in zip(ra,rb):
                if x!=y:
                    o=[];walk(json.loads(x),json.loads(y),'',o)
                    for p,u,w in o: cats[re.sub(r'\[\]','',p)[:90]]+=1
                    if shown<4 and o: shown+=1;print(root[-60:],o[:4])
print(cats.most_common(15))
import statistics
print('producer time gap hours: median',statistics.median(gaps),'min',min(gaps),'max',max(gaps))
