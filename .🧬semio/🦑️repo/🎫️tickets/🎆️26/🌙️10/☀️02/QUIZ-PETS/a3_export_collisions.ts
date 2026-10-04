/** 🔀️ Check of work package A3: which exported names of the pets modules and the schema twin collide with another file's, so that `export *` in the package glue would drop them. `bun a3_export_collisions.ts` from anywhere. */
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const pets = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const files = [join(pets, "🧬️schema", "🟦️.ts"), ...readdirSync(join(pets, "🔨️modules")).map((module) => join(pets, "🔨️modules", module, "🟦️.ts"))];
const owners = new Map<string, string[]>();
for (const file of files) {
  let source = "";
  try {
    source = readFileSync(file, "utf8");
  } catch {
    continue;
  }
  for (const match of source.matchAll(/^export\s+(?:declare\s+)?(?:const|function|type|interface|class|enum)\s+([A-Za-z_$][\w$]*)/gmu)) owners.set(match[1]!, [...(owners.get(match[1]!) ?? []), file.slice(pets.length + 1)]);
}
const twice = [...owners].filter(([, where]) => where.length > 1);
process.stdout.write(`${files.length} files, ${owners.size} exported names, ${twice.length} collisions\n`);
for (const [name, where] of twice) process.stdout.write(`${name}: ${where.join(", ")}\n`);
const glue = readFileSync(join(pets, "📦️packages", "🟦️typescript", "🟦️.ts"), "utf8");
process.stdout.write(`glue re-exports: ${[...glue.matchAll(/export \* from "([^"]+)"/gu)].map((match) => match[1]).join(" ")}\n`);
