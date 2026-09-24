import Ajv from "ajv";
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join } from "node:path";
const SUBSETS = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets";
const BASE = join(SUBSETS, "✉️base");
const ajv = new Ajv({ strict: false, allErrors: true });
const read = (p: string) => JSON.parse(readFileSync(p, "utf8"));
const seen = new Set<string>();
const walk = (dir: string): void => {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, entry.name);
    if (entry.isDirectory()) { if (!["🧫️fixtures", "🖼️assets", "🧪️tests", "target", "node_modules"].includes(entry.name)) walk(p); continue; }
    if (entry.name !== "🔣️.json" || !p.includes("🧬️schema")) continue;
    const schema = read(p);
    if (typeof schema.$id !== "string" || seen.has(schema.$id) || schema.$id.endsWith("/base/mutations.json")) continue;
    seen.add(schema.$id);
    try { ajv.addSchema(schema); } catch (e) { console.log("addSchema", p.slice(SUBSETS.length), String(e).slice(0, 160)); }
  }
};
walk(SUBSETS);
console.log("[DEBUG] registered", seen.size, seen.has("https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/brep/inference.json"));
const mutations = join(BASE, "🧬️schema", "🧬️mutations");
ajv.addSchema(read(join(mutations, "🔣️.json")));
const snapshotId = "https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/base/snapshot.json";
const leafEnvelopeId = "https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/base/mutation/set-snapshot/schema.json#/$defs/SemioSnapshot";
const mutationId = "https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/base/mutations.json";
const vByArm = ajv.getSchema(snapshotId)!, vByLeaf = ajv.getSchema(leafEnvelopeId)!, vMut = ajv.getSchema(mutationId)!;
const fixtures = join(BASE, "🧫️fixtures");
for (const dir of readdirSync(fixtures).filter((d) => d.endsWith("-applied"))) {
  const res: string[] = [];
  for (const [name, v] of [["⬅️before.json", vByArm], ["⬅️before.json", vByLeaf], ["🦠️mutation.json", vMut]] as const) {
    const ok = v(read(join(fixtures, dir, name)));
    res.push(`${name.slice(0, 8)}:${ok ? "ok" : "FAIL " + JSON.stringify(v.errors?.slice(0, 2).map((e) => `${e.instancePath} ${e.message}`))}`);
  }
  console.log(dir, res.join(" | ").slice(0, 600));
}
