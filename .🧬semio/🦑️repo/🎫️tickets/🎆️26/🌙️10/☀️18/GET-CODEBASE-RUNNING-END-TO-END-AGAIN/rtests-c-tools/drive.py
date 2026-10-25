import os,re,subprocess,sys,time,signal
root="/Users/ueli/Documents/semio"; os.chdir(root)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S=C+"/🗑️generated/repo-tests-c"
lst,out=sys.argv[1],sys.argv[2]; limit=int(sys.argv[3]) if len(sys.argv)>3 else 900
os.makedirs(out+"/logs",exist_ok=True); os.makedirs(S+"/art",exist_ok=True)
files=[l.strip() for l in open(lst) if l.strip()]
done=set()
tsv=out+"/summary.tsv"
if os.path.exists(tsv):
  for l in open(tsv): done.add(l.rstrip("\n").split("\t")[-1])
def load():
  return os.getloadavg()[0]
for n,f in enumerate(files,1):
  if f in done: continue
  while load()>60 or os.path.exists(out+"/pause"): time.sleep(10)
  log="%s/logs/%04d.log"%(out,n); art=os.path.abspath("%s/art/%04d"%(S,n)); os.makedirs(art,exist_ok=True)
  t=time.time()
  p=subprocess.Popen(["nice","-n","10","repo",root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts","--file="+f],env=dict(os.environ,RUSTUP_TOOLCHAIN="nightly-2026-07-20"),stdout=open(log,"wb"),stderr=subprocess.STDOUT,start_new_session=True)
  try: rc=p.wait(timeout=limit)
  except subprocess.TimeoutExpired:
    os.killpg(p.pid,signal.SIGTERM); time.sleep(3)
    try: os.killpg(p.pid,signal.SIGKILL)
    except Exception: pass
    p.wait(); rc="TIMEOUT"
  txt=open(log,errors="replace").read()
  m1=re.findall(r"^\s*(\d+) pass",txt,re.M); m2=re.findall(r"^\s*(\d+) fail",txt,re.M)
  with open(tsv,"a") as h: h.write("%d\t%s\t%s\t%s\t%d\t%s\n"%(n,rc,m1[-1] if m1 else "?",m2[-1] if m2 else "?",time.time()-t,f))
  subprocess.run(["rm","-rf",art])
