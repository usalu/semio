const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PNDIR=L+"🕸️graph/📥️inference/📦️project-nodes/";
export const RP=L+"👁️observation/🚧️read-parent/🟨️.mjs",RS=L+"👁️observation/🚧️read-parent/🧬️schema/🔣️.json",OB=L+"👁️observation/🟨️.mjs",OD=L+"🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs",SV=PNDIR+"🏊️project-farm/👷️service/🟨️.mjs",PN=PNDIR+"🟨️.mjs";
const edit=(from,to)=>text=>{const at=text.indexOf(from);if(at<0)throw Error("anchor missing: "+from.slice(0,90));if(text.indexOf(from,at+1)>=0)throw Error("anchor not unique: "+from.slice(0,90));return text.replace(from,()=>to);};
const bilingual=(en,de)=>JSON.stringify(en+" / "+de);
export const all=[
 {file:RS,apply:edit('"const": ["infer", "walk", "resolve", "derive", "refs"]','"const": ["infer", "walk", "resolve", "derive", "refs", "observe"]')},
 {file:RP,apply:edit("/** ⏳️ The line a queued job says while no child can take it, in both languages. */",
`const looks = new AsyncLocalStorage(), observed = { computations: 0, records: 0 };

/** 🔭️ Runs \`body\` as one look of the take pass: while the running flow is in the parent of a read that has a farm, every observed computation and every observed record inside it is counted ({@link readParentObservedCountsV1}); a look is a job of a farm child, so the count must stay zero.
 * @see ../../🕸️graph/📥️inference/📦️project-nodes/🔭️look-queue/🧪️tests/🟦️.ts */
export function withTakeLookScopeV1(body) { return looks.run(true, body); }

/** 🔭️ Counts one observed \`computation\` (\`memoizedObservedComputationV1\`: a directory visit, a closure walk) or \`record\` (a recorded file-system fact such as \`existsSync\`) that runs in the parent of a read inside a look of the take pass; nothing otherwise. */
export function noteObservedInReadParentV1(kind) { if (looks.getStore() === true && readParentV1()) observed[kind]++; }

/** 🔭️ How many observed computations and records ran in the parent of this process inside looks of the take pass so far. */
export function readParentObservedCountsV1() { return Object.freeze({ ...observed }); }

/** ⏳️ The line a queued job says while no child can take it, in both languages. */`)},
 {file:OB,apply:edit('import{fieldFactV1,fieldValueV1,installedPackageFileV1,jsonPointerValueV1,registerFieldExtractorV1}from"./🧩️field/🟨️.mjs";','import{fieldFactV1,fieldValueV1,installedPackageFileV1,jsonPointerValueV1,registerFieldExtractorV1}from"./🧩️field/🟨️.mjs";\nimport{noteObservedInReadParentV1}from"./🚧️read-parent/🟨️.mjs";')},
 {file:OB,apply:edit("export function memoizedObservedComputationV1(cache,key,compute) {\n const parent=scopes.getStore(),previous=cache.get(key);","export function memoizedObservedComputationV1(cache,key,compute) {\n noteObservedInReadParentV1(\"computations\");const parent=scopes.getStore(),previous=cache.get(key);")},
 {file:OB,apply:edit("function record(kind, path, fact) {\n  footprintPulseV1();","function record(kind, path, fact) {\n  footprintPulseV1();noteObservedInReadParentV1(\"records\");")},
 {file:OD,apply:edit('import {ROSTER_RULES_V1} from "../../🧭️roster/🟨️.mjs";','import {ROSTER_RULES_V1} from "../../🧭️roster/🟨️.mjs";\nimport {assertNotReadParentV1} from "../../../👁️observation/🚧️read-parent/🟨️.mjs";')},
 {file:OD,apply:edit("export function projectAssemblyAuthorityV1(workspaceRoot,nativeRoot,projectsByRoot,componentRoots,nativeCache=new Map(),nativeClosures=new Map()){\n let nativeOwners;","export function projectAssemblyAuthorityV1(workspaceRoot,nativeRoot,projectsByRoot,componentRoots,nativeCache=new Map(),nativeClosures=new Map()){\n assertNotReadParentV1(\"observe\",nativeRoot??\"release pipeline\");let nativeOwners;")},
 {file:SV,apply:edit('import { createProjectInferenceEnvironmentV1, observeProjectRecordV1, produceProjectRecordV1, warmProjectRecordV1 } from "../../🟨️.mjs";','import { createProjectInferenceEnvironmentV1, lookOwnersV1, observeProjectRecordV1, produceProjectRecordV1, warmProjectRecordV1 } from "../../🟨️.mjs";')},
 {file:SV,apply:edit('  if (job.observe === true) return stagedWithinV1(join(staging, "observe"), () => observeProjectRecordV1(environment, job));','  if (job.look === true) return stagedWithinV1(join(staging, "look"), () => lookOwnersV1(environment, job));\n  if (job.observe === true) return stagedWithinV1(join(staging, "observe"), () => observeProjectRecordV1(environment, job));')},
 {file:PN,apply:edit('import { assertNotReadParentV1, withReadParentScopeV1 } from "../../../../🏗️build/👁️observation/🚧️read-parent/🟨️.mjs";','import { assertNotReadParentV1, readParentObservedCountsV1, withReadParentScopeV1, withTakeLookScopeV1 } from "../../../../🏗️build/👁️observation/🚧️read-parent/🟨️.mjs";\nimport { createLookQueueV1 } from "./🔭️look-queue/🟨️.mjs";')},
 {file:PN,apply:edit('de: "wartet auf einen Farm-Kindprozess, der die Closure-Referenzen auflöst (der Elternprozess löst nichts auf)" };','de: "wartet auf einen Farm-Kindprozess, der die Closure-Referenzen auflöst (der Elternprozess löst nichts auf)" }, STAGE_LOOK_V1 = { en: "waiting for a farm child to look at the owner: assembly authority, stored record and closure references (the parent computes nothing)", de: "wartet auf einen Farm-Kindprozess, der das Projekt prüft: Zusammenbau-Autorität, gespeicherter Eintrag und Closure-Referenzen (der Elternprozess rechnet nichts)" };')},
 {file:PN,apply:edit("/** ⏳️ How long a reader waits for the verdict of a farm child before it verifies the record itself",
`/** 🔭️ Looks at the owners of one batch in a farm child, the only place a look runs while a farm exists: for each owner the child derives what the parent used to derive (native or release preparation, the assembly authority, the head of the record key) from the global parts the parent sent, verifies the stored record and resolves the closure references, and answers per owner \`{ name, native, pipeline, head, verdict, headerDigest, refs }\`; an owner it could not look at is answered \`{ name, failed }\` and does not fail the others of its batch. The reply carries a small \`weight\` per owner.
 * @see ./🔭️look-queue/🧪️tests/🟦️.ts */
export async function lookOwnersV1(environment, job) {
  const cpu = process.cpuUsage(), startRss = process.memoryUsage().rss, startPeak = process.resourceUsage().maxRSS * 1024, looks = [];
  for (const owner of job.owners) {
    try {
      const prepared = await observeInferenceSnapshotV1(() => ownerJobV1(environment, job.globals, owner));
      const { names, cpuMilliseconds, peakMegabytes, weight, ...said } = await observeProjectRecordV1(environment, { ...prepared, cachePath: owner.cachePath, closureStore: job.closureStore, verdicts: job.verdicts, refs: job.refs });
      looks.push({ name: owner.name, native: prepared.native, pipeline: prepared.pipeline, head: prepared.head, ...said });
    } catch (error) { looks.push({ name: owner.name, failed: String(error?.message ?? error).replace(/\\s+/gu, " ").slice(0, 300) }); }
  }
  const used = process.cpuUsage(cpu), endPeak = process.resourceUsage().maxRSS * 1024, top = Math.max(process.memoryUsage().rss, endPeak > startPeak ? endPeak : 0);
  return { names: [], looks, cpuMilliseconds: Math.round((used.user + used.system) / 1000), peakMegabytes: Math.max(0, Math.round((top - startRss) / 1048576)), weight: 0.01 * job.owners.length };
}

/** ⏳️ How long a reader waits for the verdict of a farm child before it verifies the record itself`)},
 {file:PN,apply:edit('    const started=performance.now();\n    await yieldInferenceV1(context,"build:projects",++projectIndex);','    let looked;if(lookQueue!==undefined){stageV1(name,STAGE_LOOK_V1);try{looked=await lookQueue.ask(name);}finally{stageV1(name,STAGE_INFER_V1);}}\n    const started=performance.now();\n    await yieldInferenceV1(context,"build:projects",++projectIndex);')},
 {file:PN,apply:edit('nativeCandidate=existsSync(join(workspaceRoot,nativeRoot,"Cargo.toml"))||nativeTargets.some(','nativeCandidate=looked!==undefined?looked.native:existsSync(join(workspaceRoot,nativeRoot,"Cargo.toml"))||nativeTargets.some(')},
 {file:PN,apply:edit('pipeline=Object.values(activeJson.targets??{}).some(target=>target.metadata?.semio?.release);\n    const assemblyAuthority=native||pipeline?','pipeline=looked!==undefined?looked.pipeline:Object.values(activeJson.targets??{}).some(target=>target.metadata?.semio?.release);\n    const assemblyAuthority=looked!==undefined?undefined:native||pipeline?')},
 {file:PN,apply:edit('head=[projectAuthority,registryDigest,native?commandRevision:null,native||pipeline?assemblyAuthority:null,','head=looked!==undefined?looked.head:[projectAuthority,registryDigest,native?commandRevision:null,native||pipeline?assemblyAuthority:null,')},
 {file:PN,apply:edit('verdict:observeJob===undefined?undefined:({headerDigest})=>{seenHeader=headerDigest;stageV1(name,STAGE_VERDICT_V1);','verdict:observeJob===undefined?undefined:({headerDigest})=>{seenHeader=headerDigest;if(looked?.verdict!==undefined&&looked.headerDigest===headerDigest){if(looked.verdict==="holds"&&looked.refs!==undefined&&looked.refs.failed===undefined)offered={refs:looked.refs,headerDigest};return Promise.resolve(looked);}stageV1(name,STAGE_VERDICT_V1);')},
 {file:PN,apply:edit('lookProject=inferProject,takeAway=Object.assign(()=>{throw Object.assign(Error("not stored here or in the fleet cache"),{code:"BUILD_TAKE_MISS"});},{take:true});let poll=Promise.resolve();',
 'lookProject=inferProject,takeAway=Object.assign(()=>{throw Object.assign(Error("not stored here or in the fleet cache"),{code:"BUILD_TAKE_MISS"});},{take:true});let poll=Promise.resolve();\n  const lookQueue=farm===undefined||options?.ownerAdmission?undefined:createLookQueueV1({farm,jobOf:name=>{const job=byName.get(name);return{name,root:job.json.root,json:job.json,cacheKey:ownerCacheKeyV1(job.json.root),cachePath:ownerCachePathV1(workspaceRoot,job.json.root)};},shared:()=>({globals:{projectAuthority,registryDigest,commandRevision,late},closureStore:closureStoreDirectoryV1(workspaceRoot),verdicts:closureVerdictScopeV1(),refs:late&&options?.rowLog!==undefined&&closureVerdictScopeV1()!==undefined}),refuse:(name,reason)=>farmJobRefusedV1(name,byName.get(name)?.json.root,"the look ("+reason+")"),say:text=>takeSay(text)});\n  farm?.onReady(()=>lookQueue?.kick());')},
 {file:PN,apply:edit('try{const outcome=await takeScope.run(async looked=>{try{await runClosureTakeV1(()=>lookProject(job,takeAway));}','try{const outcome=await takeScope.run(async looked=>{try{await withTakeLookScopeV1(()=>runClosureTakeV1(()=>lookProject(job,takeAway)));}')},
 {file:PN,apply:edit('      await Promise.all(takeLanesV1(split.light,name=>present.has(name)).map(lane=>pass(lane,takeConcurrencyV1(graphCutConcurrencyV1(process.env)))));\n      await pass(split.heavy,1);\n      reportProgress(finished);',
 '      const lanes=takeLanesV1(split.light,name=>present.has(name));lookQueue?.start([...lanes,split.heavy]);\n      await Promise.all(lanes.map(lane=>pass(lane,takeConcurrencyV1(graphCutConcurrencyV1(process.env)))));\n      await pass(split.heavy,1);\n      reportProgress(finished);\n      {const seen=readParentObservedCountsV1();if(seen.computations+seen.records>0)takeSay(()=>"the parent ran "+seen.computations+" observed computations and "+seen.records+" observed records inside looks, a farm child should / der Elternprozess führte "+seen.computations+" beobachtete Berechnungen und "+seen.records+" beobachtete Einträge in Prüfungen aus, ein Farm-Kindprozess sollte");}')},
];
export const apply=(file,text,list=all)=>{for(const item of list)if(item.file===file)text=item.apply(text);return text;};
