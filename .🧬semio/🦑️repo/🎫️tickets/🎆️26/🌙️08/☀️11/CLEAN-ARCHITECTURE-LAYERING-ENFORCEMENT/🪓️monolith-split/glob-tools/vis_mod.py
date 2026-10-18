import sys,os,re,json,pickle
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import plan
from plan import *
VIS={'sc':'pub','dc':'pub(crate)','dgd':'pub(crate)'}
# EXT needs the pre-edit analysis: rebuild from backups
HERE=os.path.dirname(os.path.abspath(__file__))
import shutil
for key,vis in VIS.items():
    rootp=os.path.join(find(roots[key]),'🦀️.rs')
    cur=read(rootp)
    shutil.copy(rootp,HERE+'/cur_'+key+'.rs')
    shutil.copy(HERE+'/orig/'+key+'.rs',rootp)
    try:
        build(key)
        mods=sorted({m for (m,cfg) in plan.EXT[key]})
    finally:
        open(rootp,'w',encoding='utf-8').write(cur)
        os.remove(HERE+'/cur_'+key+'.rs')
    new=cur
    for m in mods:
        pat=re.compile(r'^(\s*)mod %s;'%m,re.M)
        assert pat.search(new),(key,m)
        new=pat.sub(lambda mm: f'{mm.group(1)}{vis} mod {m};',new,1)
    assert read(rootp)==cur
    open(rootp,'w',encoding='utf-8').write(new)
    print(key,vis,mods)
