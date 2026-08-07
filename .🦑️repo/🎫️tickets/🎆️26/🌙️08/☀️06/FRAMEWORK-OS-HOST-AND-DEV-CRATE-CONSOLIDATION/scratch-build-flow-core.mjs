import { existsSync, readdirSync, statSync, readFileSync } from "fs";
import { join } from "path";
import { $ } from "bun";

function findFiles(root, pred, depth = 0, out = []) {
  if (depth > 10 || !existsSync(root)) return out;
  let ents;
  try { ents = readdirSync(root); } catch { return out; }
  for (const name of ents) {
    if (["node_modules", "target", ".git", "dist"].includes(name)) continue;
    const p = join(root, name);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) findFiles(p, pred, depth + 1, out);
    else if (pred(name, p)) out.push(p);
  }
  return out;
}

const flowRoot = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const scripts = findFiles(flowRoot, (n, p) => n === "📜️script.ts");
console.log("scripts", scripts);
for (const s of scripts) {
  const t = readFileSync(s, "utf8");
  if (/wasm|wasm-pack/.test(t)) console.log("wasm-capable", s);
}

const pkgJs = findFiles(flowRoot, (n) => n === "flow_core.js");
console.log("pkgJs", pkgJs.map((p) => `${p} (${statSync(p).size}b)`));

// Prefer the rust package script under core
const coreScript = scripts.find((s) => s.includes("core") && s.includes("packages")) || scripts.find((s) => /core/.test(s));
console.log("chosen", coreScript);
