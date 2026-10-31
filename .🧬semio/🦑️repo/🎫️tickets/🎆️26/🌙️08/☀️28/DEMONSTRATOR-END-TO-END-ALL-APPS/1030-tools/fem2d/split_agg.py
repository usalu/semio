import sys,os,re
# usage: split_agg.py <crate-dir> <target-name> <chunk-mods>
crate,name,k=sys.argv[1],sys.argv[2],int(sys.argv[3])
ct=open(os.path.join(crate,"Cargo.toml"),encoding="utf8").read()
m=re.search(r'\[\[test\]\]\nname = "%s"\npath = "([^"]+)"\n'%re.escape(name),ct)
rel=m.group(1);agg=os.path.normpath(os.path.join(crate,rel))
lines=open(agg,encoding="utf8").read().split("\n")
head=[];pairs=[];i=0
while i<len(lines):
    if lines[i].startswith("#[path") and i+1<len(lines) and lines[i+1].startswith("mod "):
        pairs.append((lines[i],lines[i+1]));i+=2
    else:
        if pairs and lines[i].strip(): sys.exit("non-mod line after mods: "+lines[i][:80])
        if not pairs: head.append(lines[i])
        i+=1
chunks=[pairs[j:j+k] for j in range(0,len(pairs),k)]
d=os.path.dirname(agg);parent=os.path.dirname(d);base=os.path.basename(d)
mm=re.match(r'^(.*?)(?:-(\d+))?$',base);stem=mm.group(1);n=int(mm.group(2) or 1)
tm=re.match(r'^(.*?)(?:_(\d+))?$',name);tstem=tm.group(1);tn=int(tm.group(2) or 1)
out=[];add=""
for ci,ch in enumerate(chunks):
    body="\n".join(head).rstrip("\n")+"\n"+"\n".join(l for p in ch for l in p)+"\n"
    if ci==0:
        open(agg,"w",encoding="utf8").write(body);continue
    while True:
        n+=1;nd=os.path.join(parent,"%s-%d"%(stem,n))
        if not os.path.exists(nd):break
    while True:
        tn+=1;nm="%s_%d"%(tstem,tn)
        if ("name = \"%s\""%nm) not in ct and ("name = \"%s\""%nm) not in add:break
    os.makedirs(nd);open(os.path.join(nd,"🦀️.rs"),"w",encoding="utf8").write(body)
    add+='\n[[test]]\nname = "%s"\npath = "%s"\n'%(nm,os.path.relpath(os.path.join(nd,"🦀️.rs"),crate))
    print("new",nm,nd)
marker=m.group(0)
ct=ct.replace(marker,marker+add[1:],1) if add else ct
open(os.path.join(crate,"Cargo.toml"),"w",encoding="utf8").write(ct)
