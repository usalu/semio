import json,re,os,subprocess,collections,sys
T=sys.argv[1]; NORM=sys.argv[2]
items=json.load(open(f"{T}/🗑️generated/audit-requests/items.json"))
rel=[]
for dp,dn,fn in os.walk(NORM):
    if '/node_modules' in dp or '/target' in dp: continue
    for f in fn: rel.append(os.path.relpath(os.path.join(dp,f),NORM))
relset=set(rel)
bytail=collections.defaultdict(list)
for r in rel:
    parts=r.split('/')
    for k in range(1,min(4,len(parts))+1): bytail['/'.join(parts[-k:])].append(r)
PATHEXT=re.compile(r'\.(rs|ts|py|json|mjs|feature|proto|graphql|md|sql|toml|semio|js|tsx|sh)$')
toks={}
for it in items:
    for t in re.findall(r'`([^`]+)`',it['text']):
        t=t.strip()
        if len(t)<4: continue
        toks.setdefault(t,[]).append(it['n'])
res={}
ident=[]
for t,ns in toks.items():
    base=t.split('…/')[-1].strip()
    if '/' in base or PATHEXT.search(base):
        b=base.rstrip('/').replace('{','').replace('}','')
        if ' ' in b or '*' in b: res[t]=('pathish-skip',None); continue
        key=b.lstrip('./')
        hit=None
        if key in relset: hit=key
        else:
            cands=bytail.get(key.split('/')[-1] if False else key,[])
            if not cands:
                # try suffix match of last 2 segments
                segs=key.split('/')
                for k in (3,2,1):
                    if len(segs)>=k and '/'.join(segs[-k:]) in bytail: cands=bytail['/'.join(segs[-k:])]; break
            hit=cands[0] if cands else None
        res[t]=('path', hit if hit else 'MISSING')
    else:
        if re.search(r'::|_|[a-z][A-Z]',t) and len(t)>=6 and ' ' not in t and '*' not in t:
            ident.append(t)
            res[t]=('ident',None)
# identifier counts via rg -o
pat=f"{T}/../../../../../../../../tmp/x" 
open(f"{T}/🗑️generated/audit-requests/ident-patterns.txt","w").write('\n'.join(sorted(set(ident)))+'\n')
out=subprocess.run(['rg','-o','-N','--no-filename','-F','-f',f"{T}/🗑️generated/audit-requests/ident-patterns.txt",'-g','*.rs','-g','*.ts','-g','*.py','-g','*.json','-g','*.feature','-g','*.mjs','-g','*.proto','-g','*.graphql',NORM],capture_output=True,text=True).stdout
cnt=collections.Counter(out.split('\n'))
for t in ident:
    res[t]=('ident',cnt.get(t,0))
json.dump({'toks':toks,'res':res},open(f"{T}/🗑️generated/audit-requests/tokens.json",'w'),ensure_ascii=False)
print(len(toks),len(ident))
