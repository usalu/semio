import sys,mmap
f=open(sys.argv[1],"rb");m=mmap.mmap(f.fileno(),0,access=mmap.ACCESS_READ)
def leb(p):
    r=0;s=0
    while True:
        b=m[p];p+=1;r|=(b&0x7f)<<s;s+=7
        if b<0x80:return r,p
p=8;names={}
while p<len(m):
    sid=m[p];p+=1;size,p=leb(p);end=p+size
    if sid==10:
        n,q=leb(p);res=[]
        for i in range(n):
            fs,q2=leb(q);fe=q2+fs;nl,r=leb(q2);tot=0
            for _ in range(nl):
                c,r=leb(r);r+=1;tot+=c
            res.append((tot,i,fs,q2-p))
            q=fe
        res.sort(reverse=True)
        print("functions",n)
        for t in res[:8]:print("locals",t[0],"index",t[1],"bodybytes",t[2],"offset_in_code_section",t[3])
    if sid==0:
        nl,q=leb(p);nm=m[q:q+nl].decode();
        if nm=="name":names["p"]=(q+nl,end)
    p=end
print("namesection" if names else "no-name-section")
