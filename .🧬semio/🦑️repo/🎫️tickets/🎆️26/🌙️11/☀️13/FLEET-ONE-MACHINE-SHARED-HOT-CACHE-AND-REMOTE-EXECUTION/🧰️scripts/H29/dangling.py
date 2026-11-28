# -*- coding: utf-8 -*-
import os,re
S=os.path.dirname(os.path.abspath(__file__));ov=S+"/overlay/";live="/Users/ueli/Documents/semio/"
bad=0;n=0
for d,_,names in os.walk(ov):
    for name in names:
        if not (name.endswith(".ts") or name.endswith(".json")) or "probe" in d: continue
        if name.endswith(".json"): continue
        p=os.path.join(d,name);t=open(p,encoding="utf-8").read()
        for m in re.finditer(r'(?:from|import)\s*\(?\s*"(\.{1,2}/[^"]+)"',t):
            spec=m.group(1);target=os.path.normpath(os.path.join(live+os.path.relpath(d,ov),spec));n+=1
            rel=os.path.relpath(target,live)
            if not (os.path.exists(target) or os.path.exists(ov+rel)):
                bad+=1;print("DANGLING",os.path.relpath(p,ov)[-60:],spec)
print(n,"imports",bad,"dangling")
