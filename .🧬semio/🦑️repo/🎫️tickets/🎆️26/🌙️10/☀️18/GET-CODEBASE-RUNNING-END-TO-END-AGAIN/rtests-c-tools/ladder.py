import os,re,subprocess,sys,time,signal,json
root="/Users/ueli/Documents/semio"; os.chdir(root)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G=C+"/🗑️generated/rtests-c"
src,out=sys.argv[1],sys.argv[2]; limit=int(sys.argv[3]) if len(sys.argv)>3 else 280
births=json.load(open(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🔣️.json"))["births"]
os.makedirs(out+"/logs",exist_ok=True)
tsv=out+"/summary.tsv"
done=set()
if os.path.exists(tsv):
  for l in open(tsv): done.add(l.rstrip("\n").split("\t")[5])
def run(f,birth,log):
  art=os.path.abspath(G+"/art/ladder-%d"%os.getpid()); os.makedirs(art,exist_ok=True)
  p=subprocess.Popen(["nice","-n","10","repo",root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts","--file="+f,"--birth="+birth],env=dict(os.environ,RUSTUP_TOOLCHAIN="nightly-2026-07-20"),stdout=open(log,"wb"),stderr=subprocess.STDOUT,start_new_session=True)
  try: rc=p.wait(timeout=limit)
  except subprocess.TimeoutExpired:
    os.killpg(p.pid,signal.SIGTERM); time.sleep(3)
    try: os.killpg(p.pid,signal.SIGKILL)
    except Exception: pass
    p.wait(); rc="TIMEOUT"
  subprocess.run(["rm","-rf",art]); return rc
rows=[l.rstrip("\n").split("\t") for l in open(src+"/summary.tsv")]
n=0
for row in rows:
  n0,rc,p,f,sec,file=row[:6]
  if rc=="0" or file in done: continue
  txt=open("%s/logs/%04d.log"%(src,int(n0)),errors="replace").read()
  cur="setup"; last=txt; chain=[]
  while True:
    nxt=None
    ids=[b["id"] for b in births]; i=ids.index(cur)
    for b in births[i+1:]:
      if b.get("after") and re.search(b["after"],last): nxt=b; break
    if not nxt: break
    n+=1; log="%s/logs/%04d.log"%(out,n); t=time.time()
    r=run(file,nxt["id"],log); last=open(log,errors="replace").read(); chain.append((nxt["id"],r)); cur=nxt["id"]
    m1=re.findall(r"^\s*(\d+) pass",last,re.M); m2=re.findall(r"^\s*(\d+) fail",last,re.M)
    with open(tsv,"a") as h: h.write("%d\t%s\t%s\t%s\t%d\t%s\t%s\n"%(n,r,m1[-1] if m1 else "?",m2[-1] if m2 else "?",time.time()-t,file,nxt["id"]))
    if r==0: break
