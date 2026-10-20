# 🔧 Adds the missing emoji presentation selector (U+FE0F) to two directory names and re-points their bounded text references. usage: python3 emoji-presentation.py [--apply]
import os,re,subprocess,sys
JOBS=[("✏️s/🔌️plugins/📕️norm","🏗compliant","🏗️compliant"),("✏️s/🔌️plugins/🀄️wfc","🌡fill","🌡️fill")]
apply="--apply" in sys.argv
for plugin,old,new in JOBS:
    fs=[f for f in subprocess.run(["git","ls-files","-co","--exclude-standard","--",plugin],capture_output=True,text=True).stdout.split("\n") if f and os.path.isfile(f) and "node_modules" not in f and "/target/" not in f]
    pat=re.compile(r'(?<![^/"\s(])'+re.escape(old)+r'(?=[/"\s)]|$)')
    dirs=sorted({"/".join(f.split("/")[:i+1]) for f in fs for i,s in enumerate(f.split("/")[:-1]) if s==old},key=lambda d:-d.count("/"))
    edits={}
    for f in fs:
        if not f.endswith((".rs",".ts",".json",".md",".feature",".toml",".py")): continue
        try: t=open(f,encoding="utf8").read()
        except Exception: continue
        n=pat.sub(new,t)
        if n!=t: edits[f]=n
    print(plugin[-20:],"dirs",len(dirs),"files",len(edits))
    for d in dirs: assert not os.path.exists(os.path.join(os.path.dirname(d),new)),d
    if apply:
        for f,t in edits.items(): open(f,"w",encoding="utf8").write(t)
        for d in dirs: os.rename(d,os.path.join(os.path.dirname(d),new))
