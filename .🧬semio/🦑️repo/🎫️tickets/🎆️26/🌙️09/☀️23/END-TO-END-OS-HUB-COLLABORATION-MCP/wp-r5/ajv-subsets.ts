import Ajv from "ajv";
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join } from "node:path";
const SUBSETS = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets";
const ajv = new Ajv({ strict: false, allErrors: true });
const read = (p: string) => JSON.parse(readFileSync(p, "utf8"));
const seen = new Set<string>();
const walkSchemas = (dir: string): void => {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, entry.name);
    if (entry.isDirectory()) { if (!["🧫️fixtures", "🖼️assets", "🧪️tests", "target", "node_modules"].includes(entry.name)) walkSchemas(p); continue; }
    if (entry.name !== "🔣️.json" || !p.includes("🧬️schema")) continue;
    const schema = read(p);
    if (typeof schema.$id !== "string" || seen.has(schema.$id)) continue;
    seen.add(schema.$id); ajv.addSchema(schema);
  }
};
walkSchemas(SUBSETS);
walkSchemas("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
walkSchemas("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io");
const only = process.argv.slice(2);
const files = (dir: string, out: string[] = []): string[] => { if (!existsSync(dir)) return out; for (const e of readdirSync(dir, { withFileTypes: true })) { const p = join(dir, e.name); if (e.isDirectory()) files(p, out); else if (e.name === "🔣️.json") out.push(p); } return out; };
const summary: Record<string, string> = {};
for (const subset of readdirSync(SUBSETS)) {
  if (subset === "✉️base" || (only.length && !only.some((o) => subset.includes(o)))) continue;
  const slug = subset.replace(/^[^a-z]+/u, "");
  const snap = ajv.getSchema(`https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/${slug}/snapshot.json`);
  const mut = ajv.getSchema(`https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/${slug}/mutations.json`);
  let ok = 0, bad = 0; const errors = new Map<string, number>();
  for (const f of files(join(SUBSETS, subset, "🧫️fixtures", "🧬️mutations"))) {
    const v = f.includes("/📸️snapshot/") ? snap : f.includes("/🦠️mutation/") ? mut : undefined;
    if (!v) continue;
    if (v(read(f))) ok++; else { bad++; for (const e of (v.errors ?? []).slice(0, 4)) { const k = `${f.includes("/🦠️mutation/") ? "mut" : "snap"} ${e.instancePath.replace(/\/\d+/g, "/N")} ${e.message}`; errors.set(k, (errors.get(k) ?? 0) + 1); } }
  }
  summary[subset] = `${snap ? "" : "NO-SNAP-SCHEMA "}${mut ? "" : "NO-MUT-SCHEMA "}ok=${ok} bad=${bad}`;
  console.log(subset, summary[subset]);
  for (const [k, n] of [...errors].sort((a, b) => b[1] - a[1]).slice(0, 8)) console.log("   ", n, k);
}
