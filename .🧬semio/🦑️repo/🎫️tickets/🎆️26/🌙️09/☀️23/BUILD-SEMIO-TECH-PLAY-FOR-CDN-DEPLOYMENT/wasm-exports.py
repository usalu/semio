import sys,mmap
def leb(b,i):
    r=0;s=0
    while True:
        x=b[i];i+=1;r|=(x&0x7f)<<s;s+=7
        if x<0x80:return r,i
def sections(b,i):
    n=len(b)
    while i<n:
        sid=b[i];size,j=leb(b,i+1);yield sid,j,j+size;i=j+size
def core(b,a,e):
    types=[];funcs=[];exports=[];imports=0;codesz=0
    for sid,s,t in sections(b,a+8):
        if sid==1:
            c,i=leb(b,s)
            for _ in range(c):
                assert b[i]==0x60;i+=1;p,i=leb(b,i);ps=list(b[i:i+p]);i+=p;r,i=leb(b,i);rs=list(b[i:i+r]);i+=r;types.append((ps,rs))
        elif sid==2:
            c,i=leb(b,s)
            for _ in range(c):
                l,i=leb(b,i);i+=l;l,i=leb(b,i);i+=l;k=b[i];i+=1
                if k==0:ti,i=leb(b,i);funcs.append(ti);imports+=1
                elif k==1:
                    i+=1;f=b[i];i+=1;_,i=leb(b,i)
                    if f&1:_,i=leb(b,i)
                elif k==2:
                    f=b[i];i+=1;_,i=leb(b,i)
                    if f&1:_,i=leb(b,i)
                elif k==3:i+=2
                elif k==4:i+=1;_,i=leb(b,i)
        elif sid==3:
            c,i=leb(b,s)
            for _ in range(c):
                ti,i=leb(b,i);funcs.append(ti)
        elif sid==7:
            c,i=leb(b,s)
            for _ in range(c):
                l,i=leb(b,i);nm=bytes(b[i:i+l]).decode('latin1');i+=l;k=b[i];i+=1;idx,i=leb(b,i);exports.append((nm,k,idx))
        elif sid==10:codesz=t-s
    return types,funcs,exports,codesz
f=open(sys.argv[1],'rb');b=mmap.mmap(f.fileno(),0,access=mmap.ACCESS_READ)
k=0
for sid,s,t in sections(b,8):
    if sid==1:
        k+=1
        ty,fn,ex,cs=core(b,s,t)
        owned=[(n,ty[fn[i]]) for n,kd,i in ex if kd==0 and n.startswith('semio_owned')]
        print('core module',k,'bytes',t-s,'code',cs,'exports',len(ex),'owned',len(owned))
        for n,sig in owned:
            if n in('semio_owned_describe_v1','semio_owned_checkpoint_v1','semio_owned_alloc_v1','semio_owned_restore_v1','semio_owned_poll_v1','semio_owned_channel_version_v1'):print(' ',n,sig)
