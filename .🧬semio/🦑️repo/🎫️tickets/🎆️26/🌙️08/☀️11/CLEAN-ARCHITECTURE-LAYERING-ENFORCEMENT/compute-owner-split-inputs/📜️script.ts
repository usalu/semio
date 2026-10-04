#!/usr/bin/env bun
/** 🧭️ Retains owner source pairs and applies guarded compute test ownership edits. */
import {readFileSync,writeFileSync,mkdirSync,existsSync} from "node:fs";
import {join,dirname,resolve} from "node:path";
import * as ts from "typescript";
import Ajv from "ajv";
const ticket=resolve(import.meta.dir,".."),root=resolve(ticket,"../../../../../../.."),hash=(value:string|null)=>value===null?null:new Bun.CryptoHasher("sha256").update(value).digest("hex");
process.chdir(root);
type Row={path:string;before:string|null;beforeHash:string|null;authored:string|null;authoredHash:string|null;inverse:{before:string|null;after:string|null}};
const read=(path:string)=>existsSync(path)?readFileSync(path,"utf8"):null;
const pair=(path:string,authored:string|null):Row=>{const before=read(path);return {path,before,beforeHash:hash(before),authored,authoredHash:hash(authored),inverse:{before:authored,after:before}};};
const replace=(source:string,before:string,after:string)=>{if(source.split(before).length!==2)throw Error("Nonunique splice");return source.replace(before,after);};
const indent=(value:unknown)=>JSON.stringify(value,null,2).split("\n").map(line=>"    "+line).join("\n");
async function prepareConstructor(){
 const initial=JSON.parse(readFileSync(join(import.meta.dir,"constructor-owner-ready-2.json"),"utf8"));
 const rows:Row[]=initial.rows.map((row:Row)=>{if(read(row.path)!==row.before)throw Error("Changed owned input "+row.path);return row;});
 const repo="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript";
 const script=read(join(repo,"📜️script.ts"))!;
 const start=script.indexOf('    if (segments[0] === "dependency-direction") {'),end=script.indexOf('    if (segments[0] === "canonical-architecture") {',start);
 if(start<0||end<start)throw Error("Missing constructor registration anchor");
 const route=script.slice(start,end).replaceAll("dependency-direction","compute-constructor-owner").replace("🧪️tests/🧱️compute-constructor-owner/🟦️.ts","🕸️dependencies/🧭️direction/🧩️compute-constructor/🧪️tests/🟦️.ts");
 rows.push(pair(join(repo,"📜️script.ts"),script.slice(0,start)+route+script.slice(start)));
 const projectPath=join(repo,"📋️project.json"),projectSource=read(projectPath)!,project=JSON.parse(projectSource);
 const target={executor:"nx:run-commands",cache:false,dependsOn:[],outputs:[],inputs:["sharedGlobals","{projectRoot}/📜️script.ts","{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🧩️compute-constructor/**/*","{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/**/*","{workspaceRoot}/🧰️framework/🔨️modules/🧬️schema/✅️validator/**/*","{workspaceRoot}/📜️script.ts"],options:{cwd:repo,command:"bun ./📜️script.ts test compute-constructor-owner"}};
 if(project.targets["test-compute-constructor-owner"])throw Error("Duplicate owner target");
 const newTarget='    "test-compute-constructor-owner": '+JSON.stringify(target,null,2).split("\n").join("\n    ")+",\n";
 rows.push(pair(projectPath,replace(projectSource,'    "test-dependency-direction":',newTarget+'    "test-dependency-direction":')));
 const packagePath=join(repo,"package.json"),packageSource=read(packagePath)!;
 rows.push(pair(packagePath,replace(packageSource,'    "test-dependency-direction":','    "test-compute-constructor-owner": "nx run @semio-tech/repo-lib:test-compute-constructor-owner",\n    "test-dependency-direction":')));
 const launchPath=".vscode/🧩️launch.seed.jsonc",launch=read(launchPath)!,name='"name": "🧪️test◻️2d🧮️compute📍️ownership"',at=launch.indexOf(name);
 if(at<0||launch.indexOf(name,at+1)>=0)throw Error("Missing or ambiguous GUI source anchor");
 const offset=launch.lastIndexOf("\n    {",at)+1,workspace="$"+"{workspaceFolder}";
 const entry={name:"🦑️repo 🧩️compute constructor ownership",type:"node-terminal",request:"launch",command:"bun nx run @semio-tech/repo-lib:test-compute-constructor-owner --skip-nx-cache",cwd:workspace,env:{SEMIO_TEST_ARTIFACT_DIR:workspace+"/"+ticket.slice(root.length+1)+"/🗑️generated/goal-compute-constructor"},presentation:{group:"9_gates",order:900.057865}};
 rows.push(pair(launchPath,launch.slice(0,offset)+indent(entry)+",\n"+launch.slice(offset)));
 const schema=JSON.parse(rows[3]!.authored!),corpus=JSON.parse(rows[2]!.authored!),accepted=new Ajv({strict:true}).compile(schema)(corpus);
 const parseDiagnostics=rows.filter(row=>row.path.endsWith(".ts")).flatMap(row=>ts.createSourceFile(row.path,row.authored!,ts.ScriptTarget.Latest,true).parseDiagnostics.map(d=>ts.flattenDiagnosticMessageText(d.messageText,"\n")));
 if(!accepted||parseDiagnostics.length)throw Error("Unadmitted constructor proposal");
 writeFileSync(join(import.meta.dir,"constructor-owner-route-ready-3.json"),JSON.stringify({sourceWrites:false,at:new Date().toISOString(),rows,originalLaws:initial.originalLaws,originalVectors:11,oracleAccepted:accepted,parseDiagnostics}));
 console.log(JSON.stringify({rows:rows.length,originalVectors:11,oracleAccepted:accepted,parseDiagnostics:parseDiagnostics.length}));
}
async function mountConstructor(){
 const j=JSON.parse(readFileSync(join(import.meta.dir,"constructor-owner-route-ready-3.json"),"utf8")),rows:Row[]=j.rows;
 const gui=rows.find(row=>row.path===".vscode/🧩️launch.seed.jsonc")!,currentGui=read(gui.path)!;
 if(currentGui!==gui.before){
  const needle='"name": "🧪️test◻️2d🧮️compute📍️ownership"',start=gui.before!.lastIndexOf("\n    {",gui.before!.indexOf(needle))+1,next=gui.before!.indexOf("\n    {",start+1),anchor=gui.before!.slice(start,next),delta=gui.authored!.length-gui.before!.length,addition=gui.authored!.slice(start,start+delta);
  if(currentGui.split(anchor).length!==2||currentGui.includes('"name": "🦑️repo 🧩️compute constructor ownership"'))throw Error("Changed owned GUI anchor");
  gui.before=currentGui;gui.beforeHash=hash(currentGui);gui.authored=currentGui.replace(anchor,addition+anchor);gui.authoredHash=hash(gui.authored);gui.inverse={before:gui.authored,after:currentGui};
 }
 const gaps=rows.filter(row=>read(row.path)!==row.before).map(row=>row.path);
 if(gaps.length)throw Error("Owned guard gaps "+JSON.stringify(gaps));
 writeFileSync(join(import.meta.dir,"constructor-publication-before-1.json"),JSON.stringify({at:new Date().toISOString(),rows,inverse:rows.map(r=>({path:r.path,...r.inverse}))}));
 for(const row of rows){mkdirSync(dirname(row.path),{recursive:true});writeFileSync(row.path,row.authored!);}
 const after=rows.map(row=>({path:row.path,source:read(row.path),sha256:hash(read(row.path)),inverse:row.inverse}));
 if(after.some((row,i)=>row.source!==rows[i]!.authored))throw Error("Postpublication mismatch");
 writeFileSync(join(import.meta.dir,"constructor-mounted-1.json"),JSON.stringify({at:new Date().toISOString(),rows:after}));
 console.log(JSON.stringify({mounted:after.length,postGaps:0}));
}
async function strictConstructor(){
 const j=JSON.parse(readFileSync(join(import.meta.dir,"constructor-owner-route-ready-3.json"),"utf8")),overlay=new Map<string,string>(j.rows.filter((r:Row)=>r.authored!==null).map((r:Row)=>[resolve(r.path),r.authored!]));
 const options:ts.CompilerOptions={noEmit:true,strict:true,skipLibCheck:true,allowImportingTsExtensions:true,esModuleInterop:true,resolveJsonModule:true,target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,types:["bun"]},host=ts.createCompilerHost(options),oldRead=host.readFile.bind(host),oldExists=host.fileExists.bind(host),oldDirectory=host.directoryExists!.bind(host);
 host.readFile=path=>overlay.get(resolve(path))??oldRead(path);host.fileExists=path=>overlay.has(resolve(path))||oldExists(path);host.directoryExists=path=>[...overlay.keys()].some(file=>file.startsWith(resolve(path)+"/"))||oldDirectory(path);
 const roots=j.rows.filter((r:Row)=>r.path.endsWith(".ts")&&!r.path.endsWith("📜️script.ts")).map((r:Row)=>resolve(r.path));
 const diagnostics=ts.getPreEmitDiagnostics(ts.createProgram(roots,options,host)).map(d=>({path:d.file?.fileName,start:d.start,message:ts.flattenDiagnosticMessageText(d.messageText,"\n")}));
 const output=join(ticket,"🗑️generated/goal-root/compute-constructor-strict-3.json");mkdirSync(dirname(output),{recursive:true});writeFileSync(output,JSON.stringify({roots,diagnostics}));console.log(JSON.stringify({roots:roots.length,diagnostics:diagnostics.length}));if(diagnostics.length)throw Error("Strict constructor diagnostics");
}
try{const command=process.argv[2];if(command==="prepare-constructor")await prepareConstructor();else if(command==="strict-constructor")await strictConstructor();else if(command==="mount-constructor")await mountConstructor();else throw Error("Unknown owned command");}catch(error){console.error(error);process.exit(1);}
