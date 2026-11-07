import subprocess,re,sys,os,time
APPLY=len(sys.argv)>1 and sys.argv[1]=="apply"
SKIP=("enforcement/🔌️nx/📦️graph/","custody/🧪️tests/🌊️output/","source-census/")
files=[f for f in subprocess.run(["git","grep","-lF","[DEBUG]","--","🧰️framework","✏️s","🌎️hub","♻️mit-bestand"],capture_output=True,text=True).stdout.split("\n") if f]
CUT=time.mktime(time.strptime("2026-11-05 10:00","%Y-%m-%d %H:%M"))
START=re.compile(r"(eprintln!|println!|console\.(?:log|error)|Bun\.write)\(\s*(?:Bun\.stdout,\s*)?['\"]\[DEBUG\] ")
def stmt_end(t,i):
    d=0;j=t.index("(",i)
    while True:
        c=t[j]
        if c=="(":d+=1
        elif c==")":
            d-=1
            if d==0:break
        j+=1
    j+=1
    if t[j:j+1]==";":j+=1
    return j
for f in files:
    if any(s in f for s in SKIP):continue
    if os.stat(f).st_mtime>=CUT:print("NEWER",f);continue
    t=open(f,encoding="utf-8").read();o=t
    while True:
        m=START.search(t)
        if not m:break
        e=stmt_end(t,m.start());s=m.start()
        pre=t[:s]
        wrap=re.search(r"if index<512\{$",pre)
        if wrap and t[e:e+1]=="}":s=wrap.start();e+=1
        print("DEL",f[-30:],repr(t[s:e][:160]))
        ls=t.rfind("\n",0,s)+1;le=t.find("\n",e)
        if t[ls:s].strip()=="" and t[e:le].strip()=="" and le!=-1:t=t[:ls]+t[le+1:]
        else:t=t[:s]+t[e:]
    for m in re.finditer(r"\[DEBUG\] ",t):
        print("PREFIX",f[-30:],repr(t[max(0,m.start()-30):m.end()+30]))
    t=t.replace("[DEBUG] ","")
    if APPLY and t!=o:open(f,"w",encoding="utf-8").write(t)
