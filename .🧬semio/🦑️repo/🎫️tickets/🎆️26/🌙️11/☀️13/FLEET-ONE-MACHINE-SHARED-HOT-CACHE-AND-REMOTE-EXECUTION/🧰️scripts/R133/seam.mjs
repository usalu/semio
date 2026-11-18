import {readFileSync,writeFileSync,readdirSync,statSync} from "node:fs";
import {join,relative,dirname,sep} from "node:path";
export const ROOT="/Users/ueli/Documents/semio",FRAMEWORK=join(ROOT,"🧰️framework");
export const RESOLVER=join(FRAMEWORK,"🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts");
export const SCOPES=[join(FRAMEWORK,"🛍️products/🦑️repo/🔨️modules/📚️library"),join(FRAMEWORK,"🔨️modules/🏃️process")];
const SKIP=new Set(["node_modules","target",".cache","🧪️tests","🧫️fixtures"]);
const ROOT_LIKE=new Set(["root","workspace","options.workspace","context.root","input.root","plan.workspace","fleetRoot","this.repoRoot","owner.workspace","repository","ports.root","workspaceRoot","findWorkspaceRoot(dirname(fileURLToPath(import.meta.url)))","callerRepositoryRootV1(invocation.capabilities)","callerRepositoryRootV1(store.invocation.capabilities)","request.workspace","row.workspace","context.root??\".\"","context.root!","repositoryRoot","scope.options.workspace","held.workspace","worktree","runtime.workspace","testCheckoutRootV1()","checkout"]);

const LIB_="🛍️products/🦑️repo/🔨️modules/📚️library/";
export const MATCHED=new Set();
export const PATCHES=[
 [LIB_+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts","join(root,declared.directory)","machinePathV1(root,declared.directory)",2],
 [LIB_+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts","requireMaterializedPathV1(join(root,partition.directory))","requireMaterializedPathV1(machinePathV1(root,partition.directory))",1],
 [LIB_+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts","join(requireMaterializedPathV1(resolve(workspace)),declared.directory)","machinePathV1(requireMaterializedPathV1(resolve(workspace)),declared.directory)",2],
 [LIB_+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts","join(workspace,policy.partitions[id].directory,native.definitions.ImageProtocol.const.mountDirectory)","machinePathV1(workspace,policy.partitions[id].directory,native.definitions.ImageProtocol.const.mountDirectory)",1],
 [LIB_+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/🚧️fence/🟦️.ts",".map(directory=>join(checkout,directory))",".map(directory=>machinePathV1(checkout,directory))",1],
 [LIB_+"🏗️build/🔒️leases/⌨️command/🟦️.ts","join(root,REPOSITORY_LEASE_DIRECTORY_V1)","machinePathV1(root,REPOSITORY_LEASE_DIRECTORY_V1)",1],
 [LIB_+"🏗️build/🩺️doctor/🔎️probes/🔒️leases/🟦️.ts","resolve(ports.root,REPOSITORY_LEASE_DIRECTORY_V1)","machinePathV1(ports.root,REPOSITORY_LEASE_DIRECTORY_V1)",1],
 [LIB_+"🏗️build/🪪️agent-sessions/⌨️command/🟦️.ts","`${root}/${REPOSITORY_LEASE_DIRECTORY_V1}`","machinePathV1(root,REPOSITORY_LEASE_DIRECTORY_V1)",1],
 [LIB_+"🏗️build/🚢️fleet-status/📥️readers/🟦️.ts","join(context.root,REPOSITORY_LEASE_DIRECTORY_FLEET_V1)","machinePathV1(context.root,REPOSITORY_LEASE_DIRECTORY_FLEET_V1)",1],
 [LIB_+"🏗️build/📥️inputs/💾️storage/🟨️.mjs","join(owner,policy.partitions.graph.directory,\"m\",namespace)","machinePathV1(owner,policy.partitions.graph.directory,\"m\",namespace)",1],
 [LIB_+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts","join(input.root,...P.directory)","machinePathV1(input.root,...P.directory)",2],
 [LIB_+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts","join(input.root,P.sliceReport.directory,P.sliceReport.name)","machinePathV1(input.root,P.sliceReport.directory,P.sliceReport.name)",1],
 [LIB_+"🏗️build/🧠️memory/📋️slice-report/🟨️.mjs","join(root,P.directory)","machinePathV1(root,P.directory)",1],
 [LIB_+"🏗️build/🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts","join(root,policy.directory,policy.file)","machinePathV1(root,policy.directory,policy.file)",1],
 [LIB_+"🏗️build/🪪️graph-state/📍️cursor/🟦️.ts","join(root,...policy.directory.split(\"/\"))","machinePathV1(root,...policy.directory.split(\"/\"))",1],
 [LIB_+"🏗️build/🌐️remote-execution/🛰️sidecar/🗄️cache/🟦️.ts","join(context.root,policy.fleetDirectory)","machinePathV1(context.root,policy.fleetDirectory)",1],
 [LIB_+"⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧰️setup/🟦️.ts","join(context.root,...policy.stagingDirectory,","machinePathV1(context.root,...policy.stagingDirectory,",1],
 [LIB_+"⚡️caching/💾️storage/🔒️enforcement/🟦️.ts","join(root,...authored.directory.split(\"/\"))","machinePathV1(root,...authored.directory.split(\"/\"))",1],
 [LIB_+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts","const stages=READY_STAGES_V1.map(stage=>{","const stages=(input.stages??READY_STAGES_V1).map(stage=>{",1],
 [LIB_+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts","platform:ReadyPlatformV1;hash:(path:string)=>string|undefined}>","platform:ReadyPlatformV1;hash:(path:string)=>string|undefined;stages?:readonly ReadyStageV1[]}>",1],
 [LIB_+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts","server:boolean;now():number;sleep","server:boolean;stages?:readonly ReadyStageV1[];now():number;sleep",1],
 [LIB_+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts","const names=READY_STAGES_V1.map(stage=>stage.name);","const plan=ports.stages??READY_STAGES_V1,names=plan.map(stage=>stage.name);",1],
 [LIB_+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts","const row=READY_STAGES_V1[decision.stageIndex]!","const row=plan[decision.stageIndex]!",1],
 [LIB_+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts","digests:async()=>readyDigestsV1({platform:readyPlatformV1(),hash:hashOf})","digests:async()=>readyDigestsV1({platform:readyPlatformV1(),hash:hashOf,...input.stages===undefined?{}:{stages:input.stages}})",1],
 [LIB_+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts","work?:{busy:boolean;restartPending:boolean;wake?:()=>void};say(line:string):void}>):ReadyPortsV1{","stages?:readonly ReadyStageV1[];work?:{busy:boolean;restartPending:boolean;wake?:()=>void};say(line:string):void}>):ReadyPortsV1{",1],
 [LIB_+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts","server:isServer(input.origin,input.group),now:Date.now","server:isServer(input.origin,input.group),...input.stages===undefined?{}:{stages:input.stages},now:Date.now",1],
 [LIB_+"🏗️build/🏡️resident/🧬️schema/🔣️.json","\"host-plan\": \"Planning {targets} targets.\",","\"host-plan\": \"Planning {targets} targets.\",\n      \"host-plan-roster\": \"Planning every target of the workspace.\",",1],
 [LIB_+"🏗️build/🏡️resident/🧬️schema/🔣️.json","\"host-plan\": \"Plane {targets} Ziele.\",","\"host-plan\": \"Plane {targets} Ziele.\",\n      \"host-plan-roster\": \"Plane jedes Ziel des Arbeitsbereichs.\",",1],

 [LIB_+"🏗️build/🕸️graph/📥️load/🟦️.ts","if(options.selection.kind===\"all\")return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));","if(options.selection.kind===\"all\")return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));\n    if(graphSliceReadSelectedV1())return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,{...options,baseline:true}));",1],
 [LIB_+"🏗️build/🕸️graph/📥️load/🟦️.ts","async function loadBuildGraphOnceV1(","/** 🔪️ Whether a selection other than `all` is read over its owner closure alone, which is the behaviour of a selected read; `SEMIO_BUILD_GRAPH_SLICE_READ=generation` restores the cut from the generation of the whole workspace (staged rollout only). */\nexport function graphSliceReadSelectedV1(environment:Readonly<Record<string,string|undefined>>=process.env):boolean{return environment.SEMIO_BUILD_GRAPH_SLICE_READ!==\"generation\";}\n\nasync function loadBuildGraphOnceV1(",1],

];

const LIT=/^(["'])(\.cache|\.nx)(?:\/[^"']*)?\1$/u,FN=/(?<![\w$.])((?:\w+\.)?(?:join|resolve))\(/gu;
export function* patchOnlyFiles(){for(const s of new Set(PATCHES.map(p=>p[0])))if(!/\.(ts|mjs)$/u.test(s))yield join(FRAMEWORK,s);}
export function* files(){const stack=[...SCOPES];while(stack.length){const d=stack.pop();for(const name of readdirSync(d)){if(SKIP.has(name))continue;const p=join(d,name);let s;try{s=statSync(p);}catch{continue;}if(s.isDirectory())stack.push(p);else if(/\.(ts|mjs)$/u.test(name)&&!/\.d\.m?ts$/u.test(name))yield p;}}}
function splitArgs(t,start){let i=start,depth=0,cur=start,q=null;const args=[];while(i<t.length){const c=t[i];if(q){if(c==="\\"){i+=2;continue;}if(c===q)q=null;else if(q==="`"&&c==="$"&&t[i+1]==="{"){let d=1;i+=2;while(i<t.length&&d){d+=(t[i]==="{")-(t[i]==="}");i++;}continue;}}else if(c==='"'||c==="'"||c==="`")q=c;else if("([{".includes(c))depth++;else if(")]}".includes(c)){if(depth===0){args.push([cur,i]);return args;}depth--;}else if(c===","&&depth===0){args.push([cur,i]);cur=i+1;}i++;}return null;}
export function sites(t){const out=[];for(const m of t.matchAll(FN)){const args=splitArgs(t,m.index+m[0].length);if(!args||args.length<2)continue;const a1=t.slice(args[1][0],args[1][1]).trim();if(!LIT.test(a1))continue;const a0=t.slice(args[0][0],args[0][1]).trim();out.push({start:m.index,fn:m[1],a0,a1,rootLike:ROOT_LIKE.has(a0)&&(/join$/u.test(m[1])||(/resolve$/u.test(m[1])&&a0==="root"))});}return out;}
export function importLine(file){let rel=relative(dirname(file),RESOLVER).split(sep).join("/");if(!rel.startsWith("."))rel="./"+rel;return `import{machinePathV1}from"${rel}";`;}
export function transform(file,text){if(file===RESOLVER)return text;let out=text;for(const[suffix,from,to,count]of PATCHES)if(file.endsWith(suffix)&&out.includes(from)){MATCHED.add(suffix+from);const parts=out.split(from);if(parts.length-1!==count)throw Error("patch count "+(parts.length-1)+" != "+count+" in "+suffix+": "+from);out=parts.join(to);}const found=sites(out).filter(s=>s.rootLike);if(!found.length&&out===text)return text;for(const s of found.sort((a,b)=>b.start-a.start))out=out.slice(0,s.start)+"machinePathV1"+out.slice(s.start+s.fn.length);if(!out.includes("machinePathV1"))return out;if(!/import\s*\{[^}]*\bmachinePathV1\b/u.test(out)){const m=/^import[\s{]/mu.exec(out);const line=importLine(file);out=m?out.slice(0,m.index)+line+"\n"+out.slice(m.index):line+"\n"+out;}return out;}
if(import.meta.main){const mode=process.argv[2]??"list";let total=0,changed=0;for(const f of files()){const t=readFileSync(f,"utf8"),found=sites(t);if(!found.length)continue;const ok=found.filter(s=>s.rootLike).length;total+=ok;if(mode==="list"){for(const s of found)if(!s.rootLike)console.log("SKIP",f.replace(FRAMEWORK,""),s.fn,s.a0,s.a1);}if(mode==="apply"){const o=transform(f,t);if(o!==t){writeFileSync(f,o);changed++;}}}if(mode==="apply")for(const f of patchOnlyFiles()){const t=readFileSync(f,"utf8"),o=transform(f,t);if(o!==t){writeFileSync(f,o);changed++;}}console.log(JSON.stringify({sites:total,changed}));}
