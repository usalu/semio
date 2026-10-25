import json,sys,re,os
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
d=json.load(open(C+"/🗑️generated/rcore-b/census.json"))
name=sys.argv[1]; lim=int(sys.argv[2]) if len(sys.argv)>2 else 12; kind=sys.argv[3] if len(sys.argv)>3 else None
for k,v in d.items():
    u,kd=k.split("|")
    if u!=name or (kind and kd!=kind): continue
    seen=set();n=0
    for f,a,msg in v:
        p=os.path.normpath(f)
        if (p,a,msg) in seen: continue
        seen.add((p,a,msg)); n+=1
        if n>lim: continue
        try: line=open(p).read().split("\n")[int(a)-1].strip()[:230]
        except Exception as e: line="?"
        print(p.split("🔨️modules/")[-1][-90:],a,"|",msg[:200]); print("     ",line)
    print("--",k,n)
