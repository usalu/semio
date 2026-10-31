import sys,re,collections
t=open(sys.argv[1],encoding='utf-8',errors='replace').read().split('\n')
cur=collections.defaultdict(list)
# errors are printed before the 'could not compile `crate`' line; attribute by scanning forward
pending=[]
out=collections.OrderedDict()
for i,l in enumerate(t):
    if re.match(r'error(\[E\d+\])?:',l):
        m=re.match(r'error: could not compile `([^`]+)`',l)
        if m:
            out[m.group(1)]=pending;pending=[];continue
        if 'aborting' in l or l.startswith('error: cargo') or 'Rust crate Source' in l: continue
        loc=t[i+1].strip() if i+1<len(t) else ''
        pending.append((l[:150],re.sub(r'.*(/[^/]*/[^/]*/[^/]*/[^/]*)$',r'\1',loc.replace('--> ',''))[:110]))
for k,v in out.items():
    print(f"## {k} ({len(v)})")
    for a,b in v[:int(sys.argv[2])]: print("  ",a,"|",b)
