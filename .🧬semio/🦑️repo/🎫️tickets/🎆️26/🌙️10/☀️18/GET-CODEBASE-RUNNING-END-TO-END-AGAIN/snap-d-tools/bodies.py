import json,re,sys,os
d=json.load(open(sys.argv[1]))
def body(src,start):
    i=src.index('{',src.index(')',start) if False else start)
    # find the fn signature end: first '{' after '->' return type
    depth=0;j=i
    while j<len(src):
        c=src[j]
        if c=='{':depth+=1
        elif c=='}':
            depth-=1
            if depth==0:return src[start:j+1]
        j+=1
    return src[start:]
rows=[]
for f,v in d.items():
    if v['dec'] and v['enc']:continue
    src=open(f,encoding='utf-8').read()
    m=re.search(r'fn validate_sqlite_snapshot_subset\(',src)
    if not m: rows.append((v['pkg'],f,'-',''));continue
    b=body(src,m.start())
    one=' '.join(b.split())
    calls=sorted(set(re.findall(r'\b([a-z_][a-z0-9_:]*)\(',one)))
    alloc=[c for c in calls if re.search(r'alloc|admit|retire|stage|workspace|reserve|receiv',c)]
    rows.append((v['pkg'],f,'S',one[:300],alloc))
json.dump(rows,open(sys.argv[2],'w'),ensure_ascii=False,indent=0)
for r in rows:
    if r[2]=='S':print(r[0],len(r[3]),r[4])
