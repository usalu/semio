"""🦀 Minimal Rust skeleton reader: blanks comments/strings/char literals (positions kept), scans top-level items, `mod`/`#[path]` mounts and `pub use` trees, and resolves public names of a crate to the file that defines them."""
import re,os
def skeleton(t):
    o=list(t);i=0;n=len(t)
    def blank(a,b):
        for k in range(a,b):
            if o[k]!='\n': o[k]=' '
    while i<n:
        c=t[i]
        if c=='/' and t.startswith('//',i):
            j=t.find('\n',i);j=n if j<0 else j;blank(i,j);i=j
        elif c=='/' and t.startswith('/*',i):
            d=1;j=i+2
            while j<n and d:
                if t.startswith('/*',j): d+=1;j+=2
                elif t.startswith('*/',j): d-=1;j+=2
                else: j+=1
            blank(i,j);i=j
        elif c=='r' and re.match(r'r#*"',t[i:i+12]) and (i==0 or not (t[i-1].isalnum() or t[i-1]=='_')):
            m=re.match(r'r(#*)"',t[i:]);h=m.group(1);end='"'+h
            j=t.find(end,i+len(m.group(0)))
            j=n if j<0 else j+len(end);blank(i+1,j-1) if j-1>i+1 else None;i=j
        elif c=='b' and t.startswith('b"',i) or c=='"':
            j=i+(2 if c=='b' else 1)
            while j<n and t[j]!='"':
                j+=2 if t[j]=='\\' else 1
            blank(i+(2 if c=='b' else 1),j);i=j+1
        elif c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",t[i:i+12])
            if m: blank(i+1,i+len(m.group(0))-1);i+=len(m.group(0))
            else: i+=1
        else: i+=1
    return ''.join(o)
IDENT=r'[A-Za-z_][A-Za-z0-9_]*'
def toplevel(sk):
    """yield (start,end,text) of each depth-0 statement/item header chunk, with the chunk including its attributes."""
    out=[];d=0;start=0;i=0;n=len(sk)
    while i<n:
        c=sk[i]
        if c in '{([': d+=1
        elif c in '})]':
            d-=1
            if d==0 and c=='}':
                out.append((start,i+1));start=i+1
        elif c==';' and d==0:
            out.append((start,i+1));start=i+1
        i+=1
    return out
ITEM=re.compile(r'^\s*(?P<attrs>(?:#\s*!?\[[^\]]*\]\s*)*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?(?:(?:unsafe|async|const|extern\s*"[^"]*"|default)\s+)*(?P<kw>(?:struct|enum|union|trait|type|fn|static|const|mod|use|impl|extern\s+crate)\b|macro_rules!)\s*(?P<rest>.*)',re.S)
def parse(path):
    return parse_text(open(path,encoding='utf-8',errors='ignore').read())
def parse_text(t,sk=None):
    sk=sk if sk is not None else skeleton(t)
    items=[]
    for a,b in toplevel(sk):
        chunk=sk[a:b];m=ITEM.match(chunk)
        if not m:
            im=re.match(r'^\s*(?:#\s*\[[^\]]*\]\s*)*include!\s*\(',chunk)
            if im:
                pm=re.search(r'include!\s*\(\s*"([^"]+)"',t[a:b])
                if pm: items.append({'kw':'include','vis':'','pub':False,'start':a,'end':b,'path':pm.group(1),'attrs':'','cfgtest':'cfg(test)' in re.sub(r'\s+','',t[a:b][:200])})
            continue
        attrs=m.group('attrs') or '';raw=t[a:b]
        pm=re.search(r'#\s*\[\s*path\s*=\s*"([^"]+)"',raw)
        vis=(m.group('vis') or '').strip()
        kw=' '.join(m.group('kw').split()) if not m.group('kw').startswith('macro') else 'macro_rules!'
        rest=m.group('rest')
        it={'kw':kw,'vis':vis,'pub':vis=='pub','start':a,'end':b,'path':pm.group(1) if pm else None,'attrs':attrs,'cfgtest':'cfg(test)' in re.sub(r'\s+','',attrs)}
        if kw in('struct','enum','union','trait','type','fn','static','const','mod'):
            nm=re.match(IDENT,rest)
            it['name']=nm.group(0) if nm else None
            it['inline']=(kw=='mod' and '{' in chunk and not chunk.rstrip().endswith(';'))
        elif kw=='macro_rules!':
            nm=re.match(IDENT,rest);it['name']=nm.group(0) if nm else None
            it['export']='macro_export' in attrs
        elif kw=='use':
            it['use']=rest.rstrip().rstrip(';')
        elif kw=='extern crate':
            it['ec']=rest.rstrip().rstrip(';')
        items.append(it)
        for dm in re.finditer(r'proc_macro_derive\s*\(\s*(%s)'%IDENT,attrs):
            items.append({'kw':'struct','vis':'pub','pub':True,'start':a,'end':b,'path':None,'attrs':'','cfgtest':False,'name':dm.group(1)})
    return t,sk,items
def use_leaves(u,prefix=()):
    """expand a use tree into [(path tuple, alias or None)] ; glob leaf is ('*',)."""
    u=u.strip()
    out=[]
    # split top-level commas
    def split(s):
        parts=[];d=0;cur=''
        for ch in s:
            if ch=='{': d+=1
            if ch=='}': d-=1
            if ch==',' and d==0: parts.append(cur);cur=''
            else: cur+=ch
        if cur.strip(): parts.append(cur)
        return parts
    u=re.sub(r'\s+',' ',u)
    u=re.sub(r'\s*(::|\{|\}|,)\s*',r'\1',u).strip()
    def rec(s,pre):
        if s.startswith('{') and s.endswith('}') and s[1:-1].count('{')==s[1:-1].count('}'):
            for p in split(s[1:-1]): rec(p,pre)
            return
        m=re.match(r'^(::)?(.*?)::(\{.*\})$',s)
        if m:
            segs=tuple(x for x in m.group(2).split('::') if x)
            for p in split(m.group(3)[1:-1]): rec(p,pre+segs)
            return
        am=re.match(r'^(.*?) as (%s)$'%IDENT,s)
        alias=None
        if am and not s.endswith('::*'): s,alias=am.group(1),am.group(2)
        segs=tuple(x for x in s.split('::') if x)
        out.append((pre+segs,alias))
    rec(u,tuple(prefix))
    return out
