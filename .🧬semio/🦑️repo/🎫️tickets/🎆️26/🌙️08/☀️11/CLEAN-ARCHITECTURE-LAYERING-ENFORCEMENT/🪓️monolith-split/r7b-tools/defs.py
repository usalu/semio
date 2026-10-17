# 📍️ Prints where each name is defined inside an artifact: usage `python3 defs.py <artifact dir> <names file>`.
import re,sys,os
root=sys.argv[1]; names=[l.strip() for l in open(sys.argv[2]) if l.strip()]
files=[]
for d,ds,fs in os.walk(root):
    ds[:]=[x for x in ds if x!='node_modules']
    for f in fs:
        if f.endswith('.rs'): files.append(os.path.join(d,f))
texts={p:open(p,encoding='utf8').read() for p in files}
for n in names:
    hits=[]
    for p,t in texts.items():
        for m in re.finditer(r'^[ \t]*(?:pub(?:\([a-z: ]*\))?[ \t]+)?(?:async[ \t]+)?(?:unsafe[ \t]+)?(fn|struct|enum|trait|const|type|static|mod|macro_rules!)[ \t]+'+re.escape(n)+r'\b',t,re.M):
            hits.append(os.path.relpath(p,root).replace('🏅️standards/🔖️1/🪆️subsets/✳️any/','~/')+':'+m.group(1))
    print(n,'=>',', '.join(hits[:3]))
