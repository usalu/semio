import subprocess,sys,time,re
out=sys.argv[1];cmd=sys.argv[2:]
t0=time.time()
p=subprocess.Popen(cmd,stdout=open(out+'.out','w'),stderr=subprocess.PIPE,text=True,errors='replace')
f=open(out+'.err','w')
for line in p.stderr:
    f.write(f'{time.time()-t0:7.2f} {line}');f.flush()
p.wait()
print('wall',round(time.time()-t0,1),'rc',p.returncode)
