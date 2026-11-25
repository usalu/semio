import re,subprocess,sys
root="/Users/ueli/Documents/semio"
NEW="semio-s-artifact-stdio-xml-snapshot-document"
DEPS=["semio-framework-pack-json","semio-framework-value","semio-framework-value-derive","semio-framework-value-native-owned","semio-framework-value-persistent"]
def edit(text):
    pk=re.split(r"(?m)^(?=\[\[package\]\]\n)",text)
    head=pk[0]; blocks=pk[1:]
    def name(b): return re.search(r'name = "([^"]+)"',b).group(1)
    def ver(b): return re.search(r'version = "([^"]+)"',b).group(1)
    names={}
    for b in blocks: names.setdefault(name(b),[]).append(ver(b))
    if any(name(b)==NEW for b in blocks): return text
    out=[]
    for b in blocks:
        if name(b)=="semio-s-artifact-stdio-xml-snapshot":
            m=re.search(r"dependencies = \[\n(.*?)\n\]",b,re.S)
            items=[l.strip().strip(",").strip('"') for l in m.group(1).split("\n")]
            idx=next((i for i,x in enumerate(items) if x.split(" ")[0]>NEW),len(items)); items.insert(idx,NEW)
            b=b[:m.start(1)]+"\n".join(' "%s",'%i for i in items)+b[m.end(1):]
        out.append(b)
    deps=[]
    for d in DEPS:
        vs=names.get(d,[])
        if len(vs)>1:
            vs=[v for v in vs if v.startswith("0.39")] if d=="quick-xml" else [sorted(vs,key=lambda v:[int(x) for x in re.findall(r"\d+",v)])[-1]]
            deps.append("%s %s"%(d,vs[0]))
        else: deps.append(d)
    blk='[[package]]\nname = "%s"\nversion = "0.1.0"\ndependencies = [\n%s\n]\n\n'%(NEW,"\n".join(' "%s",'%d for d in deps))
    res=[];placed=False
    for b in out:
        if not placed and name(b)>NEW: res.append(blk);placed=True
        res.append(b)
    if not placed:
        if not res[-1].endswith("\n\n"): res[-1]=res[-1].rstrip("\n")+"\n\n"
        res.append(blk.rstrip("\n")+"\n")
    return head+"".join(res)
if __name__=="__main__":
    for rel in sys.argv[1:]:
        base=subprocess.run(["git","show","HEAD:./"+rel],cwd=root,capture_output=True).stdout.decode()
        new=edit(base)
        open(root+"/"+rel,"w").write(new)
