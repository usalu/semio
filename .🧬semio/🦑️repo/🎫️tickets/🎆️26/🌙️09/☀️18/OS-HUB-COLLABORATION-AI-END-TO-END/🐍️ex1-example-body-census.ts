/**
 * 📚️ EX1 — how much of every committed descriptor is inlined example bodies.
 * One row per owner: descriptor bytes, total `manifest.examples[].artifactJson` bytes, the largest
 * single body, and what the descriptor would weigh with bodies above a candidate threshold removed.
 * Usage: bun 🐍️ex1-example-body-census.ts [threshold-bytes]
 */
import { readdirSync, readFileSync, statSync, existsSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../../../../../../..", import.meta.url));
const threshold = Number(process.argv[2] ?? 262144);

type Row = { owner: string; json: number; bodies: number; count: number; largest: number; over: number; overBytes: number };

const rows: Row[] = [];

function scan(dir: string, owner: string): void {
  const descriptorPath = join(dir, "🔣️.json");
  if (!existsSync(descriptorPath)) return;
  const text = readFileSync(descriptorPath, "utf8");
  let descriptor: { manifest?: { examples?: { id?: string; artifactJson?: string }[] } };
  try {
    descriptor = JSON.parse(text) as typeof descriptor;
  } catch {
    return;
  }
  const examples = descriptor.manifest?.examples ?? [];
  let bodies = 0;
  let largest = 0;
  let over = 0;
  let overBytes = 0;
  for (const example of examples) {
    const size = Buffer.byteLength(example.artifactJson ?? "", "utf8");
    bodies += size;
    if (size > largest) largest = size;
    if (size > threshold) {
      over += 1;
      overBytes += size;
    }
  }
  rows.push({ owner, json: statSync(descriptorPath).size, bodies, count: examples.length, largest, over, overBytes });
}

const pluginsRoot = join(root, "✏️s/🔌️plugins");
for (const entry of readdirSync(pluginsRoot, { withFileTypes: true })) {
  if (!entry.isDirectory()) continue;
  scan(join(pluginsRoot, entry.name), entry.name);
  const extensionsRoot = join(pluginsRoot, entry.name, "🧩️extensions");
  if (!existsSync(extensionsRoot)) continue;
  for (const extension of readdirSync(extensionsRoot, { withFileTypes: true })) {
    if (!extension.isDirectory()) continue;
    scan(join(extensionsRoot, extension.name), `${entry.name}/${extension.name}`);
  }
}

rows.sort((left, right) => right.bodies - left.bodies);
console.log(`threshold = ${threshold} bytes`);
console.log(["owner", "jsonBytes", "examples", "bodyBytes", "bodyShare", "largestBody", "overThreshold", "jsonAfterSplit"].join("\t"));
let totalOver = 0;
for (const row of rows) {
  if (row.bodies === 0 && row.count === 0) continue;
  totalOver += row.over;
  console.log([row.owner, row.json, row.count, row.bodies, `${((row.bodies / row.json) * 100).toFixed(1)}%`, row.largest, row.over, row.json - row.overBytes].join("\t"));
}
console.log(`owners with a descriptor: ${rows.length}; example rows over threshold: ${totalOver}`);
