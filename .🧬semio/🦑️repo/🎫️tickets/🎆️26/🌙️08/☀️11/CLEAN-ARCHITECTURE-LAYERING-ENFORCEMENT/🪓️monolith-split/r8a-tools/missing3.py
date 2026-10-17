import sys,re,os
S=sys.argv[1]
files={}
for root,_,fs in os.walk(S):
    if '/📦️packages' in root or 'node_modules' in root or '/target' in root: continue
    for f in fs:
        if f=='🦀️.rs':
            p=os.path.join(root,f); files[p]=re.sub(r'//[^\n]*','',open(p,encoding='utf8').read())
ITEM=re.compile(r'\b(?:fn|const|static|struct|enum|trait|type|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)')
def strip(t):
    t=re.sub(r'"(?:[^"\\]|\\.)*"','""',t)
    return t
alldef={}
for p,t in files.items():
    for n in ITEM.findall(t): alldef.setdefault(n,[]).append(p)
USE=re.compile(r'\buse\b([^;]*);',re.S)
def leaves(stmt):
    stmt=re.sub(r'\bas\s+([A-Za-z_0-9]+)',r', \1',stmt)
    return set(re.findall(r'([A-Za-z_][A-Za-z0-9_]*)\s*(?=[,}]|$)',stmt))
for p,t in sorted(files.items()):
    if re.search(r'use\s+(?:super|crate)::\*\s*;|::\*\s*;',t): continue
    t2=strip(t)
    have=set(ITEM.findall(t2))
    for m in USE.finditer(t2): have|=leaves(m.group(1).strip())
    have|=set(re.findall(r'\bmod\s+([A-Za-z_0-9]+)',t2))
    cand=set()
    for m in re.finditer(r'(?<![\w:.\'])([a-z_][a-z0-9_]*)\s*\(',t2): cand.add(m.group(1))
    for m in re.finditer(r'(?<![\w:.\'])([A-Z][A-Z0-9_]{2,})\b',t2): cand.add(m.group(1))
    for m in re.finditer(r'(?<![\w:.\'])([A-Z][A-Za-z0-9]+)\b(?=\s*(?:::|\{|<|\s*[,)>;=]|\s*\())',t2): cand.add(m.group(1))
    # locals: let/fn params/closure params named same
    locals_=set(re.findall(r'\blet\s+(?:mut\s+)?([a-z_][a-z0-9_]*)',t2))|set(re.findall(r'\|([^|]*)\|',t2) and [])
    miss=sorted(n for n in cand if n in alldef and n not in have and n not in locals_ and not any(p==q for q in alldef[n]) )
    # filter out generic params / local generics / enum variants of in-file enums
    if miss: print(p.split('any/')[-1],'::',' '.join(miss))
