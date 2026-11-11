import re,sys,os
G=sys.argv[1];apply=sys.argv[2]=="apply"
std={"semio_framework_trace::HeapWitness","HeapWitness","semio_framework_trace_memory_testing::RequestedAllocator","test_allocation::RequestedAllocator","heap::RequestedAllocator","allocation::RequestedAllocator","RequestedAllocator"}
skip=("💾️allocator/🦀️.rs",)
rx=re.compile(r'((?:[ \t]*///[^\n]*\n)*(?:[ \t]*#\[cfg[^\n]*\]\s*\n)*)[ \t]*#\[global_allocator\]\s*(?:pub\s+)?static\s+(\w+)\s*:\s*([^=;]+?)\s*=\s*([^;]+);[ \t]*\n?')
log=[]
for f in open(G+"/alloc-files.txt").read().split("\n"):
    if not f or f.endswith(skip):continue
    s=open(f).read()
    ms=list(rx.finditer(s))
    if len(ms)!=s.count("#[global_allocator]") or any(m.group(3).replace(" ","") not in std for m in ms):
        log.append(("SKIP",f));continue
    n=s
    types=[m.group(3).replace(" ","") for m in ms]
    n=rx.sub("",s)
    for t in set(types):
        if "::" in t:
            alias=t.split("::")[0]
            if alias.startswith("semio_"):continue
            if len(re.findall(r'\b'+alias+r'\b',n))==1:
                n=re.sub(r'^[ \t]*(?:#\[cfg[^\n]*\]\s*\n[ \t]*)*use\s+[^;\n]*\bas\s+'+alias+r'\s*;[ \t]*\n','',n,flags=re.M)
        else:
            if len(re.findall(r'\b'+t+r'\b',n))==1:
                n=re.sub(r'^[ \t]*use\s+[^;\n]*\b'+t+r'\s*;[ \t]*\n','',n,flags=re.M)
    log.append(("EDIT",f,types))
    if apply:open(f,"w").write(n)
for l in log:print(*l)
