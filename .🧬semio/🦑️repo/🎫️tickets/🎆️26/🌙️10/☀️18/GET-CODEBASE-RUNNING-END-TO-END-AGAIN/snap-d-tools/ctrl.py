import json,re,sys
d=json.load(open(sys.argv[1]))
def body(src,start):
    i=src.index('{',start); depth=0
    for j in range(i,len(src)):
        c=src[j]
        if c=='{':depth+=1
        elif c=='}':
            depth-=1
            if depth==0:return src[start:j+1]
    return src[start:]
odd=[]
for f,v in d.items():
    src=open(f,encoding='utf-8').read()
    m=re.search(r'fn validate_sqlite_snapshot_subset\(',src)
    if not m or (v['dec'] and v['enc']):continue
    b=body(src,m.start())
    sig=b[:b.index('{')]
    pm=re.search(r'(\w+)\s*:\s*&mut\s+(?:\w+::)*SqliteSnapshotControl',sig)
    if not pm:
        odd.append((v['pkg'],'?',[sig[:200]]));continue
    name=pm.group(1)
    rest=b[len(sig):]
    uses=[x for x in re.finditer(r'\b'+name+r'\b(?!\.checkpoint\()',rest)]
    if uses:
        odd.append((v['pkg'],name,[rest[max(0,u.start()-60):u.end()+60].replace('\n',' ') for u in uses][:3]))
print(len(odd))
for o in odd:print(o)
