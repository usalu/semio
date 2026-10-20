import os,sys,json,hashlib,time,signal,unicodedata
p=json.loads(sys.argv[1]); binding=sys.argv[2]; base='/tmp/semio-receiver-'+p['operation']; terminal=None; rows={}; total=0; started=time.monotonic(); reason=None; created=False
class Cancelled(Exception): pass
def stop(signum,frame): raise Cancelled('cancelled' if signum!=signal.SIGALRM else 'deadline')
for s in (signal.SIGINT,signal.SIGTERM,signal.SIGALRM): signal.signal(s,stop)
signal.alarm(900)
def check():
 if time.monotonic()-started>=900: raise Cancelled('deadline')
def exclusive(path,value):
 fd=os.open(path,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
 try:
  b=json.dumps(value,separators=(',',':')).encode(); at=0
  while at<len(b):
   n=os.write(fd,b[at:])
   if n<=0: raise ValueError('metadata short write')
   at+=n
  os.fsync(fd)
 finally: os.close(fd)
def safe(path):
 if not isinstance(path,str) or not path or path!=unicodedata.normalize('NFC',path) or path.startswith('/') or any(c in path for c in [chr(92),':']) or any(ord(c)<32 or ord(c)==127 for c in path) or any(v in ('','.','..') for v in path.split('/')) or len(path.split('/'))>64 or len(path.encode())>1024 or len(('/repo/'+path).encode('utf-16-le'))//2>256: raise ValueError('canonical Source path')
 return path
def parent(path):
 current='/repo'
 for part in path.split('/')[:-1]:
  current+='/'+part
  try: os.mkdir(current,0o755)
  except FileExistsError: pass
  if os.path.islink(current) or not os.path.isdir(current) or os.path.realpath(current)!=current: raise ValueError('physical Source ancestry')
 return '/repo/'+path
def members(stream):
 def exact(n):
  out=bytearray()
  while len(out)<n:
   check(); chunk=stream.read(n-len(out))
   if not chunk: raise ValueError('truncated physical archive')
   out.extend(chunk)
  return bytes(out)
 def octal(value):
  text=value.strip(bytes([0,32]))
  if not text or any(c<48 or c>55 for c in text): raise ValueError('canonical octal header')
  return int(text,8)
 def padding(size):
  if size%512 and any(exact(512-size%512)): raise ValueError('zero archive padding')
 pending=None
 while True:
  header=exact(512)
  if not any(header):
   if pending is not None or any(exact(512)) or stream.read(1): raise ValueError('exact final archive blocks')
   return
  if header[257:263]!=b'ustar'+bytes([0]) or header[263:265]!=b'00' or sum(header[:148])+256+sum(header[156:])!=octal(header[148:156]): raise ValueError('physical archive header')
  size=octal(header[124:136]); mode=octal(header[100:108]); kind=header[156:157]
  if kind==b'x':
   if pending is not None or size<1 or size>2048: raise ValueError('bounded physical PAX metadata')
   body=exact(size); padding(size); space=body.find(b' ')
   if space<1 or not body[:space].isdigit() or int(body[:space])!=len(body) or not body.endswith(bytes([10])) or not body[space+1:].startswith(b'path='): raise ValueError('sole exact PAX path record')
   pending=safe(body[space+6:-1].decode('utf8')); continue
  if kind!=b'0' or pending is None or size>268435456: raise ValueError('bounded physical ordinary file')
  path=pending; pending=None; remaining=size
  def read(maximum):
   nonlocal remaining
   n=min(maximum,remaining); remaining-=n
   return exact(n) if n else b''
  yield path,size,mode,read
  if remaining: raise ValueError('unconsumed physical file')
  padding(size)
try:
 if os.path.realpath('/tmp')!='/tmp' or os.path.realpath('/repo')!='/repo' or os.listdir('/repo'): raise ValueError('empty physical Source root')
 os.mkdir(base,0o700); created=True; exclusive(base+'/owner.json',{'binding':binding,'cohort':p['cohort'],'captureId':p['captureId'],'operation':p['operation'],'containerId':p['identity']['id']})
 manifest=None; manifestHash=None; manifestBytes=0
 for path,size,mode,read in members(sys.stdin.buffer):
   check(); path=safe(path)
   if manifest is not None or path in rows or path!='.semio-private/source.json' and len(rows)>=262144: raise ValueError('finite unique regular Source')
   if path=='.semio-private/source.json' and size>134217728: raise ValueError('bounded final manifest')
   fd=os.open(parent(path),os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o700 if mode&0o111 else 0o600); count=0; h=hashlib.sha256(); body=bytearray() if path=='.semio-private/source.json' else None
   try:
    while True:
     check(); chunk=read(65536)
     if not chunk: break
     count+=len(chunk)
     if count>size: raise ValueError('exact Source length')
     h.update(chunk)
     if body is not None: body.extend(chunk)
     at=0
     while at<len(chunk):
      n=os.write(fd,chunk[at:])
      if n<=0: raise ValueError('Source short write')
      at+=n
    if count!=size: raise ValueError('Source truncated')
    os.fsync(fd)
   finally: os.close(fd)
   if body is not None:
    manifest=json.loads(body); manifestHash=h.hexdigest(); manifestBytes=count
   else:
    total+=count
    if total>4294967296: raise ValueError('Source byte ceiling')
    rows[path]={'bytes':count,'sha256':h.hexdigest(),'executable':bool(mode&0o111)}
 if manifest is None or manifest.get('captureId')!=p['captureId'] or manifest.get('planSha256')!=p['capturePlanSha256'] or manifest.get('atomicHostSnapshot') is not False or manifest.get('bytes')!=total or len(manifest.get('rows',[]))!=len(rows): raise ValueError('actual final Source manifest')
 seen=set()
 for row in manifest['rows']:
  check(); path=safe(row['path']); actual=rows.get(path)
  if path in seen or actual is None or any(row[k]!=actual[k] for k in ('bytes','sha256','executable')): raise ValueError('manifest row differs')
  seen.add(path); fd=os.open('/repo/'+path,os.O_RDONLY|os.O_NOFOLLOW); h=hashlib.sha256()
  try:
   before=os.fstat(fd)
   if not __import__('stat').S_ISREG(before.st_mode) or before.st_nlink!=1 or before.st_size!=actual['bytes']: raise ValueError('physical Source row')
   while True:
    check(); chunk=os.read(fd,65536)
    if not chunk: break
    h.update(chunk)
   after=os.fstat(fd); named=os.lstat('/repo/'+path)
   if (before.st_dev,before.st_ino,before.st_size,before.st_mtime_ns,before.st_ctime_ns)!=(after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns,after.st_ctime_ns) or (after.st_dev,after.st_ino)!=(named.st_dev,named.st_ino) or h.hexdigest()!=actual['sha256']: raise ValueError('verified Source row changed')
  finally: os.close(fd)
 for row in manifest.get('absent',[]):
  if os.path.lexists('/repo/'+safe(row['path'])): raise ValueError('manifest absence differs')
 check(); terminal={'version':1,'binding':binding,'operation':p['operation'],'captureId':p['captureId'],'cohort':p['cohort'],'containerId':p['identity']['id'],'receiverTerminal':True,'state':'success','actualFiles':len(rows),'actualBytes':total,'manifestSha256':manifestHash,'manifestBytes':manifestBytes,'allSourceHashesVerified':True,'partialCustody':'complete'}
except BaseException as e:
 terminal={'version':1,'binding':binding,'operation':p['operation'],'captureId':p['captureId'],'cohort':p['cohort'],'containerId':p['identity']['id'],'receiverTerminal':True,'state':'cancelled' if isinstance(e,Cancelled) else 'failure','actualFiles':len(rows),'actualBytes':total,'allSourceHashesVerified':False,'partialCustody':'retained','reason':str(e)[:256]}
finally:
 signal.alarm(0)
 if created: exclusive(base+'/terminal.json',terminal)
 print(json.dumps(terminal,separators=(',',':')),flush=True)
sys.exit(0 if terminal['state']=='success' else 1)
