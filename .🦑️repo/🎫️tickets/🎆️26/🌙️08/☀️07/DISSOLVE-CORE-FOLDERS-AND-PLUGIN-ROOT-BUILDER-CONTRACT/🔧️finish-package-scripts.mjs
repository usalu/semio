import { readFileSync, writeFileSync, readdirSync, existsSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const RUST = join(ROOT, "🧰️framework/📦️packages/🦀️rust");
const TS = join(ROOT, "🧰️framework/📦️packages/🟦️typescript");
const MODULES = join(ROOT, "🧰️framework/🔨️modules");
const TICKET = join(
  ROOT,
  ".🦑️repo/🎫️tickets/🎆️26/� comb08/☀️07/DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT",
);

const manifestDir = readdirSync(MODULES).find((n) => n.includes("manifest"));

// Package rust script
{
  const script = readdirSync(RUST).find((n) => n.includes("script") && n.endsWith(".ts"));
  const p = join(RUST, script);
  let t = readFileSync(p, "utf8");
  t = t.replaceAll("semio-framework-core", "semio-framework");
  t = t.replaceAll("framework-core", "framework");
  t = t.replaceAll("🧩core", manifestDir);
  writeFileSync(p, t);
  console.log("updated", script);
}

// Package rust project.json
{
  const proj = readdirSync(RUST).find((n) => n.includes("project") && n.endsWith(".json"));
  if (proj) {
    const p = join(RUST, proj);
    let t = readFileSync(p, "utf8");
    const next = t.replaceAll("@semio-tech/framework-core", "@semio-tech/framework").replaceAll("framework-core", "framework");
    if (next !== t) {
      writeFileSync(p, next);
      console.log("updated", proj);
    } else {
      console.log(proj, "no change; content sample:");
      for (const line of t.split("\n")) if (/name|framework/.test(line)) console.log(" ", line.trim());
    }
  }
}

// Package TS project.json / script / package already renamed
{
  for (const name of readdirSync(TS)) {
    if (!/\.(json|ts)$/.test(name)) continue;
    const p = join(TS, name);
    let t = readFileSync(p, "utf8");
    const next = t
      .replaceAll("@semio-tech/framework-core", "@semio-tech/framework")
      .replaceAll("framework-core", "framework")
      .replaceAll("🧩core", manifestDir);
    if (next !== t) {
      writeFileSync(p, next);
      console.log("updated ts pkg", name);
    }
  }
}

// Fix generated files that mention old package
{
  const gen = join(MODULES, manifestDir, "🤖️generated");
  if (existsSync(gen)) {
    for (const name of readdirSync(gen)) {
      const p = join(gen, name);
      let t = readFileSync(p, "utf8");
      const next = t.replaceAll("@semio-tech/framework-core", "@semio-tech/framework").replaceAll("semio-framework-core", "semio-framework");
      if (next !== t) {
        writeFileSync(p, next);
        console.log("updated generated", name);
      }
    }
  }
}

// Ensure deferred json is solid
const deferred = {
  rootCargoToml: [
    {
      status: "already-applied-or-observed",
      observed: 'semio-framework = { path = "🧰️framework/📦️packages/🦀️rust" }  # 58 refs',
      note: "Wave 2 should confirm no remaining semio-framework-core workspace aliases and regenerate lock if needed.",
    },
  ],
  pluginCargoTomlStillReferencingSemioFrameworkCore: [],
  sharedFileEditsDeferred: [
    {
      file: "📜️script.ts",
      note: "Policy region ownership is Wave 0/2/4 — any hardcoded 🧩core / framework-core path strings outside package-local scripts need Wave 2 integration.",
    },
    {
      file: "package.json",
      note: "Workspace package name listing if it still mentions framework-core (scan showed typescript package path already listed).",
    },
    {
      file: ".storybook/scopes.ts",
      note: "Source roots may still mention 🧩core — Wave 2.",
    },
    {
      file: "eslint.config.mjs",
      note: "May still mention 🧩core path — Wave 2.",
    },
    {
      file: ".dependency-cruiser.cjs",
      note: "no-core-path already from Wave 0; confirm no allowlist entries for 🧩core — Wave 2.",
    },
  ],
  notes: [
    "Crate renamed to semio-framework; non-plugin path deps updated to package = semio-framework / dep key semio-framework.",
    "TS package renamed to @semio-tech/framework; consumer package.json/import strings no longer contain @semio-tech/framework-core in repo scan.",
    "Plugin Cargo.toml files still listing semio-framework-core are deferred (do not touch plugin trees in this wave).",
  ],
};

function walk(dir, acc = []) {
  for (const name of readdirSync(dir)) {
    if (["node_modules", "target", ".git", ".nx", "dist", "build"].includes(name) || name.startsWith(".")) continue;
    const p = join(dir, name);
    try {
      const st = require("fs").statSync(p);
      if (st.isDirectory()) walk(p, acc);
      else acc.push(p);
    } catch {}
  }
  return acc;
}

const { statSync } = await import("fs");
function walk2(dir, acc = []) {
  let entries;
  try {
    entries = readdirSync(dir);
  } catch {
    return acc;
  }
  for (const name of entries) {
    if (["node_modules", "target", ".git", ".nx", "dist", "build"].includes(name) || name.startsWith(".")) continue;
    const p = join(dir, name);
    let st;
    try {
      st = statSync(p);
    } catch {
      continue;
    }
    if (st.isDirectory()) walk2(p, acc);
    else acc.push(p);
  }
  return acc;
}

for (const f of walk2(ROOT).filter((f) => f.endsWith("Cargo.toml"))) {
  const rel = f.slice(ROOT.length + 1);
  if (rel.includes("🎫️tickets")) continue;
  const t = readFileSync(f, "utf8");
  if (t.includes("semio-framework-core")) deferred.pluginCargoTomlStillReferencingSemioFrameworkCore.push(rel);
}

// Find real ticket dir
const ticketsRoot = join(ROOT, ".🦑️repo/🎫️tickets");
function findTicket(dir) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    let st;
    try {
      st = statSync(p);
    } catch {
      continue;
    }
    if (!st.isDirectory()) continue;
    if (name === "DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT") return p;
    if (!name.startsWith(".") && name.length < 40) {
      const hit = findTicket(p);
      if (hit) return hit;
    }
  }
  return null;
}
const ticket = findTicket(ticketsRoot);
console.log("ticket", ticket);
writeFileSync(join(ticket, "deferred-framework-core.json"), JSON.stringify(deferred, null, 2));
console.log("wrote deferred-framework-core.json", deferred.pluginCargoTomlStillReferencingSemioFrameworkCore.length, "plugin cargos");
