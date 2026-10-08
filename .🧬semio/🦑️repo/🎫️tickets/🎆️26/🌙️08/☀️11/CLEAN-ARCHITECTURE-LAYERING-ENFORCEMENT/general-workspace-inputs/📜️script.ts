import assert from "node:assert/strict";
import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {dirname,join,relative,resolve,sep} from "node:path";
import {parse as parseToml} from "@iarna/toml";
import {parse as parseJsonc,modify,applyEdits} from "jsonc-parser";
import {parse as parseJson5} from "json5";

const ticket=dirname(import.meta.dir),[command]=process.argv.slice(2);assert.equal(command,"publish");
process.chdir(resolve(ticket,"../../../../../../.."));
const generated=join(ticket,"🗑️generated"),plan=JSON.parse(readFileSync(join(generated,"current-general-workspace-publication-plan.json"),"utf8")),rootBefore=readFileSync("Cargo.toml","utf8");
assert.equal(rootBefore,plan.rootPreimage);assert(!existsSync("🧰️framework/Cargo.toml"));
const root=Bun.TOML.parse(rootBefore) as any,general=Bun.TOML.parse(plan.generalAfter) as any,members=new Set<string>(plan.bodies.map((body:any)=>dirname(body.path))),nodeGraph="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust",rootMembers=[...root.workspace.members.filter((member:string)=>!members.has(member)),...(root.workspace.members.includes(nodeGraph)?[]:[nodeGraph])];
assert.equal(new Set(rootMembers).size,rootMembers.length);
let rootAfter=rootBefore;
for(const member of members){const line="    "+JSON.stringify(member)+",\n";assert.equal(rootAfter.split(line).length,2);rootAfter=rootAfter.replace(line,"");}
if(!root.workspace.members.includes(nodeGraph))rootAfter=rootAfter.replace("members = [\n","members = [\n    "+JSON.stringify(nodeGraph)+",\n");
rootAfter=rootAfter.replace("exclude = [\n","exclude = [\n    \"🧰️framework\",\n");
const parsedRoot=Bun.TOML.parse(rootAfter) as any;assert.deepEqual(parsedRoot,parseToml(rootAfter));assert.deepEqual({...parsedRoot,workspace:{...parsedRoot.workspace,members:root.workspace.members,exclude:root.workspace.exclude}},root);
const rows:{path:string;before:string|null;after:string}[]=[];
const origin=(source:string,owner:string)=>{const declaration=/^workspace[ \t]*=[ \t]*"[^"\r\n]*"[ \t]*$/m;if(declaration.test(source))return source.replace(declaration,"workspace = "+JSON.stringify(owner));assert(source.includes("[package]\n"));return source.replace("[package]\n","[package]\nworkspace = "+JSON.stringify(owner)+"\n");};
for(const body of plan.bodies){assert.equal(readFileSync(body.path,"utf8"),body.source);const after=origin(body.source,relative(dirname(body.path),"🧰️framework").split(sep).join("/")),beforeObject=Bun.TOML.parse(body.source) as any,afterObject=Bun.TOML.parse(after) as any;assert.deepEqual({...afterObject,package:{...afterObject.package,workspace:beforeObject.package.workspace}},beforeObject);rows.push({path:body.path,before:body.source,after});}
for(const member of rootMembers){const path=join(member,"Cargo.toml"),before=readFileSync(path,"utf8"),parsed=Bun.TOML.parse(before) as any;assert(!parsed.workspace);const after=origin(before,relative(member,".").split(sep).join("/"));if(after!==before)rows.push({path,before,after});}
rows.push({path:"Cargo.toml",before:rootBefore,after:rootAfter},{path:"🧰️framework/Cargo.toml",before:null,after:plan.generalAfter});
const generalLock="🧰️framework/Cargo.lock";rows.push({path:generalLock,before:existsSync(generalLock)?readFileSync(generalLock,"utf8"):null,after:readFileSync("Cargo.lock","utf8")});
const format={formattingOptions:{insertSpaces:true,tabSize:2,eol:"\n"}},manifestPath="package.json",manifestBefore=readFileSync(manifestPath,"utf8"),manifest=JSON.parse(manifestBefore),exclusion="!"+nodeGraph+"/🕸️bindings";
assert(!manifest.workspaces.includes(exclusion));const manifestAfter=applyEdits(manifestBefore,modify(manifestBefore,["workspaces",manifest.workspaces.length],exclusion,format));assert.deepEqual({...JSON.parse(manifestAfter),workspaces:manifest.workspaces},manifest);rows.push({path:manifestPath,before:manifestBefore,after:manifestAfter});
const lockPath="bun.lock",lockBefore=readFileSync(lockPath,"utf8"),lock=parseJson5(lockBefore) as any,nodePkg=JSON.parse(readFileSync(nodeGraph+"/package.json","utf8")),reactOwner="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript",react=JSON.parse(readFileSync(reactOwner+"/package.json","utf8"));
let lockAfter=lockBefore;
const edits:[(string|number)[],unknown][]=[[["workspaces",nodeGraph],{name:nodePkg.name}],[["packages",nodePkg.name],[nodePkg.name+"@workspace:"+nodeGraph]],[["workspaces",reactOwner,"dependencies"],react.dependencies]];
for(const [path,value]of edits)lockAfter=applyEdits(lockAfter,modify(lockAfter,path,value,format));
const lockParsed=parseJson5(lockAfter) as any;assert.deepEqual(parseJsonc(lockAfter),lockParsed);
const strip=(value:any)=>{value=structuredClone(value);delete value.workspaces[nodeGraph];delete value.packages[nodePkg.name];value.workspaces[reactOwner].dependencies=lock.workspaces[reactOwner].dependencies;return value;};assert.deepEqual(strip(lockParsed),strip(lock));rows.push({path:lockPath,before:lockBefore,after:lockAfter});
const journal={phase:"prepared",rows,atomicPublicationClaimed:false,newRuntimeVersionsAdded:false},destination=join(generated,"current-general-workspace-publication.json");assert(!existsSync(destination));writeFileSync(destination,JSON.stringify(journal));
for(const row of rows){assert.equal(existsSync(row.path)?readFileSync(row.path,"utf8"):null,row.before,"Source advanced "+row.path);writeFileSync(row.path,row.after);assert.equal(readFileSync(row.path,"utf8"),row.after);}
writeFileSync(destination,JSON.stringify({...journal,phase:"completed"}));console.log("[DEBUG] "+JSON.stringify({generalMembers:members.size,rootSpecificMembers:rootMembers.length,providerRows:Object.keys(general.workspace.dependencies).length,ownedFiles:rows.length,newNodeGraph:nodePkg.name,atomicPublicationClaimed:false}));
