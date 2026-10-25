import subprocess,sys,time,os
root="/Users/ueli/Documents/semio"; os.chdir(root)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; O=C+"/🗑️generated/dtest-d"
files=[l.strip() for l in open(O+"/bunfiles.txt",encoding="utf8") if l.strip()]
skip=("fixtures/","🧭️journeys/","🧭️cli/","🧭️authority/","🎮️registry/","🌀️control-plane","🧊️execution/🟦️.ts","❄️cold-start","🗺️coverage/🟦️.ts")
for f in files:
    if any(s in f for s in skip): continue
    log=O+"/bs-"+str(abs(hash(f)))+".log"; t=time.time()
    try:
        r=subprocess.run(["nice","-n","10","bun","test","./"+f,"--timeout","60000"],capture_output=True,text=True,timeout=300); rc=r.returncode; out=r.stdout+r.stderr
    except subprocess.TimeoutExpired: rc=124; out="timeout"
    open(log,"w",encoding="utf8").write(out)
    tail=[l for l in out.splitlines() if l.strip().endswith(" pass") or l.strip().endswith(" fail")]
    open(O+"/bunsweep.sum","a",encoding="utf8").write(f"rc={rc} {round(time.time()-t)}s {f} {tail} {log}\n")
open(O+"/bunsweep.sum","a").write("done\n")
