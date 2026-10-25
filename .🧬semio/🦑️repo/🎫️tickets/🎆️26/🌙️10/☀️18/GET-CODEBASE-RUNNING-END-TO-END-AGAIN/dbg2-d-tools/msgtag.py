import sys,os,time
R="/Users/ueli/Documents/semio"
for f in sys.stdin.read().split("\n"):
    f=f.strip()
    if not f:continue
    p=R+"/"+f
    if time.time()-os.stat(p).st_mtime<1800 and "--force" not in sys.argv:
        print("LIVE-SKIP",f);continue
    t=open(p,encoding="utf8").read()
    n=t.count("[DEBUG] ")
    open(p,"w",encoding="utf8").write(t.replace("[DEBUG] ",""))
    print(n,f)
