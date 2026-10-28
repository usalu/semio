import { lstatSync, mkdirSync, readdirSync } from "node:fs";
import { join, relative, sep } from "node:path";
import { libraryBootstrap } from "../../../🟨️.mjs";
import { inferenceRecordIdentityV1, observeInferenceSnapshotV1 } from "../../../../../../🏗️build/👁️observation/🟨️.mjs";
import { withImplementationPinV1 } from "../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import { withBuildGraphStagingV1 } from "../../../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";
import { createProjectInferenceEnvironmentV1, produceProjectRecordV1 } from "../../🟨️.mjs";
import { graphFarmIdentityV1 } from "../🪆️captured/🟨️.mjs";

let environment, staging;

/** 🔌️ Opens one worker session: the same inference implementation as the parent, memos of its own, and a staging directory of its own that nothing else writes to and that only the parent removes, once it saw this process exit (a worker recycled right after a job must leave the leaves of that job for the parent to adopt); answers the farm identity it serves under: the captured implementation it runs from and the identity its records are written under. */
export async function open(session, { slot, generation }) {
  await libraryBootstrap;
  environment = await observeInferenceSnapshotV1(() => createProjectInferenceEnvironmentV1(session));
  staging = join(session.stagingBase, "w" + slot + "-g" + generation);
  mkdirSync(staging, { recursive: true });
  return graphFarmIdentityV1(inferenceRecordIdentityV1());
}

/** 🧾️ Names a staged leaf relative to the staging root with forward slashes, or undefined when the path is no leaf of this staging. */
function leafNameV1(path) {
  const name = relative(staging, path).split(sep).join("/");
  if (name === "" || name.startsWith("../")) return undefined;
  try { return lstatSync(path).isFile() && !name.endsWith(".tmp") ? name : undefined; } catch { return undefined; }
}

/** ▶️ Infers one project into the staging directory under the key and scope its parent uses, and answers the names of the leaves it published: the record, the blocks and receipts it wrote and the pack files of its record store. */
export async function run(job) {
  const cpu = process.cpuUsage(), names = new Set(), note = path => { const name = leafNameV1(path); if (name !== undefined) names.add(name); };
  await withImplementationPinV1(() => withBuildGraphStagingV1({ root: staging, published: note }, () => produceProjectRecordV1(environment, job)));
  const records = join(staging, "inference", "np", job.cacheKey + ".json.records");
  try { for (const entry of readdirSync(records)) note(join(records, entry)); } catch (error) { if (error?.code !== "ENOENT") throw error; }
  const used = process.cpuUsage(cpu);
  return { names: [...names].sort(), cpuMilliseconds: Math.round((used.user + used.system) / 1000) };
}
