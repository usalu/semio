import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { readModuleTreeSource } from "../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const callers = JSON.parse(readFileSync("🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🧪️tests/📋️callers.json", "utf8")).callers as { source: string; manifest: string }[];
let bad = 0;
for (const row of callers) {
  if (!existsSync(row.source)) { console.log("MISSING", row.source); bad++; continue; }
  const text = readModuleTreeSource(join(process.cwd(), row.source));
  const manifest = readFileSync(row.manifest, "utf8");
  const problems: string[] = [];
  if (!text.includes("semio_framework_composition_admission::")) problems.push("noRef");
  if (/(?:os_store|store)::ChildRestoreProjection/u.test(text)) problems.push("legacyRef");
  if (!/semio-framework-composition-admission/.test(manifest)) problems.push("noManifestDep");
  if (problems.length) { bad++; console.log(problems.join(","), row.source); }
}
console.log("rows", callers.length, "bad", bad);
