import json,collections,re,pickle
R=json.load(open('records.json'))
W=[r for r in R if r['platform']=='win32-x64']
c=collections.Counter();ex={}
for r in W:
    s=r['rec']['image']
    for pat,name in ((r'[A-Za-z]:[\\/]','drive in image'),(r'\\\\','backslash in image'),(r'\.exe\b','exe in image'),(r'\.cmd\b','cmd in image'),(r'\r\n|\\r\\n','crlf in image')):
        m=re.search(pat,s)
        if m: c[name]+=1;ex.setdefault(name,(r['name'],s[max(0,m.start()-80):m.end()+80]))
    for o in r['rec']['observations']:
        k=o['kind']; p=o['path']; f=o.get('fact')
        fs=json.dumps(f,ensure_ascii=False)
        if re.search(r'[A-Za-z]:\\',p): 
            key=('rowpath-drive',k);c[key]+=1;ex.setdefault(key,(r['name'],p[:200]))
        if re.search(r'[A-Za-z]:\\',fs) or re.search(r'[A-Za-z]:/',fs):
            key=('rowfact-drive',k);c[key]+=1;ex.setdefault(key,(r['name'],fs[:200]))
        if '\\' in p and not p.startswith('⟦semio-root'):
            key=('rowpath-backslash-nonroot',k);c[key]+=1;ex.setdefault(key,(r['name'],p[:200]))
        if k in('closure','resolve'):
            key=('has-C-in-'+k,'C:\\\\' in p);c[key]+=1
for k,v in sorted(c.items(),key=str):print(k,v)
for k,v in ex.items():print(k,str(v)[:400])
