import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

if (!process.versions.bun && process.argv[1]?.replaceAll("\\", "/").endsWith("/nx.js")) {
  const require = createRequire(process.argv[1]), hashing = require("nx/src/hasher/task-hasher"), io = require("nx/src/tasks-runner/task-io-service");
  io.getTaskIOService().subscribeToTaskInputs(() => {});
  const selected = new Set(["@semio-tech/framework-renderer-wgpu:generate-browser-boot", "@semio-tech/plugin-registry:generate", "@semio-tech/framework-schema:generate"]);
  let sequence = 0;
  const digest = (value: string | Buffer) => createHash("sha256").update(value).digest("hex");
  const record = (tasks: any[], results: any[]) => {
    for (let index = 0; index < tasks.length; index++) {
      const task = tasks[index], result = results[index]; if (!selected.has(task.id)) continue;
      const files: Record<string, string> = {};
      for (const path of result.inputs?.files ?? []) {
        try { files[path] = digest(readFileSync(resolve(process.cwd(), path))); }
        catch (error) { files[path] = `unavailable:${(error as NodeJS.ErrnoException).code}`; }
      }
      const nodes = Object.fromEntries(Object.entries(result.details?.nodes ?? {}).map(([key, value]) => [key, digest(JSON.stringify(value))]));
      const receipt = { task: task.id, hash: result.value, command: result.details?.command, nodes, files, inputKinds: Object.fromEntries(Object.entries(result.inputs ?? {}).map(([key, values]) => [key, (values as any[]).length])) };
      const path = resolve(import.meta.dirname, `../../🗑️generated/boot-hash-${process.pid}-${++sequence}.json`);
      writeFileSync(path, JSON.stringify(receipt, null, 2)); console.error(`[DEBUG] Hashed ${task.id}: ${result.value}, ${Object.keys(files).length} input files; ${path}`);
    }
  };
  for (const name of ["DaemonBasedTaskHasher", "InProcessTaskHasher"]) {
    const prototype = hashing[name].prototype, original = prototype.hashTasks;
    prototype.hashTasks = async function (tasks: any[], ...args: any[]) { const result = await original.call(this, tasks, ...args); record(tasks, result); return result; };
  }
}
