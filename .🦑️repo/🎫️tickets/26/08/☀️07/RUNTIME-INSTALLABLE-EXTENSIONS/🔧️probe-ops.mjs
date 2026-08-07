import fs from "fs";
import path from "path";

function dumpMatches(file, patterns, context = 3, limit = 40) {
  const lines = fs.readFileSync(file, "utf8").split("\n");
  console.log("\n====", file, "====");
  let n = 0;
  for (let i = 0; i < lines.length; i++) {
    if (patterns.some((p) => p.test(lines[i]))) {
      n++;
      if (n > limit) {
        console.log("...truncated");
        break;
      }
      const start = Math.max(0, i - context);
      const end = Math.min(lines.length, i + context + 1);
      for (let j = start; j < end; j++) console.log(`${j + 1}|${lines[j]}`);
      console.log("---");
    }
  }
}

for (const file of [
  "🧰️framework/🛍️products/💻️os/🦀️component.rs",
  "🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs",
]) {
  dumpMatches(file, [/InstallProgram|UninstallProgram|install_program|SpaceOperation::/]);
}

const runtimePath = path.join(
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine",
  "�모바elements",
  "PluginRuntime",
  "🟦️component.tsx",
);
// fix typo: ️ vs ️
const runtimeActual = fs
  .readdirSync("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine")
  .find((n) => n.includes("elements"));
const runtimeFile = path.join(
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine",
  runtimeActual,
  "PluginRuntime",
  "🟦️component.tsx",
);
console.log("runtimeFile", runtimeFile, fs.existsSync(runtimeFile));
const handle = fs.readFileSync(runtimeFile, "utf8");
const start = handle.indexOf("export type PluginWasmHandle");
console.log("\n==== PluginWasmHandle ====");
console.log(handle.slice(start, start + 1800));

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

const labelFiles = walk(".", (p, n) => {
  if (!/\.(ts|tsx|json)$/.test(n)) return false;
  if (p.includes("/.🦑️repo/") || p.includes("/node_modules/")) return false;
  try {
    return fs.readFileSync(p, "utf8").includes("ui.plugins.action");
  } catch {
    return false;
  }
});
console.log("\nLABEL FILES", labelFiles.join("\n"));
for (const f of labelFiles.slice(0, 5)) {
  const lines = fs.readFileSync(f, "utf8").split("\n");
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes("ui.plugins")) console.log(f + ":" + (i + 1) + ":" + lines[i].trim().slice(0, 160));
  }
}

// manifest consumes field
const coreTs = "🧰️framework/🔨️modules/🧩core/🟦️component.ts";
dumpMatches(coreTs, [/consumes|contributes|PluginManifest|role/, /export type PluginManifest/], 2, 30);
