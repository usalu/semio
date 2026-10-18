import os,re,sys
root=sys.argv[1]
pat=re.compile(r'^(struct|enum) (\w+)[^\n]*\{\s*$')
out=[]
for dp,dn,fn in os.walk(root):
    dn[:]=[d for d in dn if d not in ('node_modules','dist','target','.git','🗑️generated')]
    for f in fn:
        if not f.endswith('.rs'): continue
        p=os.path.join(dp,f)
        try: L=open(p,encoding='utf8').read().split('\n')
        except: continue
        for i,l in enumerate(L):
            m=re.match(r'^(struct|enum) (\w+)',l)
            if m and not l.startswith('pub'):
                nxt=' '.join(L[i+1:i+4])
                if 'pub(super)' in nxt or 'pub(crate)' in nxt:
                    out.append((p,i+1,l.strip()[:80]))
for o in out: print(*o,sep='|')
print(len(out),'candidates',file=sys.stderr)
