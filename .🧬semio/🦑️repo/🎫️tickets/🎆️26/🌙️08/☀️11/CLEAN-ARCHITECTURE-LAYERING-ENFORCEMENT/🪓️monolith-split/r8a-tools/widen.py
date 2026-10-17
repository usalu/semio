import os,re,sys,json,time
# usage: widen.py <tag> <dir>... : restricted visibility (crate/super/self/in path) -> pub in non-test .rs files; undo json in ~/.cache/r8a
tag=sys.argv[1]; undo={}; n=0
pat=re.compile(r'\bpub\((?:crate|super|self|in [A-Za-z_:]+)\)')
for d in sys.argv[2:]:
    for dp,dn,fn in os.walk(d):
        if '🧪️tests' in dp or '🧫️fixtures' in dp: continue
        for f in fn:
            if not f.endswith('.rs'): continue
            p=os.path.join(dp,f); t=open(p,encoding='utf8').read()
            u=pat.sub('pub',t)
            if u!=t: undo[p]=t; open(p,'w',encoding='utf8').write(u); n+=len(pat.findall(t))
json.dump(undo,open(f'/Users/ueli/.cache/r8a/undo-{tag}-{int(time.time())}.json','w'))
print('files',len(undo),'sites',n)
