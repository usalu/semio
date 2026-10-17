import sys,re,os
S=sys.argv[1]
files={}
for root,_,fs in os.walk(S):
    if '/📦️packages' in root or 'node_modules' in root or '/target' in root: continue
    for f in fs:
        if f=='🦀️.rs':
            p=os.path.join(root,f); files[p]=re.sub(r'//[^\n]*','',open(p,encoding='utf8').read())
ITEM=re.compile(r'\b(?:fn|const|static|struct|enum|trait|type|mod|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)')
USE=re.compile(r'\buse\b([^;]*);',re.S)
def leaves(stmt):
    stmt=re.sub(r'\bas\s+([A-Za-z_0-9]+)',r', \1',stmt)
    out=set()
    for m in re.finditer(r'([A-Za-z_][A-Za-z0-9_]*)\s*(?=[,}]|$)',stmt): out.add(m.group(1))
    return out
decl={p:set(ITEM.findall(t)) for p,t in files.items()}
imp={}
for p,t in files.items():
    s=set()
    for m in USE.finditer(t): s|=leaves(m.group(1).strip())
    imp[p]=s
allitems=set().union(*decl.values())
def host(p):
    d=os.path.dirname(p)
    cands=[]
    # test file: <host>/🧪️tests/<x>/🦀️.rs or nested unit parts
    parts=d.split('/')
    for i in range(len(parts)-1,0,-1):
        if parts[i-1].startswith('🧪️tests') or parts[i]==('🧪️tests'):
            pass
    up=os.path.dirname(d)
    h=os.path.join(up,'🦀️.rs')
    if os.path.basename(up)=='🧪️tests': h=os.path.join(os.path.dirname(up),'🦀️.rs')
    return h if h in files else None
memo={}
def ns(p,depth=0):
    if p in memo: return memo[p]
    memo[p]=set()
    s=set(decl[p])|imp[p]
    if re.search(r'use\s+super::\*\s*;',files[p]):
        h=host(p)
        if h and depth<8: s|=ns(h,depth+1)
    memo[p]=s; return s
skipw=set('self Self crate super true false None Some Ok Err Box Vec String Option Result Default'.split())
for p,t in files.items():
    if not re.search(r'use\s+super::\*\s*;',t): continue
    h=host(p)
    if not h: continue
    have=ns(p)
    used=set(re.findall(r'(?<![\w:.\'"])([A-Za-z_][A-Za-z0-9_]*)(?=\s*(?:\(|::|!|\{|<|\s*:|;|,|\)|\}|\]))',re.sub(r'"(?:[^"\\]|\\.)*"','""',t)))
    miss=sorted(n for n in used if n in allitems and n not in have and n not in skipw)
    if miss: print(p.split('any/')[-1],'::',' '.join(miss))
