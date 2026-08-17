import { readFileSync } from "fs";
const lines = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧩core/🟦️component.ts","utf8").split("\n");
// Print outlines of major sections by looking at export type/interface/const at depth-ish
const marks = [1,5,14,67,92,136,163,197,583,1211,1385,1460,1537,1611,1652,1704,1860,1908,2259,3447,3586,3660,4090];
for (const n of marks) {
  console.log(`\n===== LINE ${n} =====`);
  for (let i=n-1; i<Math.min(n+25, lines.length); i++) {
    console.log(`${i+1}: ${lines[i].slice(0,120)}`);
  }
}
