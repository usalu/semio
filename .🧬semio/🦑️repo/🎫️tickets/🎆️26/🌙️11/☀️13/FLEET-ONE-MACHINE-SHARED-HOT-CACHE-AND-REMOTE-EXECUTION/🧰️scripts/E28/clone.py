#!/usr/bin/env python3
import os,sys,ctypes,stat
libc=ctypes.CDLL("libSystem.B.dylib",use_errno=True)
libc.clonefile.argtypes=[ctypes.c_char_p,ctypes.c_char_p,ctypes.c_uint32]
src,dst=sys.argv[1],sys.argv[2]
prune=set(sys.argv[3].split(",")) if len(sys.argv)>3 else set()
def clone(s,d):
    if libc.clonefile(s.encode(),d.encode(),0)!=0:
        e=ctypes.get_errno()
        raise OSError(e,os.strerror(e),s)
n=0
for root,dirs,files in os.walk(src,followlinks=False):
    dirs[:]=[d for d in dirs if d not in prune]
    rel=os.path.relpath(root,src)
    out=dst if rel=="." else os.path.join(dst,rel)
    os.makedirs(out,exist_ok=True)
    for d in list(dirs):
        p=os.path.join(root,d)
        if os.path.islink(p):
            os.symlink(os.readlink(p),os.path.join(out,d));dirs.remove(d)
    for f in files:
        p=os.path.join(root,f);q=os.path.join(out,f)
        if os.path.islink(p):os.symlink(os.readlink(p),q)
        else:clone(p,q)
        n+=1
print(n,"files cloned")
