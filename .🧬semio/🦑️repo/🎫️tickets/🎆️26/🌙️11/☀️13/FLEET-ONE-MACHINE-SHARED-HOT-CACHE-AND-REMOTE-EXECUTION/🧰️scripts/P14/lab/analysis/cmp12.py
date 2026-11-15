import pickle,json,collections,re
rows=pickle.load(open('p14/rows.pkl','rb'))
TOK='⟦semio-root⟧'
VOL={'mtimeMs','ctimeMs','ino','atimeMs','birthtimeMs','dev','nlink','uid','gid','blocks','blksize','mode'}
ID=re.compile(r'(/\.cache/b/d/m/j/)[0-9a-f]{16,64}(?=/)')
def norm(f):
    if isinstance(f,dict): return {k:norm(v) for k,v in f.items() if k not in VOL}
    if isinstance(f,list): return [norm(x) for x in f]
    if isinstance(f,str): return ID.sub(r'\1*',f)
    return f
def items(r,mode):
    out=set()
    for o in r['obs']:
        p=o['path']
        if mode>=1 and not p.startswith(TOK) and not o['kind'] in('stamp','closure'): continue
        if mode>=2: p=ID.sub(r'\1*',p)
        k=o['kind']
        if mode>=2 and k in('closure',): continue
        out.add(json.dumps([k,p,norm(o.get('fact')) if mode>=2 else o.get('fact')],sort_keys=True,ensure_ascii=False))
    return out
by=collections.defaultdict(lambda:collections.defaultdict(list))
for r in rows: by[r['root']][r['plat']].append(r)
for mode in (0,1,2):
    c=collections.Counter();kinds=collections.Counter()
    for root,v in by.items():
        if 'darwin-arm64' in v and 'linux-x64' in v:
            a=max(v['darwin-arm64'],key=lambda r:r['t']);b=max(v['linux-x64'],key=lambda r:r['t'])
            A,B=items(a,mode),items(b,mode)
            miss=A-B
            c['darwin⊆linux' if not miss else 'darwin has rows linux lacks']+=1
            if mode==2:
                for s in miss: kinds[json.loads(s)[0]]+=1
    print('mode',mode,dict(c),dict(kinds))
print('--- restricted to pairs with identical file-row digests')
import hashlib
def filesig(r):
    return frozenset((o['path'],o['fact'].get('sha256') if isinstance(o.get('fact'),dict) else None) for o in r['obs'] if o['kind']=='file')
c=collections.Counter();kinds=collections.Counter();ex={}
for root,v in by.items():
    if 'darwin-arm64' in v and 'linux-x64' in v:
        a=max(v['darwin-arm64'],key=lambda r:r['t']);b=max(v['linux-x64'],key=lambda r:r['t'])
        fa,fb=filesig(a),filesig(b)
        fa={(ID.sub(r'/.cache/b/d/m/j/*',p),s) for p,s in fa};fb={(ID.sub(r'/.cache/b/d/m/j/*',p),s) for p,s in fb}
        if fa!=fb: c['file rows differ (tree drift)']+=1;continue
        A,B=items(a,2),items(b,2);miss=A-B;miss2=B-A
        c['same files; darwin⊆linux' if not miss else 'same files; darwin rows not on linux']+=1
        for s in miss: kinds[json.loads(s)[0]]+=1;ex.setdefault(json.loads(s)[0],s[:240])
print(dict(c));print(dict(kinds));[print(k,v) for k,v in ex.items()]
