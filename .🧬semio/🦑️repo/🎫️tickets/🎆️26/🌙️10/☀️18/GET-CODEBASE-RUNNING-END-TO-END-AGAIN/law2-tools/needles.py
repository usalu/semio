#!/usr/bin/env python3
# usage: needles.py <list-index> <test-name-substring> [scope-path-prefix]
import re,subprocess,sys,json
R="/Users/ueli/Documents/semio/"
C=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
lst=open(C+"/law2-tools/list-mine.txt",encoding="utf-8").read().split("\n")
f=lst[int(sys.argv[1])]; name=sys.argv[2]; scope=sys.argv[3] if len(sys.argv)>3 else None
s=open(R+f,encoding="utf-8").read()
i=s.find(name)
j=s.find("\ntest(",i+10); j=len(s) if j<0 else j
body=s[i:j]
needles=[]
for m in re.finditer(r'(not\.)?toContain\(\s*("(?:[^"\\]|\\.)*"|`(?:[^`\\]|\\.)*`)\s*\)',body):
    try: v=json.loads(m.group(2)) if m.group(2)[0]=='"' else m.group(2)[1:-1]
    except Exception: v=m.group(2)[1:-1]
    needles.append((bool(m.group(1)),v))
print("needles",len(needles))
for neg,v in needles:
    args=["git","grep","-l","-F","--",v] if False else ["git","grep","-l","-F","-e",v,"--"]+([scope] if scope else ["*.rs"])
    r=subprocess.run(args,cwd=R,capture_output=True,text=True)
    hits=[h for h in r.stdout.split("\n") if h and "🗑️generated" not in h and "/.🧬semio/" not in h]
    print(("NOT " if neg else "")+repr(v[:90]),"->",len(hits),[h[-70:] for h in hits[:2]])
