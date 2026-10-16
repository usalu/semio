"""🧱 Stage 1: every module is its own crate (lib = module file). Writes the staged tree and prints findings."""
import os,re,shutil,sys,json
from lib import *
from scope import Scope, ALL_MODULE_IDENTS, STANDALONE_MODULES, module_path
from rewrite import *
from lib import collect_files

STAGE=f'{SP}/stage1/db'
GATED={'db_engine','db_cli','db_storage_driver_runtime','db_fault_testing'}

def main():
    scope=Scope(); res=Resolver(scope)
    shutil.rmtree(f'{SP}/stage1',ignore_errors=True)
    shutil.copytree(PRISTINE,STAGE)
    report={}
    for ident in ALL_MODULE_IDENTS:
        root=module_path(ident)
        files=collect_files(root)
        local=set(scope.defs[ident])
        texts={f:read(f) for f in files}
        all_ids=set()
        for f in files: all_ids|=idents(strip(texts[f]))
        explicit=set()
        for u in find_uses(texts[root]):
            for p,a,g in parse_use(u['tree']):
                if not g: explicit.add(a or p[-1])
        root_names={n for n in all_ids if n in scope.root and n not in local and n not in explicit}
        for f in files:
            t=texts[f]
            ctx=FileCtx(res,ident,local)
            file_ids=idents(strip(t)) if f!=root else all_ids
            new=rewrite_uses(t,ctx,file_ids if f!=root else all_ids,root_names if f==root else set())
            new=rewrite_paths(new,ctx)
            if f==root:
                head=[]
                if 'async fn' in new: head.append('#![allow(async_fn_in_trait)]')
                if ident in GATED: head.append('#![cfg(not(target_arch = "wasm32"))]')
                ext=[a for a in ('pack','dsl','store') if re.search(r'(?<![A-Za-z0-9_:])'+a+'::',strip(new))]
                for a in ext: head.append(f'extern crate semio_framework_os_kernel as {a};')
                if head:
                    ls=new.split('\n')
                    k=0
                    while k<len(ls) and ls[k].startswith('//!'): k+=1
                    new='\n'.join(ls[:k]+head+([''] if k<len(ls) and ls[k].strip() else [])+ls[k:])
            out=STAGE+os.path.relpath(f,PRISTINE)[0:0]+'/'+os.path.relpath(f,PRISTINE)
            write(out,new)
        report[ident]=dict(files=len(files),root_names=sorted(root_names))
    json.dump(report,open(f'{SP}/stage1.report.json','w'),ensure_ascii=False,indent=1)
    print('staged',len(report))
if __name__=='__main__': main()
