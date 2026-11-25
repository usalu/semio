import { readFile } from "node:fs/promises";
import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
const root = process.cwd();
const { readRustSourceClosureV1, createRustSourceAnalysisV1 } = await import(resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const rustc = (extra) => execFileSync("rustc", ["--print", "cfg", ...extra], { encoding: "utf8" }).trim().split("\n");
const base = { build: rustc([]), test: rustc(["--test"]) };
const jobs = JSON.parse(await readFile(process.argv[2], "utf8"));
const analysis = createRustSourceAnalysisV1();
const rows = [];
for (const job of jobs) {
  const cfg = [...base[job.test ? "test" : "build"], ...job.features.map(f => "feature=" + JSON.stringify(f))].map(l => l.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
  const row = { crate: job.crate, unit: job.unit };
  try {
    const inputs = await readRustSourceClosureV1([resolve(job.entry)], root, cfg, async path => { try { return new Uint8Array(await readFile(path)); } catch (e) { if (e.code === "ENOENT") { (row.missing ??= []).push(path); return null; } throw e; } }, { manifestDirectory: job.manifestDirectory, generated: () => undefined }, analysis);
    row.files = inputs.length; if (process.env.DUMP) row.inputs = inputs.map(i => [i.path, i.bytes, i.activeLines, i.kind]); row.bytes = inputs.reduce((s, i) => s + i.bytes, 0); row.lines = inputs.reduce((s, i) => s + i.activeLines, 0);
  } catch (e) { row.error = String(e && e.message || e).slice(0, 300); }
  rows.push(row);
}
console.log(JSON.stringify(rows));
