import os,sys,hashlib,json,re
R="/Users/ueli/Documents/semio"
S=os.path.dirname(os.path.abspath(__file__))
L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
N=L+"/🕸️graph/📥️inference/📦️project-nodes"
def sha(b): return hashlib.sha256(b).hexdigest()[:16]
class Group:
    def __init__(self,name):
        self.name=name;self.dir=f"{S}/g-{name}";os.makedirs(self.dir,exist_ok=True);self.files={}
    def text(self,path):
        if path not in self.files:
            b=open(path,"rb").read();self.files[path]=[b.decode("utf8"),sha(b)]
        return self.files[path][0]
    def sub(self,path,old,new,count=1):
        t=self.text(path);c=t.count(old)
        assert c==count,f"{path[-60:]}: anchor found {c}x (want {count}): {old[:80]!r}"
        self.files[path][0]=t.replace(old,new)
    def save(self):
        man=[]
        for i,(p,(t,h)) in enumerate(self.files.items()):
            open(f"{self.dir}/{i}.new","w",encoding="utf8").write(t);man.append({"i":i,"path":p,"base":h})
        json.dump(man,open(f"{self.dir}/manifest.json","w"),ensure_ascii=False,indent=1)
