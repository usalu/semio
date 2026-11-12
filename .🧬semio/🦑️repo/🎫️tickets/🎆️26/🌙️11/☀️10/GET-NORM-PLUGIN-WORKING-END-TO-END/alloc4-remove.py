import re,sys,json
S=sys.argv[1];apply=sys.argv[2]=="apply"
A="semio_framework_value_testing_allocator"
std={"semio_framework_trace::HeapWitness","HeapWitness","semio_framework_trace_memory_testing::RequestedAllocator","test_allocation::RequestedAllocator","heap::RequestedAllocator","allocation::RequestedAllocator","RequestedAllocator","rank_allocation::RequestedAllocator"}
rx=re.compile(r'(?m)^((?:[ \t]*///[^\n]*\n)*)((?:[ \t]*#\[cfg\(test\)\][ \t]*\n?)*)[ \t]*#\[global_allocator\]\s*(?:pub\s+)?static\s+(\w+)\s*:\s*([^=;]+?)\s*=\s*([^;]+);[ \t]*\n?')
out=[]
for f in open(S).read().split("\n"):
    if not f or f.endswith("💾️allocator/🦀️.rs"):continue
    s=open(f,encoding="utf-8").read()
    ms=list(rx.finditer(s))
    if len(ms)!=len(re.findall(r"(?m)^(?![ \t]*//)[^\n]*#\[global_allocator\]",s)) or any(m.group(4).replace(" ","") not in std for m in ms):
        out.append(("SKIP",f));continue
    types=[m.group(4).replace(" ","") for m in ms]
    linked=A in s
    first=[True]
    def sub(m):
        if linked or not first[0]:return ""
        first[0]=False
        return m.group(2)+"use "+A+" as _;\n"
    n=rx.sub(sub,s)
    for t in set(types):
        if "::" in t:
            alias=t.split("::")[0]
            if alias.startswith("semio_") or alias=="crate":continue
            if len(re.findall(r'\b'+alias+r'\b',n))==1:
                n=re.sub(r'^[ \t]*(?:#\[cfg[^\n]*\]\s*\n[ \t]*)*use\s+[^;\n]*\bas\s+'+alias+r'\s*;[ \t]*\n','',n,flags=re.M)
        else:
            if len(re.findall(r'\b'+t+r'\b',n))==1:
                n=re.sub(r'^[ \t]*(?:#\[cfg[^\n]*\]\s*\n[ \t]*)?use\s+[^;\n]*\b'+t+r'\s*;[ \t]*\n','',n,flags=re.M)
    out.append(("EDIT",f,types))
    if apply:open(f,"w",encoding="utf-8").write(n)
for o in out:print(*o)
