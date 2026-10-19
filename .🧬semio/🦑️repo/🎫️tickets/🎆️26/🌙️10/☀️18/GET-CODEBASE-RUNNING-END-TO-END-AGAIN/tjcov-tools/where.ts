import { listSourceFamily } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { readFileSync } from "node:fs";
const [dir, ...needles] = process.argv.slice(2);
const files = listSourceFamily(dir!, { tests: true });
const texts = files.map((f) => [f, readFileSync(f, "utf8")] as const);
for (const n of needles) {
  const hit = texts.filter(([, t]) => t.includes(n)).map(([f]) => f.slice(dir!.length + 1));
  const hitSpace = hit.length ? [] : texts.filter(([, t]) => t.replace(/\s+/g, "").includes(n.replace(/\s+/g, ""))).map(([f]) => "~" + f.slice(dir!.length + 1));
  console.log(JSON.stringify(n).slice(0, 110), "=>", [...hit, ...hitSpace].slice(0, 4).join(" | ") || "NONE");
}
