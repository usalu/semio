import os,re,sys,json
R="/Users/ueli/Documents/semio"
F=R+"/🧰️framework"
APPLY=len(sys.argv)>1 and sys.argv[1]=="apply"
texts={}
orig={}
REMOVE=[]
STAGE=os.environ.get('CARVE_STAGE')
def get(path):
    if path not in texts:
        orig[path]=open(path,encoding="utf8").read();texts[path]=orig[path]
    return texts[path]
def put(path,s):
    if path not in texts and os.path.exists(path):get(path)
    texts[path]=s
def sub(path,old,new,count=1):
    s=get(path)
    assert s.count(old)==count,(path,old[:80],s.count(old))
    texts[path]=s.replace(old,new)
def subre(path,pattern,new,count=1,flags=0):
    s=get(path)
    n=len(re.findall(pattern,s,flags))
    assert n==count,(path,pattern,n)
    texts[path]=re.sub(pattern,new,s,flags=flags)
def flush():
    changed=0
    tag=os.environ.get("CARVE_TAG")
    if APPLY and tag:
        rec={p:[orig.get(p),s] for p,s in texts.items() if p.endswith("Cargo.toml") and orig.get(p)!=s}
        out=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-carve-x/manifests-"+tag+".json"
        prev=json.load(open(out)) if os.path.exists(out) else {}
        for k,v in rec.items():
            if k in prev:v[0]=prev[k][0]
        prev.update(rec)
        json.dump(prev,open(out,"w"),ensure_ascii=False)
    for p,s in texts.items():
        if orig.get(p)!=s:
            changed+=1
            if STAGE:
                q=STAGE+"/"+os.path.relpath(p,R)
                os.makedirs(os.path.dirname(q),exist_ok=True)
                open(q,"w",encoding="utf8").write(s)
            elif APPLY:
                os.makedirs(os.path.dirname(p),exist_ok=True)
                open(p,"w",encoding="utf8").write(s)
            else:print("would write",p.replace(R+"/",""))
    for p in REMOVE:
        if STAGE:
            q=STAGE+"/"+os.path.relpath(p,R)
            if os.path.exists(q):os.remove(q)
        elif APPLY:
            os.remove(p)
            d=os.path.dirname(p)
            try:os.rmdir(d)
            except OSError:pass
        else:print("would remove",p.replace(R+"/",""))
    print(("applied" if APPLY else "dry-run"),changed,"files")
def project_json(id_):
    return '{\n  "name": "@semio-tech/%s-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n'%id_
def manifest(pkgdir,name,id_,desc,libpath,deps,devdeps=(),features="",workspace=None):
    ups=os.path.relpath(workspace or R,pkgdir)
    out=f'''[package]
workspace = "{ups}"
name = "{name}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "{desc}"

[package.metadata.semio]
role = "framework"
id = "{id_}"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "{libpath}"
'''
    if features:out+="\n[features]\n"+features
    def block(title,rows):
        if not rows:return ""
        return f"\n[{title}]\n"+"\n".join(rows)+"\n"
    return out+block("dependencies",deps)+block("dev-dependencies",devdeps)
def newcrate(dirname,name,id_,desc,libpath,deps,devdeps=(),features="",workspace=None):
    pkg=dirname+"/📦️packages/🦀️rust"
    put(pkg+"/Cargo.toml",manifest(pkg,name,id_,desc,libpath,deps,devdeps,features,workspace))
    put(pkg+"/📋️project.json",project_json(id_))
    return pkg
def insert_sorted_member(toml,rel):
    s=get(toml)
    lines=s.split("\n")
    start=lines.index("members = [");end=lines.index("]",start)
    line='    "'+rel+'",'
    mem=lines[start+1:end]
    assert line not in mem,(toml,rel)
    mem.append(line)
    lines[start+1:end]=mem
    texts[toml]="\n".join(lines)
def add_ws_dep_inline(toml,after_name,name,path):
    s=get(toml)
    lines=s.split("\n")
    idx=[i for i,l in enumerate(lines) if l.startswith(after_name+" = {")]
    assert len(idx)==1,(toml,after_name,idx)
    new=f'{name} = {{ path = "{path}" }}'
    assert new not in lines
    lines.insert(idx[0]+1,new)
    texts[toml]="\n".join(lines)
def add_ws_dep_table(toml,after_name,name,path):
    s=get(toml)
    hdr=f"    [workspace.dependencies.{after_name}]\n"
    assert s.count(hdr)==1,(toml,after_name)
    i=s.index(hdr);j=s.index("\n\n",i)
    block=f"\n\n    [workspace.dependencies.{name}]\n    path = \"{path}\""
    assert f"[workspace.dependencies.{name}]" not in s
    texts[toml]=s[:j]+block+s[j:]
def add_build_owner(name,root_rel):
    p=R+"/📋️build-owners.json"
    s=get(p)
    entry=f'''    {{
      "name": "@semio-tech/{name}-rs",
      "root": "{root_rel}",
      "manifest": "{root_rel}/📋️project.json",
      "packages": []
    }},
'''
    assert f'"@semio-tech/{name}-rs"' not in s,name
    names=[(m.start(),m.group(1)) for m in re.finditer(r'    \{\n      "name": "(@semio-tech/[^"]+)"',s)]
    key=f"@semio-tech/{name}-rs"
    pos=None
    for st,n in names:
        if n>key:pos=st;break
    assert pos is not None
    texts[p]=s[:pos]+entry+s[pos:]
import functools
@functools.lru_cache(None)
def has_manifest(d):
    return os.path.exists(d+"/📦️packages/🦀️rust/Cargo.toml")
def owner_manifest(filepath):
    d=os.path.dirname(filepath)
    while d.startswith(R) and d!=R:
        if has_manifest(d):return d+"/📦️packages/🦀️rust/Cargo.toml"
        if os.path.exists(d+"/Cargo.toml") and "[package]" in open(d+"/Cargo.toml",encoding="utf8").read():return d+"/Cargo.toml"
        d=os.path.dirname(d)
    return None
def balanced(s,i):
    d=0
    for j in range(i,len(s)):
        if s[j]=="{":d+=1
        elif s[j]=="}":
            d-=1
            if d==0:return j
    return -1
def split_top(s):
    parts=[];d=0;cur=""
    for ch in s:
        if ch=="{":d+=1
        if ch=="}":d-=1
        if ch=="," and d==0:parts.append(cur);cur=""
        else:cur+=ch
    if cur.strip():parts.append(cur)
    return parts
def split_use(s,crate,groups):
    """groups: {newcrate:set(symbols)}. rewrites path forms and brace use-trees; returns (new_text,set(newcrates used))"""
    used=set()
    sym2new={sym:nc for nc,syms in groups.items() for sym in syms}
    out="";pos=0
    pat=re.compile(re.escape(crate)+r'::\{')
    while True:
        m=pat.search(s,pos)
        if not m:break
        i=m.end()-1;j=balanced(s,i)
        items=split_top(s[i+1:j])
        keep=[];moved={}
        for it in items:
            h=re.match(r'\s*([A-Za-z_]+)',it)
            if h and h.group(1) in sym2new:moved.setdefault(sym2new[h.group(1)],[]).append(it.strip())
            else:keep.append(it)
        if not moved:
            out+=s[pos:j+1];pos=j+1;continue
        ls=s.rfind("\n",0,m.start())+1
        stmt_prefix=s[ls:m.start()]
        assert re.fullmatch(r'\s*(pub(\([a-z ]+\))? )?use ',stmt_prefix),(stmt_prefix,)
        k=s.index(";",j)
        indent=re.match(r'\s*',stmt_prefix).group(0)
        multiline="\n" in s[i:j]
        def render(prefix_text,path,its):
            if multiline:return prefix_text+path+"{\n"+"".join(indent+"    "+x.strip()+",\n" for x in its)+indent+"};"
            return prefix_text+path+"{"+", ".join(x.strip() for x in its)+"};"
        newstmts=[render(stmt_prefix,nc+"::",its) for nc,its in moved.items()]
        used.update(moved)
        first=""
        if any(x.strip() for x in keep):
            kept=[x for x in keep if x.strip()]
            first=render(stmt_prefix,crate+"::",kept)+"\n"+indent if False else None
        pieces=[]
        if any(x.strip() for x in keep):
            kept=[x for x in keep if x.strip()]
            pieces.append(render(stmt_prefix,crate+"::",kept))
        pieces+=newstmts
        out+=s[pos:ls]+("\n").join(pieces)
        pos=k+1
    out+=s[pos:]
    for nc,syms in groups.items():
        for sym in syms:
            pp=re.compile(re.escape(crate)+r'::'+sym+r'\b')
            if pp.search(out):
                out=pp.sub(nc+"::"+sym,out);used.add(nc)
    return out,used
def add_dep_like(manifest,anchor,newname,newpkgdir):
    """insert `newname` right after each `anchor` dependency line, in the same style (workspace or path)"""
    s=get(manifest)
    lines=s.split("\n")
    out=[];hit=0;mdir=os.path.dirname(manifest)
    pat=re.compile(r'^'+re.escape(anchor)+r'\s*=\s*(.*)$')
    present=any(re.match(r'^'+re.escape(newname)+r'\s*=',l) for l in lines)
    section=None
    secs_done=set()
    for l in lines:
        out.append(l)
        if l.startswith("["):section=l.strip()
        m=pat.match(l)
        if m and not present:
            body=m.group(1)
            if "workspace = true" in body or body.strip().startswith("{ workspace"):
                new=f'{newname} = {{ workspace = true }}'
            elif re.search(r'path\s*=\s*"',body):
                new=f'{newname} = {{ path = "{os.path.relpath(newpkgdir,mdir)}" }}'
            else:
                raise Exception(("unknown dep style",manifest,l))
            if section in secs_done:continue
            secs_done.add(section)
            out.append(new);hit+=1
    assert hit>=1 or present,(manifest,anchor,"not found")
    texts[manifest]="\n".join(out)
    return hit
import subprocess,posixpath
_entries=None
def entries():
    global _entries
    if _entries is not None:return _entries
    _entries={}
    out=subprocess.run(["rg","-l","-g","Cargo.toml","-g","!node_modules","-g","!.🧬semio","-g","!target","^\\[(lib|\\[bin\\]|\\[test\\])","/Users/ueli/Documents/semio"],capture_output=True,text=True).stdout.split("\n")
    for m in out:
        if not m:continue
        t=open(m,encoding="utf8").read()
        for sec in re.split(r'(?m)^(?=\[)',t):
            h=sec.split("\n")[0].strip()
            if h in("[lib]","[[bin]]","[[test]]","[[bench]]","[[example]]"):
                mm=re.search(r'(?m)^path\s*=\s*"([^"]+)"',sec)
                if mm:_entries.setdefault(posixpath.normpath(posixpath.join(os.path.dirname(m),mm.group(1))),set()).add(m)
    return _entries
_pathcache={}
def mounters(f):
    parts=f.split("/")
    key="/".join(parts[-3:-1]) if parts[-2] in("🧪️tests","🧫️fixtures") else parts[-2]
    if key not in _pathcache:
        _pathcache[key]=[x for x in subprocess.run(["rg","-l","--fixed-strings","-g","*.rs","-g","!node_modules","-g","!.🧬semio","-g","!target",key,"/Users/ueli/Documents/semio"],capture_output=True,text=True).stdout.split("\n") if x]
    res=set()
    for c in _pathcache[key]:
        t=open(c,encoding="utf8",errors="replace").read()
        for mm in re.finditer(r'(?:path\s*=|include!\()\s*"([^"]+)"',t):
            if posixpath.normpath(posixpath.join(os.path.dirname(c),mm.group(1)))==f:res.add(c)
    return res
def true_owners(f,seen=None):
    seen=seen if seen is not None else set()
    if f in seen:return set()
    seen.add(f)
    out=set(entries().get(f,()))
    if out:return out
    for c in mounters(f):out|=true_owners(c,seen)
    return out
_index=None
def crate_index():
    global _index
    if _index is not None:return _index
    _index={}
    out=subprocess.run(["rg","-N","--glob","Cargo.toml","--glob","!node_modules","--glob","!.🧬semio","--glob","!target","^name = \"([^\"]+)\"",R,"-r","$1","--with-filename"],capture_output=True,text=True).stdout.split("\n")
    for l in out:
        if not l:continue
        path,_,name=l.rpartition(":")
        if "[package]" not in open(path,encoding="utf8").read():continue
        _index.setdefault(name,set()).add(os.path.dirname(path))
    return _index
def pathdep(pkgdir,name,extra=""):
    idx=crate_index()
    d=idx[name]
    assert len(d)==1,(name,d)
    rel=os.path.relpath(list(d)[0],pkgdir)
    return f'{name} = {{ path = "{rel}"{extra} }}'
def versioned(name,spec):
    return f'{name} = {spec}'
def corpus_edit_dep(path,package,dep,remove=False):
    s=get(path)
    m=re.search(r'"package": ?"'+re.escape(package)+r'",',s)
    assert m,(path,package)
    d=s.index('"dependencies": [',m.end())
    nxt=s.find('"package":',m.end())
    assert nxt==-1 or d<nxt,(path,package,"dependencies after next row")
    e=s.index("]",d)
    body=s[d:e]
    items=re.findall(r'"([^"]+)"',body[len('"dependencies": ['):])
    m2=re.search(r'\n(\s*)"',body)
    indent=m2.group(1) if m2 else "        "
    if remove:
        assert dep in items,(path,package,dep)
        items.remove(dep)
    else:
        if dep in items:return
        items.append(dep);items.sort()
    close_indent=s[s.rfind("\n",0,e)+1:e]
    new='"dependencies": ['+("\n"+",\n".join(indent+'"'+i+'"' for i in items)+"\n"+close_indent if items else "")
    texts[path]=s[:d]+new+s[e:]
