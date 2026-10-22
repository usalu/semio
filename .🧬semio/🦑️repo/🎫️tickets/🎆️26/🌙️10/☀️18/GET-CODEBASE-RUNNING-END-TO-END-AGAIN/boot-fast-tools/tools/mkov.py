#!/usr/bin/env python3
"""Grow the overlay by ENOENT: run a command in the overlay, copy (clone) each missing module from the base, repeat."""
import os,re,subprocess,sys,shutil
base="/private/tmp/i6/ckpt-hot-cache";ov="/private/tmp/boot-fast/ov2"
S="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts"
args=sys.argv[1:] or ["repo","--help"]
for rnd in range(600):
    r=subprocess.run(["bun",S]+args,cwd=ov,capture_output=True,text=True,timeout=60)
    t=r.stdout+r.stderr
    m=re.search(r"Cannot find (?:module|package) '([^']+)' from '([^']+)'",t) or re.search(r"ENOENT[^']*'([^']+)'",t)
    if not m:
        print("done after",rnd,"rounds rc",r.returncode);print(t[:600]);break
    if len(m.groups())==2:
        spec,frm=m.groups()
        if not spec.startswith("."): print("bare",spec,frm);print(t[:400]);break
        p=os.path.normpath(os.path.join(os.path.dirname(frm),spec))
    else: p=m.group(1)
    rel=os.path.relpath(p,ov);src=os.path.join(base,rel)
    if not os.path.exists(src): print("missing in base",rel);break
    os.makedirs(os.path.dirname(p),exist_ok=True)
    if os.path.isdir(src): subprocess.run(["cp","-cR",src,p])
    else: subprocess.run(["cp","-c",src,p])
    if rnd%10==0: print(rnd,rel[-90:],flush=True)
