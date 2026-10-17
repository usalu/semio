import sys,os,re,glob
sys.path.insert(0,os.path.dirname(__file__)+"/ttc")
from ttclib import *
import ttcunion as U
mans=Manifests(Source(None)); mans.build_index()
src=Source(None)
for pkg in sys.argv[1:]:
    man=mans.manifest(pkg,src)
    lib=mans.lib_rel(man)
    files,code,missing=U.reach(src,lib,man["dir"])
    tot=0;fl=0
    for rel in code:
        t=files[rel].decode("utf-8","replace")
        n=len(re.findall(r"cfg_attr\(\s*test\b",t))
        if n: tot+=n; fl+=1
    print(pkg.replace("semio-s-artifact-",""),"files",len(code),"cfg_attr(test) sites",tot,"in",fl,"files")
