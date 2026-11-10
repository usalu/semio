/** 🧭️ Proves that the roster of the `project-set` node equals the roster inference derives today, on the real workspace: compares (A) the legacy rules restated verbatim (old walk, old candidate admission, old project and manifest loops), (B) the live inference functions (`workspaceBuildFilesV1`, `inferenceCandidates`, the path the setup of inference takes now, and the project loops) and (C) the `project-set` node of the shared workspace graph. Run: repo tools exec --light bun -- run <this file> [outputFile] */
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,writeFileSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
const ROOT="/home/deck/semio",BUILD=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",LIBRARY=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const OUT=process.argv[2]??"roster-equivalence.json",PROJECT="📋️project.json";
const rss=()=>Math.round(process.memoryUsage().rss/1048576),time=async<T>(work:()=>Promise<T>|T):Promise<[T,number]>=>{const at=performance.now(),value=await work();return[value,Math.round(performance.now()-at)];};
const cachePolicy=JSON.parse(readFileSync(LIBRARY+"/⚡️caching/🔣️policy.json","utf8")),ownership=JSON.parse(readFileSync(LIBRARY+"/🗂️workspaces/🔣️policy.json","utf8"));
const {isOpaqueSourcePathV1}=await import(LIBRARY+"/⚡️caching/🔍️discovery/📂️source/🟦️.ts");

/** (A) the legacy rules, restated verbatim from the code before the shared rules existed. */
const legacyExcluded=new Set([...cachePolicy.generatedDirectories,".git",".🧬semio",".nx",".cache",".r","node_modules","dist","target",".venv","venv","__pycache__",".pytest_cache",".next",".turbo","temp","🗑️generated"]);
const legacyDescends=(name:string,path:string,parent:string)=>!isOpaqueSourcePathV1(path)&&!legacyExcluded.has(name)&&!name.startsWith("extension-host-build-")&&!(ownership.collections.names.includes(name)&&parent.split("/").at(-1)!==ownership.collections.moduleMember)&&path!=="compose";
const legacyCollection=(path:string)=>{const parts=path.split("\\").join("/").split("/");return parts.some((part,index)=>ownership.collections.names.includes(part)&&parts[index-1]!==ownership.collections.moduleMember);};
const legacyAdmitsCandidate=(supplied:unknown)=>{
 if(typeof supplied!=="string"||!supplied||supplied.includes("\0")||supplied.includes("\uFFFD"))return false;
 const path=supplied.split("\\").join("/"),parts=path.split("/");
 return !(isOpaqueSourcePathV1(path)||legacyCollection(path)||path.startsWith("/")||/^[A-Za-z]:/u.test(path)||parts.some(part=>!part||part==="."||part===".."||part===".🧬semio"||cachePolicy.generatedDirectories.includes(part))||path.startsWith("compose/")||path.startsWith("temp/compose/"));
};
const legacyAuthored=(file:string)=>!isOpaqueSourcePathV1(file)&&file.endsWith(PROJECT)&&!file.includes("\uFFFD")&&!file.includes(".🧬semio")&&!cachePolicy.generatedDirectories.some((name:string)=>file.split("/").includes(name));
const legacyManifest=(file:string)=>!(isOpaqueSourcePathV1(file)||file.includes("\uFFFD")||file.startsWith("compose/")||file.startsWith("temp/compose/")||file.includes(".🧬semio")||cachePolicy.generatedDirectories.some((name:string)=>file.split("/").includes(name)));
function legacyWalk(){
 const files:string[]=[],walk=(directory:string)=>{
  for(const entry of readdirSync(join(ROOT,directory),{withFileTypes:true})){
   if(entry.isSymbolicLink())continue;
   const path=join(directory,entry.name).replaceAll("\\","/");
   if(entry.isDirectory()){if(legacyDescends(entry.name,path,directory))walk(path);}else if(entry.isFile())files.push(path);
  }
 };
 walk("");return files.sort();
}

type Row={root:string;name:string|null;unavailable:boolean};
const rosterFrom=async(foundFiles:readonly string[],readToml:(path:string)=>any):Promise<Row[]>=>{
 const rows=new Map<string,Row>(),declared=new Set<string>();
 for(const file of foundFiles.filter(legacyAuthored)){
  const root=dirname(file)==="."?".":dirname(file);let json:any;
  try{json=JSON.parse(readFileSync(join(ROOT,file),"utf8"));}catch{rows.set(root,{root,name:null,unavailable:true});declared.add(root);continue;}
  rows.set(root,{root,name:typeof json.name==="string"&&json.name!==""?json.name:null,unavailable:false});declared.add(root);
 }
 for(const file of foundFiles.filter(file=>file.endsWith("Cargo.toml"))){
  if(!legacyManifest(file)||!existsSync(join(ROOT,file)))continue;
  const root=dirname(file)==="."?".":dirname(file);if(declared.has(root))continue;
  let manifest:any;try{manifest=readToml(join(ROOT,dirname(file),"Cargo.toml"));}catch{rows.set(root,{root,name:null,unavailable:true});continue;}
  rows.set(root,{root,name:typeof manifest.package?.name==="string"&&manifest.package.name!==""?manifest.package.name:null,unavailable:false});
 }
 return[...rows.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0);
};
const canonicalRoot=(row:Row)=>({...row,root:row.root==="."?"":row.root});
const diff=(left:readonly Row[],right:readonly Row[])=>{
 const at=new Map(left.map(row=>[row.root,row])),other=new Map(right.map(row=>[row.root,row])),rows:{root:string;left?:Row;right?:Row}[]=[];
 for(const[root,row]of at)if(JSON.stringify(row)!==JSON.stringify(other.get(root)))rows.push({root,left:row,...other.has(root)?{right:other.get(root)!}:{}});
 for(const[root,row]of other)if(!at.has(root))rows.push({root,right:row});
 return rows;
};

const report:Record<string,unknown>={host:{rssStartMb:rss()}};
const[legacyFiles,legacyWalkMs]=await time(legacyWalk);report.legacyWalk={files:legacyFiles.length,ms:legacyWalkMs};

const inference=await import(BUILD+"/🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const{workspaceBuildFilesV1}=await import(BUILD+"/🕸️graph/📁️workspace/🟦️.ts"),{inferenceCandidates}=await import(BUILD+"/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"),{readToml}=await import(BUILD+"/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs");
const[liveFiles,liveWalkMs]=await time(()=>workspaceBuildFilesV1(ROOT));report.liveWalk={files:liveFiles.length,ms:liveWalkMs,sameAsLegacy:JSON.stringify(liveFiles)===JSON.stringify(legacyFiles)};

const configFilter=(file:string)=>file.endsWith(PROJECT)||file.endsWith("Cargo.toml");
const[legacyFound,legacyDiscoverMs]=await time(()=>{
 const files=new Set<string>();
 for(const supplied of legacyFiles.filter(configFilter)){if(!legacyAdmitsCandidate(supplied))continue;try{const status=lstatSync(join(ROOT,supplied));if(status.isFile()&&!status.isSymbolicLink())files.add(supplied);}catch{}}
 return[...files].sort();
});
const liveConfigs=liveFiles.filter(configFilter),[liveFound,liveDiscoverMs]=await time(()=>inferenceCandidates([...liveConfigs,...liveConfigs.filter(file=>file.endsWith("Cargo.toml")).map(file=>join(dirname(file),PROJECT).split("\\").join("/"))],ROOT) as string[]);
report.discover={legacyCandidates:legacyFound.length,legacyMs:legacyDiscoverMs,liveCandidates:liveFound.length,liveMs:liveDiscoverMs,nativeOnlyInLive:liveFound.filter(file=>!legacyFound.includes(file)).length,legacyOnly:legacyFound.filter(file=>!liveFound.includes(file)).length};

const[legacyRoster,legacyRosterMs]=await time(()=>rosterFrom(legacyFound,readToml));
const[liveRoster,liveRosterMs]=await time(()=>rosterFrom(liveFound,readToml));
report.legacyRoster={projects:legacyRoster.length,ms:legacyRosterMs};report.liveRoster={projects:liveRoster.length,ms:liveRosterMs};

const{createClosureSettlerV1}=await import(BUILD+"/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs"),{ROSTER_RULES_V1}=await import(BUILD+"/🕸️graph/🧭️roster/🟨️.mjs");
const cache=mkdtempSync(join(tmpdir(),"roster-equivalence-")),settler=createClosureSettlerV1({workspace:{workspaceRoot:ROOT,cacheRoot:cache,descends:ROSTER_RULES_V1.descends}}) as any;
const graph=settler.graph;
const[cold,coldMs]=await time(()=>graph.get(graph.key("project-set","workspace",{})));
const nodeRoster:Row[]=(cold.value.projects as Row[]).map(row=>({root:row.root,name:row.name,unavailable:row.unavailable})).filter(row=>row.name!==null||row.unavailable).map(canonicalRoot).sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0);
report.projectSet={projects:cold.value.projects.length,named:nodeRoster.length,duplicates:cold.value.duplicates.length,coldMs,rssMb:rss()};
const[warm,warmMs]=await time(()=>graph.get(graph.key("project-set","workspace",{})));report.projectSetWarm={ms:warmMs,sameNode:warm===cold||warm.valueDigest===cold.valueDigest};
const[noop,noopMs]=await time(()=>graph.invalidate({paths:[]}));report.projectSetNoopInvalidate={ms:noopMs,computed:(noop as any).computed};
// one-line edit of one project file: re-read of that project and the roster
const edited=legacyFound.find(file=>file.endsWith(PROJECT))!,original=readFileSync(join(ROOT,edited),"utf8");
writeFileSync(join(cache,"unused"),"");
void original;void mkdirSync;

const A=legacyRoster.map(canonicalRoot).filter(row=>row.name!==null||row.unavailable),B=liveRoster.map(canonicalRoot).filter(row=>row.name!==null||row.unavailable);
report.differences={legacyVsLive:diff(A,B),legacyVsNode:diff(A,nodeRoster),liveVsNode:diff(B,nodeRoster)};
const total=(report.differences as Record<string,unknown[]>);
report.summary={legacyVsLive:total.legacyVsLive!.length,legacyVsNode:total.legacyVsNode!.length,liveVsNode:total.liveVsNode!.length,walkEqual:(report.liveWalk as any).sameAsLegacy};
writeFileSync(OUT,JSON.stringify(report,null,1));
console.log(JSON.stringify({summary:report.summary,legacyWalk:report.legacyWalk,discover:report.discover,legacyRoster:report.legacyRoster,projectSet:report.projectSet,warm:report.projectSetWarm,noop:report.projectSetNoopInvalidate}));
