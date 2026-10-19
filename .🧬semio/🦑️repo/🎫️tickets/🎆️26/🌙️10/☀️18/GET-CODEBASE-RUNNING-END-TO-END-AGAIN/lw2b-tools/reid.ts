import { readFileSync, writeFileSync } from "node:fs";
const map: ReadonlyArray<readonly [string, string]> = [
  ["framework/schema/artifact-child.json", "framework/schema/artifact-reference/child/schema.json"],
  ["framework/schema/artifact-link.json", "framework/schema/artifact-reference/link/schema.json"],
  ["framework/schema/blob-ref.json", "framework/schema/artifact-reference/blob/schema.json"],
];
let total = 0;
for (const file of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const text = readFileSync(file, "utf8");
  let next = text;
  for (const [from, to] of map) next = next.split(from).join(to);
  if (next !== text) { writeFileSync(file, next); total += 1; }
}
console.log("files changed", total);
