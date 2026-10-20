import {spawn} from "node:child_process";
const program=String.raw`
import {readFileSync} from "node:fs";
import {realpath} from "node:fs/promises";
import {createHash} from "node:crypto";
import * as paths from "node:path";
import {pathToFileURL} from "node:url";
const root="/repo",authorizer="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers",file=paths.join(root,authorizer,"🟦️.ts");
const api=await import(pathToFileURL(file).href),workspace=await import(pathToFileURL(paths.join(root,authorizer,"📦️workspace/🟦️.ts")).href),loaded=await import("typescript"),ts=loaded.default??loaded,source=readFileSync(file,"utf8"),tree=ts.createSourceFile(file,source,ts.ScriptTarget.Latest,true),transpiler=new Bun.Transpiler({loader:"ts",target:"bun"});
const functionSource=name=>{const node=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name);if(!node)throw Error("Actual function absent: "+name);return node.getText(tree).replace(/^export\s+/u,"");};
const helpers=["scannedImport","sourcePath"].map(functionSource).map(value=>transpiler.transformSync(value)).join("\n"),capture=functionSource("captureNxAuthorizerSourcesV1"),guard="files.size>256||total>16_777_216";
if(capture.split(guard).length!==2)throw Error("Exact original capacity guard required");
const role=process.env.SEMIO_AUTHORIZER_CENSUS_ROLE;
if(!["runtime","compiler"].includes(role))throw Error("Explicit census role required");
const hash=value=>createHash("sha256").update(value).digest("hex"),entry=authorizer+(role==="compiler"?"/📜️script.ts":"/📦️runtime/🟦️.ts"),results=[];
for(const kind of ["original-capacity","bounded-census"]){
 const rows=new Map(),controller=new AbortController(),timer=setTimeout(()=>controller.abort(Error("Bounded Source census deadline exceeded")),60000),started=new Date().toISOString();
 class ObservedMap extends Map{set(key,value){if(typeof key==="string"&&value instanceof Uint8Array)rows.set(key,{path:key,bytes:value.byteLength,sha256:hash(value)});return super.set(key,value);}}
 const {dirname,extname,isAbsolute,join,relative,resolve}=paths;
 const ports={dirname,extname,isAbsolute,join,relative,resolve,realpath,hash,compare:(left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)),external:new Set(api.nxAuthorizerExternalModulesV1),readNxAuthorizerSourceV1:api.readNxAuthorizerSourceV1,nxAuthorizerWorkspaceCandidateV1:workspace.nxAuthorizerWorkspaceCandidateV1,nxAuthorizerWorkspaceIndexV1:workspace.nxAuthorizerWorkspaceIndexV1,resolveNxAuthorizerWorkspaceV1:workspace.resolveNxAuthorizerWorkspaceV1,capturedBytes:new WeakMap(),Map:ObservedMap,isBuiltin:(await import("node:module")).isBuiltin};
 const body=kind==="original-capacity"?capture:capture.replace(guard,"files.size>4096||total>67_108_864"),fn=new Function(...Object.keys(ports),helpers+"\n"+transpiler.transformSync(body)+"\nreturn captureNxAuthorizerSourcesV1;")(...Object.values(ports));
 let failure=null,manifest=null;
 try{const value=await fn(root,entry,controller.signal);manifest={sources:value.manifest.sources.length,resolutions:value.manifest.resolutions.length,digest:value.digest};}catch(error){failure=String(error);}finally{clearTimeout(timer);}
 const files=[...rows.values()],totalBytes=files.reduce((sum,row)=>sum+row.bytes,0),record={kind,started,finished:new Date().toISOString(),scope:"Source projection of actual private capture with exact actual reader/Bun scanner/resolver; telemetry-only Map; diagnostic census guard4096files/64MiB only in second projection; no production Source or Native authority change",authorizerSha256:hash(source),entry,files:files.length,totalBytes,maximumFileBytes:Math.max(0,...files.map(row=>row.bytes)),failure,manifest,rows:files};
 results.push(record);console.error("[DEBUG] Bounded authorizer Source observation "+JSON.stringify({...record,rows:undefined}));
}
console.log(JSON.stringify({version:1,results},null,2));
if(![null,"Error: Authorizer source closure capacity exceeded"].includes(results[0].failure)||results[1].failure)process.exitCode=1;
`;
const role=process.argv[2]??"runtime";
if(!["runtime","compiler"].includes(role))throw Error("Exact authorizer role required");
console.error("[DEBUG] Explicit Source census role "+role);
const child=spawn("docker",["exec","--env","SEMIO_AUTHORIZER_CENSUS_ROLE="+role,"--workdir","/repo","semio-course-builder-sol-8579fa82486d","bun","--eval",program],{stdio:"inherit"});
const status=await new Promise<number|null>(resolve=>child.once("exit",resolve));
process.exitCode=status??1;

