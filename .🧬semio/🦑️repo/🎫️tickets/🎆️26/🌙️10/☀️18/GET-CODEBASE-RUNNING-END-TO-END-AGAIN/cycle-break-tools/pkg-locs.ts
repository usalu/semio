import { loadPackages } from "../exports-stage-tools/lib.ts";
const m = loadPackages();
let inPk = 0, other: string[] = [];
for (const p of m.all) { if (p.dir.includes("/📦️packages/") ) inPk++; else other.push(p.name + "  " + p.dir); }
console.log("under packages", inPk, "other", other.length); console.log(other.slice(0, 80).join("\n"));
