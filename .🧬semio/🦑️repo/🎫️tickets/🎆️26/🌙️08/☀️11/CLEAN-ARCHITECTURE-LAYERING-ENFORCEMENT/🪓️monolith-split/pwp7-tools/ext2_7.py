import sys; sys.path.insert(0,'.')
from ext7 import *
from src7 import methods
ms=methods()
names={}
for m in ms:
    if m['crate'] in ('app','composer-api'): continue
    names[m['name']]=m
files=plugin_files()
fam=set(PLUGREL+'/'+r for r in TIER2_FILES)
# also exclude my out files
pat=re.compile(r'([\w\.\[\]\(\)&\*]*?)\s*\.\s*('+'|'.join(sorted(map(re.escape,names),key=len,reverse=True))+r')\b\s*(?:::<[^>]*>)?\s*\(')
rows=collections.Counter()
for f in files:
    if f in fam or '/📥️staging/' in f or '/🧺️bundle' in f: continue
    if not os.path.exists(ROOT+f): continue
    t=open(ROOT+f,encoding='utf-8').read(); mk=mask(t)
    for mm in pat.finditer(mk):
        rows[(f.replace(PLUGREL+'/',''),mm.group(1).strip()[-30:],mm.group(2))]+=1
recv=collections.Counter()
for (f,r,n),c in rows.items(): recv[(r)]+=c
for r,c in recv.most_common(60): print(c,repr(r))
print(len(rows))
print('-----')
for (f,r,n),c in sorted(rows.items()):
    if r!='app': print(c,f.split('/')[-2] if f.count('/')>1 else f,'|',r,'|',n)
