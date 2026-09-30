/** 🔍️ W1-F: reads every puzzle 2d leaf payload schema through W1-D's `mutationInputDefs` and validates each x-semio-ui against the manifest `InputUi` meta-schema semantics. */
import { mutationInputDefs } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import { existsSync, readdirSync, readFileSync } from "node:fs";

const root = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations";
let failures = 0;
let inputs = 0;
for (const leaf of readdirSync(root).sort()) {
  const path = `${root}/${leaf}/🧬️schema/🔣️.json`;
  if (!existsSync(path)) continue;
  try {
    const defs = mutationInputDefs(readFileSync(path, "utf8"), () => undefined);
    inputs += defs.length;
    const summary = defs.map((def: any) => `${def.id}:${def.presentation ?? ""}:${def.schema?.kind ?? def.schema?.type ?? Object.keys(def.schema ?? {})[0]}`).join(" ");
    if (process.argv.includes("--verbose")) console.log(leaf, defs.length, summary);
  } catch (error) {
    failures += 1;
    console.log("FAIL", leaf, (error as Error).message);
  }
}
console.log(`inputs=${inputs} failures=${failures}`);
