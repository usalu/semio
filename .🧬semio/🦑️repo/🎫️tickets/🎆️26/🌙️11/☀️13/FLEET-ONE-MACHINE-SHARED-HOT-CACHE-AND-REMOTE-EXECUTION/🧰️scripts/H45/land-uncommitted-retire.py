import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from lib import edit
P="🧰️framework/🔨️modules/🏃️process/📦️artifacts/"
edit(P+"🏗️native-build/💾️storage/🟦️.ts",[
('/** 💾️ Atomically checkpoints only the exact serialized durable state;',
 '/** 🪦️ Retires the journal charge of one crashed uncommitted compiler-state reservation whose physical child never existed, removing its empty recorded namespace first; the caller holds the exclusive lease proving its writer dead. */\nexport function retireNativeStorageUncommittedReservationV1(options:StorageCustodyOptionsV1&Readonly<{id:string}>):Promise<void>{\n options=Object.freeze({...options,policy:Object.freeze({...options.policy})});return options.port.withOperation(options.control,async control=>{const custody=await storageCustodyV1({...options,control}),check=()=>checkResourceLeaseControl(control);await custody.transaction(control,async(state,append)=>{const allocation=state.allocations.find(row=>row.id===options.id);if(!allocation||allocation.phase!=="reserved"||allocation.kind!=="compiler-state"||!allocation.child)throw Error("Original uncommitted child reservation required");if(existsSync(allocation.child.directory))throw Error("Physical child of an uncommitted reservation remains charged");const receipt=allocation.namespace;if(receipt&&existsSync(receipt.directory)){verifyNativeStorageNamespaceV1(allocation,custody.backing.root,check);removeNativeStorageNamespaceEntryV1(custody.backing.root,receipt,check);await options.port.syncMetadata(custody.backing,custody.backing.root,control);check();}append({type:"retiring",id:allocation.id});append({type:"retired",id:allocation.id});});});\n}\n\n/** 💾️ Atomically checkpoints only the exact serialized durable state;'),
])
edit(P+"💾️cache/💾️volume/🟦️.ts",[
('recoverNativeStorageReservationV1,observeNativeStorageStateV1,type NativeStorageReservationV1}',
 'recoverNativeStorageReservationV1,retireNativeStorageUncommittedReservationV1,observeNativeStorageStateV1,type NativeStorageReservationV1}'),
('return{orphan:true as const,entry:structuredClone(entry!),reservation:stray?await recoverNativeStorageReservationV1({policy:request,control:actual,port:opened.image.port,id:stray.id}):undefined,paths};',
 'const uncommitted=!!stray&&stray.phase==="reserved";return{orphan:true as const,entry:structuredClone(entry!),reservation:stray&&!uncommitted?await recoverNativeStorageReservationV1({policy:request,control:actual,port:opened.image.port,id:stray.id}):undefined,uncommitted:uncommitted?stray!.id:undefined,paths};'),
('if(held.orphan){if(held.reservation){await retireNativePlatformFundedChildStorageV1(opened.image,held.reservation,actual,actual,invocation);victim.check();}',
 'if(held.orphan){if(held.reservation){await retireNativePlatformFundedChildStorageV1(opened.image,held.reservation,actual,actual,invocation);victim.check();}else if(held.uncommitted){await retireNativeStorageUncommittedReservationV1({policy:request,control:actual,port:opened.image.port,id:held.uncommitted});victim.check();}'),
])
edit(P+"💾️cache/📍️layout/🟦️.ts",[
('import{join,resolve,isAbsolute}from"node:path";',
 'import{join,resolve,isAbsolute}from"node:path";\nimport{readdirSync,rmdirSync}from"node:fs";'),
('/** 🧭️ Derives hash-bound producer paths as DATA',
 '/** 🚮️ Removes exactly one empty hash-bound producer directory and refuses any content. */\nexport function removeEmptyNativeArtifactCacheProducerDirectoryV1(directory:string):void{if(readdirSync(directory).length)throw Error("Orphaned producer directory is not empty: "+directory);rmdirSync(directory);}\n\n/** 🧭️ Derives hash-bound producer paths as DATA'),
])
edit(P+"💾️cache/💾️volume/🟦️.ts",[
('{nativeArtifactCacheLayoutV1,nativeArtifactCacheProducerLayoutV1}from"../📍️layout/🟦️.ts"','{nativeArtifactCacheLayoutV1,nativeArtifactCacheProducerLayoutV1,removeEmptyNativeArtifactCacheProducerDirectoryV1}from"../📍️layout/🟦️.ts"'),
('if(existsSync(paths.directory)){if(readdirSync(paths.directory).length)throw new Error("Orphaned producer directory is not empty: "+paths.directory);rmdirSync(paths.directory);}check();',
 'if(existsSync(paths.directory))removeEmptyNativeArtifactCacheProducerDirectoryV1(paths.directory);check();'),
(',rmSync,readdirSync,rmdirSync}from"node:fs"',',rmSync}from"node:fs"'),
])
