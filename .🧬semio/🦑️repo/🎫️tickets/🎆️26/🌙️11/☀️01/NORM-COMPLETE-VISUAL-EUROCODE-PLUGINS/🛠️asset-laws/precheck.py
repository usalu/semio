import os,re,sys,json
R="/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
SUB="🏅️standards/🔖️1/🪆️subsets/✳️any"
ARTS=sys.argv[1:] or ["⚖️en1990","🏋️en1991","🏛️en1992","🔩️en1993","🧩️en1994","🪵️en1995","🪨️en1996","🌍️en1997","🫨️en1998","🪶️en1999"]
def norm(s): return re.sub(r'[^a-z0-9]','',s.lower())
def tokens(s):
    out=[];i=0;n=len(s)
    while i<n:
        c=s[i]
        if c.isspace(): i+=1; continue
        j=i;depth=0;q=False
        while j<n:
            c=s[j]
            if q:
                if c=='\\': j+=1
                elif c=='"': q=False
            elif c=='"': q=True
            elif c in '[{': depth+=1
            elif c in ']}': depth-=1
            elif c.isspace() and depth==0: break
            j+=1
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
def parse(text):
    lines=text.split("\n"); head=lines[0]; body="\n".join(lines[1:])
    i=0;n=len(body);scalars={};tables={}
    while i<n:
        if body[i].isspace(): i+=1; continue
        j=i
        while j<n and not body[j].isspace() and body[j]!='=' : j+=1
        name=body[i:j]
        if j<n and body[j]=='=':
            k=j+1; q=False
            while k<n and (q or not body[k].isspace()):
                if body[k]=='"': q=not q
                k+=1
            scalars[name]=body[j+1:k]; i=k; continue
        k=j
        while body[k].isspace(): k+=1
        if body[k]=='=':  # "key = value" is not used
            raise ValueError("odd scalar "+name)
        if body[k]=='[':
            e=match(body,k,'[',']'); header=body[k+1:e]
            m=e+1
            while body[m].isspace(): m+=1
            assert body[m]=='{',(name,body[m:m+20])
            ce=match(body,m,'{','}')
            rows=[ln for ln in body[m+1:ce].split("\n") if ln.strip()]
            tables[name]=(header,rows); i=ce+1; continue
        scalars[name]=None; i=j
    return head,scalars,tables
def load_schema(art):
    return json.load(open(f"{R}/{art}/{SUB}/🧬️schema/📸️snapshot/🔣️.json"))
def resolve(schema,node):
    while node and "$ref" in node: node=schema["$defs"][node["$ref"].split("/")[-1]]
    return node
def item_def(schema,prop):
    if prop.get("type")=="array": return resolve(schema,prop.get("items",{}))
    return resolve(schema,prop)
def check_cell(tok,col,typ,defn,schema,where,issues):
    pn=norm(col)
    prop=None
    for k,v in (defn or {}).get("properties",{}).items():
        if norm(k)==pn: prop=v
    if prop is None: return
    if typ in("TABLE","REC"):
        inner=item_def(schema,prop)
        body=tok.strip()
        if body[0] in "[{":
            for rec in (tokens(body[1:-1]) if typ=="TABLE" else [body]):
                rb=rec.strip()
                if not rb.startswith("{"): continue
                for kv in tokens(rb[1:-1]):
                    key=kv.split("=",1)[0]
                    if inner and all(norm(key)!=norm(k) for k in inner.get("properties",{})):
                        issues.append(f"{where}.{col}: nested key `{key}` not in schema")
        return
    enum=resolve(schema,prop).get("enum") if resolve(schema,prop) else None
    if enum and typ in ("TEXT","ENUM"):
        v=tok.strip('"')
        if v!="" and v not in enum: issues.append(f"{where}.{col}: value `{v}` not in schema enum {enum}")
def check(art,path,text):
    schema=load_schema(art); issues=[]
    head,scalars,tables=parse(text)
    props=schema["properties"]
    code=re.search(r'en\d+',art).group(0)
    if head.strip()!=f"semio norm.{code}.dsl v1": issues.append(f"header `{head}`")
    pn={norm(k):k for k in props}
    for k in scalars:
        if norm(k) not in pn: issues.append(f"scalar `{k}` not in schema")
    for k,(header,rows) in tables.items():
        if norm(k) not in pn: issues.append(f"table `{k}` not in schema"); continue
        prop=props[pn[norm(k)]]; defn=item_def(schema,prop)
        cols=[c.rsplit(":",1) for c in tokens(header)]
        names=[c[0] for c in cols]; types=[c[1] for c in cols]
        dprops=defn.get("properties",{})
        dn={norm(x):x for x in dprops}
        for c in names:
            if norm(c) not in dn: issues.append(f"{k}: column `{c}` not in schema (stale)")
        have={norm(c) for c in names}
        for x in dprops:
            if norm(x) not in have: issues.append(f"{k}: schema field `{x}` missing from header (new)")
        order=[dn.get(norm(c)) for c in names]
        for r,row in enumerate(rows):
            toks=tokens(row)
            if len(toks)!=len(names): issues.append(f"{k}[{r}]: {len(toks)} cells for {len(names)} columns"); continue
            for t,c,ty in zip(toks,names,types): check_cell(t,c,ty,defn,schema,f"{k}[{r}]",issues)
    for k,p in props.items():
        if norm(k) not in {norm(x) for x in tables} and norm(k) not in {norm(x) for x in scalars} and p.get("type")=="array":
            issues.append(f"collection `{k}` has no table (new)")
    return issues
total=0
for art in ARTS:
    base=f"{R}/{art}/{SUB}"
    for root,ds,fs in os.walk(f"{base}/🖼️assets"):
        for f in fs:
            if f.endswith("dsl.semio"):
                p=os.path.join(root,f)
                try: iss=check(art,p,open(p).read())
                except Exception as e: iss=[f"PARSE ERROR {type(e).__name__}: {e}"]
                total+=len(iss)
                print(("OK   " if not iss else "STALE")+f" {art} {p.split('🖼️assets/')[1]}")
                for i in iss[:40]: print("      -",i)
                if len(iss)>40: print(f"      ... {len(iss)-40} more")
print("TOTAL issues",total)
