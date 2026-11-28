import texts from "./📖️texts/🔣️.json" with {type:"json"};
import {existsSync,readdirSync,readFileSync,renameSync,statSync,writeFileSync,mkdirSync} from "node:fs";
import {dirname,join,relative,resolve,sep} from "node:path";

/** 🌐️ One bilingual text; every user-facing line exists in English and German and no language is assumed. */
export type PreflightTextV1=Readonly<{en:string;de:string}>;
/** 🚧️ One typed finding: a stable code, the owning module, the subject path and the fix, in both languages. */
export type PreflightFindingV1=Readonly<{code:string;severity:"refusal"|"warning";owner:string;subject:string;detail:PreflightTextV1;fix:PreflightTextV1;command?:string}>;
/** 📅️ A producer target that must run before the requested target, with the generated path that needs it. */
export type PreflightScheduleV1=Readonly<{target:string;output:string;needed:string;contract:string}>;
/** 📏️ The per-unit size and memory plan; `peakMegabytes` is undefined while the unit has no measurement. */
export type PreflightUnitV1=Readonly<{unit:string;files:number;sourceBytes:number;largestFile:string;largestBytes:number;peakMegabytes?:number;verdict:"fits"|"over-ceiling"|"unmeasured"|"large-source"}>;
/** 🧾️ The whole answer of one preflight run. */
export type PreflightReportV1=Readonly<{version:1;ok:boolean;findings:readonly PreflightFindingV1[];schedule:readonly PreflightScheduleV1[];units:readonly PreflightUnitV1[];millis:Readonly<Record<string,number>>;filesRead:number;truncated:boolean}>;
/** 🔌️ Everything the run needs from outside, injectable so tests run on a scratch tree. */
export type PreflightInputV1=Readonly<{root:string;targets:readonly string[];files?:readonly string[];ceilingMegabytes?:number;largeSourceBytes?:number;peaks?:Readonly<Record<string,number>>;cacheFile?:string;maximumFiles?:number;check?:()=>void;parseToml?:(text:string)=>Record<string,any>}>;
type Owner=Readonly<{name:string;root:string;manifest:string;packages:readonly string[]}>;
type Contract=Readonly<{name:string;ownership:string;ownerPath:string|null;target:string|null;outputRoots:readonly Readonly<{path:string;inclusion:string;producer?:Readonly<{ownerPath:string;target:string}>}>[]}>;
type Ctx={root:string;owners:Map<string,Owner>;byRoot:Owner[];findings:PreflightFindingV1[];schedule:Map<string,PreflightScheduleV1>;contracts:Contract[];toml:(text:string)=>Record<string,any>;check:()=>void;parsed:Map<string,Record<string,any>>;cache:Record<string,any>;dirty:boolean;filesRead:number;maximumFiles:number;truncated:boolean};

const skipDirectories=new Set(["node_modules","target",".git",".cache","🗑️generated","dist"]);
const tsExtensions=[".ts",".tsx",".mts",".mjs",".js",".json",".d.mts"];
const indexNames=["🟦️.ts","index.ts","index.js"];

/** 🗣️ Fills `{name}` placeholders of a bilingual text. */
export function renderPreflightTextV1(text:PreflightTextV1,values:Readonly<Record<string,string>>):PreflightTextV1{
 const fill=(line:string)=>line.replace(/\{(\w+)\}/g,(_,key)=>values[key]??`{${key}}`);return{en:fill(text.en),de:fill(text.de)};
}
function add(ctx:Ctx,code:keyof typeof texts.findings,severity:"refusal"|"warning",owner:string,subject:string,values:Record<string,string>,command?:string){
 const entry=texts.findings[code] as {detail:PreflightTextV1;fix:PreflightTextV1},all={owner,subject,...values};
 ctx.findings.push({code:`preflight.${code}`,severity,owner,subject,detail:renderPreflightTextV1(entry.detail,all),fix:renderPreflightTextV1(entry.fix,all),...(command?{command}:{})});
}
function readToml(ctx:Ctx,path:string):Record<string,any>|undefined{
 let value=ctx.parsed.get(path);if(value)return value;try{value=ctx.toml(readFileSync(path,"utf8"));}catch{return undefined;}ctx.parsed.set(path,value);return value;
}
function ownerOf(ctx:Ctx,path:string):string{
 const rel=relative(ctx.root,path).split(sep).join("/");let best:Owner|undefined;for(const owner of ctx.byRoot)if((rel===owner.root||rel.startsWith(owner.root+"/"))&&(!best||owner.root.length>best.root.length))best=owner;return best?.name??"(no owner)";
}
function* walk(directory:string,limit:{left:number}):Generator<string>{
 let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}
 for(const entry of entries){if(limit.left<=0)return;const path=join(directory,entry.name);if(entry.isDirectory()){if(!skipDirectories.has(entry.name)&&!entry.name.endsWith("dist"))yield*walk(path,limit);}else if(entry.isFile()){limit.left--;yield path;}}
}

/** 🏭️ Maps every generated output root of the taxonomy to the Build target that produces it; an output without its own producer belongs to the producer of its contract. */
export function generatorProducersV1(contracts:readonly Contract[]):Readonly<{root:string;contract:string;target:string|null;ownerPath:string|null}>[]{
 return contracts.flatMap(contract=>contract.outputRoots.map(output=>({root:output.path,contract:contract.name,target:output.producer?.target??contract.target,ownerPath:output.producer?.ownerPath??contract.ownerPath})));
}
function producerFor(ctx:Ctx,path:string){
 const rel=relative(ctx.root,path).split(sep).join("/");let best;for(const row of generatorProducersV1(ctx.contracts))if((rel===row.root||rel.startsWith(row.root+"/"))&&(!best||row.root.length>best.root.length))best=row;return best;
}
function resolveProducer(ctx:Ctx,producer:NonNullable<ReturnType<typeof producerFor>>,subject:string,needed:string,ownerName:string):boolean{
 if(!producer.target||!producer.ownerPath){add(ctx,"generated-no-producer","refusal",ownerName,subject,{contract:producer.contract,needed});return false;}
 const name=producer.target.slice(0,producer.target.lastIndexOf(":")),target=producer.target.slice(producer.target.lastIndexOf(":")+1),owner=ctx.owners.get(name);
 if(!owner||owner.root!==producer.ownerPath&&!owner.root.startsWith(producer.ownerPath)){add(ctx,"generated-producer-unregistered","refusal",ownerName,subject,{contract:producer.contract,target:producer.target,needed});return false;}
 if(owner.manifest.endsWith("project.json")){const project=readJson(join(ctx.root,owner.manifest));if(!project?.targets?.[target]){add(ctx,"generated-producer-target-missing","refusal",owner.name,owner.manifest,{contract:producer.contract,target:producer.target,needed});return false;}}
 if(!ctx.schedule.has(producer.target))ctx.schedule.set(producer.target,{target:producer.target,output:subject,needed,contract:producer.contract});return true;
}
function readJson(path:string):any{try{return JSON.parse(readFileSync(path,"utf8"));}catch{return undefined;}}
function exists(path:string):boolean{return existsSync(path);}

/** 📥️ Extracts import-like specifiers of a TypeScript or JavaScript source without compiling it. */
export function tsSpecifiersV1(source:string):string[]{
 const stripped=source.replace(/\/\*[\s\S]*?\*\//g,"").replace(/(^|\s)\/\/[^\n]*/g,"$1"),out:string[]=[];
 const patterns=[/\b(?:import|export)\b\s*(?:type\b\s*)?(?:[\w*$\s{},]*?\s*from\s*)?["']([^"'\n]+)["']/g,/\bimport\s*\(\s*["']([^"'\n]+)["']/g,/\brequire\s*\(\s*["']([^"'\n]+)["']\s*\)/g];
 for(const pattern of patterns)for(const match of stripped.matchAll(pattern))out.push(match[1]!);return[...new Set(out)];
}
function resolveTs(from:string,specifier:string):string|undefined{
 const base=resolve(dirname(from),specifier.replace(/[?#].*$/,""));if(exists(base)&&statSync(base).isFile())return base;
 for(const extension of tsExtensions)if(exists(base+extension))return base+extension;
 if(exists(base)&&statSync(base).isDirectory())for(const name of indexNames)if(exists(join(base,name)))return join(base,name);
 const stem=base.replace(/\.(js|mjs)$/,"");if(stem!==base)for(const extension of [".ts",".tsx",".mts"])if(exists(stem+extension))return stem+extension;return undefined;
}
function cachedSpecifiers(ctx:Ctx,path:string,parse:(source:string)=>string[]):string[]|undefined{
 let info;try{info=statSync(path);}catch{return undefined;}ctx.filesRead++;
 const entry=ctx.cache[path];if(entry&&entry.m===info.mtimeMs&&entry.s===info.size)return entry.v;
 let source;try{source=readFileSync(path,"utf8");}catch{return undefined;}const value=parse(source);ctx.cache[path]={m:info.mtimeMs,s:info.size,v:value};ctx.dirty=true;return value;
}
function checkTypeScript(ctx:Ctx,entries:readonly string[],ownerName:string){
 const seen=new Set<string>(),queue=[...entries];
 while(queue.length){
  if(seen.size>=ctx.maximumFiles){ctx.truncated=true;return;}
  const file=queue.pop()!;if(seen.has(file))continue;seen.add(file);ctx.check();
  const specs=cachedSpecifiers(ctx,file,tsSpecifiersV1);if(!specs)continue;
  for(const spec of specs){
   if(!spec.startsWith(".")&&!spec.startsWith("/")||/(^|\/)\.semio-/.test(spec))continue;
   const target=resolveTs(file,spec);
   if(target){if(/\.(ts|tsx|mts|mjs|js)$/.test(target)&&!target.includes(`${sep}node_modules${sep}`))queue.push(target);continue;}
   const wanted=resolve(dirname(file),spec.replace(/[?#].*$/,"")),producer=producerFor(ctx,wanted),subject=relative(ctx.root,file).split(sep).join("/"),needed=relative(ctx.root,wanted).split(sep).join("/");
   if(producer){resolveProducer(ctx,producer,subject,needed,ownerOf(ctx,file));continue;}
   add(ctx,"import-unresolved","refusal",ownerOf(ctx,file),subject,{specifier:spec});
  }
 }
}

/** ✂️ Removes `macro_rules!` bodies: their code expands in other modules and crates, so it proves nothing about the file that defines it. */
export function withoutRustMacroBodiesV1(text:string):string{
 let out="",index=0;const start=/macro_rules!\s*\w+\s*([{(\[])/g;
 for(let match=start.exec(text);match;match=start.exec(text)){
  out+=text.slice(index,match.index);const open=match[1]!,close=open==="{"?"}":open==="("?")":"]";let depth=0,i=match.index+match[0].length-1;
  for(;i<text.length;i++){if(text[i]===open)depth++;else if(text[i]===close&&--depth===0)break;}
  index=i+1;start.lastIndex=index;
 }
 return out+text.slice(index);
}
/** 📥️ Extracts `mod x;`, `include!`-family and `#[path]` references of a Rust source without compiling it. */
export function rustReferencesV1(source:string):Readonly<{mods:readonly Readonly<{name:string;path?:string}>[];includes:readonly string[];uses:readonly string[];declared:readonly string[]}>{
 const text=withoutRustMacroBodiesV1(source.replace(/\/\*[\s\S]*?\*\//g,"").replace(/(^|\s)\/\/[^\n]*/g,"$1")),mods:{name:string;path?:string}[]=[],includes:string[]=[],uses:string[]=[],declared:string[]=[];
 for(const m of text.matchAll(/((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_]\w*)\s*;/g)){const path=/#\[\s*path\s*=\s*"([^"]+)"\s*\]/.exec(m[1]??"")?.[1];mods.push({name:m[2]!,...(path?{path}:{})});}
 for(const m of text.matchAll(/\binclude(?:_str|_bytes)?!\s*\(\s*"([^"]+)"\s*\)/g))includes.push(m[1]!);
 for(const m of text.matchAll(/^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_]\w*)/gm))uses.push(m[1]!);
 for(const m of text.matchAll(/\b(?:mod|struct|enum|trait|type|fn|const|static|union|macro_rules!)\s+([A-Za-z_]\w*)/g))declared.push(m[1]!);
 for(const m of text.matchAll(/\bextern\s+crate\s+([A-Za-z_]\w*)(?:\s+as\s+([A-Za-z_]\w*))?/g))declared.push(m[2]??m[1]!);
 for(const m of text.matchAll(/\buse\s+([^;]*);/g)){const names=m[1]!.match(/[A-Za-z_]\w*/g)??[];declared.push(...names.slice(1));}
 return{mods,includes,uses:[...new Set(uses)],declared:[...new Set(declared)]};
}
type RustCrate={name:string;dir:string;manifest:string;entries:string[];dependencies:Map<string,string>;table:Record<string,any>};
function crateEntries(table:Record<string,any>,dir:string):string[]{
 const entries:string[]=[];const lib=table.lib?.path??"src/lib.rs";if(exists(join(dir,lib)))entries.push(join(dir,lib));
 for(const key of ["bin","test","example","bench"])for(const item of table[key]??[])if(item.path&&exists(join(dir,item.path)))entries.push(join(dir,item.path));
 const main=join(dir,"src/main.rs");if(exists(main))entries.push(main);const build=table.package?.build;if(typeof build==="string"&&exists(join(dir,build)))entries.push(join(dir,build));else if(build!==false&&exists(join(dir,"build.rs")))entries.push(join(dir,"build.rs"));
 return[...new Set(entries)];
}
function dependencyTables(table:Record<string,any>):Record<string,any>[]{
 const tables=[table.dependencies,table["dev-dependencies"],table["build-dependencies"]];for(const target of Object.values(table.target??{}) as any[])tables.push(target.dependencies,target["dev-dependencies"],target["build-dependencies"]);return tables.filter(Boolean);
}
function externName(ctx:Ctx,dir:string,manifest:string,key:string,value:any):string{
 if(value&&typeof value==="object"&&value.package)return key.replace(/-/g,"_");
 let path=value&&typeof value==="object"?value.path:undefined;
 if(value&&typeof value==="object"&&value.workspace){const workspace=findWorkspace(ctx,manifest);const inherited=workspace?readToml(ctx,workspace)?.workspace?.dependencies?.[key]:undefined;if(inherited&&typeof inherited==="object"&&inherited.path&&workspace)path=relative(dir,resolve(dirname(workspace),inherited.path));}
 if(path){const dependency=readToml(ctx,join(resolve(dir,path),"Cargo.toml"));if(dependency?.lib?.name)return String(dependency.lib.name).replace(/-/g,"_");}
 return key.replace(/-/g,"_");
}
function loadCrate(ctx:Ctx,manifest:string):RustCrate|undefined{
 const table=readToml(ctx,manifest);if(!table?.package?.name)return undefined;const dependencies=new Map<string,string>(),dir=dirname(manifest);
 for(const set of dependencyTables(table))for(const [key,value] of Object.entries(set))dependencies.set(externName(ctx,dir,manifest,key,value),String((value as any)?.package??key));
 const name=table.package.name as string,libName=(table.lib?.name??name).replace(/-/g,"_");return{name,dir,manifest,entries:crateEntries(table,dir),dependencies,table:{...table,libName}};
}
function checkRustModules(ctx:Ctx,crate:RustCrate,workspaceCrates:Map<string,string>,ownerName:string){
 const seen=new Set<string>(),declaredAnywhere=new Set<string>(),usedBy:{use:string;subject:string}[]=[],queue=crate.entries.map(file=>({file,dir:dirname(file),inline:true}));
 while(queue.length){
  if(seen.size>=ctx.maximumFiles){ctx.truncated=true;return;}
  const item=queue.pop()!;if(seen.has(item.file))continue;seen.add(item.file);ctx.check();
  const refs=cachedSpecifiers(ctx,item.file,source=>{const r=rustReferencesV1(source);return[JSON.stringify(r)];});if(!refs)continue;
  const parsed=JSON.parse(refs[0]!) as ReturnType<typeof rustReferencesV1>,subject=relative(ctx.root,item.file).split(sep).join("/");
  for(const mod of parsed.mods){
   const candidates=mod.path?[resolve(dirname(item.file),mod.path)]:[join(item.dir,mod.name+".rs"),join(item.dir,mod.name,"mod.rs")];
   const found=candidates.find(candidate=>exists(candidate));
   if(found){queue.push({file:found,dir:mod.path?dirname(found):found.endsWith("mod.rs")?dirname(found):join(item.dir,mod.name),inline:false});continue;}
   const producer=producerFor(ctx,candidates[0]!);if(producer){resolveProducer(ctx,producer,subject,relative(ctx.root,candidates[0]!).split(sep).join("/"),ownerName);continue;}
   add(ctx,"rust-mod-unresolved","refusal",ownerName,subject,{module:mod.name});
  }
  for(const include of parsed.includes){
   const wanted=resolve(dirname(item.file),include);if(exists(wanted))continue;
   const producer=producerFor(ctx,wanted);if(producer){resolveProducer(ctx,producer,subject,relative(ctx.root,wanted).split(sep).join("/"),ownerName);continue;}
   if(/\bOUT_DIR\b/.test(include))continue;add(ctx,"rust-include-unresolved","refusal",ownerName,subject,{path:include});
  }
  for(const name of parsed.declared)declaredAnywhere.add(name);for(const use of parsed.uses)usedBy.push({use,subject});
 }
 for(const {use,subject} of usedBy){
  if(declaredAnywhere.has(use)||crate.dependencies.has(use)||use===crate.table.libName)continue;
  if(workspaceCrates.has(use))add(ctx,"rust-crate-undeclared","refusal",ownerName,subject,{crate:workspaceCrates.get(use)!,manifest:relative(ctx.root,crate.manifest).split(sep).join("/")});
 }
}

function lockPackages(text:string):Map<string,{version:string;source:boolean;dependencies:string[]}[]>{
 const packages=new Map<string,{version:string;source:boolean;dependencies:string[]}[]>(),blocks=text.split(/^\[\[package\]\]\s*$/m);
 for(let i=1;i<blocks.length;i++){
  const block=blocks[i]!,name=/^name = "([^"]+)"/m.exec(block)?.[1],version=/^version = "([^"]*)"/m.exec(block)?.[1]??"";if(!name)continue;
  const dependencies:string[]=[],list=/^dependencies = \[([\s\S]*?)^\]/m.exec(block);if(list)for(const m of list[1]!.matchAll(/"([^"]+)"/g))dependencies.push(m[1]!);
  const row=packages.get(name)??[];row.push({version,source:/^source = /m.test(block),dependencies});packages.set(name,row);
 }
 return packages;
}
function expandMembers(workspaceDir:string,patterns:readonly string[]):string[]{
 const out:string[]=[];
 for(const pattern of patterns){
  if(!pattern.includes("*")){out.push(join(workspaceDir,pattern));continue;}
  const parts=pattern.split("/"),star=parts.indexOf(parts.find(part=>part.includes("*"))!),base=join(workspaceDir,...parts.slice(0,star)),rest=parts.slice(star+1);
  let entries:string[]=[];try{entries=readdirSync(base);}catch{}
  for(const entry of entries){const candidate=join(base,entry,...rest);if(exists(join(candidate,"Cargo.toml")))out.push(candidate);}
 }
 return out;
}
function findWorkspace(ctx:Ctx,manifest:string):string|undefined{
 const table=readToml(ctx,manifest);const explicit=table?.package?.workspace;if(typeof explicit==="string"){const candidate=resolve(dirname(manifest),explicit,"Cargo.toml");return exists(candidate)?candidate:undefined;}
 if(table?.workspace)return manifest;let directory=dirname(dirname(manifest));
 while(directory.startsWith(ctx.root)){const candidate=join(directory,"Cargo.toml");if(exists(candidate)&&readToml(ctx,candidate)?.workspace)return candidate;const parent=dirname(directory);if(parent===directory)break;directory=parent;}return undefined;
}
function satisfies(version:string,requirement:string):boolean{
 const req=requirement.trim();if(req===""||req==="*")return true;const v=version.split(/[-+]/)[0]!.split(".").map(Number);
 return req.split(",").every(part=>{
  const m=/^(\^|~|=|>=|<=|>|<)?\s*(\d+)(?:\.(\d+|\*))?(?:\.(\d+|\*))?/.exec(part.trim());if(!m)return true;
  const op=m[1]??"^",r=[Number(m[2]),m[3]===undefined||m[3]==="*"?undefined:Number(m[3]),m[4]===undefined||m[4]==="*"?undefined:Number(m[4])];
  const cmp=()=>{for(let i=0;i<3;i++){if(r[i]===undefined)return 0;if((v[i]??0)!==r[i])return(v[i]??0)<r[i]!?-1:1;}return 0;};
  if(op==="=")return cmp()===0;if(op===">=")return cmp()>=0;if(op==="<=")return cmp()<=0;if(op===">")return cmp()>0;if(op==="<")return cmp()<0;
  if(cmp()<0)return false;if(op==="~")return r[1]===undefined?v[0]===r[0]:v[0]===r[0]&&v[1]===r[1];
  if(r[0]!==0)return v[0]===r[0];if(r[1]===undefined)return v[0]===0;if(r[1]!==0)return v[0]===0&&v[1]===r[1];return v[0]===0&&v[1]===0&&(r[2]===undefined||v[2]===r[2]);
 });
}

/** 🔒️ Checks one Cargo workspace against its lock without a network or a cargo process: membership, member versions, declared dependencies against the lock and the lock against the declared dependencies. */
export function checkCargoWorkspaceV1(ctx:Ctx,workspaceManifest:string,focus:ReadonlySet<string>){
 const table=readToml(ctx,workspaceManifest)!,dir=dirname(workspaceManifest),relManifest=relative(ctx.root,workspaceManifest).split(sep).join("/"),lockPath=join(dir,"Cargo.lock"),relLock=relative(ctx.root,lockPath).split(sep).join("/"),owner=ownerOf(ctx,workspaceManifest);
 const command=`repo cargo generate-lockfile --manifest-path ${relManifest}`;
 if(!exists(lockPath)){add(ctx,"lock-missing","refusal",owner,relLock,{manifest:relManifest},command);return;}
 const members=expandMembers(dir,table.workspace?.members??[]).map(path=>join(path,"Cargo.toml")),excluded=new Set<string>((table.workspace?.exclude??[]).map((entry:string)=>join(dir,entry,"Cargo.toml")));
 if(table.package)members.unshift(workspaceManifest);
 const lock=lockPackages(readFileSync(lockPath,"utf8")),inherited=table.workspace?.dependencies??{},crates:RustCrate[]=[];
 for(const manifest of members){
  if(excluded.has(manifest))continue;
  if(!exists(manifest)){add(ctx,"manifest-missing","refusal",owner,relative(ctx.root,manifest).split(sep).join("/"),{workspace:relManifest});continue;}
  const crate=loadCrate(ctx,manifest);if(!crate){add(ctx,"manifest-invalid","refusal",owner,relative(ctx.root,manifest).split(sep).join("/"),{});continue;}crates.push(crate);
 }
 const memberDirs=new Set(crates.map(crate=>crate.dir));
 for(const crate of crates){
  const subject=relative(ctx.root,crate.manifest).split(sep).join("/"),version=crate.table.package.version===undefined||typeof crate.table.package.version==="object"?table.workspace?.package?.version:crate.table.package.version;
  const entries=lock.get(crate.name)?.filter(row=>!row.source),entry=entries?.[0];
  if(focus.size&&!focus.has(crate.name)&&!focus.has("*"))continue;
  if(!entry){add(ctx,"lock-stale","refusal",ownerOf(ctx,crate.manifest),relLock,{crate:crate.name,why:"member missing from the lock",manifest:subject},command);continue;}
  if(version&&entry.version!==version){add(ctx,"lock-stale","refusal",ownerOf(ctx,crate.manifest),relLock,{crate:crate.name,why:`lock has version ${entry.version}, manifest ${version}`,manifest:subject},command);continue;}
  const declared=new Map<string,string>();
  for(const set of dependencyTables(crate.table))for(const [key,raw] of Object.entries(set) as [string,any][]){
   const spec=raw?.workspace?{...(typeof inherited[key]==="object"?inherited[key]:{version:inherited[key]}),...raw}:raw,name=spec?.package??key,requirement=typeof spec==="string"?spec:spec?.version??"",path=typeof spec==="object"?spec.path:undefined;
   if(!declared.has(name)||requirement)declared.set(name,path?"path:"+path:requirement);
  }
  const locked=new Map<string,string[]>();for(const text of entry.dependencies){const parts=text.split(" ");locked.set(parts[0]!,[...(locked.get(parts[0]!)??[]),text]);}
  for(const [name,requirement] of declared){
   if(!locked.has(name)){add(ctx,"lock-stale","refusal",ownerOf(ctx,crate.manifest),relLock,{crate:crate.name,why:`dependency ${name} is declared but not in the lock`,manifest:subject},command);break;}
   if(requirement&&!requirement.startsWith("path:")){const rows=lock.get(name)??[];if(rows.length&&!rows.some(row=>row.source&&satisfies(row.version,requirement))&&!rows.some(row=>!row.source)){add(ctx,"lock-stale","refusal",ownerOf(ctx,crate.manifest),relLock,{crate:crate.name,why:`locked ${name} ${rows.map(row=>row.version).join("/")} does not satisfy ${requirement}`,manifest:subject},command);break;}}
  }
  const extra=[...locked.keys()].find(name=>!declared.has(name));if(extra)add(ctx,"lock-stale","refusal",ownerOf(ctx,crate.manifest),relLock,{crate:crate.name,why:`dependency ${extra} is locked but no longer declared`,manifest:subject},command);
  for(const set of dependencyTables(crate.table))for(const [key,raw] of Object.entries(set) as [string,any][]){
   const path=raw?.path;if(!path)continue;const target=resolve(crate.dir,path);
   if(!memberDirs.has(target)&&exists(join(target,"Cargo.toml"))&&target.startsWith(dir)&&!excluded.has(join(target,"Cargo.toml")))add(ctx,"manifest-not-member","refusal",ownerOf(ctx,crate.manifest),relative(ctx.root,join(target,"Cargo.toml")).split(sep).join("/"),{dependent:crate.name,workspace:relManifest});
  }
 }
}

function resolveTarget(ctx:Ctx,spec:string):Owner|undefined{
 const name=spec.includes(":")?spec.slice(0,spec.lastIndexOf(":")):spec;return ctx.owners.get(name)??ctx.byRoot.find(owner=>owner.root===spec);
}
function rustPathDependencies(ctx:Ctx,crate:RustCrate,limit:number):RustCrate[]{
 const seen=new Map<string,RustCrate>(),queue=[crate];seen.set(crate.manifest,crate);
 while(queue.length&&seen.size<limit){
  const current=queue.pop()!;
  for(const set of dependencyTables(current.table))for(const raw of Object.values(set) as any[]){
   if(!raw?.path)continue;const manifest=join(resolve(current.dir,raw.path),"Cargo.toml");if(seen.has(manifest)||!exists(manifest))continue;const next=loadCrate(ctx,manifest);if(!next)continue;seen.set(manifest,next);queue.push(next);
  }
 }
 return[...seen.values()];
}
function unitPlan(ctx:Ctx,unit:string,dir:string,peaks:Readonly<Record<string,number>>|undefined,ceiling:number,large:number):PreflightUnitV1{
 const limit={left:20000};let files=0,bytes=0,largest="",largestBytes=0;
 for(const file of walk(dir,limit)){if(!/\.(rs|ts|tsx|mjs|js|go|py|cpp|cs|tex)$/.test(file))continue;let size=0;try{size=statSync(file).size;}catch{continue;}files++;bytes+=size;if(size>largestBytes){largest=relative(ctx.root,file).split(sep).join("/");largestBytes=size;}}
 const peak=peaks?.[unit]??peaks?.[unit.replace(/-/g,"_")];let verdict:PreflightUnitV1["verdict"]="unmeasured";
 if(peak!==undefined)verdict=peak>ceiling?"over-ceiling":"fits";else if(largestBytes>large)verdict="large-source";
 return{unit,files,sourceBytes:bytes,largestFile:largest,largestBytes,...(peak!==undefined?{peakMegabytes:peak}:{}),verdict};
}

/** 🛫️ Runs every cheap static check the named targets need before any compile starts: generated-file producers, module and import resolution, Cargo manifests and lock, and the per-unit size and memory plan. Reads files only and starts no process. */
export async function preflightTargetsV1(input:PreflightInputV1):Promise<PreflightReportV1>{
 const millis:Record<string,number>={},lap=(name:string,start:number)=>{millis[name]=Math.round(performance.now()-start);},started=performance.now();
 const root=resolve(input.root),registry=readJson(join(root,"📋️build-owners.json")),owners=new Map<string,Owner>(),all:Owner[]=(registry?.owners??[]);for(const owner of all)owners.set(owner.name,owner);
 const taxonomy=readJson(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")),contracts:Contract[]=Object.entries(taxonomy?.generatorContracts??{}).map(([name,value]:[string,any])=>({name,ownership:value.ownership,ownerPath:value.ownerPath,target:value.target,outputRoots:value.outputRoots}));
 let cache:Record<string,any>={};if(input.cacheFile){const stored=readJson(input.cacheFile);if(stored?.version===1)cache=stored.rows??{};}
 const toml=input.parseToml??((text:string)=>(globalThis as any).Bun.TOML.parse(text));
 const ctx:Ctx={root,owners,byRoot:all,findings:[],schedule:new Map(),contracts,toml,check:input.check??(()=>{}),parsed:new Map(),cache,dirty:false,filesRead:0,maximumFiles:input.maximumFiles??20000,truncated:false};
 lap("load",started);
 const units:PreflightUnitV1[]=[],workspaces=new Map<string,Set<string>>(),workspaceCrates=new Map<string,string>();
 const resolved:{spec:string;owner:Owner}[]=[];
 for(const spec of input.targets){const owner=resolveTarget(ctx,spec);if(!owner){add(ctx,"target-unknown","refusal","(repo)",spec,{target:spec});continue;}resolved.push({spec,owner});}
 let step=performance.now();
 for(const {owner} of resolved){
  const dir=join(root,owner.root),manifestPath=join(dir,"Cargo.toml");
  if(exists(manifestPath)){
   const crate=loadCrate(ctx,manifestPath);if(!crate){add(ctx,"manifest-invalid","refusal",owner.name,relative(root,manifestPath).split(sep).join("/"),{});continue;}
   const workspace=findWorkspace(ctx,manifestPath);
   if(!workspace)add(ctx,"manifest-no-workspace","refusal",owner.name,relative(root,manifestPath).split(sep).join("/"),{});
   else{const set=workspaces.get(workspace)??new Set<string>();set.add(crate.name);workspaces.set(workspace,set);}
   for(const member of rustPathDependencies(ctx,crate,200))workspaceCrates.set(member.table.libName,member.name);
  }
 }
 lap("manifests",step);step=performance.now();
 for(const [workspace,focus] of workspaces)checkCargoWorkspaceV1(ctx,workspace,focus);
 lap("lock",step);step=performance.now();
 for(const {owner} of resolved){
  const dir=join(root,owner.root),manifestPath=join(dir,"Cargo.toml");
  if(exists(manifestPath)){const crate=loadCrate(ctx,manifestPath);if(crate){for(const member of rustPathDependencies(ctx,crate,200))checkRustModules(ctx,member,workspaceCrates,ownerOf(ctx,member.manifest));}}
  const entries:string[]=[];
  if(owner.manifest.endsWith("project.json")||exists(join(dir,"package.json"))){const limit={left:20000};for(const file of walk(dir,limit))if(/\.(ts|tsx|mts|mjs)$/.test(file)&&!file.endsWith(".d.ts"))entries.push(file);}
  for(const file of input.files??[]){const absolute=resolve(root,file);if(/\.(ts|tsx|mts|mjs)$/.test(absolute)&&absolute.startsWith(dir))entries.push(absolute);}
  if(entries.length)checkTypeScript(ctx,entries,owner.name);
 }
 lap("modules",step);step=performance.now();
 for(const {owner} of resolved)units.push(unitPlan(ctx,owner.name,join(root,owner.root),input.peaks,input.ceilingMegabytes??1536,input.largeSourceBytes??1048576));
 for(const unit of units){
  if(unit.verdict==="over-ceiling")add(ctx,"memory-over-ceiling","refusal",unit.unit,unit.unit,{peak:String(unit.peakMegabytes),ceiling:String(input.ceilingMegabytes??1536)});
  else if(unit.verdict==="large-source")add(ctx,"source-large","warning",unit.unit,unit.largestFile,{bytes:String(unit.largestBytes)});
 }
 lap("units",step);
 if(input.cacheFile&&ctx.dirty){try{mkdirSync(dirname(input.cacheFile),{recursive:true});const temporary=`${input.cacheFile}.${process.pid}.tmp`;writeFileSync(temporary,JSON.stringify({version:1,rows:ctx.cache}));renameSync(temporary,input.cacheFile);}catch{}}
 const seen=new Set<string>(),findings=ctx.findings.filter(finding=>{const key=finding.code+"\0"+finding.subject+"\0"+finding.detail.en;if(seen.has(key))return false;seen.add(key);return true;});
 lap("total",started);
 return{version:1,ok:!findings.some(finding=>finding.severity==="refusal"),findings,schedule:[...ctx.schedule.values()],units,millis,filesRead:ctx.filesRead,truncated:ctx.truncated};
}

/** 📊️ Reads the measured peak of every compiler unit from the memory files the wardens of the fleet publish (`origins/<origin>/memory.json`, `lastAlarms`), keyed by crate name with underscores; the largest peak wins. Pass the result as `peaks`; a richer per-unit table (H90) replaces it without changing the report. */
export function unitPeaksFromWardensV1(storeRoot:string):Record<string,number>{
 const peaks:Record<string,number>={};let origins:string[]=[];try{origins=readdirSync(join(storeRoot,"origins"));}catch{return peaks;}
 for(const origin of origins){const memory=readJson(join(storeRoot,"origins",origin,"memory.json"));for(const alarm of memory?.lastAlarms??[]){const name=/--crate-name\s+([A-Za-z0-9_]+)/.exec(String(alarm.owner??""))?.[1];if(name&&typeof alarm.megabytes==="number")peaks[name]=Math.max(peaks[name]??0,alarm.megabytes);}}
 return peaks;
}
/** 🖨️ Prints a report in exactly one language, one finding per block with its fix and command; no language is assumed. */
export function formatPreflightV1(report:PreflightReportV1,language:"en"|"de"):string{
 const label=(key:keyof typeof texts.labels)=>texts.labels[key][language],lines=[`${label(report.ok?"ok":"failed")} (${report.millis.total??0} ms)`];
 for(const row of report.schedule)lines.push(`${label("schedule")}: ${row.target} <- ${row.needed}`);
 for(const finding of report.findings){lines.push(`${label(finding.severity)} ${finding.code}`,`  ${finding.detail[language]}`,`  -> ${finding.fix[language]}`);if(finding.command)lines.push(`  $ ${finding.command}`);}
 return lines.join("\n");
}
