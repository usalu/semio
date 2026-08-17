import { existsSync, readdirSync, statSync, readFileSync } from "fs";
import { join } from "path";

function findDirs(root, name, depth = 0, out = []) {
  if (depth > 10 || !existsSync(root)) return out;
  let ents;
  try { ents = readdirSync(root); } catch { return out; }
  for (const ent of ents) {
    if (["node_modules", "target", ".git", "dist"].includes(ent)) continue;
    const p = join(root, ent);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (!st.isDirectory()) continue;
    if (ent === name) out.push(p);
    findDirs(p, name, depth + 1, out);
  }
  return out;
}

function listDeep(dir, depth = 0, max = 4) {
  if (!existsSync(dir) || depth > max) return;
  for (const name of readdirSync(dir)) {
    if (["node_modules", "target"].includes(name)) continue;
    const p = join(dir, name);
    const st = statSync(p);
    console.log(`${"  ".repeat(depth)}${st.isDirectory() ? "D" : "F"} ${name}${st.isFile() ? ` ${st.size}` : ""}`);
    if (st.isDirectory()) listDeep(p, depth + 1, max);
  }
}

const flows = findDirs("/Users/ueli/Documents/semio/🧰️framework", "🌊️flow").filter((p) => p.includes("products") && p.includes("modules"));
console.log("flows", flows);
const flow = flows[0];
if (!flow) process.exit(1);
listDeep(flow, 0, 5);

for (const rel of ["📦️packages/🦀️rust/Cargo.toml", "🙰core/pkg/package.json", "🙰core/pkg/flow_core.js"]) {
  // try both emoji variants for core
}
const cargoHits = [];
function walkFiles(dir, depth = 0) {
  if (depth > 6 || !existsSync(dir)) return;
  for (const name of readdirSync(dir)) {
    if (["node_modules", "target"].includes(name)) continue;
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) walkFiles(p, depth + 1);
    else if (["Cargo.toml", "📜️script.ts", "package.json", "flow_core.js", "lib.rs"].includes(name) || name.endsWith(".rs")) {
      if (name === "Cargo.toml" || name === "📜️script.ts" || name === "package.json" || name === "flow_core.js" || name === "lib.rs") {
        console.log(`FILE ${p} (${st.size})`);
        if (name === "Cargo.toml" || name === "package.json" || (name === "flow_core.js" && st.size < 500)) {
          console.log(readFileSync(p, "utf8").slice(0, 600));
          console.log("---");
        }
      }
    }
  }
}
walkFiles(flow);
