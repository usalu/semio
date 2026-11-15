import pickle,json,collections,re,hashlib
out=pickle.load(open('p3/dec.pkl','rb'))
dec=json.JSONDecoder()
ID=re.compile(r'(/\.cache/b/d/m/j/)[0-9a-f]{16,64}(?=/|$)')
VOL={'mtimeMs','ctimeMs','ino','atimeMs','birthtimeMs','dev','nlink','uid','gid','blocks','blksize','mode'}
TOK='⟦semio-root⟧'
def norm(f):
    if isinstance(f,dict): return {k:norm(v) for k,v in f.items() if k not in VOL}
    if isinstance(f,list): return [norm(x) for x in f]
    if isinstance(f,str): return ID.sub(r'\1*',f)
    return f
latest={}
for x,b in out:
    o,_=dec.raw_decode(b.decode('utf8'))
    try: img=json.loads(o['image'])
    except: continue
    root=img.get('root') if isinstance(img,dict) else None
    if root is None or x['platform']=='win32-x64': continue
    k=(root,x['platform'])
    if k not in latest or x['createdMs']>latest[k]['t']:
        rows=[(ob['kind'],ID.sub(r'\1*',ob['path']),json.dumps(norm(ob.get('fact')),sort_keys=True,ensure_ascii=False)) for ob in o['observations'] if ob['path'].startswith(TOK) or ob['kind'] in('stamp','closure')]
        latest[k]=dict(t=x['createdMs'],rows=set(rows),files=frozenset((r[1],json.loads(r[2]).get('sha256')) for r in rows if r[0]=='file'))
del out
roots=collections.defaultdict(dict)
for (root,p),v in latest.items(): roots[root][p]=v
c=collections.Counter();kinds=collections.Counter();pairs_with=collections.Counter()
for root,v in roots.items():
    if 'darwin-arm64' in v and 'linux-x64' in v:
        a,b=v['darwin-arm64'],v['linux-x64']
        if a['files']!=b['files']: c['tree differs (file digests)']+=1;continue
        c['same file digests']+=1
        d=a['rows'].symmetric_difference(b['rows'])
        ks={r[0] for r in d if r[0] not in('analysis','closure','stamp')}
        if not ks: c['  rows equal apart from analysis/closure/stamp (code-state) rows']+=1
        for k in ks: pairs_with[k]+=1
        if ks: c['  environment-dependent rows differ']+=1
print(dict(c));print('pairs with differing kinds',dict(pairs_with))
