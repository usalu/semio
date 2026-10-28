import{refuseBuildV1}from"../../📖️texts/🟦️.ts";
import{createHash}from"node:crypto";
import{capturedSourceImportDiagnosticV1}from"./🧾️diagnostics/🟦️.ts";
import{closureRefusalMessagesV1,closureRefusalTextV1,closureTargetSchemesV1}from"../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";
import{constants,readFileSync}from"node:fs";
import{open,lstat}from"node:fs/promises";
import{isBuiltin}from"node:module";
import{dirname,extname,isAbsolute,join,relative,resolve}from"node:path";
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
import{advanceScriptInvocation,checkScriptInvocation,requireIssuedScriptInvocationV1,type ScriptInvocation}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🧊️ Bounded original module images and module-owned schemas for immutable execution. */
export interface BuildSourceClosureV1{readonly version:1;readonly root:string;readonly entry:string;readonly files:readonly{readonly path:string;readonly sha256:string;readonly base64:string;readonly imports:Readonly<Record<string,string>>}[]}
const installedTargetPattern=/^installed:(?:@[a-z0-9._-]+\/)?[a-z0-9._-]+@[^\s]{1,64}$/u,installedTargetV1=(target:string)=>target.startsWith(closureTargetSchemesV1.installed)&&installedTargetPattern.test(target);
const policy=schema.definitions.Policy.const,hash=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),names=(a:string,b:string)=>Buffer.compare(Buffer.from(a),Buffer.from(b));
function pathIn(root:string,path:string):string{if(!path||isAbsolute(path)||/[:\\\0]/u.test(path)||path.split("/").some(part=>!part||part==="."||part===".."))throw Error("Closed implementation Source path required");const target=join(root,path);if(target.length>256)throw Error("Implementation Source path exceeds 256 characters");return target;}
/** 📖️ A captured child reads only its parent's admitted images; a parent re-reads a torn physical file a bounded number of times. */
async function sourceBytes(root:string,path:string,original:ScriptInvocation):Promise<Buffer>{
 const admitted=admittedSourcesV1 as {root:string;sources:readonly{path:string;sha256:string}[]}|undefined,row=admitted?.root===root?admitted.sources.find(candidate=>candidate.path===path):undefined;
 if(row){checkScriptInvocation(original);const bytes=Buffer.from(readFileSync(pathIn(root,path)));if(hash(bytes)!==row.sha256)throw refuseBuildV1("stale","closure.source-differs");return bytes;}
 for(let attempt=1;;attempt++){try{return await sourceBytesOnceV1(root,path,original);}catch(error){checkScriptInvocation(original);if(attempt>=3||!/changed during capture|descriptor|shortened/u.test(String((error as Error).message)))throw error;}}
}
async function sourceBytesOnceV1(root:string,path:string,original:ScriptInvocation):Promise<Buffer>{
 checkScriptInvocation(original);const target=pathIn(root,path),parents=[];for(let current=dirname(target);;current=dirname(current)){const facts=await lstat(current,{bigint:true});checkScriptInvocation(original);if(!facts.isDirectory()||facts.isSymbolicLink())throw Error("Implementation Source requires no-follow ancestry");parents.push({path:current,dev:facts.dev,ino:facts.ino});if(current===root)break;if(dirname(current)===current)throw Error("Implementation Source escaped its root");}
 const file=await open(target,constants.O_RDONLY|(constants.O_NOFOLLOW??0));let failed=false;
 try{checkScriptInvocation(original);const before=await file.stat({bigint:true});checkScriptInvocation(original);if(!before.isFile()||before.size>BigInt(policy.maximumFileBytes))throw Error("Bounded regular implementation Source required");const bytes=Buffer.alloc(Number(before.size));for(let offset=0;offset<bytes.length;){checkScriptInvocation(original);const row=await file.read(bytes,offset,Math.min(policy.readChunkBytes,bytes.length-offset),offset);checkScriptInvocation(original);if(!row.bytesRead)throw Error("Implementation Source shortened");offset+=row.bytesRead;}const extra=await file.read(Buffer.alloc(1),0,1,bytes.length),after=await file.stat({bigint:true}),named=await lstat(target,{bigint:true});checkScriptInvocation(original);if(extra.bytesRead||named.isSymbolicLink()||["dev","ino","size","mtimeNs","ctimeNs"].some(key=>before[key as keyof typeof before]!==after[key as keyof typeof after]||before[key as keyof typeof before]!==named[key as keyof typeof named]))throw Error("Implementation Source changed during capture");for(const row of parents){const current=await lstat(row.path,{bigint:true});checkScriptInvocation(original);if(!current.isDirectory()||current.isSymbolicLink()||current.dev!==row.dev||current.ino!==row.ino)throw Error("Implementation Source ancestry changed");}return bytes;}catch(error){failed=true;throw error;}finally{try{await file.close();checkScriptInvocation(original);}catch(error){if(!failed)throw error;}}
}

declare global{interface ImportMeta{readonly semioCapturedPhysicalSources?:unknown;readonly semioCapturedSourceResolutions?:unknown}}
const admittedSourcesV1:unknown=import.meta.semioCapturedPhysicalSources;
function requireSourceSelectionDataV1(snapshot:BuildSourceClosureV1,selection:unknown):void{
 if(validateJsonSchemaSubset(schema,snapshot).length||validateJsonSchemaSubset(schema.definitions.AdmittedSources,selection,schema).length)throw Error("Actual admitted parent Source selection required");const value=selection as{version:1;root:string;sources:readonly{path:string;sha256:string}[]};if(!isAbsolute(value.root)||resolve(value.root)!==value.root||snapshot.root!==value.root)throw Error("Same admitted Source root required");const selected=new Map<string,string>();let previous="";for(const row of value.sources){pathIn(value.root,row.path);if(previous&&names(previous,row.path)>=0)throw Error("Canonical admitted Source order required");selected.set(row.path,row.sha256);previous=row.path;}for(const row of snapshot.files)if(selected.get(row.path)!==row.sha256)throw Error("Captured Source differs from admitted parent implementation");
}

/** 🔏️ Binds all executable child images to the actual evaluated parent selection under its Original. */
export function requireAdmittedBuildSourceClosureV1(snapshot:BuildSourceClosureV1,original:ScriptInvocation):void{requireIssuedScriptInvocationV1(original);requireSourceSelectionDataV1(snapshot,admittedSourcesV1);requireIssuedScriptInvocationV1(original);}

type SourceResolutionV1={readonly path:string;readonly sha256:string;readonly imports:Readonly<Record<string,string>>};
const admittedResolutionsV1:unknown=import.meta.semioCapturedSourceResolutions;
function sourceResolutionMapV1(root:string,selection:unknown,admitted:unknown):ReadonlyMap<string,SourceResolutionV1>{
 if(validateJsonSchemaSubset(schema.definitions.AdmittedResolutions,selection,schema).length||validateJsonSchemaSubset(schema.definitions.AdmittedSources,admitted,schema).length)throw Error("Actual admitted Source resolution map required");const value=selection as{version:1;root:string;files:readonly SourceResolutionV1[]},physical=admitted as{version:1;root:string;sources:readonly{path:string;sha256:string}[]};if(!isAbsolute(root)||resolve(root)!==root||value.root!==root||physical.root!==root||value.files.length!==physical.sources.length)throw Error("Same complete admitted Source resolution root required");const result=new Map<string,SourceResolutionV1>();let previous="";for(let index=0;index<value.files.length;index++){const row=value.files[index]!,expected=physical.sources[index]!;pathIn(root,row.path);if(previous&&names(previous,row.path)>=0||row.path!==expected.path||row.sha256!==expected.sha256)throw Error("Canonical admitted Source resolutions required");result.set(row.path,row);previous=row.path;}for(const row of result.values())for(const[specifier,target]of Object.entries(row.imports)){if(!specifier)throw Error("Complete admitted Source resolution target required");if(installedTargetV1(target))continue;if(!result.has(target))throw Error("Complete admitted Source resolution target required");pathIn(root,target);}return result;
}
function resolveSourceImportV1(files:ReadonlyMap<string,SourceResolutionV1>,declaring:string,specifier:string):string{const row=files.get(declaring);if(!row||!Object.hasOwn(row.imports,specifier)||!installedTargetV1(row.imports[specifier]!)&&!files.has(row.imports[specifier]!))throw Error("Admitted implementation declaring image and import required");return row.imports[specifier]!;}

/** 🔨️ How far a capture follows what its files mention: `closure` follows every import the parent resolved and every quoted file literal, which is everything a parent may run; `static` follows only the static imports of each file, the JSON files its text names and the module-owned schemas, plus the declared lazy roots and the declared leaves, files a loaded module reads as bytes and whose own imports are not followed, so that a child which loads anything else is refused at that import. */
export type BuildSourceScopeV1=Readonly<{kind:"closure"}>|Readonly<{kind:"static";roots:readonly string[];leaves:readonly string[]}>;

/** 📦️ Names the specifiers a module imports statically: `import … from`, `import "…"` and `export … from`, but neither type-only forms nor calls such as `import()` or `require()`. */
export function staticImportSpecifiersV1(text:string):ReadonlySet<string>{
 return new Set([...text.matchAll(/(?:^|[;{}\n])[ \t]*(?:import(?![ \t]+type[ \t{*])(?:[^"';]*?[ \t}*]from)?|export(?![ \t]+type[ \t{*])[ \t]*(?:\*(?:[ \t]+as[ \t]+[\w$]+)?|\{[^}"';]*\})[ \t]*from)[ \t]*(["'])([^"'\n]+)\1/gu)].map(match=>match[2]!));
}

/** 📥️ Captures child images only through their actual admitted parent's exact declaring-file import map. */
export async function captureBuildSourceClosureV1(workspace:string,entry:string,original:ScriptInvocation,scope:BuildSourceScopeV1={kind:"closure"}):Promise<BuildSourceClosureV1>{
 requireIssuedScriptInvocationV1(original);checkScriptInvocation(original);const root=resolve(workspace),admitted=sourceResolutionMapV1(root,admittedResolutionsV1,admittedSourcesV1),pending=[entry,...(scope.kind==="static"?scope.roots:[])],files=new Map<string,Buffer>(),resolutions=new Map<string,Readonly<Record<string,string>>>(),schemas=new Map<string,string[]>();let total=0;
 for(const row of admitted.values()){const parts=row.path.split("/"),index=parts.indexOf("🧬️schema");if(index>=0&&extname(row.path)===".json"){const parent=parts.slice(0,index).join("/"),group=schemas.get(parent)??[];group.push(row.path);schemas.set(parent,group);}}
 while(pending.length){checkScriptInvocation(original);const path=pending.pop()!;if(files.has(path))continue;const selected=admitted.get(path);if(!selected)throw refuseBuildV1("refused","closure.source-not-admitted",{path:String(path)});pathIn(root,path);if(path.split("/").some(part=>part==="node_modules"||part===".cache"))throw Error("External implementation runtime dependency refused: "+path);const bytes=await sourceBytes(root,path,original);requireIssuedScriptInvocationV1(original);if(hash(bytes)!==selected.sha256)throw refuseBuildV1("stale","closure.source-differs");files.set(path,bytes);resolutions.set(path,selected.imports);total+=bytes.length;if(files.size>policy.maximumFiles||total>policy.maximumSourceBytes)throw refuseBuildV1("refused","closure.capacity-exceeded",{path:String(JSON.stringify({files:files.size,bytes:total,path,maximumFiles:policy.maximumFiles,maximumSourceBytes:policy.maximumSourceBytes}))});if(extname(path)===".json"){JSON.parse(bytes.toString("utf8"));continue;}
  const followed=scope.kind==="static"?staticImportSpecifiersV1(bytes.toString("utf8")):undefined;for(const target of schemas.get(dirname(path)==="."?"":dirname(path))??[])pending.push(target);for(const specifier of Object.keys(selected.imports)){if(followed&&!followed.has(specifier))continue;const target=resolveSourceImportV1(admitted,path,specifier);if(!installedTargetV1(target))pending.push(target);}for(const match of bytes.toString("utf8").matchAll(/(["'])(\.{1,2}\/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1/gu)){checkScriptInvocation(original);const specifier=match[2]!;if(/\.d\.[cm]?ts$/u.test(specifier)||followed&&!specifier.endsWith(".json"))continue;const candidate=relative(root,resolve(root,dirname(path),specifier)).replaceAll("\\","/");if(admitted.has(candidate))pending.push(candidate);}if(pending.length+files.size>policy.maximumFiles*2)throw Error("Implementation Source asset capacity exceeded");
  if(files.size%policy.progressFiles===0){await advanceScriptInvocation(original,"build:implementation-source","running");await original.control.yieldContinuation();checkScriptInvocation(original);}
 }
 if(scope.kind==="static")for(const path of scope.leaves){checkScriptInvocation(original);if(files.has(path))continue;const selected=admitted.get(path);if(!selected)throw refuseBuildV1("refused","closure.source-not-admitted",{path:String(path)});pathIn(root,path);const bytes=await sourceBytes(root,path,original);requireIssuedScriptInvocationV1(original);if(hash(bytes)!==selected.sha256)throw refuseBuildV1("stale","closure.source-differs");files.set(path,bytes);resolutions.set(path,selected.imports);total+=bytes.length;if(files.size>policy.maximumFiles||total>policy.maximumSourceBytes)throw refuseBuildV1("refused","closure.capacity-exceeded",{path:String(path)});}
 const value:BuildSourceClosureV1=Object.freeze({version:1,root,entry,files:Object.freeze([...files].sort(([a],[b])=>names(a,b)).map(([path,bytes])=>Object.freeze({path,sha256:hash(bytes),base64:bytes.toString("base64"),imports:Object.freeze(Object.fromEntries(Object.entries(resolutions.get(path)!).filter(([,target])=>installedTargetV1(target)||files.has(target))))})))});if(validateJsonSchemaSubset(schema,value).length||Buffer.byteLength(JSON.stringify(value))>policy.maximumSnapshotBytes)throw Error("Closed implementation Source snapshot required");requireAdmittedBuildSourceClosureV1(value,original);checkScriptInvocation(original);return value;
}

/** 🏗️ Hashes the exact serialized executable images rather than mutable disk aliases. */
export function buildSourceClosureIdentityV1(value:BuildSourceClosureV1):string{if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed implementation Source snapshot required");return hash(JSON.stringify(value));}

import { sourceClosureLoaderV1 } from "./🍞️bun/🟦️.ts";
import { buildSchedulingOverlaySuffixV1, buildSchedulingPrefixesV1 } from "./🗓️scheduling/🟦️.ts";


/** 📁️ Executes an immutable file snapshot through the shared checksum-bound module loader. */
export const buildSourceClosureLoaderV1=sourceClosureLoaderV1(false,{prefixes:buildSchedulingPrefixesV1(),suffix:buildSchedulingOverlaySuffixV1});

/** 🚰️ Executes bounded owned auxiliary DATA without consuming ordinary stdin or mutable source aliases. */
export const buildSourceClosureStreamLoaderV1=sourceClosureLoaderV1(true);
