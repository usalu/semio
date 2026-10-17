"""tc2: cfg(test)-only items of a parent crate (outside the moved files) whose names a moved test file uses: these would be cross-crate cfg(test) items."""
import os,sys,re,json,glob
sys.path.insert(0,os.path.dirname(__file__)); sys.path.insert(0,os.path.dirname(__file__)+"/ttc")
from ttclib import *
import ttcunion as U
from ttcstub import mask
mans=Manifests(Source(None)); mans.build_index(); src=Source(None)
ITEM=re.compile(r"#\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+|unsafe\s+|const\s+)*(fn|struct|enum|const|static|type|trait|union|mod)\s+([A-Za-z_]\w*)")
for f in sorted(glob.glob(os.path.dirname(__file__)+"/specs/*.json")):
    s=json.load(open(f)); parent=s["parent"]
    man=mans.manifest(parent,src); lib=mans.lib_rel(man)
    files,code,missing=U.reach(src,lib,man["dir"])
    names={}
    for rel in code:
        t=mask(files[rel].decode("utf-8","replace"))
        for m in ITEM.finditer(t):
            if m.group(1)=="mod" : continue
            names.setdefault(m.group(2),rel)
    used={}
    for tf in s["files"]:
        txt=mask(open(ROOT+"/"+tf).read())
        for n,rel in names.items():
            if re.search(r"(?<![A-Za-z0-9_])"+re.escape(n)+r"(?![A-Za-z0-9_])",txt):
                used.setdefault(n,(rel.split("subsets/")[-1][-60:],tf.split("subsets/")[-1][-40:]))
    print(s["name"].replace("semio-s-artifact-",""),"cfg(test) item names in parent:",len(names),"used by moved files:",len(used))
    for n,(r,t) in list(used.items())[:8]: print("    ",n,"defined in",r,"used in",t)
