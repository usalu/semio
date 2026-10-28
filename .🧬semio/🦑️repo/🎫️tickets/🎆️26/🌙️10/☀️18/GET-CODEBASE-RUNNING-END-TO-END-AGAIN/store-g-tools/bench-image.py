import os,sys,time
root=sys.argv[1]; big=int(sys.argv[2]) if len(sys.argv)>2 else 400
d=os.path.join(root,"bench"); os.makedirs(d,exist_ok=True)
buf=os.urandom(1<<20)
t=time.time(); f=open(os.path.join(d,"big"),"wb")
for _ in range(big): f.write(buf)
f.flush(); os.fsync(f.fileno()); f.close(); tw=time.time()-t
t=time.time(); sm=os.path.join(d,"small"); os.makedirs(sm,exist_ok=True)
for i in range(6000):
    with open(os.path.join(sm,f"f{i}"),"wb") as g: g.write(buf[:4096])
ts=time.time()-t
t=time.time(); f=open(os.path.join(d,"big"),"rb"); 
while f.read(1<<20): pass
f.close(); tr=time.time()-t
print(f"write {big}MB {tw:.2f}s  small6000 {ts:.2f}s  read {tr:.2f}s")
