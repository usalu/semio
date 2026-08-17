import { readFileSync } from "fs";
const lines = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧩core/🟦️component.ts","utf8").split("\n");
const chunk = lines.slice(582, 1210).join("\n");
// find identifiers that look like references to earlier exports
const idents = new Set();
for (const m of chunk.matchAll(/\b([A-Z][A-Za-z0-9_]+)\b/g)) idents.add(m[1]);
const earlier = lines.slice(0,582).join("\n");
const needed = [...idents].filter(id => new RegExp(`(type|interface|const|function) ${id}\\b`).test(earlier) || earlier.includes(`export type ${id}`) || earlier.includes(`export interface ${id}`) || earlier.includes(`export const ${id}`) || earlier.includes(`export function ${id}`));
console.log("mesh needs from earlier:", needed.sort().join(", "));

const platformChunk = lines.slice(1384, 1650).join("\n") + "\n" + lines.slice(1708, 1859).join("\n");
const pIdents = new Set();
for (const m of platformChunk.matchAll(/\b([A-Z][A-Za-z0-9_]+)\b/g)) pIdents.add(m[1]);
const neededP = [...pIdents].filter(id => {
  const before = lines.slice(0,1384).join("\n");
  return new RegExp(`export (type|interface|const|function) ${id}\\b`).test(before);
});
console.log("platform dock/inspector needs:", neededP.sort().join(", "));
