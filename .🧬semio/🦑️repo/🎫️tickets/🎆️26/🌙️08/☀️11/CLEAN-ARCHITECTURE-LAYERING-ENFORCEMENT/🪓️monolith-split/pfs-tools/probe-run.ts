// usage: bun probe-run.ts <module> <level> <out.json>   runs every check of vizInferenceChecks(level) and records module/name/outcome digest
import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";
const [modulePath, level, out] = process.argv.slice(2);
const mod = await import(modulePath);
const checks = await mod.vizInferenceChecks(level);
const rows: string[] = [];
const digest = (value: unknown) => createHash("sha1").update(JSON.stringify(value, (_k, v) => (typeof v === "number" && !Number.isFinite(v) ? String(v) : typeof v === "bigint" ? String(v) : v)) ?? "undefined").digest("hex").slice(0, 12);
for (const check of checks) {
  let row = `${check.module}\t${check.name}\t${check.tolerance ?? ""}\t`;
  try { row += `subject=${digest(await check.subject())}\toracle=${digest(await check.oracle())}`; } catch (error) { row += `error=${(error as Error).message.slice(0, 120)}`; }
  rows.push(row);
}
writeFileSync(out, rows.sort().join("\n") + "\n");
console.log(`checks=${rows.length} errors=${rows.filter((r) => r.includes("\terror=")).length}`);
