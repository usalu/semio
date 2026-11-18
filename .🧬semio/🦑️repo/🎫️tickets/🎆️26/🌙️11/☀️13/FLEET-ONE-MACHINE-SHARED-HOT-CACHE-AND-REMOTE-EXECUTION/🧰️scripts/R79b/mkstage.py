#!/usr/bin/env python3
"""R79b stage builder (landing 1): a physical identity (dev, ino) of the protected maintenance request is an exact token end to end: the JSON integer while it is a safe number (bytes unchanged for every value that worked before), else its decimal digits, so a 64-bit NTFS file id or a 128-bit ReFS id is never rounded and never refused. Producers read bigint stats, comparators compare tokens."""
import os,shutil,sys,json
R="/Users/ueli/Documents/semio/"
M="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
OBS=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/📸️observation/"
H=os.path.dirname(os.path.abspath(__file__))+"/stage/"
shutil.rmtree(H,ignore_errors=True);os.makedirs(H)
rows=[];n=[0]
def put(rel,base,text,kind="mod"):
    i="f%d"%n[0];n[0]+=1
    if base is not None:open(H+i+".base","w",encoding="utf8").write(base)
    open(H+i+".new","w",encoding="utf8").write(text);rows.append("%s %s %s"%(i,rel,kind))
def stage(rel,edits):
    text=open(R+rel,encoding="utf8").read();base=text
    for old,rep,count in edits:
        got=text.count(old)
        if got!=count:sys.exit("anchor count %d != %d in %s: %r"%(got,count,rel,old[:100]))
        text=text.replace(old,rep)
    put(rel,base,text)
def stagejson(rel,fn):
    base=open(R+rel,encoding="utf8").read();d=json.loads(base);fn(d)
    put(rel,base,json.dumps(d,indent=2,ensure_ascii=False)+"\n")
ID={"anyOf":[{"type":"integer","minimum":0,"maximum":9007199254740991},{"type":"string","pattern":"^[1-9][0-9]{15,38}$"}]}
def schema(d):
    d["definitions"]["PhysicalIdentity"]={"description":"One physical identity (device, volume serial, inode, file id) as an exact token: the JSON integer while it is a safe number, else its decimal digits above 2^53 (a 64-bit NTFS file id, a 128-bit ReFS id). Never a rounded number.","anyOf":ID["anyOf"]}
    r=d["definitions"]["ProtectedMaintenanceRequest"]["properties"]
    for k in("directory","backing","root"):
        for f in("dev","ino"):r[k]["properties"][f]={"$ref":"#/definitions/PhysicalIdentity"}
stagejson(M+"🧬️schema/🔣️.json",schema)
def obs(d):
    for k in("Directory","Backing"):
        for f in("dev","ino"):d["$defs"][k]["properties"][f]={"anyOf":ID["anyOf"]}
stagejson(OBS+"🧬️schema/🔣️.json",obs)
stage(M+"📋️contract/🟦️.ts",[('export type NativeProtectedMaintenanceRequestV1=Omit<NativeProtectedParentRequestV1,"purpose">&Readonly<{purpose:"mounted-parent-maintenance"}>;',
 '/** 🪪️ One physical identity as an exact token: a JSON integer while it is a safe number, else its decimal digits. */\nexport type NativePhysicalIdentityV1=number|string;\nexport type NativeProtectedMaintenanceRequestV1=Readonly<{version:1;purpose:"mounted-parent-maintenance";wait:false;receiptSha256:string;directory:Readonly<{dev:NativePhysicalIdentityV1;ino:NativePhysicalIdentityV1;mode:number}>;backing:Readonly<{dev:NativePhysicalIdentityV1;ino:NativePhysicalIdentityV1;size:number;allocatedBytes:number}>;root:Readonly<{dev:NativePhysicalIdentityV1;ino:NativePhysicalIdentityV1;mode:number}>}>;',1)])
CT=M+"🌐️platform/📋️contract/🟦️.ts"
stage(CT,[
 ('NativeProtectedMaintenanceProviderV1} from "../../📋️contract/🟦️.ts";','NativeProtectedMaintenanceProviderV1,NativePhysicalIdentityV1} from "../../📋️contract/🟦️.ts";',2),
 ('export function readNativeProtectedMaintenanceRequestV1(value:unknown):NativeProtectedMaintenanceRequestV1{',
  '''/** 🪪️ Encodes one physical identity exactly: the JSON integer while it is a safe number, else its decimal digits, so a 64-bit NTFS file id or a 128-bit ReFS id is never rounded. */
export function nativePhysicalIdentityTokenV1(value:bigint):NativePhysicalIdentityV1{if(typeof value!=="bigint"||value<0n||value>=(1n<<128n))throw Error("Exact physical identity required");return value<=9007199254740991n?Number(value):value.toString();}
/** 🔢️ Narrows an identity token to the number that the persisted image receipt holds; an identity above 2^53 has no receipt form. */
export function nativePhysicalIdentityNumberV1(value:NativePhysicalIdentityV1):number{if(typeof value!=="number")throw Error("Image receipt identity above 2^53 unsupported");return value;}
function requireCanonicalNativePhysicalIdentitiesV1(request:NativeProtectedMaintenanceRequestV1):void{for(const row of[request.directory,request.backing,request.root])for(const value of[row.dev,row.ino])if(typeof value==="string"&&(BigInt(value)<=9007199254740991n||BigInt(value)>=(1n<<128n)||BigInt(value).toString()!==value))throw Error("Canonical physical identity token required");}
export function readNativeProtectedMaintenanceRequestV1(value:unknown):NativeProtectedMaintenanceRequestV1{''',1),
 ('throw Error("Closed distinct protected maintenance request required");return captured as NativeProtectedMaintenanceRequestV1;}','throw Error("Closed distinct protected maintenance request required");requireCanonicalNativePhysicalIdentitiesV1(captured as NativeProtectedMaintenanceRequestV1);return captured as NativeProtectedMaintenanceRequestV1;}',1),
])
stage(OBS+"🟦️.ts",[
 ('import {nativeImageRootV1,nativeImageBackingFilenameV1,readNativeProtectedMaintenanceRequestV1,','import {nativeImageRootV1,nativeImageBackingFilenameV1,readNativeProtectedMaintenanceRequestV1,nativePhysicalIdentityTokenV1,',1),
 ('const row=value as{directory:{dev:number;ino:number;mode:number};backing:{dev:number;ino:number;size:number;allocatedBytes:number};root:{dev:number;ino:number;mode:number};','const row=value as{directory:{dev:number|string;ino:number|string;mode:number};backing:{dev:number|string;ino:number|string;size:number;allocatedBytes:number};root:{dev:number|string;ino:number|string;mode:number};',1),
 ('const identity=(facts:{dev:number;ino:number;mode:number})=>','const identity=(facts:{dev:number|string;ino:number|string;mode:number})=>',1),
 ('identity=(row:BigIntStats)=>({dev:number(row.dev),ino:number(row.ino),mode:number(row.mode),','identity=(row:BigIntStats)=>({dev:nativePhysicalIdentityTokenV1(row.dev),ino:nativePhysicalIdentityTokenV1(row.ino),mode:number(row.mode),',1),
])
FX=M+"🧱️fixed/🟦️.ts"
BI='JSON.stringify([file.dev,file.ino,file.size,nativeImageBackingAllocatedBytesV1(file)])'
stage(FX,[
 ('import{readNativeProtectedMaintenanceRequestV1,nativeProtectedMaintenanceProtocolV1,type NativeProtectedMaintenanceRequestV1,','import{readNativeProtectedMaintenanceRequestV1,nativePhysicalIdentityTokenV1,nativeProtectedMaintenanceProtocolV1,type NativeProtectedMaintenanceRequestV1,',1),
 ('/** 📸️ Retains bounded reserved publication bytes as immutable preconditions under actual protected maintenance EX. */','/** 🏷️ Names one backing by its exact identity tokens, size and allocation; the same bytes as before for every safe identity. */\nfunction fixedBackingIdentityV1(file:Readonly<{dev:bigint;ino:bigint;size:bigint}>):string{return JSON.stringify([nativePhysicalIdentityTokenV1(file.dev),nativePhysicalIdentityTokenV1(file.ino),Number(file.size),Number(nativeImageBackingAllocatedBytesV1(file))]);}\n/** 📸️ Retains bounded reserved publication bytes as immutable preconditions under actual protected maintenance EX. */',1),
 ('folder=lstatSync(options.directory),file=lstatSync(image),mounted=lstatSync(root),record=readNativeImageRetirementFileV1(join(options.directory,"image.json")','folder=lstatSync(options.directory,{bigint:true}),file=lstatSync(image,{bigint:true}),mounted=lstatSync(root,{bigint:true}),record=readNativeImageRetirementFileV1(join(options.directory,"image.json")',1),
 ('folder.dev!==request.directory.dev||folder.ino!==request.directory.ino||folder.mode!==request.directory.mode||!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.dev!==request.backing.dev||file.ino!==request.backing.ino||file.size!==request.backing.size||nativeImageBackingAllocatedBytesV1(file)!==request.backing.allocatedBytes||file.size<1||file.size>options.maximumPhysicalBytes||process.platform==="linux"&&nativeImageBackingAllocatedBytesV1(file)<file.size||!mounted.isDirectory()||mounted.isSymbolicLink()||mounted.dev!==request.root.dev||mounted.ino!==request.root.ino||mounted.mode!==request.root.mode||!nativeImageAttachedV1(image,root)',
  'nativePhysicalIdentityTokenV1(folder.dev)!==request.directory.dev||nativePhysicalIdentityTokenV1(folder.ino)!==request.directory.ino||Number(folder.mode)!==request.directory.mode||!file.isFile()||file.isSymbolicLink()||file.nlink!==1n||nativePhysicalIdentityTokenV1(file.dev)!==request.backing.dev||nativePhysicalIdentityTokenV1(file.ino)!==request.backing.ino||Number(file.size)!==request.backing.size||Number(nativeImageBackingAllocatedBytesV1(file))!==request.backing.allocatedBytes||file.size<1n||file.size>BigInt(options.maximumPhysicalBytes)||process.platform==="linux"&&nativeImageBackingAllocatedBytesV1(file)<file.size||!mounted.isDirectory()||mounted.isSymbolicLink()||nativePhysicalIdentityTokenV1(mounted.dev)!==request.root.dev||nativePhysicalIdentityTokenV1(mounted.ino)!==request.root.ino||Number(mounted.mode)!==request.root.mode||!nativeImageAttachedV1(image,root)',1),
 ('return Object.freeze({root,image,backingIdentity:'+BI+',publicationIdentity});','return Object.freeze({root,image,backingIdentity:fixedBackingIdentityV1(file),publicationIdentity});',1),
 ('file=lstatSync(image),mounted=lstatSync(root);if(nativeImageRootV1(directory)!==root||!file.isFile()||file.isSymbolicLink()||file.nlink!==1||!mounted.isDirectory()','file=lstatSync(image,{bigint:true}),mounted=lstatSync(root,{bigint:true});if(nativeImageRootV1(directory)!==root||!file.isFile()||file.isSymbolicLink()||file.nlink!==1n||!mounted.isDirectory()',1),
 ('return Object.freeze({root,image,backingIdentity:'+BI+',kernelIdentity:JSON.stringify([fixedDirectoryMappingV1(image,root),mounted.dev,mounted.ino])});','return Object.freeze({root,image,backingIdentity:fixedBackingIdentityV1(file),kernelIdentity:JSON.stringify([fixedDirectoryMappingV1(image,root),nativePhysicalIdentityTokenV1(mounted.dev),nativePhysicalIdentityTokenV1(mounted.ino)])});',1),
])
PL=M+"📤️publication/🌐️platform/🟦️.ts"
stage(PL,[
 ('import{readNativeProtectedMaintenanceRequestV1,readNativeProtectedMaintenanceDeclarationV1,','import{readNativeProtectedMaintenanceRequestV1,nativePhysicalIdentityTokenV1,nativePhysicalIdentityNumberV1,type NativePhysicalIdentityV1,readNativeProtectedMaintenanceDeclarationV1,',1),
 ('terminalRoot?:Readonly<{dev:number;ino:number;backingIdentity:string}>','terminalRoot?:Readonly<{dev:NativePhysicalIdentityV1;ino:NativePhysicalIdentityV1;backingIdentity:string}>',3),
 ('let terminalRoot:Readonly<{dev:number;ino:number;backingIdentity:string}>|undefined','let terminalRoot:Readonly<{dev:NativePhysicalIdentityV1;ino:NativePhysicalIdentityV1;backingIdentity:string}>|undefined',1),
 ('const root=lstatSync(nativeImageRootV1(captured.directory)),file=lstatSync(join(captured.directory,nativeImageBackingFilenameV1()));','const root=lstatSync(nativeImageRootV1(captured.directory),{bigint:true}),file=lstatSync(join(captured.directory,nativeImageBackingFilenameV1()),{bigint:true});',1),
 ('file.nlink!==1||file.dev!==current.backing.dev||file.ino!==current.backing.ino||file.size!==current.backing.size||nativeImageBackingAllocatedBytesV1(file)!==current.backing.allocatedBytes)throw Error("Actual settled publisher physical facts required");terminalRoot=Object.freeze({dev:root.dev,ino:root.ino,backingIdentity:JSON.stringify([file.dev,file.ino,file.size,nativeImageBackingAllocatedBytesV1(file)])});',
  'file.nlink!==1n||nativePhysicalIdentityTokenV1(file.dev)!==current.backing.dev||nativePhysicalIdentityTokenV1(file.ino)!==current.backing.ino||Number(file.size)!==current.backing.size||Number(nativeImageBackingAllocatedBytesV1(file))!==current.backing.allocatedBytes)throw Error("Actual settled publisher physical facts required");terminalRoot=Object.freeze({dev:nativePhysicalIdentityTokenV1(root.dev),ino:nativePhysicalIdentityTokenV1(root.ino),backingIdentity:JSON.stringify([nativePhysicalIdentityTokenV1(file.dev),nativePhysicalIdentityTokenV1(file.ino),Number(file.size),Number(nativeImageBackingAllocatedBytesV1(file))])});',1),
 ('expectedImage:{receiptSha256:current.receiptSha256,dev:current.backing.dev,ino:current.backing.ino,','expectedImage:{receiptSha256:current.receiptSha256,dev:nativePhysicalIdentityNumberV1(current.backing.dev),ino:nativePhysicalIdentityNumberV1(current.backing.ino),',1),
])
TEST='''/** 🪪️ Contract: a physical identity above 2^53 (a 64-bit NTFS file id, a 128-bit ReFS id) crosses the protected maintenance request exactly: the token is the JSON integer while it is safe and the decimal digits above, two neighbouring ids never collapse into one rounded number, and a refusal never mentions a safe-integer limit. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{nativePhysicalIdentityTokenV1,nativePhysicalIdentityNumberV1,readNativeProtectedMaintenanceRequestV1}from"../../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import observationSchema from"../../🧬️schema/🔣️.json"with{type:"json"};

const big=(1n<<63n)+5n,request=(dev:unknown,ino:unknown)=>({version:1,purpose:"mounted-parent-maintenance",wait:false,receiptSha256:"a".repeat(64),directory:{dev,ino,mode:16877},backing:{dev,ino,size:1048576,allocatedBytes:1048576},root:{dev,ino:5,mode:16877}});

test("an identity of 2^63+5 round-trips exactly and neighbours stay distinct",()=>{
 expect(nativePhysicalIdentityTokenV1(big)).toBe("9223372036854775813");expect(nativePhysicalIdentityTokenV1(big-1n)).not.toBe(nativePhysicalIdentityTokenV1(big));
 expect(Number(big)).toBe(Number(big-1n));expect(nativePhysicalIdentityTokenV1(9007199254740991n)).toBe(9007199254740991);expect(nativePhysicalIdentityTokenV1(9007199254740992n)).toBe("9007199254740992");expect(nativePhysicalIdentityTokenV1((1n<<127n)+1n)).toBe(((1n<<127n)+1n).toString());
 const read=readNativeProtectedMaintenanceRequestV1(request(1234,nativePhysicalIdentityTokenV1(big)));expect(read.backing.ino).toBe("9223372036854775813");expect(BigInt(read.backing.ino as string)).toBe(big);expect(read.backing.dev).toBe(1234);
});

test("tokens are canonical and bounded, and a number above 2^53 is refused as a number",()=>{
 expect(()=>readNativeProtectedMaintenanceRequestV1(request(1,"5"))).toThrow();expect(()=>readNativeProtectedMaintenanceRequestV1(request(1,"09223372036854775813"))).toThrow();expect(()=>readNativeProtectedMaintenanceRequestV1(request(1,9223372036854775813))).toThrow();expect(()=>readNativeProtectedMaintenanceRequestV1(request(1,"9007199254740991"))).toThrow();
 expect(()=>nativePhysicalIdentityTokenV1(-1n)).toThrow();expect(()=>nativePhysicalIdentityTokenV1(1n<<128n)).toThrow();expect(nativePhysicalIdentityNumberV1(7)).toBe(7);expect(()=>nativePhysicalIdentityNumberV1("9223372036854775813")).toThrow("above 2^53");
});

test("the dependency observation wire carries the same exact tokens and no producer converts an identity with Number",()=>{
 const directory={dev:1,ino:"9223372036854775813",mode:16877,kind:"directory"},bad={dev:1,ino:9223372036854775813,mode:16877,kind:"directory"};
 expect(validateJsonSchemaSubset(observationSchema.$defs.Directory as never,directory,observationSchema as never)).toEqual([]);expect(validateJsonSchemaSubset(observationSchema.$defs.Directory as never,bad,observationSchema as never).length).toBeGreaterThan(0);
 const source=readFileSync(fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),"utf8");expect(source).toContain("ino:nativePhysicalIdentityTokenV1(row.ino)");expect(source).not.toContain("ino:number(row.ino)");expect(source).not.toContain("dev:number(row.dev)");
 const fixed=readFileSync(fileURLToPath(new URL("../../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts",import.meta.url)),"utf8"),from=fixed.indexOf("function protectedMaintenancePhysicalFactsV1"),to=fixed.indexOf("export function captureNativeProtectedMaintenancePhysicalV1"),facts=fixed.slice(from,to);
 expect(facts).toContain("{bigint:true}");expect(facts).not.toMatch(/folder\\.ino!==|file\\.ino!==|mounted\\.ino!==/u);
});
'''
put(OBS+"🧪️tests/🪪️exact-identity/🟦️.ts",None,TEST,"new")

stage(L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts",[('backing:Object.freeze({dev:backing.dev,ino:backing.ino,size:backing.size,allocatedBytes:nativeImageBackingAllocatedBytesV1(backing)})','backing:Object.freeze({dev:backing.dev,ino:backing.ino,size:backing.size,allocatedBytes:Number(nativeImageBackingAllocatedBytesV1(backing))})',1)])
SQ="🧰️framework/🔨️modules/⚙️runtime/🗃️sqlite/🟦️.ts"
SQNEW='''import { requireRuntimeMemberV1 } from "../🧭️selection/🟦️.ts";
import type { RuntimeSqliteDatabaseV1, RuntimeSqliteOptionsV1 } from "../📋️contract/🟦️.ts";

type SqliteFilesystemV1={mkdirSync(path:string,options:{recursive:boolean}):unknown;openSync(path:string,flags:string):number;closeSync(descriptor:number):void;unlinkSync(path:string):void;lstatSync(path:string):{size:number;nlink:number};statfsSync(path:string):{bavail:number;bsize:number;ffree:number}};
function sqliteFilesystemV1():SqliteFilesystemV1|undefined{const host=(globalThis as {process?:{getBuiltinModule?:(name:string)=>unknown}}).process;return host?.getBuiltinModule?.("node:fs") as SqliteFilesystemV1|undefined;}
function sqliteDirectoryOfV1(path:string):string{const cut=Math.max(path.lastIndexOf("/"),path.lastIndexOf("\\\\\\\\"));return cut>0?path.slice(0,cut):".";}
/** 🩺️ Names why a SQLite file cannot be opened, in English and German: the path, the errno of a sibling create probe, the database state and the free space of its directory; the original code, errno and result code stay on the error so every classifier of the callers keeps working. */
export function explainSqliteOpenFailureV1(error:unknown,path:string|undefined):unknown{
  const text=error instanceof Error?error.message:String(error);
  if(path===undefined||path===""||path===":memory:"||!/unable to open database file/u.test(text))return error;
  const fs=sqliteFilesystemV1(),facts:string[]=[],code=(cause:unknown)=>(cause as {code?:string}).code??String(cause);let probeCode="unknown";
  if(fs){
    try{const probe=path+".probe-"+String(Date.now());const descriptor=fs.openSync(probe,"wx");fs.closeSync(descriptor);fs.unlinkSync(probe);probeCode="ok";}catch(cause){probeCode=code(cause);}
    facts.push("create probe "+probeCode);
    try{const stat=fs.lstatSync(path);facts.push(`database ${stat.size} bytes nlink ${stat.nlink}`);}catch(cause){facts.push("database "+code(cause));}
    try{const stat=fs.statfsSync(sqliteDirectoryOfV1(path));facts.push(`free ${stat.bavail*stat.bsize} bytes ${stat.ffree} files`);}catch(cause){facts.push("statfs "+code(cause));}
  }
  const detail=`${path} (${facts.join(", ")||"no filesystem probe"})`,quota=probeCode==="EDQUOT"||probeCode==="ENOSPC"?" The disk or the quota of the user is full: free space under the cache directory. / Der Datentr\u00e4ger oder die Quote des Benutzers ist voll: Platz im Cache-Verzeichnis freigeben.":"";
  const source=error as {code?:unknown;errno?:unknown;errcode?:unknown},explained=new Error(`${text}: ${detail}.${quota} / Die SQLite-Datenbank kann nicht ge\u00f6ffnet werden: ${detail}.`,{cause:error});
  return Object.assign(explained,{...(source.code!==undefined?{code:source.code}:{}),...(source.errno!==undefined?{errno:source.errno}:{}),...(source.errcode!==undefined?{errcode:source.errcode}:{}),path,probe:probeCode});
}
/** 📂️ Creates the missing parent directories of a database that is opened for creation, once. */
function createSqliteParentsV1(path:string|undefined,options:RuntimeSqliteOptionsV1|undefined):boolean{
  if(path===undefined||path===""||path===":memory:"||options?.readonly===true||options?.create===false)return false;
  const fs=sqliteFilesystemV1();if(!fs)return false;
  try{fs.lstatSync(sqliteDirectoryOfV1(path));return false;}catch{try{fs.mkdirSync(sqliteDirectoryOfV1(path),{recursive:true});return true;}catch{return false;}}
}
/** 🗃️ A SQLite connection whose implementation is chosen by the hosting runtime. */
export interface SqliteDatabaseV1 extends RuntimeSqliteDatabaseV1 {}
export class SqliteDatabaseV1 {
  constructor(path?: string, options?: RuntimeSqliteOptionsV1) {
    const Member=requireRuntimeMemberV1("sqlite")();
    try{return new Member(path, options) as SqliteDatabaseV1;}
    catch(error){
      if(createSqliteParentsV1(path,options))try{return new Member(path, options) as SqliteDatabaseV1;}catch(second){throw explainSqliteOpenFailureV1(second,path);}
      throw explainSqliteOpenFailureV1(error,path);
    }
  }
}
'''
SQNEW=SQNEW.replace('lastIndexOf("'+'\\'*4+'")','lastIndexOf("'+'\\'*2+'")')
base=open(R+SQ,encoding="utf8").read()
put(SQ,base,SQNEW)
SQT='''/** 🩺️ Contract: a SQLite file that cannot be opened is explained with its path, the errno of a create probe and the free space, in English and German, and keeps its original code; a missing parent directory of a database opened for creation is created once. */
import{expect,test}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync,existsSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{SqliteDatabaseV1,explainSqliteOpenFailureV1}from"../../🟦️.ts";

test("a database below a regular file is refused with the path, the probe errno and both languages",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-r79b-sqlite-"));
 try{writeFileSync(join(root,"file"),"x");const path=join(root,"file","lease.sqlite");let caught:unknown;
  try{new SqliteDatabaseV1(path,{create:true,readwrite:true});}catch(error){caught=error;}
  const error=caught as Error&{path?:string;probe?:string};expect(error).toBeInstanceOf(Error);expect(error.message).toContain(path);expect(error.message).toContain("create probe ENOTDIR");expect(error.message).toContain("Datenbank");expect(error.path).toBe(path);expect(error.probe).toBe("ENOTDIR");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a missing parent directory of a database opened for creation is created once; a read-only open does not create it",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-r79b-sqlite-"));
 try{const path=join(root,"a","b","lease.sqlite"),database=new SqliteDatabaseV1(path,{create:true,readwrite:true});database.exec("CREATE TABLE t(x)");database.close();expect(existsSync(path)).toBe(true);
  expect(()=>new SqliteDatabaseV1(join(root,"c","d","x.sqlite"),{readonly:true})).toThrow();expect(existsSync(join(root,"c"))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("only an open failure is rewritten and the original code survives",()=>{
 const original=Object.assign(Error("unable to open database file"),{code:"SQLITE_CANTOPEN",errno:14});
 const explained=explainSqliteOpenFailureV1(original,"/nonexistent-r79b/dir/x.sqlite") as Error&{code?:string;errno?:number};expect(explained.code).toBe("SQLITE_CANTOPEN");expect(explained.errno).toBe(14);expect(explained.message).toContain("/nonexistent-r79b/dir/x.sqlite");
 const other=Error("database is locked");expect(explainSqliteOpenFailureV1(other,"/x")).toBe(other);expect(explainSqliteOpenFailureV1(original,":memory:")).toBe(original);
});
'''
put("🧰️framework/🔨️modules/⚙️runtime/🗃️sqlite/🧪️tests/🩺️open-failure/🟦️.ts",None,SQT,"new")
open(H+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
print("staged",len(rows))
