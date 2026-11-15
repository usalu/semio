import json,pickle,collections,re
exec(open('a2.py').read().split("cnt=collections")[0])
cnt=collections.Counter(); ex={}
tok=collections.Counter()
for w,p in pairs:
    s=w['rec']['image']
    for m in re.findall(r'⟦semio-root[-a-z]*⟧[\\/]{1,2}',s): tok[m]+=1
    s=s.replace('⟦semio-root-slash⟧','⟦semio-root⟧')
    iw=json.loads(s); ip=json.loads(p['rec']['image'])
    out=[]; diff(iw,ip,'',out)
    for path,a,b in out:
        if 'closureRef' in path: continue
        cnt[path]+=1; ex.setdefault(path,(a,b,w['name']))
print(tok)
for k,v in cnt.most_common(30):
    a,b,n=ex[k]; print(v,k,n,'\n   W:',str(a)[:700],'\n   P:',str(b)[:700])
