import os,sys,re
files=[l.rstrip('\n') for l in sys.stdin]
out={}
for f in files:
    d=os.path.dirname(os.path.abspath(f));found=None
    while d!='/':
        for c in (os.path.join(d,'📦️packages','🦀️rust','Cargo.toml'),os.path.join(d,'Cargo.toml')):
            if os.path.exists(c):
                t=open(c,encoding='utf-8').read()
                if '[package]' in t: found=c;break
        if found:break
        d=os.path.dirname(d)
    if not found: print('NOCRATE',f);continue
    t=open(found,encoding='utf-8').read();n=re.search(r'\[package\][^\[]*?name\s*=\s*"([^"]+)"',t,re.S).group(1)
    out.setdefault(found,set()).add(n)
for m,n in out.items():print(m,sorted(n))
