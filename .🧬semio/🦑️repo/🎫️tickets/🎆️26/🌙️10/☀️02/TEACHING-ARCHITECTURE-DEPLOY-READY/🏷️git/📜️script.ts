import {dirname,join} from "node:path";
import {mkdirSync,writeFileSync,readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {spawn} from "node:child_process";
import {Readable} from "node:stream";
import {pipeline} from "node:stream/promises";

const ticket=dirname(import.meta.dir),root="/Users/ueli/Documents/semio",output=join(ticket,"🗑️generated/g");
mkdirSync(output,{recursive:true});
if(process.argv[2]==="transport"){await transportGit();process.exit(0);}
const code=String.raw`
import os,sys,pathlib,subprocess,hashlib,json,zlib,struct,threading,time,signal
root,out=map(pathlib.Path,sys.argv[1:]); target=out/'.git'; started=time.monotonic(); limit=512*1024*1024
env={**os.environ,'GIT_OPTIONAL_LOCKS':'0','GIT_NO_REPLACE_OBJECTS':'1'}
def git(*args):
 result=subprocess.run(['git','-c','core.fsmonitor=false',*args],cwd=root,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=120)
 if result.returncode: raise RuntimeError(result.stderr.decode(errors='replace'))
 return result.stdout
def fence():
 if time.monotonic()-started>300: raise RuntimeError('Git metadata capture deadline exceeded')
def read(path):
 full=root/'.git'/path
 if full.is_symlink() or not full.is_file(): raise RuntimeError('Ordinary Git metadata required: '+str(path))
 before=full.stat(); data=full.read_bytes(); after=full.stat()
 if (before.st_dev,before.st_ino,before.st_size,before.st_mtime_ns,before.st_ctime_ns)!=(after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns,after.st_ctime_ns): raise RuntimeError('Git metadata changed: '+str(path))
 return data
head=git('rev-parse','HEAD').strip().decode(); top=git('rev-parse','HEAD^{tree}').strip().decode(); created=git('log','-1','--format=%cI').strip().decode()
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
def private(*args):
 result=subprocess.run(['git','--git-dir='+str(target),'--work-tree='+str(root),*args],cwd=root,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=120)
 if result.returncode: raise RuntimeError('Independent Git rejected captured metadata '+str(args)+': '+result.stderr.decode(errors='replace'))
 return result.stdout
if private('rev-parse','HEAD').strip().decode()!=head or private('log','-1','--format=%cI').strip().decode()!=created: raise RuntimeError('Independent genuine revision/time mismatch')
status=private('status','--porcelain','--untracked-files=no'); original=git('-c','status.renames=false','status','--porcelain','--untracked-files=no')
if bool(status)!=bool(original): raise RuntimeError('Actual dirty classifier diverged across metadata observations')
files=[]
for file in sorted(target.rglob('*')):
 if file.is_file():
  data=file.read_bytes(); files.append({'path':str(file.relative_to(out)),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
report={'version':1,'capturedAt':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),'head':head,'created':created,'objects':len(oids),'bytes':sum(row['bytes'] for row in files),'sourceBlobs':0,'packSha1':checksum.hex(),'privateGitStatusBytes':len(status),'originalGitStatusBytes':len(original),'statusEqualAtSeparateObservations':status==original,'files':files,'configuration':'Private operational core format0/bare=false/filemode=true/fsmonitor=false, status.renames=false prevents historical blob similarity reads; actual HEAD/ref/index/commits/HEADtrees retained; no remote or host user configuration copied','limits':{'maximumBytes':limit,'maximumMilliseconds':300000},'scope':'Authentic HEAD and ancestor commits, current HEAD reachable trees and actual index; no historical trees or blobs. Intended for read-only rev-parse/log-1/status provenance, not a full Git clone.'}
(out/'git-metadata.json').write_text(json.dumps(report,indent=2)+'\n'); print('[DEBUG] Genuine metadata verified by system Git '+json.dumps({k:v for k,v in report.items() if k!='files'}),flush=True)
`;
const child=Bun.spawn(["python3","-c",code,root,output],{stdin:"ignore",stdout:"inherit",stderr:"inherit"});
const stop=()=>child.kill();process.on("SIGINT",stop);process.on("SIGTERM",stop);
const timer=setTimeout(stop,310000);
try{const status=await child.exited;if(status!==0)throw Error("Genuine Git metadata capture failed: "+status);writeFileSync(join(ticket,"2026-10-19-git-input.json"),JSON.stringify({version:1,root:output,manifest:join(output,"git-metadata.json"),sourceBlobs:0},null,2)+"\n");}finally{clearTimeout(timer);process.off("SIGINT",stop);process.off("SIGTERM",stop);}

async function transportGit():Promise<void>{
 const receipt=JSON.parse(readFileSync(join(ticket,"🗑️generated/private-source-capture-receipt.json"),"utf8")),manifestBytes=readFileSync(join(output,"git-metadata.json")),manifest=JSON.parse(manifestBytes.toString("utf8")),manifestSha256=createHash("sha256").update(manifestBytes).digest("hex");
 if(!/^semio-course-source-[a-f0-9]{12}-[a-f0-9]{8}$/u.test(receipt.volume)||!/^sha256:[a-f0-9]{64}$/u.test(receipt.image)||manifest.bytes>512*1024**2)throw Error("Bounded owned Source volume/Git supplement required");
 const inspect=Bun.spawn(["docker","volume","inspect",receipt.volume],{stdout:"pipe",stderr:"inherit"});const volume=JSON.parse(await new Response(inspect.stdout).text())[0];if(await inspect.exited!==0||volume.Labels?.["semio.ticket"]!=="teaching-architecture-deploy-ready")throw Error("Owned private Source volume required");
 const pack=String.raw`import sys,pathlib,json,tarfile,hashlib,io
root=pathlib.Path(sys.argv[1]); manifest=json.loads((root/'git-metadata.json').read_text()); total=0
with tarfile.open(fileobj=sys.stdout.buffer,mode='w|',format=tarfile.PAX_FORMAT) as archive:
 for row in manifest['files']:
  path=root/row['path']; data=path.read_bytes(); total+=len(data)
  assert path.is_file() and not path.is_symlink() and len(data)==row['bytes'] and hashlib.sha256(data).hexdigest()==row['sha256'] and total<=536870912
  entry=tarfile.TarInfo(row['path']); entry.size=len(data); entry.mode=0o444; archive.addfile(entry,io.BytesIO(data))
 data=(root/'git-metadata.json').read_bytes(); entry=tarfile.TarInfo('.semio-private/git.json'); entry.size=len(data); entry.mode=0o444; archive.addfile(entry,io.BytesIO(data))
`;
 const verify="const fs=require('node:fs'),crypto=require('node:crypto'),v=fs.readFileSync('/repo/.semio-private/git.json');if(crypto.createHash('sha256').update(v).digest('hex')!=="+JSON.stringify(manifestSha256)+")throw Error('Git manifest digest mismatch');const m=JSON.parse(v);let n=0;for(const f of m.files){const s=fs.lstatSync('/repo/'+f.path),b=fs.readFileSync('/repo/'+f.path);if(!s.isFile()||s.isSymbolicLink()||b.length!==f.bytes||crypto.createHash('sha256').update(b).digest('hex')!==f.sha256)throw Error('Git metadata changed '+f.path);n+=b.length;}fs.mkdirSync('/repo/.git/info',{recursive:true});fs.writeFileSync('/repo/.git/info/exclude','.semio-private/\\n');if(fs.readFileSync('/repo/.git/info/exclude').at(-1)!==10)throw Error('Private exclude newline mismatch');console.log('[DEBUG] Guest authentic Git metadata hash proof '+JSON.stringify({head:m.head,files:m.files.length,bytes:n,sourceBlobs:m.sourceBlobs}));";
 const quote=(value:string)=>"'"+value.replaceAll("'","'\\''")+"'",name=receipt.volume+"-git",producer=Bun.spawn(["python3","-c",pack,output],{stdin:"ignore",stdout:"pipe",stderr:"inherit"}),consumer=spawn("docker",["run","--rm","--pull=never","-i","--name",name,"--network=none","--platform","linux/amd64","--memory=256m","--pids-limit=128","--cpus=2","--mount","type=volume,source="+receipt.volume+",target=/repo",receipt.image,"sh","-ec","test ! -e /repo/.git; tar -xf - -C /repo; /repo/.semio-private/bun -e "+quote(verify)],{stdio:["pipe","inherit","inherit"]});
 const terminal=new Promise<number>((resolve,reject)=>{consumer.once("error",reject);consumer.once("close",status=>resolve(status??1));});void terminal.catch(()=>{});
 const stop=()=>{producer.kill();consumer.kill();},timer=setTimeout(stop,180000);process.on("SIGINT",stop);process.on("SIGTERM",stop);
 try{let bytes=0;async function* bounded(){for await(const chunk of producer.stdout){bytes+=chunk.length;if(bytes>512*1024**2)throw Error("Git metadata stream bound exceeded");yield chunk;}}await pipeline(Readable.from(bounded(),{objectMode:false}),consumer.stdin!);if(await producer.exited!==0||await terminal!==0)throw Error("Genuine Git supplement transport refused");writeFileSync(join(ticket,"🗑️generated/git-transport-receipt.json"),JSON.stringify({version:1,completedAt:new Date().toISOString(),volume:receipt.volume,head:manifest.head,manifestSha256,sourceBlobs:0,streamedBytes:bytes,guestHashProof:true,guestGitCommands:false},null,2)+"\n");}finally{clearTimeout(timer);process.off("SIGINT",stop);process.off("SIGTERM",stop);stop();const cleanup=Bun.spawn(["docker","rm","-f",name],{stdout:"ignore",stderr:"ignore"});await cleanup.exited;await Promise.allSettled([producer.exited,terminal]);}
}
