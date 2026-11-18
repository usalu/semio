const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const SI=L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs",SID=L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.d.mts",EN=L+"🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs",PN=L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs";
const edit=(from,to)=>text=>{const at=text.indexOf(from);if(at<0||text.indexOf(from,at+1)>=0)throw Error("anchor not unique: "+from.slice(0,70));return text.replace(from,()=>to);};
const helpers=`/** 🗂️ The verdicts of one read, shared by every process of it: the first process that verifies a stored closure layer or entry writes a small verdict file named by its digest into a directory of the validation round it ran in, every other look of that round (the parent, farm children, pool children) reads the file instead of probing the rows again; a round is one snapshot of the tree, so a new round has a new directory and sees an edit at once. Without a round there is no directory and nothing is shared. A child joins the round of its parent with \\\`withClosurePassV1(compute, { verdicts })\\\`.
 * @see ./🧪️tests/🧬️read-verdicts */
const verdictOverrideV1 = new AsyncLocalStorage(), verdictDirsV1 = new WeakMap(), verdictSeenV1 = new Map(), verdictOwnedV1 = new Set(), verdictCountsV1 = { read: 0, written: 0, waited: 0 };
let verdictSerialV1 = 0;
export function closureVerdictScopeV1() {
  const given = verdictOverrideV1.getStore(); if (given !== undefined) return given;
  const round = closurePassV1 ?? observationRoundV1(); if (round === null || typeof round !== "object") return undefined;
  let dir = verdictDirsV1.get(round);
  if (dir === undefined) {
    const parent = join(vTmpV1(), "semio-read-verdicts-" + (process.getuid?.() ?? 0));
    try { vMkdirV1(parent, { recursive: true }); for (const name of vListV1(parent)) { try { if (Date.now() - vStatV1(join(parent, name)).mtimeMs > 7200000) vRmV1(join(parent, name), { recursive: true, force: true }); } catch {} } } catch {}
    dir = join(parent, process.pid + "-" + Date.now().toString(36) + "-" + ++verdictSerialV1); verdictDirsV1.set(round, dir); verdictOwnedV1.add(dir);
  }
  return dir;
}
process.once("exit", () => { for (const dir of verdictOwnedV1) { try { vRmV1(dir, { recursive: true, force: true }); } catch {} } });
const verdictFileV1 = (kind, digest) => { if (typeof digest !== "string") return undefined; const dir = closureVerdictScopeV1(); return dir === undefined ? undefined : join(dir, kind + "-" + digest); };
const readVerdictV1 = file => {
  if (file === undefined) return undefined;
  const seen = verdictSeenV1.get(file); if (seen !== undefined) return seen;
  try { const text = vReadV1(file, "utf8"), holds = text === "1" ? true : text === "0" ? false : undefined; if (holds !== undefined) { if (verdictSeenV1.size > 100000) verdictSeenV1.clear(); verdictSeenV1.set(file, holds); } return holds; } catch { return undefined; }
};
const verdictDirsMadeV1 = new Set();
const verdictDirMadeV1 = dir => { if (verdictDirsMadeV1.has(dir)) return; vMkdirV1(dir, { recursive: true }); verdictDirsMadeV1.add(dir); };
const writeVerdictV1 = (file, holds) => {
  if (file === undefined) return;
  try { verdictDirMadeV1(dirname(file)); vWriteV1(file, holds ? "1" : "0"); verdictCountsV1.written++; verdictSeenV1.set(file, holds); } catch {}
};
async function sharedVerdictV1(file, compute) {
  if (file === undefined) return compute();
  const known = readVerdictV1(file); if (known !== undefined) { verdictCountsV1.read++; return known; }
  let mine = false; try { verdictDirMadeV1(dirname(file)); vWriteV1(file, "", { flag: "wx" }); mine = true; } catch {}
  if (!mine) for (let at = 0; at < 1200; at++) { await new Promise(done => setTimeout(done, 25)); const later = readVerdictV1(file); if (later !== undefined) { verdictCountsV1.read++; verdictCountsV1.waited++; return later; } }
  try { const holds = await compute(); writeVerdictV1(file, holds); return holds; } catch (error) { if (mine) { try { vRmV1(file, { force: true }); } catch {} } throw error; }
}
/** 📊️ How the verdict files went in this process: read from another look, written, and waited for while another process verified. */
export function closureVerdictCountsV1() { return { ...verdictCountsV1 }; }
`;
export const all=[
 {file:SI,change:edit('import { AsyncLocalStorage } from "node:async_hooks";','import { AsyncLocalStorage } from "node:async_hooks";\nimport { mkdirSync as vMkdirV1, readFileSync as vReadV1, writeFileSync as vWriteV1, renameSync as vRenameV1, rmSync as vRmV1, readdirSync as vListV1, statSync as vStatV1 } from "node:fs";\nimport { tmpdir as vTmpV1 } from "node:os";')},
 {file:SI,change:edit('const layerVerdictsV1 = new WeakMap(), layerFlightsV1',helpers+'const layerVerdictsV1 = new WeakMap(), layerFlightsV1')},
 {file:SI,change:edit('  const check = (async () => {\n    const started = performance.now();','  const check = sharedVerdictV1(verdictFileV1("l", digest), async () => {\n    const started = performance.now();')},
 {file:SI,change:edit('    verdicts?.set(digest, holds);\n    return holds;\n  })();\n  flights?.set(digest, check);\n  try { return await check; } finally { flights?.delete(digest); }','    verdicts?.set(digest, holds);\n    return holds;\n  });\n  flights?.set(digest, check);\n  try { const result = await check; verdicts?.set(digest, result); return result; } finally { flights?.delete(digest); }')},
 {file:SI,change:edit('function storedLayerHoldsV1(entry, digest) {\n  const round','function storedLayerHoldsV1(entry, digest) {\n  const file = verdictFileV1("l", digest), filed = readVerdictV1(file);\n  if (filed !== undefined) { verdictCountsV1.read++; return filed; }\n  const holds = storedLayerComputeV1(entry, digest);\n  writeVerdictV1(file, holds);\n  return holds;\n}\nfunction storedLayerComputeV1(entry, digest) {\n  const round')},
 {file:SI,change:edit('  closureCountsV1.checked++;\n  const step = cooperativeStepV1(scripts, "build:closure-entry-rows");','  const entryFile = verdictFileV1("e", entry.digest), filed = readVerdictV1(entryFile);\n  if (filed !== undefined) { verdictCountsV1.read++; if (round) entry.checked = { round, holds: filed }; return filed; }\n  closureCountsV1.checked++;\n  const step = cooperativeStepV1(scripts, "build:closure-entry-rows");')},
 {file:SI,change:edit('  if (round) entry.checked = { round, holds };\n  return holds;\n}\n\nconst closureEntryHoldsV1 = entry => {\n  const round = closurePassV1 ?? observationRoundV1();\n  if (round && entry.checked?.round === round) return entry.checked.holds;\n  const holds =','  writeVerdictV1(entryFile, holds);\n  if (round) entry.checked = { round, holds };\n  return holds;\n}\n\nconst closureEntryHoldsV1 = entry => {\n  const round = closurePassV1 ?? observationRoundV1();\n  if (round && entry.checked?.round === round) return entry.checked.holds;\n  const entryFile = verdictFileV1("e", entry.digest), filed = readVerdictV1(entryFile);\n  if (filed !== undefined) { verdictCountsV1.read++; if (round) entry.checked = { round, holds: filed }; return filed; }\n  const holds =')},
 {file:SI,change:edit('observationRowsHoldV1(entry.rows) && (entry.layers ? entry.layers.every(observationLayerHoldsV1) : entry.digests.every(digest => storedLayerHoldsV1(entry, digest)));\n  if (round) entry.checked','observationRowsHoldV1(entry.rows) && (entry.layers ? entry.layers.every(observationLayerHoldsV1) : entry.digests.every(digest => storedLayerHoldsV1(entry, digest)));\n  writeVerdictV1(entryFile, holds);\n  if (round) entry.checked')},
 {file:SI,change:edit('export async function withClosurePassV1(compute) {','export async function withClosurePassV1(compute, { verdicts } = {}) {')},
 {file:SI,change:edit('try { return await withObservationPassRoundV1(closurePassV1, compute); } finally { closurePassV1 = outer; }','try { return await verdictOverrideV1.run(verdicts ?? verdictOverrideV1.getStore(), () => withObservationPassRoundV1(closurePassV1, compute)); } finally { closurePassV1 = outer; }')},
 {file:SID,change:edit('export function withClosurePassV1<T>(compute:()=>Promise<T>|T):Promise<T>;','export function withClosurePassV1<T>(compute:()=>Promise<T>|T,options?:{verdicts?:string}):Promise<T>;\nexport function closureVerdictScopeV1():string|undefined;\nexport function closureVerdictCountsV1():{read:number;written:number;waited:number};')},
 {file:EN,change:edit('return{recipe:record.recipe,value,rows:shareObservationRowsV1(record.rows),digests,','return{recipe:record.recipe,digest:record.digest,value,rows:shareObservationRowsV1(record.rows),digests,')},
 {file:PN,change:edit('let runClosureTakeV1, releaseAfterTakeLookV1, withClosurePassV1,','let closureVerdictScopeV1, runClosureTakeV1, releaseAfterTakeLookV1, withClosurePassV1,')},
 {file:PN,change:edit('({ runClosureTakeV1, releaseAfterTakeLookV1, withClosurePassV1, primeSharedClosuresInChildV1,','({ closureVerdictScopeV1, runClosureTakeV1, releaseAfterTakeLookV1, withClosurePassV1, primeSharedClosuresInChildV1,')},
 {file:PN,change:edit('job.selectedInputs) }))); } finally { releaseAfterTakeLookV1(environment.scripts, { shrink: false }); }\n  const used','job.selectedInputs) })), { verdicts: job.verdicts }); } finally { releaseAfterTakeLookV1(environment.scripts, { shrink: false }); }\n  const used')},
 {file:PN,change:edit('observe:true,closureStore:closureStoreDirectoryV1(workspaceRoot)}','observe:true,closureStore:closureStoreDirectoryV1(workspaceRoot),verdicts:closureVerdictScopeV1()}')},
];
export const apply=(file,text,list=all)=>list.filter(item=>file.endsWith(item.file)).reduce((value,item)=>item.change(value),text);
