import json,collections,os,pickle
R='/Users/Shared/semio/'
C=R+'.cache/b/g/m/build-input-closures/'
tokfiles={}
def files(tok):
    k=tok[21:]
    if k in tokfiles:return tokfiles[k]
    p=C+k+'.json'
    tokfiles[k]=json.load(open(p))['files'] if os.path.exists(p) else None
    return tokfiles[k]
own={}
missing=0
cc=collections.Counter()
for line in open('model.jsonl'):
    d=json.loads(line)
    fs=[]
    for t in [x['t'] for x in d['groups'].get('nativeSources',[]) if isinstance(x,dict) and 't' in x]:
        f=files('x'*21+t)
        if f is None:missing+=1;continue
        fs+=f
    own[d['name']]=(d['root'],fs)
    for f in set(fs):cc[f]+=1
print('missing leaves',missing,'owners',len(own))
helpers={f for f,n in cc.items() if n>=1000}
print(len(helpers),sorted(helpers)[:5])
res={}
dist=collections.Counter()
big=[]
for n,(root,fs) in own.items():
    r=[f for f in fs if f not in helpers]
    res[n]=r
    dist[min(len(r)//5*5,60)]+=1
    if len(r)>=40:big.append((len(r),n))
print(sorted(dist.items()))
print(sorted(big,reverse=True)[:15])
pickle.dump((own,helpers,res),open('own.pkl','wb'))
# classify file types
ext=collections.Counter()
for n,r in res.items():
    for f in r:ext[os.path.splitext(f)[1]]+=1
print(ext.most_common(15))
