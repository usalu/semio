import os,re,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rsitems import items,name_kind
root=sys.argv[1]
ptxt=open(root+'/🦀️.rs',encoding='utf-8').read()
lines=ptxt.split('\n');out=[]
for i,l in enumerate(lines):
    m=re.match(r'^@@REEXPORT:([a-z_]+):(.+)@@$',l)
    if not m:out.append(l);continue
    mod,folder=m.groups()
    txt=open(root+'/'+folder+'/🦀️.rs',encoding='utf-8').read()
    best=0
    for it in items(txt):
        v,k,nm=name_kind(it['toks'])
        if k in('use','impl','mod','macro','include') or k.endswith('!') or not nm:
            continue
        r=2 if v=='pub' else 1
        best=max(best,r)
    # cfg attribute lines directly preceding the mod line
    cfg=None
    j=len(out)-1
    # find the '#[cfg' line before '#[path' of this mod
    k=len(out)-1
    while k>=0 and out[k].startswith('mod ') or (k>=0 and out[k].startswith('pub mod ')) or (k>=0 and out[k].startswith('#[path')):
        k-=1
    if k>=0 and out[k].startswith('#[cfg'):cfg=out[k]
    if best==0:
        print('no re-export for',mod)
        continue
    if cfg:out.append(cfg)
    out.append(('pub use %s::*;' if best==2 else 'use %s::*;')%mod)
    print(('pub ' if best==2 else 'priv ')+mod)
open(root+'/🦀️.rs','w',encoding='utf-8').write('\n'.join(out))
