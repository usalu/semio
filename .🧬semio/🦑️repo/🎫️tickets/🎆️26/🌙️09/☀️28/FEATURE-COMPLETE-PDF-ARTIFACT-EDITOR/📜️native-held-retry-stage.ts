/** ♻️ Stages native Cargo retry against actual broker failure and exact rustc dependency bytes. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const state=join(import.meta.dir,"🗑️generated","native-held-retry");
const paths={"loader":"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts","fch":"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🔁️failure-churn/🟦️.ts"};
const rows:{path:string;before:string;after:string}[]=[];
const replace=(source:string,a:string,b:string)=>{
  assert.equal(source.split(a).length,2,a);
  return source.replace(a,b);
};
const replaceSpan=(source:string,a:string,b:string,text:string)=>{
  const first=source.indexOf(a),last=source.indexOf(b,first+1);
  assert.ok(first>=0&&last>first,a+" "+b);
  return replace(source,source.slice(first,last),text);
};
const record=(path:string,patch:(source:string)=>string)=>{
  const source=readFileSync(path,"utf8");
  const before=createHash("sha256").update(source).digest("hex");
  const after=patch(source);
  assert.notEqual(source,after,path);
  rows.push({path,before,after});
};

record(paths.fch,source=>{
  const index=source.indexOf("  const inTree =");
  const finish=source.indexOf("\n}\n",index);
  assert.ok(index>=0&&finish>index);
  const old=source.slice(index,finish);
  let reused=old.slice(old.indexOf("  const inTree"),old.indexOf("  const artifacts = new Map"));
  const scan=old.slice(old.indexOf("  const seenDeps"));
  const deps=old.slice(old.indexOf('    for (const file'),old.indexOf('    artifacts.set(row.identity, paths);'))
    .replace("row.filenames ?? []","input.filenames");
  reused+= "  const paths = new Set<string>();\n"+deps;
  reused+=replace(scan,"for (const identity of dependencies) for (const dep of artifacts.get(identity) ?? [])","for (const dep of paths)");
  const delegate=`  const byIdentity = new Map<string, string[]>();
  for (const row of input.artifacts) {
    if (!dependencies.has(row.identity)) continue;
    const files = byIdentity.get(row.identity) ?? [];
    files.push(...row.filenames ?? []);
    byIdentity.set(row.identity, files);
  }
  const filenames = [...dependencies].flatMap(identity => byIdentity.get(identity) ?? []);
  return failedCompilerArtifactDriftV1({ ...input, filenames });`;
  const addition=`

/** 🔬️ Verifies one former physical rustc read against the corresponding bounded live source. */
export async function failedCompilerArtifactDriftV1(input: Readonly<{
  workspace: string; build: string; target: string; filenames: readonly string[];
  read: (path: string) => Promise<Uint8Array | undefined>;
}>): Promise<Readonly<{ path: string; expected: string; observed: string }> | undefined> {
`+reused+`
}

/** 🧾️ Recognizes relevant rustc diagnostic headers without consuming or rewriting Cargo output. */
export function cargoDependencyDiagnosticV1(line: string): boolean {
  const display = line.replace(/\\u001b\\[[0-9;]*m/gu, "");
  return policy.dependencyErrorCodes.some(code => display.startsWith("error[" + code + "]:"));
}
`;
  const changed=replace(source,old,delegate);
  const anchor="/** 🕸️ Admits the locked retry selection";
  return replace(changed,anchor,addition+"\n"+anchor);
});

record(paths.loader,source=>{
  const imports=source.split("\n");
  const owner=imports.find(line=>line.includes("import {readPhysicalCargoSourceV1}"))?.split(" from ")[1]?.replace(/\\?[";]+$/g,"");
  assert.ok(owner);
  const shared=owner.replace(/^"/u,"").split("/📥️source/")[0];
  const commandBase=shared.split("/📦️artifacts/")[0];
  const componentPass=shared.replace("/📦️packages/🟦️typescript","")+"/🧩️component-pass/🔁️failure-churn";
  const newImports=[
    'import {admitWarmCargoRetryGraphV1,cargoDependencyDiagnosticV1,failedCompilerArtifactDriftV1,runCargoPassAttemptsV1} from "'+componentPass+'/🟦️.ts";',
    'import failureChurn from "'+componentPass+'/🔣️.json" with {type:"json"};',
    'import {readPhysicalFileV1} from "'+shared.split("/🏃️process/")[0]+"/📁️filesystem/🧾️observation/🟦️.ts"+'";',
  ].join("\n");
  let next=replace(source,'import schema from "./🧬️schema/🔣️.json" with {type:"json"};',newImports+'\nimport schema from "./🧬️schema/🔣️.json" with {type:"json"};');
  next=replace(next,
    'type Execution=Readonly<{kind:"capture";result:OwnedCommandCaptureV1;full:boolean}>|Readonly<{kind:"stream";status:number;full:boolean}>;',
    'type Execution=(Readonly<{kind:"capture";result:OwnedCommandCaptureV1}>|Readonly<{kind:"stream";status:number}>)&Readonly<{full:boolean;dependencyDiagnostic:boolean}>;');
  next=replace(next,'full:!success&&gcOutOfSpaceV1(result.stderr+"\\n"+result.stdout)','full:!success&&gcOutOfSpaceV1(result.stderr+"\\n"+result.stdout),dependencyDiagnostic:!success&&result.stderr.split("\\n").some(cargoDependencyDiagnosticV1)');
  next=replace(next,'const retryingFullImage=(run:()=>Promise<Execution>):Promise<Execution>=>','const retryingFullImage=<Result extends Execution>(run:()=>Promise<Result>):Promise<Result>=>');
  next=replace(next,'let status:number,full=false;try{','let status:number,full=false,dependencyDiagnostic=false;try{');
  next=replace(next,'if(!full&&gcOutOfSpaceV1(line))full=true;','if(!full&&gcOutOfSpaceV1(line))full=true;if(!dependencyDiagnostic&&cargoDependencyDiagnosticV1(line))dependencyDiagnostic=true;');
  next=replace(next,'{kind:"stream",status,full:status!==0&&full}','{kind:"stream",status,full:status!==0&&full,dependencyDiagnostic}');
  next=replace(next,'beforeConsume?:()=>void,generatedFile?:string):Promise<number|T>=>','readmit:(retryControl:ResourceLeaseControl,suffix:string)=>Promise<unknown>,beforeConsume?:()=>void,generatedFile?:string):Promise<number|T>=>');
  const execStart='const execution=await retryingFullImage(()=>jobserver.withClient(inherited=>execute(compilerArgs,';
  const execTail='},cpuControl)));return()=>consumeExecution(execution,env,control,beforeConsume,generatedFile);}';
  next=replace(next,execStart,'const runAttempt=()=>jobserver.withClient(inherited=>execute(compilerArgs,');
  next=replace(next,execTail,'},cpuControl));\n       '+"const compile=async()=>{\n        let recoveredStorage=false;\n        const result=await retryingFullImage(async()=>{\n         const started=broker.units().length;\n         const execution=await runAttempt();\n         recoveredStorage ||= execution.full;\n         return {...execution,units:broker.units().slice(started)};\n        });\n        const {units,...execution}=result;\n        return {execution,units,recoveredStorage};\n       };\n       const {execution}=await runCargoPassAttemptsV1(compile,async({execution,units,recoveredStorage})=>{\n        const code=execution.kind===\"stream\"?execution.status:execution.result.signal===null?execution.result.status:null;\n        if(code!==101||recoveredStorage||execution.full||!execution.dependencyDiagnostic||units.some(unit=>unit.exit>=128))return;\n        const failed=units.filter(unit=>unit.exit!==0&&!unit.cache);\n        if(failed.length!==1||failed[0]!.exit!==1)return;\n        checkResourceLeaseControl(cpuControl);await store.storage.recheck();\n        const filenames=failed[0]!.dependencies.flatMap(edge=>edge.artifacts);\n        if(filenames.length===0)return;\n        const probeDeadline=performance.now()+failureChurn.maximumProbeMilliseconds;\n        const read=async(path:string)=>{\n         const remainingMs=()=>Math.min(probeDeadline-performance.now(),checkResourceLeaseControl(cpuControl)??Infinity);\n         if(remainingMs()<=0)return;\n         try{return (await readPhysicalFileV1(path,{maxBytes:failureChurn.maximumReadBytes,chunkBytes:failureChurn.readChunkBytes,maxWork:Math.ceil(failureChurn.maximumReadBytes/failureChurn.readChunkBytes)+5,cancelled:()=>cpuControl.signal.aborted,remainingMs,onProgress:()=>{}})).bytes;}\n         catch{checkResourceLeaseControl(cpuControl);return;}\n        };\n        return failedCompilerArtifactDriftV1({workspace,build:selected.build,target:selected.target,filenames,read});\n       },async(drift,suffix)=>{\n        await readmit(cpuControl,suffix);\n        spaceReport(spaceLanguage===\"de\"?`[cargo] Wiederholung im gehaltenen Compilerbereich nach Quelltextänderung: ${drift.path}`:`[cargo] Retrying in the held compiler lane after a source change: ${drift.path}`);\n       });\n       "+'return()=>consumeExecution(execution,env,control,beforeConsume,generatedFile);}');
  return replace(next,"mark(\"graph\",\"begin\");const graph=await captureCargoCompilationGraphV1({workspaceRoot:workspace,cwd,program:\"cargo\",prefix:[],args:compilerArgs,environment:compilerEnvironment,maximumOutputBytes:cargoPlanningOutputBytesV1},control,repositoryCargoPlanningPortV1(store));mark(\"graph\",\"end\");mark(\"source\",\"begin\");await admitCargoSourceBoundariesV1(graph,workspace,store.sourceBoundary,control,actualCargoSourceAdmissionPortV1(compilerEnvironment,cwd,store.storage),repositoryCargoSourceMemoV1(store));mark(\"source\",\"end\");await store.storage.recheck();\n    try{return await admitted(compilerArgs,compilerEnvironment,selected,control,","const plan={workspaceRoot:workspace,cwd,program:\"cargo\",prefix:[],args:compilerArgs,environment:compilerEnvironment,maximumOutputBytes:cargoPlanningOutputBytesV1};\n     mark(\"graph\",\"begin\");const graph=await captureCargoCompilationGraphV1(plan,control,repositoryCargoPlanningPortV1(store));mark(\"graph\",\"end\");\n     const admitSource=async(selectedGraph:typeof graph,sourceControl:ResourceLeaseControl)=>{\n      mark(\"source\",\"begin\");await admitCargoSourceBoundariesV1(selectedGraph,workspace,store.sourceBoundary,sourceControl,actualCargoSourceAdmissionPortV1(compilerEnvironment,cwd,store.storage),repositoryCargoSourceMemoV1(store));mark(\"source\",\"end\");await store.storage.recheck();\n     };\n     await admitSource(graph,control);\n     const readmit=async(retryControl:ResourceLeaseControl,suffix:string)=>{\n      let planned=graph;\n      await admitWarmCargoRetryGraphV1({manifest:inputs.manifest,suffix,previous:graph,observe:async()=>planned=await captureCargoCompilationGraphV1(plan,retryControl,actualCargoCompilerPlanningPortV1()),admit:()=>admitSource(planned,retryControl),replacementFits:()=>false});\n     };\n     try{return await admitted(compilerArgs,compilerEnvironment,selected,control,readmit,");
});

mkdirSync(state,{recursive:true});
writeFileSync(join(state,"overlay.json"),JSON.stringify(rows,null,2)+"\n");
console.log("[DEBUG] Staged "+rows.length+" native held-retry paths");
