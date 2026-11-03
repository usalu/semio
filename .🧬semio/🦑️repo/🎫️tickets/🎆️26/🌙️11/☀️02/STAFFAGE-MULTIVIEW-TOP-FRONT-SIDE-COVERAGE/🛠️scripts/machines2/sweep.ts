import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "../machines/lib.ts";
import { ALL } from "./ids.ts";
let n = 0;
for (const id of ALL) {
  const f = fileOf(id);
  const t = readFileSync(f, "utf8");
  const out = t.replace(/^(\s*)"kind": "extruded",\n(?!\s*"sweep")/gm, (_m, ind) => { n++; return `${ind}"kind": "extruded",\n${ind}"sweep": "w",\n`; });
  if (out !== t) writeFileSync(f, out);
}
console.log("sweep added", n);
