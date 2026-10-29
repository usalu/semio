import re,sys,subprocess
sys.path.insert(0,__file__.rsplit('/',1)[0])
dry=sys.argv[1]=='dry'; roots=sys.argv[2:]
def split_top_commas(body):
    d=0
    for i,c in enumerate(body):
        if c in '<([': d+=1
        elif c in '>)]' and not (c=='>' and body[i-1:i]=='-'): d-=1
        elif c==',' and d==0: return True
    return False
def key(t): return re.sub(r'<.*$','',re.sub(r'^.*::','',t.strip()))
def split_top(body):
    out=[];d=0;cur=''
    for i,c in enumerate(body):
        if c in '<([': d+=1
        if c in '>)]' and not (c=='>' and body[i-1:i]=='-'): d-=1
        if c=='+' and d==0: out.append(cur.strip()); cur=''
        else: cur+=c
    out.append(cur.strip()); return [t for t in out if t]
STATE=re.compile(r'^(P|S|[A-Z][A-Za-z]*P|Snapshot|State)$')
MUT=re.compile(r'^(M|Mu|Mutation|[A-Z][A-Za-z]*Mutation)$')
REDUND_P={'Clone','ToValue','ValueToValue','FromValue','ValueFromValue','Send','Sync',"'static"}
REDUND_M={'Clone','ToValue','ValueToValue','FromValue','ValueFromValue','Mutation','ColdMutation','OpBinary','OpText','Send',"'static"}
def aug(name,body,pref):
    toks=split_top(body); ks={key(t) for t in toks}
    if STATE.match(name) and ('StoreState' not in ks) and ({'ToValue','ValueToValue','ArtifactPack','ArtifactStoreSnapshot','RetireOwned'}&ks):
        rest=[t for t in toks if key(t) not in REDUND_P]
        return ' + '.join(rest+[pref+'StoreState'])
    if MUT.match(name) and 'StoreMutationFor' not in ks:
        mt=[t for t in toks if key(t) in ('Mutation','ColdMutation') and '<' in t]
        if mt:
            mx=re.search(r'<(.*)>$',mt[0])
            if not mx: return None
            X=mx.group(1)
            rest=[t for t in toks if key(t) not in REDUND_M]
            return ' + '.join(rest+[f'{pref}StoreMutationFor<{X}>'])
    return None
def prefix_of(src):
    m=re.search(r'((?:::)?(?:[A-Za-z_0-9]+::)+)(?:ColdMutation|StoreState)',src)
    return m.group(1) if m else '::semio_framework_os_kernel::'
CLAUSE=re.compile(r'^(\s*)((?:where )?)([A-Za-z_][A-Za-z_0-9]*)(\s*):(?!:)\s*(.+?)(,?)\s*$')
def inline(l,pref,start_pat):
    changed=False
    for m in list(re.finditer(start_pat,l))[::-1]:
        i=m.end(); d=0; j=i; parts=[]; cur=i
        opener = l[i-1]
        while j<len(l):
            c=l[j]
            if c in '<([': d+=1
            elif c in ')]' : d-=1
            elif c=='>' and l[j-1:j]!='-':
                if d==0 and opener=='<': break
                d-=1
            if d==0 and c==',': parts.append((cur,j)); cur=j+1
            if d==0 and c in '{;' and opener!='<': break
            j+=1
        parts.append((cur,j)); res=[]; ch=False
        for a,b in parts:
            seg=l[a:b]; mm=re.match(r'^(\s*)([A-Za-z_][A-Za-z_0-9]*)(\s*):(?!:)\s*(.+?)(\s*)$',seg)
            if mm:
                r=aug(mm.group(2),mm.group(4),pref)
                if r: seg=f"{mm.group(1)}{mm.group(2)}{mm.group(3)}: {r}{mm.group(5)}"; ch=True
            res.append(seg)
        if ch: l=l[:i]+','.join(res)+l[j:]; changed=True
    return l,changed
def fix(src):
    pref=prefix_of(src); lines=src.split('\n'); ch=False
    for k,l in enumerate(lines):
        if re.match(r'^\s*(pub\s+)?(unsafe\s+)?(async\s+)?(fn|impl|struct|enum|trait)\b',l) or 'where' in l:
            nl,c=inline(l,pref,r'\b(?:fn|impl|struct|enum|trait)\s*(?:[A-Za-z_0-9]+)?<|\bwhere\b')
            if c: lines[k]=nl; ch=True; l=nl
        m=CLAUSE.match(l)
        if m and not ('where' in l and not l.strip().startswith('where')):
            ind,w,name,sp,body,comma=m.groups()
            if re.search(r'[{};]',body) or split_top_commas(body): continue
            r=aug(name,body,pref)
            if r: lines[k]=f"{ind}{w}{name}{sp}: {r}{comma}"; ch=True
    return '\n'.join(lines),ch
files=[f for f in subprocess.run(['rg','--files','-g','*.rs']+roots,capture_output=True,text=True).stdout.split('\n') if f and '/🧪️' not in f]
n=0
for p in files:
    s=open(p).read()
    if 'ToValue' not in s and 'ArtifactPack' not in s and 'Mutation<' not in s: continue
    t,c=fix(s)
    if c:
        n+=1
        if not dry: open(p,'w').write(t)
print('files',n)
