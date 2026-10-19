import sys,glob,subprocess,re
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=sys.argv[1]
logs=glob.glob(C+"/🗑️generated/tst4-out/logs/*.log")
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
    if fs:
        out=subprocess.run(["python3",C+"/tst4-tools/first.py",fs[0],n],capture_output=True,text=True).stdout
        print("\n".join(l[:330] for l in out.split("\n")[:40]))
