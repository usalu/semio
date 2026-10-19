import re,sys
src=open(sys.argv[1]).read().split("\n")
if src[-1]=="": src.pop()
n=len(src)
# top-level block starts: lines at col0 not blank, not '}' or ')' closers, not continuing comment
starts=[]
i=0
incom=False
while i<n:
    l=src[i]
    if incom:
        if "*/" in l: incom=False
        i+=1; continue
    if l.startswith("/**") or l.startswith("/*"):
        s=i
        if "*/" not in l: incom=True
        starts.append(("doc",i)); i+=1; continue
    if re.match(r"^(export |async |function |class |const |let |type |interface |import |//|abstract )",l) or re.match(r"^[A-Za-z_]",l):
        starts.append(("code",i))
    i+=1
# merge doc+code into items
items=[];k=0
while k<len(starts):
    kind,i=starts[k]
    if kind=="doc" and k+1<len(starts) and starts[k+1][0]=="code" and all(not src[j].strip() for j in range(0,0)):
        items.append((i,starts[k+1][1])); k+=2
    else:
        items.append((i,i)); k+=1
for a,(i,c) in enumerate(items):
    end=(items[a+1][0]-1) if a+1<len(items) else n-1
    while end>c and not src[end].strip(): end-=1
    print(f"{i+1}-{end+1}\t{src[c][:100]}")
