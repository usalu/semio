import json,subprocess,sys,os
R="/Users/ueli/Documents/semio"
here=os.path.dirname(os.path.abspath(__file__))
units=json.load(open(here+"/units.json"))
extra=json.load(open(here+"/extra_units.json")) if os.path.exists(here+"/extra_units.json") else []
for u in units+extra:
    for label,cfg in [("lib",[]),("test",["test"])]:
        p=subprocess.run(["bun",here+"/closure.ts",R+"/"+u["entry"],R+"/"+u["pkg"],"sum",*cfg],capture_output=True,text=True,cwd=R)
        out=(p.stdout.strip().splitlines() or [p.stderr.strip()[:200]])[-1]
        print(u["name"],"|",os.path.basename(os.path.dirname(u["entry"])),"|",label,"|",out,flush=True)
