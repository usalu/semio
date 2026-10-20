import {appendFileSync} from "node:fs";

const out = process.env.SEMIO_CENSUS_OUT;
const cp = require("node:child_process");
const rows: Array<{ program: string; head: string; ms: number }> = [];
const label = (command: unknown, args?: unknown) => {
  const list = Array.isArray(command) ? command.map(String) : [String(command), ...(Array.isArray(args) ? args.map(String) : [])];
  const program = list[0]!.split("/").pop()!;
  return { program, head: list.slice(1, 3).join(" ").slice(0, 80) };
};
const record = (command: unknown, args: unknown, ms: number) => {
  const row = { ...label(command, args), ms };
  rows.push(row);
  if (out) appendFileSync(out, JSON.stringify({ at: Math.round(performance.now()), ...row }) + "\n");
};
for (const name of ["spawnSync", "execFileSync", "execSync"]) {
  const original = cp[name];
  cp[name] = function (...args: unknown[]) { const start = performance.now(); try { return original.apply(this, args); } finally { record(args[0], args[1], performance.now() - start); } };
}
for (const name of ["spawn", "execFile", "exec"]) {
  const original = cp[name];
  cp[name] = function (...args: unknown[]) { const start = performance.now(), child = original.apply(this, args); child.once?.("close", () => record(args[0], args[1], performance.now() - start)); return child; };
}
const bunSpawn = Bun.spawn, bunSpawnSync = Bun.spawnSync;
(Bun as any).spawn = function (...args: any[]) { const start = performance.now(), child = (bunSpawn as any).apply(this, args); child.exited.then(() => record(args[0]?.cmd ?? args[0], undefined, performance.now() - start)); return child; };
(Bun as any).spawnSync = function (...args: any[]) { const start = performance.now(); try { return (bunSpawnSync as any).apply(this, args); } finally { record(args[0]?.cmd ?? args[0], undefined, performance.now() - start); } };
process.on("exit", () => {
  const groups = new Map<string, { count: number; ms: number; max: number }>();
  for (const row of rows) { const key = `${row.program} ${row.head}`; const g = groups.get(key) ?? { count: 0, ms: 0, max: 0 }; g.count++; g.ms += row.ms; g.max = Math.max(g.max, row.ms); groups.set(key, g); }
  const lines = [...groups].sort((a, b) => b[1].ms - a[1].ms).slice(0, 25).map(([key, g]) => `${String(g.count).padStart(5)} ${String(Math.round(g.ms)).padStart(8)} ms (max ${Math.round(g.max)}) ${key}`);
  console.error("[census] spawns " + rows.length + ", wall in children " + Math.round(rows.reduce((s, r) => s + r.ms, 0)) + " ms\n" + lines.join("\n"));
});
