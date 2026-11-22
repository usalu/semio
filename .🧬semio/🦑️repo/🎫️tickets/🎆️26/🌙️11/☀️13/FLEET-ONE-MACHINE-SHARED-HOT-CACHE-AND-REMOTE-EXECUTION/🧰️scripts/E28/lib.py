import os,sys,hashlib,importlib.util,glob,json
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def sha(b):return hashlib.sha256(b).hexdigest()[:12]
class Plan:
    def __init__(self,root,apply=False,base=None):
        self.root=root;self.apply=apply;self.base=base;self.text={};self.orig={};self.new=set()
    def path(self,rel):return os.path.join(self.root,rel)
    def load(self,rel):
        if rel not in self.text:
            data=open(self.path(rel),"rb").read();self.orig[rel]=data;self.text[rel]=data.decode("utf8")
        return self.text[rel]
    def sub(self,rel,old,new,count=1):
        t=self.load(rel);n=t.count(old)
        if n!=count:raise SystemExit(f"ANCHOR {n}!={count} in {rel}: {old[:120]!r}")
        self.text[rel]=t.replace(old,new)
    def json(self,rel,change):
        t=self.load(rel);d=json.loads(t)
        if json.dumps(d,indent=2,ensure_ascii=False)+"\n"!=t:raise SystemExit("JSON does not round-trip: "+rel)
        change(d);self.text[rel]=json.dumps(d,indent=2,ensure_ascii=False)+"\n"
    def create(self,rel,text):
        if os.path.exists(self.path(rel)) and rel not in self.new:raise SystemExit("exists: "+rel)
        self.text[rel]=text;self.new.add(rel);self.orig.setdefault(rel,b"")
    def finish(self):
        rows=[]
        for rel,t in sorted(self.text.items()):
            data=t.encode("utf8");rows.append((rel,sha(self.orig[rel]) if self.orig[rel] else "new",sha(data)))
            if not self.apply:continue
            if self.base and self.orig[rel] and not os.path.exists(os.path.join(self.base,rel)):
                os.makedirs(os.path.dirname(os.path.join(self.base,rel)),exist_ok=True);open(os.path.join(self.base,rel),"wb").write(self.orig[rel])
            if self.base and not self.orig[rel]:
                os.makedirs(self.base,exist_ok=True);open(os.path.join(self.base,".new"),"a").write(rel+"\n")
            p=self.path(rel);os.makedirs(os.path.dirname(p),exist_ok=True)
            mode=os.stat(p).st_mode&0o777 if os.path.exists(p) else 0o644
            tmp=p+".e28.tmp";open(tmp,"wb").write(data);os.chmod(tmp,mode);os.replace(tmp,p)
        return rows
def run(root,apply,base=None):
    here=os.path.dirname(os.path.abspath(__file__));p=Plan(root,apply,base)
    for f in sorted(glob.glob(os.path.join(here,"edits","e*.py"))):
        spec=importlib.util.spec_from_file_location(os.path.basename(f)[:-3],f);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);m.plan(p,L)
    return p.finish()
def reset(root,base):
    import shutil
    for dp,dn,fn in os.walk(base):
        for f in fn:
            if f==".new" and dp==base:continue
            s=os.path.join(dp,f);r=os.path.relpath(s,base);shutil.copyfile(s,os.path.join(root,r))
    nf=os.path.join(base,".new")
    if os.path.exists(nf):
        for r in set(open(nf).read().split("\n")):
            if r and os.path.exists(os.path.join(root,r)):os.remove(os.path.join(root,r))
        os.remove(nf)

if __name__=="__main__":
    root=sys.argv[1];apply="--apply" in sys.argv
    if "--reset" in sys.argv:
        reset(root,[a.split("=",1)[1] for a in sys.argv if a.startswith("--base=")][0]);sys.exit(0)
    base=[a.split("=",1)[1] for a in sys.argv if a.startswith("--base=")]
    for r in run(root,apply,base[0] if base else None):print(*r)
