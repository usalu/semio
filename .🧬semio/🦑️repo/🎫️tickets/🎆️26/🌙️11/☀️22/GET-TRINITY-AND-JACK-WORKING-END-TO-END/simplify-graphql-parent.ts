import { readFileSync, writeFileSync } from "node:fs";

const path = process.argv[2]!;
const lines = readFileSync(path, "utf8").split("\n");
const before = lines.length;
const kept = lines.filter(line => !line.startsWith(" const valueType=(value:any):any=>") && !line.startsWith(" const kind=(value:any)=>({...value,properties"));
const index = kept.findIndex(line => line.startsWith(" const parent={...vector.snapshot.workingGraph,manifest:"));
if (index >= 0) kept[index] = " const parent=vector.snapshot.workingGraph;";
writeFileSync(path, kept.join("\n"));
console.log(`removed=${before - kept.length} parent=${index >= 0}`);
