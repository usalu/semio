import json,pickle,re,hashlib,sys
pairs=pickle.load(open('pairs.pkl','rb'))
def sj(v):
    if isinstance(v,list):return [sj(x) for x in v]
    if isinstance(v,dict):return {k:sj(v[k]) for k in sorted(v) if v[k] is not None}
    return v
def key(r): return hashlib.sha256(json.dumps(sj(r),separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
def walk(v,f):
    if isinstance(v,str):return f(v)
    if isinstance(v,list):return [walk(x,f) for x in v]
    if isinstance(v,dict):return {k:walk(x,f) for k,x in v.items()}
    return v
def win(s):
    return s.replace('⟦semio-root-json⟧','C:\\semio').replace('⟦semio-root-slash⟧','C:/semio').replace('⟦semio-root⟧','C:\\semio')
def pos(s,root,gen):
    return s.replace('⟦semio-root⟧',root).replace('⟦generation⟧',gen)
def unneutral(rec,plat,root,gen):
    rec=json.loads(json.dumps(rec))
    f=win if plat=='win32-x64' else (lambda s:pos(s,root,gen))
    rec['observations']=walk(rec['observations'],f);rec['authority']=walk(rec['authority'],f)
    rec['image']=f(rec['image'])
    return rec
def ok(rec):
    img=json.loads(rec['image']);t=img.get('closureRecipes') or {}
    return all(key(v)==k for k,v in t.items())
VOL={'mtimeMs','ctimeMs','ino'}
best=[]
for w,p in pairs:
    if p['platform']!='posix-shared':continue
    root='/Users/Shared/semio'
    gens=set(re.findall(r'\.cache/b/d/m/j/([0-9a-f]{16,64})/',json.dumps(p['rec']['observations'])))
    wr=unneutral(w['rec'],'win32-x64','C:\\semio','');
    gen='86415d695aa883fffc22'
    pr=unneutral(p['rec'],p['platform'],root,gen)
    # windows generation: from observations
    wg=set(re.findall(r'\.cache\\\\?b\\\\?d\\\\?m\\\\?j\\\\?([0-9a-f]{16,64})',json.dumps(w['rec']['observations'])))
    if not ok(wr) or not ok(pr):continue
    kw=set((json.loads(wr['image']).get('closureRecipes') or {}));kp=set((json.loads(pr['image']).get('closureRecipes') or {}))
    if 'rootkeys' in sys.argv[2:] and (kw==kp or not kw):continue
    best.append((len(json.dumps(wr))+len(json.dumps(pr)),w['name'],wr,pr,wg))
best.sort(key=lambda x:x[0])
print(len(best))
for b in best[:8]:print(b[0],b[1],b[4])
want=[b[1] for b in best[:2]]+["@semio-tech/flow-plugin"]
print([ (b[0],b[1]) for b in best[:4]])
cases=[]
for size,name,wr,pr,wg in best:
    if name not in want:continue
    wgen=sorted(wg)[0]
    # restore the generation id in the Windows rows (the stored record carries it) and the POSIX id
    cases.append({"name":name,"windows":{"root":"C:\\semio","record":wr},"posix":{"root":"/Users/Shared/semio","record":pr}})
import os
out="ov/"+sys.argv[1]+"/🧪️tests/🌍️real/🔣️.json"
os.makedirs(os.path.dirname(out),exist_ok=True)
json.dump({"cases":cases},open(out,"w",encoding="utf8"),ensure_ascii=False,indent=1);open(out,"a").write("\n")
print("real cases",len(cases),os.path.getsize(out))
