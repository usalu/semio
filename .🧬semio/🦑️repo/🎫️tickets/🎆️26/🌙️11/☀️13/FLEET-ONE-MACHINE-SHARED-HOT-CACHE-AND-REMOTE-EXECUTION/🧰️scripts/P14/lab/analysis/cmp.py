import pickle,json,collections,hashlib,sys
out=pickle.load(open('p3/dec.pkl','rb'))
dec=json.JSONDecoder()
rows=[]
for x,b in out:
    o,_=dec.raw_decode(b.decode('utf8'))
    try: img=json.loads(o['image'])
    except: continue
    root=img.get('root') if isinstance(img,dict) else None
    if root is None: continue
    rows.append(dict(root=root,plat=x['platform'],t=x['createdMs'],prod=x['producer'].split(':')[0][-14:],hd=o['digest'],img=o['image'],obs=o['observations'],name=img.get('name')))
del out
print('records',len(rows),collections.Counter(r['plat'] for r in rows))
by=collections.defaultdict(lambda:collections.defaultdict(list))
for r in rows: by[r['root']][r['plat']].append(r)
print('roots',len(by),collections.Counter(tuple(sorted(v)) for v in by.values()))
def canon(s): return hashlib.sha256(s.encode()).hexdigest()[:12]
res=collections.Counter(); diffs=[]
for root,v in by.items():
    plats=sorted(v)
    if len(plats)<2: continue
    for i in range(len(plats)):
        for j in range(i+1,len(plats)):
            a=sorted(v[plats[i]],key=lambda r:r['t'])[-1]; b=sorted(v[plats[j]],key=lambda r:r['t'])[-1]
            pair=(plats[i],plats[j])
            same_hd=a['hd']==b['hd']
            same_img=a['img']==b['img']
            res[(pair,same_hd,same_img)]+=1
            if not same_img: diffs.append((root,pair,same_hd,a,b))
for k,v in sorted(res.items(),key=str): print(k,v)
pickle.dump([(root,pair,hd,a['img'],b['img'],a['name']) for root,pair,hd,a,b in diffs],open('p14/diffs.pkl','wb'))
print('diff roots',len(diffs))
