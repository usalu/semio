import os,re,sys
bad=0
for f in sys.argv[1:]:
    s=open(f,encoding="utf8").read()
    for m in re.finditer(r'''(?:from|import)\s*\(?\s*["'](\.{1,2}/[^"']+)["']''',s):
        t=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
        if not os.path.exists(t): print("MISSING",f.split("/")[-2],m.group(1)); bad+=1
print("bad",bad)
