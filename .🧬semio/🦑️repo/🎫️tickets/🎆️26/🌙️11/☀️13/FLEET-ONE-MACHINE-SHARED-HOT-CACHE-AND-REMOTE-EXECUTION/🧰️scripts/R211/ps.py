import subprocess,re,sys
out=subprocess.run(["ps","-axo","pid=,ppid=,etime=,%cpu=,rss=,command="],capture_output=True,text=True).stdout
pat=re.compile(sys.argv[1]) if len(sys.argv)>1 else re.compile("semio")
for l in out.splitlines():
    if "grandchild.js" in l or "ready-tools" in l: continue
    if not pat.search(l): continue
    l=re.sub(r"/Users/ueli/Documents/semio/\.cache/b/t/m/bun/[0-9a-f]+/bun ","bun ",l)
    l=re.sub(r"(/Users/ueli/Documents/semio/)([^ ]*)",lambda m:"~/"+"/".join(m.group(2).split("/")[-2:]),l)
    print(l[:300])
