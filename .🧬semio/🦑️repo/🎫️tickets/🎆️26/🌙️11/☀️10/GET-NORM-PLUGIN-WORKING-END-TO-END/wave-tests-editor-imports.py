import re,sys,os,glob
ROOT="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
def uses(src):
    out=[]
    for m in re.finditer(r'^use ([^;]*);',src,re.M|re.S):
        out.append(m.group(1))
    return out
def leaves(body):
    body=re.sub(r'\s+as\s+','\x00',body); body=re.sub(r'\s+','',body).replace('\x00',' as ')
    res=[]
    def walk(prefix,s):
        if '{' not in s:
            res.append((prefix+s)); return
        i=s.index('{'); pre=prefix+s[:i]; inner=s[i+1:s.rindex('}')]
        depth=0;cur=''
        for ch in inner:
            if ch=='{':depth+=1
            if ch=='}':depth-=1
            if ch==',' and depth==0: walk(pre,cur);cur=''
            else: cur+=ch
        if cur: walk(pre,cur)
    walk('',body)
    return res
def ident(path):
    if ' as ' in path: return path.split(' as ')[1]
    return path.split('::')[-1]
for fam in sorted(glob.glob(ROOT+"/*")):
    eds=glob.glob(fam+"/🏅️standards/*/🪆️subsets/*/✏️editor")
    if not eds: continue
    ed=eds[0]
    cargo=open(ed+"/📦️packages/🦀️rust/Cargo.toml",encoding='utf8').read()
    lib=re.search(r'name = "([^"]+)"',cargo).group(1).replace('-','_')
    if not os.path.exists(ed+"/🧪️tests/🧩️integration/🦀️.rs"): continue
    root=open(ed+"/🧪️tests/🧩️integration/🦀️.rs",encoding='utf8').read()
    files=[os.path.normpath(os.path.join(ed,"🧪️tests/🧩️integration",p)) for p in re.findall(r'#\[path = "([^"]+)"\]',root)]
    files=[f for f in files if os.path.exists(f)]
    for f in files:
        s=open(f,encoding='utf8').read()
        node=os.path.dirname(os.path.dirname(os.path.dirname(f)))
        mods=[node+"/🦀️.rs"]
        if os.path.normpath(f)==os.path.normpath(ed+"/🧪️tests/🔬️unit/🦀️.rs"): mods=[ed+"/🦀️.rs"]
        have=set(ident(l) for u in uses(s) for l in leaves(u))
        body=re.sub(r'^use [^;]*;','',s,flags=re.M|re.S)
        add=[]
        for m in mods:
            if not os.path.exists(m): continue
            ms=open(m,encoding='utf8').read()
            for u in uses(ms):
                for l in leaves(u):
                    if l.endswith('*') or l.startswith('super::') or l.startswith('self::'): continue
                    i=ident(l)
                    if i in have or i=='_': continue
                    if not re.search(r'(?<![A-Za-z_0-9:])'+re.escape(i)+r'\b',body): continue
                    if re.search(r'\b(fn|struct|enum|const|static|type|mod|trait)\s+'+re.escape(i)+r'\b',body): continue
                    p=l.replace('crate::',lib+'::',1) if l.startswith('crate::') else l
                    add.append("use "+p+";"); have.add(i)
        if add:
            # insert after the last leading `use` line
            lines=s.split('\n'); idx=0
            for k,ln in enumerate(lines):
                if ln.startswith('use '): idx=k+1
            lines[idx:idx]=add
            open(f,'w',encoding='utf8').write('\n'.join(lines))
            print(f.split('/✏️editor/')[1],len(add))
