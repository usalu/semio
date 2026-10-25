import re,sys,os,collections
R="/Users/ueli/Documents/semio"
C=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
D=C+"/🗑️generated/dbg2-d";O=D+"/orig"
TOK=re.compile(r'''//[^\n]*|"(?:\\.|[^"\\])*"|[A-Za-z_][A-Za-z0-9_]*''',re.S)
def idents(t):
    c=collections.Counter()
    for x in TOK.findall(t):
        if x.startswith("//"):continue
        if x.startswith('"'):
            for y in re.findall(r"\{([A-Za-z_]\w*)",x):c[y]+=1
        else:c[x]+=1
    return c
files={}
for l in open(O+"/map.txt",encoding="utf8"):
    n,f=l.rstrip("\n").split(" ",1)
    if not f.endswith(".rs"):continue
    a=open(f"{O}/{n}.bak",encoding="utf8").read();b=open(R+"/"+f,encoding="utf8").read()
    files[f]=(idents(a)-idents(b),b.split("\n"))
rx=re.compile(r"^(warning|error)(\[[^\]]+\])?: (.*)$")
loc=re.compile(r"^\s*--> (.*?):(\d+):(\d+)")
KIND=re.compile(r"unused|never used|never read|does not need to be mutable|assigned to|never constructed|dead_code|is never|unreachable|`Result`")
for logf,base in [(sys.argv[i],sys.argv[i+1]) for i in range(1,len(sys.argv),2)]:
    L=open(logf,encoding="utf8",errors="replace").read().split("\n")
    seen=set()
    for i,l in enumerate(L):
        m=rx.match(l)
        if not m or not KIND.search(m.group(3)) or "generated" in m.group(3):continue
        mm=None
        for k in range(i+1,min(i+3,len(L))):
            mm=loc.match(L[k])
            if mm:break
        if not mm:continue
        p=os.path.normpath(base+"/"+mm.group(1));ln=int(mm.group(2))
        rel=p[len(R)+1:] if p.startswith(R) else p
        if rel not in files:continue
        rem,lines=files[rel]
        names=re.findall(r"`([A-Za-z_][A-Za-z0-9_]*)`",m.group(3))+re.findall(r"\b([A-Za-z_][A-Za-z0-9_]*)\b",m.group(3).split(":")[-1])
        hit=[x for x in names if rem.get(x,0)>0 and x not in ("unused","variable","value","assigned","never","read","used")]
        key=(rel,ln,m.group(3))
        if key in seen:continue
        seen.add(key)
        print("HIT" if hit else "   ",m.group(1),m.group(3)[:90],"|",rel.split("/")[-4:],ln,hit[:3])
