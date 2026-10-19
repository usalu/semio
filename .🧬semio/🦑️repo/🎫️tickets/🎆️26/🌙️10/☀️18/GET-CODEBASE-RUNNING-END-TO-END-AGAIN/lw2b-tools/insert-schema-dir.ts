import { readFileSync, writeFileSync } from "node:fs";
const pattern = /(🗿️artifact-reference\/(?:🪆️child|🔗️link|📦️blob)\/)(?!🧬️schema\/)/gu;
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
let total = 0;
for (const file of files) {
  const text = readFileSync(file, "utf8");
  let count = 0;
  const next = text.replace(pattern, (_, head: string) => { count += 1; return `${head}🧬️schema/`; });
  if (count > 0) { writeFileSync(file, next); total += count; }
}
console.log("files", files.length, "replacements", total);
