import os,shutil,sys
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
BK=C+"/🗑️generated/boot-rebind"
S=ROOT+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage"
CMD=S+"/🧹️cleanup/🎛️command/🟦️.ts"
PUB=S+"/📤️publication/🎛️owner/🟦️.ts"
IMPORT='import{readNativeBootSessionV1,nativeBootSessionV1,readNativeImageAncestryV1,decideNativeReceiptBootV1}from"../../🌐️platform/🥾️boot/🟦️.ts";\n'
REBIND='''/** 🔁️ Rebinds a receipt of an earlier boot to the current device numbers once the image is proven unattached and identical in every boot-stable fact; only the exclusive writer rewrites. */
async function rebindPersistentReceiptV1(receiver:PersistentReceiver,control:ResourceLeaseControl,f:Readonly<{record:{bytes:Buffer;stamp:string};authored:Record<string,unknown>;folder:Stats;backing:Stats;mount:Stats;image:string;root:string;receipt:string;boot:string}>):Promise<"rebound"|"deferred">{
 const x=receiver.options,authored=f.authored;
 if(validateJsonSchemaSubset(imageSchema.definitions.PriorImageReceipt,authored,imageSchema,[mountDirectorySchema]).length||authored.filesystemBytes!==x.filesystemBytes||(authored.maximumPhysicalBytes as number)>x.maximumPhysicalBytes||!f.folder.isDirectory()||f.folder.isSymbolicLink()||!f.backing.isFile()||f.backing.isSymbolicLink()||f.backing.nlink!==1||!f.mount.isDirectory()||f.mount.isSymbolicLink()||f.mount.dev!==f.backing.dev||names(f.root,0).length)throw Error("Exact boot-stable inactive persistent Native receipt required: "+x.directory);
 const attached=await observeSharedNativeImageInventoryV1(()=>captureOwnedCommandV1("/usr/bin/hdiutil",["info","-plist"],x.directory,{control,output:{maximumBytes:limits.maximumOutputBytes,maximumLines:limits.maximumOutputLines}}),information=>{const images=information.images;if(!Array.isArray(images)||images.length>limits.maximumObservedImages)throw Error("Complete bounded Native receiver kernel inventory required");return images.some(row=>{if(!row||typeof row!=="object"||Array.isArray(row))throw Error("Complete Native kernel entry required");const entry=row as Record<string,unknown>,entities=entry["system-entities"];if(!Array.isArray(entities)||entities.length>limits.maximumObservedEntities)throw Error("Complete Native kernel entities required");return entry["image-path"]===f.image||entities.some(entity=>{const point=entity&&typeof entity==="object"&&!Array.isArray(entity)?(entity as Record<string,unknown>)["mount-point"]:undefined;return typeof point==="string"&&(point===f.root||point.startsWith(f.root+sep));});});},inventoryPolicy(control));
 checkResourceLeaseControl(control);
 const ancestry=readNativeImageAncestryV1(x.directory),prior=authored as {bootSession?:string;dev?:number;ino:number;size:number;filesystemBytes:number;maximumPhysicalBytes:number;mountDirectory:{ino:string;mode:number};ancestry?:readonly string[]};
 const verdict=decideNativeReceiptBootV1({version:1,receipt:{bootSession:prior.bootSession??null,dev:prior.dev??1,ino:prior.ino,size:prior.size,maximumPhysicalBytes:prior.maximumPhysicalBytes,mountDirectory:{ino:prior.mountDirectory.ino,mode:prior.mountDirectory.mode},ancestry:prior.ancestry??null},current:{bootSession:f.boot,dev:f.backing.dev,ino:f.backing.ino,size:f.backing.size,allocatedBytes:f.backing.blocks*512,mountIno:String(f.mount.ino),mountMode:f.mount.mode,ancestry},attached});
 if(verdict!=="rebind")throw Error("Exact boot-stable persistent Native identity required: "+x.directory+" "+JSON.stringify({verdict,attached,backing:[prior.ino,f.backing.ino,prior.size,f.backing.size],mount:[prior.mountDirectory.ino,f.mount.ino]}));
 if(receiver.stage!=="writer")return"deferred";
 if(readNativeImageRetirementFileV1(f.receipt,receiverSchema.definitions.Protocol.const.maximumReceiptBytes).stamp!==f.record.stamp)throw Error("Native receiver receipt changed before boot rebind");
 const next={version:1,mountDirectory:observeNativeImageMountDirectoryV1(f.root),bootSession:f.boot,dev:f.backing.dev,ino:f.backing.ino,size:f.backing.size,ancestry,filesystemBytes:prior.filesystemBytes,maximumPhysicalBytes:prior.maximumPhysicalBytes};
 if(validateJsonSchemaSubset(imageSchema.definitions.ImageReceipt,next,imageSchema,[mountDirectorySchema]).length)throw Error("Closed current boot rebind receipt required");
 const bytes=Buffer.from(JSON.stringify(next)+"\\n"),temporary=f.receipt+".rebind";
 if(bytes.length>receiverSchema.definitions.Protocol.const.maximumReceiptBytes)throw Error("Finite Native receiver receipt required");
 if(existsSync(temporary)){const old=lstatSync(temporary);if(!old.isFile()||old.isSymbolicLink()||old.nlink!==1)throw Error("Unknown persistent Native rebind residue retained");unlinkSync(temporary);}
 const fd=openSync(temporary,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|(constants.O_NOFOLLOW??0),0o600);let failed=false,failure:unknown;
 try{writeFileSync(fd,bytes);fsyncSync(fd);}catch(error){failed=true;failure=error;}
 try{closeSync(fd);}catch(error){if(!failed){failed=true;failure=error;}}
 if(failed){unlinkSync(temporary);throw failure;}
 checkResourceLeaseControl(control);renameSync(temporary,f.receipt);
 const directory=openSync(x.directory,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_DIRECTORY??0));try{fsyncSync(directory);}finally{closeSync(directory);}
 return"rebound";
}
'''
def edit(path,pairs):
    text=open(path).read()
    done=all(new in text for _,new in pairs)
    if done: print("already applied",os.path.basename(os.path.dirname(path))); return
    for old,new in pairs:
        if text.count(old)!=1: sys.exit(f"anchor count {text.count(old)} for {old[:80]!r}")
    shutil.copyfile(path,BK+"/"+os.path.basename(os.path.dirname(path))+".orig")
    for old,new in pairs: text=text.replace(old,new)
    open(path,"w").write(text)
    print("patched",path[-60:])
imageImport='import imageSchema from"../../🧬️schema/🔣️.json" with{type:"json"};'
fsOld='import{mkdirSync,lstatSync,statSync,opendirSync,rmdirSync,unlinkSync,existsSync,statfsSync,writeFileSync,openSync,fstatSync,fsyncSync,closeSync,constants}from"node:fs";'
fsNew=fsOld.replace("closeSync,constants","closeSync,renameSync,constants")
inspHead='async function inspectPersistentReceiver(receiver:PersistentReceiver,control:ResourceLeaseControl,retirement=false):Promise<"absent"|"inactive"|"mounted">{checkResourceLeaseControl(control);const x=receiver.options,directory=x.directory;'
inspMid='mount=lstatSync(root);if(!folder.isDirectory()||folder.isSymbolicLink()||validateJsonSchemaSubset(imageSchema.definitions.ImageReceipt,authored,imageSchema,[mountDirectorySchema]).length||authored.filesystemBytes!==x.filesystemBytes'
rcpOld='JSON.stringify({version:1,mountDirectory:scope.mountDirectory,dev:backing.dev,ino:backing.ino,size:backing.size,filesystemBytes:x.filesystemBytes,maximumPhysicalBytes:x.maximumPhysicalBytes})'
rcpNew='JSON.stringify({version:1,mountDirectory:scope.mountDirectory,bootSession:nativeBootSessionV1(),dev:backing.dev,ino:backing.ino,size:backing.size,ancestry:readNativeImageAncestryV1(scope.directory),filesystemBytes:x.filesystemBytes,maximumPhysicalBytes:x.maximumPhysicalBytes})'
stamp='function stampFrom(row:Stats):Identity{'
edit(CMD,[
 (imageImport,imageImport+"\n"+IMPORT.rstrip("\n")),
 (fsOld,fsNew),
 (inspHead,inspHead+'const boot=await readNativeBootSessionV1(control,x.leaseDirectory);'),
 (inspMid,'mount=lstatSync(root);if(authored.bootSession!==boot&&!retirement){if(await rebindPersistentReceiptV1(receiver,control,{record,authored,folder,backing,mount,image,root,receipt,boot})==="deferred")return"inactive";return inspectPersistentReceiver(receiver,control,retirement);}if(!folder.isDirectory()||folder.isSymbolicLink()||validateJsonSchemaSubset(imageSchema.definitions.ImageReceipt,authored,imageSchema,[mountDirectorySchema]).length||authored.filesystemBytes!==x.filesystemBytes'),
 (rcpOld,rcpNew),
 (stamp,REBIND+stamp),
])
pubImp=open(PUB).read()
PUBIMP='import mountSchema from"../../🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json" with { type: "json" };'
edit(PUB,[
 (PUBIMP,PUBIMP+'\nimport{readNativeBootSessionV1,readNativeImageAncestryV1}from"../../🌐️platform/🥾️boot/🟦️.ts";'),
 ('interface NativeMacImagePublicationReceiptV1{readonly version:1;readonly filesystemBytes:number;readonly maximumPhysicalBytes:number;readonly dev:number;readonly ino:number;readonly size:number;readonly mountDirectory:NativeImageMountDirectoryV1}','interface NativeMacImagePublicationReceiptV1{readonly version:1;readonly filesystemBytes:number;readonly maximumPhysicalBytes:number;readonly bootSession:string;readonly dev:number;readonly ino:number;readonly size:number;readonly ancestry:readonly string[];readonly mountDirectory:NativeImageMountDirectoryV1}'),
 ('Object.freeze({version:1 as const,filesystemBytes:x.filesystemBytes,maximumPhysicalBytes:x.maximumPhysicalBytes,dev:Number(scope.request.backing.dev),ino:Number(scope.request.backing.ino),size:scope.request.backing.size,mountDirectory})','Object.freeze({version:1 as const,filesystemBytes:x.filesystemBytes,maximumPhysicalBytes:x.maximumPhysicalBytes,bootSession:await readNativeBootSessionV1(scope.body,x.leaseDirectory),dev:Number(scope.request.backing.dev),ino:Number(scope.request.backing.ino),size:scope.request.backing.size,ancestry:readNativeImageAncestryV1(x.directory),mountDirectory})'),
])
