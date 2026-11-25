import os,re,sys,json,importlib.util,collections
spec=importlib.util.spec_from_file_location('m',os.path.join(os.path.dirname(os.path.abspath(__file__)),'final-carve-manifest-closure.py'));m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
dirs={}
for p in m.paths:
    n,_,_=m.parse(p)
    if n: dirs[n]=os.path.dirname(p)
manifest_dirs=set(dirs.values())
def libroot(n):
    d=dirs[n]
    lp=None
    for line in open(os.path.join(d,'Cargo.toml')):
        mm=re.match(r'path\s*=\s*"([^"]+\.rs)"',line.strip())
        if mm and lp is None and 'workspace' not in line: lp=mm.group(1)
    return os.path.normpath(os.path.join(d,os.path.dirname(lp))) if lp else d
cache={}
def loc(n):
    if n in cache: return cache[n]
    if n not in dirs: cache[n]=0;return 0
    root=libroot(n);tot=0
    for dp,dn,fn in os.walk(root):
        dn[:]=[x for x in dn if x not in('node_modules','target') and '🧪' not in x and os.path.join(dp,x) not in manifest_dirs or os.path.join(dp,x)==dirs[n]]
        for f in fn:
            if f.endswith('.rs'): tot+=os.path.getsize(os.path.join(dp,f))
    cache[n]=tot;return tot
if __name__=='__main__':
    s=json.load(open(sys.argv[1]))
    for n in ['semio-hub-cad','semio-s-artifact-stdio-dwg-snapshot-objects','semio-plugin-actor-input','semio-s-artifact-stdio-binary-io-registry']:
        if n in dirs: print(n,loc(n)/1e6,'MB src')
    json.dump({n:loc(n) for n in set(s['process'])|set(s['demonstrator'])},open(sys.argv[2],'w'))
