import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { appendFileSync, existsSync, readFileSync, watch } from "node:fs";
import { fileURLToPath } from "node:url";
const require = createRequire(import.meta.url), ticket = dirname(dirname(fileURLToPath(import.meta.url)));
const output = join(ticket, "🗑️generated", "executor-graph.jsonl");
const record = (kind: string, environment = process.env): void => {
  const directory = environment.NX_WORKSPACE_DATA_DIRECTORY ?? join(process.cwd(), ".nx/workspace-data"), path = join(directory, "project-graph.json");
  let graph: unknown;
  try { const value = JSON.parse(readFileSync(path, "utf8")); graph = { computedAt: value.computedAt, errors: value.errors, projects: Object.keys(value.nodes ?? {}).length, names: Object.keys(value.nodes ?? {}).sort() }; } catch (error) { graph = String(error); }
  appendFileSync(output, JSON.stringify({ kind, pid: process.pid, cwd: process.cwd(), environment: Object.fromEntries(Object.entries(environment).filter(([key]) => key.startsWith("NX_") && !/TOKEN|SECRET|KEY/.test(key))), graph, path }) + "\n");
};
record("startup");
const subprocess = require("node:child_process"), fork = subprocess.fork;
subprocess.fork = (...args: unknown[]) => { const options = args.find((arg) => arg && typeof arg === "object" && !Array.isArray(arg)) as { env?: NodeJS.ProcessEnv } | undefined; record("fork:" + args[0], options?.env); return fork(...args); };

if (process.argv[2] === "watch") {
  const directory = process.env.NX_WORKSPACE_DATA_DIRECTORY ?? join(process.cwd(), ".nx/workspace-data");
  const observer = watch(directory, () => record("changed"));
  await new Promise((accept) => setTimeout(accept, 30000));
  observer.close();
}
