# 🔬️ Compares two oracle `.short` diagnostics files as multisets (path relative to the sandbox plugin, no line/column): usage `python3 oracle-diff.py <base.short> <new.short> [--all]`; prints lines only in new (regressions) and only in base.
import re,sys,collections,os
def norm(path):
    out=[]
    for l in open(path,encoding='utf8'):
        l=l.rstrip('\n')
        m=re.match(r'^(.*?):\d+:\d+: (.*)$',l)
        if not m: continue
        p=m.group(1)
        p=re.sub(r'^.*/o-[^/]+/','',p)
        p=os.path.normpath(p)
        msg=m.group(2)
        if 'unlinked crate' in msg or 'cannot find attribute' in msg or 'cannot find derive macro' in msg or 'cannot find macro' in msg or msg.startswith('warning'): continue
        out.append(p+' :: '+msg)
    return collections.Counter(out)
a=norm(sys.argv[1]); b=norm(sys.argv[2])
new=b-a; gone=a-b
print('base',sum(a.values()),'new',sum(b.values()),'only-new',sum(new.values()),'only-base',sum(gone.values()))
for k,v in sorted(new.items()): print('+',v,k[:420])
if '--all' in sys.argv:
    for k,v in sorted(gone.items()): print('-',v,k[:420])
