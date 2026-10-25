import sys,re
R="/Users/ueli/Documents/semio"
O=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d/orig"
n,pat=sys.argv[1],sys.argv[2]
f=[l.split(" ",1)[1].strip() for l in open(O+"/map.txt",encoding="utf8") if l.startswith(n+" ")][0]
a=open(f"{O}/{n}.bak",encoding="utf8").read().split("\n");b=open(R+"/"+f,encoding="utf8").read().split("\n")
for name,L in (("orig",a),("new",b)):
    for i,l in enumerate(L):
        for m in re.finditer(pat,l):
            print(name,i+1,"...",l[max(0,m.start()-150):m.end()+200]);
