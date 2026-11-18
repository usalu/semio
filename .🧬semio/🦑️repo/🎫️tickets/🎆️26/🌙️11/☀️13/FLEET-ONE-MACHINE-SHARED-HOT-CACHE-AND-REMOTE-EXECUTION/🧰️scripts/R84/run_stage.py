#!/usr/bin/env python3
"""R84: run the STAGED version of a test file (never the live file) through the genuine-birth harness as an absolute-import shadow; staged sibling test files that it imports are shadowed too. usage: run_stage.py <staged id|rel substring> [birth] [pattern] [art]"""
import sys,os,subprocess,json
H=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,H)
import shadow
R=shadow.R
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R84/stage/"
rows=[l.split(" ") for l in open(S+"index.txt",encoding="utf8").read().splitlines() if l]
key=sys.argv[1];pick=[r for r in rows if r[0]==key or key in r[1]]
assert len(pick)==1,pick
tests=[r for r in rows if r[1].endswith(".ts") and "🧪️tests" in r[1]]
paths={r[1]:shadow.shadow(r[1],open(S+r[0]+".new",encoding="utf8").read(),"stage-"+r[0]) for r in tests}
jsons={}
for r in rows:
    if r[1].endswith(".json") and ("🧪️tests" in r[1] or "🧬️schema" in r[1]):
        q=os.path.join(R,".cache/test-sandbox/r84-shadow/json-"+r[0]+".json");open(q,"w",encoding="utf8").write(open(S+r[0]+".new",encoding="utf8").read());jsons[r[1]]=q
for rel,p in paths.items():
    t=open(p,encoding="utf8").read()
    for other,q in jsons.items():t=t.replace(json.dumps(os.path.join(R,other),ensure_ascii=False),json.dumps(q,ensure_ascii=False))
    for other,q in paths.items():
        if other!=rel:t=t.replace(json.dumps(os.path.join(R,other),ensure_ascii=False),json.dumps(q,ensure_ascii=False))
    open(p,"w",encoding="utf8").write(t)
args=["python3",H+"/run1.py",paths[pick[0][1]]]+sys.argv[2:]
r=subprocess.run(args,capture_output=True,text=True);print(r.stdout.strip());print(r.stderr.strip()[:500])
