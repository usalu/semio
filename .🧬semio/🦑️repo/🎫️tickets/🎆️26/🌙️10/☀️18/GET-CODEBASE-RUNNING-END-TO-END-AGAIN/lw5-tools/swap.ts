import { readFileSync, writeFileSync } from "node:fs";
const R = "/Users/ueli/Documents/semio/";
const [, , planPath, dir, only] = process.argv;
const plan = JSON.parse(readFileSync(planPath!, "utf8"));
for (const f of plan.fixes) {
  if (only && !f.from.includes(only)) continue;
  const file = R + f.from; let s = readFileSync(file, "utf8");
  const [a, b] = dir === "revert" ? [f.replacement, f.specifier] : [f.specifier, f.replacement];
  const q = (x: string) => `"${x}"`;
  if (s.includes(q(a))) { s = s.replace(q(a), q(b)); writeFileSync(file, s); console.log(dir, f.from.slice(-50)); } else console.log("skip(not found)", f.from.slice(-50));
}
