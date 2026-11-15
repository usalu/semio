import pickle,json,collections,re,hashlib,datetime
rows=pickle.load(open('p14/rows.pkl','rb'))
VOL={'mtimeMs','ctimeMs','ino','atimeMs','birthtimeMs','dev','nlink','uid','gid','blocks','blksize','mode'}
def norm(f):
    if isinstance(f,dict): return {k:norm(v) for k,v in f.items() if k not in VOL}
    if isinstance(f,list): return [norm(x) for x in f]
    return f
def sig(r):
    items=sorted(json.dumps([o['kind'],o['path'],norm(o.get('fact'))],sort_keys=True,ensure_ascii=False) for o in r['obs'])
    return hashlib.sha256('\n'.join(items).encode()).hexdigest()
for r in rows: r['sig']=sig(r)
by=collections.defaultdict(lambda:collections.defaultdict(list))
for r in rows: by[r['root']][r['plat']].append(r)
cnt=collections.Counter();det=[]
for root,v in by.items():
    ps=sorted(v)
    for i in range(len(ps)):
        for j in range(i+1,len(ps)):
            for a in v[ps[i]]:
                for b in v[ps[j]]:
                    if a['sig']==b['sig']:
                        cnt[(ps[i],ps[j],'same-inputs',a['img']==b['img'])]+=1
                        if a['img']!=b['img']: det.append((root,ps[i],ps[j],a,b))
                    else: cnt[(ps[i],ps[j],'other-inputs')]+=1
for k,n in sorted(cnt.items(),key=str):print(k,n)
pickle.dump(det,open('p14/det.pkl','wb'))
print('differing with same inputs',len(det))
