import os,re,sys,subprocess
R="/Users/ueli/Documents/semio"
FX="🧫️fixtures"
files=subprocess.run(["git","grep","-l","--untracked","-I",FX,"--","✏️s/🔌️plugins/🏗️fem"],cwd=R,capture_output=True,text=True).stdout.splitlines()
lit=re.compile(r'''["'`]([^"'`\n]*'''+FX+r'''[^"'`\n]*)["'`]''')
miss=[];total=0
for f in files:
    fa=R+"/"+f
    if "/node_modules/" in fa or "/target/" in fa or not fa.endswith((".rs",".ts",".tsx",".json",".feature",".toml")): continue
    if "/"+FX+"/" in fa and fa.endswith(".json") and "🧪️tests/"+FX not in fa: pass
    txt=open(fa,encoding="utf-8",errors="ignore").read()
    for m in lit.finditer(txt):
        s=m.group(1)
        if "://" in s or "*" in s or "${" in s or "{" in s: 
            continue
        total+=1
        base=os.path.dirname(fa)
        cands=[os.path.normpath(os.path.join(base,s))]
        if s.startswith("/"): cands.append(R+s)
        if not any(os.path.exists(c) for c in cands):
            miss.append((f.replace("🏅️standards/🔖️1/🪆️subsets/","S/"),s))
print("checked",total,"missing",len(miss))
import collections
c=collections.Counter(m[0] for m in miss)
for k,v in c.most_common(60): print(v,k)
open(sys.argv[1]+"/missing.txt","w",encoding="utf-8").write("\n".join(a+"\t"+b for a,b in miss))
