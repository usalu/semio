import { readFileSync } from "fs";
const ts = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧩core/🟦️component.ts", "utf8");
const lines = ts.split("\n");

// Print region map with line ranges and sample export names
let stack = [];
const regions = [];
for (let i = 0; i < lines.length; i++) {
  const m1 = lines[i].match(/\/\/\s*#region\s+(.+)/);
  const m2 = lines[i].match(/\/\/\s*#endregion\s*(.*)/);
  if (m1) {
    stack.push({ name: m1[1].trim(), start: i+1 });
  } else if (m2) {
    const top = stack.pop();
    if (top) regions.push({ name: top.name, start: top.start, end: i+1, depth: stack.length });
  }
}
console.log("REGIONS:");
for (const r of regions) {
  const exports = [];
  for (let i = r.start-1; i < r.end; i++) {
    const m = lines[i].match(/^export (?:type |interface |class |enum |const |function |async function )?([A-Za-z0-9_]+)/);
    if (m) exports.push(m[1]);
  }
  console.log(`${r.start}-${r.end} depth=${r.depth} ${r.name} exports=${exports.length} sample=${exports.slice(0,8).join(',')}`);
}

// Top-level content between regions
console.log("\n--- top-level export names (all) count ---");
const allExports = [];
for (let i = 0; i < lines.length; i++) {
  const m = lines[i].match(/^export (?:type |interface |class |enum |const |function |async function |\* )?(?:\{[^}]*\}|([A-Za-z0-9_]+))/);
  if (m && m[1]) allExports.push({line: i+1, name: m[1]});
}
console.log("named exports:", allExports.length);

// Content between header end and first regions - group by gaps
console.log("\n--- non-region chunks (approx) ---");
let inRegion = 0;
let chunkStart = null;
for (let i = 0; i < lines.length; i++) {
  if (/\/\/\s*#region/.test(lines[i])) {
    if (inRegion === 0 && chunkStart !== null) {
      console.log(`chunk ${chunkStart}-${i} (before region)`);
    }
    inRegion++;
    chunkStart = null;
  } else if (/\/\/\s*#endregion/.test(lines[i])) {
    inRegion = Math.max(0, inRegion-1);
    if (inRegion === 0) chunkStart = i+2;
  } else if (inRegion === 0 && chunkStart === null) {
    chunkStart = i+1;
  }
}
