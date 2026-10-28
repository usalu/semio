#!/bin/sh
# 🔴 quiet16: runs the given law files of a mirror against the BASE sources of batch `hold` (every source file of the batch swapped to its base bytes, the new laws kept) and swaps the batch sources back afterwards: sh redbase.sh <mirror root> <law path>...
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}; shift
keep=$(mktemp -d)
python3 -I - "$here" "$root" "$keep" save <<'PY'
import importlib.util,os,shutil,sys
here,root,keep,verb=sys.argv[1:5]
spec=importlib.util.spec_from_file_location("make",os.path.join(here,"make.py"));sys.argv=["make.py","hold",root];make=importlib.util.module_from_spec(spec);make.__file__=os.path.join(here,"make.py")
source=open(os.path.join(here,"make.py"),encoding="utf8").read().replace("\nmain()\n","\n")
exec(compile(source,"make.py","exec"),make.__dict__)
for index,rel in enumerate(make.FILES["hold"]):
    if "🧪️tests" in rel: continue
    shutil.copy2(os.path.join(root,rel),os.path.join(keep,str(index)))
    make.write(os.path.join(root,rel),make.base_of(rel,"hold"))
print("[DEBUG] base sources in place")
PY
(cd "$root" && bun test "$@" 2>&1 | grep -a -E "^\(fail\)|^ *[0-9]+ (pass|fail)|Ran ")
python3 -I - "$here" "$root" "$keep" <<'PY'
import os,shutil,sys
here,root,keep=sys.argv[1:4]
make={"__name__":"make","__file__":os.path.join(here,"make.py")};exec(compile(open(os.path.join(here,"make.py"),encoding="utf8").read().replace("\nmain()\n","\n"),"make.py","exec"),make);rels=make["FILES"]["hold"]
for index,rel in enumerate(rels):
    if os.path.exists(os.path.join(keep,str(index))):shutil.copy2(os.path.join(keep,str(index)),os.path.join(root,rel))
shutil.rmtree(keep);print("[DEBUG] batch sources restored")
PY
