import { mkdtempSync, readFileSync, realpathSync, mkdirSync, rmSync, readdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const gcStats = { gc: 0, gcMs: 0, shrink: 0, shrinkMs: 0 };
{ const g = Bun.gc.bind(Bun), sh = Bun.shrink?.bind(Bun);
Bun.gc = (...a) => { const t = performance.now(); try { return g(...a); } finally { gcStats.gc++; gcStats.gcMs += performance.now() - t; } };
if (sh) Bun.shrink = (...a) => { const t = performance.now(); try { return sh(...a); } finally { gcStats.shrink++; gcStats.shrinkMs += performance.now() - t; } }; }

import { createRequire } from "node:module";
const jsc = createRequire(import.meta.url)("bun:jsc"); const censusStats = { calls: 0, ms: 0 };
{ const orig = jsc.heapStats; jsc.heapStats = (...a) => { const t = performance.now(); try { return orig(...a); } finally { censusStats.calls++; censusStats.ms += performance.now() - t; } }; }
const lag = { max: 0, over1s: 0, sum: 0, n: 0 }; { let last = Date.now(); setInterval(() => { const now = Date.now(), l = Math.max(0, now - last - 200); last = now; lag.n++; lag.sum += l; lag.max = Math.max(lag.max, l); if (l > 1000) lag.over1s++; }, 200); }
const lines = { starved: 0, sampled: 0 };
const fp = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🦶️footprint/🟨️.mjs");
fp.bindProcessGateV1(() => ({ limit: { memoryMegabytes: 14336 } }), { write: () => {}, trace: Object.assign(line => { if (line.includes("starved sample")) lines.starved++; else if (line.includes("sample step")) lines.sampled++; }, { path: "-" }) });
const wallStart = Date.now();
const root = "/Users/ueli/Documents/semio", L = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", B = L + "/🏗️build", I = B + "/🕸️graph/📥️inference", P = I + "/📦️project-nodes";
const N = Number(process.env.R57_N ?? 12), PROFILE_PARENT = process.env.R57_PARENT !== "0";
const { libraryBootstrap } = await import(I + "/🟨️.mjs"); await libraryBootstrap;
const pn = await import(P + "/🟨️.mjs"), obs = await import(B + "/👁️observation/🟨️.mjs"), service = await import(P + "/🏊️project-farm/👷️service/🟨️.mjs");
const reg = await import(I + "/🟨️.mjs");
const sc = await import(I + "/🧭️script-inputs/🟨️.mjs"), pa = await import(I + "/🧱️project-assembly/🟨️.mjs");
const { withBuildGraphStagingV1 } = await import(B + "/📥️inputs/💾️storage/🟨️.mjs");
const dirs = (process.env.R57_OWNERS ?? "").split(",").filter(Boolean);
const list = dirs.length ? dirs : JSON.parse(readFileSync(process.env.R57_LIST, "utf8"));
const contracts = await obs.observeInferenceSnapshotV1(() => pn.projectGeneratorContractsV1(root));
const closureStore = sc.closureStoreDirectoryV1(root);
const session = { version: 3, workspaceRoot: root, closureStore, registry: null, contracts, commandInputs: { closureRecipe: { fn: "native-command" } }, lateClosures: true, projectsByRoot: [], componentRoots: [] };
const globals = { projectAuthority: "r57", registryDigest: "r57", commandRevision: "r57", late: true };
const rootS = mkdtempSync(join(realpathSync(tmpdir()), "r57-")), parentS = mkdtempSync(join(realpathSync(tmpdir()), "r57p-"));
await service.open({ ...session, stagingBase: rootS }, { slot: 0, generation: 1 });
const env = service.sessionEnvironmentV1(), staging = service.stagingDirectoryV1();
for (const s of [staging, parentS]) for (const d of ["ce", "ip", "mr", "np", "rl", "rr"]) mkdirSync(join(s, "inference", d), { recursive: true });
const jobs = list.slice(0, N).map(rel => { const json = { ...JSON.parse(readFileSync(join(root, rel, "📋️project.json"), "utf8")), root: rel }; return pn.ownerJobV1(env, globals, { name: json.name, json }); });
let records = [];
let t = Date.now();
import { existsSync, writeFileSync } from "node:fs";
const RF = process.env.R57_RECORDS;
if (RF && existsSync(RF)) records = JSON.parse(readFileSync(RF, "utf8"));
else for (const job of jobs) { const s = Date.now(); await service.run(job); const cp = join(staging, "inference", "np", job.cacheKey + ".json"); const rec = await withBuildGraphStagingV1({ root: staging, published() {} }, () => obs.exportProjectRecordV1(cp)); records.push(rec); console.log("child", job.name, Date.now() - s, "ms rows", rec?.observations.length); }
console.log("children total ms", Date.now() - t); if (RF && !existsSync(RF)) writeFileSync(RF, JSON.stringify(records));
const parentEnv = pn.createProjectInferenceEnvironmentV1({ ...session, stagingBase: parentS });
const scripts = parentEnv.scripts;
const totals = { observe: 0, refs: 0 };
if (PROFILE_PARENT) await withBuildGraphStagingV1({ root: parentS, published() {} }, async () => {
  await obs.observeInferenceSnapshotV1(async () => {
    for (let i = 0; i < jobs.length; i++) {
      const job = jobs[i], cachePath = join(parentS, "inference", "np", job.cacheKey + ".json");
      await obs.importProjectRecordV1(cachePath, records[i]);
      const s = Date.now();
      const project = await obs.observeProjectInferenceV1({ cachePath, key: pn.projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs), owner: job.root, stamps: [], reusable: pn.projectRecordReusableV1 }, () => { throw Error("would infer " + job.name); }).catch(e => ({ error: String(e.message).slice(0, 80) }));
      const m = Date.now(); totals.observe += m - s;
      let refs = "n/a";
      if (project && !project.error && project.closureRecipes) { await pa.resolveClosureRefsV1(project, root, scripts, "hash"); refs = Date.now() - m; totals.refs += Date.now() - m; }
      console.log("parent", job.name, "observe", m - s, "refs", refs, project.error ?? "");
    }
  });
});
console.log("totals", JSON.stringify(totals), "gc", JSON.stringify({...gcStats, gcMs: Math.round(gcStats.gcMs), shrinkMs: Math.round(gcStats.shrinkMs)}), "wall", Date.now() - wallStart, "census", JSON.stringify({...censusStats, ms: Math.round(censusStats.ms)}), "lines", JSON.stringify(lines), "lag", JSON.stringify({...lag, avg: Math.round(lag.sum / Math.max(1, lag.n))}));
rmSync(rootS, { recursive: true, force: true }); rmSync(parentS, { recursive: true, force: true });
process.exit(0);
