"""✂️ Rewrite a component with the `name` custom section removed from its core modules — the exact
byte count `strip = "symbols"` is expected to produce, computed without rebuilding."""
import sys
def leb(b,i):
    r=0;s=0
    while True:
        x=b[i];i+=1;r|=(x&0x7f)<<s
        if not x&0x80:return r,i
        s+=7
def enc(n):
    out=bytearray()
    while True:
        b=n&0x7f;n>>=7
        if n: out.append(b|0x80)
        else: out.append(b); return bytes(out)
def nm(b,i):
    n,i=leb(b,i);return b[i:i+n].decode("utf-8","replace"),i+n
def secs(b,s,e):
    i=s
    while i<e:
        sid=b[i];i+=1;sz,i=leb(b,i);yield sid,i,sz;i+=sz
src,dst=sys.argv[1],sys.argv[2]
b=open(src,"rb").read()
out=bytearray(b[:8])
removed=0
for sid,o,sz in secs(b,8,len(b)):
    if sid!=1:
        out+=bytes([sid])+enc(sz)+b[o:o+sz]; continue
    mod=bytearray(b[o:o+8])
    for s2,o2,z2 in secs(b,o+8,o+sz):
        if s2==0 and nm(b,o2)[0]=="name":
            removed+=z2+1+len(enc(z2)); continue
        mod+=bytes([s2])+enc(z2)+b[o2:o2+z2]
    out+=bytes([1])+enc(len(mod))+bytes(mod)
open(dst,"wb").write(out)
print(f"source {len(b)} -> stripped {len(out)} bytes (name payload+header removed {removed}, container LEB delta {len(b)-removed-len(out)})")
