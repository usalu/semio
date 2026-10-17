"""tc2: associated functions `Type::name(` called by moved files where the parent crate defines `name` in an inherent `impl Type` without `pub`."""
import json,glob,os,re,sys,collections
sys.path.insert(0,os.path.dirname(__file__)); sys.path.insert(0,os.path.dirname(__file__)+"/ttc")
import mkcomp,unglob
import ttcunion as U
from ttclib import *
from ttcstub import mask
R="/Users/ueli/Documents/semio/"
IMPL=re.compile(r"(?<![A-Za-z0-9_])impl(?:<[^{>]*(?:<[^>]*>[^{>]*)*>)?\s+([A-Za-z_][A-Za-z0-9_]*)(?:<[^{]*>)?\s*(?:where[^{]*)?\{")
FN=re.compile(r"^\s*((?:pub(?:\([^)]*\))?\s+)?)(?:const\s+|async\s+|unsafe\s+)*fn\s+([A-Za-z_]\w*)",re.M)
bad=0
for f in sorted(glob.glob(os.path.dirname(__file__)+"/specs/*.json")):
    s=json.load(open(f)); parent=s["parent"]
    man,mdir,lib=mkcomp.parent_info(parent)
    files,code,missing=U.reach(Source(None),mkcomp.MANS.lib_rel(man),man["dir"])
    meth=collections.defaultdict(dict)
    for rel in code:
        t=mask(files[rel].decode("utf-8","replace"))
        for m in IMPL.finditer(t):
            depth=0;i=m.end()-1;j=i
            while j<len(t):
                if t[j]=="{": depth+=1
                elif t[j]=="}":
                    depth-=1
                    if depth==0: break
                j+=1
            body=t[i+1:j]
            # only depth-0 fns of the body
            d=0;seg=[]
            for k,ch in enumerate(body):
                if ch=="{": d+=1
                elif ch=="}": d-=1
                seg.append(ch if d==0 or ch in "{}" and d<=1 else " ")
            top="".join(seg)
            for fm in FN.finditer(top):
                meth[m.group(1)].setdefault(fm.group(2),fm.group(1).strip() or "private")
    for tf in s["files"]:
        t=mask(open(R+tf).read())
        for m in re.finditer(r"(?<![A-Za-z0-9_:])([A-Z][A-Za-z0-9_]*)::([a-z_][a-z0-9_]*)\s*\(",t):
            ty,fn=m.group(1),m.group(2)
            vis=meth.get(ty,{}).get(fn)
            if vis is not None and vis!="pub":
                bad+=1; print("PRIVATE-ASSOC",s["name"][17:],tf.split("subsets/")[-1][-45:],f"{ty}::{fn}",vis)
print("private associated fns called by moved files:",bad)
