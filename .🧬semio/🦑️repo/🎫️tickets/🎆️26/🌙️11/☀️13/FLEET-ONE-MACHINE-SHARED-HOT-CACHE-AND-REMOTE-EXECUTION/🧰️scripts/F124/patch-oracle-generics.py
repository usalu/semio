import os
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/🧪️tests/🐍️oracle/🐍️.py"
s=open(p,encoding="utf8").read()
if "def generic_end(" not in s:
    old='''def expression_range(src,toks,match,start):
    k=start
    while k<len(toks):
        token=toks[k]
        if token[0]=="p" and token[1] in OPEN and k in match:k=match[k]+1;continue
'''
    new='''def generic_end(toks,match,k):
    if k<1 or toks[k-1][0]!="id" or not toks[k-1][1][:1].isupper():return None
    depth=0;i=k
    while i<len(toks) and i<k+400:
        x=toks[i]
        if x[0]=="p" and x[1] in OPEN and i in match:i=match[i]+1;continue
        if x[0]=="p" and x[1]=="<":depth+=1
        elif x[0]=="p" and x[1]==">":
            depth-=1
            if depth==0:return i+1
        elif not (x[0] in ("id","str","num") or x[1] in TYPE_TOKENS or x[1] in (":",";","=>")):return None
        i+=1
    return None
def expression_range(src,toks,match,start):
    k=start
    while k<len(toks):
        token=toks[k]
        if token[0]=="p" and token[1] in OPEN and k in match:k=match[k]+1;continue
        if token[0]=="p" and token[1]=="<":
            g=generic_end(toks,match,k)
            if g is not None:k=g;continue
'''
    assert s.count(old)==1;s=s.replace(old,new)
tmp=p+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,p);print("generics ok")
