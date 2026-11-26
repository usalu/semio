import sys,re,os,tempfile
d=sys.argv[1]; new=sys.stdin.read().strip("\n").split("\n")
m=d[:-6]+".mjs"
ms=open(m).read().split("\n")
src=open(d).read()
mark="/** 🧾️ Declared from the script by `repo typecheck`"
i=src.find(mark)
if i<0: sys.exit("no marker")
head=src[:i]
out=[];prev_doc=False
for l in new:
    if l.startswith("/**"): out.append(l); prev_doc=True; continue
    mm=re.match(r"export (?:declare )?(?:async )?(?:function|const|class|default function) ?(\w+)",l)
    if mm and not prev_doc:
        n=mm.group(1)
        for k,x in enumerate(ms):
            if re.search(r"(function\*? |const |class )%s\b"%n,x) and k>0 and ms[k-1].startswith("/**") and ms[k-1].rstrip().endswith("*/"):
                out.append(ms[k-1]); break
    prev_doc=False; out.append(l)
res=head+"\n".join(out)+"\n"
fd,tmp=tempfile.mkstemp(dir=os.path.dirname(d),prefix=".f67.")
os.write(fd,res.encode());os.close(fd);os.replace(tmp,d)
print("ok",len(out))
