#!/usr/bin/env python3
"""Runs a command inside an overlay root and copies every module it cannot find from the live checkout (read only) until it runs or a module is missing live. Usage: closure-run.py <overlay> -- <command...>"""
import os,re,shutil,subprocess,sys
R="/Users/ueli/Documents/semio"
ov=sys.argv[1];cmd=sys.argv[sys.argv.index("--")+1:]
for i in range(300):
    p=subprocess.run(cmd,capture_output=True,text=True,cwd=ov,env={**os.environ,"SEMIO_TEST_REPO_ROOT":R})
    out=p.stdout+p.stderr
    m=re.search(r"Cannot find module '([^']+)' from '([^']+)'",out) or re.search(r"Cannot find package '[^']+' from '([^']+)'",out)
    if not m or len(m.groups())<2:
        print(out[-6000:]);sys.exit(p.returncode)
    spec,frm=m.groups()
    target=os.path.normpath(os.path.join(os.path.dirname(frm),spec));rel=os.path.relpath(target,ov);src=os.path.join(R,rel)
    if not os.path.exists(src):print("missing live:",rel);print(out[-1500:]);sys.exit(3)
    os.makedirs(os.path.dirname(target),exist_ok=True);shutil.copyfile(src,target);print("copied",rel[-90:],file=sys.stderr)
