import Ajv from "ajv";
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join } from "node:path";
const SUBSETS = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets";
const ajv = new Ajv({ strict: false, allErrors: true });
const seen = new Set<string>();
const walk = (dir: string): void => { for (const e of readdirSync(dir, { withFileTypes: true })) { const p = join(dir, e.name); if (e.isDirectory()) { if (!["🧫️fixtures", "🖼️assets", "🧪️tests", "target", "node_modules"].includes(e.name)) walk(p); continue; } if (e.name !== "🔣️.json" || !p.includes("🧬️schema")) continue; const s = JSON.parse(readFileSync(p, "utf8")); if (typeof s.$id === "string" && !seen.has(s.$id)) { seen.add(s.$id); ajv.addSchema(s); } } };
walk(SUBSETS);
walk("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
walk("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io");
const [subset] = process.argv.slice(2);
const slug = subset.replace(/^[^a-z]+/u, "");
const files = (dir: string, out: string[] = []): string[] => { if (!existsSync(dir)) return out; for (const e of readdirSync(dir, { withFileTypes: true })) { const p = join(dir, e.name); if (e.isDirectory()) files(p, out); else if (e.name === "🔣️.json") out.push(p); } return out; };
for (const f of files(join(SUBSETS, subset, "🧫️fixtures", "🧬️mutations"))) {
  const kind = f.includes("/📸️snapshot/") ? "snapshot" : f.includes("/🦠️mutation/") ? "mutations" : f.includes("/🔺️diff/") ? "diff" : null;
  if (!kind) continue;
  const v = ajv.getSchema(`https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/${slug}/${kind}.json`)!;
  if (!v(JSON.parse(readFileSync(f, "utf8")))) console.log(f.split("🧫️fixtures/")[1], "\n  ", (v.errors ?? []).filter((e) => e.keyword !== "const" && e.keyword !== "oneOf").map((e) => `${e.instancePath} ${e.message} ${JSON.stringify(e.params)}`).slice(0, 8).join("\n   "));
}
