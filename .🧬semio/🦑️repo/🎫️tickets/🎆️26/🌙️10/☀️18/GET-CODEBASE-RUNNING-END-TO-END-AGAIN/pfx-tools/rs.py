import re,sys
SUBSETS="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/"
SNAP="🚪️io/🪶️sqlite/📸️snapshot/"
def native(domain):return SUBSETS+domain+"/"+SNAP+"🛬️native/🦀️.rs"
def skip(s,i):
    c=s[i]
    if s.startswith("//",i):
        j=s.find("\n",i);return len(s) if j<0 else j
    if s.startswith("/*",i):
        j=s.find("*/",i);return j+2
    if c=='"':
        j=i+1
        while s[j]!='"':
            j+=2 if s[j]=="\\" else 1
        return j+1
    if c=="'":
        m=re.match(r"'(\\.|\\u\{[0-9a-fA-F]+\}|\\x[0-9a-fA-F]{2}|[^\\'])'",s[i:])
        if m:return i+m.end()
    return None
def span(s,name,start=0):
    m=None
    for m in re.finditer(r"(?m)^[ \t]*(pub(\([a-z]+\))?[ \t]*)?(async[ \t]+)?fn[ \t]+"+re.escape(name)+r"\b",s):
        if m.start()>=start:break
    else:
        m=None
    if m is None:raise SystemExit("fn not found: "+name)
    a=m.start()
    while True:
        prev=s.rfind("\n",0,a-1)
        line=s[prev+1:a-1] if a>0 else ""
        if line.strip().startswith("///"):a=prev+1
        else:break
    i=s.index("{",m.end())
    # skip generics/where braces: first '{' after signature is body unless signature has braces in types (const generics) - assume not
    depth=0
    while True:
        k=skip(s,i)
        if k is not None:i=k;continue
        c=s[i]
        if c=="{":depth+=1
        elif c=="}":
            depth-=1
            if depth==0:break
        i+=1
    b=i+1
    if b<len(s) and s[b]=="\n":b+=1
    return a,b
def replace(path,name,new,start=0):
    s=open(path,encoding="utf-8").read()
    a,b=span(s,name,start)
    s=s[:a]+new.rstrip("\n")+"\n"+s[b:]
    open(path,"w",encoding="utf-8").write(s)
def get(path,name):
    s=open(path,encoding="utf-8").read();a,b=span(s,name);return s[a:b]
if __name__=="__main__":
    print(get(sys.argv[1],sys.argv[2]))
