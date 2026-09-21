import re,os,sys
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
G=os.path.join(T,"🗑️generated")
tag=sys.argv[1]
p=os.path.join(G,f"fp7-{tag}-serial.txt")
txt=open(p,encoding='utf-8',errors='replace').read()
names=sorted(set(m.group(1) for m in re.finditer(r"^test (\S+) \.\.\. FAILED$", txt, re.M)))
open(os.path.join(G,f"fp7-{tag}.names"),"w",encoding='utf-8').write("\n".join(names)+"\n")
blocks=re.split(r"\n---- (\S+) stdout ----\n", txt)
res={}
for i in range(1,len(blocks),2):
    name=blocks[i]; body=blocks[i+1].split("\nfailures:\n")[0]
    lines=[l for l in body.strip().split("\n") if l.strip()]
    msg=""
    for j,l in enumerate(lines):
        if "panicked at" in l:
            msg=" | ".join(lines[j+1:j+3])[:240]; break
    if not msg: msg=" | ".join(lines[:2])[:240]
    res[name]=msg
with open(os.path.join(G,f"fp7-{tag}-panics.txt"),"w",encoding='utf-8') as w:
    for n in names: w.write(f"{n}\n    {res.get(n,'?')}\n")
prev=sys.argv[2] if len(sys.argv)>2 else None
if prev:
    old=set(open(os.path.join(G,f"fp7-{prev}.names"),encoding='utf-8').read().split())
    new=set(names)
    print("GREEN:", *sorted(x.split("::")[-1] for x in old-new), sep="\n  ")
    print("NEWRED:", *sorted(x.split("::")[-1] for x in new-old), sep="\n  ")
print("total failed:", len(names))
