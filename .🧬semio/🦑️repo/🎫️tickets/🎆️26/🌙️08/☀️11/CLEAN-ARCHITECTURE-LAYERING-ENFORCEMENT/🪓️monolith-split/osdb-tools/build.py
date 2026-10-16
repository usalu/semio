"""🏗️ Build the staged tree: compose all crates into SP/stage/db and write the oracle crate list."""
import json,os,shutil,sys,collections
from lib import *
from compose import Composer
from plan import CRATES, SPLIT_MODULES


def mirror_external(stage):
    """Copy include_str!/include_bytes! targets that live outside the db tree into the stage mirror (census/oracle only)."""
    import re
    n=0
    for root,ds,fs in os.walk(stage):
        for f in fs:
            if not f.endswith('.rs'): continue
            p=os.path.join(root,f)
            for m in re.finditer(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"',read(p)):
                tgt=os.path.normpath(os.path.join(root,m.group(1)))
                if tgt.startswith(stage+'/') or os.path.exists(tgt): continue
                real=os.path.join(REPO,os.path.relpath(tgt,STAGE_ROOT))
                if os.path.exists(real):
                    os.makedirs(os.path.dirname(tgt),exist_ok=True); shutil.copy(real,tgt); n+=1
    print('mirrored external includes',n)

def main():
    c=Composer()
    out=c.run()
    shutil.rmtree(STAGE_ROOT,ignore_errors=True)
    shutil.copytree(PRISTINE,STAGE)
    # delete originals that belong to split modules and were not rewritten in place
    gone=[]
    for fid,s in c.srcs.items():
        if s.module in SPLIT_MODULES and fid not in out and s.role in ('sub','test','main') :
            if s.role=='keep': continue
            p=os.path.join(STAGE,fid)
            if os.path.exists(p): os.remove(p); gone.append(fid)
    for rel,text in out.items():
        write(os.path.join(STAGE,rel),text)
    crates=[]
    for cr in CRATES:
        root=f'{cr.node}/🦀️.rs'
        files=collect_files(os.path.join(STAGE,root))
        crates.append(dict(lib=cr.lib,root=root,files=[os.path.relpath(f,STAGE) for f in files]))
    json.dump(crates,open(f'{SP}/crates.json','w'),ensure_ascii=False,indent=1)
    print('written',len(out),'deleted',len(gone))
    for k,v in c.report.items(): print(k,v[:6])
    from rewrite import UNKNOWN
    print('unknown homes',sorted(UNKNOWN))
if __name__=='__main__': main()
