import re,sys,subprocess
S="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/F67"
files=[l.strip() for l in open(S+"/files.txt")]
names=set()
for f in files:
    d=subprocess.run(["git","diff","-U0","--",f],capture_output=True,text=True).stdout
    for l in d.split("\n"):
        if l.startswith("+") and not l.startswith("+++"):
            names.update(re.findall(r"export (?:declare )?(?:async )?(?:function|const|class|interface|type) (\w+)",l))
            names.update(re.findall(r"\b(\w+V1)\b",l))
names={n for n in names if len(n)>4}
hits=[]
for l in open(S+"/last-errors.txt"):
    m=re.match(r"(.*?)\((\d+),(\d+)\): error",l)
    if not m: continue
    try: src=open(m.group(1)).read().split("\n")[int(m.group(2))-1]
    except Exception: continue
    for n in names:
        if re.search(r"\b%s\b"%re.escape(n),src) or n in l: hits.append((n,l.strip()[:300])); break
print(len(hits),"errors touch changed names")
for h in hits[:int(sys.argv[1]) if len(sys.argv)>1 else 15]: print(h)
