import os,re,sys,collections
roots=["🧰️framework","✏️s"]
skip=("node_modules","/target","🧪️","/tests/","🏭️bridge","👽️guest","🔮️oracles",".cache")
files=[]
for r in roots:
    for d,ds,fs in os.walk(r):
        ds[:]=[x for x in ds if x not in("node_modules","target",".cache")]
        if any(k in d for k in skip): continue
        for f in fs:
            if f.endswith(".rs"): files.append(os.path.join(d,f))
derive_re=re.compile(r'#\[derive\(([^\]]*)\)\]')
item_re=re.compile(r'(?:pub(?:\([^)]*\))?\s+)?(struct|enum)\s+([A-Za-z0-9_]+)')
def crate_of(p):
    parts=p.split(os.sep)
    # crate = path up to the 📦️packages or the artifact root: use first 6 dirs after root for grouping
    return "/".join(parts[:min(len(parts),8)])
def balanced(s,i,open_,close):
    depth=0
    for j in range(i,len(s)):
        if s[j]==open_: depth+=1
        elif s[j]==close:
            depth-=1
            if depth==0: return j
    return -1
types=set();derived={};recv_impl=set();aliases={}
for p in files:
    try: s=open(p,encoding="utf-8").read()
    except: continue
    for m in re.finditer(r'impl(?:<[^>]*>)?\s*(?:::)?[A-Za-z_:]*DslOwnedReceivingField\s+for\s*\[?([A-Za-z0-9_]+)',s): recv_impl.add(m.group(1))
    for m in re.finditer(r'dsl_transparent_receiving!\(\s*([A-Za-z0-9_]+)',s): recv_impl.add(m.group(1))
    for m in re.finditer(r'(?m)^\s*(?:pub\s+)?type\s+([A-Za-z0-9_]+)\s*=\s*(\[[^;]*;[^\]]*\])\s*;',s): aliases[m.group(1)]=m.group(2)
    for m in re.finditer(r'dsl_scalar_newtype_receiving!\(\s*([A-Za-z0-9_]+)',s): recv_impl.add(m.group(1))
    for m in re.finditer(r'impl(?:<[^>]*>)?\s*(?:::)?[A-Za-z_:]*DslReceivingField\s+for\s+([A-Za-z0-9_]+)',s): recv_impl.add(m.group(1))
    for m in derive_re.finditer(s):
        d=m.group(1)
        if not re.search(r'DslRecord|DslEnum|DslScalar',d): continue
        rest=s[m.end():m.end()+400]
        rest=re.sub(r'(?:\s*#\[[^\]]*\]|\s*///[^\n]*)*','',rest,count=1) if False else rest
        im=item_re.search(rest)
        if not im: continue
        kind,name=im.group(1),im.group(2)
        start=m.end()+im.end()
        # find body opener
        k=start
        while k<len(s) and s[k] not in "{(;": k+=1
        if k>=len(s) or s[k]==";": body=""
        else:
            close={"{":"}","(":")"}[s[k]]
            e=balanced(s,k,s[k],close)
            body=s[k+1:e] if e>0 else ""
        derived[name]=(p,kind,body,'DslScalar' in d and 'DslRecord' not in d and 'DslEnum' not in d)
scalars=set("String bool u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 char SharedUtf8 DslValue".split())
wrappers=set("Option Vec Box PersistentList PagedList Arc Rc".split())
def leaf_types(ty):
    ty=ty.strip()
    out=[]
    m=re.match(r'^([A-Za-z0-9_:]+)\s*<(.*)>$',ty,re.S)
    if m:
        head=m.group(1).split("::")[-1]
        inner=m.group(2)
        # split top-level commas
        parts=[];depth=0;cur=""
        for ch in inner:
            if ch in "<([": depth+=1
            if ch in ">)]": depth-=1
            if ch=="," and depth==0: parts.append(cur);cur=""
            else: cur+=ch
        parts.append(cur)
        if head in wrappers: 
            for q in parts: out+=leaf_types(q)
        else:
            out.append(head+"<…>")
        return out
    if ty.startswith("["):
        am=re.match(r"^\[\s*([A-Za-z0-9_:]+)\s*;",ty)
        return leaf_types(am.group(1)) if am and am.group(1).split("::")[-1] in scalars else ["[array of "+(am.group(1) if am else "?")+"]"]
    if ty.startswith("("): return ["(tuple)"]
    if ty.startswith("&"): return [ty]
    return [ty.split("::")[-1]]
def fields(body):
    # split top-level by commas, skip attrs
    parts=[];depth=0;cur=""
    for ch in body:
        if ch in "<([{": depth+=1
        if ch in ">)]}": depth-=1
        if ch=="," and depth==0: parts.append(cur);cur=""
        else: cur+=ch
    parts.append(cur)
    res=[]
    for q in parts:
        q=re.sub(r'#\[[^\]]*\]','',q,flags=re.S)
        q=re.sub(r'//[^\n]*','',q).strip()
        if not q: continue
        m=re.match(r'^(?:pub(?:\([^)]*\))?\s+)?([A-Za-z0-9_]+)\s*:\s*(.+)$',q,re.S)
        if m: res.append(m.group(2).strip())
        else:
            # variant (maybe with tuple/struct body) or tuple field
            mv=re.match(r'^([A-Za-z0-9_]+)\s*([\(\{].*)$',q,re.S)
            if mv:
                inner=mv.group(2); res+=fields(inner[1:-1])
            elif re.match(r'^[A-Za-z0-9_:<>,\s\[\];&()]+$',q) and not re.match(r'^[A-Z][A-Za-z0-9_]*$',q): res.append(q)
    return res
missing=collections.defaultdict(set)   # type -> {derived type}
bycrate=collections.defaultdict(lambda:collections.defaultdict(set))
unit=0
for name,(p,kind,body,scalar) in derived.items():
    if scalar: continue
    fl=fields(body)
    for ft in fl:
        for lt in leaf_types(ft):
            if lt in aliases and all(x in scalars for x in leaf_types(aliases[lt])): continue
            if lt in scalars or lt in derived or lt.split("<")[0] in recv_impl or lt.split("<")[0] in derived or lt=="Self" or lt==name: continue
            missing[lt].add(name); bycrate[crate_of(p)][lt].add(name)
print("derived types scanned:",len(derived),"files:",len(files),"explicit receiver impls:",sorted(recv_impl))
print("\n== MISSING FIELD TYPES (type: #derived types using it)")
for t,u in sorted(missing.items(),key=lambda x:-len(x[1])): print(f"{t}\t{len(u)}\t{', '.join(sorted(u)[:4])}")
print("\n== BY CRATE (path prefix): missing type -> count")
for c,d in sorted(bycrate.items()):
    print(c, "|", "; ".join(f"{t}({len(u)})" for t,u in sorted(d.items(),key=lambda x:-len(x[1]))[:8]))
