"""📏️ One line per component: total / name-section / code / data of its biggest core module."""
import sys, collections
def leb(b,i):
    r=0;s=0
    while True:
        x=b[i];i+=1;r|=(x&0x7f)<<s
        if not x&0x80:return r,i
        s+=7
def nm(b,i):
    n,i=leb(b,i);return b[i:i+n].decode("utf-8","replace"),i+n
def secs(b,s,e):
    i=s
    while i<e:
        sid=b[i];i+=1;sz,i=leb(b,i);yield sid,i,sz;i+=sz
for p in sys.argv[1:]:
    b=open(p,"rb").read()
    mods=[(o,s) for sid,o,s in secs(b,8,len(b)) if sid==1]
    if not mods: print(f"{p} no core module"); continue
    o,s=max(mods,key=lambda m:m[1])
    name=code=data=0; nfn=0
    for sid,oo,ss in secs(b,o+8,o+s):
        if sid==10:
            code=ss; j=oo; nfn,_=leb(b,j)
        elif sid==11: data=ss
        elif sid==0:
            n,_=nm(b,oo)
            if n=="name": name=ss
    print(f"{len(b):>11} tot | name {name:>11} ({100*name/len(b):5.1f}%) | code {code:>10} | data {data:>8} | fns {nfn:>7} | {p.split('/')[-1]}")
