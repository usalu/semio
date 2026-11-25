import json,os,re,sys,collections
G=2**30
ledger,index=sys.argv[1],sys.argv[2]
idx=json.load(open(index,encoding='utf-8'))
def lib_dir(manifest):
    t=open(manifest,encoding='utf-8').read()
    m=re.search(r'^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"',t,re.S|re.M)
    base=os.path.dirname(manifest)
    return os.path.dirname(os.path.normpath(os.path.join(base,m.group(1)))) if m else os.path.join(base,'src')
def lines(d):
    n=0
    for dp,dn,fn in os.walk(d):
        dn[:]=[x for x in dn if x not in('target','node_modules','dist','.git')]
        for f in fn:
            if f.endswith('.rs'):
                with open(os.path.join(dp,f),'rb') as h: n+=h.read().count(b'\n')
    return n
rows=[json.loads(l) for l in open(ledger,encoding='utf-8')]
by=collections.defaultdict(list)
for r in rows: by[r['key']].append(r)
out=[]
for k,v in by.items():
    v.sort(key=lambda r:r['atMs'])
    pkg=k.split('@')[0]; parts=k.split('|')
    cur=None
    if pkg in idx and os.path.exists(idx[pkg]):
        cur=lines(lib_dir(idx[pkg]))
    withl=[r for r in v if 'lines' in r]
    state='exists' if cur is not None else 'gone'
    fresh=[r for r in withl if cur is not None and (r['lines']==cur if (cur==0 or r['lines']==0) else max(r['lines'],cur)<=1.25*min(r['lines'],cur))]
    last=v[-1]
    out.append(dict(key=k,pkg=pkg,target=parts[1],types=parts[3],state=state,cur=cur,n=len(v),rowsWithLines=len(withl),fresh=len(fresh),
        lastPeak=last['peakRssBytes'],lastLines=last.get('lines'),freshMax=max([r['peakRssBytes'] for r in fresh],default=None),allMax=max(r['peakRssBytes'] for r in v),manifest=idx.get(pkg)))
json.dump(out,open(sys.argv[3],'w'),ensure_ascii=False,indent=0)
