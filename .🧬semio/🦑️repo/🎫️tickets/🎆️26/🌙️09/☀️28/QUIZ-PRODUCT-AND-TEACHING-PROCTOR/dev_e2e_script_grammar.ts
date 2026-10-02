/** 🚦️ Tells which class of a `📜️script.ts` the repo's command-router grammar (`root-script` contract) rejects: the
 * whole file first, then the file with each class (and its registration) removed in turn.
 * `bun dev_e2e_script_grammar.ts [path to a 📜️script.ts]` (default: the architecture quiz site's). */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fixedSourceDispositionDecision } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const root = join(import.meta.dir, "../../../../../../..");
const path = process.argv[2] ?? join(root, "🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📜️script.ts");
const content = readFileSync(path, "utf8");
const accepted = (body: string): boolean => fixedSourceDispositionDecision("root-script", body)?.finding === null;
console.log(`[DEBUG] whole file accepted: ${accepted(content)}`);
for (const found of content.matchAll(/^class (\w+) extends BundleScript \{\n(?:.*\n)*?\}\n/gmu)) {
  const name = found[1]!;
  const without = content.replace(found[0], "").replace(new RegExp(`\\.register\\("[^"]+", ${name}\\)`, "u"), "");
  console.log(`[DEBUG] without ${name}: ${accepted(without)}`);
}
