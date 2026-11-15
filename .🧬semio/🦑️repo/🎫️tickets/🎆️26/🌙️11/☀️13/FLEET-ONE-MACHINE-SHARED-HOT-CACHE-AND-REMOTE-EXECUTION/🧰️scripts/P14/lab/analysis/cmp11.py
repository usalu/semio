import pickle,json,collections,re
rows=pickle.load(open('p14/rows.pkl','rb'))
latest={}
for r in rows:
    k=(r['root'],r['plat'])
    if k not in latest or r['t']>latest[k]['t']: latest[k]=r
TOK='⟦semio-root⟧'
def classes(r):
    c=set()
    for o in r['obs']:
        p=o['path']
        if o['kind'] in('closure','stamp'): continue
        if p.startswith(TOK):
            rest=p[len(TOK):]
            if rest.startswith('/.cache/') : c.add('in-root-cache(.cache)')
            elif '/node_modules/' in rest: c.add('node_modules')
        elif p.startswith('/') : c.add('absolute-outside-root')
        elif re.match(r'^[A-Za-z]:',p): c.add('windows-abs')
    return c
for plat in ('darwin-arm64','linux-x64','win32-x64'):
    rs=[r for (root,p),r in latest.items() if p==plat]
    cnt=collections.Counter()
    for r in rs:
        cs=classes(r)
        for c in cs: cnt[c]+=1
        cnt['any' if cs else 'none']+=1
    print(plat,len(rs),dict(cnt))
ex=collections.Counter()
for (root,p),r in latest.items():
    if p!='darwin-arm64':continue
    for o in r['obs']:
        pp=o['path']
        if pp.startswith('/') : ex[(o['kind'],re.sub(r'[0-9a-f]{16,}','H',pp)[:70])]+=1
print(ex.most_common(12))
exc=collections.Counter()
for (root,p),r in latest.items():
    if p!='darwin-arm64':continue
    for o in r['obs']:
        pp=o['path']
        if pp.startswith(TOK+'/.cache/'): exc[(o['kind'],re.sub(r'[0-9a-f]{16,}','H',pp)[len(TOK):][:60])]+=1
print(exc.most_common(12))
