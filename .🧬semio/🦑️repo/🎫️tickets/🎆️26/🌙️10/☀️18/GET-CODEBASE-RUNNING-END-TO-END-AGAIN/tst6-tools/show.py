import json,sys,re
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/tst6-tools/"
key=sys.argv[1]; n=int(sys.argv[2]) if len(sys.argv)>2 else 1500
for fn in ("core1.jsonl","r1.jsonl"):
  for l in open(C+fn,encoding="utf-8"):
    r=json.loads(l)
    if key in r["file"] and r.get("log"):
        t=open(r["log"],encoding="utf-8",errors="replace").read()
        i=t.find("error")
        print("##",r["file"][-100:],r["pass_"],r["fail"]); print(t[max(0,i-300):i+n])
