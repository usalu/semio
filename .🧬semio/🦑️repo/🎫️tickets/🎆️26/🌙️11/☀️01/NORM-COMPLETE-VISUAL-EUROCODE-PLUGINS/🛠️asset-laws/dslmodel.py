import os,re,json
R="/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
SUB="🏅️standards/🔖️1/🪆️subsets/✳️any"
ARTS=["⚖️en1990","🏋️en1991","🏛️en1992","🔩️en1993","🧩️en1994","🪵️en1995","🪨️en1996","🌍️en1997","🫨️en1998","🪶️en1999"]
def norm(s): return re.sub(r'[^a-z0-9]','',s.lower())
def kebab(s): return s.replace('_','-')
def pascal_kebab(s):
    return re.sub(r'(?<=[a-z0-9])(?=[A-Z])','-',s).lower()
KEYS={}
def spell(enum,variant):
    return KEYS.get((enum,variant)) or pascal_kebab(variant)
STRUCT=re.compile(r'((?:^[ \t]*(?:#\[[^\n]*\]|///[^\n]*)\n)*)^[ \t]*pub struct (\w+)\s*\{(.*?)^\}',re.S|re.M)
ENUM=re.compile(r'^[ \t]*pub enum (\w+)\s*\{(.*?)^\}',re.S|re.M)
class Field:
    def __init__(s,name,rtype,attrs): s.name=name;s.rtype=rtype.strip();s.attrs=attrs
    @property
    def key(s): return kebab(s.name)
def parse_rust(base):
    structs={};enums={}
    for root,ds,fs in os.walk(base):
        if "node_modules" in root or "🧪️tests" in root: continue
        for f in fs:
            if f!="🦀️.rs": continue
            p=os.path.join(root,f)
            try:t=open(p).read()
            except:continue
            for m in STRUCT.finditer(t):
                body=m.group(3);fields=[];attrs=[]
                for ln in body.split("\n"):
                    s=ln.strip()
                    if s.startswith("#["): attrs.append(s); continue
                    if s.startswith("///") or not s: continue
                    mm=re.match(r'pub (\w+):\s*(.+?),?$',s)
                    if mm: fields.append(Field(mm.group(1),mm.group(2),attrs)); attrs=[]
                    else: attrs=[]
                if fields: structs.setdefault(m.group(2),[]).append((p,fields))
            for m in ENUM.finditer(t):
                vs=[];default=None;pend=False;key=None
                for ln in m.group(2).split("\n"):
                    s=ln.strip()
                    if s.startswith("#[default]"): pend=True; continue
                    mk=re.match(r'#\[dsl\(key\s*=\s*"([^"]+)"',s)
                    if mk: key=mk.group(1); continue
                    if s.startswith("#[") or s.startswith("///") or not s: continue
                    mm=re.match(r'(\w+)\s*(?:[,=({]|$)',s)
                    if mm:
                        vs.append(mm.group(1))
                        if key: KEYS[(m.group(1),mm.group(1))]=key; key=None
                        if pend: default=mm.group(1); pend=False
                if vs: enums.setdefault(m.group(1),(vs,default or vs[0]))
    return structs,enums
def unit_of(attrs):
    for a in attrs:
        m=re.search(r'unit\s*=\s*"([^"]*)"',a)
        if m and a.startswith("#[dsl"): return m.group(1)
def tag(f,structs,enums):
    t=re.sub(r'\b\w+::','',f.rtype)
    opt=False
    if t.startswith("Option<"): t=t[7:-1];opt=True
    u=unit_of(f.attrs)
    if t.startswith("Vec<"):
        inner=t[4:-1]
        tbl=any(a.startswith("#[dsl") and "table" in a for a in f.attrs)
        return ("TABLE" if tbl else "LIST"),inner,opt
    if t=="String": return "TEXT",None,opt
    if t in("f64","f32"): return ("QTY" if u else "NUM"),u,opt
    if t in("u8","u16","u32","u64","usize"): return "UINT",None,opt
    if t in("i8","i16","i32","i64","isize"): return "INT",None,opt
    if t=="bool": return "BOOL",None,opt
    if t in enums: return "ENUM",t,opt
    if t in structs: return "REC",t,opt
    return "?"+t,None,opt
def schema_of(art):
    return json.load(open(f"{R}/{art}/{SUB}/🧬️schema/📸️snapshot/🔣️.json"))
def pick(structs,name,jprops):
    c=structs.get(name)
    if not c: return None
    jn=[norm(x) for x in jprops]
    for p,fl in c:
        if [norm(f.name) for f in fl]==jn: return fl
    for p,fl in c:
        if set(norm(f.name) for f in fl)==set(jn): return fl
    return None

def getstruct(structs,name):
    c=structs.get(name)
    if not c: return None
    for pref in ("📸️snapshot","model"):
        for p,fl in c:
            if pref in p: return fl
    return c[0][1]

from decimal import Decimal
def rust_display(x):
    if x==int(x) and abs(x)<1e16: return str(int(x))
    d=Decimal(repr(float(x)))
    s=format(d,'f')
    if '.' in s: s=s.rstrip('0').rstrip('.')
    return s
def parse_defaults(base):
    out={}
    pat=re.compile(r'impl Default for (\w+)\s*\{\s*(?:#\[[^\n]*\]\s*)*fn default\(\)\s*->\s*(?:Self|\w+)\s*\{(.*?)^    \}\s*^\}',re.S|re.M)
    for root,ds,fs in os.walk(base):
        if "node_modules" in root or "🧪️tests" in root: continue
        for f in fs:
            if f!="🦀️.rs": continue
            try:t=open(os.path.join(root,f)).read()
            except:continue
            for m in pat.finditer(t):
                fields={}
                for mm in re.finditer(r'^\s{8,16}(\w+):\s*(.+?),\s*$',m.group(2),re.M):
                    fields[mm.group(1)]=mm.group(2).strip()
                out.setdefault(m.group(1),{}).update(fields)
    return out
def lit(expr,t,x,enums):
    e=expr.strip()
    e=re.sub(r'\.(into|to_string|to_owned)\(\)$','',e)
    m=re.fullmatch(r'String::from\((".*")\)',e)
    if m: e=m.group(1)
    if t=="TEXT":
        m=re.fullmatch(r'"(.*)"',e)
        if m or e=='String::new()':
            v=m.group(1) if m else ""
            return '""' if v=="" else (v if re.fullmatch(r'[A-Za-z_][A-Za-z0-9_-]*',v) and v not in("true","false") else '"'+v+'"')
    if t in("NUM","QTY","UINT","INT"):
        e2=e.replace("_","")
        e2=re.sub(r'(f64|f32|u8|u16|u32|u64|usize|i8|i16|i32|i64)$','',e2)
        try:
            v=float(e2)
            return rust_display(v)+(x if t=="QTY" else "")
        except ValueError: pass
    if t=="BOOL" and e in("true","false"): return e
    if t=="ENUM":
        m=re.search(r'(\w+)$',e)
        if m and m.group(1) in enums[x][0]: return spell(x,m.group(1))
    if t in("TABLE","LIST") and e in("vec![]","Vec::new()"): return "[ ]"
    return None
