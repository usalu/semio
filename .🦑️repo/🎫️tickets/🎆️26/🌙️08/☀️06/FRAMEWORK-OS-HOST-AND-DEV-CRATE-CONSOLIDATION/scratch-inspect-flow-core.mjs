import { existsSync, readdirSync, readFileSync } from "fs";
import { join } from "path";

const roots = [
  "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow",
  "/Users/ueli/Documents/semio/node_modules/@semio-tech/flow-core",
];

function walk(dir, pred, out=[], depth=0) {
  if (depth > 8 || !existsSync(dir)) return out;
  for (const name of readdirSync(dir)) {
    if (name === "node_modules" || name === "target" || name === ".git") continue;
    const p = join(dir, name);
    try {
      const st = Bun.file(p);
      // use sync fs
    } catch {}
    try {
      const { statSync } = require("fs");
    } catch {}
  }
  return out;
}

import { statSync } from "fs";
function walk2(dir, out=[], depth=0) {
  if (depth > 10 || !existsSync(dir)) return out;
  let ents;
  try { ents = readdirSync(dir); } catch { return out; }
  for (const name of ents) {
    if (["node_modules","target",".git","pkg-node"].includes(name)) continue;
    const p = join(dir, name);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walk2(p, out, depth+1);
    else if (name === "package.json" || name === "flow_core.js") out.push(p);
  }
  return out;
}

for (const root of roots) {
  console.log("ROOT", root, "exists", existsSync(root));
  if (!existsSync(root)) continue;
  const files = walk2(root);
  for (const f of files) console.log(" ", f);
}

// resolve via bun import meta from workspace
const pkgCandidates = walk2("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow").filter(f => f.endsWith("package.json"));
for (const f of pkgCandidates) {
  const j = JSON.parse(readFileSync(f, "utf8"));
  if (j.name === "@semio-tech/flow-core" || /flow/.test(j.name||"")) {
    console.log("PKG", f, j.name, JSON.stringify({exports:j.exports, main:j.main, module:j.module}));
  }
}

// also check vite alias resolution by reading demonstrator vite config
const vite = "/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts";
if (existsSync(vite)) {
  const t = readFileSync(vite, "utf8");
  const hits = [...t.matchAll(/flow-core[^\n]*/g)].slice(0,20);
  console.log("vite flow-core mentions", hits.map(h=>h[0]));
}
