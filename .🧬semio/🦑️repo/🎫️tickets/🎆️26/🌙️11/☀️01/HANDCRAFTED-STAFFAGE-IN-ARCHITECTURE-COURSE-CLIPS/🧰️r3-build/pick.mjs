import { readFileSync } from "node:fs";
const [f, ...paths] = process.argv.slice(2);
const d = JSON.parse(readFileSync(f, "utf8")).asset;
for (const p of paths) {
  let v = d;
  for (const k of p.split(".")) v = v?.[k];
  console.log("##", p);
  console.log(JSON.stringify(v, null, 1).slice(0, 5500));
}
