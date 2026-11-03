import os,re,sys,json,shutil
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from dslmodel import *
APPLY="--apply" in sys.argv
FILES=[a.split("=",1)[1] for a in sys.argv[1:] if a.startswith("--file=")]
ONLY=[a for a in sys.argv[1:] if not a.startswith("--")]
BLOCKING=("is not a variant","trailing tokens","stray token","unparsed word","orphan scalar","tables not in model","nested block form")
BACKUP=os.path.join(os.path.dirname(os.path.abspath(__file__)),"..","🗑️generated","asset-backup")
SHARED="/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance"
def scan_tokens(s,i=0,stop=None):
    out=[];n=len(s)
    while i<n:
        if s[i].isspace(): i+=1; continue
        j=i;depth=0;q=False
        while j<n:
            c=s[j]
            if q:
                if c=='\\': j+=1
                elif c=='"': q=False
            elif c=='"': q=True
            elif c in '[{': depth+=1
            elif c in ']}':
                depth-=1
                if depth<0: break
            elif c.isspace() and depth==0: break
            j+=1
        if j==i: break
        out.append(s[i:j]); i=j
    return out
def match(s,i,o,c):
    d=0;q=False;n=len(s)
    while i<n:
        ch=s[i]
        if q:
            if ch=='\\': i+=1
            elif ch=='"': q=False
        elif ch=='"': q=True
        elif ch==o: d+=1
        elif ch==c:
            d-=1
            if d==0: return i
        i+=1
    raise ValueError("unbalanced")
def parse_body(body,warn):
    items=[];i=0;n=len(body)
    while i<n:
        if body[i].isspace(): i+=1; continue
        m=re.match(r'[A-Za-z][A-Za-z0-9_-]*',body[i:])
        if not m:
            e=i
            while e<n and not body[e].isspace(): e+=1
            warn.append(f"stray token dropped: {body[i:e]!r}"); i=e; continue
        j=i+m.end();name=m.group(0)
        if j<n and body[j]=='=':
            k=j+1;q=False;d=0
            while k<n and (q or d>0 or not body[k].isspace()):
                if body[k]=='"': q=not q
                elif not q and body[k] in '[{': d+=1
                elif not q and body[k] in ']}': d-=1
                k+=1
            items.append(("scalar",name,body[i:k])); i=k; continue
        k=j
        while k<n and body[k].isspace(): k+=1
        if k<n and body[k]=='[':
            e=match(body,k,'[',']'); header=scan_tokens(body[k+1:e])
            m2=e+1
            while body[m2].isspace(): m2+=1
            assert body[m2]=='{',(name,body[m2:m2+30])
            ce=match(body,m2,'{','}')
            items.append(("table",name,header,scan_tokens(body[m2+1:ce]))); i=ce+1; continue
        warn.append(f"unparsed word dropped: {name}"); i=j
    return items
DEFAULTS={};WARN=[]
OVR_FIELD={("en1992","relative_humidity"):"0.5",("en1994","patch_bearing_length_m"):"0.1",("en1994","cracked_length_fraction"):"0.15",("en1999","product_form"):"extrudedProfile",("en1999","fabrication_class"):"B"}
OVR_ASSET={("en1992","liquid-retaining-fem-anchor","head_diameter"):"0.03"}
CTX={"art":"","asset":""}
def default_cell(f,structs,enums,owner=None,depth=0):
    t,x,opt=tag(f,structs,enums)
    if opt: return "_"
    o=OVR_ASSET.get((CTX["art"],CTX["asset"],f.name)) or OVR_FIELD.get((CTX["art"],f.name))
    if o is not None:
        return lit(o if t not in("TEXT",) else '"'+o+'"',t,x,enums)
    if owner and owner in DEFAULTS and f.name in DEFAULTS[owner]:
        v=lit(DEFAULTS[owner][f.name],t,x,enums)
        if v is not None: return v
        WARN.append(f"{owner}.{f.name}: default expr `{DEFAULTS[owner][f.name]}` not literal, using type zero")
    if t=="TEXT": return '""'
    if t=="NUM" or t=="UINT" or t=="INT": return "0"
    if t=="QTY": return "0"+x
    if t=="BOOL": return "false"
    if t=="ENUM": vs,d=enums[x]; return spell(x,d)
    if t in("TABLE","LIST"): return "[ ]"
    if t=="REC":
        inner=getstruct(structs,x)
        return "{"+" ".join(f"{g.key}={default_cell(g,structs,enums,x)}" for g in inner)+"}"
    raise ValueError(f"no default for {f.name}:{f.rtype}")
def norm_record(tok,sname,structs,enums,warn,where):
    fl=getstruct(structs,sname)
    body=tok.strip()
    assert body[0]=="{" and body[-1]=="}"
    parts=scan_tokens(body[1:-1])
    if any("=" not in x.split("[")[0].split("{")[0].split('"')[0] for x in parts):
        warn.append(f"{where}: nested block form left untouched"); return tok
    kv={}
    for x in parts:
        k,_,v=x.partition("=")
        kv[norm(k)]=v
    names={norm(f.name) for f in fl}
    for k in kv:
        if k not in names: warn.append(f"{where}: stale nested key `{k}` dropped from {sname}")
    out=[]
    for f in fl:
        k=norm(f.name)
        out.append(f"{f.key}="+(convert(kv[k],f,structs,enums,warn,where+"."+f.key) if k in kv else default_cell(f,structs,enums,sname)))
    return "{"+" ".join(out)+"}"
def convert(cell,g,structs,enums,warn,where):
    t,x,opt=tag(g,structs,enums)
    if cell=="_" and opt: return cell
    if t in("TABLE","LIST") and x in structs:
        b=cell.strip()
        if not (b.startswith("[") and b.endswith("]")): return cell
        recs=scan_tokens(b[1:-1])
        if not recs: return "[ ]"
        if not all(r.startswith("{") for r in recs): return cell
        return "[ "+" ".join(norm_record(r,x,structs,enums,warn,where) for r in recs)+" ]"
    if t=="REC" and cell.strip().startswith("{"):
        return norm_record(cell,x,structs,enums,warn,where)
    if t=="ENUM":
        v=cell.strip('"')
        vs=enums[x][0]
        kv=[spell(x,e) for e in vs]
        if v in kv: return v
        for e,k in zip(vs,kv):
            if norm(e)==norm(v): return k
        warn.append(f"UNMAPPED {where}: `{v}` is not a variant of {x} {kv}")
        return cell
    if t=="QTY":
        if re.fullmatch(r'-?[0-9.]+(e-?[0-9]+)?',cell): return cell+x
        return cell
    if t in("NUM","UINT","INT"):
        m=re.fullmatch(r'(-?[0-9.]+(?:e-?[0-9]+)?)[A-Za-z%/°0-9]+',cell)
        if m and not re.fullmatch(r'-?[0-9.]+(e-?[0-9]+)?',cell): return rust_display(float(m.group(1)))
        if re.fullmatch(r'-?[0-9]+\.[0-9]*',cell) or re.fullmatch(r'-?[0-9.]+e-?[0-9]+',cell): return rust_display(float(cell))
        return cell
    return cell
def header_tag(f,structs,enums):
    t,x,opt=tag(f,structs,enums)
    return f"{f.key}:{ 'ENUM' if t=='ENUM' else t}"
def json_field(name,p,defs):
    attrs=[];opt=False
    u=p.get("x-semio-dsl",{}).get("unit")
    if u: attrs.append(f'#[dsl(unit = "{u}")]')
    if p.get("x-semio-dsl",{}).get("table"): attrs.append("#[dsl(table)]")
    if "oneOf" in p:
        r=[x for x in p["oneOf"] if "$ref" in x]; opt=True; p=r[0] if r else p
    ty=p.get("type")
    if isinstance(ty,list): opt=opt or "null" in ty; ty=[x for x in ty if x!="null"][0]
    if "$ref" in p: rt=p["$ref"].split("/")[-1]
    elif ty=="string": rt="String"
    elif ty=="number": rt="f64"
    elif ty=="integer": rt="u32" if p.get("minimum")==0 else "i64"
    elif ty=="boolean": rt="bool"
    elif ty=="array":
        it=p["items"]
        rt="Vec<"+(it["$ref"].split("/")[-1] if "$ref" in it else {"string":"String","number":"f64","integer":"i64","boolean":"bool"}[it["type"]])+">"
    else: raise ValueError(f"json type unsupported for {name}: {p}")
    return Field(re.sub(r'(?<=[a-z0-9])([A-Z])',lambda m:"_"+m.group(1).lower(),name).lower() if not name.islower() else name,("Option<"+rt+">") if opt else rt,attrs)
def snake(n):
    return re.sub(r'([A-Z])',lambda m:'_'+m.group(1).lower(),n)
def overlay_json(structs,enums,sch):
    defs=sch.get("$defs",{})
    objs={sch["title"]:sch,**{k:v for k,v in defs.items() if v.get("type")=="object"}}
    for n,o in objs.items():
        cur={norm(f.name):f for g in structs.get(n,[]) for f in g[1]}
        fl=[]
        for pn,p in o["properties"].items():
            f=cur.get(norm(pn))
            if f is None:
                f=json_field(pn,p,defs); f.name=snake(pn)
            fl.append(f)
        structs[n]=[("📸️snapshot/json-schema",fl)]+[c for c in structs.get(n,[])]
    for n,e in defs.items():
        if e.get("type")=="string" and "enum" in e:
            vs,d=enums.get(n,([],None))
            vs=list(vs)
            for v in e["enum"]:
                if norm(v) not in {norm(x) for x in vs}: vs.append(v[:1].upper()+v[1:])
            enums[n]=(vs,d or vs[0])
def patch(art,path):
    base=f"{R}/{art}/{SUB}"
    CTX["art"]=re.search(r'en\d+',art).group(0);CTX["asset"]=path.split("🖼️assets/")[1].split("/")[0].lstrip(re.match(r'[^a-z]*',path.split("🖼️assets/")[1].split("/")[0]).group(0)) if False else re.sub(r'^[^a-z0-9]+','',path.split("🖼️assets/")[1].split("/")[0])
    structs,enums=parse_rust(base)
    DEFAULTS.clear();DEFAULTS.update(parse_defaults(base));WARN.clear()
    for p,fl in parse_rust(SHARED)[0].items():
        structs.setdefault(p,[]).extend(fl)
    sch=schema_of(art)
    overlay_json(structs,enums,sch)
    top=getstruct(structs,sch["title"])
    text=open(path).read()
    head,_,body=text.partition("\n")
    warn=[];changes=[]
    items=parse_body(body,warn)
    scal={};tabs={};last=None
    for it in items:
        if it[0]=="scalar":
            if norm(it[1]) in {norm(f.name) for f in top}: scal[norm(it[1])]=[it[2]];last=norm(it[1])
            else:
                if last is None: warn.append(f"orphan scalar {it[1]}")
                else: scal[last].append(it[2])
        else: tabs[norm(it[1])]=it
    out_lines=[head];pend=[]
    def flush():
        nonlocal pend
        if pend: out_lines.append(" ".join(pend)); pend=[]
    for f in top:
        k=norm(f.name);t,x,_=tag(f,structs,enums)
        if t=="LIST":
            if k in scal: pend.extend(scal[k])
            else:
                changes.append(f"{f.key}: ADD list default"); pend.append(f"{f.key}=[ ]")
        elif t=="TABLE":
            flush()
            elem=getstruct(structs,x)
            hdr_new=[header_tag(g,structs,enums) for g in elem]
            if k in tabs:
                _,name,header,toks=tabs[k]
                old=[h.rsplit(":",1) for h in header]
                oldn=[norm(o[0]) for o in old]
                newn=[norm(g.name) for g in elem]
                drop=[o[0] for o in old if norm(o[0]) not in newn]
                add=[g.name for g in elem if norm(g.name) not in oldn]
                if drop: changes.append(f"{f.key}: DROP stale columns {drop}")
                if add: changes.append(f"{f.key}: ADD columns {add}")
                rows=[];nc=len(old)
                full=len(toks)//nc if nc else 0
                if len(toks)%nc: warn.append(f"{f.key}: {len(toks)%nc} trailing tokens dropped (partial row)")
                for r in range(full):
                    cells=toks[r*nc:(r+1)*nc];byn={oldn[i]:cells[i] for i in range(nc)}
                    rows.append([convert(byn[norm(g.name)],g,structs,enums,warn,f"{f.key}[{r}].{g.key}") if norm(g.name) in byn else default_cell(g,structs,enums,x) for g in elem])
                out_lines.append(f"{f.key} [{' '.join(hdr_new)}] "+"{")
                for r in rows: out_lines.append("  "+" ".join(r))
                out_lines.append("}")
            else:
                changes.append(f"{f.key}: ADD empty collection")
                out_lines.append(f"{f.key} [{' '.join(hdr_new)}] "+"{"); out_lines.append("}")
        else:
            if k in scal: pend.extend(scal[k])
            else:
                changes.append(f"{f.key}: ADD scalar default"); pend.append(f"{f.key}={default_cell(f,structs,enums,sch['title'])}")
    flush()
    extra=[k for k in tabs if k not in {norm(f.name) for f in top}]
    if extra: warn.append(f"tables not in model dropped: {extra}")
    warn.extend(sorted(set(WARN)))
    new="\n".join(out_lines)+"\n"
    return text,new,changes,warn
res=0
for art in ARTS:
    if ONLY and not any(o in art for o in ONLY): continue
    base=f"{R}/{art}/{SUB}/🖼️assets"
    for root,ds,fs in os.walk(base):
        for f in fs:
            if not f.endswith("dsl.semio"): continue
            p=os.path.join(root,f)
            if FILES and not any(x in p for x in FILES): continue
            try: old,new,ch,wr=patch(art,p)
            except Exception as e:
                print("ERROR",art,p.split("🖼️assets/")[1],type(e).__name__,e); res+=1; continue
            print(("CHANGED " if old!=new else "same    ")+art,p.split("🖼️assets/")[1])
            for c in ch: print("    +",c)
            for w in wr: print("    !",w)
            blocked=[w for w in wr if any(b in w for b in BLOCKING)]
            if APPLY and old!=new and blocked: print("    SKIPPED apply: blocking warnings")
            if APPLY and old!=new and not blocked:
                dst=os.path.join(BACKUP,art,p.split("🖼️assets/")[1]); os.makedirs(os.path.dirname(dst),exist_ok=True)
                if not os.path.exists(dst): shutil.copy2(p,dst)
                open(p,"w").write(new)
print("errors",res)
