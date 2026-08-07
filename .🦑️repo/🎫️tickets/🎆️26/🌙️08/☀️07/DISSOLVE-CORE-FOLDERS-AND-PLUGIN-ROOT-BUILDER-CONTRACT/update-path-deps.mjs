import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join, relative } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const TICKET = "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️07/DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT";
const SKIP = new Set(["node_modules", "target", ".git", ".nx", "dist", "build", ".repo-cache", ".venv"]);

function walk(dir, acc = []) {
  let entries;
  try { entries = readdirSync(dir); } catch { return acc; }
  for (const name of entries) {
    if (SKIP.has(name) || name.startsWith(".")) continue;
    const p = join(dir, name);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walk(p, acc);
    else acc.push(p);
  }
  return acc;
}

const files = walk(ROOT).filter((f) => f.endsWith("Cargo.toml"));
const deferred = {
  rootCargoToml: [],
  pluginCargoToml: [],
  updatedPathDeps: [],
  deferredTsConsumers: [],
  updatedTsConsumers: [],
  notes: [],
};

for (const f of files) {
  const rel = relative(ROOT, f);
  let text = readFileSync(f, "utf8");
  if (!text.includes("semio-framework-core")) continue;

  if (rel === "Cargo.toml") {
    const lines = text.split("\n");
    for (let i = 0; i < lines.length; i++) {
      if (lines[i].includes("semio-framework-core")) {
        deferred.rootCargoToml.push({
          line: i + 1,
          text: lines[i].trim(),
          replacement: lines[i].replaceAll("semio-framework-core", "semio-framework").trim(),
        });
      }
    }
    continue;
  }

  if (rel.includes("🔌️plugins") || rel.startsWith("✏️s/")) {
    deferred.pluginCargoToml.push(rel);
    continue;
  }

  const next = text
    .replace(/package\s*=\s*"semio-framework-core"/g, 'package = "semio-framework"')
    .replace(/^semio-framework-core\s*=/gm, "semio-framework =");
  if (next !== text) {
    writeFileSync(f, next);
    deferred.updatedPathDeps.push(rel);
    console.log("updated", rel);
  }
}

const tsFiles = walk(ROOT).filter((f) => /\.(ts|tsx|json|mjs|cjs)$/.test(f));
for (const f of tsFiles) {
  const rel = relative(ROOT, f);
  if (rel.includes("🎫️tickets") || rel.includes(".cursor/plans")) continue;
  let text;
  try { text = readFileSync(f, "utf8"); } catch { continue; }
  if (!text.includes("@semio-tech/framework-core")) continue;

  if (
    rel === "package.json" ||
    rel === "📜️script.ts" ||
    rel === "script.ts" ||
    rel === ".dependency-cruiser.cjs" ||
    rel === "eslint.config.mjs" ||
    rel.startsWith(".storybook/") ||
    rel.includes("🔌️plugins") ||
    rel.startsWith("✏️s/") ||
    rel.startsWith("compose/")
  ) {
    deferred.deferredTsConsumers.push(rel);
    continue;
  }

  if (rel.startsWith("🧰️framework/")) {
    const next = text.replaceAll("@semio-tech/framework-core", "@semio-tech/framework");
    if (next !== text) {
      writeFileSync(f, next);
      deferred.updatedTsConsumers.push(rel);
      console.log("ts", rel);
    }
  } else {
    deferred.deferredTsConsumers.push(rel);
  }
}

deferred.notes.push("Root Cargo.toml workspace alias left for Wave 2");
deferred.notes.push("Plugin Cargo.toml and plugin TS imports deferred (do not touch plugin trees)");
deferred.notes.push("Shared roots (package.json workspaces, script.ts, storybook, eslint, compose) deferred");

writeFileSync(join(TICKET, "deferred-framework-core.json"), JSON.stringify(deferred, null, 2));
console.log(JSON.stringify({
  updatedPathDeps: deferred.updatedPathDeps.length,
  updatedTs: deferred.updatedTsConsumers.length,
  deferredPluginsCargo: deferred.pluginCargoToml.length,
  deferredTs: deferred.deferredTsConsumers.length,
  rootCargo: deferred.rootCargoToml.length,
}, null, 2));
