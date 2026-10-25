import sys,os,shutil
R="/Users/ueli/Documents/semio"
C=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O=C+"/🗑️generated/dbg2-d/orig"
mp=O+"/map.txt"
have={}
if os.path.exists(mp):
    for l in open(mp,encoding="utf8"):
        l=l.rstrip("\n")
        if l:
            n,f=l.split(" ",1);have[f]=int(n)
nxt=max(have.values(),default=0)
for f in sys.argv[1:]:
    f=f.strip()
    if not f or f in have:continue
    nxt+=1
    shutil.copy2(R+"/"+f,f"{O}/{nxt}.bak")
    have[f]=nxt
    open(mp,"a",encoding="utf8").write(f"{nxt} {f}\n")
print(len(have))
