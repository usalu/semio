import os,sys,fcntl,struct,resource
F_PUNCHHOLE=99; CH=1<<20
def punch(fd,off,length):
    fcntl.fcntl(fd,F_PUNCHHOLE,struct.pack("<IIqq",0,0,off,length))
p=sys.argv[1]; fd=os.open(p,os.O_RDWR); size=os.fstat(fd).st_size; off=0; zero=bytes(CH); punched=0
while off<size:
    n=min(CH,size-off); b=os.pread(fd,n,off)
    if b==zero[:n]:
        punch(fd,off,n); punched+=n
    off+=n
os.close(fd)
print("punched",punched,"of",size,"maxrss_mb",resource.getrusage(resource.RUSAGE_SELF).ru_maxrss/1048576)
