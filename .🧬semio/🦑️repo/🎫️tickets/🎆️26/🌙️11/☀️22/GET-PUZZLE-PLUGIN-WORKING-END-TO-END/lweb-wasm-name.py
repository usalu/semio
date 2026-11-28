import sys,mmap
def leb(d,p):
    v=s=0
    while True:
        b=d[p];p+=1;v|=(b&127)<<s
        if b<128:return v,p
        s+=7
f=open(sys.argv[1],'rb');d=mmap.mmap(f.fileno(),0,access=mmap.ACCESS_READ)
target=int(sys.argv[2])
p=8;imports=0
while p<len(d):
    sec=d[p];size,q=leb(d,p+1);end=q+size
    if sec==2:
        n,r=leb(d,q)
        for _ in range(n):
            l,r=leb(d,r);r+=l;l,r=leb(d,r);r+=l;k=d[r];r+=1
            if k==0:imports+=1;_,r=leb(d,r)
            elif k==1:r+=1;fl=d[r];r+=1;_,r=leb(d,r);
            elif k==2:fl=d[r];r+=1;_,r=leb(d,r)
            elif k==3:r+=2
            if k==1 and fl&1:_,r=leb(d,r)
            if k==2 and fl&1:_,r=leb(d,r)
    if sec==0:
        l,r=leb(d,q);nm=d[r:r+l]
        print("custom",bytes(nm),size)
        if nm==b'name':
            r+=l
            while r<end:
                sid=d[r];sz,r2=leb(d,r+1)
                if sid==1:
                    n,t=leb(d,r2)
                    for _ in range(n):
                        idx,t=leb(d,t);ln,t=leb(d,t)
                        if idx==target+imports:print(idx,bytes(d[t:t+ln]).decode(errors='replace'))
                        t+=ln
                r=r2+sz
    p=end
print("imports",imports)
