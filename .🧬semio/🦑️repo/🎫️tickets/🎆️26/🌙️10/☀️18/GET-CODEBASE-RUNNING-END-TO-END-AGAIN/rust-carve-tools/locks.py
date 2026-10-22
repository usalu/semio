import os,re,sys,json,subprocess
sys.path.insert(0,os.path.dirname(__file__))
from lib import R
def parse_manifest(text):
    """returns (package_name, workspace_rel or None, normal:set, dev:set) with `package = "x"` renames resolved"""
    secs=re.split(r'(?m)^(?=\[)',text)
    name=None;ws=None;normal=set();dev=set()
    for sec in secs:
        head=sec.split("\n")[0].strip()
        body=sec[len(sec.split("\n")[0]):]
        if head=="[package]":
            m=re.search(r'(?m)^name\s*=\s*"([^"]+)"',body);name=m.group(1) if m else None
            m=re.search(r'(?m)^workspace\s*=\s*"([^"]+)"',body);ws=m.group(1) if m else None
            continue
        kind=None
        m=re.fullmatch(r'\[(?:target\.[^\]]*\.)?(dependencies|dev-dependencies|build-dependencies)\]',head)
        if m:kind=m.group(1)
        m2=re.fullmatch(r'\[(?:target\.[^\]]*\.)?(dependencies|dev-dependencies|build-dependencies)\.([^\]]+)\]',head)
        if m2:
            real=re.search(r'(?m)^package\s*=\s*"([^"]+)"',body)
            n=real.group(1) if real else m2.group(2).strip('"')
            (dev if m2.group(1)=="dev-dependencies" else normal).add(n);continue
        if kind:
            for line in body.split("\n"):
                mm=re.match(r'^([A-Za-z0-9_\-"]+)\s*=\s*(.*)$',line)
                if not mm:continue
                key=mm.group(1).strip('"')
                real=re.search(r'package\s*=\s*"([^"]+)"',mm.group(2))
                n=real.group(1) if real else key
                (dev if kind=="dev-dependencies" else normal).add(n)
    return name,ws,normal,dev
def diff_manifests(before_after):
    """before_after: {path:(old_text|None,new_text)} -> list of dicts per package"""
    out={}
    for path,(old,new) in before_after.items():
        if not path.endswith("Cargo.toml"):continue
        nn,ws,nnormal,ndev=parse_manifest(new)
        if nn is None:continue
        if old is None:onormal=set();odev=set();created=True
        else:
            _,_,onormal,odev=parse_manifest(old);created=False
        wsdir=os.path.normpath(os.path.join(os.path.dirname(path),ws)) if ws else None
        out[nn]={"path":path,"workspace":wsdir,"addN":sorted(nnormal-onormal),"addD":sorted(ndev-odev-nnormal),"remN":sorted(onormal-nnormal),"remD":sorted(odev-ndev),"created":created,"normal":sorted(nnormal),"dev":sorted(ndev)}
    return out
BLOCK=re.compile(r'\[\[package\]\]\nname = "([^"]+)"\nversion = "([^"]+)"\n((?:(?!\n\[\[package\]\]).)*?)(?=\n\n\[\[package\]\]|\n*\Z)',re.S)
def locks():
    out=subprocess.run(["rg","-l","--glob","Cargo.lock","--glob","!node_modules","--glob","!.🧬semio","--glob","!target","^# This file is automatically @generated",R],capture_output=True,text=True).stdout.split("\n")
    return [x for x in out if x]
def apply_to_lock(path,changes,write,donors=('semio-framework-async',)):
    text=open(path,encoding="utf8").read()
    lockdir=os.path.dirname(path)
    head,*rest=re.split(r'\n(?=\[\[package\]\]\n)',text) if False else (None,)
    # split blocks
    parts=text.split("\n[[package]]\n")
    pre=parts[0];blocks=["[[package]]\n"+b for b in parts[1:]]
    def name_of(b):return re.search(r'name = "([^"]+)"',b).group(1)
    names={}
    for i,b in enumerate(blocks):names.setdefault(name_of(b),[]).append(i)
    def deps_list(b):
        m=re.search(r'dependencies = \[\n(.*?)\n\]',b,re.S)
        if not m:return []
        return [x.strip().strip(",").strip('"') for x in m.group(1).split("\n") if x.strip()]
    def set_deps(b,deps):
        deps=sorted(set(deps))
        block=("dependencies = [\n"+"\n".join(f' "{d}",' for d in deps)+"\n]") if deps else None
        if re.search(r'dependencies = \[\n(.*?)\n\]',b,re.S):
            if block:return re.sub(r'dependencies = \[\n(.*?)\n\]',lambda m:block,b,count=1,flags=re.S)
            return re.sub(r'\ndependencies = \[\n(.*?)\n\]','',b,count=1,flags=re.S)
        if not block:return b
        return b.rstrip("\n")+"\n"+block
    changed=False
    def dep_token(dep,donor_deps):
        for t in donor_deps:
            if t==dep or t.startswith(dep+" "):return t
        idx=names.get(dep,[])
        if len(idx)==1:return dep
        if len(idx)==0:return None
        vers=[re.search(r'version = "([^"]+)"',blocks[i]).group(1) for i in idx]
        raise Exception(("ambiguous",path,dep,vers))
    # 1. edge additions/removals on existing packages
    newly_needed=set()
    for pkg,ch in changes.items():
        if pkg not in names:continue
        if ch["created"]:continue
        member=ch["workspace"] is not None and os.path.normpath(ch["workspace"])==lockdir
        i=names[pkg][0]
        cur=deps_list(blocks[i])
        add=list(ch["addN"])+(list(ch["addD"]) if member else [])
        rem=list(ch["remN"])+(list(ch["remD"]) if member else [])
        for d in add:
            if d in changes and changes[d]["created"]:
                newly_needed.add(d)
        for d in rem:
            cur=[t for t in cur if not (t==d or t.startswith(d+" "))]
        for d in add:
            if d in changes and changes[d]["created"]:
                tok=d
            else:
                tok=dep_token(d,cur)
                if tok is None:
                    tok=None
                    continue
            if tok not in cur and not any(t==d or t.startswith(d+" ") for t in cur):cur.append(tok)
        nb=set_deps(blocks[i],cur)
        if nb!=blocks[i]:blocks[i]=nb;changed=True
    # 2. new packages
    for pkg,ch in changes.items():
        if not ch["created"]:continue
        member=ch["workspace"] is not None and os.path.normpath(ch["workspace"])==lockdir
        if pkg in names:continue
        referenced=any(pkg in deps_list(b) for b in blocks)
        if not (member or referenced):continue
        dd=[]
        for dn in donors:
            if dn in names:dd+=deps_list(blocks[names[dn][0]])
        toks=[]
        want=list(ch["normal"])+(list(ch["dev"]) if member else [])
        for d in want:
            if d in names or any(d==x for x in changes if changes[x]["created"]):
                if d in names:
                    t=dep_token(d,dd)
                    if t is None:raise Exception(("dep missing",path,pkg,d))
                    toks.append(t)
                else:toks.append(d)
            else:
                raise Exception(("dep missing",path,pkg,d))
        block=f'[[package]]\nname = "{pkg}"\nversion = "0.1.0"'
        toks=sorted(set(toks))
        if toks:block+="\ndependencies = [\n"+"\n".join(f' "{t}",' for t in toks)+"\n]"
        blocks.append(block);changed=True
        names.setdefault(pkg,[]).append(len(blocks)-1)
    if not changed:return False
    # sort blocks by name (cargo orders by name then version)
    def key(b):
        n=name_of(b);v=re.search(r'version = "([^"]+)"',b).group(1)
        return (n,[int(x) if x.isdigit() else x for x in re.split(r'[.\-+]',v)])
    blocks=[b.rstrip("\n") for b in blocks]
    # stable: keep original order for unchanged blocks, insert new ones by name
    orig_order=[b for b in blocks]
    blocks.sort(key=lambda b:name_of(b).encode("utf8") if False else (name_of(b),))
    out=pre+"\n"+"\n\n".join(blocks)+"\n"
    if write:open(path,"w",encoding="utf8").write(out)
    return True
