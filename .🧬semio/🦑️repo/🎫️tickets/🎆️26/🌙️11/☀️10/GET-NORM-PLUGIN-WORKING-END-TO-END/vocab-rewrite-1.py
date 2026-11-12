import re,sys
subs=[(r"`NoMutation` (?:was|is) dropped: the ","The "),(r"`NoMutation` (?:was|is) dropped: ","")]
for l in open(sys.argv[1],encoding="utf-8"):
    f=l.rstrip("\n").split("\t")[1]
    t=open("/home/deck/semio/"+f,encoding="utf-8").read();o=t
    for a,b in subs:t=re.sub(a,b,t)
    if t!=o:open("/home/deck/semio/"+f,"w",encoding="utf-8").write(t)
