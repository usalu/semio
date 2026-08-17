import { readFileSync } from "fs";
const lines = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧩core/🟦️component.ts","utf8").split("\n");
const chunk = lines.slice(2450, 2608).join("\n");
console.log(chunk.slice(0, 500));
console.log("---");
// identifiers
const idents = new Set([...chunk.matchAll(/\b([A-Z][A-Za-z0-9_]+)\b/g)].map(m=>m[1]));
const exportedBefore = new Map();
for (let i=0;i<2450;i++) {
  const m = lines[i].match(/^export (?:type |interface |const |function |async function )?([A-Za-z0-9_]+)/);
  if (m) exportedBefore.set(m[1], i+1);
}
const needed = [...idents].filter(id => exportedBefore.has(id)).sort();
console.log("action-bus needs:", needed.join(", "));

const kchunk = lines.slice(2420,2450).join("\n") + "\n" + lines.slice(2608,3658).join("\n");
const kidents = new Set([...kchunk.matchAll(/\b([A-Z][A-Za-z0-9_]+)\b/g)].map(m=>m[1]));
const neededK = [...kidents].filter(id => {
  // defined in manifest ranges
  return exportedBefore.has(id) && exportedBefore.get(id) < 2421;
}).sort();
console.log("kernel needs from pre-2421:", neededK.join(", "));

// Check if PluginRegistryEntry is before or after 2421
for (const n of ["PluginRegistryEntry","PluginWasmHandle","WindowLayout","NamedLayout","AppDefinition","ModeDefinition","UtilityNode","ActionArgDef","WindowMeasure","ActionDefinition","ToolDefinition","ModeDefinition"]) {
  console.log(n, exportedBefore.get(n) ?? "not before 2421");
}
// find PluginRegistryEntry
for (let i=0;i<lines.length;i++) if (lines[i].includes("export type PluginRegistryEntry")) console.log("PluginRegistryEntry at", i+1);
