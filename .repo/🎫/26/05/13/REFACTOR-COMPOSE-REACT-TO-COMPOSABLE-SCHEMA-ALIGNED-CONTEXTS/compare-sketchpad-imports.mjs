import fs from "node:fs";

const react = fs.readFileSync("compose/client/lib/react/index.tsx", "utf8");
const sketch = fs.readFileSync("compose/client/lib/sketchpad/index.tsx", "utf8");
const block = sketch.split("import { gunzipSync }")[0];
const names = new Set();

function grab(inner) {
  for (const raw of inner.split(",")) {
    let p = raw.trim();
    if (!p) continue;
    const as = p.match(/^(\w+)\s+as\s+(\w+)/);
    if (as) {
      names.add(as[2]);
      continue;
    }
    if (p.startsWith("type ")) {
      const r = p.slice(5).trim();
      const am = r.match(/^(\w+)\s+as\s+(\w+)/);
      names.add(am ? am[2] : r.split(/\s/)[0]);
      continue;
    }
    names.add(p.split(/\s/)[0]);
  }
}

const m1 = block.match(/import type\s*\{([\s\S]*?)\}\s*from\s*["']@compose\/react["']/);
if (m1) grab(m1[1]);
const m2 = block.match(/import\s*\{([\s\S]*?)\}\s*from\s*["']@compose\/react["']/);
if (m2) grab(m2[1]);

const exported = new Set();
const re = /^export (function|const|type|class|async function) (\w+)/gm;
let x;
while ((x = re.exec(react))) exported.add(x[2]);

const missing = [...names].filter((n) => !exported.has(n) && !/^useSchema/.test(n)).sort();
console.log("imported", names.size, "missing", missing.length);
for (const n of missing) console.log(n);
