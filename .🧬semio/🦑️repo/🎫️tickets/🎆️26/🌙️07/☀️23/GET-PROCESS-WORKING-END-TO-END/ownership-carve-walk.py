import re,os,sys
seen={}
def walk(f):
    f=os.path.normpath(f)
    if f in seen or not os.path.exists(f): 
        if not os.path.exists(f): print("MISSING",f)
        return
    s=open(f,encoding='utf8').read(); seen[f]=len(s.encode())
    for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:pub\s*)?(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s*)?mod\s+(\w+)\s*;',s):
        # path relative to dir of f (for non-mod-rs files, rust uses dir of file)
        walk(os.path.join(os.path.dirname(f),m.group(1)))
    for m in re.finditer(r'^\s*(?:pub\s+)?mod\s+(\w+)\s*;',s,re.M):
        pre=s[max(0,m.start()-200):m.start()]
        if not re.search(r'#\[path[^\]]*\]\s*$',pre): print("PLAINMOD",f,m.group(1))
walk(sys.argv[1])
t=0
for k,v in sorted(seen.items(),key=lambda x:-x[1]): print(v,k); t+=v
print(len(seen),t)
