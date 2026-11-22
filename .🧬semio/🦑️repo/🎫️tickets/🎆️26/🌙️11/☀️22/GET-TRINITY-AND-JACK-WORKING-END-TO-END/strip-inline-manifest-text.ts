import { readFileSync, writeFileSync } from "node:fs";

const path = process.argv[2]!;
const text = readFileSync(path, "utf8");
const marker = '"manifest":{';
let result = text, removed = 0;
for (;;) {
  const start = result.indexOf(marker);
  if (start < 0) break;
  let depth = 0, index = start + marker.length - 1, inString = false;
  for (; index < result.length; index++) {
    const character = result[index]!;
    if (inString) { if (character === "\\") index++; else if (character === '"') inString = false; }
    else if (character === '"') inString = true;
    else if (character === "{") depth++;
    else if (character === "}" && --depth === 0) break;
  }
  const end = result[index + 1] === "," ? index + 2 : index + 1;
  result = result.slice(0, start) + result.slice(end);
  removed++;
}
writeFileSync(path, result);
console.log(`removed=${removed}`);
