import { readFileSync, writeFileSync } from "node:fs";
const [p, rowsFile, after] = process.argv.slice(2);
let raw = readFileSync(p, "utf8");
const rows = JSON.parse(readFileSync(rowsFile, "utf8"));
const fmt = (r) => JSON.stringify(r, null, 2).split("\n").map((l) => `    ${l}`).join("\n");
const bounds = (id) => {
  const at = raw.indexOf(`"id": "${id}"`);
  if (at < 0) return null;
  const start = raw.lastIndexOf("{", at);
  let depth = 0;
  for (let k = start; k < raw.length; k += 1) {
    if (raw[k] === "{") depth += 1;
    if (raw[k] === "}") { depth -= 1; if (depth === 0) return [start, k + 1]; }
  }
};
let anchor = after;
for (const r of rows) {
  const b = bounds(r.id);
  if (b) raw = `${raw.slice(0, b[0])}${fmt(r).trimStart()}${raw.slice(b[1])}`;
  else {
    const a = bounds(anchor);
    raw = `${raw.slice(0, a[1])},\n${fmt(r)}${raw.slice(a[1])}`;
  }
  anchor = r.id;
}
JSON.parse(raw);
writeFileSync(p, raw);
console.log("rows", rows.map((r) => r.id));
