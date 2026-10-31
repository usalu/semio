import json,os,re,sys,glob
R="/Users/ueli/Documents/semio"
COMP=R+"/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions"
PKG="📦️packages/🦀️rust"
def create(src_dir,new_dir,old,new,old_id,new_id,part,mount_lines,corpus_files,doc_roots,after_member):
    src=COMP+"/"+src_dir;dst=COMP+"/"+new_dir
    assert not os.path.exists(dst)
    corpus="".join(open(f).read() for f in corpus_files)+mount_lines
    cargo=open(src+"/"+PKG+"/Cargo.toml").read()
    def keep(line):
        mm=re.match(r"^([\w\-]+)\s*=",line)
        if not mm:return True
        names={mm.group(1).replace("-","_")}
        p=re.search(r'package = "([^"]+)"',line)
        if p:names.add(p.group(1).replace("-","_"))
        return True
    out=[];sec=""
    for l in cargo.split("\n"):
        if l.startswith("["):sec=l
        if sec in("[dependencies]","[dev-dependencies]") and not l.startswith("[") and l.strip() and not keep(l):continue
        out.append(l)
    c="\n".join(out)
    c=c.replace('name = "%s"'%old,'name = "%s"'%new).replace('id = "%s"'%old_id,'id = "%s"'%new_id)
    c=re.sub(r"\(part \d+\)","(part %d)"%part,c)
    os.makedirs(dst+"/"+PKG)
    open(dst+"/"+PKG+"/Cargo.toml","w").write(c)
    open(dst+"/"+PKG+"/📋️project.json","w").write('{\n  "name": "@semio-tech/%s-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n'%new_id)
    return dst
def register(new_dir,new_id,after_dir):
    rel="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/%s/%s"%(new_dir,PKG)
    line='    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/%s/%s",\n'%(after_dir,PKG)
    ct=open(R+"/Cargo.toml").read();assert line in ct and rel not in ct
    open(R+"/Cargo.toml","w").write(ct.replace(line,line+line.replace(after_dir,new_dir),1))
    fl=line.replace("🧰️framework/","",1)
    p=R+"/🧰️framework/Cargo.toml";ft=open(p).read();assert fl in ft
    open(p,"w").write(ft.replace(fl,fl+fl.replace(after_dir,new_dir),1))
    op=R+"/📋️build-owners.json";o=open(op).read()
    oname="@semio-tech/%s-rs"%new_id
    row='    {\n      "name": "%s",\n      "root": "%s",\n      "manifest": "%s/📋️project.json",\n      "packages": []\n    },\n'%(oname,rel,rel)
    names=[(m.start(),m.group(1)) for m in re.finditer(r'    \{\n      "name": "(@semio-tech/[^"]+)"',o)]
    assert oname not in o
    pos=next(p_ for p_,n in names if n>oname)
    o=o[:pos]+row+o[pos:];json.loads(o);open(op,"w").write(o)
