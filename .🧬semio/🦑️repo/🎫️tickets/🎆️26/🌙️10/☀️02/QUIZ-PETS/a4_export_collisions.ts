/** 🧾️ Check of work package A4: which export names of the climbing module are also exported by the package glue or by any other module of the pets core, so the glue can re-export it with `export *` only when nothing collides. `bun a4_export_collisions.ts` from the repository root; the answer goes to the terminal of whoever runs it. */
import { readdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const pets = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const climbing = Object.keys(await import(resolve(pets, "🔨️modules/🧗️climbing/🟦️.ts")));
const owners = new Map<string, string[]>();
for (const module of readdirSync(resolve(pets, "🔨️modules"))) {
  if (module === "🧗️climbing") continue;
  try {
    for (const name of Object.keys(await import(resolve(pets, "🔨️modules", module, "🟦️.ts")))) owners.set(name, [...(owners.get(name) ?? []), module]);
  } catch (error) {
    owners.set(`(${module} did not load: ${String(error).slice(0, 80)})`, [module]);
  }
}
for (const name of Object.keys(await import(resolve(pets, "🧬️schema/🟦️.ts")))) owners.set(name, [...(owners.get(name) ?? []), "🧬️schema"]);
const clashes = climbing.filter((name) => owners.has(name)).map((name) => `${name}: ${owners.get(name)!.join(", ")}`);
const failures = [...owners.keys()].filter((name) => name.startsWith("("));
process.stdout.write(`${climbing.length} runtime exports of 🧗️climbing; collisions: ${clashes.length === 0 ? "none" : clashes.join("; ")}; modules that did not load: ${failures.length === 0 ? "none" : failures.join("; ")}\n`);
