/** ✅️ Ticket tool: prints the issues the pets product's own validator finds in species documents (exit code 1 when any exists).
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/validate_species.ts" <species.json>...
 */
import { readFileSync } from "node:fs";
import { speciesIssues } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/✅️validation/🟦️.ts";

let failed = false;
for (const file of process.argv.slice(2)) {
  const issues = speciesIssues(JSON.parse(readFileSync(file, "utf8")));
  console.log(`${issues.length === 0 ? "ok  " : "FAIL"} ${file}`);
  for (const issue of issues) console.log(`     ${issue.code} at ${issue.path}`);
  failed ||= issues.length > 0;
}
process.exit(failed ? 1 : 0);
