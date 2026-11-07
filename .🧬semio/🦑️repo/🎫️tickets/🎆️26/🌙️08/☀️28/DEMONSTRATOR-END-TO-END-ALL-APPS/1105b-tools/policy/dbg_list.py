import subprocess,os,re,time
files=subprocess.run(["git","grep","-lF","[DEBUG]","--","🧰️framework","✏️s","🌎️hub","♻️mit-bestand"],capture_output=True,text=True).stdout.split("\n")
for f in files:
    if not f: continue
    m=time.strftime("%m-%d %H:%M",time.localtime(os.stat(f).st_mtime))
    t=open(f,encoding="utf-8").read()
    for i,l in enumerate(t.split("\n"),1):
        for x in re.finditer(r"\[DEBUG\]",l):
            print(m,f[-40:],i,len(l),repr(l[max(0,x.start()-45):x.end()+40]))
