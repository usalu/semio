/** bun surrogate.ts [--edits] : offline key surrogate of the course Bazel actions, old rules (monolithic executor identity, router-union command closure, scheduling inside) against the new rules (keyed executor identity without the scheduling overlay, per-command closure, scheduling excluded). No graph run, no Bazel: virtual edits only (an in-memory text override of one file), the real split/identity functions of the closure module. */
import {execFileSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,readFileSync,readdirSync,statSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {dirname,join,normalize,relative,resolve,sep} from "node:path";
import ts from "typescript";

const root=process.cwd(),lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const bazel=lib+"🏗️build/🔧️tools/🏗️bazel/";
const closureModule=await import(resolve(root,lib+"🏗️build/📥️source/🔏️closure/🟦️.ts")),scheduling=await import(resolve(root,lib+"🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts"));
const {buildTaskInputsV1}=await import(resolve(root,lib+"🏗️build/🧮️task-model/🟦️.ts"));
const inference=await import(resolve(root,lib+"🏗️build/🕸️graph/📥️inference/🟨️.mjs")) as any;await inference.libraryBootstrap;
const scriptsModule=await import(resolve(root,lib+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs")) as any;
const {bindBuildCommandV1}=await import(resolve(root,lib+"🏗️build/📤️arguments/🟨️.mjs")) as any;

const sha=(value:string|Buffer)=>createHash("sha256").update(value).digest("hex");
const edits=new Map<string,string>();
const read=(path:string):Buffer=>edits.has(path)?Buffer.from(edits.get(path)!):readFileSync(resolve(root,path));
const digests=new Map<string,string>();
const digest=(path:string):string=>{if(edits.has(path))return sha(edits.get(path)!);let known=digests.get(path);if(known===undefined){try{known=sha(readFileSync(resolve(root,path)));}catch{known="missing";}digests.set(path,known);}return known;};

const g=".cache/b/g/m/generations/",cur=JSON.parse(readFileSync(g+"current.json","utf8")),man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const projects:any={},dependencies:any[]=[];
for(const [name,row] of Object.entries<any>(man.owners)){if(row.kind!=="project")continue;let frag:any;try{frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));}catch{continue}projects[name]={name,root:frag.root,namedInputs:frag.node?.namedInputs,targets:frag.node?.targets??{}};for(const e of frag.edges??[])dependencies.push({source:e.source,target:e.target});}
const policy=JSON.parse(readFileSync(lib+"🏗️build/🔣️policy.json","utf8"));
const inventory=execFileSync("git",["ls-files","-z","--cached","--others","--exclude-standard"],{cwd:root,maxBuffer:1<<30}).toString("utf8").split("\0").filter(Boolean);

const specs=["@semio-tech/ui-styling-tokens#generate","@semio-tech/ui-rs#generate","@semio-tech/ui-contract-rs#generate","@semio-tech/framework-rs#generate","@semio-tech/framework-actor-rs#typegen","@semio-tech/assets#build","@teaching/architecture-course#build"];

const cache=inference.createScriptInputCache({workspaceRoot:root});
const stripped=(path:string)=>path.replace("{workspaceRoot}/","");
const selection=(graph:any,project:string,definition:any)=>{
 const patterns=buildTaskInputsV1(graph,{project,definition}).filter((entry:any)=>entry.kind==="file");
 const pos=patterns.filter((e:any)=>!e.exclude).map((e:any)=>new Bun.Glob(e.pattern.replace("{workspaceRoot}/",""))),neg=patterns.filter((e:any)=>e.exclude).map((e:any)=>new Bun.Glob(e.pattern.replace("{workspaceRoot}/","")));
 return inventory.filter(file=>pos.some(glob=>glob.match(file))&&!neg.some(glob=>glob.match(file)));
};
const oldGraph={version:1,projects,dependencies,namedInputs:policy.namedInputs};
const newGraph={...oldGraph,namedInputs:{...policy.namedInputs,schedulingSourceExclusions:scheduling.buildSchedulingExclusionPatternsV1()}};
const rawTarget=(projectRoot:string,target:string)=>JSON.parse(readFileSync(join(projectRoot,"📋️project.json"),"utf8")).targets[target];
const boundScripts=(target:any,projectRoot:string)=>{const options=target.options??{},commands=typeof options.command==="string"?[options.command]:options.commands??[];const scripts:string[]=[];for(const operation of commands){const command=typeof operation==="string"?operation:operation.command;try{const bound=bindBuildCommandV1(command,root,options.cwd??".",projectRoot,options.args??[]);if(bound.script)scripts.push(bound.script);}catch{}}return scripts;};

type Action={id:string;project:string;target:string;old:string[];new:string[]};
const actions:Action[]=[];
for(const spec of specs){
 const [project,target]=spec.split("#"),def=projects[project!].targets[target!],projectRoot=projects[project!].root,raw=rawTarget(projectRoot,target!);
 const newClosure=((await scriptsModule.targetScriptSources(raw,root,cache,projectRoot)).closure??[]).map(stripped) as string[];
 const oldClosure=(await scriptsModule.relativeScriptInputs(boundScripts(raw,projectRoot),root,inference.createScriptInputCache({workspaceRoot:root}),true)).map(stripped) as string[];
 const negations=scheduling.buildSchedulingPrefixesV1(),notScheduling=(file:string)=>!negations.some(prefix=>file.startsWith(prefix));
 const oldDeclared=new Set([...selection(oldGraph,project!,def),...oldClosure]);
 const newDef={...def,inputs:[...(def.inputs??[]),"schedulingSourceExclusions"]};
 const newDeclared=new Set([...selection(newGraph,project!,newDef),...newClosure.filter(notScheduling)]);
 actions.push({id:spec,project:project!,target:target!,old:[...oldDeclared].sort(),new:[...newDeclared].sort()});
 console.log(spec.padEnd(44),"declared old",String(oldDeclared.size).padStart(6),"new",String(newDeclared.size).padStart(6),"| command closure old",String(oldClosure.length).padStart(5),"new",String(newClosure.length).padStart(4));
}

const table=JSON.parse(readFileSync(process.env.SCOPES??bazel+"🎬️scopes/🔣️.json","utf8")).scopes[0] as {roots:string[];leaves:string[];scheduling?:string[]};
const entry=bazel+"🎬️executor/🟦️.ts";
const regular=(path:string)=>existsSync(path)&&statSync(path).isFile()&&!path.split(sep).some(part=>part==="node_modules"||part===".cache");
const schemaFiles=(folder:string):string[]=>existsSync(folder)?readdirSync(folder,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1).flatMap(e=>e.isDirectory()&&e.name!=="node_modules"?schemaFiles(join(folder,e.name)):e.isFile()&&e.name.endsWith(".json")?[join(folder,e.name)]:[]):[];
const staticSpecifiers=(path:string):string[]=>{const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),found:string[]=[];for(const node of file.statements){if(ts.isImportDeclaration(node)&&!node.importClause?.isTypeOnly&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);if(ts.isExportDeclaration(node)&&!node.isTypeOnly&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);}return found;};
const jsonLiteral=/(["'])(\.{1,2}\/[^"'\\\r\n]+\.json)\1/gu;
const closure=(()=>{const seen=new Set<string>(),pending=[normalize(resolve(root,entry)),...table.roots.map(r=>normalize(resolve(root,r)))];while(pending.length){const path=pending.pop()!;if(seen.has(path)||!regular(path))continue;seen.add(path);if(path.endsWith(".json"))continue;pending.push(...schemaFiles(join(dirname(path),"🧬️schema")));for(const s of staticSpecifiers(path))if(s.startsWith("."))pending.push(normalize(join(dirname(path),s)));for(const m of readFileSync(path,"utf8").matchAll(jsonLiteral))pending.push(normalize(join(dirname(path),m[2]!)));}for(const leaf of table.leaves)if(regular(normalize(resolve(root,leaf))))seen.add(normalize(resolve(root,leaf)));return seen;})();
const imports=(path:string)=>{const map:Record<string,string>={};if(!path.endsWith(".json"))for(const s of staticSpecifiers(path)){if(!s.startsWith("."))continue;const target=normalize(join(dirname(path),s));if(closure.has(target))map[s]=relative(root,target);}return map;};
const snapshot=()=>({version:1 as const,root,entry,files:[...closure].sort().map(path=>{const local=relative(root,path),bytes=read(local);return {path:local,sha256:sha(bytes),base64:bytes.toString("base64"),imports:imports(path)};})});
const loaderSha=sha(closureModule.buildSourceClosureLoaderV1);

const identityOld=()=>closureModule.buildSourceClosureIdentityV1(snapshot());
const identityNew=()=>closureModule.buildSourceClosureIdentityV1(scheduling.splitBuildSourceClosureV1(snapshot(),table.scheduling??[]).keyed);

const keyOf=(action:Action,which:"old"|"new"):string=>{
 const files=which==="old"?action.old:action.new,hash=createHash("sha256");
 hash.update(which==="old"?identityOld():identityNew());hash.update(loaderSha);hash.update(action.id);
 for(const file of files)hash.update(file+"\0"+digest(file)+"\0");
 return hash.digest("hex");
};
const keys=(which:"old"|"new")=>Object.fromEntries(actions.map(action=>[action.id,keyOf(action,which)]));

const cases:{name:string;path:string;why:string}[]=[];
const find=(fragment:string)=>[...closure].map(path=>relative(root,path)).find(path=>path.includes(fragment));
const pick=(label:string,fragment:string,why:string)=>{const path=find(fragment);if(!path)throw Error("surrogate file missing: "+fragment);cases.push({name:label,path,why});};
pick("scheduling: lease order module","🔒️leases/🧭️order/🟦️.ts","executor + command closures");
pick("scheduling: host capacity","🌡️host-capacity/🟦️.ts","executor");
pick("scheduling: cache store custody","🏪️store/🟦️.ts","executor");
pick("scheduling: invocation envelope","🧭️routing/📥️invocation/🟦️.ts","executor + command closures");
pick("scheduling: progress texts","📖️texts/🟦️.ts","executor");
pick("output-producing: executor body","🎬️executor/🟦️.ts","executor");
pick("output-producing: task environment (caller)","🏗️build/📥️caller/🟦️.ts","executor");
pick("output-producing: glob of declared outputs","⚙️runtime/🌐️glob/🟦️.ts","executor");
const extra=[
 ["other command of the router: vitest runner (test command)","🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts","ui-styling router closure"],
 ["own command: styling projection (generate)","🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts","ui-styling-tokens generate"],
 ["own command: file writer (generate)","🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗂️files/🟦️.ts","every generator"],
 ["own command script: assets router","🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts","assets build"],
 ["own command script: ui-styling router","🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts","ui-styling-tokens generate"],
 ["own command script: course router","🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📜️script.ts","course build"]
] as const;
for(const [name,path,why] of extra)if(existsSync(resolve(root,path)))cases.push({name,path,why});

const base={old:keys("old"),new:keys("new")};
const rows:string[]=["| edit (virtual) | file | "+actions.map(a=>a.id.split("/")[1]!.replace("#",":")).join(" | ")+" |","|---|---|"+actions.map(()=>"---").join("|")+"|"];
const result:Record<string,{old:string[];new:string[]}>={};
for(const item of cases){
 const original=read(item.path).toString("utf8");edits.set(item.path,original+"\n// exec-f virtual edit\n");
 const after={old:keys("old"),new:keys("new")};edits.delete(item.path);
 const changed={old:actions.filter(a=>after.old[a.id]!==base.old[a.id]).map(a=>a.id),new:actions.filter(a=>after.new[a.id]!==base.new[a.id]).map(a=>a.id)};
 result[item.name]=changed;
 rows.push("| "+item.name+" | `"+item.path.replace("🧰️framework/","").slice(-60)+"` | "+actions.map(a=>(changed.old.includes(a.id)?"old KEY MOVES":"old same")+" / "+(changed.new.includes(a.id)?"new KEY MOVES":"new same")).join(" | ")+" |");
}
const out=rows.join("\n");console.log(out);
if(process.env.OUT)await Bun.write(process.env.OUT,out+"\n\n"+JSON.stringify({base,result,declared:Object.fromEntries(actions.map(a=>[a.id,{old:a.old.length,new:a.new.length}]))},null,1));
