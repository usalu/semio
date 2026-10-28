import os,re,subprocess,sys
root="/Users/ueli/Documents/semio/"
files=subprocess.run(["git","status","--short","--","🧰️framework",".github"],capture_output=True,text=True,cwd=root).stdout.splitlines()
imp=re.compile(r'(?:from|import)\s*["\'](\.[^"\']+)["\']')
bad=0
for line in files:
    path=line[3:].strip().strip('"')
    if not path.endswith((".ts",".mjs")) or not os.path.exists(root+path): continue
    t=open(root+path,encoding="utf8").read()
    for m in imp.finditer(t):
        spec=m.group(1); target=os.path.normpath(os.path.join(os.path.dirname(root+path),spec))
        if not os.path.exists(target) and not os.path.exists(target+".ts"):
            print("MISSING",path[-90:],spec[-90:]); bad+=1
print("checked",len(files),"bad",bad)
