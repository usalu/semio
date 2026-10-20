import { readFileSync } from "node:fs";
import { rustFunctions } from "./census.ts";
for (const file of process.argv.slice(2)) {
  const rows = rustFunctions(file, readFileSync(file, "utf8")).filter((r) => !r.nested).sort((a, b) => b.lines - a.lines).slice(0, 3);
  console.log(rows.map((r) => `${r.name}:${r.lines}`).join(" "));
}
