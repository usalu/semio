import os,re,sys
root="/home/deck/semio"
roots=["✏️s","🧰️framework","🌎️hub","♻️mit-bestand","🏢️semio-tech","👴️leutwiler","🎓️teaching"]
skip={"node_modules","target","dist","build","out","bin","obj","__pycache__","storybook-static","🎫️tickets","⚡️cache","🤖️generated","🗑️generated"}
hi=[("SetSnapshot",re.compile("SetSnapshot")),("NoMutation",re.compile("NoMutation")),("CollectionMutation<...>",re.compile(r"CollectionMutation(<|::)"))]
for r in roots:
    for d,ds,fs in os.walk(os.path.join(root,r)):
        ds[:]=[x for x in ds if x not in skip and not x.startswith(".")]
        for f in fs:
            if not f.endswith(".rs"):continue
            p=os.path.join(d,f);rel=os.path.relpath(p,root)
            if "/🧬️mutations/" not in "/"+rel and "/🎮️commands/" not in "/"+rel:continue
            t=open(p,encoding="utf-8",errors="replace").read()
            h=[l for l,x in hi if x.search(t)]
            if h:print("vocab-high\t"+rel+"\t"+",".join(h))
