import json,sys,re
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
d=json.load(open(C+"/🗑️generated/rcore-b/census.json"))
name=sys.argv[1]; lim=int(sys.argv[2]) if len(sys.argv)>2 else 40
for k,v in d.items():
    if k.split("|")[0]==name and (len(sys.argv)<4 or k.split("|")[1]==sys.argv[3]):
        seen=set()
        for f,a,msg in v:
            if (f,a,msg) in seen: continue
            seen.add((f,a,msg))
            if len(seen)<=lim: print(re.sub(r".*📦️packages/🦀️rust/","",f)[-100:],a,msg[:260])
        print("--",k,len(seen))
