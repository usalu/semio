import { mkdtempSync, readFileSync, realpathSync, mkdirSync, rmSync, readdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const gcStats = { gc: 0, gcMs: 0, shrink: 0, shrinkMs: 0 };
{ const g = Bun.gc.bind(Bun), sh = Bun.shrink?.bind(Bun);
Bun.gc = (...a) => { const t = performance.now(); try { return g(...a); } finally { gcStats.gc++; gcStats.gcMs += performance.now() - t; } };
if (sh) Bun.shrink = (...a) => { const t = performance.now(); try { return sh(...a); } finally { gcStats.shrink++; gcStats.shrinkMs += performance.now() - t; } }; }
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
const { createRowLogV1 } = await import(B + "/🕸️graph/🕰️generations/🗃️row-log/🟨️.mjs");
const rowLog = createRowLogV1({ path: join(parentS, "rl", "test.rows") });
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
      if (project && !project.error && project.closureRecipes) { await pa.resolveClosureRefsV1(project, root, scripts, "hash", rowLog); refs = Date.now() - m; totals.refs += Date.now() - m; }
      console.log("parent", job.name, "observe", m - s, "refs", refs, project.error ?? "");
      if (process.env.R57_WARM && i === jobs.length - 1 && project.closureRecipes) { const W = Number(process.env.R57_WARM); const ts = []; for (let k = 0; k < W; k++) { const a = Date.now(); await pa.resolveClosureRefsV1(project, root, scripts, "hash", rowLog); ts.push(Date.now() - a); } console.log("warm refs ms", ts.join(",")); }
    }
  });
});
console.log("totals", JSON.stringify(totals), "gc", JSON.stringify({...gcStats, gcMs: Math.round(gcStats.gcMs), shrinkMs: Math.round(gcStats.shrinkMs)}), "wall", Date.now() - wallStart);
rmSync(rootS, { recursive: true, force: true }); rmSync(parentS, { recursive: true, force: true });
process.exit(0);
