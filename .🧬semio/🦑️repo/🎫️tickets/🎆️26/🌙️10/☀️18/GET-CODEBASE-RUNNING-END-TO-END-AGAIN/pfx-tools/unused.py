import re,sys
def leaves(tree):
    tree=tree.strip()
    if "{" not in tree:
        t=tree.split("::")[-1].strip()
        if " as " in tree:return [tree.split(" as ")[-1].strip()]
        return [t] if t not in("self","*") else ([] if t=="*" else [tree.split("::")[-2].strip()] )
    pre,rest=tree.split("{",1);body=rest.rsplit("}",1)[0]
    out=[];depth=0;cur=""
    for ch in body:
        if ch=="{":depth+=1
        if ch=="}":depth-=1
        if ch=="," and depth==0:out+=leaves(cur);cur=""
        else:cur+=ch
    if cur.strip():out+=leaves(cur)
    res=[]
    for l in out:res.append(l)
    return res
def report(path):
    s=open(path,encoding="utf-8").read()
    uses=list(re.finditer(r"(?m)^[ \t]*(?:pub(?:\([a-z]+\))?[ \t]+)?use[ \t]+(.+?);",s,flags=re.S))
    body=s
    for m in reversed(uses):body=body[:m.start()]+" "*(m.end()-m.start())+body[m.end():]
    unused=[]
    for m in uses:
        text=m.group(1)
        if text.startswith("{"):text=text
        for l in leaves(text):
            if not l or l=="_":continue
            if not re.search(r"(?<![A-Za-z0-9_])"+re.escape(l)+r"(?![A-Za-z0-9_])",body):unused.append(l)
    return unused
if __name__=="__main__":
    for p in sys.argv[1:]:print(p.split("subsets/")[-1][:60],report(p))
