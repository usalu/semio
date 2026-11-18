import { readFileSync, writeFileSync } from "node:fs";

const files = process.argv.slice(2);
const pattern = /\bobserve\(\)(\s*:\s*void)?\s*\{\}/g;

for (const file of files) {
  const source = readFileSync(file, "utf8");
  let count = 0;
  const next = source.replace(pattern, match => {
    count += 1;
    const separator = match.endsWith(" {}") ? " " : "";
    return match + separator + match.replace("observe", "unobserve");
  });
  if (count > 0) writeFileSync(file, next);
  console.log(`${count}\t${file}`);
}
