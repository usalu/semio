import sys,subprocess
f,line,frag=sys.argv[1],int(sys.argv[2]),sys.argv[3]
t=open(f,encoding="utf8").read().split("\n")
print("NOW:",t[line-1][:1500])
h=subprocess.run(["git","-c","core.fsmonitor=false","show","HEAD:"+f],capture_output=True,text=True,cwd="/Users/ueli/Documents/semio").stdout.split("\n")
for i,l in enumerate(h,1):
    if frag in l: print("HEAD",i,":",l[:1500])
