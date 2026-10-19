import sys
def rep(f,old,new,n=1):
    t=open(f,encoding="utf-8").read()
    c=t.count(old)
    assert c==n,(f,old,c,n)
    open(f,"w",encoding="utf-8").write(t.replace(old,new))
