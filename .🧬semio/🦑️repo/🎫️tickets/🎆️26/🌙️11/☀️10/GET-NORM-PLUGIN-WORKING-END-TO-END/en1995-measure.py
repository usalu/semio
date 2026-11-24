import os,re,sys
root=sys.argv[1]
ws=re.findall(r'^\s*"([^"]+)",?\s*$',re.search(r'members = \[(.*?)\]',open(os.path.join(root,'Cargo.toml'),encoding='utf8').read(),re.S).group(1),re.M)
pat=re.compile(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:pub\s+)?mod\s+(\w+)|include_str!\("([^"]+)"\)|include_bytes!\("([^"]+)"\)|\bmod\s+(\w+)\s*;')
def closure(f,seen):
    if f in seen or not os.path.isfile(f): return
    seen.add(f)
    if not f.endswith('.rs'): return
    d=os.path.dirname(f); t=open(f,encoding='utf8').read()
    for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;',t): closure(os.path.normpath(os.path.join(d,m.group(1))),seen)
    for m in re.finditer(r'include_(?:str|bytes)!\("([^"]+)"\)',t): closure(os.path.normpath(os.path.join(d,m.group(1))),seen)
res=[]
for m in ws:
    p=os.path.join(root,m); ct=open(os.path.join(p,'Cargo.toml'),encoding='utf8').read()
    mm=re.search(r'\[lib\][^\[]*?path\s*=\s*"([^"]+)"',ct); lp=mm and mm.group(1)
    nm=re.search(r'^name\s*=\s*"([^"]+)"',ct,re.M).group(1)
    if not lp: continue
    seen=set(); closure(os.path.normpath(os.path.join(p,lp)),seen)
    b=sum(os.path.getsize(f) for f in seen); l=sum(open(f,encoding='utf8',errors='ignore').read().count('\n') for f in seen)
    res.append((b,len(seen),l,nm))
for r in sorted(res)[-12:]: print(*r)
if len(sys.argv)>2:
    for m in ws:
        p=os.path.join(root,m); ct=open(os.path.join(p,'Cargo.toml'),encoding='utf8').read()
        if ('"%s"'%sys.argv[2]) in ct.split('[dependencies]')[0]:
            mm=re.search(r'\[lib\][^\[]*?path\s*=\s*"([^"]+)"',ct); seen=set(); closure(os.path.normpath(os.path.join(p,mm.group(1))),seen)
            for f in sorted(seen,key=lambda f:-os.path.getsize(f)): print(os.path.getsize(f),f.replace(root,''))
