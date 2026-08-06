import { readFileSync, readdirSync, statSync } from "fs";
import { join } from "path";
function findNamed(root, needle) {
  const hit = readdirSync(root).find((e) => e.includes(needle));
  if (!hit) throw new Error(`no ${needle} in ${root}`);
  return join(root, hit);
}
function walk(dir, depth = 0, out = []) {
  if (depth > 5) return out;
  for (const e of readdirSync(dir)) {
    const p = join(dir, e);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (st.isFile() && e === "Cargo.toml") out.push(p);
    else if (st.isDirectory() && e !== "target" && e !== "node_modules") walk(p, depth + 1, out);
  }
  return out;
}
const fw = findNamed(".", "framework");
const products = findNamed(fw, "products");
const os = findNamed(products, "os");
const modules = findNamed(os, "modules");
const dsl = findNamed(modules, "dsl");
for (const p of walk(dsl)) {
  const t = readFileSync(p, "utf8");
  console.log(p);
  console.log(" ", t.match(/\[lib\][\s\S]{0,120}/)?.[0]?.replace(/\n/g, " | "));
  console.log(" ", t.match(/name = "[^"]+"/)?.[0]);
}
