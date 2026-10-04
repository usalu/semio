/** 🔺️ S3-NORM census: every committed norm `🔺️diff` fixture against its artifact diff schema (strict Ajv, catalog `$ref`s). Usage: `bun 🧪️s3-norm-diff-schema-census.ts`. */
import { readdirSync, readFileSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";
import { semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
const A = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts";
const CATALOG = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json";
const read = (p: string) => JSON.parse(readFileSync(p, "utf8"));
const scopes = Object.values(read(CATALOG).scopes as Record<string, { path: string; formats: Record<string, string> }>);
const walk = (d: string): string[] => existsSync(join(d, "🔺️diff/🔣️.json")) && existsSync(join(d, "🦠️mutation/🔣️.json")) ? [d] : readdirSync(d).filter((n) => statSync(join(d, n)).isDirectory()).flatMap((n) => walk(join(d, n)));
for (const artifact of readdirSync(A).sort()) {
  const schema = join(A, artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  if (!existsSync(join(schema, "🔺️diff/🔣️.json"))) continue;
  const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
  const docs = new Map<string, any>();
  for (const f of ["📸️snapshot/🔣️.json", "🔺️diff/🔣️.json", "🔣️.json"]) { const d = read(join(schema, f)); docs.set(d.$id, d); }
  for (let pending = () => [...new Set([...docs.values()].flatMap((d) => [...JSON.stringify(d).matchAll(/"\$ref":"(https:[^"#]+)/g)].map((m) => m[1]!)))].filter((id) => !docs.has(id)), missing = pending(); missing.length; missing = pending()) {
    for (const id of missing) { const s = scopes.find((c) => { const f = join("/Users/ueli/Documents/semio", c.path, c.formats["🔣️jsonschema"] ?? ""); return f.endsWith(".json") && existsSync(f) && read(f).$id === id; }); docs.set(id, s ? read(join("/Users/ueli/Documents/semio", s.path, s.formats["🔣️jsonschema"]!)) : { $id: id }); if (!s) console.log("unresolved", id); }
  }
  for (const d of docs.values()) ajv.addSchema(d);
  const diffId = read(join(schema, "🔺️diff/🔣️.json")).$id;
  let v; try { v = ajv.getSchema(diffId)!; } catch (e) { console.log(artifact, "COMPILE", String(e).slice(0, 150)); continue; }
  const fixtures = walk(join(A, artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations"));
  let bad = 0; const samples: string[] = [];
  for (const b of fixtures) { const ok = v(read(join(b, "🔺️diff/🔣️.json"))); if (!ok) { bad++; if (samples.length < 2) samples.push(b.split("🧬️mutations/")[1] + " " + JSON.stringify(v.errors?.slice(0, 2))); } }
  console.log(artifact, `${fixtures.length - bad}/${fixtures.length}`, samples.join(" | ").slice(0, 400));
}
