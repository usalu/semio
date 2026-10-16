/** 📦️ hr2 manifest editor: adds dependency rows to package.json files preserving the round-trip-stable layout. Usage: bun deps.ts <plan.json> [--write] where plan = [{manifest, section, name, version}] */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const [planPath, flag] = process.argv.slice(2);
const write = flag === "--write";
const plan: { manifest: string; section: "dependencies" | "devDependencies"; name: string; version: string }[] = JSON.parse(readFileSync(planPath!, "utf8"));
const byManifest = new Map<string, typeof plan>();
for (const row of plan) byManifest.set(row.manifest, [...(byManifest.get(row.manifest) ?? []), row]);
for (const [manifest, rows] of byManifest) {
  const path = join(ROOT, manifest);
  const original = readFileSync(path, "utf8");
  const doc = JSON.parse(original);
  if (JSON.stringify(doc, null, 2) + "\n" !== original) throw new Error(`not round-trip stable: ${manifest}`);
  for (const row of rows) {
    const existing = doc[row.section] ?? {};
    if (existing[row.name] !== undefined) continue;
    const merged = { ...existing, [row.name]: row.version };
    const keys = Object.keys(existing);
    const sorted = keys.every((key, index) => index === 0 || keys[index - 1]! <= key);
    doc[row.section] = sorted ? Object.fromEntries(Object.entries(merged).sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0))) : merged;
  }
  const next = JSON.stringify(doc, null, 2) + "\n";
  console.log(`${manifest}: ${rows.map((r) => `${r.section}:${r.name}@${r.version}`).join(", ")}${next === original ? " (no change)" : ""}`);
  if (write && next !== original) {
    if (readFileSync(path, "utf8") !== original) throw new Error(`changed under us: ${manifest}`);
    writeFileSync(path, next);
  }
}
