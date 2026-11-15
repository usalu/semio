import pickle,json,collections,re,hashlib
out=pickle.load(open('p3/dec.pkl','rb'))
dec=json.JSONDecoder()
rows=[]
for x,b in out:
    o,_=dec.raw_decode(b.decode('utf8'))
    try: img=json.loads(o['image'])
    except: continue
    root=img.get('root') if isinstance(img,dict) else None
    if root is None: continue
    rows.append(dict(root=root,plat=x['platform'],t=x['createdMs'],prod=x['producer'],obs=o['observations'],img=o['image']))
del out
kinds=collections.Counter(); 
for r in rows[:300]:
    for ob in r['obs']: kinds[ob['kind']]+=1
print(kinds)
ex=[ob for ob in rows[0]['obs'] if ob['kind'] in('file','readdir','field','analysis')][:6]
print(json.dumps(ex,ensure_ascii=False)[:1500])
pickle.dump(rows,open('p14/rows.pkl','wb'))
