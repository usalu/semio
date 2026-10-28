import { basename, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import * as observation from "../../../../../../../../🏗️build/👁️observation/🟨️.mjs";
import { libraryBootstrap } from "../../../../../🟨️.mjs";
import { projectGeneratorContractsV1 } from "../../../../🟨️.mjs";
import { openProjectFarmV1 } from "../../../🟨️.mjs";

/** 🧪️ A parent process of the farm with a history of its own: `history` reads a workspace file before its first project the way a graph child's command body does (the read joins its sealed authority, a fresh worker never makes it); `plain` reads only the generator contracts every parent hands its workers; `adopt` checks the authority hand-off inside one process. Prints one JSON line. */
const mode = process.argv[2], root = (() => { let at = dirname(fileURLToPath(import.meta.url)); while (basename(at) !== "🧰️framework") at = dirname(at); return dirname(at); })();
await libraryBootstrap;
if (mode === "adopt") {
  const { adoptInferenceAuthorityV1, inferenceAuthorityRowsV1, inferenceRecordIdentityV1, observedFs } = observation;
  observedFs.readFileSync(join(root, "package.json"));
  const own = inferenceAuthorityRowsV1(), identity = inferenceRecordIdentityV1(), file = own.find(row => row.kind === "file");
  const forged = own.map(row => row === file ? { ...row, fact: { ...row.fact, sha256: "0".repeat(64) } } : row), conflicts = adoptInferenceAuthorityV1(forged), unchanged = inferenceRecordIdentityV1() === identity;
  const extra = [...own, { kind: "exists", path: join(root, "semio-farm-law-absent"), fact: false }], adopted = adoptInferenceAuthorityV1(extra), changed = inferenceRecordIdentityV1() !== identity, again = adoptInferenceAuthorityV1(own);
  console.log(JSON.stringify({ conflicts, unchanged, adopted, changed, restored: again.length === 0 && inferenceRecordIdentityV1() === identity }));
  process.exit(0);
}
if (mode === "history") observation.observedFs.readFileSync(join(root, "package.json"));
const lines = [], plan = JSON.stringify({ version: 1, id: "law", jobs: 1, cpu: 1, memoryMegabytes: 4096, fixedMegabytes: 0 }), job = name => ({ name, root: name, json: {}, native: false, pipeline: false, nativeRoot: name, selectedInputs: [], head: [], cacheKey: "0".repeat(24) });
const contracts = projectGeneratorContractsV1(root), farm = openProjectFarmV1({ workspaceRoot: root, context: { registry: null }, contracts, projects: 100, commandInputs: null, projectsByRoot: new Map(), componentRoots: [], environment: { ...process.env, LANG: "en_US.UTF-8", SEMIO_GRAPH_WORKERS: "1", SEMIO_HOST_CAPACITY_PLAN: plan }, processed: () => 24, cores: 4, write: line => lines.push(line), launch: null, request: () => ({ granted: Promise.resolve(undefined), release: () => undefined }) });
for (let index = 0; index < 24; index++) await farm.prepare(job("miss" + index));
const ready = await new Promise(done => { farm.onReady(() => done(true)); const poll = setInterval(() => { if (lines.some(line => line.includes("differ from this process"))) { clearInterval(poll); done(false); } }, 50); poll.unref?.(); setTimeout(() => done(false), 120000).unref?.(); });
await farm.close();
console.log(JSON.stringify({ ready, refused: lines.filter(line => line.includes("differ from this process")) }));
process.exit(0);
