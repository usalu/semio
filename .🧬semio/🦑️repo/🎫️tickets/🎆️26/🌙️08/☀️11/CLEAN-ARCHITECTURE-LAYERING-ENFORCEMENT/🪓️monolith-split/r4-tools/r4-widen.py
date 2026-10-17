# usage: r4-widen.py <dir> <fn-name>...   widens `pub(crate) fn NAME` to `pub fn NAME` in every 🦀️.rs below <dir> (source change; prints the files it changed)
import os,re,sys
root=sys.argv[1]; names=sys.argv[2:]
for d,_,files in os.walk(root):
    for f in files:
        if f!='🦀️.rs': continue
        p=os.path.join(d,f); t=open(p,encoding='utf8').read(); new=t
        for n in names:
            new=re.sub(r'pub\(crate\)\s*((?:async\s+)?fn\s+'+re.escape(n)+r'\b)',r'pub \1',new)
        if new!=t:
            open(p,'w',encoding='utf8').write(new); print('widened',p)
