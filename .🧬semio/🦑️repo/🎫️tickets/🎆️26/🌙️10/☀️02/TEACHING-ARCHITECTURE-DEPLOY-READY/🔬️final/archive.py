import sys,json,hashlib,io,gzip,os,stat,time
MAX=4*1024**3
class Meter:
 def __init__(self,source,limit=MAX): self.source=source;self.left=limit;self.bytes=0;self.digest=hashlib.sha256()
 def read(self,n=-1):
  if n<0: raise ValueError('Bounded reads required')
  b=self.source.read(min(n,self.left+1));self.left-=len(b)
  if self.left<0: raise ValueError('Archive bound exceeded')
  self.bytes+=len(b);self.digest.update(b);return b
class Part:
 def __init__(self,source,size): self.source=source;self.left=size
 def read(self,n=-1):
  if n<0: raise ValueError('Bounded reads required')
  b=self.source.read(min(n,self.left));self.left-=len(b)
  if not b and self.left: raise ValueError('Truncated member')
  return b
 def drain(self):
  while self.left: self.read(min(65536,self.left))
def take(source,n):
 out=bytearray()
 while len(out)<n:
  b=source.read(n-len(out))
  if not b: raise ValueError('Truncated tar')
  out.extend(b)
 return bytes(out)
def number(b):
 if b and b[0]&128: raise ValueError('Base256 tar numeric unsupported')
 s=b.rstrip(b'\0 ').lstrip(b' ');return int(s or b'0',8)
def path(s):
 if not s or len(s.encode())>4096 or s.startswith('/') or '\\' in s or any(ord(c)<32 or ord(c)==127 for c in s): raise ValueError('Safe tar path required')
 s=s.removeprefix('./').rstrip('/')
 if any(p in ('','..','.') for p in s.split('/')): raise ValueError('Confined tar path required')
 return s
def members(source,consume,limit):
 count=0;pending=None
 while True:
  h=take(source,512)
  if h==bytes(512):
   if take(source,512)!=bytes(512) or pending is not None: raise ValueError('Exact tar end required')
   while True:
    b=source.read(65536)
    if not b: return
    if any(b): raise ValueError('Trailing tar payload')
  count+=1
  if count>limit or sum(h[:148])+256+sum(h[156:])!=number(h[148:156]): raise ValueError('Tar checksum/count refused')
  size=number(h[124:136]);kind=h[156:157];name=h[:100].split(b'\0')[0];prefix=h[345:500].split(b'\0')[0];name=(prefix+b'/' if prefix else b'')+name
  if kind in (b'x',b'L'):
   if pending is not None or size>65536: raise ValueError('Bounded single tar extension required')
   b=take(source,size)
   if kind==b'L': pending=path(b.rstrip(b'\0').decode())
   else:
    pos=0;fields={}
    while pos<len(b):
     space=b.find(b' ',pos)
     if space<0 or space-pos>8: raise ValueError('PAX length refused')
     length=int(b[pos:space]);record=b[space+1:pos+length]
     if length<=space-pos+1 or pos+length>len(b) or not record.endswith(b'\n') or b'=' not in record: raise ValueError('PAX record refused')
     key,value=record[:-1].split(b'=',1)
     if key in fields: raise ValueError('PAX duplicate')
     fields[key]=value;pos+=length
    if any(k not in {b'path',b'mtime',b'atime',b'ctime',b'uid',b'gid',b'uname',b'gname'} and not k.startswith(b'SCHILY.xattr.') for k in fields): raise ValueError('PAX authority unsupported')
    pending=path(fields.get(b'path',name).decode())
  else:
   if kind in (b'g',b'S',b'K') or size>1024**3: raise ValueError('Tar extension/size refused')
   rawname=pending or name.decode();pending=None
   if rawname in ('.','./') and kind==b'5' and size==0:continue
   name=path(rawname)
   part=Part(source,size);consume(name,kind,size,number(h[100:108]),part);part.drain()
  pad=(-size)%512
  if pad and any(take(source,pad)): raise ValueError('Tar zero padding required')
def inspect_archive(source):
 files={};layers={};selected={'usr/local/bin/proctor','srv/course/deploy/compose.yaml','srv/course/deploy/Caddyfile'}
 def consume(name,kind,size,mode,part):
  if name in files: raise ValueError('Unique archive paths required')
  if kind==b'5':
   if size: raise ValueError('Directory bytes refused')
   return
  if kind not in (b'0',b'\0'): raise ValueError('Archive regular files required')
  raw=Meter(part,1024**3);prefix=take(raw,min(512,size));body=io.BufferedReader(Prefix(prefix,raw),65536)
  records={};layer=False
  if prefix[:2]==b'\x1f\x8b': decoded=Meter(gzip.GzipFile(fileobj=body),MAX);layer=True
  elif len(prefix)==512 and prefix[257:262]==b'ustar': decoded=Meter(body,MAX);layer=True
  else: decoded=None
  if layer:
   def entry(p,k,n,m,part):
    if p in selected:
     if k not in (b'0',b'\0') or n>268435456: records[p]={'invalid':True};return
     digest=hashlib.sha256();first=b'';text=bytearray()
     while part.left:
      b=part.read(min(65536,part.left));digest.update(b)
      if len(first)<64:first+=b[:64-len(first)]
      if p!='usr/local/bin/proctor':
       if len(text)+len(b)>65536: raise ValueError('Stack text bound')
       text.extend(b)
     records[p]={'sha256':digest.hexdigest(),'bytes':n,'mode':m,'prefix':first.hex()}
     if p!='usr/local/bin/proctor':records[p]['text']=text.decode()
    base=p.rsplit('/',1)[-1]
    if base.startswith('.wh.'): records[p]={'whiteout':True}
   members(decoded,entry,262144);layers[name]={'diffId':'sha256:'+decoded.digest.hexdigest(),'bytes':decoded.bytes,'selected':records}
  else:
   content=bytearray(prefix)
   while True:
    b=raw.read(65536)
    if not b:break
    if len(content)<1048577:content.extend(b)
   if len(content)<=1048576 and content[:1] in (b'{',b'['):
    try: records=json.loads(content)
    except Exception:records=None
   else:records=None
  while body.read(65536):pass
  files[name]={'sha256':raw.digest.hexdigest(),'bytes':raw.bytes,'json':records if not layer else None}
 members(source,consume,131072)
 manifest=files.get('manifest.json',{}).get('json')
 if not isinstance(manifest,list) or not 2<=len(manifest)<=32:raise ValueError('Docker-save finite image manifests required; other archive formats unavailable')
 images=[]
 for row in manifest:
  if set(row)-{'Config','RepoTags','Layers','LayerSources'}:raise ValueError('Docker-save manifest keys unsupported')
  config=files[row['Config']];doc=config['json'];ordered=[layers[p] for p in row['Layers']]
  if not isinstance(doc,dict) or ['sha256:'+config['sha256']]!=['sha256:'+row['Config'].split('/')[-1].removesuffix('.json')]:raise ValueError('Image config digest filename mismatch')
  if doc.get('rootfs',{}).get('type')!='layers' or doc['rootfs'].get('diff_ids')!=[r['diffId'] for r in ordered]:raise ValueError('Layer diff ID mismatch')
  selectedFiles={}
  for layer in ordered:
   for p,v in sorted(layer['selected'].items(),key=lambda item:not item[1].get('whiteout',False)):
    if v.get('whiteout'):
     parent,_,base=p.rpartition('/');target=(parent+'/' if parent else '')+base[4:]
     if base=='.wh..wh..opq':
      for key in list(selectedFiles):
       if key.startswith(parent+'/'):del selectedFiles[key]
     else:
      for key in list(selectedFiles):
       if key==target or key.startswith(target+'/'):del selectedFiles[key]
    else:selectedFiles[p]=v
  images.append({'id':'sha256:'+config['sha256'],'tags':row.get('RepoTags') or [],'config':doc,'layers':[{'path':p,'sha256':files[p]['sha256'],'diffId':layers[p]['diffId'],'bytes':files[p]['bytes']} for p in row['Layers']],'selected':selectedFiles})
 unique={}
 for image in images:
  prior=unique.get(image['id'])
  if prior:
   if [r['sha256'] for r in prior['layers']]!=[r['sha256'] for r in image['layers']] or prior['selected']!=image['selected']:raise ValueError('Conflicting image alias rows')
   prior['tags']=list(dict.fromkeys(prior['tags']+image['tags']))
  else:unique[image['id']]=image
 images=list(unique.values())
 if len(images)!=2:raise ValueError('Exactly two unique saved image configs required')
 index=files.get('index.json',{}).get('json');oci=None
 if 'index.json' in files and not isinstance(index,dict):raise ValueError('Valid OCI index document required')
 if index is not None:
  byid={r['id']:r for r in images};found={};seen=set();active=set()
  def normalized(tag):return tag.removeprefix('docker.io/library/').removeprefix('docker.io/')
  def descriptor(d,depth,annotations):
   if depth>4 or set(d)-{'mediaType','digest','size','annotations','platform'}:raise ValueError('Unsupported OCI descriptor')
   digest=d.get('digest','');size=d.get('size');key=digest.removeprefix('sha256:')
   if len(key)!=64 or any(c not in '0123456789abcdef' for c in key) or digest in active:raise ValueError('Acyclic OCI sha256 descriptor required')
   seen.add(digest);active.add(digest)
   if len(seen)>16:raise ValueError('OCI descriptor bound')
   blob=files.get('blobs/sha256/'+key)
   if not blob or blob['sha256']!=key or blob['bytes']!=size or not isinstance(blob['json'],dict):raise ValueError('OCI descriptor bytes/digest mismatch')
   value=blob['json'];annotations={**annotations,**d.get('annotations',{})};media=d.get('mediaType')
   if media in ('application/vnd.oci.image.index.v1+json','application/vnd.docker.distribution.manifest.list.v2+json'):
    for child in value.get('manifests',[]):descriptor(child,depth+1,annotations)
   elif media in ('application/vnd.oci.image.manifest.v1+json','application/vnd.docker.distribution.manifest.v2+json'):
    config=value.get('config',{});image=byid.get(config.get('digest'));layerDigests=[r.get('digest') for r in value.get('layers',[])]
    if not image or layerDigests!=['sha256:'+r['sha256'] for r in image['layers']]:raise ValueError('OCI/classic image layer mapping differs')
    for layer in value.get('layers',[]):
     blob=files.get('blobs/sha256/'+layer['digest'].removeprefix('sha256:'))
     if not blob or blob['sha256']!=layer['digest'][7:] or blob['bytes']!=layer.get('size'):raise ValueError('Actual OCI layer blob required')
    configFile=files.get('blobs/sha256/'+image['id'][7:]) or files.get(image['id'][7:]+'.json')
    if config.get('size')!=configFile['bytes']:raise ValueError('OCI config size mismatch')
    platform=d.get('platform',{})
    if platform and (platform.get('os')!=image['config'].get('os') or platform.get('architecture')!=image['config'].get('architecture')):raise ValueError('OCI platform mismatch')
    names=[v for k,v in annotations.items() if k in ('io.containerd.image.name','org.opencontainers.image.ref.name')]
    if not names or not all(any(normalized(tag)==normalized(name) or name==tag.rsplit(':',1)[-1] for tag in image['tags']) for name in names):raise ValueError('OCI tag annotations differ')
    found[image['id']]=list(dict.fromkeys(found.get(image['id'],[])+names))
   else:raise ValueError('OCI media type unsupported')
   active.remove(digest)
  if index.get('schemaVersion')!=2 or not 2<=len(index.get('manifests',[]))<=32:raise ValueError('Two-image OCI index required')
  for d in index['manifests']:descriptor(d,0,{})
  if set(found)!=set(byid):raise ValueError('OCI/classic image set differs')
  oci=found
 return {'images':images,'files':len(files),'ociIndex':oci}
class Prefix(io.RawIOBase):
 def __init__(self,prefix,source):self.prefix=prefix;self.source=source
 def readable(self):return True
 def readinto(self,b):
  if self.prefix:data=self.prefix[:len(b)];self.prefix=self.prefix[len(data):]
  else:data=self.source.read(len(b))
  b[:len(data)]=data;return len(data)
def main():
 name=sys.argv[1];fd=os.open(name,os.O_RDONLY|getattr(os,'O_NOFOLLOW',0));before=os.fstat(fd)
 try:
  if not stat.S_ISREG(before.st_mode) or before.st_nlink!=1 or before.st_size>1024**3:raise ValueError('Bounded single-link archive required')
  with os.fdopen(os.dup(fd),'rb',buffering=65536) as f:
   source=Meter(f,1024**3);result=inspect_archive(source);result['archiveSha256']=source.digest.hexdigest();result['archiveBytes']=source.bytes
  after=os.fstat(fd);named=os.lstat(name)
  if any(getattr(before,k)!=getattr(after,k) or getattr(after,k)!=getattr(named,k) for k in ('st_dev','st_ino','st_size','st_mtime_ns','st_ctime_ns')):raise ValueError('Archive changed')
  print(json.dumps(result,separators=(',',':')))
 finally:os.close(fd)
if __name__=='__main__':main()
