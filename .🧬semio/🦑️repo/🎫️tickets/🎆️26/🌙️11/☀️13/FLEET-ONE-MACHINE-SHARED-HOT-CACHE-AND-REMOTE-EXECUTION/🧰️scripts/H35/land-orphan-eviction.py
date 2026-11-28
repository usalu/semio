import sys;sys.path.insert(0,__import__("os").path.dirname(__file__))
from lib import edit
C="🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/"
edit(C+"📋️model/🟦️.ts",[
("function finishNativeArtifactCacheRetirementV1(state:NativeArtifactCacheCatalogV1,slot:number):NativeArtifactCacheCatalogV1{",
 "/** 🧟️ Drops a dirty slot whose writer died before it charged any storage; only a dirty slot without a retirement binding can be an orphan, a clean or retiring slot always owns storage. */\nfunction retireOrphanNativeArtifactCacheSlotV1(state:NativeArtifactCacheCatalogV1,slot:number):NativeArtifactCacheCatalogV1{const next=readNativeArtifactCacheCatalogV1(state),row=next.slots.find(s=>s.slot===slot);if(row?.state!==\"dirty\"||row.retirement!==null)throw Error(\"Exact orphaned compiler slot required\");next.slots=next.slots.filter(s=>s.slot!==slot);return readNativeArtifactCacheCatalogV1(next);}\nfunction finishNativeArtifactCacheRetirementV1(state:NativeArtifactCacheCatalogV1,slot:number):NativeArtifactCacheCatalogV1{"),
("retireNativeArtifactCacheSlotsV1,finishNativeArtifactCacheRetirementV1});}","retireNativeArtifactCacheSlotsV1,finishNativeArtifactCacheRetirementV1,retireOrphanNativeArtifactCacheSlotV1});}"),
])
V=C+"💾️volume/🟦️.ts"
edit(V,[
("import{mkdirSync,lstatSync,existsSync,openSync,closeSync,writeFileSync,fsyncSync,renameSync,rmSync}from\"node:fs\";","import{mkdirSync,lstatSync,existsSync,openSync,closeSync,writeFileSync,fsyncSync,renameSync,rmSync,readdirSync,rmdirSync}from\"node:fs\";"),
("function imagePolicy(image:NativeImageStorageV1):","/** 🧟️ Finishes the retirement of a dirty slot whose writer died before it bound its charge: the empty producer directory goes, then the slot. */\nfunction dropOrphan(state:NativeArtifactCacheCatalogV1,slot:number,paths:ReturnType<typeof nativeArtifactCacheProducerLayoutV1>,check:()=>void):void{if(existsSync(paths.directory)){if(readdirSync(paths.directory).length)throw new Error(\"Orphaned producer directory is not empty: \"+paths.directory);rmdirSync(paths.directory);}check();write(layout.catalog,model.retireOrphanNativeArtifactCacheSlotV1(state,slot));}\nfunction imagePolicy(image:NativeImageStorageV1):"),
("bound=(()=>{try{return record(paths.charge,\"Binding\")as Binding;}catch(error){throw (error as NodeJS.ErrnoException).code===\"ENOENT\"?new Error(\"Producer charge record missing for a \"+entry!.state+\" slot: \"+paths.charge,{cause:error}):error;}})(),observed=await observeNativeStorageStateV1({policy:request,control:actual,port:opened.image.port}),allocation=observed.allocations.find(row=>row.id===bound.allocation);",
 "missing=(():{bound:Binding}|{absent:unknown}=>{try{return{bound:record(paths.charge,\"Binding\")as Binding};}catch(error){if((error as NodeJS.ErrnoException).code!==\"ENOENT\")throw error;return{absent:error};}})(),observed=await observeNativeStorageStateV1({policy:request,control:actual,port:opened.image.port});if(\"absent\" in missing){const owned=observed.allocations.filter(row=>row.owner===producerOwner(candidate.context)),stray=owned[0],orphan=entry!.state===\"dirty\"&&entry!.retirement===null&&!existsSync(paths.intent)&&owned.length<=1&&(!stray||stray.kind===\"compiler-state\"&&stray.phase===\"committed\"&&!!stray.child&&stray.child.directory===paths.directory&&stray.child.policySha256===policyDigest&&stray.child.parentIdentity===metadata.backing.identity);if(!orphan)throw new Error(\"Producer charge record missing for a \"+entry!.state+\" slot: \"+paths.charge,{cause:missing.absent});return{orphan:true as const,entry:structuredClone(entry!),reservation:stray?await recoverNativeStorageReservationV1({policy:request,control:actual,port:opened.image.port,id:stray.id}):undefined,paths};}const bound=missing.bound,allocation=observed.allocations.find(row=>row.id===bound.allocation);"),
("return{entry:structuredClone(entry!),reservation,bound,paths};});if(!held)return false;",
 "return{orphan:false as const,entry:structuredClone(entry!),reservation,bound,paths};});if(!held)return false;if(held.orphan){if(held.reservation){await retireNativePlatformFundedChildStorageV1(opened.image,held.reservation,actual,actual,invocation);victim.check();}await catalogue(async(state,check)=>{const entry=state.slots.find(row=>row.slot===held.entry.slot);if(!sameEntry(entry,held.entry))throw Error(\"Exact orphaned producer retirement metadata CAS required\");dropOrphan(state,held.entry.slot,held.paths,check);});return true;}"),
])
edit(C+"🧬️schema/🔣️.json",[
('    "ChildRetireCase": {','''    "OrphanCase": {
      "type": "object",
      "additionalProperties": false,
      "required": ["name", "state", "digest", "identity", "accepted"],
      "properties": {
        "name": {"type": "string", "minLength": 1, "maxLength": 64},
        "state": {"enum": ["clean", "dirty", "retiring"]},
        "digest": {"anyOf": [{"type": "string", "pattern": "^[a-f0-9]{64}$"}, {"type": "null"}]},
        "identity": {"type": "boolean"},
        "accepted": {"type": "boolean"}
      }
    },
    "ChildRetireCase": {'''),
])
edit(C+"🧪️tests/🔣️.json",[
('  "childRetirements": [','''  "orphanRetirements": [
    {"name": "dirty-writer-died-before-charge", "state": "dirty", "digest": null, "identity": false, "accepted": true},
    {"name": "clean-slot-owns-storage", "state": "clean", "digest": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "identity": false, "accepted": false},
    {"name": "retiring-slot-owns-storage", "state": "retiring", "digest": null, "identity": true, "accepted": false}
  ],
  "childRetirements": ['''),
])
edit(C+"🧪️tests/🟦️.ts",[
('test("neutral Core cache dirty child retirement retains exact original identity",','''test("neutral Core cache orphaned dirty slot of a crashed writer is evictable and every other slot is not",()=>{const validate=new Ajv({strict:true}).compile(schema.definitions.OrphanCase),model=nativeArtifactCacheModelV1(readNativeArtifactCachePolicyV1(corpus.smallPolicy)),db=new Database(":memory:");try{for(const row of corpus.orphanRetirements){expect(validate(row)).toBe(true);const oracle=(db.query("select ?1='dirty' admitted").get(row.state)as{admitted:number}).admitted===1;expect(oracle,row.name).toBe(row.accepted);const catalog={version:1 as const,sequence:1,slots:[{slot:0,context:"a".repeat(64),content:"b".repeat(64),state:row.state,sequence:1,digest:row.digest,retirement:row.identity?{dev:1,ino:2}:null}]};let accepted=true,left=-1;try{left=model.retireOrphanNativeArtifactCacheSlotV1(catalog,0).slots.length;}catch{accepted=false;}expect(accepted,row.name).toBe(oracle);if(accepted)expect(left).toBe(0);}expect(()=>model.retireOrphanNativeArtifactCacheSlotV1({version:1,sequence:1,slots:[]},0)).toThrow("orphaned");}finally{db.close();}});
test("neutral Core cache dirty child retirement retains exact original identity",'''),
])
