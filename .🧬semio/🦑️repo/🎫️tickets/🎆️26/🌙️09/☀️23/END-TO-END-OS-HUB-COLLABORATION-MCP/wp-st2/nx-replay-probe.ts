#!/usr/bin/env bun
/** 🔁️ ST2 probe: does a cached consumer's `dependentTasksOutputFiles` see a gitignored receipt its producer rewrites,
 * and does a CACHED producer keyed on membership replay a stale receipt? Runs Nx itself in throwaway git workspaces. */
import { mkdtempSync, mkdirSync, readFileSync, symlinkSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const out = join(import.meta.dir, "generated");
mkdirSync(out, { recursive: true });

function workspace(producerCache: boolean): string {
  const root = mkdtempSync(join(out, `nx-replay-${producerCache ? "cached" : "fresh"}-`));
  mkdirSync(join(root, "src"));
  writeFileSync(join(root, ".gitignore"), "receipt/\nout/\nruns.log\n.nx/\nnode_modules\n");
  writeFileSync(join(root, "package.json"), JSON.stringify({ name: "probe", private: true }) + "\n");
  writeFileSync(join(root, "nx.json"), JSON.stringify({ namedInputs: { default: ["{projectRoot}/project.json"] } }) + "\n");
  writeFileSync(join(root, "src/descriptor.json"), '{"version":1}\n');
  const produce = `const c=require("crypto"),f=require("fs");const d=c.createHash("sha256").update(f.readFileSync("src/descriptor.json")).digest("hex");const b=JSON.stringify({digest:d})+"\\n";f.mkdirSync("receipt",{recursive:true});if(!f.existsSync("receipt/digest.json")||f.readFileSync("receipt/digest.json","utf8")!==b)f.writeFileSync("receipt/digest.json",b);`;
  const consume = `const f=require("fs");f.mkdirSync("out",{recursive:true});f.copyFileSync("src/descriptor.json","out/catalog.json");f.appendFileSync("runs.log","run\\n");`;
  const project = {
    name: "probe",
    targets: {
      inputs: { executor: "nx:run-commands", cache: producerCache, inputs: ["default"], outputs: ["{workspaceRoot}/receipt/digest.json"], options: { command: `node -e '${produce}'` } },
      generate: { executor: "nx:run-commands", cache: true, dependsOn: ["inputs"], inputs: [{ dependentTasksOutputFiles: "receipt/digest.json" }], outputs: ["{workspaceRoot}/out"], options: { command: `node -e '${consume}'` } },
    },
  };
  writeFileSync(join(root, "project.json"), JSON.stringify(project, null, 2) + "\n");
  symlinkSync(join(repo, "node_modules"), join(root, "node_modules"));
  for (const args of [["init", "-q"], ["add", "-A"]]) {
    const git = Bun.spawnSync(["git", ...args], { cwd: root });
    if (git.exitCode !== 0) throw new Error(`git ${args.join(" ")}: ${git.stderr}`);
  }
  return root;
}

function run(root: string): { runs: number; catalog: string; log: string } {
  const env = { ...process.env, NX_DAEMON: "false", NX_NO_CLOUD: "true", NX_TUI: "false", NX_CACHE_DIRECTORY: join(root, ".nx/cache"), NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/workspace-data"), NX_SKIP_NX_CACHE: undefined, NX_PLUGIN_NO_TIMEOUTS: "true" } as Record<string, string | undefined>;
  const child = Bun.spawnSync(["node", join(repo, "node_modules/nx/dist/bin/nx.js"), "run", "probe:generate", "--outputStyle=static"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
  const log = child.stdout.toString() + child.stderr.toString();
  if (child.exitCode !== 0) throw new Error(`nx failed:\n${log}`);
  const runs = existsSync(join(root, "runs.log")) ? readFileSync(join(root, "runs.log"), "utf8").split("\n").filter(Boolean).length : 0;
  return { runs, catalog: readFileSync(join(root, "out/catalog.json"), "utf8").trim(), log };
}

for (const producerCache of [true, false]) {
  const root = workspace(producerCache);
  const rows: string[] = [];
  const initial = run(root); rows.push(`initial runs=${initial.runs} catalog=${initial.catalog}`);
  const unchanged = run(root); rows.push(`unchanged runs=${unchanged.runs} catalog=${unchanged.catalog}`);
  writeFileSync(join(root, "src/descriptor.json"), '{"version":2}\n');
  const edited = run(root); rows.push(`edited runs=${edited.runs} catalog=${edited.catalog}`);
  console.log(`producerCache=${producerCache}: ${rows.join(" | ")}`);
  writeFileSync(join(root, "last-nx.log"), edited.log);
}
