import { readFileSync, readdirSync, existsSync, lstatSync } from "fs";
import { join } from "path";
function findNamed(root, needle) {
  const entries = readdirSync(root);
  const hit = entries.find((e) => e.includes(needle));
  if (!hit) throw new Error(`no ${needle} in ${root}: ${entries}`);
  return join(root, hit);
}
const fw = findNamed(".", "framework");
const products = findNamed(fw, "products");
const os = findNamed(products, "os");
const modules = findNamed(os, "modules");
const dsl = findNamed(modules, "dsl");
console.log("dsl entries", readdirSync(dsl).map((e) => ({ e, len: e.length, codes: [...e].slice(0,3).map(c=>c.codePointAt(0).toString(16)) })));
const implEntry = readdirSync(dsl).find((e) => e.includes("implementations") || e.includes("⚡"));
console.log("implEntry", implEntry);
const known = join(dsl, "⚡️implementations", "🦀️rust", "📦️lib.rs");
console.log("known exists", existsSync(known), known);
const comp = join(dsl, "🦀️component.rs");
for (const p of [known, comp].filter(existsSync)) {
  const t = readFileSync(p, "utf8");
  const i = t.indexOf("pub fn hooks_for");
  console.log("====", p);
  console.log(t.slice(Math.max(0,i-50), i + 900));
  console.log("passthrough count", (t.match(/pub fn passthrough_hooks/g) || []).length);
  // detect corruption
  if (t.includes("te }\n\n/// @emoji 🪞")) console.log("CORRUPT te }");
  if (t.includes("IdiomHooks {\n    IdiomHooks")) console.log("CORRUPT nested");
}
