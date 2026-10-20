import re,sys,collections
lines=open(sys.argv[1],encoding="utf8").read().split("\n")
start=lines.index("Call graph:")+1
end=next(i for i,l in enumerate(lines) if l.startswith("Total number in stack"))
pat=re.compile(r'^([ +!:|]*)(\d+) (.*)$')
nodes=[];thread=None
for l in lines[start:end]:
    m=pat.match(l)
    if not m: continue
    prefix,count,text=m.groups();depth=len(prefix)
    if depth==4 and "Thread_" in text: thread=text
    nodes.append([depth,int(count),text,thread])
cs=[0]*len(nodes);st=[]
for i,n in enumerate(nodes):
    while st and nodes[st[-1]][0]>=n[0]: st.pop()
    if st: cs[st[-1]]+=n[1]
    st.append(i)
per=collections.defaultdict(collections.Counter)
for i,n in enumerate(nodes):
    s=n[1]-cs[i]
    if s>0:
        a=re.search(r'\+ (0x[0-9a-f]+)',n[2]);key=a.group(1) if a else re.sub(r'\s+\(in .*','',n[2])
        per[n[3]][key]+=s
main=[t for t in per if "main-thread" in t][0]
helpers=[t for t in per if "Heap Helper" in t]
gc=set()
for h in helpers:
    for k in per[h]:
        if k.startswith("0x"): gc.add(k)
mc=per[main];tot=sum(mc.values());wait=sum(v for k,v in mc.items() if k.startswith("__"))
g=sum(v for k,v in mc.items() if k in gc);other=sum(v for k,v in mc.items() if k.startswith("0x") and k not in gc)
print(sys.argv[1],"main total",tot,"wait",wait,"gc-leaf",g,"other native",other,"jit/other",tot-wait-g-other)
hb=sum(v for h in helpers for k,v in per[h].items() if k.startswith("0x"));print(" helpers busy samples (3 threads)",hb,"of",3*tot)
