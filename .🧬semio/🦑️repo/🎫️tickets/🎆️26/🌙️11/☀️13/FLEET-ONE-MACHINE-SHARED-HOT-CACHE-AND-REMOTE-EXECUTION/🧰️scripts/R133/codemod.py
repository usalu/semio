import re,sys,os,collections
sys.path.insert(0,os.path.dirname(__file__))
from scan import files,ROOT
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
FN=re.compile(r'(?<![\w$.])((?:\w+\.)?(?:join|resolve))\(')
LIT=re.compile(r'^(["\'])(\.cache|\.nx)(?:/[^"\']*)?\1$')
def split_args(t,start):
    i=start;depth=0;args=[];cur=start;q=None
    while i<len(t):
        c=t[i]
        if q:
            if c=="\\":i+=2;continue
            if c==q:q=None
            elif q=="`" and c=="$" and t[i+1:i+2]=="{":
                d=1;i+=2
                while i<len(t) and d:
                    d+= (t[i]=="{")-(t[i]=="}");i+=1
                continue
        elif c in "\"'`":q=c
        elif c in "([{":depth+=1
        elif c in ")]}":
            if depth==0:
                args.append((cur,i));return args,i
            depth-=1
        elif c=="," and depth==0:
            args.append((cur,i));cur=i+1
        i+=1
    return None,None
def candidates(t):
    out=[]
    for m in FN.finditer(t):
        args,end=split_args(t,m.end())
        if not args or len(args)<2:continue
        a1=t[args[1][0]:args[1][1]].strip()
        if LIT.match(a1): out.append((m.start(),m.end(),m.group(1),t[args[0][0]:args[0][1]].strip(),a1,args))
    return out
if __name__=="__main__":
    c=collections.Counter()
    for f in files():
        t=open(f,encoding="utf8").read()
        for s,e,fn,a0,a1,args in candidates(t):
            c[(fn,a0)]+=1
    for k,v in sorted(c.items(),key=lambda x:-x[1]):print(v,k)
