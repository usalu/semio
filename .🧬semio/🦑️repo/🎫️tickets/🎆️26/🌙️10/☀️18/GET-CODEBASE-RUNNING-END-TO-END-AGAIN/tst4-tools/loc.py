import sys,glob,re
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
logs=glob.glob(C+"/🗑️generated/tst4-out/logs/*.log")
W=int(sys.argv[1])
for k in sys.argv[2:]:
    toks=[t for t in re.split(r"[^A-Za-z0-9]+",k) if t]
    def ok(f):
        name=f.rsplit("/",1)[1];pos=0
        for t in toks:
            i=name.find(t,pos)
            if i<0:return False
            pos=i+len(t)
        return True
    fs=[f for f in logs if ok(f)]
    print("=====",k,len(fs))
    if not fs: continue
    t=open(fs[0],encoding="utf-8",errors="replace").read()
    seen=0
    for m in re.finditer(r"at <anonymous> \((/[^)]+\.tsx?):(\d+):(\d+)\)",t):
        if "node_modules" in m.group(1): continue
        try:
            line=open(m.group(1),encoding="utf-8").read().split("\n")[int(m.group(2))-1]
        except Exception as e: print("?",e);continue
        c=int(m.group(3))
        print("@%s:%s:%s"%(m.group(1).rsplit("/",2)[-2][:20],m.group(2),m.group(3)),line[max(0,c-W):c+80])
        seen+=1
        if seen>=2:break
