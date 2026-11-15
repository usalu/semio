"""R36 part 3b: slow synchronous stretches name themselves in the trace (the TypeScript programs and the module globs of the script inputs)."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
FM=B+"🧠️memory/🦶️footprint/🟨️.mjs"
add(FM,'let installed,step,pulser,','''let slowTrace;
/** ⏱️ Names a slow stretch in the trace of this process: `label` and the milliseconds since `startedAt`, written when they reach `thresholdMilliseconds` (the pulse cannot speak inside a stretch that calls no instrumented function, so the code that is known to be heavy reports itself afterwards). */
export function noteSlowV1(label,startedAt,thresholdMilliseconds=2000,trace=(slowTrace??=createFootprintTraceV1())){
 const milliseconds=Date.now()-startedAt;
 if(milliseconds>=thresholdMilliseconds)trace("slow "+label+" "+milliseconds+" ms");
 return milliseconds;
}

let installed,step,pulser,''')
add(B+"🧠️memory/🦶️footprint/🟨️.d.mts",'export function footprintPulseV1():void;','export function footprintPulseV1():void;\nexport function noteSlowV1(label:string,startedAt:number,thresholdMilliseconds?:number,trace?:(line:string)=>void):number;')
SI=B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
add(SI,'footprintGateV1, decideFootprintV1 } from','footprintGateV1, decideFootprintV1, noteSlowV1 } from')
add(SI,'''  const compiler = parserTypescriptV1();
  return declaredTypeScriptProjectsV1(projects, workspaceRoot, projectRoot, { compiler, host: observedTypeScriptHost(compiler.sys), readdir: declaredDirectoryV1, join, dirname, isAbsolute, reuse: activeTypeScriptProgramReuseV1() ?? sharedSourceFilesV1 });''','''  const compiler = parserTypescriptV1(), startedAt = Date.now();
  try { return declaredTypeScriptProjectsV1(projects, workspaceRoot, projectRoot, { compiler, host: observedTypeScriptHost(compiler.sys), readdir: declaredDirectoryV1, join, dirname, isAbsolute, reuse: activeTypeScriptProgramReuseV1() ?? sharedSourceFilesV1 }); }
  finally { noteSlowV1("typescript-program " + projectRoot + " " + JSON.stringify(projects).slice(0, 160), startedAt); }''')
add(SI,'() => declaredModuleGlobsV1(globs, workspaceRoot, projectRoot, { readdir: declaredDirectoryV1, Glob: GlobV1, join }).map(path => resolve(workspaceRoot, path)));','() => { const startedAt = Date.now(); try { return declaredModuleGlobsV1(globs, workspaceRoot, projectRoot, { readdir: declaredDirectoryV1, Glob: GlobV1, join }).map(path => resolve(workspaceRoot, path)); } finally { noteSlowV1("module-globs " + projectRoot, startedAt); } });')
TF=B+"🧠️memory/🦶️footprint/🧪️tests/🟦️.ts"
add(TF,'import{releaseIfGrownV1,','import{noteSlowV1,releaseIfGrownV1,')
add(TF,'//R36b\n','''//R36b
test("a slow stretch names itself in the trace, a fast one does not",()=>{
 const lines:string[]=[];
 expect(noteSlowV1("fast",Date.now(),2000,line=>lines.push(line))).toBeLessThan(2000);
 const spent=noteSlowV1("typescript-program x",Date.now()-2500,2000,line=>lines.push(line));
 expect(spent).toBeGreaterThanOrEqual(2500);expect(lines.length).toBe(1);expect(lines[0]).toMatch(/^slow typescript-program x 2[5-9]\\d\\d ms$/u);
});
''')
PN=B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
add(PN,'import { useEnvironmentGateV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";','import { useEnvironmentGateV1, noteSlowV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";')
add(PN,'const resolved=await tagProjectFailureV1(root,()=>resolveClosureRefsV1(result.projects[name],workspaceRoot,scripts,"hash",options.rowLog));','const resolved=await tagProjectFailureV1(root,async()=>{const startedAt=Date.now();try{return await resolveClosureRefsV1(result.projects[name],workspaceRoot,scripts,"hash",options.rowLog);}finally{noteSlowV1("resolve-closure-refs "+name,startedAt,5000);}});')
add(PN,'result.projects[name]=await tagProjectFailureV1(root,()=>observeProjectInferenceV1(','const inferStartedAt=Date.now();result.projects[name]=await tagProjectFailureV1(root,()=>observeProjectInferenceV1(')
add(PN,'let rows=projectInferenceObservationsV1(cachePath)??[],derived=[];','noteSlowV1("owner-inference "+name,inferStartedAt,10000);let rows=projectInferenceObservationsV1(cachePath)??[],derived=[];')
