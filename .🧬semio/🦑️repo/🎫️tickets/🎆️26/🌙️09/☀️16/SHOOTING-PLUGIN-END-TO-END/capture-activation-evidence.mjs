/** 🔬️ Observes actual activation outputs without issuing producer authority or changing installed bytes. */
import {browserProductRootsV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/📂️paths/🟦️.ts";
import {ACTIVATION_RECEIPT_FILE,parseActivationReceipt,developmentRuntimeRoot} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import {findWorkspaceRoot} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {componentModelLayoutV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/📋️contract/🟦️.ts";
import {readComponentReceiptV1,COMPONENT_RECEIPT_FILE} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import {componentSourceContentHash} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🔍️freshness/🟦️.ts";
import modelSchema from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🧬️schema/🔣️.json" with {type:"json"};
import {COMPONENT_MODULE_DIRECTORIES,PLUGIN_BUILD_TARGETS,EXTENSION_TARGETS} from "../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/📦️packages/🟦️typescript/📚️catalog/🧩️plugins/🟦️.ts";
import {moduleDirectoryName,componentOwnerRootV1,MODULE_SHARD_DIRECTORY} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import {PREVIEW2_VENDOR_RELATIVE} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";
import {repositoryCachePartitionRootDataV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import {fontAssetPublicationRootV1} from "../../../../../../../🧰️framework/🔨️modules/🖼️canvas/📦️packages/🟦️typescript/🔤️fonts/📦️publication/🟦️.ts";
import {createHash} from "node:crypto";
import {createReadStream,existsSync,lstatSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {readdir,lstat} from "node:fs/promises";
import {join,relative,resolve} from "node:path";
import assert from "node:assert/strict";

const workspace=findWorkspaceRoot(),output=join(import.meta.dir,"🗑️generated","activation-evidence");
const [command,label,modeArgument]=process.argv.slice(2);
const mode=command==="inspect"?(label??"prototype"):(modeArgument??"prototype");
const controller=new AbortController(),cancel=()=>controller.abort();
process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
const protocol=modelSchema.definitions.Protocol.const,targets=[...PLUGIN_BUILD_TARGETS,...EXTENSION_TARGETS];
const limits={files:20000,bytes:2*1024*1024*1024,fileBytes:512*1024*1024};
let count=0,total=0;
const name=value=>{if(!/^[a-z0-9][a-z0-9-]{0,31}$/.test(value??""))throw Error("Bounded lowercase snapshot label required");return value;};
const destination=value=>{const path=join(output,name(value)+".json");if(path.length>256)throw Error("Bounded evidence path required");return path;};
const sha=value=>createHash("sha256").update(value).digest("hex");
const stats=info=>({bytes:Number(info.size),mtimeNs:String(info.mtimeNs),ctimeNs:String(info.ctimeNs),mode:Number(info.mode),device:String(info.dev),inode:String(info.ino)});
const stable=(a,b)=>["size","mtimeNs","ctimeNs","mode","dev","ino"].every(key=>a[key]===b[key]);
const locations=selectedMode=>{
 const roots=browserProductRootsV1(workspace,selectedMode),products=repositoryCachePartitionRootDataV1(workspace,"products");
 const target=targets.find(row=>row.pluginId==="shooting");
 if(!target)throw Error("Current shooting catalog target required");
 const directoryName=moduleDirectoryName("shooting",COMPONENT_MODULE_DIRECTORIES),runtime=developmentRuntimeRoot(workspace,"shooting",selectedMode,"react");
 const model=componentModelLayoutV1(products,{version:1,mode:selectedMode,pluginId:"shooting",cratePath:target.cratePath});
 return {mode:selectedMode,products,roots,runtime,target,directoryName,module:join(roots.modules,directoryName),model,fonts:fontAssetPublicationRootV1(products,selectedMode),activationReceipt:join(runtime,"activation",ACTIVATION_RECEIPT_FILE)};
};
async function file(path){
 controller.signal.throwIfAborted();
 const before=await lstat(path,{bigint:true});
 if(!before.isFile()||before.isSymbolicLink()||before.size>BigInt(limits.fileBytes)||path.length>256)throw Error("Finite regular evidence file required: "+path);
 if(++count>limits.files||Number(before.size)>limits.bytes-total)throw Error("Evidence extent exceeded");
 total+=Number(before.size);
 const hash=createHash("sha256");let seen=0,nextProgress=16*1024*1024;
 for await(const chunk of createReadStream(path,{highWaterMark:65536,signal:controller.signal})){
  controller.signal.throwIfAborted();hash.update(chunk);seen+=chunk.length;
  if(seen>=nextProgress){console.log("[DEBUG] hashing "+relative(workspace,path)+" "+seen+" bytes");nextProgress+=16*1024*1024;}
 }
 const after=await lstat(path,{bigint:true});
 if(!stable(before,after)||seen!==Number(before.size))throw Error("File changed during evidence capture: "+path);
 return {path,...stats(after),sha256:hash.digest("hex")};
}
async function tree(root){
 if(!existsSync(root))return {root,exists:false,files:[]};
 const before=await lstat(root,{bigint:true});
 if(!before.isDirectory()||before.isSymbolicLink())throw Error("Regular evidence directory required: "+root);
 const files=[];
 const walk=async directory=>{
  for(const entry of (await readdir(directory,{withFileTypes:true})).sort((a,b)=>a.name.localeCompare(b.name))){
   controller.signal.throwIfAborted();
   const path=join(directory,entry.name);
   if(entry.isSymbolicLink())throw Error("Linked evidence entry refused: "+path);
   if(entry.isDirectory())await walk(path);
   else if(entry.isFile())files.push({relative:relative(root,path).replaceAll("\\","/"),...await file(path)});
   else throw Error("Special evidence entry refused: "+path);
  }
 };
 await walk(root);
 const after=await lstat(root,{bigint:true});
 if(!stable(before,after))throw Error("Directory changed during evidence capture: "+root);
 return {root,exists:true,stat:stats(after),files};
}
async function receipt(path,component=false){
 if(!existsSync(path))return {path,exists:false};
 const info=await file(path);if(info.bytes>1024*1024)throw Error("Bounded receipt required");
 const text=readFileSync(path,"utf8"),value=component?readComponentReceiptV1(path):parseActivationReceipt(JSON.parse(text));
 if(sha(Buffer.from(text))!==info.sha256)throw Error("Receipt changed during readback");
 return {exists:true,...info,text,value};
}
const taskFiles=[
"🌎️hub/🧩️compositions/🎥️shooting/🧩️component/📦️packages/🦀️rust/Cargo.toml",
"✏️s/🧑‍💻dev/🎭️variants/🎥️shooting/🧬️schema/🔣️.json",
"✏️s/🧑‍💻dev/🎭️variants/🎥️shooting/🏗️builder/🌐️vite/🟦️.ts",
"✏️s/🧑‍💻dev/🎭️variants/🎥️shooting/🚀️entry/🟦️.ts",
"🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts",
"🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/⚡️host-effects/🟦️.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎨️theme-driver-settings/🛍️settings-and-marketplace-hosts/🟦️.tsx",

 "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🔌️host-ports/🟦️.tsx",
 "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🔌️host-ports/🧊️transform/🟦️.ts",
 "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🧊️icon-render/🟦️.ts",
 "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/📤️export/🟦️.ts",
 "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/⚡️host-effects/📤️export-effects/🟦️.ts",
 "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧵️TaskManager/🟦️.tsx"
];
async function sources(ids){
 const rows={};
 for(const id of ids){
  const target=targets.find(row=>row.pluginId===id);if(!target)throw Error("Unknown activation component: "+id);
  const root=resolve(workspace,componentOwnerRootV1(target.cratePath));
  rows["component-owner:"+id]=existsSync(root)?await componentSourceContentHash(root,20000,controller.signal):null;
 }
 const plugin=join(workspace,"✏️s/🔌️plugins/🎥️shooting");
 rows["shooting-implementation"]=await componentSourceContentHash(plugin,20000,controller.signal);
 rows["activation-implementation"]=await componentSourceContentHash(join(workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation"),20000,controller.signal);
 for(const path of taskFiles)rows[path]=existsSync(join(workspace,path))?(await file(join(workspace,path))).sha256:null;
 return rows;
}
function summarizeFiles(group){
 return {exists:group.exists,directoryMtimeNs:group.stat?.mtimeNs,files:group.files.map(row=>({relative:row.relative,bytes:row.bytes,sha256:row.sha256,mtimeNs:row.mtimeNs,mode:row.mode}))};
}
async function capture(snapshotLabel){
 const start=new Date().toISOString(),where=locations(mode),activation=await receipt(where.activationReceipt);
 if(activation.exists&&(activation.value.variant!=="shooting"||activation.value.profile!==mode))throw Error("Exact shooting activation identity required");
 const ids=activation.exists?activation.value.plugins.map(row=>row.pluginId):["shooting"];
 if(ids.length>64)throw Error("Bounded selected component capture required");
 const beforeSources=await sources(ids),installed={};
 for(const id of ids)installed["module:"+id]=await tree(join(where.roots.modules,moduleDirectoryName(id,COMPONENT_MODULE_DIRECTORIES)));
 installed["support:vendor"]=await tree(join(where.roots.modules,PREVIEW2_VENDOR_RELATIVE));
 installed["support:shard"]=await tree(join(where.roots.modules,MODULE_SHARD_DIRECTORY));
 installed["support:fonts"]=await tree(where.fonts);
 if(ids.some(id=>targets.find(row=>row.pluginId===id)?.role==="extension"))installed["runtime:extensions"]=await tree(join(where.runtime,"extensions"));
 const model=await tree(where.model.path),rawReceipt=await receipt(join(where.model.path,protocol.receipt),true),copiedReceipt=await receipt(join(where.module,COMPONENT_RECEIPT_FILE),true);
 const raw=model.files.find(row=>row.relative===protocol.component),pack=model.files.find(row=>row.relative===protocol.descriptorPack);
 const findings=[];
 if(!rawReceipt.exists)findings.push("canonical component model receipt is absent");
 else{
  const value=rawReceipt.value;
  if(value.pluginId!=="shooting"||value.packageId!==where.target.packageId||value.profile!==mode)findings.push("raw component identity/mode mismatch");
  if(!raw||raw.sha256!==value.wasmSha256||raw.bytes!==value.wasmBytes)findings.push("raw component bytes do not match receipt");
  if(!pack||pack.sha256!==value.semanticDescriptorSha256)findings.push("raw semantic descriptor does not match receipt");
 }
 if(!copiedReceipt.exists||!rawReceipt.exists||copiedReceipt.text!==rawReceipt.text)findings.push("installed module receipt differs from canonical model receipt");
 if(rawReceipt.exists&&model.files.find(row=>row.relative===protocol.receipt)?.sha256!==rawReceipt.sha256)findings.push("model receipt changed after tree observation");
 if(copiedReceipt.exists&&installed["module:shooting"]?.files.find(row=>row.relative===COMPONENT_RECEIPT_FILE)?.sha256!==copiedReceipt.sha256)findings.push("module receipt changed after tree observation");
 const installedChurnDuringCapture=[];
 for(const [key,group]of Object.entries(installed))if(group.exists){const now=stats(await lstat(group.root,{bigint:true}));if(["mtimeNs","ctimeNs","device","inode"].some(field=>now[field]!==group.stat[field]))installedChurnDuringCapture.push(key);}
 const afterSources=await sources(ids),afterActivation=await receipt(where.activationReceipt);
 if(JSON.stringify(activation)!==JSON.stringify(afterActivation))findings.push("activation receipt changed during capture");
 const result={schema:"semio.shooting.activation-evidence/v1",label:snapshotLabel,start,end:new Date().toISOString(),locations:where,catalogMapping:ids.map(id=>{const row=targets.find(row=>row.pluginId===id);return {pluginId:id,cratePath:row.cratePath,role:row.role,directoryName:moduleDirectoryName(id,COMPONENT_MODULE_DIRECTORIES)};}),sourceCoverage:"selected component-owner aggregates, shooting implementation and task frontend files; observed Cargo source identity remains in producer receipt",beforeSources,afterSources,sourceChurnDuringCapture:JSON.stringify(beforeSources)!==JSON.stringify(afterSources),activation,installed,mutableSourceState:await tree(join(where.runtime,"🔖️")),rawComponent:{model,rawReceipt,copiedReceipt,findings},installedChurnDuringCapture,freshness:"not asserted; actual owned producer execution/provenance required",physicalActivationExecutionVerified:false};
 mkdirSync(output,{recursive:true});writeFileSync(destination(snapshotLabel),JSON.stringify(result,null,2)+"\n");
 console.log("[DEBUG] captured "+snapshotLabel+" "+count+" files / "+total+" bytes; raw findings "+findings.length);
}
function compare(first,second){
 const a=JSON.parse(readFileSync(destination(first),"utf8")),b=JSON.parse(readFileSync(destination(second),"utf8"));
 assert.equal(a.schema,"semio.shooting.activation-evidence/v1");assert.equal(b.schema,a.schema);
 const rawA=a.rawComponent.rawReceipt.value,rawB=b.rawComponent.rawReceipt.value;
 const sourceChurn={duringCapture:a.sourceChurnDuringCapture||b.sourceChurnDuringCapture,watchedSources:JSON.stringify(a.afterSources)!==JSON.stringify(b.afterSources),producerSources:rawA?.sourceTreeDigest!==rawB?.sourceTreeDigest,toolchain:JSON.stringify(rawA?.toolchain)!==JSON.stringify(rawB?.toolchain),catalog:JSON.stringify(a.catalogMapping)!==JSON.stringify(b.catalogMapping),roots:JSON.stringify(a.locations)!==JSON.stringify(b.locations)};
 const incomplete=!a.activation.exists||!b.activation.exists||!a.activation.value.plugins.some(row=>row.pluginId==="shooting")||!b.activation.value.plugins.some(row=>row.pluginId==="shooting")||a.rawComponent.findings.length>0||b.rawComponent.findings.length>0||a.installedChurnDuringCapture.length>0||b.installedChurnDuringCapture.length>0||!Object.values(a.installed).every(row=>row.exists&&row.files.length>0)||!Object.values(b.installed).every(row=>row.exists&&row.files.length>0);
 const installedChanges=[...new Set([...Object.keys(a.installed),...Object.keys(b.installed)])].filter(key=>JSON.stringify(a.installed[key]&&summarizeFiles(a.installed[key]))!==JSON.stringify(b.installed[key]&&summarizeFiles(b.installed[key])));
 const activationChanged=!a.activation.exists||!b.activation.exists||a.activation.text!==b.activation.text||a.activation.mtimeNs!==b.activation.mtimeNs;
 const rawIdentityChanged=JSON.stringify(rawA)!==JSON.stringify(rawB);
 const mutableSourceStateChanged=JSON.stringify(a.mutableSourceState)!==JSON.stringify(b.mutableSourceState);
 const outcome=incomplete?"incomplete-evidence":Object.values(sourceChurn).some(Boolean)?"source-churn":installedChanges.length||activationChanged||rawIdentityChanged?"invariant-difference":"observed-invariants-unchanged";
 const result={schema:"semio.shooting.activation-comparison/v1",first,second,outcome,sourceChurn,installedChanges,activationChanged,rawIdentityChanged,mutableSourceStateChanged,physicalActivationExecutionVerified:false,remaining:"correlate with actual issued producer/activation logs; browser acceptance is separate"};
 const path=join(output,"comparison-"+name(first)+"-"+name(second)+".json");if(path.length>256)throw Error("Bounded comparison path required");
 mkdirSync(output,{recursive:true});writeFileSync(path,JSON.stringify(result,null,2)+"\n");console.log(JSON.stringify(result,null,2));
 if(outcome!=="observed-invariants-unchanged")process.exitCode=2;
}
try{
 if(command==="inspect"){const where=locations(mode);console.log(JSON.stringify({schema:"semio.shooting.activation-inspection/v1",locations:where,exists:{module:existsSync(where.module),model:existsSync(where.model.path),fonts:existsSync(where.fonts),activationReceipt:existsSync(where.activationReceipt)},freshness:"not asserted"},null,2));}
 else if(command==="capture"){name(label);await capture(label);}
 else if(command==="compare"){name(label);name(modeArgument);compare(label,modeArgument);}
 else throw Error("Usage: capture-activation-evidence.mjs inspect [prototype|release] | capture <label> [prototype|release] | compare <first> <second>");
}catch(error){if(controller.signal.aborted){console.error("[DEBUG] evidence capture cancelled");process.exitCode=130;}else{console.error("[DEBUG] evidence capture failed",error);process.exitCode=1;}}
finally{process.off("SIGINT",cancel);process.off("SIGTERM",cancel);}
