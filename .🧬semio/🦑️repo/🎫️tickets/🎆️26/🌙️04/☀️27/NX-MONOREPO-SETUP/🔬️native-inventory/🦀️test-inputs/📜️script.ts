import { createRequire } from "node:module";
import { readFileSync, existsSync, writeFileSync } from "node:fs";
import { join, dirname, relative, resolve } from "node:path";
import { cacheInternals } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs";
const require = createRequire(import.meta.url), workspace = process.env.NX_WORKSPACE_ROOT!;
const { createProjectGraphAsync } = require("@nx/devkit");
const graph = await createProjectGraphAsync({ exitOnError: false });
const selected: any = Object.values(graph.nodes).find((node: any) => node.data.tags?.includes("type:test") && node.data.tags?.includes("impl:🦀️.rs") && node.data.root.includes("stdio") && node.data.targets["test-subject"]);
if (!selected) throw new Error("No native artifact test case found");
const owner = selected.data.tags.find((tag: string) => tag.startsWith("owner:")).slice(6);
let directory = owner, manifest: string | undefined;
for (let depth = 0; depth < 16 && directory !== "."; depth++) {
  const candidate = join(directory, "📦️packages/🦀️rust/Cargo.toml");
  if (existsSync(join(workspace, candidate))) { manifest = candidate; break; }
  directory = dirname(directory);
}
if (!manifest) throw new Error("The selected Rust subject has no package");
const roots = cacheInternals.nativeDependencyRoots(dirname(manifest), workspace, false);
const { createTaskGraph } = require("nx/src/tasks-runner/create-task-graph"), { createTaskHasher } = require("nx/src/hasher/create-task-hasher");
const tasks = createTaskGraph(graph, {}, [selected.name], ["test-subject"], undefined, {}, true), task: any = Object.values(tasks.tasks)[0];
require("nx/src/tasks-runner/task-io-service").getTaskIOService().subscribeToTaskInputs(() => {});
const hashed = await createTaskHasher(graph, JSON.parse(readFileSync(join(workspace, "nx.json"), "utf8")), {}).hashTask(task, tasks, process.env);
const files = new Set((hashed.inputs?.files ?? []).map((path: string) => relative(workspace, resolve(workspace, path)).replaceAll("\\", "/")));
if (!files.size) throw new Error("Native hash input collection returned no files");
const candidates = ["Cargo.toml", "rust-toolchain.toml", ...roots.map((root: string) => join(root, "Cargo.toml").replaceAll("\\", "/"))].filter((path: string) => existsSync(join(workspace, path)));
const result = { project: selected.name, owner, subjectManifest: manifest, task: task.id, hash: hashed.value, fileCount: files.size, sampleFiles: [...files].slice(0, 3), dependencyManifests: candidates.map((path: string) => ({ path, hashed: files.has(path) })), inputs: selected.data.targets["test-subject"].inputs };
writeFileSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "native-inferred-rust-test-inputs.json"), JSON.stringify(result, null, 2) + "\n");
console.log(JSON.stringify({ project: result.project, fileCount: files.size, dependencies: candidates.length, missing: result.dependencyManifests.filter((entry: any) => !entry.hashed).length }));
