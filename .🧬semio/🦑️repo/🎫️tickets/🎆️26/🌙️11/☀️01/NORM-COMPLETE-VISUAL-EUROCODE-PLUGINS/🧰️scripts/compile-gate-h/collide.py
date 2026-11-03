import re,os,sys,collections
sys.path.insert(0,os.environ.get("RSCK_DIR","/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🗑️generated/compile-review-a"))
from rsck import clean
def toplevel(src):
    out=[];d=0;cur=[]
    # split into top-level statements at depth 0 using braces/semicolons
    i=0;n=len(src);stmt=[];
    depth=0
    res=[]
    buf=''
    for ch in src:
        if ch=='{':
            if depth==0: buf+=ch+' '
            depth+=1
            continue
        if ch=='}':
            depth-=1
            if depth==0:
                res.append(buf+'}'); buf=''
            continue
        if depth==0:
            buf+=ch
            if ch==';':
                res.append(buf); buf=''
    if buf.strip(): res.append(buf)
    return res
def flatten_use(body):
    # body like a::{b, c::d as e, f::{g,h}, *}
    names=[]
    def rec(prefix,s):
        s=s.strip()
        if not s: return
        if s.startswith('{') :
            inner=s[1:s.rfind('}')]
            depth=0;cur='';parts=[]
            for ch in inner:
                if ch=='{':depth+=1
                if ch=='}':depth-=1
                if ch==',' and depth==0: parts.append(cur);cur=''
                else: cur+=ch
            parts.append(cur)
            for p in parts: rec(prefix,p)
            return
        m=re.match(r'^([\w:]*?)(::)?(\{.*\})$',s,re.S)
        if m and m.group(3):
            rec(prefix+m.group(1)+('::' if m.group(1) else ''),m.group(3)); return
        if s.endswith('*'): return
        mm=re.match(r'^(.*?)\s+as\s+(\w+)$',s)
        if mm:
            names.append((mm.group(2),prefix+mm.group(1))); return
        last=s.split('::')[-1].strip()
        if last=='self':
            last=prefix.rstrip(':').split('::')[-1]
        if last and last!='_': names.append((last,prefix+s))
    rec('',body)
    return names
def analyse(path):
    try: src=open(path,encoding='utf8').read()
    except: return []
    src=clean(src)
    src=re.sub(r'#!?\[(?:[^\[\]]|\[[^\]]*\])*\]','',src)
    tl=toplevel(src)
    imports=collections.defaultdict(list); defs=collections.defaultdict(list)
    for st in tl:
        s=st.strip()
        s=re.sub(r'^(?:pub(?:\([^)]*\))?\s+)','',s)
        m=re.match(r'^use\s+(.*?);$',s,re.S)
        if m:
            for nm,full in flatten_use(m.group(1).strip()): imports[nm].append(full)
            continue
        m=re.match(r'^(?:unsafe\s+)?(?:const\s+)?(?:async\s+)?(struct|enum|trait|type|union)\s+(\w+)',s)
        if m: defs[m.group(2)].append(m.group(1)); continue
        m=re.match(r'^(?:const\s+|unsafe\s+|async\s+|extern\s+"[^"]*"\s+)*fn\s+(\w+)',s)
        if m: defs[m.group(1)].append('fn'); continue
        m=re.match(r'^(const|static)\s+(?:mut\s+)?(\w+)',s)
        if m: defs[m.group(2)].append(m.group(1)); continue
        m=re.match(r'^mod\s+(\w+)',s)
        if m: defs[m.group(1)].append('mod'); continue
        m=re.match(r'^macro_rules!\s*(\w+)',s)
    out=[]
    for nm,l in defs.items():
        kinds=l
        if len(l)>1:
            ns=collections.Counter('v' if k in('fn','const','static') else 't' for k in l)
            if any(c>1 for c in ns.values()): out.append((path,'duplicate def %s %s'%(nm,l)))
        if nm in imports:
            for full in imports[nm]:
                out.append((path,'import/def clash %s: use %s vs local %s'%(nm,full,l)))
    for nm,l in imports.items():
        if len(set(l))>1 or len(l)>1 and len(set(l))==1:
            if len(l)>1: out.append((path,'name imported %d times %s: %s'%(len(l),nm,l)))
    return out
if __name__=='__main__':
    for a in sys.argv[1:]:
        for dp,dn,fn in os.walk(a):
            dn[:]=[x for x in dn if x not in('node_modules','target','.git')]
            for f in fn:
                if f.endswith('.rs'):
                    for p,m in analyse(os.path.join(dp,f)): print(p.replace('/home/deck/semio/',''),':',m)
