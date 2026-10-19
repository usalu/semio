import json,os,sys,subprocess
root="/Users/ueli/Documents/semio"
os.chdir(root)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
suites=[l.strip() for l in open(C+"/tst4-tools/envelope-gated.txt") if l.strip()]
pj=subprocess.run(["git","ls-files","--","*project.json"],capture_output=True,text=True).stdout.split("\n")
pj=[p.strip('"') for p in pj if p]
import codecs
out=subprocess.run(["git","-c","core.quotepath=off","ls-files","--","*project.json"],capture_output=True,text=True).stdout.split("\n")
pj=[p for p in out if p.endswith("project.json") and not p.startswith(".🧬semio")]
projs={}
for p in pj:
    try: d=json.load(open(p))
    except Exception as e: continue
    projs[p]=d
res={}
for s in suites:
    cands=[]
    # nearest ancestor project
    parts=s.split("/")
    anc=None
    for i in range(len(parts)-1,0,-1):
        q="/".join(parts[:i]+["📋️project.json"])
        if q in projs: anc=q;break
    sd=os.path.dirname(s); sdn=os.path.basename(sd)
    for p,d in projs.items():
        for tn,t in d.get("targets",{}).items():
            cmd=json.dumps(t,ensure_ascii=False)
            if s in cmd or (sd in cmd):
                cands.append((d.get("name"),tn,p,s in cmd))
    res[s]={"anc":anc,"cands":cands}
json.dump(res,open(C+"/env-pass-tools/map.json","w"),ensure_ascii=False,indent=1)
for s,v in res.items():
    print(s[-90:], v["anc"] and projs[v["anc"]].get("name"), [(n,t,e) for n,t,p,e in v["cands"]][:4])
