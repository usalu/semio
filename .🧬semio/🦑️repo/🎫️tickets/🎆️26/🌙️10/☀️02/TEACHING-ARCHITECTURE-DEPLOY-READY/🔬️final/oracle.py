import sys,json,io,tarfile,hashlib,sqlite3,importlib.util
spec=importlib.util.spec_from_file_location('actual',sys.argv[1]);actual=importlib.util.module_from_spec(spec);spec.loader.exec_module(actual)
def digest(b):return hashlib.sha256(b).hexdigest()
def tar(files):
 out=io.BytesIO()
 with tarfile.open(fileobj=out,mode='w',format=tarfile.PAX_FORMAT) as t:
  for p,b,mode in files:
   i=tarfile.TarInfo(p);i.size=len(b) if b is not None else 0;i.mode=mode
   if b is None:i.type=tarfile.DIRTYPE
   t.addfile(i,io.BytesIO(b) if b is not None else None)
 return out.getvalue()
elf=bytearray(64);elf[:7]=b'\x7fELF\x02\x01\x01';elf[16:18]=(2).to_bytes(2,'little');elf[18:20]=(62).to_bytes(2,'little');elf[20:24]=(1).to_bytes(4,'little')
layer=tar([('usr/local/bin/proctor',elf,0o555),('srv/course/deploy/compose.yaml',open(sys.argv[3],'rb').read(),0o644),('srv/course/deploy/Caddyfile',open(sys.argv[4],'rb').read(),0o644)])
proxy=tar([('usr/bin/caddy',b'neutral-proxy',0o555)])
configs=[];files=[];manifest=[]
for name,body,tag in [('p',layer,'ghcr.io/usalu/architecture-course-proctor:neutral'),('c',proxy,'caddy:sha256-'+'0c994536bddb66445885237f1a5dcc1916bccea922661c76b4e9fc24061f9b52')]:
 config=dict(os='linux',architecture='amd64',config=dict(Labels={'org.opencontainers.image.version':'0.0.1','org.opencontainers.image.revision':'b'*40+'-dirty','org.opencontainers.image.created':'2026-10-20T00:00:00+00:00'}),rootfs=dict(type='layers',diff_ids=['sha256:'+digest(body)]));raw=json.dumps(config,separators=(',',':')).encode();path=digest(raw)+'.json';files.extend([(path,raw,0o644),(name+'/layer.tar',body,0o644)]);manifest.append(dict(Config=path,RepoTags=[tag],Layers=[name+'/layer.tar']));configs.append(config)
archive=tar(files+[('manifest.json',json.dumps(manifest).encode(),0o644)])
result=actual.inspect_archive(actual.Meter(io.BytesIO(archive),len(archive)));result['archiveSha256']=digest(archive);result['archiveBytes']=len(archive)
with tarfile.open(fileobj=io.BytesIO(archive)) as t:
 for image,m in zip(result['images'],manifest):
  config=t.extractfile(m['Config']).read();assert image['id']=='sha256:'+digest(config)
  for row in image['layers']:assert row['diffId']=='sha256:'+digest(t.extractfile(row['path']).read())
negative=[]
for name,payload in [('truncated',archive[:600]),('oversized-pax',None)]:
 if payload is None:
  header=bytearray(512);header[:4]=b'pax\0';header[100:108]=b'0000644\0';header[124:136]=b'00000400001\0';header[156:157]=b'x';header[148:156]=b'        ';header[148:156]=('%06o\0 '%sum(header)).encode();payload=bytes(header)
 accepted=True
 try:actual.inspect_archive(actual.Meter(io.BytesIO(payload),len(payload)))
 except Exception:accepted=False
 assert not accepted;negative.append(dict(id=name,accepted=False))
extra=[]
def checked(name,payload,expected):
 ok=True
 try:
  observed=actual.inspect_archive(actual.Meter(io.BytesIO(payload),len(payload)))
  with tarfile.open(fileobj=io.BytesIO(payload)) as independent:
   for image in observed['images']:
    assert len(image['id'])==71
 except Exception:ok=False
 assert ok==expected,name
 extra.append(dict(id=name,accepted=ok))
rootlayer=tar([('./',None,0o755),('usr/local/bin/proctor',elf,0o555)])
rootconfig=dict(configs[0]);rootconfig['rootfs']=dict(type='layers',diff_ids=['sha256:'+digest(rootlayer)]);raw=json.dumps(rootconfig,separators=(',',':')).encode();rm=[dict(manifest[0]),dict(manifest[1])];rm[0]['Config']=digest(raw)+'.json';rm[0]['Layers']=['root/layer.tar'];checked('root-dir',tar(files+[(rm[0]['Config'],raw,0o644),('root/layer.tar',rootlayer,0o644),('manifest.json',json.dumps(rm).encode(),0o644)]),True)
alias=dict(manifest[0]);alias['RepoTags']=['ghcr.io/usalu/architecture-course-proctor:neutral-alias'];checked('alias-rows',tar(files+[('manifest.json',json.dumps(manifest+[alias]).encode(),0o644)]),True)
conflict=dict(alias);conflict['Layers']=['c/layer.tar'];checked('conflicting-alias',tar(files+[('manifest.json',json.dumps(manifest+[conflict]).encode(),0o644)]),False)
for variant in ['hybrid','hybrid-tag-conflict','hybrid-layer-conflict']:
 blobs=[];descriptors=[]
 for i,m in enumerate(manifest):
  configBody=next(b for p,b,mode in files if p==m['Config']);layerBody=next(b for p,b,mode in files if p==m['Layers'][0]);configSha=digest(configBody);layerSha=digest(layerBody);blobs.extend([('blobs/sha256/'+configSha,configBody,0o644),('blobs/sha256/'+layerSha,layerBody,0o644)])
  document=dict(schemaVersion=2,mediaType='application/vnd.oci.image.manifest.v1+json',config=dict(mediaType='application/vnd.oci.image.config.v1+json',digest='sha256:'+configSha,size=len(configBody)),layers=[dict(mediaType='application/vnd.oci.image.layer.v1.tar',digest='sha256:'+('0'*64 if variant=='hybrid-layer-conflict' and i==0 else layerSha),size=len(layerBody))]);body=json.dumps(document,separators=(',',':')).encode();blobs.append(('blobs/sha256/'+digest(body),body,0o644));tag='foreign' if variant=='hybrid-tag-conflict' and i==0 else m['RepoTags'][0];descriptors.append(dict(mediaType=document['mediaType'],digest='sha256:'+digest(body),size=len(body),annotations={'io.containerd.image.name':tag},platform=dict(os='linux',architecture='amd64')))
 index=json.dumps(dict(schemaVersion=2,manifests=descriptors)).encode();checked(variant,tar(files+blobs+[('manifest.json',json.dumps(manifest).encode(),0o644),('index.json',index,0o644)]),variant=='hybrid')
import gzip
layerVariants=[]
def variant(name,last,compressed=False):
 layers=[layer] if compressed else [layer,last];paths=['p/layer.tar.gz'] if compressed else ['p/layer.tar','last/layer.tar'];config=dict(configs[0]);config['rootfs']=dict(type='layers',diff_ids=['sha256:'+digest(b) for b in layers]);raw=json.dumps(config,separators=(',',':')).encode();m=[dict(manifest[0]),dict(manifest[1])];m[0]['Config']=digest(raw)+'.json';m[0]['Layers']=paths;parts=[(p,b,mode) for p,b,mode in files if p not in [manifest[0]['Config'],'p/layer.tar']];parts.append((m[0]['Config'],raw,0o644));parts.extend((p,gzip.compress(b,mtime=0) if compressed else b,0o644) for p,b in zip(paths,layers));payload=tar(parts+[('manifest.json',json.dumps(m).encode(),0o644)]);observed=actual.inspect_archive(actual.Meter(io.BytesIO(payload),len(payload)));layerVariants.append(dict(id=name,archive=observed));return observed
variant('gzip-layer',None,True)
corrupt=bytearray(layer);corrupt[512]^=1;parts=[(p,bytes(corrupt) if p=='p/layer.tar' else b,mode) for p,b,mode in files];checked('mutated-layer-bytes',tar(parts+[('manifest.json',json.dumps(manifest).encode(),0o644)]),False)
changed=bytes(elf)+b'new';observed=variant('selected-overwrite',tar([('usr/local/bin/proctor',changed,0o555)]));assert observed['images'][0]['selected']['usr/local/bin/proctor']['sha256']==digest(changed)
variant('selected-whiteout',tar([('usr/local/bin/.wh.proctor',b'',0o644)]))
variant('selected-opaque',tar([('usr/local/bin/.wh..wh..opq',b'',0o644)]))
linked=io.BytesIO()
with tarfile.open(fileobj=linked,mode='w') as t:
 i=tarfile.TarInfo('usr/local/bin/proctor');i.type=tarfile.SYMTYPE;i.linkname='/foreign';t.addfile(i)
variant('selected-link',linked.getvalue())
xattr=io.BytesIO()
with tarfile.open(fileobj=xattr,mode='w',format=tarfile.PAX_FORMAT) as t:
 i=tarfile.TarInfo('usr/local/bin/proctor');i.size=len(elf);i.mode=0o555;i.pax_headers={'SCHILY.xattr.security.capability':'neutral'};t.addfile(i,io.BytesIO(elf))
variant('pax-xattr',xattr.getvalue())
checked('hybrid-invalid-index',tar(files+[('manifest.json',json.dumps(manifest).encode(),0o644),('index.json',b'not-json',0o644)]),False)
rows=json.loads(sys.argv[2]);db=sqlite3.connect(':memory:')
for row in rows:assert db.execute('select ? = ?',(row['actual'],row['expected'])).fetchone()[0],row['id']
print(json.dumps(dict(archive=result,negative=negative,extra=extra,layerVariants=layerVariants,oracle='Python tarfile+hashlib+SQLite',cases=len(rows))))
