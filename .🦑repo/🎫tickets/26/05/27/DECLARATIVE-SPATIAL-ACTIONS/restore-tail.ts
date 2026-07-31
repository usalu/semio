import { readFileSync, writeFileSync } from "node:fs";

const head = readFileSync("c:/git/compose/.repo/🎫/26/05/27/DECLARATIVE-SPATIAL-ACTIONS/index-head.tsx", "utf8");
const headLines = head.split(/\r?\n/);
const tailStart = headLines.findIndex((l) => l.includes("<span>Selection kinds</span>"));
if (tailStart < 0) throw new Error("tail not found");
let tail = headLines.slice(tailStart).join("\n");
// drop duplicate selection kinds header block until first Selection kinds content - we already have show kinds closing
const current = readFileSync("c:/git/compose/spatial/js/renderer-r3f/index.tsx", "utf8");
writeFileSync("c:/git/compose/spatial/js/renderer-r3f/index.tsx", `${current}\n${tail}`);
