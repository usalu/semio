import sys,os,hashlib
base=sys.argv[1]
def load(p):return open(os.path.join(base,p),encoding="utf8").read()
files={p:load(p) for p in["🧬️schema/🔣️.json","📤️uploader/🟦️.ts","📤️uploader/🧪️tests/🟦️.ts"]}
out=dict(files)
def rep(p,a,b):
    assert out[p].count(a)==1,(p,a[:90],out[p].count(a));out[p]=out[p].replace(a,b)
rep("🧬️schema/🔣️.json",'''        "notBeforeMs": { "type": "integer", "minimum": 0 }
      }
    },
    "KeySamplesRef": {''','''        "notBeforeMs": { "type": "integer", "minimum": 0 },
        "deferredBy": { "description": "The fleet identity (endpoint, instance and credential) whose client was not usable (breaker open, credential denied) when the job was put off: a drainer with another identity takes it at once instead of after `notBeforeMs`.", "type": "string", "minLength": 1 }
      }
    },
    "KeySamplesRef": {''')
u="📤️uploader/🟦️.ts"
rep(u,"queuedMs:number;attempts?:number;notBeforeMs?:number};","queuedMs:number;attempts?:number;notBeforeMs?:number;deferredBy?:string};")
rep(u,"if(at===undefined){try{at=(JSON.parse(readFileSync(join(spool,name),\"utf8\")) as SpoolJobV1).notBeforeMs??0;}catch{at=0;}dueAt.set(name,at);}",
      "if(at===undefined){try{const waiting=JSON.parse(readFileSync(join(spool,name),\"utf8\")) as SpoolJobV1;at=waiting.deferredBy!==undefined&&waiting.deferredBy!==settings.identity?0:waiting.notBeforeMs??0;}catch{at=0;}dueAt.set(name,at);}")
rep(u,'''   const wait=(at:number,reason:string)=>{
    if(!rewriteJobV1(path,{...job,claimed:false,notBeforeMs:at}))return;''','''   const wait=(at:number,reason:string,deferredBy?:string)=>{
    const {deferredBy:_,...rest}=job;
    if(!rewriteJobV1(path,{...rest,claimed:false,notBeforeMs:at,...deferredBy===undefined?{}:{deferredBy}}))return;''')
rep(u,'if(!client.usable()){wait(started+policy.retryBackoffMilliseconds*(0.5+Math.random()),"upload-deferred: "+client.state());await release();return false;}',
      'if(!client.usable()){wait(started+policy.retryBackoffMilliseconds*(0.5+Math.random()),"upload-deferred: "+client.state(),settings.identity);await release();return false;}')
t="📤️uploader/🧪️tests/🟦️.ts"
rep(t,'''test("offers of local hits fill at most half of the spool''','''test("a job put off while the credential was denied waits for a drainer of that credential but is taken at once by a drainer with another credential, not after the retry delay",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("srv-cred"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(server);
 const cas=directory("cas-cred"),root=directory("cache-cred"),out=directory("out-cred"),right={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas},wrong={...right,SEMIO_REMOTE_CACHE_TOKEN:"wrong-token"};
 process.env.SEMIO_CAS_ROOT=cas;
 writeFileSync(join(out,"libcore-1.rlib"),"rlib bytes");writeFileSync(join(out,"core-1.d"),`${out}/core-1.d: ${out}/lib.rs\\n`);writeFileSync(join(out,"lib.rs"),"pub fn id()->u32{1}\\n");
 const localKey="8".repeat(64),key="c".repeat(64),denied=readFleetSettingsV1(root,wrong)!.identity;
 expect(publishUnitV1(root,{key:localKey,crate:"core",cwd:out,outDir:out,laneRoot:undefined,outputs:[join(out,"libcore-1.rlib"),join(out,"core-1.d")],stdout:"",stderr:"",stampDirectory:undefined})).toBeDefined();
 expect(enqueueUploadV1(root,job({key,localKey,cwd:out,queuedMs:Date.now(),claimed:false}))).toBe(true);
 mkdirSync(join(root,fleet.layout.state),{recursive:true});writeFileSync(join(root,fleet.layout.state,fleet.layout.denied),denied);
 expect(await drainUploadsV1(root,wrong,{idleMilliseconds:0})).toMatchObject({uploaded:0,failed:0,dropped:0,deferred:1});
 const waiting=join(root,fleet.layout.state,fleet.layout.spool),held=JSON.parse(readFileSync(join(waiting,readdirSync(waiting).find(name=>name.endsWith(".json"))!),"utf8")) as SpoolJobV1;
 const ajv=new Ajv({strict:false});ajv.addSchema(schema);expect(ajv.getSchema(`${schema.$id}#/definitions/SpoolJob`)!(held)).toBe(true);
 expect(held.deferredBy).toBe(denied);expect(held.notBeforeMs!,"put off by the retry delay").toBeGreaterThan(Date.now());
 expect(await drainUploadsV1(root,wrong,{idleMilliseconds:0}),"the same credential leaves the job until it is due").toMatchObject({uploaded:0,failed:0,dropped:0});
 expect(spoolLengthV1(root)).toBe(1);
 expect(await drainUploadsV1(root,right,{idleMilliseconds:0}),"another credential takes it at once").toMatchObject({uploaded:1,failed:0});
 expect(spoolLengthV1(root)).toBe(0);expect(await createFleetClientV1(readFleetSettingsV1(root,right)!).hasResult(key)).toBe(true);
},120000);

test("offers of local hits fill at most half of the spool''')
for p,s in out.items():
    full=os.path.join(base,p);open(full+".t3tmp","w",encoding="utf8").write(s)
for p,s in out.items():
    full=os.path.join(base,p);os.replace(full+".t3tmp",full)
    print(p,hashlib.sha256(files[p].encode()).hexdigest()[:12],"->",hashlib.sha256(s.encode()).hexdigest()[:12])
