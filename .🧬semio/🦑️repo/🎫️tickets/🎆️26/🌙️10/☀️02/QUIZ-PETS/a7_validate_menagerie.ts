/** 🩺️ Check of work package A7: the menagerie committed with case ⚗️chemistry-rules and the sample menagerie of the product are judged by the product's own validator (`menagerieIssues`). `bun a7_validate_menagerie.ts` from the repository root; prints the findings per menagerie. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { menagerieIssues } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/✅️validation/🟦️.ts";

const fixtures = resolve(import.meta.dir, "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧫️fixtures");
for (const name of ["⚗️chemistry-rules", "🧬️schema-conformance"]) {
  const document = JSON.parse(readFileSync(resolve(fixtures, name, "🔣️.json"), "utf8")) as { menagerie: unknown };
  const issues = menagerieIssues(document.menagerie);
  process.stdout.write(`${name}: ${issues.length} finding(s) ${JSON.stringify(issues.slice(0, 12))}\n`);
}
