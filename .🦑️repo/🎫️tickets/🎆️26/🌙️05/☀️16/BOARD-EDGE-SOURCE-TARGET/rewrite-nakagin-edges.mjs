import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "..", "..", "..", "..", "..", "..");
const p = join(root, ".storybook", "fixtures", "nakagin-capsule-tower.board.json");
let t = readFileSync(p, "utf8");
const fromN = t.split('"from":').length - 1;
t = t.split('"from":').join('"source":').split('"to":').join('"target":');
writeFileSync(p, t);
console.log("replaced edge from keys:", fromN);
