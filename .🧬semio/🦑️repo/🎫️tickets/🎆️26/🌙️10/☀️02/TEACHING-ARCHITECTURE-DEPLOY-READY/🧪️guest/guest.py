import os,sys,json,stat,hashlib,base64,time,select,selectors,signal,subprocess,unicodedata
MAX_PATH=256
def units(value): return len(value.encode("utf-16-le"))//2
def digest(data): return hashlib.sha256(data).hexdigest()
def safe(name):
 if not isinstance(name,str) or not name or name!=unicodedata.normalize("NFC",name) or name.startswith("/") or any(c in name for c in "\\:\0") or any(ord(c)<32 or ord(c)==127 for c in name) or any(p in ("",".","..") for p in name.split("/")): raise ValueError("Canonical relative data path required")
 return name
def stamp(s): return(s.st_dev,s.st_ino,s.st_mode,s.st_nlink,s.st_size,s.st_mtime_ns,s.st_ctime_ns)
def ancestry(path):
 current=os.path.dirname(path);rows=[]
 while True:
  s=os.lstat(current)
  if not stat.S_ISDIR(s.st_mode) or stat.S_ISLNK(s.st_mode): raise ValueError("Physical ancestry required")
  rows.append((current,(s.st_dev,s.st_ino,s.st_mode)))
  parent=os.path.dirname(current)
  if parent==current: return rows
  current=parent
def stable(path,maximum):
 if not os.path.isabs(path) or os.path.normpath(path)!=path or units(path)>MAX_PATH: raise ValueError("Bounded physical file required")
 parents=ancestry(path);before=os.lstat(path)
 if not stat.S_ISREG(before.st_mode) or before.st_nlink!=1 or before.st_size>maximum: raise ValueError("Bounded single-link file required")
 fd=os.open(path,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK)
 try:
  if stamp(os.fstat(fd))!=stamp(before): raise ValueError("Physical descriptor changed")
  chunks=[];remaining=before.st_size
  while remaining:
   b=os.read(fd,min(65536,remaining))
   if not b: raise ValueError("Exact finite file required")
   remaining-=len(b);chunks.append(b)
  if os.read(fd,1) or stamp(os.fstat(fd))!=stamp(before) or stamp(os.lstat(path))!=stamp(before): raise ValueError("Physical input changed")
  for parent,identity in parents:
   s=os.lstat(parent)
   if (s.st_dev,s.st_ino,s.st_mode)!=identity or not stat.S_ISDIR(s.st_mode): raise ValueError("Ancestry changed")
  return b"".join(chunks),list(stamp(before))
 finally: os.close(fd)
def write_loop(fd,data,write=os.write):
 offset=0
 while offset<len(data):
  n=write(fd,data[offset:offset+65536])
  if not isinstance(n,int) or n<=0: raise ValueError("Finite write made no progress")
  offset+=n
def publish_json(path,value,write=os.write):
 ancestry(path);fd=os.open(path,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
 try: write_loop(fd,json.dumps(value,separators=(",",":"),ensure_ascii=False).encode(),write);os.fsync(fd)
 finally: os.close(fd)
def open_dirs(parent,parts,exclusive_last=False):
 owned=os.dup(parent)
 try:
  for index,name in enumerate(parts):
   safe(name)
   try: os.mkdir(name,0o700,dir_fd=owned)
   except FileExistsError:
    if exclusive_last and index==len(parts)-1: raise
   child=os.open(name,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW,dir_fd=owned);os.close(owned);owned=child
  return owned
 except BaseException: os.close(owned);raise
def write_file(rootfd,name,data):
 parts=safe(name).split("/");parent=open_dirs(rootfd,parts[:-1])
 try:
  fd=os.open(parts[-1],os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600,dir_fd=parent)
  try: write_loop(fd,data);os.fsync(fd)
  finally: os.close(fd)
 finally: os.close(parent)
def read_packet(data,expected):
 packet=json.loads(data)
 if set(packet)!=set(("version","kind","files")) or packet["version"]!=1 or packet["kind"]!="captured-neutral-ticket-package" or not isinstance(packet["files"],list): raise ValueError("Closed Ticket package required")
 paths=[r["path"] for r in packet["files"]]
 if len(paths)!=len(set(paths)) or paths!=expected: raise ValueError("Exact ordered Ticket payload roster required")
 for row in packet["files"]:
  if set(row)!=set(("path","bytes","sha256","base64")): raise ValueError("Closed payload row required")
  safe(row["path"]);body=base64.b64decode(row["base64"],validate=True)
  if len(body)!=row["bytes"] or digest(body)!=row["sha256"] or len(body)>16777216: raise ValueError("Exact Ticket bytes required")
 return packet
def receive_files(root,ticket,packet):
 ancestry(root+"/leaf");base=os.open(root,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW)
 try: destination=open_dirs(base,safe(ticket).split("/"),True)
 finally: os.close(base)
 try:
  for row in packet["files"]:
   full=root+"/"+ticket+"/"+row["path"]
   if units(full)>256: raise ValueError("Bounded Ticket leaf required")
   write_file(destination,row["path"],base64.b64decode(row["base64"],validate=True))
  os.mkdir("🗑️generated",0o700,dir_fd=destination);os.fsync(destination)
 finally: os.close(destination)
 return [{"path":r["path"],"identity":stable(root+"/"+ticket+"/"+r["path"],16777216)[1]} for r in packet["files"]]
def source_pins(root,manifest,pins):
 rows={};identities={}
 for row in manifest["rows"]:
  safe(row["path"])
  if row["path"] in rows: raise ValueError("Unique captured Source rows required")
  rows[row["path"]]=row
 for path,pin in pins.items():
  safe(path)
  if any(p in (".🧬semio",".git",".semio-private","🗑️generated","node_modules") for p in path.split("/")): raise ValueError("Source stays outside Ticket")
  row=rows.get(path)
  if row is None or row["bytes"]!=pin["bytes"] or row["sha256"]!=pin["sha256"]: raise ValueError("Copied combined pins must derive from captured Source")
  body,identity=stable(root+"/"+path,pin["bytes"])
  if len(body)!=pin["bytes"] or digest(body)!=pin["sha256"]: raise ValueError("Captured guest Source changed")
  identities[path]=identity
 return identities
def context(wire):
 if wire["ticket"]!=wire["policy"]["ticket"] or not all(isinstance(wire[k],str) and len(wire[k])==32 and all(c in "0123456789abcdef" for c in wire[k]) for k in ("cohort","captureId","operation")): raise ValueError("Closed neutral operation required")
 base="/tmp/semio-neutral-"+wire["operation"]
 return base,{"binding":wire["binding"],"cohort":wire["cohort"],"operation":wire["operation"],"containerId":wire["containerId"]}
def require_owner(base,owner):
 if os.path.realpath(base)!=base or json.loads(stable(base+"/owner.json",2048)[0])!=owner: raise ValueError("Exact operation custody required")
 return base
def physical(wire):
 base,owner=context(wire);return require_owner(base,owner)
def source_manifest(wire):
 body,_=stable("/repo/.semio-private/source.json",wire["source"]["bytes"])
 if len(body)!=wire["source"]["bytes"] or digest(body)!=wire["source"]["sha256"]: raise ValueError("Actual captured manifest required")
 value=json.loads(body)
 if value["captureId"]!=wire["captureId"] or value["kind"]!="fresh-source-manifest": raise ValueError("Actual captured epoch required")
 return value
def native_check(wire):
 body,_=stable(wire["native"]["path"],wire["native"]["bytes"])
 s=os.lstat(wire["native"]["path"])
 if len(body)!=wire["native"]["bytes"] or digest(body)!=wire["native"]["sha256"] or not s.st_mode&0o111 or s.st_mode&0o6000: raise ValueError("Exact audited native required")
def ticket_check(wire,transfer):
 root="/repo/"+wire["ticket"]
 for path,identity in transfer["directories"].items():
  s=os.lstat(path)
  if not stat.S_ISDIR(s.st_mode) or [s.st_dev,s.st_ino,s.st_mode]!=identity: raise ValueError("Persistent Ticket ancestry changed")
 for path,identity in transfer["authorityIdentities"].items():
  if list(stamp(os.lstat(path)))!=identity: raise ValueError("Captured manifest or native identity changed")
 for row in transfer["files"]:
  body,identity=stable(root+"/"+row["path"],row["bytes"])
  if len(body)!=row["bytes"] or digest(body)!=row["sha256"] or identity!=row["identity"]: raise ValueError("Transferred Ticket custody changed")
 raw,_=stable(root+"/📦️fresh/combined.inputs.json",1048576)
 if digest(raw)!=wire["manifestSha256"]: raise ValueError("Raw combined manifest changed")
 manifest=json.loads(raw);captured=source_manifest(wire);observed=source_pins("/repo",captured,manifest["sourceFiles"])
 if "sourceIdentities" in transfer and observed!=transfer["sourceIdentities"]: raise ValueError("Persistent captured Source identities changed")
 transfer["sourceIdentities"]=observed
 contract=json.loads(stable(root+"/📦️fresh/combined.schema.json",16777216)[0]);roster=contract["properties"]["sourceFiles"]
 if roster.get("additionalProperties") is not False or set(roster["required"])!=set(roster["properties"]) or set(roster["required"])!=set(manifest["sourceFiles"]) or roster["required"]!=wire["policy"]["sourcePaths"]: raise ValueError("Exact copied Source roster required")
 if len(manifest["sourceFiles"])!=wire["sourceCount"]: raise ValueError("Exact captured pin count required")
 return manifest
def receipt_check(wire):
 root="/repo/"+wire["ticket"]+"/🗑️generated"
 names=os.listdir(root)
 if len(names)!=1 or not names[0].startswith("n-"): raise ValueError("One fresh actual neutral output required")
 path=root+"/"+names[0]+"/receipt.json";raw,_=stable(path,1048576);receipt=json.loads(raw)
 if receipt.get("kind")!="architecture-combined-neutral-receipt" or receipt.get("accepted") is not True or receipt.get("manifestSha256")!=wire["manifestSha256"] or receipt.get("phaseCount")!=len(wire["policy"]["phases"]) or receipt.get("sourceInputFiles")!=wire["sourceCount"] or receipt.get("parentTerminalProven") is not False or [r["id"] for r in receipt["results"]]!=wire["policy"]["phases"]: raise ValueError("Actual complete neutral receipt required")
 for result in receipt["results"]:
  if result.get("accepted") is not True or result.get("status")!=0 or result.get("signal") is not None or result.get("ownedCallSettled") is not True or result.get("collection",{}).get("accepted") is not True: raise ValueError("Actual complete collection required")
 return {"path":path,"bytes":len(raw),"sha256":digest(raw),"base64":base64.b64encode(raw).decode()}
def receive(wire):
 base,owner=context(wire);ancestry(base);os.mkdir(base,0o700);publish_json(base+"/owner.json",owner)
 try:
  source_manifest(wire);native_check(wire);data=bytearray();deadline=time.monotonic()+45
  while True:
   left=deadline-time.monotonic()
   if left<=0 or not select.select([0],[],[],left)[0]: raise TimeoutError("Ticket transfer deadline")
   chunk=os.read(0,min(65536,33554433-len(data)))
   if not chunk: break
   data.extend(chunk)
   if len(data)>33554432: raise ValueError("Bounded encoded Ticket transfer required")
  if digest(data)!=wire["packageSha256"]: raise ValueError("Exact reviewed Ticket package required")
  packet=read_packet(data,wire["policy"]["files"]+["📦️fresh/combined.inputs.json"]);identities=receive_files("/repo",wire["ticket"],packet)
  files=[dict(row,identity=identity["identity"]) for row,identity in zip(packet["files"],identities)]
  for row in files: del row["base64"]
  directories={}
  for row in files:
   for parent,identity in ancestry("/repo/"+wire["ticket"]+"/"+row["path"]): directories[parent]=list(identity)
  authorities={path:list(stamp(os.lstat(path))) for path in ("/repo/.semio-private/source.json",wire["native"]["path"])}
  transfer={"directories":directories,"authorityIdentities":authorities,"binding":wire["binding"],"packageSha256":wire["packageSha256"],"receiverTerminal":True,"accepted":True,"files":files}
  ticket_check(wire,transfer);publish_json(base+"/transfer.json",transfer);print(json.dumps({"binding":wire["binding"],"accepted":True,"receiverTerminal":True,"packageSha256":wire["packageSha256"],"files":len(files)}))
 except BaseException as error:
  publish_json(base+"/transfer-terminal.json",{"binding":wire["binding"],"receiverTerminal":True,"accepted":False,"error":str(error)[:512]});raise
def worker(wire):
 base=physical(wire);publish_json(base+"/worker.json",{"binding":wire["binding"],"pid":os.getpid()})
 transfer=json.loads(stable(base+"/transfer.json",1048576)[0])
 if transfer.get("binding")!=wire["binding"] or transfer.get("packageSha256")!=wire["packageSha256"] or transfer.get("accepted") is not True: raise ValueError("Actual transfer before native invocation required")
 terminal={"version":1,"kind":"captured-neutral-terminal","binding":wire["binding"],"operation":wire["operation"],"cohort":wire["cohort"],"containerId":wire["containerId"],"sourceManifestSha256":wire["source"]["sha256"],"packageSha256":wire["packageSha256"],"manifestSha256":wire["manifestSha256"],"nativeSha256":wire["native"]["sha256"],"native":{"launched":False,"joined":False,"status":None,"signal":None},"checks":{"source":False,"ticket":False,"native":False,"receipt":False},"drained":False,"accepted":False}
 child=None;log=None;reason=None;requested=[False]
 def cancelled(*_): requested[0]=True
 signal.signal(signal.SIGINT,cancelled);signal.signal(signal.SIGTERM,cancelled)
 try:
  ticket_check(wire,transfer);native_check(wire)
  if os.listdir("/repo/"+wire["ticket"]+"/🗑️generated"): raise ValueError("Fresh empty neutral output required")
  log=os.open(base+"/output.log",os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600);env=dict(os.environ);env["SEMIO_BUILD_MODE"]="release";env["SEMIO_BUILD_BUDGET_MS"]=str(wire["milliseconds"]);env["PATH"]=os.path.dirname(wire["native"]["path"])+":"+env.get("PATH","")
  for name in ("SEMIO_SCRIPT_PROCESS_INVOCATION","SEMIO_SCRIPT_POLICY","SEMIO_TEST_LEVEL"): env.pop(name,None)
  argv=[wire["native"]["path"],"exec","--projects=workspace","--","bun","/repo/"+wire["ticket"]+"/📦️fresh/combined.ts",wire["manifestSha256"]]
  child=subprocess.Popen(argv,cwd="/repo",env=env,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True);terminal["native"]["launched"]=True
  selector=selectors.DefaultSelector()
  for stream in (child.stdout,child.stderr): os.set_blocking(stream.fileno(),False);selector.register(stream,selectors.EVENT_READ)
  started=time.monotonic();stopping=None;level=0;count=0;lines=0;output_hash=hashlib.sha256()
  while selector.get_map() or child.poll() is None:
   physical(wire)
   if requested[0]: reason=reason or "cancelled"
   try:
    cancel=json.loads(stable(base+"/cancel.json",2048)[0])
    if cancel!={"binding":wire["binding"]}: raise ValueError("Exact cancellation binding required")
    reason=reason or "cancelled"
   except FileNotFoundError: pass
   if time.monotonic()-started>=wire["milliseconds"]/1000: reason=reason or "deadline"
   if reason and stopping is None: stopping=time.monotonic()
   if stopping is not None and child.poll() is None:
    wanted=2 if time.monotonic()-stopping>=2 else 1
    if wanted>level:
     level=wanted
     try: os.killpg(child.pid,signal.SIGKILL if level==2 else signal.SIGTERM)
     except ProcessLookupError: pass
   for key,_ in selector.select(0.1):
    chunk=os.read(key.fileobj.fileno(),65536)
    if not chunk: selector.unregister(key.fileobj);key.fileobj.close();continue
    if count+len(chunk)>wire["policy"]["outputBytes"] or lines+chunk.count(b"\n")>wire["policy"]["outputLines"]: reason=reason or "output-limit";continue
    write_loop(log,chunk);count+=len(chunk);lines+=chunk.count(b"\n");output_hash.update(chunk)
   if stopping is not None and time.monotonic()-stopping>=20: break
  terminal["drained"]=not selector.get_map()
  selector.close()
  if child.poll() is None: raise ValueError("Native child terminal unconfirmed")
  code=child.wait();terminal["native"].update({"joined":True,"status":code if code>=0 else None,"signal":signal.Signals(-code).name if code<0 else None})
  terminal["output"]={"bytes":count,"lines":lines,"sha256":output_hash.hexdigest()}
  if reason is not None: raise ValueError(reason)
  if code!=0 or not terminal["drained"]: raise ValueError("Actual joined drained native zero required")
  ticket_check(wire,transfer);terminal["checks"]["ticket"]=True;source_manifest(wire);terminal["checks"]["source"]=True;native_check(wire);terminal["checks"]["native"]=True
  terminal["receipt"]=receipt_check(wire);terminal["checks"]["receipt"]=True;terminal["accepted"]=True
 except BaseException as error: terminal["error"]=str(error)[:512]
 finally:
  cleanup_errors=[]
  try:
   if child is not None and child.poll() is None:
    try: os.killpg(child.pid,signal.SIGTERM)
    except ProcessLookupError: pass
    try: child.wait(timeout=2)
    except subprocess.TimeoutExpired:
     try: os.killpg(child.pid,signal.SIGKILL)
     except ProcessLookupError: pass
     child.wait(timeout=10)
   if child is not None and child.poll() is not None and not terminal["native"]["joined"]:
    code=child.wait();terminal["native"].update({"joined":True,"status":code if code>=0 else None,"signal":signal.Signals(-code).name if code<0 else None})
  except BaseException as error: cleanup_errors.append(str(error)[:512])
  if log is not None:
   try: os.fsync(log)
   except BaseException as error: cleanup_errors.append(str(error)[:512])
   try: os.close(log)
   except BaseException as error: cleanup_errors.append(str(error)[:512])
  if cleanup_errors:
   terminal["cleanupErrors"]=cleanup_errors;terminal["accepted"]=False
   if "error" not in terminal: terminal["error"]=cleanup_errors[0]
 try: publish_json(base+"/terminal.json",terminal)
 except BaseException as error:
  print(json.dumps({"primary":terminal.get("error"),"terminalPublicationError":str(error)[:512]}),file=sys.stderr);return 1
 print(json.dumps(terminal,separators=(",",":"),ensure_ascii=False));return 0 if terminal["accepted"] else 1
def observe(wire,action):
 base=physical(wire)
 if action=="cancel":
  try: publish_json(base+"/cancel.json",{"binding":wire["binding"]})
  except FileExistsError:
   if json.loads(stable(base+"/cancel.json",2048)[0])!={"binding":wire["binding"]}: raise ValueError("Cancellation custody changed")
 result={"binding":wire["binding"],"terminal":None,"transferTerminal":None,"worker":None}
 for name,key,maximum in (("terminal.json","terminal",2097152),("transfer-terminal.json","transferTerminal",2048),("worker.json","worker",2048)):
  try: result[key]=json.loads(stable(base+"/"+name,maximum)[0])
  except FileNotFoundError: pass
 print(json.dumps(result,separators=(",",":"),ensure_ascii=False))
def neutral_physical_cases(artifact):
 import tempfile,shutil
 names=["short-write","partial-input","duplicate-path","existing-leaf","parent-symlink","hardlink","wrong-hash","extra-file","manifest-source-missing","duplicate-worker-claim","cancel-wrong-operation","terminal-write-failure"];results=[]
 for name in names:
  root=tempfile.mkdtemp(prefix="p-",dir=artifact);fd=None
  try:
   body=b"neutral";row={"path":"a","bytes":len(body),"sha256":digest(body),"base64":base64.b64encode(body).decode()};packet={"version":1,"kind":"captured-neutral-ticket-package","files":[row]}
   try:
    if name=="short-write":
     fd=os.open(root+"/a",os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o600);write_loop(fd,body,lambda *_:0)
    elif name=="partial-input": read_packet(b'{"version":1',["a"])
    elif name=="duplicate-path": packet["files"].append(dict(row));read_packet(json.dumps(packet).encode(),["a"])
    elif name=="wrong-hash": row["sha256"]="0"*64;read_packet(json.dumps(packet).encode(),["a"])
    elif name=="extra-file": packet["files"].append(dict(row,path="b"));read_packet(json.dumps(packet).encode(),["a"])
    elif name=="hardlink":
     open(root+"/a","wb").write(body);os.link(root+"/a",root+"/b");stable(root+"/a",64)
    elif name=="duplicate-worker-claim": publish_json(root+"/worker.json",{"binding":"a"});publish_json(root+"/worker.json",{"binding":"a"})
    elif name=="cancel-wrong-operation": publish_json(root+"/owner.json",{"binding":"foreign"});require_owner(root,{"binding":"own"})
    elif name=="terminal-write-failure": publish_json(root+"/terminal.json",{"accepted":True},lambda *_:0)
    elif name=="manifest-source-missing": source_pins(root,{"rows":[]},{"a":{"bytes":1,"sha256":"0"*64}})
    else:
     os.mkdir(root+"/t");base=os.open(root+"/t",os.O_RDONLY|os.O_DIRECTORY)
     try:
      if name=="existing-leaf": open(root+"/t/a","wb").write(body);write_file(base,"a",body)
      else: os.mkdir(root+"/outside");os.symlink(root+"/outside",root+"/t/d");write_file(base,"d/a",body)
     finally: os.close(base)
    results.append({"id":name,"accepted":True})
   except (ValueError,OSError,json.JSONDecodeError): results.append({"id":name,"accepted":False})
  finally:
   if fd is not None: os.close(fd)
   shutil.rmtree(root)
 return results
if __name__=="__main__":
 try:
  if len(sys.argv)!=3 or sys.argv[1] not in ("receive","run","observe","cancel"): raise ValueError("Closed neutral guest action required")
  wire=json.loads(sys.argv[2]);action=sys.argv[1]
  if action=="receive": receive(wire)
  elif action=="run": sys.exit(worker(wire))
  else: observe(wire,action)
 except BaseException as error:
  if isinstance(error,SystemExit): raise
  print(json.dumps({"error":str(error)[:512]}),file=sys.stderr);sys.exit(1)
