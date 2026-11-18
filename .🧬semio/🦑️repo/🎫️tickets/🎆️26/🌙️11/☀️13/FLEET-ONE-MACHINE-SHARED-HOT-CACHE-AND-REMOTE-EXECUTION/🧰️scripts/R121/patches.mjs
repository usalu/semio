const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/";
export const SI=L+"🧭️script-inputs/🟨️.mjs",AS=L+"🧱️project-assembly/🟨️.mjs",ASD=L+"🧱️project-assembly/🟨️.d.mts",PN=L+"📦️project-nodes/🟨️.mjs",SV=L+"📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs";
const edit=(from,to)=>text=>{const at=text.indexOf(from);if(at<0||text.indexOf(from,at+1)>=0)throw Error("anchor not unique: "+from.slice(0,70));return text.replace(from,()=>to);};
const bt="`";
const siPlan=`      return { values, rows: builder ? builder.finish() : rows, derived: [...derived.values()] };
    },
    /** 🫗️ The shape of {@link resolve} for a process that writes no row log: the values of the recipes and, instead of rows, the plan a log is made from (the rows of the entries, the digests of the stored layers to add, the digests whose rows a derived fact stands for, the derived facts with the digest of their layer), so that the process that owns the log lists the rows from the closure store without opening one entry. Undefined when a layer is not stored (it has no digest). */
    async plan(recipes) {
      const values = new Map(), derived = new Map(), layers = new Map(), walked = new Set(), entries = [];
      for (const [key, recipe] of recipes) { const entry = await ensureClosureEntryV1(workspaceRoot, recipe), view = walkViewOfRecipeV1(recipe); values.set(key, entry.value); entries.push(entry); for (const layer of closureLayerViewsV1(entry)) { if (typeof layer.digest !== "string") return undefined; layers.set(layer.key, layer); if (layer.fact) { const row = view === undefined ? layer.fact.row : layer.fact.views?.[view] ?? layer.fact.row; derived.set(row.path, { row, digest: layer.digest }); walked.add(layer.fact.row.path); } } }
      const entryRows = []; for (const entry of entries) for (const row of entry.rows) entryRows.push(row);
      return { values, plan: { entryRows, excluded: [...derived.values()].filter(({ row }) => !row.path.startsWith('{"program":')).map(({ digest }) => digest), layers: [...layers.values()].filter(layer => !layer.fact && (!layer.members || !someMemberV1(layer.members, question => walked.has(question)))).map(layer => layer.digest), derived: [...derived.values()] } };
    },
    /** 🫗️ The rows and derived facts of a {@link plan} as lists of ${bt}log${bt}, the layers listed once per log from the stored ids (no entry is loaded, nothing is derived); throws when a stored layer is gone, so the caller resolves itself. */
    adopt(plan, log) {
      const store = closureStoreOfV1(workspaceRoot);
      if (!store) throw refuseBuildV1("invalid", "closure.provider-entry-stale", { port: "layers" });
      const listOf = digest => closureLayerListV1(log, { key: digest, digest, store }), builder = log.builder();
      for (const digest of plan.excluded) builder.exclude(listOf(digest));
      builder.add(plan.entryRows);
      for (const digest of plan.layers) builder.addList(listOf(digest));
      return { rows: builder.finish(), derived: plan.derived.map(({ row, digest }) => ({ row, support: listOf(digest) })) };
    },
`;
const asHead=`  const { values, rows, derived } = await closureApiV1(scripts, workspaceRoot).resolve(Object.entries(project.closureRecipes), log);
  return { project: bindClosureRefsV1(project, values, workspaceRoot, sourceRepresentation), rows, derived };
}

/** 🪅️ The project with every closure reference replaced by the files of its closure and its symbolic groups compacted: the answer of {@link resolveClosureRefsV1} and of {@link resolveClosureRefsPlanV1}, one implementation. */
function bindClosureRefsV1(project, values, workspaceRoot, sourceRepresentation) {
  const root = project.root;
`;
const asTail=`  return mergeCommandSourceGroupsV1(bound);
}

/** 🪅️ Resolves the closure references of a project in a process that writes no row log (a farm child): the resolved project and the plan its rows are listed from (see ${bt}closureApiV1(...).plan${bt}); undefined when a layer of a closure is not stored, so the owner of the log resolves it itself. */
export async function resolveClosureRefsPlanV1(project, workspaceRoot, scripts, sourceRepresentation = "hash") {
  if (!project.closureRecipes) return { project, plan: null };
  const planned = await closureApiV1(scripts, workspaceRoot).plan(Object.entries(project.closureRecipes));
  return planned === undefined ? undefined : { project: bindClosureRefsV1(project, planned.values, workspaceRoot, sourceRepresentation), plan: planned.plan };
}

/** 🪅️ The rows and derived facts of a plan from {@link resolveClosureRefsPlanV1} as lists of ${bt}log${bt}, read from the closure store: nothing is resolved, derived or parsed beyond the ids of the layers, once per log and layer. */
export function adoptClosureRefsV1(plan, workspaceRoot, scripts, log) {
  return plan === null ? { rows: [], derived: [] } : closureApiV1(scripts, workspaceRoot).adopt(plan, log);
}
`;
const pnHelpers=`
/** 🪘️ What the parent of a read did with the closure references of its owners: ${bt}adopted${bt} from a farm child (the child resolved them and filed the result), ${bt}resolved${bt} by the parent itself (no child, no result or a result that did not hold). */
export const ownerRefsCountsV1 = { adopted: 0, resolved: 0 };

/** 🪘️ The file a farm child files the resolved result of one owner in: inside the verdict directory of the read, named by the digest of the header of the record it resolved. */
export const ownerRefsFileV1 = (directory, headerDigest) => join(directory, "refs-" + headerDigest);

/** 🪘️ Resolves the closure references of the stored record whose header digest is ${bt}headerDigest${bt} in this process (a farm child that just verified it) and files the resolved project and the plan of its rows in the verdict directory of the read; answers the reference the parent keeps (path, digest of the file, size, header digest, the process that resolved it) or undefined when the record changed, is withheld, has a layer that is not stored or there is no read directory. The parent streams the file only when it assembles the owner.
 * @see ../🧪️tests/🪘️refs-file */
export async function resolveOwnerRefsV1(environment, job, headerDigest) {
  const directory = closureVerdictScopeV1();
  if (directory === undefined || typeof headerDigest !== "string") return undefined;
  let text; try { text = refsBrotliV1(rawReadFileSync(job.cachePath)).toString("utf8"); } catch { return undefined; }
  const cut = text.indexOf("\\n");
  if (cut < 0 || createHash("sha256").update(text.slice(0, cut)).digest("hex") !== headerDigest) return undefined;
  let value; try { value = JSON.parse(text.slice(cut + 1)); } catch { return undefined; }
  if (value === null || typeof value !== "object" || value.unavailable !== undefined) return undefined;
  const resolved = await resolveClosureRefsPlanV1(value, environment.workspaceRoot, environment.scripts, "hash");
  if (resolved === undefined) return undefined;
  const body = JSON.stringify({ v: 1, headerDigest, resolver: process.pid, project: resolved.project, plan: resolved.plan }), path = ownerRefsFileV1(directory, headerDigest), temporary = path + "." + process.pid + ".tmp";
  rawWriteFileSync(temporary, body); rawRenameSync(temporary, path);
  return { path, digest: createHash("sha256").update(body).digest("hex"), size: Buffer.byteLength(body), headerDigest, resolver: process.pid };
}

/** 🪘️ The resolved result a farm child filed for the record the parent adopted, as the closure resolution of the parent would have answered it: the project and rows and derived facts listed in ${bt}log${bt} from the closure store. Undefined (the parent then resolves itself) when the file is gone, torn or not the one the reference names, when the record it was made from is not the one the parent holds, or when a layer of the store is lost; the file is removed once read.
 * @see ../🧪️tests/🪘️refs-file */
export function adoptOwnerRefsV1(refs, headerDigest, workspaceRoot, scripts, log) {
  if (log === undefined || refs === undefined || refs.headerDigest !== headerDigest) return undefined;
  let body; try { body = rawReadFileSync(refs.path); } catch { return undefined; }
  try { rawRmSync(refs.path, { force: true }); } catch {}
  if (body.length !== refs.size || createHash("sha256").update(body).digest("hex") !== refs.digest) return undefined;
  try { const parsed = JSON.parse(body.toString("utf8")); if (parsed.headerDigest !== headerDigest) return undefined; const adopted = adoptClosureRefsV1(parsed.plan, workspaceRoot, scripts, log); return { project: parsed.project, rows: adopted.rows, derived: adopted.derived, resolver: parsed.resolver }; } catch { return undefined; }
}
`;
export const all=[
 {file:SI,apply:edit(`      return { values, rows: builder ? builder.finish() : rows, derived: [...derived.values()] };
    },
`,siPlan)},
 {file:AS,apply:edit(`  const { values, rows, derived } = await closureApiV1(scripts, workspaceRoot).resolve(Object.entries(project.closureRecipes), log), root = project.root;
`,asHead)},
 {file:AS,apply:edit(`  return { project: mergeCommandSourceGroupsV1(bound), rows, derived };
}
`,asTail)},
 {file:ASD,apply:edit(`export function resolveClosureRefsV1(project: any, workspaceRoot: string, scripts: any, sourceRepresentation?: string, log?: unknown): Promise<any>;
`,`export function resolveClosureRefsV1(project: any, workspaceRoot: string, scripts: any, sourceRepresentation?: string, log?: unknown): Promise<any>;
export function resolveClosureRefsPlanV1(project: any, workspaceRoot: string, scripts: any, sourceRepresentation?: string): Promise<{ project: any; plan: any } | undefined>;
export function adoptClosureRefsV1(plan: any, workspaceRoot: string, scripts: any, log: unknown): { rows: any; derived: any[] };
`)},
 {file:PN,apply:edit(`import { statSync as rawStatSync, readFileSync as rawReadFileSync } from "node:fs";`,`import { statSync as rawStatSync, readFileSync as rawReadFileSync, writeFileSync as rawWriteFileSync, renameSync as rawRenameSync, rmSync as rawRmSync } from "node:fs";
import { brotliDecompressSync as refsBrotliV1 } from "node:zlib";`)},
 {file:PN,apply:edit(`resolveClosureRefsV1, inferenceCandidates`,`resolveClosureRefsV1, resolveClosureRefsPlanV1, adoptClosureRefsV1, inferenceCandidates`)},
 {file:PN,apply:edit(`({ projectWithDefaults, resolveClosureRefsV1 } = registry.projectAssembly);`,`({ projectWithDefaults, resolveClosureRefsV1, resolveClosureRefsPlanV1, adoptClosureRefsV1 } = registry.projectAssembly);`)},
 {file:PN,apply:edit(`  try { verdict = await withClosurePassV1(() => observeInferenceSnapshotV1(() => verifyStoredProjectRecordV1({ cachePath: job.cachePath, key: projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs) })), { verdicts: job.verdicts }); } finally {`,`  let refs;
  try { verdict = await withClosurePassV1(async () => { const said = await observeInferenceSnapshotV1(() => verifyStoredProjectRecordV1({ cachePath: job.cachePath, key: projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs) })); if (job.refs === true && typeof job.verdicts === "string" && said.verdict === "holds") { try { refs = await observeInferenceSnapshotV1(() => resolveOwnerRefsV1(environment, job, said.headerDigest)); } catch { refs = undefined; } } return said; }, { verdicts: job.verdicts }); } finally {`)},
 {file:PN,apply:edit(`  return { names: [], ...verdict, cpuMilliseconds: Math.round((used.user + used.system) / 1000), peakMegabytes: Math.max(0, Math.round((top - startRss) / 1048576)), weight: 0.01 };
}
`,`  return { names: [], ...verdict, ...(refs === undefined ? {} : { refs }), cpuMilliseconds: Math.round((used.user + used.system) / 1000), peakMegabytes: Math.max(0, Math.round((top - startRss) / 1048576)), weight: 0.01 };
}
${pnHelpers}`)},
 {file:PN,apply:edit(`verdict:away.take===true&&farm?.width?.()>0?()=>verdictWithinV1(farm.observe({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey,cachePath,observe:true,closureStore:closureStoreDirectoryV1(workspaceRoot),verdicts:closureVerdictScopeV1()}),OWNER_VERDICT_WAIT_MILLISECONDS_V1):undefined}`,`verdict:away.take===true&&farm?.width?.()>0?({headerDigest})=>verdictWithinV1(farm.observe({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey,cachePath,observe:true,refs:late&&options.rowLog!==undefined&&closureVerdictScopeV1()!==undefined,closureStore:closureStoreDirectoryV1(workspaceRoot),verdicts:closureVerdictScopeV1()}),OWNER_VERDICT_WAIT_MILLISECONDS_V1).then(reply=>{if(reply?.verdict==="holds"&&reply.headerDigest===headerDigest&&reply.refs!==undefined)offered={refs:reply.refs,headerDigest};return reply;}):undefined}`)},
 {file:PN,apply:edit(`const inferStartedAt=Date.now();stageV1(name,STAGE_INFER_V1);`,`let offered;const inferStartedAt=Date.now();stageV1(name,STAGE_INFER_V1);`)},
 {file:PN,apply:edit(`try{return await resolveClosureRefsV1(result.projects[name],workspaceRoot,scripts,"hash",options.rowLog);}finally{`,`try{const adopted=offered===undefined?undefined:adoptOwnerRefsV1(offered.refs,offered.headerDigest,workspaceRoot,scripts,options.rowLog);if(adopted!==undefined){ownerRefsCountsV1.adopted++;return adopted;}ownerRefsCountsV1.resolved++;return await resolveClosureRefsV1(result.projects[name],workspaceRoot,scripts,"hash",options.rowLog);}finally{`)},
];
export const apply=(file,text,list=all)=>{for(const item of list)if(item.file===file)text=item.apply(text);return text;};
