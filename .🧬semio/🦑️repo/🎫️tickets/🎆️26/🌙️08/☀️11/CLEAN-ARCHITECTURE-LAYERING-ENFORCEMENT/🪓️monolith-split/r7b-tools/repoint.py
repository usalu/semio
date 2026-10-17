# 🧭️ Re-points `<prefix>::NAME` paths and `use <prefix>::{..}` groups to the owner paths of a name map: usage `python3 repoint.py <map.json> <prefix> <file>... [--dry]`; prefix is `crate` or an external crate name. Values starting with `@` are absolute paths without the prefix.
import json,re,sys,os
args=[a for a in sys.argv[1:] if not a.startswith('--')]
dry='--dry' in sys.argv
MAP=json.load(open(args[0])); PREFIX=args[1]; FILES=[]
for a in args[2:]:
    if a.startswith('@'): FILES+= [l.strip() for l in open(a[1:],encoding='utf8') if l.strip()]
    else: FILES.append(a)
IDENT=re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
def full(prefix,name,rest):
    new=MAP[name]
    if new.startswith('@'): return new[1:]+rest
    return prefix+'::'+new+rest
def split_top(body):
    items=[];depth=0;cur=''
    for ch in body:
        if ch in '{(<[': depth+=1
        if ch in '})>]': depth-=1
        if ch==',' and depth==0:
            items.append(cur.strip());cur=''
        else: cur+=ch
    if cur.strip(): items.append(cur.strip())
    return items
def find_group_end(t,i):
    depth=1
    while depth:
        c=t[i]
        if c=='{':depth+=1
        elif c=='}':depth-=1
        i+=1
    return i
def head_of(item):
    item=re.sub(r'^\s*(//[^\n]*\n\s*)*','',item)
    m=IDENT.match(item)
    return (m.group(0) if m else None, item)
STMT=re.compile(r'((?:pub(?:\([^)]*\))?\s+)?use\s+)'+re.escape(PREFIX)+r'::\{')
def rewrite_groups(t,warn):
    out='';pos=0
    for m in STMT.finditer(t):
        if m.start()<pos: continue
        start=m.start(); gs=m.end(); ge=find_group_end(t,gs)
        semi=t.index(';',ge)
        if t[ge:semi].strip(): warn(f'complex tail after group at {start}'); continue
        body=t[gs:ge-1]; items=split_top(body)
        keep=[];moved={}
        changed=False
        for it in items:
            h,it2=head_of(it)
            if h in MAP:
                changed=True
                p=full(PREFIX,h,it2[len(h):])
                simple=re.fullmatch(r'(.*)::([A-Za-z_][A-Za-z0-9_]*)(\s+as\s+[A-Za-z_][A-Za-z0-9_]*)?',p)
                if simple and '{' not in p:
                    moved.setdefault(simple.group(1),[]).append(simple.group(2)+(simple.group(3) or ''))
                else: moved.setdefault(p,None)
            else: keep.append(it)
        if not changed: continue
        stmts=[]
        if keep: stmts.append(f'{m.group(1)}{PREFIX}::{{{", ".join(keep)}}};')
        for k,v in moved.items():
            if v is None: stmts.append(f'{m.group(1)}{k};')
            elif len(v)==1: stmts.append(f'{m.group(1)}{k}::{v[0]};')
            else: stmts.append(f'{m.group(1)}{k}::{{{", ".join(v)}}};')
        out+=t[pos:start]+'\n'.join(stmts); pos=semi+1
    return out+t[pos:]
PATH=re.compile(r'(?<![A-Za-z0-9_])(?<!::)'+re.escape(PREFIX)+r'::([A-Za-z_][A-Za-z0-9_]*)')
def rewrite_paths(t):
    def sub(m):
        n=m.group(1)
        if n not in MAP: return m.group(0)
        return full(PREFIX,n,'')
    return PATH.sub(sub,t)
changed_files=0
for f in FILES:
    t=open(f,encoding='utf8').read(); warns=[]
    n=rewrite_groups(t,warns.append); n=rewrite_paths(n)
    if n!=t:
        changed_files+=1
        if not dry: open(f,'w',encoding='utf8').write(n)
    for w in warns: print('WARN',f,w)
print('changed',changed_files,'of',len(FILES))
