import json,re,sys
a=json.load(open(sys.argv[1])); b=json.load(open(sys.argv[2]))
def norm(x): return re.sub(r'SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_[0-9a-f]{64}','TOK',json.dumps(x,sort_keys=True))
def diff(x,y,path=''):
    if type(x)!=type(y): return path+' type',str(x)[:100],str(y)[:100]
    if isinstance(x,dict):
        for kk in sorted(set(x)|set(y)):
            if kk not in x or kk not in y: return path+'/'+kk+' missing', kk in x, kk in y
            r=diff(x[kk],y[kk],path+'/'+kk)
            if r: return r
    elif isinstance(x,list):
        if len(x)!=len(y): return path+' len',len(x),len(y),[str(i)[:60] for i in x[:3]],[str(i)[:60] for i in y[:3]]
        for i,(p,q) in enumerate(zip(x,y)):
            r=diff(p,q,path+'[%d]'%i)
            if r: return r
    elif x!=y: return path,str(x)[:150],str(y)[:150]
n=0
for k in sorted(a):
    if k not in b: print('missing',k); n+=1; continue
    if norm(a[k])!=norm(b[k]):
        n+=1
        if n<=4: print(k,diff(json.loads(norm(a[k])),json.loads(norm(b[k]))))
print('projects',len(a),len(b),'differing (loose)',n)
strict=sum(1 for k in a if k in b and json.dumps(a[k],sort_keys=True)!=json.dumps(b[k],sort_keys=True))
print('differing strict (incl tokens)',strict)
