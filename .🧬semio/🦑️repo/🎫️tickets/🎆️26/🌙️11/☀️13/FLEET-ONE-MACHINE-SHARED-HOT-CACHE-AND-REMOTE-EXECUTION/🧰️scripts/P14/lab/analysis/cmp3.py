import pickle,json,collections,re
d=pickle.load(open('p14/diffs.pkl','rb'))
def walk(a,b,path,out):
    if type(a)!=type(b): out.append((path,'type',repr(a)[:100],repr(b)[:100]));return
    if isinstance(a,dict):
        for k in sorted(set(a)|set(b)):
            if k not in a: out.append((path+'/'+k,'onlyB',None,repr(b[k])[:100]))
            elif k not in b: out.append((path+'/'+k,'onlyA',repr(a[k])[:100],None))
            else: walk(a[k],b[k],path+'/'+k,out)
    elif isinstance(a,list):
        if len(a)!=len(b): out.append((path,'len',len(a),len(b)))
        for x,y in zip(a,b): walk(x,y,path+'[]',out)
    elif a!=b: out.append((path,'val',repr(a)[:200],repr(b)[:200]))
pairs=[x for x in d if x[1]==('darwin-arm64','linux-x64')]
print('darwin/linux differing',len(pairs))
cats=collections.Counter();ex={};eq_after=0;proj=collections.defaultdict(set)
for root,pair,hd,ia,ib,name in pairs:
    o=[];walk(json.loads(ia),json.loads(ib),'',o)
    o2=[x for x in o if not re.search(r'closureRef$',x[0])]
    if not o2: eq_after+=1
    for p,k,x,y in o2:
        c=(re.sub(r'commandSources\d+','commandSourcesN',re.sub(r'/targets/[^/]+','/targets/*',re.sub(r'/projects/[^/]+','/projects/*',p))),k);cats[c]+=1;ex.setdefault(c,(root,x,y));proj[c].add(root)
print('equal once closureRef ignored',eq_after,'of',len(pairs))
for c,n in cats.most_common(30): print(n,len(proj[c]),c,ex[c])
