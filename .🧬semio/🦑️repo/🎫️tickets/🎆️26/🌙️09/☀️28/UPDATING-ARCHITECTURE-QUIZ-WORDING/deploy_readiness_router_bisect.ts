/** 🧭️ Deploy-readiness probe: which class of the site's task router does the `root-script` grammar of the taxonomy
 * reject? Prints the verdict for the file as it is and for the file with each one of its classes (and its registration)
 * removed. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fixedSourceDispositionDecision } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const root = join(import.meta.dir, "../../../../../../..");
const script = readFileSync(join(root, "🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📜️script.ts"), "utf8");
const verdict = (content: string): string => JSON.stringify(fixedSourceDispositionDecision("root-script", content)?.role);
console.log(`[DEBUG] as it is: ${verdict(script)}`);
for (const [, name] of script.matchAll(/^class (\w+) extends BundleScript/gmu)) {
  const without = script.replace(new RegExp(`(?:/\\*\\*(?:(?!\\*/)[\\s\\S])*\\*/\\n)?class ${name} extends BundleScript \\{[\\s\\S]*?\\n\\}\\n`, "u"), "").replace(new RegExp(`\\.register\\("[a-z0-9-]+", ${name}\\)`, "u"), "");
  console.log(`[DEBUG] without ${name}: ${verdict(without)}`);
}
