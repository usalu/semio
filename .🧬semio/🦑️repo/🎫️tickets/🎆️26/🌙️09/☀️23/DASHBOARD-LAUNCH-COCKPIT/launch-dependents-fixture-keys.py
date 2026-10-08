import json,re,sys,os
R="/Users/ueli/Documents/semio/"
lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
G=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/🗑️generated/launch-dependents/"
scan=json.load(open(G+"scan.json",encoding="utf-8"))["results"]
def walk(o,path=""):
    if isinstance(o,dict):
        for k,v in o.items():
            p=f"{path}.{k}" if path else k
            yield (p,v)
            yield from walk(v,p)
    elif isinstance(o,list):
        for i,v in enumerate(o):
            yield from walk(v,f"{path}[{i}]")
strongk=["launch.json","launch.seed","claude-launch","launchSeed","LAUNCH_OUTPUT_REL_PATH","mod:🚀️launch","devLaunchers","launch-wording"]
for f in sorted(scan):
    if not any(k in scan[f] for k in strongk): continue
    if not f.endswith(".json") or "🧫️fixtures" not in f: continue
    try: d=json.load(open(R+f,encoding="utf-8"))
    except Exception as e: print("##",f,"UNPARSEABLE",e); continue
    hits=[]
    for p,v in walk(d):
        if re.search(r"(?i)launch|vscode",p) or (isinstance(v,str) and re.search(r"launch\.json|launch\.seed|🧩️launch|\.vscode",v)):
            if not isinstance(v,(dict,list)) : hits.append((p,str(v)[:100]))
            elif re.search(r"(?i)launch",p): hits.append((p,"<"+type(v).__name__+">"))
    print("##",f.replace(lib,"LIB/"))
    for h in hits[:12]: print("    ",h)
