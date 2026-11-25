import sys,mmap,re,subprocess
f=open(sys.argv[1],"rb");m=mmap.mmap(f.fileno(),0,access=mmap.ACCESS_READ)
want=[int(x) for x in sys.argv[2:]]
def leb(p):
    r=0;s=0
    while True:
        b=m[p];p+=1;r|=(b&0x7f)<<s;s+=7
        if b<0x80:return r,p
p=8;nfi=0;locals_={};names={};imports=0
while p<len(m):
    sid=m[p];p+=1;size,p=leb(p);end=p+size
    if sid==2:
        n,q=leb(p)
        for _ in range(n):
            l,q=leb(q);q+=l;l,q=leb(q);q+=l;k=m[q];q+=1
            if k==0:
                _,q=leb(q);imports+=1
            elif k==1: q+=1;fl=m[q];q+=1;_,q=leb(q); (leb(q)[1] if fl&1 else 0); q=leb(q)[1] if fl&1 else q
            elif k==2:
                fl=m[q];q+=1;_,q=leb(q)
                if fl&1:_,q=leb(q)
            elif k==3:q+=2
    if sid==10:
        n,q=leb(p)
        for i in range(n):
            fs,q2=leb(q);fe=q2+fs;nl,r=leb(q2);tot=0
            for _ in range(nl):
                c,r=leb(r);r+=1;tot+=c
            locals_[i+imports]=(tot,fs);q=fe
    if sid==0:
        nl,q=leb(p);nm=m[q:q+nl].decode(errors="replace");q+=nl
        if nm=="name":
            while q<end:
                sub=m[q];q+=1;sz,q=leb(q);se=q+sz
                if sub==1:
                    c,q=leb(q)
                    for _ in range(c):
                        i,q=leb(q);l,q=leb(q);names[i]=m[q:q+l].decode(errors="replace");q+=l
                q=se
    p=end
print("imports",imports,"names",len(names))
rows=sorted(locals_.items(),key=lambda kv:-kv[1][0])[:12]
def dem(s):
    try:return subprocess.run(["rustfilt"],input=s,capture_output=True,text=True).stdout.strip() or s
    except Exception:return s
for i,(t,fs) in rows:
    print(i,"locals",t,"body",fs,names.get(i,"?")[:400])
for w in [x+imports for x in want]:
    print("want",w,locals_.get(w),names.get(w,"?")[:400])
