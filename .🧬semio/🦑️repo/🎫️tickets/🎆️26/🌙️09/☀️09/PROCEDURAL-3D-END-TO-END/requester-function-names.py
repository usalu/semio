"""🔎 Resolve a browser WASM function index to the original Rust linker name."""
from pathlib import Path
import mmap
import struct
import sys

g=Path(__file__).resolve().parent/"🗑️generated/bridge-current"
def u(data,p):
    n=0
    for shift in range(0,35,7):
        b=data[p];p+=1;n|=(b&127)<<shift
        if b<128:return n,p
    raise ValueError("overflowing WASM index")
def read_name(data,p):
    size,start=u(data,p); return data[start:start+size],start+size
def names(data,requested):
    p=8
    while p<len(data):
        tag=data[p];size,at=u(data,p+1);p=at+size
        if tag!=0:continue
        label,at=read_name(data,at)
        if label!=b"name":continue
        while at<p:
            tag=data[at];size,start=u(data,at+1);at=start+size
            if tag!=1:continue
            count,pos=u(data,start)
            for _ in range(count):
                index,pos=u(data,pos);length,start=u(data,pos);pos=start+length
                if index in requested: yield index,data[start:pos].decode()
            return
def original_index(indices):
    for pass_no in [2,1]:
        with (g/f"requester-fold{pass_no}-index.u32").open("rb") as f, mmap.mmap(f.fileno(),0,access=mmap.ACCESS_READ) as d:
            indices=[struct.unpack_from("<I",d,n*4)[0] for n in indices]
    return indices
if __name__=="__main__":
    hot=[int(arg,0) for arg in sys.argv[1:]]
    original=original_index(hot)
    with (g/"requester-final-objects/procedural.core.wasm").open("rb") as f, mmap.mmap(f.fileno(),0,access=mmap.ACCESS_READ) as data:
        resolved=dict(names(data,set(original)))
    for a,b in zip(hot,original):
        print(f"{a}\t{b}\t{resolved.get(b,'(unnamed)')}")
