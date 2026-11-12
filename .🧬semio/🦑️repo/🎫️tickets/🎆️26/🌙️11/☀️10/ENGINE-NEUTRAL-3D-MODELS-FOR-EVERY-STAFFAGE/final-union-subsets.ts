import { readFileSync } from "node:fs";
import { unionPolygons } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
const d = JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/🗑️generated/final/union-inputs4.json", "utf8"));
const x = d[0];
const n = x.input.length;
let found = 0;
for (let mask = 1; mask < 1 << n; mask++) {
  const sub = x.input.filter((_: unknown, i: number) => (mask >> i) & 1);
  try { unionPolygons(sub); } catch { if (found++ < 6) console.log("fails", sub.length, [...Array(n).keys()].filter((i) => (mask >> i) & 1).join(",")); }
}
console.log("failing subsets", found, "of", (1 << n) - 1);
