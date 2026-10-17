/** 🔎️ Lists the edges that are neither skipped nor in a ui or asset group, with their owner. Usage: bun opens.ts <scan.json> */
import { readFileSync } from "node:fs";
import { edgeSkip } from "./skip.ts";
const edges = JSON.parse(readFileSync(process.argv[2], "utf8")) as any[];
for (const e of edges) {
  if (edgeSkip(e)) continue;
  if (e.resolved.includes("/🖱️ui/")) continue;
  console.log([e.test ? "T" : "P", e.sourceOwner.replace(/🧰️framework\/🔨️modules\//g, "F/M/").replace(/🧰️framework\/🛍️products\//g, "F/P/"), e.file.replace(/🧰️framework\/🔨️modules\//g, "F/M/").replace(/🧰️framework\/🛍️products\//g, "F/P/"), e.line, e.resolved.replace(/🧰️framework\/🔨️modules\//g, "F/M/").replace(/🧰️framework\/🛍️products\//g, "F/P/"), JSON.stringify(e.named)].join(" | "));
}
