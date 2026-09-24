import { readFileSync } from "node:fs";
import { discoverTestContributions } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
let same = 0, diff = 0;
for (const c of discoverTestContributions(root)) {
  const text = readFileSync(`${root}/${c.manifestPath}`, "utf8");
  if (JSON.stringify(JSON.parse(text), null, 2) + "\n" === text) same++; else { diff++; if (diff < 4) console.log("differs", c.manifestPath); }
}
console.log({ same, diff });
