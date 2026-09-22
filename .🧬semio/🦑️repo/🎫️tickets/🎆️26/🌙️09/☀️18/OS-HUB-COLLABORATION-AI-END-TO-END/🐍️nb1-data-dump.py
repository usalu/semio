"""🧾️ Data-segment census of a component's biggest core module: segment count, sizes, and a preview of the largest."""
import sys
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
b=open(sys.argv[1],"rb").read()
o,s=max([(o,s) for sid,o,s in secs(b,8,len(b)) if sid==1],key=lambda m:m[1])
for sid,oo,ss in secs(b,o+8,o+s):
    if sid!=11: continue
    j=oo; cnt,j=leb(b,j)
    print(f"data section {ss} bytes, {cnt} segments")
    segs=[]
    for _ in range(cnt):
        flag,j=leb(b,j)
        if flag==0:
            # expr
            while b[j]!=0x0b: j+=1
            j+=1
        elif flag==2:
            _mi,j=leb(b,j)
            while b[j]!=0x0b: j+=1
            j+=1
        n,j=leb(b,j)
        segs.append((n,b[j:j+n])); j+=n
    segs.sort(key=lambda x:-x[0])
    print(f"largest segment {segs[0][0]} B; total {sum(x[0] for x in segs)} B")
    for n,d in segs[:5]:
        print(f"\n-- segment {n} B, first 400 printable chars --")
        t="".join(chr(c) if 32<=c<127 else "." for c in d[:400])
        print(t)
    break
