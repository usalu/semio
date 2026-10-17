import sys,re,os,math
# 🪓️ splittarget.py <manifest> <test-name> <parts> : splits a [[test]] target that is a pure #[path] mount list into <parts> sibling targets (dir suffix -2.. , test name suffix _2..), losslessly.
R="/Users/ueli/Documents/semio"
man=os.path.join(R,sys.argv[1]); name=sys.argv[2]; parts=int(sys.argv[3])
mt=open(man).read()
m=re.search(r'\[\[test\]\]\nname = "%s"\npath = "([^"]+)"\n'%re.escape(name),mt)
assert m,"stanza"
rel=m.group(1)
tf=os.path.normpath(os.path.join(os.path.dirname(man),rel))
lines=open(tf).read().split("\n")
first=next(i for i,l in enumerate(lines) if l.startswith("#[path"))
header=lines[:first]
body=[l for l in lines[first:]]
blocks=[];i=0
while i<len(body):
    if not body[i].strip(): i+=1; continue
    assert body[i].startswith("#[path") and re.match(r'(pub )?mod \w+;',body[i+1]),("not a pure mount list at",body[i:i+2])
    blocks.append(body[i:i+2]); i+=2
n=len(blocks); size=math.ceil(n/parts)
chunks=[blocks[k*size:(k+1)*size] for k in range(parts)]
d=os.path.dirname(tf); base=os.path.basename(d); root=os.path.dirname(d)
newstanzas=""
for k,ch in enumerate(chunks):
    if not ch: continue
    text="\n".join(header)+"\n"+"\n".join(l for b in ch for l in b)+"\n"
    if k==0: open(tf,"w").write(text); continue
    nd=os.path.join(root,f"{base}-{k+1}"); os.makedirs(nd,exist_ok=False)
    open(os.path.join(nd,os.path.basename(tf)),"w").write(text)
    newrel=rel.replace(f"/{base}/",f"/{base}-{k+1}/")
    assert newrel!=rel
    newstanzas+=f'\n[[test]]\nname = "{name}_{k+1}"\npath = "{newrel}"\n'
anchor=m.group(0)
mt=mt.replace(anchor,anchor+newstanzas)
open(man,"w").write(mt)
print(f"split {n} mounts of {name} into {sum(1 for c in chunks if c)} targets")
