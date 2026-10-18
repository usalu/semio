#!/usr/bin/env python3
import subprocess,re,sys,os
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
W=C+"/🗑️generated/gofix2"
CL="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli"
env=dict(os.environ,GOWORK="/Users/ueli/Documents/semio/go.work")
for _ in range(20):
    subprocess.run(["python3",C+"/gocomp/overlay.py",CL,W+"/plan-test.json","--tests"],capture_output=True)
    r=subprocess.run(["go","test","-overlay="+W+"/plan-test.json","-gcflags=-e","-c","-o","/dev/null","."],cwd=CL,env=env,capture_output=True,text=True)
    errs=re.findall(r'^\./(.+?\.go):(\d+):\d+: "([^"]+)" imported and not used',r.stderr,re.M)
    other=[l for l in r.stderr.split("\n") if l and "imported and not used" not in l and not l.startswith("#")]
    if not errs:
        print("remaining:","\n".join(other[:20]) if other else "none"); break
    byfile={}
    for f,ln,imp in errs: byfile.setdefault(f,[]).append(int(ln))
    for f,lns in byfile.items():
        path=CL+"/"+f
        lines=open(path,encoding="utf-8").read().split("\n")
        for ln in sorted(lns,reverse=True): del lines[ln-1]
        open(path,"w",encoding="utf-8").write("\n".join(lines)); print("fixed imports",f,len(lns))
