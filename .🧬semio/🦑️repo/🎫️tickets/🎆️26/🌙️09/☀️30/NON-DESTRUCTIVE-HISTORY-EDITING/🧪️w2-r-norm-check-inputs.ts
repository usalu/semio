/** 🔍️ W2-R norm: reads every norm leaf payload schema on disk — catalogued or not — through W1-D's `mutationInputDefs`, one top-level input at a time like the `schema-mutation-input-ui` lint, and prints every refusal; with `--before <snapshot.json>` it also compiles every leaf in the strict Ajv oracle (`x-semio-ui` registered) now and before the rollout and prints every leaf that compiled before and no longer does. */
import { InputSchemaError, mutationInputDefs } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import { semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const scope = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm";
const documents = new Map<string, unknown>();
const leaves: string[] = [];
const walk = (directory: string): void => {
  for (const name of readdirSync(directory)) {
    const path = join(directory, name);
    if (statSync(path).isDirectory()) walk(path);
    else if (name.endsWith(".json")) {
      try {
        const document = JSON.parse(readFileSync(path, "utf8"));
        if (typeof document?.$id === "string") documents.set(document.$id, document);
      } catch {}
      if (/🧬️mutations\/[^/]+\/🧬️schema\/🔣️\.json$/u.test(path)) leaves.push(path);
    }
  }
};
walk(scope);
let inputs = 0;
let failures = 0;
for (const path of leaves.sort()) {
  const leaf = JSON.parse(readFileSync(path, "utf8"));
  const keys = Object.keys(leaf.properties ?? {});
  for (const key of keys.length === 0 ? [null] : keys) {
    const single = key === null ? leaf : { ...leaf, properties: { [key]: leaf.properties[key] }, required: (leaf.required ?? []).filter((name: string) => name === key) };
    try {
      const defs = mutationInputDefs(single, (id) => documents.get(id));
      inputs += defs.length;
      if (process.argv.includes("--verbose")) for (const def of defs) console.log(path.split("🧬️mutations/")[1]!.split("/")[0], JSON.stringify(def));
    } catch (error) {
      if (!(error instanceof InputSchemaError)) throw error;
      failures += 1;
      inputs += 1;
      console.log("FAIL", path.replace(`${scope}/`, ""), error.message);
    }
  }
}
console.log(`leaves=${leaves.length} inputs=${inputs} failures=${failures}`);
let regressed = 0;
if (process.argv.includes("--before")) {
  const before = JSON.parse(readFileSync(process.argv[process.argv.indexOf("--before") + 1]!, "utf8")) as Record<string, string>;
  const compiles = (text: string): boolean => {
    try {
      const schema = JSON.parse(text);
      const { $id: _, ...anonymous } = schema;
      semioSchemaAjvV1().compile(anonymous);
      return true;
    } catch {
      return false;
    }
  };
  let now = 0;
  let was = 0;
  for (const path of leaves) {
    const relative = path.replace("/Users/ueli/Documents/semio/", "");
    const ok = compiles(readFileSync(path, "utf8"));
    const old = before[relative] === undefined ? ok : compiles(before[relative]!);
    now += ok ? 1 : 0;
    was += old ? 1 : 0;
    if (old && !ok) {
      regressed += 1;
      console.log("AJV-REGRESSED", relative);
    }
  }
  console.log(`strict-ajv compiles now=${now} before=${was} regressed=${regressed} of ${leaves.length}`);
}
process.exit(failures === 0 && regressed === 0 ? 0 : 1);
