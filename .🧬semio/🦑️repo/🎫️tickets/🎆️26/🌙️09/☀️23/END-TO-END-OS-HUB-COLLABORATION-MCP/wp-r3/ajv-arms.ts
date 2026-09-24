import Ajv from "ajv";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
const SUBSETS = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets";
const ajv = new Ajv({ strict: false, allErrors: true });
const read = (p: string) => JSON.parse(readFileSync(p, "utf8"));
const seen = new Set<string>();
const walk = (dir: string): void => {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, entry.name);
    if (entry.isDirectory()) { if (!["🧫️fixtures", "🖼️assets", "🧪️tests", "target", "node_modules"].includes(entry.name)) walk(p); continue; }
    if (entry.name !== "🔣️.json" || !p.includes("🧬️schema")) continue;
    const schema = read(p);
    if (typeof schema.$id !== "string" || seen.has(schema.$id)) continue;
    seen.add(schema.$id);
    ajv.addSchema(schema);
  }
};
walk(SUBSETS);
walk("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
walk("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io");
const BASE = join(SUBSETS, "✉️base");
const vSnap = ajv.getSchema("https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/base/snapshot.json")!;
const vMut = ajv.getSchema("https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/base/mutations.json")!;
const fixtures = join(BASE, "🧫️fixtures");
const report = (label: string, v: typeof vSnap, value: unknown) => { const ok = v(value); return ok ? `${label}:ok` : `${label}:FAIL ${JSON.stringify((v.errors ?? []).filter((e) => e.keyword !== "oneOf" && e.keyword !== "const" && !(e.keyword === "required" && e.instancePath === "/subset")).slice(0, 3).map((e) => `${e.instancePath} ${e.message} ${JSON.stringify(e.params)}`))}`; };
for (const dir of readdirSync(fixtures).filter((d) => d.endsWith("-applied"))) {
  console.log(dir, [report("before", vSnap, read(join(fixtures, dir, "⬅️before.json"))), report("after", vSnap, read(join(fixtures, dir, "➡️after.json"))), report("mut", vMut, read(join(fixtures, dir, "🦠️mutation.json")))].join(" | ").slice(0, 700));
}
