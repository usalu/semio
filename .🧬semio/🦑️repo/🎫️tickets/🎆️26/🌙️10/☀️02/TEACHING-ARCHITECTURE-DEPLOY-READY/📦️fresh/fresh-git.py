import os,sys,pathlib,subprocess,hashlib,json,zlib,struct,threading,time,signal
root,out=map(pathlib.Path,sys.argv[1:3]); expected,capture_id=sys.argv[3:5]; target=out/'.git'; started=time.monotonic(); limit=512*1024*1024
if not root.is_absolute() or not out.is_absolute() or len(capture_id)!=32 or any(c not in '0123456789abcdef' for c in capture_id): raise RuntimeError('Closed fresh genuine Git capture required')
if out.name!=capture_id or out.parent.name!='g2' or out.exists(): raise RuntimeError('Fresh exclusive Git metadata destination required')
if out.parent.is_symlink() or not out.parent.is_dir(): raise RuntimeError('Ordinary Git metadata parent required')
out.mkdir(mode=0o700)
def interrupt(signum,frame): raise InterruptedError('Genuine Git capture cancelled')
signal.signal(signal.SIGTERM,interrupt);signal.signal(signal.SIGINT,interrupt)
base_env={key:value for key,value in os.environ.items() if not key.startswith('GIT_')}
def environment(directory): return {**base_env,'GIT_OPTIONAL_LOCKS':'0','GIT_NO_REPLACE_OBJECTS':'1','GIT_DIR':str(directory),'GIT_WORK_TREE':str(root),'GIT_INDEX_FILE':str(directory/'index'),'GIT_COMMON_DIR':str(directory)}
env=environment(root/'.git')
def git_command(arguments,directory=None):
 fence();child=subprocess.Popen(arguments,cwd=root,env=environment(directory or root/'.git'),stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE);output=bytearray();errors=bytearray();failures=[]
 def drain(stream,target,maximum):
  try:
   while True:
    chunk=stream.read(65536)
    if not chunk: break
    if len(target)+len(chunk)>maximum: raise RuntimeError('Bounded Git observation output required')
    target.extend(chunk)
  except BaseException as error:
   failures.append(error)
   try: child.kill()
   except ProcessLookupError: pass
 readers=[threading.Thread(target=drain,args=(child.stdout,output,128*1024*1024)),threading.Thread(target=drain,args=(child.stderr,errors,1024*1024))]
 for reader in readers: reader.start()
 try:
  status=child.wait(timeout=min(120,max(1,300-(time.monotonic()-started))))
 finally:
  if child.poll() is None: child.kill()
  child.wait()
  for reader in readers: reader.join(timeout=10)
  if any(reader.is_alive() for reader in readers): raise RuntimeError('Git output readers did not settle')
 if failures: raise failures[0]
 if status: raise RuntimeError(errors.decode(errors='replace'))
 fence();return bytes(output)
def git(*args): return git_command(['git','-c','core.fsmonitor=false',*args])
def fence():
 if time.monotonic()-started>300: raise RuntimeError('Git metadata capture deadline exceeded')
def read(path):
 full=root/'.git'/path
 if len(str(full))>256 or len(str(full).encode())>256: raise RuntimeError('Bounded Git metadata path required')
 parents=[]
 for parent in reversed(full.parents):
  facts=parent.lstat()
  if parent.is_symlink() or not parent.is_dir(): raise RuntimeError('Physical ordinary Git ancestry required')
  parents.append((parent,facts.st_dev,facts.st_ino,facts.st_mode))
 named=full.lstat();fd=os.open(full,os.O_RDONLY|getattr(os,'O_NOFOLLOW',0)|getattr(os,'O_NONBLOCK',0))
 try:
  before=os.fstat(fd)
  if not __import__('stat').S_ISREG(before.st_mode) or before.st_nlink!=1 or before.st_size>128*1024*1024 or (named.st_dev,named.st_ino,named.st_mode)!=(before.st_dev,before.st_ino,before.st_mode): raise RuntimeError('Bounded ordinary Git metadata descriptor required')
  data=bytearray()
  while len(data)<before.st_size:
   fence();chunk=os.read(fd,min(65536,before.st_size-len(data)))
   if not chunk: raise RuntimeError('Exact Git metadata bytes required')
   data.extend(chunk)
  if os.read(fd,1): raise RuntimeError('Git metadata grew')
  after=os.fstat(fd);current=full.lstat();key=lambda s:(s.st_dev,s.st_ino,s.st_mode,s.st_nlink,s.st_size,s.st_mtime_ns,s.st_ctime_ns)
  if key(before)!=key(after) or key(after)!=key(current): raise RuntimeError('Git metadata changed')
  for parent,dev,ino,mode in parents:
   facts=parent.lstat()
   if (facts.st_dev,facts.st_ino,facts.st_mode)!=(dev,ino,mode) or parent.is_symlink(): raise RuntimeError('Git metadata ancestry changed')
  return bytes(data)
 finally: os.close(fd)
head=git('rev-parse','HEAD').strip().decode(); top=git('rev-parse','HEAD^{tree}').strip().decode(); created=git('log','-1','--format=%cI').strip().decode()
if head!=expected or len(head)!=40 or any(c not in '0123456789abcdef' for c in head): raise RuntimeError('Actual reviewed current HEAD required')
if git('for-each-ref','--format=%(refname)','refs/replace').strip(): raise RuntimeError('Uncaptured replace refs refused')
metadata={'HEAD':read('HEAD'),'index':read('index')}
if metadata['HEAD'].startswith(b'ref: '):
 ref=metadata['HEAD'][5:].strip().decode()
 if not ref.startswith('refs/') or '..' in ref.split('/'): raise RuntimeError('Invalid actual HEAD ref')
 if (root/'.git'/ref).exists(): metadata[ref]=read(ref)
 else: metadata['packed-refs']=read('packed-refs')
oids={top,*git('rev-list',head).decode().splitlines()}
for row in git('ls-tree','-r','-d','-z',head).split(b'\0'):
 if row: oids.add(row.split(b'\t',1)[0].split(b' ')[2].decode())
if len(oids)>200000 or sum(map(len,metadata.values()))>128*1024*1024: raise RuntimeError('Finite Git metadata bound exceeded')
for path,data in metadata.items():
 file=target/path; file.parent.mkdir(parents=True,exist_ok=True); file.write_bytes(data)
(target/'objects/pack').mkdir(parents=True,exist_ok=True)
configuration=b'[core]\n repositoryformatversion = 0\n bare = false\n filemode = true\n fsmonitor = false\n[status]\n renames = false\n'
(target/'config').write_bytes(configuration)
overhead=sum(map(len,metadata.values()))+len(configuration)+1072+28*len(oids)+20
pack=target/'objects/pack/p.pack'; records=[]; digest=hashlib.sha1(); total=0
def put(file,data):
 global total
 total+=len(data)
 if total+overhead>limit: raise RuntimeError('Git metadata aggregate bound exceeded')
 file.write(data); digest.update(data)
child=subprocess.Popen(['git','-c','core.fsmonitor=false','cat-file','--batch'],cwd=root,env=env,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
def feed():
 try:
  for oid in sorted(oids): child.stdin.write(oid.encode()+b'\n')
  child.stdin.close()
 except BrokenPipeError: pass
writer=threading.Thread(target=feed,daemon=True); writer.start()
try:
 with pack.open('wb') as file:
  put(file,b'PACK'+struct.pack('>II',2,len(oids)))
  for ordinal,oid in enumerate(sorted(oids)):
   fence(); header=child.stdout.readline().strip().split(b' ')
   if len(header)!=3 or header[0].decode()!=oid or header[1] not in (b'commit',b'tree'): raise RuntimeError('Authentic tree/commit required: '+str(header))
   size=int(header[2])
   if size>32*1024*1024: raise RuntimeError('Finite Git object required')
   data=child.stdout.read(size)
   if len(data)!=size or child.stdout.read(1)!=b'\n' or hashlib.sha1(header[1]+b' '+str(size).encode()+b'\0'+data).hexdigest()!=oid: raise RuntimeError('Original Git object digest mismatch')
   kind=1 if header[1]==b'commit' else 2; value=size>>4; first=(kind<<4)|(size&15); encoded=bytearray([first|(128 if value else 0)])
   while value:
    part=value&127; value>>=7; encoded.append(part|(128 if value else 0))
   blob=bytes(encoded)+zlib.compress(data,6); offset=total; crc=zlib.crc32(blob)&0xffffffff; put(file,blob); records.append((bytes.fromhex(oid),crc,offset))
   if ordinal%10000==0: print('[DEBUG] Genuine Git objects captured '+str(ordinal)+'/'+str(len(oids)),flush=True)
  checksum=digest.digest(); file.write(checksum)
 writer.join(timeout=10)
 if child.wait(timeout=10): raise RuntimeError(child.stderr.read().decode(errors='replace'))
except BaseException:
 child.kill(); child.wait(); raise
name='pack-'+checksum.hex(); final=pack.with_name(name+'.pack'); pack.rename(final); records.sort(key=lambda row:row[0]); fanout=[0]*256
for oid,_,_ in records: fanout[oid[0]]+=1
for i in range(1,256): fanout[i]+=fanout[i-1]
index=b'\xfftOc'+struct.pack('>I',2)+b''.join(struct.pack('>I',n) for n in fanout)+b''.join(row[0] for row in records)+b''.join(struct.pack('>I',row[1]) for row in records)+b''.join(struct.pack('>I',row[2]) for row in records)+checksum
index+=hashlib.sha1(index).digest(); final.with_suffix('.idx').write_bytes(index)
for path,data in metadata.items():
 if read(path)!=data: raise RuntimeError('Git metadata changed across capture: '+path)
if git('rev-parse','HEAD').strip().decode()!=head: raise RuntimeError('Actual HEAD changed across capture')
def private(*args): return git_command(['git','--git-dir='+str(target),'--work-tree='+str(root),*args],target)
if private('rev-parse','HEAD').strip().decode()!=head or private('log','-1','--format=%cI').strip().decode()!=created: raise RuntimeError('Independent genuine revision/time mismatch')
status=private('status','--porcelain','--untracked-files=no'); original=git('-c','status.renames=false','status','--porcelain','--untracked-files=no')
if bool(status)!=bool(original): raise RuntimeError('Actual dirty classifier diverged across metadata observations')
if git('for-each-ref','--format=%(refname)','refs/replace').strip(): raise RuntimeError('Replace refs changed across capture')
files=[]
for file in sorted(target.rglob('*')):
 if file.is_file():
  data=file.read_bytes(); files.append({'path':file.relative_to(out).as_posix(),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
report={'version':1,'captureId':capture_id,'freshlyGenerated':True,'sourceSnapshot':False,'gitEnvironment':'No inherited GIT_*; explicit physical GIT_DIR, GIT_WORK_TREE, GIT_INDEX_FILE and GIT_COMMON_DIR; read-only optional locks and replace-object policy','capturedAt':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),'head':head,'created':created,'objects':len(oids),'bytes':sum(row['bytes'] for row in files),'sourceBlobs':0,'packSha1':checksum.hex(),'privateGitStatusBytes':len(status),'originalGitStatusBytes':len(original),'statusEqualAtSeparateObservations':status==original,'files':files,'configuration':'Private operational core format0/bare=false/filemode=true/fsmonitor=false, status.renames=false prevents historical blob similarity reads; actual HEAD/ref/index/commits/HEADtrees retained; no remote or host user configuration copied','limits':{'maximumBytes':limit,'maximumMilliseconds':300000},'scope':'Authentic HEAD and ancestor commits, current HEAD reachable trees and actual index; no historical trees or blobs. Intended for read-only rev-parse/log-1/status provenance, not a full Git clone.'}
(out/'git-metadata.json').write_text(json.dumps(report,separators=(',',':'))+'\n'); print('[DEBUG] Genuine metadata verified by system Git '+json.dumps({k:v for k,v in report.items() if k!='files'}),flush=True)
