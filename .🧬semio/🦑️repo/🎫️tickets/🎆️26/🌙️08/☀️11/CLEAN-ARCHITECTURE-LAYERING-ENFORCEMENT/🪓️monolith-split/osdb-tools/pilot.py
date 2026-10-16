"""🧪 Pilot: rewrite one module as a whole crate and show the diff."""
import sys,difflib,re,os
from lib import *
from scope import Scope, ALL_MODULE_IDENTS
from rewrite import *
mod=sys.argv[1]
scope=Scope()
res=Resolver(scope)
ident=MODULE_IDENT.get(mod,mod)
path=module_path_ = f'{PRISTINE}/{MODULE_DIRS[mod]}/🦀️.rs'
text=read(path)
st=strip(text)
# test files mounted
tests=[]
for m in re.finditer(r'#\[path = "([^"]+)"\]',text):
    p=os.path.normpath(os.path.join(os.path.dirname(path),m.group(1)))
    if os.path.exists(p) and '🧪️tests' in p: tests.append(p)
ttexts=[read(p) for p in tests]
all_ids=idents(st)
for t in ttexts: all_ids|=idents(strip(t))
local=set(scope.defs[ident])
explicit=set()
for u in find_uses(text):
    for p,a,g in parse_use(u['tree']):
        if not g: explicit.add(a or p[-1])
root_names={n for n in all_ids if n in scope.root and n not in local and n not in explicit}
ctx=FileCtx(res,ident,local)
new=rewrite_uses(text,ctx,all_ids,root_names)
new=rewrite_paths(new,ctx)
for l in difflib.unified_diff(text.split('\n'),new.split('\n'),lineterm='',n=0):
    print(l[:230])
