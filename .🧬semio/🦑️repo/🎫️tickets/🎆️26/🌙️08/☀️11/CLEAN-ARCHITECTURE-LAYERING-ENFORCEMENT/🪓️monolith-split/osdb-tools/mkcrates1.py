import json,os
from lib import *
from scope import ALL_MODULE_IDENTS, module_path
stage=f'{SP}/stage1/db'
out=[]
for ident in ALL_MODULE_IDENTS:
    root=module_path(ident,stage)
    files=collect_files(root)
    out.append(dict(lib=ident,root=os.path.relpath(root,stage),files=[os.path.relpath(f,stage) for f in files]))
json.dump(out,open(f'{SP}/crates1.json','w'),ensure_ascii=False,indent=1)
print(len(out),sum(len(c['files']) for c in out))
