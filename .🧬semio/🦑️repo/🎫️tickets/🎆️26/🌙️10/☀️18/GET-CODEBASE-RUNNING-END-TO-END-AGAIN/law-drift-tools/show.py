import json,sys,re
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r1=[json.loads(l) for l in open(C+"/🗑️generated/law-drift-run/r1.jsonl")]
import os
cur={}
for fn in ["r1","r2","r3"]:
    q=C+"/🗑️generated/law-drift-run/"+fn+".jsonl"
    if os.path.exists(q):
        for l in open(q): 
            d=json.loads(l); cur[d['file']]=d
rs=[cur[r['file']] for r in r1]
n=int(sys.argv[1]); mx=int(sys.argv[2]) if len(sys.argv)>2 else 2500
r=rs[n]; print(r['file'])
t=open(r['log'],encoding='utf-8').read() if r.get('log') else ''
i=t.find('error:')
print(t[max(0,i-200):max(0,i-200)+mx] if i>=0 else t[:mx])
