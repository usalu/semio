import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const { semioSchemaAjvV1 } = await import(join(process.cwd(), "🧰️framework/📦️packages/🟦️typescript/🧬️schema/🔮️oracles/✅️validator/🟦️.ts"));
const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
const check = ajv.compile({ $ref: "https://json.schemas.assets.semio-tech.com/framework/manifest/schema.json#/$defs/InputUi" });
const failures: unknown[] = [];
let checked = 0;
function inspect(value: unknown, file: string, pointer: string): void {
  if (!value || typeof value !== "object") return;
  for (const [key, child] of Object.entries(value)) {
    if (key === "x-semio-ui") { checked++; if (!check(child)) failures.push({ file, pointer, value: child, errors: structuredClone(check.errors) }); }
    if (child && typeof child === "object") inspect(child, file, pointer + "/" + key);
  }
}
function walk(path: string): void {
  for (const entry of readdirSync(path, { withFileTypes: true })) {
    if (["node_modules", "target", "🧪️tests", "📦️packages"].includes(entry.name)) continue;
    const file = join(path, entry.name);
    if (entry.isDirectory()) walk(file);
    else if (entry.name === "🔣️.json" && file.includes("🧬️schema")) inspect(JSON.parse(readFileSync(file, "utf8")), file, "");
  }
}
walk("✏️s/🔌️plugins/📕️norm/🗿️artifacts");
writeFileSync(join(import.meta.dir, "ui-annotation-audit.md"), "# 🎛️ Strict UI Annotation Audit\n\n" + checked + " annotations checked; " + failures.length + " invalid.\n\n" + "\x60\x60\x60json\n" + JSON.stringify(failures, null, 2) + "\n\x60\x60\x60\n");
console.log(JSON.stringify({ checked, failures }));
