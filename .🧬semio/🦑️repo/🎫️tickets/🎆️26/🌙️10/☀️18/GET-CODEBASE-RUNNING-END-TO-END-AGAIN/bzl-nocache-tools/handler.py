#!/usr/bin/env python3
"""handler.py <project:target>: prints the command, the registered class of the script it runs, and its imports of that class."""
import json,sys,os,re
C=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
W="/Users/ueli/Documents/semio/"
rows={l.split("\t")[0]:l.rstrip("\n").split("\t") for l in open(C+"/🗑️generated/bzl-nocache/authored-uncached.tsv")}
for a in sys.argv[1:]:
    r=rows[a]; mf=json.load(open(W+r[3])); t=mf["targets"][a.split(":",1)[1]]
    root=os.path.dirname(W+r[3]); cwd=t.get("options",{}).get("cwd",""); cwd=cwd.replace("{projectRoot}",os.path.relpath(root,W))
    cmd=t.get("options",{}).get("command","")
    m=re.match(r'bun\s+"?([^"\s]+)"?\s*(.*)',cmd)
    script=os.path.normpath(os.path.join(W,cwd,m.group(1))) if m else None
    words=m.group(2).split() if m else []
    print("##",a,"|",cmd[:120]); 
    if not script or not os.path.exists(script): print("  script?",script); continue
    s=open(script,encoding="utf8").read()
    w0=words[0] if words else ""
    reg=re.search(r'register\("%s"\s*,\s*(\w+)'%re.escape(w0),s)
    if reg:
        cls=reg.group(1); i=s.find("class "+cls); print("  class",cls); print("  ",s[i:i+900].replace("\n","\n   "))
    else:
        print("  (no register for %r) head:"%w0, s[:200].replace("\n"," "))
