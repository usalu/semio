import sys; sys.path.insert(0,'.')
from finish7 import *
if True:
    W,writes,deletes=finish()
    outd=GEN+'/pwp7-out'
    import shutil
    if os.path.exists(outd): shutil.rmtree(outd)
    for rel,txt in writes.items():
        p=outd+'/'+rel
        os.makedirs(os.path.dirname(p),exist_ok=True)
        open(p,'w',encoding='utf-8').write(txt)
    print(len(writes),'writes')
