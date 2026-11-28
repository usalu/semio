import {spawn} from "node:child_process";
import {constants,copyFileSync,cpSync,existsSync,mkdirSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";
import {dirname,isAbsolute,join,normalize,relative,resolve} from "node:path";
import {readTypecheckPolicyV1,resolveTypecheckPathsV1,renderTypecheckResultV1,runTypecheckV1,typecheckCacheDirectoryV1,typecheckImportersV1,type TypecheckPolicyV1} from "../../🔬️typecheck/🟦️.ts";
import {readLandPolicyV1,type LandChangesetV1,type LandPolicyV1} from "../🟦️.ts";
import {landTextV1,refuseLandV1,type LandLanguageV1,type LandTextKeyV1} from "../📖️texts/🟦️.ts";

/** 🚦️ The checks of a landing in the order they run. */
export type LandGateIdV1="debug-lines"|"dangling-imports"|"typecheck"|"rust"|"suite-law"|"tests"|"bench";
/** 🧾️ The verdict of one check. */
export type LandGateRowV1=Readonly<{id:LandGateIdV1;status:"passed"|"failed"|"skipped";seconds:number;detail:readonly string[]}>;
/** 🧪️ The outcome of a test run: the lines to show and, per failing file, a signature of its failure (timings and roots removed) used to tell a pre-existing failure from a new one. */
export type LandTestOutcomeV1=Readonly<{passed:boolean;lines:readonly string[];failures?:readonly Readonly<{file:string;signature:string;bound?:boolean}>[]}>;

/** 🔌️ The seams of the checks; tests replace them, the defaults run the real tools against the overlay. */
export type LandGatePortsV1=Readonly<{
 clone?:(source:string,target:string,signal?:AbortSignal)=>Promise<void>;
 typecheck?:(overlay:string,paths:readonly string[],language:LandLanguageV1,signal?:AbortSignal)=>Promise<Readonly<{passed:boolean;lines:readonly string[]}>>;
 rust?:(request:Readonly<{overlay:string;crates:readonly string[];signal?:AbortSignal}>)=>Promise<Readonly<{passed:boolean;lines:readonly string[]}>>;
 tests?:(request:Readonly<{overlay:string;files:readonly string[];signal?:AbortSignal}>)=>Promise<LandTestOutcomeV1>;
 bench?:(request:Readonly<{root:string;signal?:AbortSignal}>)=>Promise<Readonly<{passed:boolean;lines:readonly string[]}>>;
 deferred?:string[];notice?:(line:string)=>void;now?:()=>number;signal?:AbortSignal;environment?:NodeJS.ProcessEnv;
}>;

const EXTENSIONS=[".ts",".tsx",".mts",".cts",".mjs",".cjs",".js"],SKIPPED=new Set(["node_modules","dist","target",".cache","🤖️generated","🗑️generated"]);
const REGEX_PRECEDERS=new Set([..."(,=:[!&|?{};+-*%<>~^","return","typeof","case","do","else","in","of","void","delete","throw","new","yield","await"]);
type TokenV1=readonly ["str"|"word"|"punct"|"num",string];
const isWordStart=(c:string):boolean=>/[\p{L}_$]/u.test(c)||c.charCodeAt(0)>127,isWordPart=(c:string):boolean=>/[\p{L}\p{N}_$]/u.test(c)||c.charCodeAt(0)>127;

/** 🔤️ Tokenizes a TypeScript or JavaScript source into words, punctuation and string literals; comments, template text and regular expressions are skipped, so fixtures and `toContain("import …")` never count as imports. */
export function landScanV1(text:string):readonly TokenV1[]{
 const n=text.length,out:TokenV1[]=[],braces:string[]=[];let i=0;
 const template=(resume=false):"expr"|undefined=>{
  if(!resume)i++;
  while(i<n){const c=text[i]!;if(c==="\\")i+=2;else if(c==="`"){i++;return undefined;}else if(c==="$"&&text[i+1]==="{"){i+=2;braces.push("t");return "expr";}else i++;}
  return undefined;
 };
 const regexAllowed=():boolean=>{if(!out.length)return true;const [kind,value]=out[out.length-1]!;if(kind==="str"||kind==="num")return false;if(kind==="word")return REGEX_PRECEDERS.has(value);return ![")","]","}"].includes(value);};
 while(i<n){
  const c=text[i]!;
  if(" \t\r\n".includes(c))i++;
  else if(text.startsWith("//",i)){const j=text.indexOf("\n",i);i=j<0?n:j;}
  else if(text.startsWith("/*",i)){const j=text.indexOf("*/",i+2);i=j<0?n:j+2;}
  else if(c==="\""||c==="'"){let j=i+1;while(j<n&&text[j]!==c&&text[j]!=="\n")j+=text[j]==="\\"?2:1;out.push(["str",decode(text.slice(i+1,j))]);i=j+1;}
  else if(c==="`"){if(template()==="expr")out.push(["punct","${"]);}
  else if(c==="{"){braces.push("b");out.push(["punct",c]);i++;}
  else if(c==="}"){if(braces.length&&braces.pop()==="t"){out.push(["punct","}"]);i++;if(template(true)==="expr")out.push(["punct","${"]);}else{out.push(["punct",c]);i++;}}
  else if(c==="/"&&regexAllowed()){let j=i+1,inClass=false;while(j<n&&text[j]!=="\n"){if(text[j]==="\\"){j+=2;continue;}if(text[j]==="[")inClass=true;else if(text[j]==="]")inClass=false;else if(text[j]==="/"&&!inClass)break;j++;}i=j+1;while(i<n&&/\p{L}/u.test(text[i]!))i++;out.push(["num","regex"]);}
  else if(isWordStart(c)){let j=i;while(j<n&&isWordPart(text[j]!))j++;out.push(["word",text.slice(i,j)]);i=j;}
  else if(c>="0"&&c<="9"){let j=i;while(j<n&&(/[\p{L}\p{N}]/u.test(text[j]!)||text[j]==="."||text[j]==="_"))j++;out.push(["num",text.slice(i,j)]);i=j;}
  else{out.push(["punct",c]);i++;}
 }
 return out;
}
const decode=(value:string):string=>value.includes("\\")?value.replace(/\\u\{([0-9a-fA-F]+)\}|\\u([0-9a-fA-F]{4})|\\x([0-9a-fA-F]{2})|\\([\s\S])/gu,(_all,brace,unit,byte,char)=>{const code=brace??unit??byte;return code?String.fromCodePoint(parseInt(code,16)):char==="n"?"\n":char==="t"?"\t":char;}):value;

/** 🔗️ The relative module specifiers a source really imports (static import, export-from, dynamic `import()`, `require()`). */
export function landSpecifiersV1(text:string):readonly string[]{
 const tokens=landScanV1(text),found:string[]=[];
 tokens.forEach(([kind,value],index)=>{
  if(kind!=="str"||!(value.startsWith("./")||value.startsWith("../")))return;
  const before=tokens.slice(Math.max(0,index-3),index).map(row=>row[1]),last=before[before.length-1];
  if(last==="from"||last==="import"||(before[before.length-2]==="import"&&last==="(")||(before[before.length-2]==="require"&&last==="(")){if(before[before.length-2]===".")return;found.push(value);}
 });
 return found;
}

const resolves=(directory:string,specifier:string):boolean=>{
 const base=normalize(join(directory,specifier.split("?")[0]!.split("#")[0]!));
 if(existsSync(base))return true;
 const extension=/\.[^./]+$/u.exec(base)?.[0]??"",stem=extension?base.slice(0,-extension.length):base,swaps:Record<string,readonly string[]>={".js":[".ts",".tsx",".mts"],".mjs":[".mts",".ts"],".cjs":[".cts"],".jsx":[".tsx"]};
 if((swaps[extension]??[]).some(swap=>existsSync(stem+swap)))return true;
 return [...EXTENSIONS,".json",".d.ts",".d.mts"].some(suffix=>existsSync(base+suffix)||existsSync(join(base,"index"+suffix)));
};

/** 🔎️ Lists the relative imports of the given files (checkout-relative) that resolve to no existing file under the root. */
export function landDanglingImportsV1(root:string,files:readonly string[]):readonly Readonly<{file:string;specifier:string}>[]{
 const rows:{file:string;specifier:string}[]=[];
 for(const file of files){
  const path=resolve(root,file);
  if(!existsSync(path)||!EXTENSIONS.some(extension=>file.endsWith(extension))||file.endsWith(".d.ts"))continue;
  for(const specifier of landSpecifiersV1(readFileSync(path,"utf8")))if(!resolves(dirname(path),specifier))rows.push({file,specifier});
 }
 return Object.freeze(rows);
}

const spawnWait=(command:string,args:readonly string[],options:Readonly<{cwd?:string;signal?:AbortSignal;environment?:NodeJS.ProcessEnv}>):Promise<Readonly<{status:number|null;output:string}>>=>new Promise((finish,fail)=>{
 const child=spawn(command,[...args],{cwd:options.cwd,stdio:["ignore","pipe","pipe"],signal:options.signal,env:options.environment??process.env,windowsHide:true});
 let output="";
 child.stdout.on("data",chunk=>{output+=chunk;});child.stderr.on("data",chunk=>{output+=chunk;});
 child.on("error",fail);child.on("close",status=>finish({status,output}));
});

/** 🪞️ Clones a tree copy-on-write where the file system can (APFS `cp -c`, reflink `cp`), else copies it. */
export async function landCloneTreeV1(source:string,target:string,signal?:AbortSignal,platform:string=process.platform):Promise<void>{
 mkdirSync(dirname(target),{recursive:true});
 try{
  const run=platform==="darwin"?await spawnWait("cp",["-cR",source,target],{signal}):platform==="linux"?await spawnWait("cp",["-a","--reflink=auto",source,target],{signal}):{status:1,output:""};
  if(run.status===0)return;
 }catch(error){if(signal?.aborted)throw error;}
 rmSync(target,{recursive:true,force:true});
 cpSync(source,target,{recursive:true,mode:constants.COPYFILE_FICLONE,verbatimSymlinks:true});
}

/** 🗂️ The folder of the scratch overlay of one lane below the short overlay base: stable per checkout and lane so the compiler's incremental state survives between runs. */
export const landOverlayRootV1=(overlayBase:string,lane:string):string=>join(overlayBase,lane.replace(/[^A-Za-z0-9._-]+/gu,"-").replace(/^[^A-Za-z0-9]+/u,"")||"lane");

/** 🔗️ Binds the dependencies of an overlay to the overlay itself: `node_modules` becomes a real folder (an unmanaged namespace, which the launcher accepts for any root, where a link to the managed generation names only the live checkout as its Source) whose entries are the links of the live generation, except the workspace packages, which point to their folder inside the overlay, so a module imported by package name is the overlay's copy and never the live one. */
export function landBindDependenciesV1(root:string,overlay:string,limits:LandPolicyV1=readLandPolicyV1()):void{
 const realRoot=realpathSync(root),source=realpathSync(join(root,"node_modules")),target=join(overlay,"node_modules"),inside=(path:string):string|undefined=>{
  for(const name of limits.overlay.roots){const folder=join(realRoot,name),offset=relative(folder,path);if(offset===""||(!offset.startsWith("..")&&!isAbsolute(offset))){const mapped=join(overlay,name,offset);if(existsSync(mapped))return mapped;}}
  return undefined;
 },bind=(from:string,to:string):void=>{
  const real=realpathSync(from);symlinkSync(inside(real)??real,to,"junction");
 };
 mkdirSync(target,{recursive:true});
 for(const entry of readdirSync(source,{withFileTypes:true})){
  const from=join(source,entry.name),to=join(target,entry.name);
  if(entry.name.startsWith("@")&&statSync(from).isDirectory()){
   mkdirSync(to,{recursive:true});
   for(const child of readdirSync(from))try{bind(join(from,child),join(to,child));}catch{}
  }else try{bind(from,to);}catch{}
 }
}

/** 🪞️ Mirrors the first `depth` levels of a live folder as real folders whose other entries are links to the live ones: the storage laws refuse a symbolic link among the ancestors of a cache partition. */
export function landMirrorDirectoriesV1(source:string,target:string,depth:number):void{
 mkdirSync(target,{recursive:true});
 for(const entry of readdirSync(source,{withFileTypes:true})){
  const from=join(source,entry.name),to=join(target,entry.name);
  if(depth>1&&entry.isDirectory())landMirrorDirectoriesV1(from,to,depth-1);else symlinkSync(from,to,"junction");
 }
}

/** 🔗️ Links what an overlay (or a source snapshot) shares with the live tree: the root files it lacks, the dependency folder bound to the overlay, the mirrored cache folders, the heavy members of the Rust roots and the other declared links; whatever already exists is kept, so the call is repeatable. */
export async function landLinkWorkspaceV1(root:string,overlay:string,clone:(source:string,target:string,signal?:AbortSignal)=>Promise<void>=landCloneTreeV1,signal?:AbortSignal,limits:LandPolicyV1=readLandPolicyV1()):Promise<void>{
 const rootFiles=limits.overlay.rootFiles.includes("*")?readdirSync(root,{withFileTypes:true}).filter(entry=>entry.isFile()).map(entry=>entry.name):limits.overlay.rootFiles;
 for(const name of rootFiles)if(existsSync(join(root,name))&&!existsSync(join(overlay,name)))copyFileSync(join(root,name),join(overlay,name),constants.COPYFILE_FICLONE);
 for(const name of limits.overlay.links)if(existsSync(join(root,name))&&!existsSync(join(overlay,name))){
  if(name==="node_modules")landBindDependenciesV1(root,overlay,limits);
  else if((limits.overlay.mirrors as Record<string,number>)[name]!==undefined)landMirrorDirectoriesV1(realpathSync(join(root,name)),join(overlay,name),(limits.overlay.mirrors as Record<string,number>)[name]!);
  else if(limits.overlay.rustRoots.includes(name)&&statSync(join(root,name)).isDirectory()){mkdirSync(join(overlay,name));for(const child of readdirSync(join(root,name))){const from=join(realpathSync(join(root,name)),child);if(limits.overlay.heavyMembers.includes(child))symlinkSync(from,join(overlay,name,child),"junction");else if(statSync(from).isDirectory())await clone(from,join(overlay,name,child),signal);else copyFileSync(from,join(overlay,name,child),constants.COPYFILE_FICLONE);}}
  else symlinkSync(realpathSync(join(root,name)),join(overlay,name),"junction");
 }
 for(const name of limits.overlay.rustRoots)if(existsSync(join(overlay,name))&&existsSync(join(root,name)))for(const child of limits.overlay.heavyMembers)if(existsSync(join(root,name,child))&&!existsSync(join(overlay,name,child)))symlinkSync(join(realpathSync(join(root,name)),child),join(overlay,name,child),"junction");
}

/** 🧱️ Builds the scratch overlay of the live tree (a Rust root that is not staged is a real folder: its small members are cloned, its heavy members linked, because the source capture refuses a symbolic link inside it): the clone of the declared roots (and the top folder of every staged path), the root files, the dependency links, then the staged writes and deletes. The live tree is only read. */
export async function landBuildOverlayV1(root:string,overlay:string,changeset:LandChangesetV1,ports:LandGatePortsV1={},limits:LandPolicyV1=readLandPolicyV1()):Promise<void>{
 const clone=ports.clone??landCloneTreeV1,top=new Set<string>(limits.overlay.roots),rust=changeset.entries.some(entry=>limits.gates.rustExtensions.some(extension=>entry.path.endsWith(extension)));
 const staged=new Set(changeset.entries.map(entry=>entry.path.split("/")[0]!));
 for(const name of staged)top.add(name);
 if(rust)for(const name of limits.overlay.rustRoots){top.add(name);staged.add(name);}
 rmSync(overlay,{recursive:true,force:true});mkdirSync(overlay,{recursive:true});
 for(const name of [...top].sort()){
  if(limits.overlay.links.includes(name)&&!staged.has(name))continue;
  if(ports.signal?.aborted)throw refuseLandV1("cancelled","cancelled");
  const source=join(root,name);
  if(existsSync(source)&&statSync(source).isDirectory())await clone(source,join(overlay,name),ports.signal);
  else if(existsSync(source))copyFileSync(source,join(overlay,name));
 }
 await landLinkWorkspaceV1(root,overlay,clone,ports.signal,limits);
 for(const entry of changeset.entries){
  const path=resolve(overlay,entry.path);
  if(entry.op==="write"){mkdirSync(dirname(path),{recursive:true});writeFileSync(path,changeset.bytes.get(entry.path)!);}else rmSync(path,{force:true});
 }
}

const isScript=(path:string,limits:LandPolicyV1):boolean=>limits.gates.scriptExtensions.some(extension=>path.endsWith(extension));
const keyOf=(file:string):string=>file.replace(/(?:\.d\.mts|\.mts|\.mjs|\.tsx|\.ts|\.js|\.jsx|\.cjs|\.cts)$/u,"");

/** 📐️ The type-check policy of an overlay: the declared scan roots when the overlay holds them, else the cloned roots, so importers are found wherever the overlay was cloned. */
export function landTypecheckPolicyV1(overlay:string,limits:LandPolicyV1=readLandPolicyV1()):TypecheckPolicyV1{
 const base=readTypecheckPolicyV1();
 return base.scanRoots.some(scan=>existsSync(join(overlay,scan)))?base:{...base,scanRoots:[...limits.overlay.roots]};
}

/** 🧭️ The scripts a changeset puts under check: every written script plus the importers of every deleted one. */
export function landCheckedScriptsV1(overlay:string,changeset:LandChangesetV1,cacheDirectory:string,limits:LandPolicyV1=readLandPolicyV1()):readonly string[]{
 const written=changeset.entries.filter(entry=>entry.op==="write"&&isScript(entry.path,limits)).map(entry=>entry.path),removed=changeset.entries.filter(entry=>entry.op==="delete"&&isScript(entry.path,limits)).map(entry=>entry.path);
 const importers=removed.length?typecheckImportersV1(overlay,removed.map(keyOf),cacheDirectory,landTypecheckPolicyV1(overlay,limits)).filter(file=>existsSync(resolve(overlay,file))):[];
 return Object.freeze([...new Set([...written,...importers])].sort());
}

/** 🦀️ The crate names whose sources or manifests a changeset touches: the nearest `Cargo.toml` with a `[package]` name above each path. */
export function landTouchedCratesV1(overlay:string,changeset:LandChangesetV1,limits:LandPolicyV1=readLandPolicyV1()):readonly string[]{
 const crates=new Set<string>();
 for(const entry of changeset.entries){
  if(!limits.gates.rustExtensions.some(extension=>entry.path.endsWith(extension)))continue;
  for(let directory=dirname(entry.path);directory!=="."&&directory!=="";directory=dirname(directory)){
   const manifest=join(overlay,directory,"Cargo.toml");
   if(!existsSync(manifest))continue;
   const name=/^\s*\[package\][^[]*?\bname\s*=\s*"([^"]+)"/mu.exec(readFileSync(manifest,"utf8"))?.[1];
   if(name){crates.add(name);break;}
  }
 }
 return Object.freeze([...crates].sort());
}

/** 🧪️ The declared tests of the touched members: a written test file itself, the `🧪️tests/🟦️.ts` entry beside a written test fixture, else the `🧪️tests/🟦️.ts` entry of the nearest member above every other touched path. */
export function landDeclaredTestsV1(overlay:string,changeset:LandChangesetV1):readonly string[]{
 const tests=new Set<string>();
 for(const entry of changeset.entries){
  if(entry.op!=="write")continue;
  if(entry.path.includes("/🧪️tests/")&&entry.path.endsWith(".ts")){tests.add(entry.path);continue;}
  if(entry.path.includes("/🧪️tests/")){const entryTest=entry.path.slice(0,entry.path.indexOf("/🧪️tests/"))+"/🧪️tests/🟦️.ts";if(existsSync(join(overlay,entryTest)))tests.add(entryTest);continue;}
  for(let directory=dirname(entry.path);directory!=="."&&directory!=="";directory=dirname(directory)){
   const file=join(directory,"🧪️tests","🟦️.ts");
   if(existsSync(join(overlay,file))){tests.add(file.split("\\").join("/"));break;}
  }
 }
 return Object.freeze([...tests].sort());
}

/** ⚙️ The default type check: `repo typecheck` of the checked scripts and their importers in the overlay. */
export const landTypecheckV1:NonNullable<LandGatePortsV1["typecheck"]>=async(overlay,paths,language,signal)=>{
 const real=realpathSync(overlay),rules=landTypecheckPolicyV1(real),touched=resolveTypecheckPathsV1(real,real,paths,rules),result=await runTypecheckV1(real,touched,{paths,json:false,dryRun:false,noImporters:false,allErrors:false,declarations:false,language},language,{cacheDirectory:typecheckCacheDirectoryV1(real),signal},rules);
 return {passed:result.status==="passed",lines:renderTypecheckResultV1(result,language).split("\n")};
};

/** 🐞️ The temporary debug lines a changeset writes under the checked roots: the lines of every written file below an area root that carry the marker. Scoped to the changeset, so a peer's in-flight line elsewhere never blocks a landing. */
export function landDebugLinesV1(changeset:LandChangesetV1,limits:LandPolicyV1=readLandPolicyV1()):readonly Readonly<{file:string;line:number}>[]{
 const rows:{file:string;line:number}[]=[],decoder=new TextDecoder();
 for(const entry of changeset.entries){
  const bytes=changeset.bytes.get(entry.path);
  if(entry.op!=="write"||bytes===undefined||(limits.gates.debugAllowPaths as readonly string[]).includes(entry.path)||!limits.areas.some(rule=>entry.path.startsWith(rule.prefix+"/")))continue;
  decoder.decode(bytes).split("\n").forEach((line,index)=>{if(line.includes(limits.gates.debugMarker))rows.push({file:entry.path,line:index+1});});
 }
 return Object.freeze(rows);
}

/** ⚙️ The default latency check: `repo bench latency --strict` on the live checkout (the budgets measure the installed commands, not the overlay). */
export const landBenchV1:NonNullable<LandGatePortsV1["bench"]>=async request=>{
 const run=await spawnWait(process.env.SEMIO_REPO_BINARY??"repo",[...readLandPolicyV1().gates.benchCommand],{cwd:request.root,signal:request.signal});
 return {passed:run.status===0,lines:run.output.split("\n").filter(line=>/breach|warn|over/iu.test(line)).slice(0,10)};
};

/** ⚙️ The default Rust check: `repo cargo check -p <crate>` per touched crate, in the overlay. */
export const landRustV1:NonNullable<LandGatePortsV1["rust"]>=async request=>{
 const lines:string[]=[];let passed=true;
 for(const crate of request.crates){
  const run=await spawnWait(process.env.SEMIO_REPO_BINARY??"repo",["cargo","--store=shared","--configuration=prototype","--","check","-p",crate],{cwd:request.overlay,signal:request.signal});
  if(run.status!==0){passed=false;lines.push(...run.output.split("\n").filter(line=>/error|warning: unused/u.test(line)).slice(0,10),crate+": exit "+run.status);}
 }
 return {passed,lines};
};

/** ✍️ A signature of a failure text: the names of the failing tests when the runner printed them, else the text with the roots replaced by `<root>`, timings removed and whitespace folded. */
export function landFailureSignatureV1(text:string,roots:readonly string[]):string{
 let value=text;
 for(const root of roots)value=value.split(root).join("<root>");
 const failed=[...value.matchAll(/\(fail\) ([^\n|[]*)/gu)].map(match=>match[1]!.trim()).sort();
 if(failed.length)return "failed: "+[...new Set(failed)].join("; ");
 return value.replace(/\[(?:script|budget)\][^\n|]*/gu,"").replace(/\[\d+(?:\.\d+)?m?s\]/gu,"").replace(/\b\d+(?:\.\d+)? ?(?:ms|s)\b/gu,"<t>").replace(/\s+/gu," ").trim().slice(0,600);
}

/** ⚙️ The default test run: `repo test-file --json` over the files, in the overlay, with the checkout-test offload switched off (a worker would run the live copy of the files, not the overlay). */
export const landTestsV1:NonNullable<LandGatePortsV1["tests"]>=async request=>{
 const {runTestFileCliV1}=await import("../../🔬️test-file/⌨️command/🟦️.ts"),policy=readLandPolicyV1(),real=realpathSync(request.overlay),out:string[]=[];
 const code=await runTestFileCliV1(real,["--json","--",...request.files],{...process.env,...policy.gates.offloadEnvironment},line=>out.push(line),{directory:real});
 const lines:string[]=[],failures:{file:string;signature:string;bound?:boolean}[]=[];
 try{
  const result=JSON.parse(out.join("\n")) as {files:readonly {file:string;status:string;error?:string;tail?:string}[]};
  for(const row of result.files)if(row.status!=="passed"){lines.push(row.file+": "+row.status+" "+(row.error??"")+" "+(row.tail??"").split("\n").slice(-6).join(" | "));{const output=(row.error??"")+(row.tail??"");failures.push({file:row.file,signature:landFailureSignatureV1(output,[request.overlay,real]),bound:policy.gates.workspaceBound.some(marker=>output.includes(marker))});}}
 }catch{lines.push(...out.slice(-6));}
 return {passed:code===0,lines,failures};
};

/** 🚦️ Runs the checks of a landing against the overlay and returns one verdict per check; a check with nothing to look at is skipped, never passed. */
export async function landRunGatesV1(input:Readonly<{root:string;overlay:string;cacheDirectory:string;changeset:LandChangesetV1;language:LandLanguageV1;bench:boolean}>,ports:LandGatePortsV1={},limits:LandPolicyV1=readLandPolicyV1()):Promise<readonly LandGateRowV1[]>{
 const now=ports.now??Date.now,notice=ports.notice??(()=>{}),rows:LandGateRowV1[]=[],language=input.language,text=(key:LandTextKeyV1,values:Record<string,string|number>={})=>landTextV1(language,key,values),cut=(lines:readonly string[])=>lines.slice(0,20).map(line=>line.slice(0,limits.limits.diagnosticCharacters));
 const baseline=async(files:readonly string[],outcome:LandTestOutcomeV1):Promise<LandTestOutcomeV1>=>{
  if(outcome.passed||outcome.failures===undefined||!outcome.failures.length)return outcome;
  const live=await (ports.tests??landTestsV1)({overlay:input.root,files:outcome.failures.map(row=>row.file),signal:ports.signal}),known=new Map((live.failures??[]).map(row=>[row.file,row.signature]));
  const unequal=outcome.failures.filter(row=>known.get(row.file)!==row.signature),old=outcome.failures.filter(row=>known.get(row.file)===row.signature);
  const later=unequal.filter(row=>row.bound===true&&!known.has(row.file)&&ports.deferred!==undefined),fresh=unequal.filter(row=>!later.includes(row));
  for(const row of later)ports.deferred!.push(row.file);
  const notes=[...old.map(row=>row.file+": pre-existing (fails the same way on the live tree)"),...later.map(row=>text("tests.deferred",{file:row.file}))];
  return {passed:fresh.length===0,lines:[...notes,...outcome.lines.filter(line=>fresh.some(row=>line.startsWith(row.file+":")))],failures:fresh};
 };
 const timed=async(id:LandGateIdV1,skip:LandTextKeyV1|undefined,work:()=>Promise<{passed:boolean;lines:readonly string[]}>):Promise<void>=>{
  if(ports.signal?.aborted)throw refuseLandV1("cancelled","cancelled");
  if(skip!==undefined){rows.push({id,status:"skipped",seconds:0,detail:[text(skip)]});notice(text("gate.skipped",{gate:id,reason:text(skip)}));return;}
  notice(text("progress.gate",{gate:id}));
  const begin=now();let outcome:{passed:boolean;lines:readonly string[]};
  try{outcome=await work();}catch(error){if(ports.signal?.aborted)throw refuseLandV1("cancelled","cancelled");outcome={passed:false,lines:[String((error as Error).message??error)]};}
  const seconds=Math.round((now()-begin)/10)/100;
  rows.push({id,status:outcome.passed?"passed":"failed",seconds,detail:cut(outcome.lines)});
  notice(text(outcome.passed?"gate.passed":"gate.failed",{gate:id,seconds}));
 };
 const scripts=landCheckedScriptsV1(input.overlay,input.changeset,input.cacheDirectory,limits),crates=landTouchedCratesV1(input.overlay,input.changeset,limits),law=input.changeset.entries.some(entry=>limits.gates.suiteLawTriggers.some(trigger=>("/"+entry.path).endsWith(trigger)));
 const debugRoots=input.changeset.entries.some(entry=>entry.op==="write"&&limits.areas.some(rule=>entry.path.startsWith(rule.prefix+"/")));
 await timed("debug-lines",debugRoots?undefined:"skip.roots",async()=>{const found=landDebugLinesV1(input.changeset,limits);return {passed:!found.length,lines:found.map(row=>text("debug.row",{file:row.file,line:row.line}))};});
 await timed("dangling-imports",scripts.length?undefined:"skip.scripts",async()=>{const found=landDanglingImportsV1(input.overlay,scripts);return {passed:!found.length,lines:found.map(row=>text("dangling.row",{file:row.file,specifier:row.specifier}))};});
 await timed("typecheck",scripts.length?undefined:"skip.scripts",()=>(ports.typecheck??landTypecheckV1)(input.overlay,scripts,language,ports.signal));
 await timed("rust",crates.length?undefined:"skip.rust",()=>(ports.rust??landRustV1)({overlay:input.overlay,crates,signal:ports.signal}));
 await timed("suite-law",law?undefined:"skip.law",async()=>baseline(limits.gates.suiteLawTests,await (ports.tests??landTestsV1)({overlay:input.overlay,files:limits.gates.suiteLawTests,signal:ports.signal})));
 const declared=landDeclaredTestsV1(input.overlay,input.changeset),chosen=declared.slice(0,limits.gates.maximumTests);
 if(declared.length>chosen.length)notice(text("tests.truncated",{count:declared.length,maximum:limits.gates.maximumTests}));
 await timed("tests",chosen.length?undefined:"skip.tests",async()=>baseline(chosen,await (ports.tests??landTestsV1)({overlay:input.overlay,files:chosen,signal:ports.signal})));
 await timed("bench",input.bench?undefined:"skip.bench",()=>(ports.bench??landBenchV1)({root:input.root,signal:ports.signal}));
 return Object.freeze(rows);
}
