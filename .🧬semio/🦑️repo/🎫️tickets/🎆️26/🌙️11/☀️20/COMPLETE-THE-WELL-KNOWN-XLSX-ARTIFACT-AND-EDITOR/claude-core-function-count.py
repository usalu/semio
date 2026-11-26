import sys,mmap
def leb(b,i):
    r=0;s=0
    while True:
        x=b[i];i+=1;r|=(x&0x7f)<<s;s+=7
        if not x&0x80:return r,i
def secs(b,i,end):
    while i<end:
        id=b[i];i+=1;n,i=leb(b,i);yield id,i,n;i+=n
f=open(sys.argv[1],'rb');b=mmap.mmap(f.fileno(),0,access=mmap.ACCESS_READ)
# component: header 8 bytes
for id,s,n in secs(b,8,len(b)):
    if id==1:
        # core module: header 8
        for id2,s2,n2 in secs(b,s+8,s+n):
            if id2==3:
                c,_=leb(b,s2);print("functions",c)
            if id2==10:print("code bytes",n2)
        break
