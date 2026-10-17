import os,re,sys,json,regex
ROOT="/Users/ueli/Documents/semio"
G=os.environ["G"]
added=[l[2:].rstrip("\n") for l in open(G+"/cpf-diff-name-status.txt",encoding="utf-8") if l.startswith("A\t")]
unt=[l.rstrip("\n") for l in open(G+"/cpf-untracked.txt",encoding="utf-8")]
cands=sorted(set(p for p in added+unt if re.search(r"\.(rs|ts|tsx)$",p) and not p.endswith(".d.ts") and "🤖️generated" not in p and not p.startswith(".🧬semio/") and not p.startswith("Users/")))
MARK=regex.compile(r"^(?:\p{Extended_Pictographic}|\p{So}|\p{Sm}|\p{Regional_Indicator}|[0-9#*]️?⃣)")
res=[]
for p in cands:
    try: t=open(os.path.join(ROOT,p),encoding="utf-8").read()
    except Exception as e: continue
    lines=t.split("\n")
    i=0
    while i<len(lines) and (lines[i].strip()=="" or lines[i].startswith("#!") or lines[i].startswith("/// <reference") or re.match(r"^//\s*@vitest",lines[i])): i+=1
    # shebang
    first=lines[i] if i<len(lines) else ""
    kind=None
    if p.endswith(".rs"):
        m=re.match(r"^\s*//[!/]\s?(.*)$",first)
        if m and not first.lstrip().startswith("////"):
            kind="doc-ok" if MARK.match(m.group(1)) else "doc-noemoji"
        elif first.lstrip().startswith("/**") or first.lstrip().startswith("/*!"):
            kind="doc-ok"
        elif first.lstrip().startswith("//"): kind="plain-comment"
        else: kind="none"
    else:
        s=first.lstrip()
        if s.startswith("/**") and not s.startswith("/**/"):
            c=s[3:].strip()
            if not c and i+1<len(lines): c=re.sub(r"^\s*\*?\s*","",lines[i+1])
            kind="doc-ok" if MARK.match(c) else "doc-noemoji"
        elif s.startswith("//"): kind="plain-comment"
        elif s.startswith("/*"): kind="plain-comment"
        else: kind="none"
    res.append((kind,p))
import collections
print(collections.Counter(k for k,_ in res), len(cands))
json.dump(res,open(G+"/cpf-docscan.json","w"),ensure_ascii=False)
