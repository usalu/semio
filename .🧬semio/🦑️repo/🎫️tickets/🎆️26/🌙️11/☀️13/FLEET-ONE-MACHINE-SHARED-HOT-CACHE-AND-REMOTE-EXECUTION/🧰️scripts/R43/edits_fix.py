"""R43 fix 1: the structural tests of the project loop read the source text, so the farm and body calls keep their literal form; the farm wait is timed by a wrapper of the farm and the inline body by the environment it receives."""
import edits as first
L=first.L;B=first.B;G=first.G;M=first.M;F=first.F;P=first.P
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
add(P+"🟨️.mjs",
 'prepare:farm?()=>{missed.add(name);const at=performance.now();return away(()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey})).finally(()=>clock.add(name,"farm",performance.now()-at));}:undefined},()=>{missed.add(name);const at=performance.now();return inferProjectBodyV1(projectEnvironment,{name,root,json,native,pipeline,nativeRoot,selectedTargets,selectedInputs},options?.ownerAdmission).finally(()=>clock.add(name,"inline",performance.now()-at));}));',
 'prepare:farm?()=>{missed.add(name);return away(()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey}));}:undefined},()=>{missed.add(name);return inferProjectBodyV1(projectEnvironment,{name,root,json,native,pipeline,nativeRoot,selectedTargets,selectedInputs},options?.ownerAdmission);}));')
add(P+"🟨️.mjs",
 '  const farm=options?.ownerAdmission?undefined:openProjectFarmV1({',
 '  const farm=timedFarmV1(options?.ownerAdmission?undefined:openProjectFarmV1({')
add(P+"🟨️.mjs",
 'engage:options?.coldInference===true||wantedRoots!==undefined});',
 'engage:options?.coldInference===true||wantedRoots!==undefined}),clock);')
add(P+"🟨️.mjs",
 '/** 🧭️ The order owners start in:',
 '''/** ⏲️ The farm with its wait charged to the owner clock under `farm` (the same farm when there is none). */
export function timedFarmV1(farm, clock) {
  if (farm === undefined) return undefined;
  return Object.freeze({ ...farm, prepare: async job => { const at = performance.now(); try { return await farm.prepare(job); } finally { clock.add(job.name, "farm", performance.now() - at); } } });
}

/** 🧭️ The order owners start in:''')
add(P+"🟨️.mjs",
 'const projectEnvironment = { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot, componentRoots, nativeCache, nativeClosures, context };',
 'const projectEnvironment = { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot, componentRoots, nativeCache, nativeClosures, context, clock };')
add(P+"🟨️.mjs",
 '  const { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot, componentRoots, nativeCache, nativeClosures, context } = environment, { name, root, json, native, pipeline, selectedTargets, selectedInputs } = job;\n  try {',
 '  const { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot, componentRoots, nativeCache, nativeClosures, context } = environment, { name, root, json, native, pipeline, selectedTargets, selectedInputs } = job, began = performance.now();\n  try {')
add(P+"🟨️.mjs",
 'observeWithheldSourceV1(error); return { unavailable }; }\n}',
 'observeWithheldSourceV1(error); return { unavailable }; }\n  finally { environment.clock?.add(name, "inline", performance.now() - began); }\n}')
add(P+"🟨️.mjs",
 'clock.add(name,"observe",Date.now()-inferStartedAt);',
 'clock.add(name,"observe",Math.max(0,Date.now()-inferStartedAt-clock.get(name,"farm")-clock.get(name,"inline")));')
add(F+"🟨️.mjs",
 '  add(name,step,milliseconds){const row=open.get(name);if(row!==undefined)row.steps.set(step,(row.steps.get(step)??0)+milliseconds);},',
 '  add(name,step,milliseconds){const row=open.get(name);if(row!==undefined)row.steps.set(step,(row.steps.get(step)??0)+milliseconds);},\n  get(name,step){return open.get(name)?.steps.get(step)??0;},')
add(F+"🟨️.d.mts",
 'add:(name:string,step:string,milliseconds:number)=>void;finish:',
 'add:(name:string,step:string,milliseconds:number)=>void;get:(name:string,step:string)=>number;finish:')
