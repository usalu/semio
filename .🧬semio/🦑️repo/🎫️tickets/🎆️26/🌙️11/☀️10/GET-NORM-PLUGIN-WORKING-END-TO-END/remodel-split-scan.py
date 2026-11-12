import os,re,sys
pat=re.compile(r'(#\[cfg\(test\)\]\s*)?(?:#\[[^\]]*\]\s*)*?#\[path\s*=\s*"([^"]+)"\]|include_(?:str|bytes)!\(\s*"([^"]+)"|include!\(\s*"([^"]+)"')
def closure(root,test):
    seen={};st=[os.path.normpath(root)]
    while st:
        f=st.pop()
        if f in seen or not os.path.isfile(f): continue
        seen[f]=os.path.getsize(f)
        if not f.endswith('.rs'): continue
        d=os.path.dirname(f)
        try: t=open(f,encoding='utf8').read()
        except Exception: continue
        for m in pat.finditer(t):
            if m.group(1) and not test: continue
            p=m.group(2) or m.group(3) or m.group(4)
            if p=='.': continue
            st.append(os.path.normpath(os.path.join(d,p)))
    return sum(seen.values()),len(seen)
rows=[]
for dp,dn,fn in os.walk("."):
    dn[:]=[d for d in dn if d not in("target","node_modules",".git","🗑️generated","vendor",".cargo")]
    if "Cargo.toml" not in fn: continue
    p=os.path.join(dp,"Cargo.toml")
    t=open(p,encoding="utf8").read()
    if "[package.metadata.semio" not in t: continue
    n=re.search(r'^name\s*=\s*"([^"]+)"',t,re.M)
    m=re.search(r'maxSourceBytes\s*=\s*(\d+)',t)
    l=re.search(r'\[lib\][^\[]*?path\s*=\s*"([^"]+)"',t,re.S)
    if not n or not l: continue
    lib=os.path.normpath(os.path.join(dp,l.group(1)))
    lim=int(m.group(1)) if m else 262144
    a=closure(lib,False);b=closure(lib,True)
    r=max(a[0],b[0])/lim
    if r>=0.9: rows.append((r,a[0],b[0],a[1],b[1],lim,n.group(1),dp))
for r in sorted(rows,reverse=True): print("%.3f lib=%d test=%d files=%d/%d limit=%d %s %s"%r)
