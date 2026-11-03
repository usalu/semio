import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
const t = readFileSync(file, "utf8");
const from = " * World space: metres, x right, y up, ground at y = 0.";
if (!t.includes(from)) throw new Error("anchor");
writeFileSync(file, t.replace(from, " * World space: metres, x ahead, y up, z left of the asset (a left-handed triple), ground at y = 0; the side elevation is seen from the asset's right with ahead on the paper's right (see `../🎨️depiction/🔭️views/🟦️.ts`)."));
