import pickle,json,collections,hashlib,re
rows=pickle.load(open('p14/rows.pkl','rb'))
def normimg(s):
    A=json.loads(s);rec=A.get('closureRecipes') or {}
    m={k:'R'+hashlib.sha256(json.dumps(v,sort_keys=True).encode()).hexdigest()[:12] for k,v in rec.items()}
    def sub(s):
        for k,v in m.items(): s=s.replace(k,v)
        return re.sub(r'sharedSources[0-9a-f]{16}','sharedSourcesH',s)
    def mp(x):
        if isinstance(x,dict): return {sub(k):mp(v) for k,v in x.items()}
        if isinstance(x,list): return [mp(v) for v in x]
        if isinstance(x,str): return sub(x)
        return x
    return json.dumps(mp(A),sort_keys=True,ensure_ascii=False)
by=collections.defaultdict(lambda:collections.defaultdict(list))
for r in rows: r['n']=normimg(r['img']); by[r['root']][r['plat']].append(r)
c=collections.Counter();bad=[]
for root,v in by.items():
    ps=sorted(v)
    for i in range(len(ps)):
        for j in range(i+1,len(ps)):
            a=max(v[ps[i]],key=lambda r:r['t']);b=max(v[ps[j]],key=lambda r:r['t'])
            eq=a['n']==b['n'];c[(ps[i],ps[j],eq)]+=1
            if not eq and ps[:2]==['darwin-arm64','linux-x64'] and (ps[i],ps[j])==('darwin-arm64','linux-x64'): bad.append((root,a,b))
for k,n in sorted(c.items(),key=str):print(k,n)
print(len(bad))
def walk(a,b,path,out):
    if type(a)!=type(b): out.append((path,repr(a)[:120],repr(b)[:120]));return
    if isinstance(a,dict):
        for k in sorted(set(a)|set(b)):
            if k not in a: out.append((path+'/'+k,'<absent>',repr(b[k])[:120]))
            elif k not in b: out.append((path+'/'+k,repr(a[k])[:120],'<absent>'))
            else: walk(a[k],b[k],path+'/'+k,out)
    elif isinstance(a,list):
        if len(a)!=len(b): out.append((path,'len%d'%len(a),'len%d'%len(b)))
        for x,y in zip(a,b): walk(x,y,path+'[]',out)
    elif a!=b: out.append((path,repr(a)[:120],repr(b)[:120]))
cc=collections.Counter()
for root,a,b in bad:
    o=[];walk(json.loads(a['n']),json.loads(b['n']),'',o)
    for p,x,y in o: cc[re.sub(r'/targets/[^/]+','/T',p)[:100]]+=1
    if bad.index((root,a,b))<5: print(root[-50:],o[:3])
print(cc.most_common(10))
