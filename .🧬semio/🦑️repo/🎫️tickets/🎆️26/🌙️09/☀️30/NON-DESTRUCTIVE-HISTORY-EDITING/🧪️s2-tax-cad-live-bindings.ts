/** 🔗️ Rebinds the sealed CAD/Draw projection golden's CAD `liveBindings[*].live` carriers onto the current CAD tree (path-budget renames), byte-preserving every other line; prints the new document digest for the re-seal. */
import { createHash } from "node:crypto";
import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const root = process.cwd(), path = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json");
const text = readFileSync(path, "utf8"), golden = JSON.parse(text);
const cad = golden.projections.find((projection: { contractId: string }) => projection.contractId === "artifact-example-model-catalog-v1");
const walk = (directory: string, prefix = ""): string[] => readdirSync(directory).flatMap((name) => { const absolute = join(directory, name), relative = prefix ? `${prefix}/${name}` : name; return statSync(absolute).isDirectory() ? walk(absolute, relative) : [relative]; });
const live = new Set(walk(join(root, cad.sourceRoot)).filter((file) => file.endsWith(".json")));
const renames: readonly (readonly [string, string])[] = [["🌉️aec.building.structure.classic/", "🌉️aec.building/"], ["📏️aec.building.structure.fem.line/", "📏️aec.building/"], ["🗺️aec.building.structure.fem.surface/", "🗺️aec.building/"], ["🧊️aec.building.structure.fem.solid/", "🧊️aec.building/"], ["/🚧️ReinforcedConcreteInternalWall/", "/🚧️ReinforcedConcreteInternal/"], ["/🛡️ReinforcedConcreteExternalWall/", "/🛡️ReinforcedConcreteExternal/"]];
let rebound = 0, output = text;
for (const binding of cad.liveBindings as { source: string; live: string }[]) {
  if (live.has(binding.live)) continue;
  const current = renames.reduce((value, [from, to]) => value.replace(from, to), binding.live);
  if (!live.has(current)) throw new Error(`No current carrier for ${binding.live}`);
  const before = `"live": ${JSON.stringify(binding.live)}`, after = `"live": ${JSON.stringify(current)}`;
  if (output.split(before).length !== 2) throw new Error(`Binding is not unique: ${binding.live}`);
  output = output.replace(before, after);
  rebound++;
}
const next = JSON.parse(output), bound = new Set(next.projections.find((projection: { contractId: string }) => projection.contractId === cad.contractId).liveBindings.map((binding: { live: string }) => binding.live));
if (bound.size !== live.size || [...live].some((file) => !bound.has(file))) throw new Error("Rebound carriers do not cover the current CAD tree exactly");
if (process.argv.includes("--write")) writeFileSync(path, output);
console.log(JSON.stringify({ rebound, sha256: createHash("sha256").update(output).digest("hex"), written: process.argv.includes("--write") }));
