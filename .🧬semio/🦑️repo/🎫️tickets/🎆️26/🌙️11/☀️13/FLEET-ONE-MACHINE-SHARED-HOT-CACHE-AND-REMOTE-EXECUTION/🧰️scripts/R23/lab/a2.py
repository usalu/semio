import json,pickle,collections,re
pairs=pickle.load(open('pairs.pkl','rb'))
def diff(a,b,path,out):
    if type(a)!=type(b): out.append((path,a,b));return
    if isinstance(a,dict):
        for k in sorted(set(a)|set(b)):
            if k not in a: out.append((path+'/'+k,None,'<added>'))
            elif k not in b: out.append((path+'/'+k,'<removed>',None))
            else: diff(a[k],b[k],path+'/'+k,out)
    elif isinstance(a,list):
        if len(a)!=len(b): out.append((path+'[len]',len(a),len(b)))
        for i,(x,y) in enumerate(zip(a,b)): diff(x,y,path+'[]',out)
    elif a!=b: out.append((path,a,b))
cnt=collections.Counter(); ex={}
same=0
for w,p in pairs:
    iw=json.loads(w['rec']['image']); ip=json.loads(p['rec']['image'])
    out=[]; diff(iw,ip,'',out)
    if not out: same+=1
    for path,a,b in out:
        cnt[path]+=1; ex.setdefault(path,(a,b,w['name']))
print('identical images',same,'of',len(pairs))
for k,v in cnt.most_common(40):
    a,b,n=ex[k]; print(v,k,'\n   W:',str(a)[:230],'\n   P:',str(b)[:230])
