import os,sys,json,hashlib,time,signal,unicodedata,stat,subprocess,threading
p=json.loads(sys.argv[1]);binding=sys.argv[2];parser=sys.argv[3];started=time.monotonic();base='/tmp/semio-git-phase-'+p['operation'];created=False;active=None;rows={};total=0
class Cancelled(Exception): pass
def check():
 if time.monotonic()-started>=900: raise Cancelled('Git phase deadline')
def stop(signum,frame): raise Cancelled('Git phase cancelled')
for s in (signal.SIGINT,signal.SIGTERM,signal.SIGALRM): signal.signal(s,stop)
signal.alarm(900)
def exclusive(path,value,raw=False):
 fd=os.open(path,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
 try:
  b=value if raw else json.dumps(value,separators=(',',':')).encode();at=0
  while at<len(b):
   n=os.write(fd,b[at:])
   if n<=0: raise ValueError('metadata short write')
   at+=n
  os.fsync(fd)
 finally: os.close(fd)
def digest(path,maximum):
 if os.path.realpath(path)!=path: raise ValueError('physical metadata file')
 fd=os.open(path,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK)
 try:
  a=os.fstat(fd)
  if not stat.S_ISREG(a.st_mode) or a.st_nlink!=1 or a.st_size>maximum: raise ValueError('bounded ordinary metadata')
  h=hashlib.sha256();count=0
  while True:
   check();b=os.read(fd,65536)
   if not b: break
   count+=len(b);h.update(b)
  z=os.fstat(fd);n=os.lstat(path)
  key=lambda s:(s.st_dev,s.st_ino,s.st_mode,s.st_nlink,s.st_size,s.st_mtime_ns,s.st_ctime_ns)
  if key(a)!=key(z) or key(z)!=key(n) or count!=a.st_size: raise ValueError('stable exact metadata')
  return h.hexdigest(),count
 finally: os.close(fd)
def source():
 actual=digest('/repo/.semio-private/source.json',134217728)
 if actual!=(p['source']['manifestSha256'],p['source']['manifestBytes']): raise ValueError('captured Source manifest differs')
exec(compile(parser,'sealed-physical-parser','exec'))
def allowed(path):
 safe(path)
 if path in ('.git/HEAD','.git/index','.git/config','.git/packed-refs','.semio-private/git.json'): return path
 if path.startswith('.git/refs/') and path!='.git/refs/replace' and not path.startswith('.git/refs/replace/'): return path
 import re
 if re.fullmatch(r'[.]git/objects/pack/pack-[a-f0-9]{40}[.](pack|idx)',path): return path
 raise ValueError('closed genuine Git metadata path')
def parent(path):
 current='/repo'
 for part in path.split('/')[:-1]:
  current+='/'+part
  try: os.mkdir(current,0o700)
  except FileExistsError: pass
  if not os.path.isdir(current) or os.path.islink(current) or os.path.realpath(current)!=current: raise ValueError('physical Git ancestry')
 return '/repo/'+path
class Limited:
 def __init__(self,stream): self.stream=stream;self.bytes=0;self.hash=hashlib.sha256()
 def read(self,n):
  check()
  if n<0 or n>65536: raise ValueError('bounded archive read')
  b=self.stream.read(n);self.bytes+=len(b)
  if self.bytes>p['git']['archiveBytes'] or self.bytes>536870912: raise ValueError('bounded Git archive')
  self.hash.update(b);return b
gitEnvironment={"GIT_DIR":"/repo/.git","GIT_WORK_TREE":"/repo","GIT_INDEX_FILE":"/repo/.git/index","GIT_COMMON_DIR":"/repo/.git","GIT_OPTIONAL_LOCKS":"0","GIT_NO_REPLACE_OBJECTS":"1","GIT_CONFIG_NOSYSTEM":"1","GIT_CONFIG_GLOBAL":"/dev/null"}
def git(id,args,maximum):
 global active
 check();env={**{k:v for k,v in os.environ.items() if not k.startswith('GIT_')},**gitEnvironment};argv=['/usr/bin/git','-c','core.fsmonitor=false','-c','status.renames=false','-c','core.hooksPath=/dev/null','-C','/repo',*args];active=subprocess.Popen(argv,env=env,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True);child=active;buffers=[bytearray(),bytearray()];failed=[]
 def drain(index,stream):
  try:
   while True:
    b=stream.read(65536)
    if not b: break
    if len(buffers[index])+len(b)>maximum: raise ValueError('Git proof output bound')
    buffers[index].extend(b)
  except BaseException as e:
   failed.append(e)
   try: os.killpg(child.pid,signal.SIGKILL)
   except ProcessLookupError: pass
 threads=[threading.Thread(target=drain,args=(i,s)) for i,s in enumerate((child.stdout,child.stderr))]
 for thread in threads: thread.start()
 try:
  until=time.monotonic()+60
  while child.poll() is None:
   check()
   if time.monotonic()>=until: raise Cancelled('Git proof command deadline')
   time.sleep(.05)
 except BaseException:
  try: os.killpg(child.pid,signal.SIGKILL)
  except ProcessLookupError: pass
  child.wait(timeout=10)
  for thread in threads: thread.join(timeout=10)
  raise
 status=child.wait(timeout=10)
 for thread in threads: thread.join(timeout=10)
 active=None
 if failed or any(t.is_alive() for t in threads) or status!=0: raise ValueError('actual system Git refused')
 output=bytes(buffers[0]);return output,dict(id=id,program='/usr/bin/git',argv=argv[1:],status=status,signal=None,stdoutBytes=len(output),stdoutSha256=hashlib.sha256(output).hexdigest(),stderrBytes=len(buffers[1]),stderrSha256=hashlib.sha256(buffers[1]).hexdigest())
try:
 if os.environ.get('SEMIO_SCRIPT_PROCESS_INVOCATION') or os.environ.get('SEMIO_SCRIPT_POLICY'): raise ValueError('Guest cannot inherit a host script envelope')
 if os.path.realpath('/tmp')!='/tmp' or os.path.realpath('/repo')!='/repo' or os.path.lexists('/repo/.git') or os.path.lexists('/repo/.semio-private/git.json'): raise ValueError('fresh physical Git destination')
 source();os.mkdir(base,0o700);created=True;exclusive(base+'/owner.json',dict(binding=binding,cohort=p['cohort'],containerId=p['builder']['id'],operation=p['operation']));manifest=None;stream=Limited(sys.stdin.buffer)
 for path,size,mode,read in members(stream):
  allowed(path)
  if manifest is not None or path in rows or mode&0o111 or path!='.semio-private/git.json' and len(rows)>=32: raise ValueError('closed nonexecutable Git metadata')
  if path=='.semio-private/git.json' and size>1048576: raise ValueError('bounded Git manifest')
  fd=os.open(parent(path),os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600);h=hashlib.sha256();count=0;body=bytearray() if path=='.semio-private/git.json' else None
  try:
   while True:
    chunk=read(65536)
    if not chunk: break
    count+=len(chunk);h.update(chunk)
    if body is not None: body.extend(chunk)
    at=0
    while at<len(chunk):
     n=os.write(fd,chunk[at:])
     if n<=0: raise ValueError('Git metadata short write')
     at+=n
   os.fsync(fd)
  finally: os.close(fd)
  if count!=size: raise ValueError('exact Git metadata length')
  if body is not None:
   if h.hexdigest()!=p['git']['manifestSha256']: raise ValueError('Git manifest digest')
   manifest=json.loads(body)
  else: rows[path]=dict(bytes=count,sha256=h.hexdigest());total+=count
 if stream.bytes!=p['git']['archiveBytes'] or stream.hash.hexdigest()!=p['git']['archiveSha256'] or manifest is None or manifest.get('freshlyGenerated') is not True or manifest.get('captureId')!=p['captureId'] or manifest.get('sourceBlobs')!=0 or manifest.get('head')!=p['git']['head'] or manifest.get('created')!=p['git']['created'] or total!=p['git']['bytes'] or len(rows)!=p['git']['files'] or len(manifest['files'])!=len(rows): raise ValueError('exact fresh genuine Git cohort')
 seen=set()
 def verify():
  for row in manifest['files']:
   path=allowed(row['path']);actual=rows.get(path)
   if path in seen or actual is None or actual['bytes']!=row['bytes'] or actual['sha256']!=row['sha256'] or digest('/repo/'+path,268435456)!=(row['sha256'],row['bytes']): raise ValueError('actual Git metadata digest')
   seen.add(path)
 verify();os.makedirs('/repo/.git/info',mode=0o700,exist_ok=True);exclusive('/repo/.git/info/exclude',b'.semio-private/'+bytes([10]),True)
 head,rev=git('revision',['rev-parse','HEAD'],4096);date,log=git('log',['log','-1','--format=%cI'],4096);status,statusProof=git('status',['status','--porcelain=v1','-z','--untracked-files=no'],67108864)
 if head.decode().strip()!=p['git']['head'] or date.decode().strip()!=p['git']['created']: raise ValueError('actual guest Git identity/time')
 seen.clear();verify();source();check();terminal=dict(version=1,kind='fresh-linux-git-proved',binding=binding,cohort=p['cohort'],captureId=p['captureId'],containerId=p['builder']['id'],operation=p['operation'],head=head.decode().strip(),created=date.decode().strip(),sourceManifestSha256=p['source']['manifestSha256'],metadataManifestSha256=p['git']['manifestSha256'],archiveSha256=p['git']['archiveSha256'],metadataBytes=total,metadataFiles=len(rows),sourceBlobs=0,gitEnvironment=gitEnvironment,guestGitVerified=True,sourceRechecked=True,parentEnvelopeForwarded=False,dirty=bool(status),dirtyAtCapture=p['git']['dirtyAtCapture'],dirtyMatchesCapture=bool(status)==p['git']['dirtyAtCapture'],statusScope='tracked-only-at-guest-observation',commands=[rev,log,statusProof],operationalExcludeSha256=hashlib.sha256(b'.semio-private/'+bytes([10])).hexdigest(),terminal=True)
except BaseException as e:
 if active is not None:
  try: os.killpg(active.pid,signal.SIGKILL)
  except ProcessLookupError: pass
  active.wait(timeout=10)
 terminal=dict(version=1,kind='fresh-linux-git-failed',binding=binding,cohort=p['cohort'],containerId=p['builder']['id'],operation=p['operation'],guestGitVerified=False,terminal=True,partialCustody='retained',reason=str(e)[:256])
finally:
 signal.alarm(0)
 if created: exclusive(base+'/terminal.json',terminal)
 print(json.dumps(terminal,separators=(',',':')),flush=True)
sys.exit(0 if terminal['guestGitVerified'] else 1)
