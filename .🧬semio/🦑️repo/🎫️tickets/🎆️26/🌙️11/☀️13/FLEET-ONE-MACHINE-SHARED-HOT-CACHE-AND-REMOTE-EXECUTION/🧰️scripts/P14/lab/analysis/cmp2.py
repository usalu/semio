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
        if len(a)!=len(b): out.append((path,'len',len(a),len(b)));
        for i,(x,y) in enumerate(zip(a,b)): walk(x,y,path+'[]',out)
    elif a!=b: out.append((path,'val',repr(a)[:120],repr(b)[:120]))
cats=collections.Counter(); ex={}
nodiff=0
for root,pair,hd,ia,ib,name in d:
    A=json.loads(ia);B=json.loads(ib);o=[];walk(A,B,'',o)
    key=tuple(sorted({re.sub(r'\[\]','',p) for p,*_ in o}))
    if not o: nodiff+=1
    for p,k,x,y in o:
        c=(re.sub(r'/targets/[^/]+','/targets/*',re.sub(r'/projects/[^/]+','/projects/*',p)),k); cats[c]+=1; ex.setdefault(c,(root,pair,x,y))
print('structurally equal',nodiff,'of',len(d))
for c,n in cats.most_common(40): print(n,c,ex[c])
