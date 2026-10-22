#!/usr/bin/env python3
"""grow.py ROOT[,ROOT2..] -- cmd... : run cmd in the first ROOT, copy every module it reports missing from the pristine source into all ROOTs, repeat until the command no longer fails on a missing module."""
import os,re,subprocess,sys
SRC="/private/tmp/i6/ckpt-hot-cache"
roots=sys.argv[1].split(",");cmd=sys.argv[sys.argv.index("--")+1:]
for rnd in range(400):
    r=subprocess.run(cmd,cwd=roots[0],capture_output=True,text=True,timeout=300)
    t=r.stdout+r.stderr
    m=re.search(r"Cannot find (?:module|package) '([^']+)' from '([^']+)'",t)
    if not m:print("settled after",rnd,"copies, rc",r.returncode);print("\n".join(t.strip().splitlines()[-12:])[:1800]);break
    spec,frm=m.groups()
    if not spec.startswith("."):print("bare",spec,"from",frm);break
    p=os.path.normpath(os.path.join(os.path.dirname(frm),spec));rel=os.path.relpath(p,roots[0]);src=os.path.join(SRC,rel)
    if not os.path.exists(src):print("missing in source",rel);break
    for root in roots:
        dst=os.path.join(root,rel);os.makedirs(os.path.dirname(dst),exist_ok=True)
        if not os.path.exists(dst):subprocess.run(["cp","-cR",src,dst])
    if rnd%20==0:print(rnd,rel[-80:],flush=True)
