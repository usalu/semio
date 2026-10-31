from pathlib import Path
import os
p=Path("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🎛️owner/🌅️initial")
root=Path("🧰️framework/🔨️modules")
imports={
 "@descriptor":root/"📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts",
 "@validator":root/"🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts",
 "@leases":root/"🏃️process/🔒️leases/🟦️.ts",
 "@routing":root/"🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts",
 "@cleanup":root/"🏃️process/🧭️routing/📥️invocation/🏃️process/🧹️cleanup/🟦️.ts",
 "@execution":root/"🏃️process/🎛️owned-execution/🟦️.ts",
 "@hdiutil":root/"🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🛠️hdiutil/🟦️.ts",
 "@information":root/"🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📥️information/🟦️.ts",
 "@boot":root/"🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🟦️.ts",
 "@extent":root/"🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪶️extent/🟦️.ts",
 "@durability":root/"🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧾️metadata/🟦️.ts",
 "@publication-schema":p.parent.parent/"🧬️schema/🔣️.json",
 "@receipt-schema":root/"🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json",
 "@mount-schema":root/"🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json"
}
code=r'''import{createHash}from"node:crypto";
import{lstatSync,statfsSync,type BigIntStats}from"node:fs";
import{dirname,join,posix,resolve}from"node:path";
import{types}from"node:util";
import{validateJsonSchemaSubset}from"@validator";
import{withArtifactRelativeDirectoryV1,requireArtifactDescriptorClosureV1,type ArtifactReadFileIdentityV1}from"@descriptor";
import{checkResourceLeaseControl,requireMaterializedPathV1,requireResourceLeaseWriterProofV1,resourceLeasePathV1,type ResourceLeaseControl,type ResourceLeaseWriterProofV1}from"@leases";
import{requireIssuedResourceControlRefinementV1,requireAcceptedPhysicalCleanupControlV1,readAcceptedPhysicalCleanupV1,readIssuedResourceControlCapabilitiesV1,withRequestedIssuedResourceControlV1}from"@routing";
import type{AcceptedPhysicalCleanupV1}from"@cleanup";
import{captureOwnedCommandResultV1}from"@execution";
import{nativeImageHdiutilCommandV1}from"@hdiutil";
import{nativeImageInformationV1}from"@information";
import{readNativeImageAncestryV1}from"@boot";
import{nativeImageBackingAllocatedBytesV1}from"@extent";
import{syncNativeDirectoryMetadataV1}from"@durability";
import type{NativeImagePublicationOptionsV1}from"../🟦️.ts";
import publicationSchema from"@publication-schema"with{type:"json"};
import receiptSchema from"@receipt-schema"with{type:"json"};
import mountSchema from"@mount-schema"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

type Identity=Readonly<{dev:string;ino:string;mode:number}>;
type Metadata=Identity&Readonly<{name:string;kind:string;nlink:number;size:number}>;
type Facts=Readonly<{platform:string;ancestry:readonly string[];directory:string;declaration:Readonly<{filesystemBytes:number;maximumPhysicalBytes:number}>;baseNames:readonly string[];parent:Identity;backing:Identity&Readonly<{kind:string;nlink:number;size:number;allocatedBytes:number}>;receipt:Readonly<{version:number;bootSession:string;ancestry:readonly string[];dev:number;ino:number;size:number;filesystemBytes:number;maximumPhysicalBytes:number;mountDirectory:Identity}>;namespace:Identity&Readonly<{entries:readonly Metadata[]}>;root:Identity&Readonly<{filesystemBytes:number}>;imageMatches:number;associations:readonly Readonly<{imagePath:string|null;mountPoint:string;device:string}>[]}>;
/** 🌄️ Describes a fresh metadata-only decision without ordinary readiness or retained Native authority. */
export interface InitialNativeMountedPublicationMetadataDecisionV1{readonly authority:false;readonly purpose:"current-predecessor-mounted-publication-metadata";readonly entries:number;readonly ordinaryCurrentAdmitted:false}
const policy=Object.freeze({...schema.$defs.Protocol.const,baseRoles:Object.freeze([...schema.$defs.Protocol.const.baseRoles]),metadataRoles:Object.freeze([...schema.$defs.Protocol.const.metadataRoles])}),hash=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex"),stamp=(row:ArtifactReadFileIdentityV1|BigIntStats)=>JSON.stringify([row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].map(String)),identity=(row:ArtifactReadFileIdentityV1|BigIntStats):Identity=>Object.freeze({dev:String(row.dev),ino:String(row.ino),mode:Number(row.mode)});
/** 🧊️ Captures bounded plain facts without invoking accessors, proxies or custom serialization. */
function capture(value:unknown):unknown{
 let nodes=0,bytes=0;const active=new Set<object>();
 const visit=(value:unknown,depth:number):unknown=>{
  if(++nodes>131072||depth>12)throw Error("Bounded current metadata DATA required");
  if(value===null||typeof value==="boolean"||typeof value==="number"&&Number.isFinite(value))return value;
  if(typeof value==="string"){bytes+=Buffer.byteLength(value);if(bytes>2097152||Buffer.from(value).toString("utf8")!==value)throw Error("Exact bounded current metadata text required");return value;}
  if(typeof value!=="object"||types.isProxy(value)||active.has(value))throw Error("Plain acyclic current metadata DATA required");
  const array=Array.isArray(value),prototype=Object.getPrototypeOf(value);if(array?prototype!==Array.prototype:prototype!==Object.prototype&&prototype!==null)throw Error("Own current metadata values required");
  const fields=Object.getOwnPropertyDescriptors(value),names=Reflect.ownKeys(fields);if(names.some(name=>typeof name!=="string"))throw Error("Named current metadata DATA required");active.add(value);
  try{if(array){const length=fields.length?.value;if(!Number.isSafeInteger(length)||length<0||length>16384||names.length!==length+1)throw Error("Dense bounded current metadata array required");return Object.freeze(Array.from({length},(_,index)=>{const field=fields[String(index)];if(!field||!("value"in field)||!field.enumerable)throw Error("Own current metadata array fields required");return visit(field.value,depth+1);}));}
   if(names.length>32)throw Error("Closed current metadata object required");const output:Record<string,unknown>=Object.create(null);for(const name of names as string[]){const field=fields[name]!;if(!("value"in field)||!field.enumerable)throw Error("Own current metadata fields required");output[name]=visit(field.value,depth+1);}return Object.freeze(output);
  }finally{active.delete(value);}
 };return visit(value,0);
}
/** 🔎️ Requires a strict physical predecessor so discarding metadata cannot admit a current ordinary receipt. */
export function inspectInitialNativeMountedPublicationMetadataV1(input:unknown):InitialNativeMountedPublicationMetadataDecisionV1{
 const value=capture(input);if(validateJsonSchemaSubset(schema.$defs.Facts,value,schema).length)throw Error("Closed current mounted metadata facts required");const x=value as Facts,r=x.receipt,d=x.declaration,b=x.backing,p=x.parent,n=x.namespace,root=x.root,mode=(value:number,kind:number)=>(value&0o170000)===kind;
 if(x.directory.includes("\0")||!posix.isAbsolute(x.directory)||posix.resolve(x.directory)!==x.directory||[posix.join(x.directory,"fixed.dmg"),posix.join(x.directory,"m")].some(path=>path.length>256)||d.filesystemBytes>d.maximumPhysicalBytes||r.filesystemBytes>r.maximumPhysicalBytes||r.filesystemBytes>d.filesystemBytes||r.maximumPhysicalBytes>d.maximumPhysicalBytes||r.filesystemBytes===d.filesystemBytes&&r.maximumPhysicalBytes===d.maximumPhysicalBytes)throw Error("Strict current Native predecessor envelope required");
 if(r.ancestry.length!==x.ancestry.length||r.ancestry.some((ino,index)=>ino!==x.ancestry[index]))throw Error("Exact current Native image ancestry required");
 if(!mode(p.mode,0o040000)||!mode(n.mode,0o040000)||!mode(root.mode,0o040000)||!mode(r.mountDirectory.mode,0o040000)||!mode(b.mode,0o100000)||b.kind!=="file"||b.nlink!==1||b.dev!==p.dev||n.dev!==p.dev||root.dev===p.dev||r.mountDirectory.dev!==p.dev||String(r.dev)!==b.dev||String(r.ino)!==b.ino||r.size!==b.size||b.size>r.maximumPhysicalBytes||b.allocatedBytes<1||b.allocatedBytes>r.maximumPhysicalBytes||root.filesystemBytes>r.filesystemBytes||(n.mode&511)!==policy.namespaceMode)throw Error("Coherent actual predecessor backing and mounted root required");
 if(x.baseNames.length!==policy.baseRoles.length||new Set(x.baseNames).size!==x.baseNames.length||x.baseNames.some(name=>!policy.baseRoles.includes(name))||new Set(n.entries.map(row=>row.name)).size!==n.entries.length||n.entries.some(row=>!policy.metadataRoles.includes(row.name)||row.kind!=="file"||!mode(row.mode,0o100000)||(row.mode&511)!==policy.metadataMode||row.nlink!==1||row.dev!==p.dev))throw Error("Closed current Native system metadata namespace required");
 const image=posix.join(x.directory,"fixed.dmg"),mount=posix.join(x.directory,"m"),matches=x.associations.filter(row=>row.imagePath===image),atRoot=x.associations.filter(row=>row.mountPoint===mount);
 if(x.imageMatches!==1||matches.length!==1||atRoot.length!==1||matches[0]!==atRoot[0]||!/^\/dev\/disk[0-9]+(?:s[0-9]+)*$/u.test(matches[0]!.device)||x.associations.some(row=>row.mountPoint.startsWith(mount+"/")))throw Error("Exact complete current Native kernel association required");
 return Object.freeze({authority:false,purpose:"current-predecessor-mounted-publication-metadata",entries:n.entries.length,ordinaryCurrentAdmitted:false});
}
/** 🧷️ Retires only bounded current Mac metadata under the actual retained publisher and one owned kernel observation. */
export async function retireInitialNativeMountedPublicationMetadataV1(input:NativeImagePublicationOptionsV1,body:ResourceLeaseControl,enrollment:AcceptedPhysicalCleanupV1,proof:ResourceLeaseWriterProofV1):Promise<void>{
 if(arguments.length!==4||!input||types.isProxy(input)||Object.getPrototypeOf(input)!==Object.prototype)throw Error("Plain current Native publication options required");
 const fields:Record<string,unknown>={};for(const key of Reflect.ownKeys(input)){if(typeof key!=="string"||!["directory","leaseDirectory","filesystemBytes","maximumPhysicalBytes","maximumLogicalBytes","control","maximumLeaseDatabaseBytes","onProgress"].includes(key))throw Error("Closed current Native publication options required");const field=Object.getOwnPropertyDescriptor(input,key);if(!field||!("value"in field)||!field.enumerable)throw Error("Own current Native publication options required");fields[key]=field.value;}
 const options=Object.freeze(fields)as unknown as NativeImagePublicationOptionsV1;if(typeof options.directory!=="string"||typeof options.leaseDirectory!=="string"||process.platform!==policy.platform||![options.filesystemBytes,options.maximumPhysicalBytes,options.maximumLogicalBytes].every(Number.isSafeInteger)||options.filesystemBytes<33554432||options.filesystemBytes>options.maximumPhysicalBytes||options.maximumLogicalBytes<1)throw Error("Actual declared mounted Mac publisher required");
 requireIssuedResourceControlRefinementV1(options.control,body);requireAcceptedPhysicalCleanupControlV1(enrollment,body);const resource="native-fixed-image:"+options.directory,binding={directory:options.leaseDirectory,resource,path:resourceLeasePathV1(options.leaseDirectory,resource)},accepted=readAcceptedPhysicalCleanupV1(enrollment);
 if(accepted.kind!=="native-image"||accepted.resource!==resource||accepted.reserveMilliseconds!==10000)throw Error("Actual defining Native metadata cleanup required");
 const capabilities=readIssuedResourceControlCapabilitiesV1(body),native="native"in capabilities?capabilities.native:null,child=native&&typeof native==="object"&&!Array.isArray(native)?(native as Record<string,unknown>).child:null,transport=native&&typeof native==="object"&&!Array.isArray(native)?(native as Record<string,unknown>).transport:null,limits={maximumElapsedMilliseconds:child&&typeof child==="object"?(child as Record<string,unknown>).maximumElapsedMilliseconds:undefined,maximumBytes:transport&&typeof transport==="object"?(transport as Record<string,unknown>).maximumBytes:undefined,maximumLines:transport&&typeof transport==="object"?(transport as Record<string,unknown>).maximumLines:undefined};
 if(validateJsonSchemaSubset(publicationSchema.definitions.NativeGrant,limits,publicationSchema).length)throw Error("Actual received Native child and transport grant required");
 for(const path of[options.directory,options.leaseDirectory,...["fixed.dmg","image.json","m",".publication",".publication/intent.json",".publication/receipt.json"].map(name=>join(options.directory,name))])if(typeof path!=="string"||resolve(path)!==path||requireMaterializedPathV1(path)!==path)throw Error("Canonical bounded Native metadata paths required");
 const ancestors:Array<{path:string;dev:bigint;ino:bigint;mode:bigint}>=[];for(let path=options.directory;;path=dirname(path)){checkResourceLeaseControl(body);requireResourceLeaseWriterProofV1(proof,binding,body);const row=lstatSync(path,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||ancestors.length>=64)throw Error("Exact bounded Native metadata ancestry required");ancestors.push({path,dev:row.dev,ino:row.ino,mode:row.mode});if(dirname(path)===path)break;}
 const check=()=>{checkResourceLeaseControl(body);requireAcceptedPhysicalCleanupControlV1(enrollment,body);requireResourceLeaseWriterProofV1(proof,binding,body);for(const before of ancestors){const row=lstatSync(before.path,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||row.dev!==before.dev||row.ino!==before.ino||row.mode!==before.mode)throw Error("Current Native metadata ancestry changed");}};
 check();const directory=options.directory,namespacePath=join(directory,".publication"),namespace=lstatSync(namespacePath,{bigint:true,throwIfNoEntry:false});check();if(!namespace)return;
 if(!namespace.isDirectory()||namespace.isSymbolicLink()||namespace.dev!==ancestors[0]!.dev||(namespace.mode&511n)!==BigInt(policy.namespaceMode))throw Error("Actual current Native metadata namespace required");
 const backingPath=join(directory,"fixed.dmg"),receiptPath=join(directory,"image.json"),rootPath=join(directory,"m"),backing=lstatSync(backingPath,{bigint:true}),receipt=lstatSync(receiptPath,{bigint:true}),mounted=lstatSync(rootPath,{bigint:true});
 if(!backing.isFile()||backing.isSymbolicLink()||backing.nlink!==1n||backing.dev!==ancestors[0]!.dev||!receipt.isFile()||receipt.isSymbolicLink()||receipt.nlink!==1n||receipt.dev!==ancestors[0]!.dev||!mounted.isDirectory()||mounted.isSymbolicLink()||mounted.dev===ancestors[0]!.dev)throw Error("Actual singleton mounted image and predecessor metadata required");
 const currentAncestry=readNativeImageAncestryV1(directory),immutable=()=>{check();if(JSON.stringify(readNativeImageAncestryV1(directory))!==JSON.stringify(currentAncestry))throw Error("Current Native image ancestry changed");for(const[path,before]of[[backingPath,backing],[receiptPath,receipt],[rootPath,mounted]]as const)if(stamp(lstatSync(path,{bigint:true}))!==stamp(before))throw Error("Current Native predecessor physical context changed");};
 immutable();requireArtifactDescriptorClosureV1();await body.publish({resource:"native-initial-metadata:information",mode:"exclusive",elapsedMs:0});immutable();
 const text=await withRequestedIssuedResourceControlV1(body,limits.maximumElapsedMilliseconds as number,async actual=>{immutable();const spec=nativeImageHdiutilCommandV1(["info","-plist"],directory),result=await captureOwnedCommandResultV1(spec.program,spec.args,spec.cwd,{control:actual,env:spec.env,output:{maximumBytes:Math.min(policy.maximumInformationBytes,limits.maximumBytes as number),maximumLines:Math.min(policy.maximumInformationLines,limits.maximumLines as number)}});immutable();if(result.status!==0||result.signal!==null||result.stderr!=="")throw Error("Closed successful Native kernel information child required");return result.stdout;});
 immutable();const information=nativeImageInformationV1(text),images=information.images;if(!Array.isArray(images)||images.length>policy.maximumImages)throw Error("Complete bounded Native image inventory required");const associations:Array<{imagePath:string|null;mountPoint:string;device:string}>=[];let count=0,imageMatches=0;
 for(const image of images){if(!image||typeof image!=="object"||Array.isArray(image))throw Error("Exact Native image inventory entry required");const row=image as Record<string,unknown>,entities=row["system-entities"];if(!Array.isArray(entities)||count+entities.length>policy.maximumEntities)throw Error("Complete bounded Native kernel entities required");count+=entities.length;if(row["image-path"]===backingPath)imageMatches++;for(const entity of entities){if(!entity||typeof entity!=="object"||Array.isArray(entity))throw Error("Exact Native kernel entity required");const item=entity as Record<string,unknown>;if(typeof item["mount-point"]==="string"){if(typeof item["dev-entry"]!=="string")throw Error("Actual mounted Native kernel device required");associations.push({imagePath:typeof row["image-path"]==="string"?row["image-path"]:null,mountPoint:item["mount-point"],device:item["dev-entry"]});}}}
 await withArtifactRelativeDirectoryV1(directory,immutable,async root=>{
  const names=await root.names();immutable();const parent=root.stat(),receiptEntry=await root.entry("image.json");if(!receiptEntry||receiptEntry.kind!=="file"||receiptEntry.nlink!==1n||receiptEntry.size<0n||receiptEntry.size>BigInt(policy.maximumMetadataBytes))throw Error("Actual bounded current image receipt required");
  const bytes=await root.withFile("image.json",async file=>{immutable();if(stamp(file.stat())!==stamp(receiptEntry))throw Error("Actual current image receipt descriptor required");const bytes=Buffer.alloc(Number(receiptEntry.size)+1);let offset=0;while(offset<Number(receiptEntry.size)){immutable();const read=file.read(bytes,offset,Number(receiptEntry.size)-offset,offset);if(read<1)throw Error("Complete current image receipt bytes required");offset+=read;}if(file.read(bytes,offset,1,offset)!==0||stamp(file.stat())!==stamp(receiptEntry))throw Error("Unchanged current image receipt required");return bytes.subarray(0,offset);});
  const attested=await root.attestFilePublication("image.json",receiptEntry,policy.maximumMetadataBytes);immutable();if(attested.sha256!==hash(bytes)||stamp(attested.identity)!==stamp(receiptEntry)||stamp(receiptEntry)!==stamp(receipt))throw Error("Same current image receipt entry required");const published:unknown=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes));if(validateJsonSchemaSubset(receiptSchema.definitions.ImageReceipt,published,receiptSchema,[mountSchema]).length)throw Error("Closed current image receipt required");
  const filesystem=statfsSync(rootPath,{bigint:true}),filesystemBytes=filesystem.blocks*filesystem.bsize;if(filesystemBytes<1n||filesystemBytes>BigInt(Number.MAX_SAFE_INTEGER))throw Error("Bounded actual mounted filesystem required");
  let finalNamespace:string|undefined;
  await withArtifactRelativeDirectoryV1(namespacePath,immutable,async actual=>{
   immutable();if(stamp(actual.stat())!==stamp(namespace))throw Error("Exact current Native metadata directory required");const original=await actual.names(),records=new Map<string,{stamp:string;sha256:string;facts:Metadata}>();
   if(original.length>policy.maximumMetadataEntries||original.some(name=>!policy.metadataRoles.includes(name)))throw Error("Closed Native system metadata names required");
   for(const name of original){const entry=await actual.entry(name);if(!entry||entry.kind!=="file"||entry.nlink!==1n||entry.size<0n||entry.size>BigInt(policy.maximumMetadataBytes)||(entry.mode&511n)!==BigInt(policy.metadataMode))throw Error("Bounded single-link Native system metadata required");const observed=await actual.attestFilePublication(name,entry,policy.maximumMetadataBytes);immutable();if(stamp(observed.identity)!==stamp(entry))throw Error("Same current Native metadata attestation required");records.set(name,{stamp:stamp(entry),sha256:observed.sha256,facts:{name,kind:entry.kind,...identity(entry),nlink:Number(entry.nlink),size:Number(entry.size)}});}
   const allocatedBytes=Number(nativeImageBackingAllocatedBytesV1(backing));if(!Number.isSafeInteger(allocatedBytes)||allocatedBytes<1)throw Error("Representable actual backing extent required");immutable();inspectInitialNativeMountedPublicationMetadataV1({platform:process.platform,ancestry:currentAncestry,directory,declaration:{filesystemBytes:options.filesystemBytes,maximumPhysicalBytes:options.maximumPhysicalBytes},baseNames:names,parent:identity(parent),backing:{...identity(backing),kind:"file",nlink:Number(backing.nlink),size:Number(backing.size),allocatedBytes},receipt:published,namespace:{...identity(actual.stat()),entries:[...records.values()].map(row=>row.facts)},root:{...identity(mounted),filesystemBytes:Number(filesystemBytes)},imageMatches,associations});
   const remaining=new Set(original),verify=async()=>{immutable();if(JSON.stringify(await root.names())!==JSON.stringify(names)||JSON.stringify(await actual.names())!==JSON.stringify([...remaining].sort()))throw Error("Exact remaining current Native metadata inventory required");};
   await verify();for(const name of [...original].sort((a,b)=>Number(a==="intent.json")-Number(b==="intent.json")||(a<b?-1:a>b?1:0))){await verify();const prior=records.get(name)!,entry=await actual.entry(name);if(!entry||stamp(entry)!==prior.stamp)throw Error("Exact current Native metadata removal entry required");const current=await actual.attestFilePublication(name,entry,policy.maximumMetadataBytes);immutable();if(current.sha256!==prior.sha256||stamp(current.identity)!==prior.stamp)throw Error("Same-entry Native metadata removal attestation required");await actual.remove(name,entry);remaining.delete(name);await actual.sync();await verify();}
   if(remaining.size)throw Error("Empty current Native publication metadata required");finalNamespace=stamp(actual.stat());
  });
  immutable();const empty=await root.entry(".publication");if(!empty||empty.kind!=="directory"||stamp(empty)!==finalNamespace)throw Error("Same durably empty Native metadata directory required");await root.remove(".publication",empty);await root.sync();immutable();if(await root.entry(".publication")!==null||JSON.stringify(await root.names())!==JSON.stringify(names.filter(name=>name!==".publication")))throw Error("Exact Native metadata-only namespace transition required");
 });
 immutable();await syncNativeDirectoryMetadataV1(directory,directory,body);immutable();requireArtifactDescriptorClosureV1();
}
'''
for marker,path in imports.items():code=code.replace(marker,"./"+os.path.relpath(path,p).replace(os.sep,"/"))
f=p/"🟦️.ts"
assert f.read_text()=="export {};\n", "Only replace the intended empty implementation after actual RED"
f.write_text(code)
print("Mounted initial metadata helper implementation authored; actual runtime effects not executed.")
