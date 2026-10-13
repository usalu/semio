import json,os,re,sys,collections,argparse,hashlib

ROOT="/Users/ueli/Documents/semio"
CODES={"deprecated","elided_lifetimes_in_paths","unused_qualifications","unused_imports","unused_mut","unused_braces","unused_parens","unused_variables"}

ap=argparse.ArgumentParser()
ap.add_argument("--run",action="append",required=True,help="label=json:basedir")
ap.add_argument("--apply",action="store_true")
ap.add_argument("--codes",default=",".join(sorted(CODES)))
ap.add_argument("--report")
ap.add_argument("--deny")
args=ap.parse_args()
codes=set(args.codes.split(","))
deny=set()
if args.deny and os.path.exists(args.deny):
    deny=set(l.strip() for l in open(args.deny,encoding="utf-8") if l.strip())

def resolve(base,name):
    p=name if os.path.isabs(name) else os.path.join(base,name)
    return os.path.realpath(p)

def seg_text(span):
    lines=[t["text"] for t in span["text"]]
    if not lines:return None
    hs=span["text"][0]["highlight_start"]-1
    he=span["text"][-1]["highlight_end"]-1
    if len(lines)==1:return lines[0][hs:span["text"][0]["highlight_end"]-1]
    return "\n".join([lines[0][hs:]]+lines[1:-1]+[lines[-1][:he]])

runs={}
entries=collections.defaultdict(dict)
unit_flags=collections.defaultdict(set)
unit_cover={}

def parse_depinfo(path,base):
    out=set()
    try:txt=open(path,encoding="utf-8").read()
    except Exception:return out
    for line in txt.split("\n"):
        if not line or line.startswith("#"):continue
        if ":" not in line:continue
        head,_,rest=line.partition(": ")
        toks=re.split(r"(?<!\\) ",rest)
        for t in toks:
            t=t.replace("\\ "," ").strip()
            if t.endswith(".rs"):
                out.add(resolve(base,t))
    return out

for spec in args.run:
    label,rest=spec.split("=",1)
    path,base=rest.rsplit(":",1)
    runs[label]=path
    for l in open(path,encoding="utf-8"):
        try:m=json.loads(l)
        except Exception:continue
        if m.get("reason")=="compiler-artifact" and m.get("filenames"):
            fn=m["filenames"][0]
            dn,bn=os.path.dirname(fn),os.path.basename(fn)
            stem=bn.rsplit(".",1)[0]
            if stem.startswith("lib"):stem=stem[3:]
            dp=os.path.join(dn,stem+".d")
            uk=(label,m["package_id"],m["target"]["name"],tuple(m["target"]["kind"]))
            if os.path.exists(dp):unit_cover[uk]=parse_depinfo(dp,base)
            continue
        if m.get("reason")!="compiler-message":continue
        d=m["message"]
        if d["level"]!="warning":continue
        code=(d.get("code") or {}).get("code")
        if code not in codes:continue
        prim=[s for s in d["spans"] if s["is_primary"]]
        if not prim:continue
        sug=None
        for ch in d["children"]:
            sp=[s for s in ch["spans"] if s.get("suggested_replacement") is not None]
            if sp and all(s["suggestion_applicability"]=="MachineApplicable" or (code=="elided_lifetimes_in_paths" and s["suggestion_applicability"]=="Unspecified") for s in sp):
                sug=(ch["message"],sp);break
        if not sug:continue
        helpmsg,sp=sug
        if code=="unused_variables":
            if "try ignoring the field" in helpmsg:
                bad=False
                for s in sp:
                    line=s["text"][0]["text"] if s["text"] else ""
                    tail=line[s["text"][0]["highlight_end"]-1:].lstrip()
                    if tail.startswith(":"):bad=True
                if bad:continue
            elif "prefix it with an underscore" not in helpmsg:
                continue
        edits=[]
        ok=True
        for s in sp:
            f=resolve(base,s["file_name"])
            if not f.startswith(ROOT+"/") or "/.🧬semio/" in f or "/target/" in f or "/node_modules/" in f:ok=False;break
            exp=seg_text(s)
            if exp is None:ok=False;break
            edits.append((f,s["byte_start"],s["byte_end"],s["suggested_replacement"],exp))
        if not ok:continue
        files={e[0] for e in edits}
        if len(files)!=1:continue
        key=(code,tuple(sorted(edits)),tuple(re.findall(r"`([^`]*)`",d["message"])))
        entries[key][label]=d["message"]
        unit_flags[(label,m["package_id"],m["target"]["name"],tuple(m["target"]["kind"]))].add(key)

def test_text(f):
    chunks=[]
    try:src=open(f,encoding="utf-8").read()
    except Exception:return ""
    i=src.find("#[cfg(test)]")
    if i>=0:chunks.append(src[i:])
    d=os.path.dirname(f)
    for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]',src):
        p=os.path.normpath(os.path.join(d,m.group(1)))
        if os.path.isfile(p):
            try:chunks.append(open(p,encoding="utf-8").read())
            except Exception:pass
    td=os.path.join(d,"🧪️tests")
    if os.path.isdir(td):
        for r,_,fs in os.walk(td):
            for n in fs:
                if n.endswith(".rs"):
                    try:chunks.append(open(os.path.join(r,n),encoding="utf-8").read())
                    except Exception:pass
    return "\n".join(chunks)

def has_test_context(f):
    try:src=open(f,encoding="utf-8").read()
    except Exception:return False
    d=os.path.dirname(f)
    return "#[cfg(test)]" in src or os.path.isdir(os.path.join(d,"🧪️tests"))


def ident_of(item):
    item=item.strip()
    m=re.search(r" as (\w+)$",item)
    if m:return m.group(1)
    return item.split("::")[-1]

def word_in(ident,text):
    return re.search(r"(?<![A-Za-z0-9_])"+re.escape(ident)+r"(?![A-Za-z0-9_])",text) is not None

def split_group(f,edits,names,tt):
    try:data=open(f,"rb").read();src=data.decode("utf-8")
    except Exception:return None
    bs=min(e[1] for e in edits);be=max(e[2] for e in edits)
    pre=data[:bs].decode("utf-8")
    cands=list(re.finditer(r"(?:^|(?<=[\n;{}]))([ \t]*)((?:pub(?:\([^)]*\))?[ \t]+)?use[ \t]+)",pre))
    if not cands:return None
    m=cands[-1]
    sb=len(pre[:m.start(2)].encode("utf-8"))
    indent=m.group(1)
    semi=data.find(b";",be)
    if semi<0:return None
    se=semi+1
    stmt=data[sb:se].decode("utf-8")
    g=re.match(r"^((?:pub(?:\([^)]*\))?\s+)?use\s+)(.+?)::\{([^{}]*)\}\s*;$",stmt,re.S)
    if not g or "//" in stmt or "/*" in stmt:return None
    head,path,body=g.groups()
    items=[i.strip() for i in body.split(",") if i.strip()]
    unused=set(names)
    keep=[];tests=[]
    for it in items:
        if it in unused or ident_of(it) in unused:
            if it.endswith(" as _") or it=="self":return None
            if word_in(ident_of(it),tt):tests.append(it)
        else:keep.append(it)
    if not tests and not [i for i in items if i in unused or ident_of(i) in unused]:return None
    parts=[]
    if keep:parts.append(f"{head}{path}::{{{', '.join(keep)}}};")
    if tests:parts.append(f"#[cfg(test)] {head}{path}::{{{', '.join(tests)}}};")
    new=("\n"+indent).join(parts)
    return ((f,sb,se,new,stmt),)

stats=collections.Counter()
plan=collections.defaultdict(list)
cache={}
for key,present in entries.items():
    code,edits,names=key
    f=edits[0][0]
    covering=[u for u,fs in unit_cover.items() if f in fs]
    if not covering:
        stats[(code,"no-coverage")]+=1;continue
    if any(key not in unit_flags.get(u,()) for u in covering):
        stats[(code,"unit-veto")]+=1;continue
    if code=="unused_imports":
        tt=cache.get(f)
        if tt is None:tt=cache[f]=test_text(f)
        skip=False
        for n in names:
            if "::" in n or n.endswith("*") or n=="*":
                last=n.split("::")[-1]
                alias=re.search(r" as (\w+)$",n)
                ident=alias.group(1) if alias else last
            else:ident=n
            if ident=="*" :
                if has_test_context(f):skip=True
            elif re.search(r"(?<![A-Za-z0-9_])"+re.escape(ident)+r"(?![A-Za-z0-9_])",tt):skip=True
        if skip:
            whole=all(re.match(r"(pub(\([a-z]+\))? )?use [^\n]*;\n?$",e[4]) and e[3]=="" for e in edits)
            if whole:
                edits=tuple((e[0],e[1],e[2],"#[cfg(test)] "+e[4].rstrip("\n")+("\n" if e[4].endswith("\n") else ""),e[4]) for e in edits)
                stats[(code,"cfg-test-gated")]+=1
                plan[f].append((code+"+cfgtest",edits));continue
            split=split_group(f,edits,names,tt)
            if split:
                stats[(code,"cfg-test-split")]+=1
                plan[f].append((code+"+cfgsplit",split));continue
            stats[(code,"test-guard-partial")]+=1;continue
    if any(f+"|"+e[4] in deny for e in edits):stats[(code,"denied")]+=1;continue
    plan[f].append((code,edits))
    stats[(code,"planned")]+=1

for k,v in sorted(stats.items()):print(v,k)
print("files",len(plan))
if args.report:
    with open(args.report,"w",encoding="utf-8") as fh:
        for f,ps in plan.items():
            for code,edits in ps:fh.write(f"{f}\t{code}\t{edits[0][1]}\t{edits[0][4][:60]!r}\n")
if not args.apply:sys.exit(0)

applied=collections.Counter()
skipped=collections.Counter()
log=[]
for f,ps in plan.items():
    try:
        st=os.stat(f);data=open(f,"rb").read()
    except Exception:continue
    accepted=[]
    taken=[]
    for code,edits in sorted(ps,key=lambda p:p[1][0][1]):
        good=True
        for (_,bs,be,rep,exp) in edits:
            if data[bs:be].decode("utf-8","replace").rstrip("\n")!=exp.rstrip("\n"):good=False;break
            for (a,b) in taken:
                if bs<b and be>a:good=False;break
            if not good:break
        if not good:
            skipped[code]+=1;continue
        for (_,bs,be,rep,exp) in edits:taken.append((bs,be))
        accepted.append((code,edits))
    if not accepted:continue
    flat=[]
    for code,edits in accepted:
        for (_,bs,be,rep,exp) in edits:flat.append((bs,be,rep,code,exp))
    flat.sort(reverse=True)
    out=bytearray(data)
    for bs,be,rep,code,exp in flat:
        out[bs:be]=rep.encode("utf-8")
        log.append((f,code,exp,rep))
    for code,_ in accepted:applied[code]+=1
    st2=os.stat(f)
    if st2.st_mtime_ns!=st.st_mtime_ns or st2.st_size!=st.st_size:
        skipped["raced"]+=1;continue
    with open(f,"wb") as fh:fh.write(bytes(out))
print("applied",dict(applied),"skipped",dict(skipped))
if args.report:
    with open(args.report+".applied","w",encoding="utf-8") as fh:
        for f,code,exp,rep in log:fh.write(json.dumps([f,code,exp,rep],ensure_ascii=False)+"\n")
