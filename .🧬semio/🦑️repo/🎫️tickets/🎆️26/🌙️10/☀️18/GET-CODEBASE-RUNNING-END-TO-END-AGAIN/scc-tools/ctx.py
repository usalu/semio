import re,sys
pat=re.compile(sys.argv[1]); w=int(sys.argv[2])
for f in sys.argv[3:]:
    print("===",f)
    t=open(f,encoding="utf8").read()
    for m in list(pat.finditer(t))[:12]:
        s=max(0,m.start()-w);e=min(len(t),m.end()+w)
        print("  @",t.count("\n",0,m.start())+1,":",t[s:e].replace("\n","⏎"))
