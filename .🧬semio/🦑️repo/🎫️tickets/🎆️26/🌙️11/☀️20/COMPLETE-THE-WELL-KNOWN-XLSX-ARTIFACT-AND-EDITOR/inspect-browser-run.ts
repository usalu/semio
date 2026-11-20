import { readdir } from "node:fs/promises";
import { dirname, join } from "node:path";

const pid = Number(process.argv[2]);
const generated = join(dirname(import.meta.path), "🗑️generated");
if (!Number.isInteger(pid) || pid <= 1) {
  for (const mode of ["acceptance", "io-matrix", "host", "xlsx-host", "io-matrix-direct", "xlsx-host-direct", "describe-office-direct", "test-demo"]) {
    const log = Bun.file(join(generated, `${mode}.log`));
    const status = Bun.file(join(generated, `${mode}.status`));
    console.log("[DEBUG]", mode, await status.exists() ? (await status.text()).trim() : "no exit status");
    if (await log.exists()) console.log((await log.text()).trim().split("\n").slice(-12).join("\n"));
  }
  const host = Bun.file(".🧬semio/🌐hub/dev-serves/serve-6070.log");
  if (await host.exists()) console.log("[DEBUG] dev server", (await host.text()).trim().split("\n").slice(-20).join("\n"));
  for (const variant of ["s", "stdio-xlsx"]) {
    const receipt = Bun.file(`.cache/b/p/m/browser/prototype/runtime/react/${variant}/activation/🔣️receipt.json`);
    console.log(`[DEBUG] activation receipt for ${variant}: ${await receipt.exists()}`);
  }
  process.exit(0);
}
const ps = Bun.spawnSync(["ps", "-axo", "pid,ppid,etime,state,command"], { stdout: "pipe" });
if (!ps.success) throw new Error("Process query failed");
const processes = new TextDecoder().decode(ps.stdout).trim().split("\n").slice(1).map(line => {
  const [, id, parent, elapsed, state, command] = line.match(/^\s*(\d+)\s+(\d+)\s+(\S+)\s+(\S+)\s+(.*)$/) ?? [];
  return { id: Number(id), parent: Number(parent), elapsed, state, command };
});
const target = processes.find(row => row.id === pid);
console.log("[DEBUG]", target ?? "Browser run process has exited");
const queue = [pid];
while (queue.length) {
  const owner = queue.shift();
  for (const child of processes.filter(row => row.parent === owner)) {
    queue.push(child.id);
    console.log("[DEBUG]", child);
  }
}
const output = join(generated, "io-matrix");
const entries = await readdir(output, { recursive: true }).catch(error => error.code === "ENOENT" ? [] : Promise.reject(error));
console.log("[DEBUG] Browser output files", entries);
