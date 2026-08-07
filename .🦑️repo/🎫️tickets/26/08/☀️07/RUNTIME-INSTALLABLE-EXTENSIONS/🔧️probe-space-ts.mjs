import fs from "fs";
import path from "path";

function walk(dir, pred, acc = []) {
  let entries;
  try {
    entries = fs.readdirSync(dir, { withFileTypes: true });
  } catch {
    return acc;
  }
  for (const e of entries) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, pred, acc);
    else if (pred(p, e.name)) acc.push(p);
  }
  return acc;
}

const hits = walk(".", (p, n) => {
  if (!/\.(ts|tsx|rs)$/.test(n)) return false;
  if (p.includes("/.🦑️repo/") || p.includes("/node_modules/") || p.includes("/target/")) return false;
  try {
    const t = fs.readFileSync(p, "utf8");
    return /InstallProgram|install-program|SpaceProjection/.test(t) && /extensions|InstallExtension|programs/.test(t);
  } catch {
    return false;
  }
});
console.log("RELATED", hits.join("\n"));

// shell labels for plugins
const labelHits = walk(".", (p, n) => {
  if (!/\.(ts|tsx|json)$/.test(n)) return false;
  if (p.includes("/.🦑️repo/") || p.includes("/node_modules/")) return false;
  try {
    return fs.readFileSync(p, "utf8").includes("ui.plugins");
  } catch {
    return false;
  }
});
console.log("\nLABEL FILES", labelHits.slice(0, 20).join("\n"));

// PluginWasmHandle invoke
const runtimeFiles = walk("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer", (p, n) => n.includes("PluginRuntime") || n.includes("component"));
for (const p of runtimeFiles) {
  if (!p.includes("PluginRuntime")) continue;
  console.log("\nRUNTIME", p);
  const t = fs.readFileSync(p, "utf8");
  for (const line of t.split("\n")) {
    if (/invoke|export type PluginWasmHandle|interface PluginWasmHandle/.test(line)) console.log(line.trim().slice(0, 140));
  }
}

// How space ops get applied from TS
const spaceTs = walk(".", (p, n) => {
  if (!/\.(ts|tsx)$/.test(n)) return false;
  if (p.includes("/.🦑️repo/") || p.includes("/node_modules/")) return false;
  try {
    const t = fs.readFileSync(p, "utf8");
    return /installProgram|InstallProgram|space.*dispatch|applySpaceOp/.test(t);
  } catch {
    return false;
  }
});
console.log("\nSPACE TS", spaceTs.join("\n"));
