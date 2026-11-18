#!/usr/bin/env python3
"""R84: run a staged test with literal debug edits: dbg_stage.py <rel substring> <birth> <pattern> <old> <new> [art]; prints lines containing DBG from the output."""
import sys,os,subprocess,hashlib
H=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,H)
import shadow
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R84/"
rows=[l.split(" ") for l in open(S+"stage/index.txt",encoding="utf8").read().splitlines() if l]
pick=[r for r in rows if sys.argv[1] in r[1]];assert len(pick)==1,pick
t=open(S+"stage/"+pick[0][0]+".new",encoding="utf8").read()
import json
edits=json.load(open(sys.argv[4])) if sys.argv[4].endswith(".json") else [[sys.argv[4],sys.argv[5]]]
for old,new in edits:
    assert t.count(old)==1,(old[:60],t.count(old));t=t.replace(old,new)
path=shadow.shadow(pick[0][1],t,"dbg-"+pick[0][0])
jt=open(path,encoding="utf8").read()
for r in rows:
    if r[1].endswith(".json") and ("🧪️tests" in r[1] or "🧬️schema" in r[1]):
        q=os.path.join(shadow.R,".cache/test-sandbox/r84-shadow/json-"+r[0]+".json");open(q,"w",encoding="utf8").write(open(S+"stage/"+r[0]+".new",encoding="utf8").read());jt=jt.replace(json.dumps(os.path.join(shadow.R,r[1]),ensure_ascii=False),json.dumps(q,ensure_ascii=False))
open(path,"w",encoding="utf8").write(jt)
r=subprocess.run(["python3",H+"/run1.py",path,sys.argv[2],sys.argv[3]]+sys.argv[6:],capture_output=True,text=True);print(r.stdout.strip()[:400])
out=open(S+"out/"+hashlib.md5(path.encode()).hexdigest()[:8]+".out",encoding="utf8",errors="replace").read()
i=out.find("DBG")
while i>=0:
    print(out[i:i+1800]);print("....");i=out.find("DBG",i+1800)
