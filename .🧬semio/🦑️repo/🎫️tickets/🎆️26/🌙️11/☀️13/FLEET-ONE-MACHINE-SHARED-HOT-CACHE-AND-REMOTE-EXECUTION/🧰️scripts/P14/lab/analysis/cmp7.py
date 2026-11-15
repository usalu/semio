import pickle,json,collections,re
rows=pickle.load(open('p14/rows.pkl','rb'))
VOL={'mtimeMs','ctimeMs','ino','atimeMs','birthtimeMs','dev','nlink','uid','gid','blocks','blksize','mode'}
def norm(f):
    if isinstance(f,dict): return {k:norm(v) for k,v in f.items() if k not in VOL}
    if isinstance(f,list): return [norm(x) for x in f]
    return f
def items(r): return {json.dumps([o['kind'],o['path'],norm(o.get('fact'))],sort_keys=True,ensure_ascii=False) for o in r['obs']}
by=collections.defaultdict(lambda:collections.defaultdict(list))
for r in rows: by[r['root']][r['plat']].append(r)
kindsA=collections.Counter();kindsB=collections.Counter();ex={}
n=0;allsame_kind=collections.Counter()
for root,v in by.items():
    if 'darwin-arm64' in v and 'linux-x64' in v:
        a=max(v['darwin-arm64'],key=lambda r:r['t']);b=max(v['linux-x64'],key=lambda r:r['t'])
        A,B=items(a),items(b);onlyA=A-B;onlyB=B-A;n+=1
        ks=set()
        for s in onlyA:
            k=json.loads(s)[0];kindsA[k]+=1;ks.add(k);ex.setdefault(('A',k),s[:260])
        for s in onlyB:
            k=json.loads(s)[0];kindsB[k]+=1;ks.add(k);ex.setdefault(('B',k),s[:260])
        allsame_kind[tuple(sorted(ks))]+=1
print('pairs',n);print('only darwin',kindsA);print('only linux',kindsB)
for k,v in ex.items():print(k,v)
print(allsame_kind.most_common(12))
