import fs from "fs";
import path from "path";
import { spawnSync } from "child_process";

const root = "/Users/ueli/Documents/semio";
const framework = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const flowFw = path.join(root, framework, "🛍️products", "💻️os", "🔨️modules", "🌊️flow");
const fwExt = path.join(flowFw, fs.readdirSync(flowFw).find((n) => n.includes("extensions")));
const pluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));
const SIX = ["core", "math", "text", "logic", "dictionary", "list"];

const report = { ok: [], fail: [], notes: [] };

// 1) framework no longer has the six
const fwListing = fs.readdirSync(fwExt);
for (const id of SIX) {
  if (fwListing.some((n) => n.endsWith(id))) report.fail.push(`framework still has ${id}`);
  else report.ok.push(`framework removed ${id}`);
}
report.notes.push("framework extensions remain: " + fwListing.join(", "));

// 2) plugin packages exist
for (const id of SIX) {
  const dir = fs.readdirSync(pluginExt).find((n) => n.endsWith(id));
  const files = ["🦀️component.rs", "📦️packages/🦀️rust/Cargo.toml", "📦️packages/🦀️rust/📦️glue.rs", "📦️packages/🦀️rust/📜️script.ts", "📦️packages/🦀️rust/📋️project.json"];
  for (const f of files) {
    const p = path.join(pluginExt, dir, f);
    if (!fs.existsSync(p)) report.fail.push(`missing ${id}/${f}`);
  }
  const cargo = fs.readFileSync(path.join(pluginExt, dir, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
  if (!cargo.includes('role = "extension"')) report.fail.push(`${id} missing role`);
  if (!cargo.includes('extends = "flow"')) report.fail.push(`${id} missing extends`);
  if (!cargo.includes('contributes = ["flow.extension"]')) report.fail.push(`${id} missing contributes`);
  if (!cargo.includes(`package = "semio:flow-extension-${id}"`)) report.fail.push(`${id} wrong package id`);
  const comp = fs.readFileSync(path.join(pluginExt, dir, "🦀️component.rs"), "utf8");
  if (!comp.includes("extension_exports!")) report.fail.push(`${id} missing extension_exports`);
  if (!comp.includes("Contribution::FlowExtension")) report.fail.push(`${id} missing FlowExtension contrib`);
  if (!comp.includes('.handler("evaluate"')) report.fail.push(`${id} missing evaluate handler`);
  if (!comp.includes("pub fn register")) report.fail.push(`${id} missing register`);
  if (comp.includes("standalone-wasm") || comp.includes("WasmExt")) report.fail.push(`${id} still has wasm-pack path`);
  report.ok.push(`package ${id} structure ok`);
}

// 3) glue
const glue = fs.readFileSync(path.join(flowFw, "📦️packages", "🦀️rust", "📦️glue.rs"), "utf8");
for (const id of SIX) {
  if (new RegExp(`mod ${id === "core" ? "ext_core" : id}\\b`).test(glue) || glue.includes(`/${id}/`)) {
    // core id might match 🟀️core path of flow core - check extension path mods only
  }
}
if (/extensions\/(core|math|text|logic|dictionary|list)/.test(glue)) report.fail.push("glue still refs light ext paths");
if (!glue.includes("brep")) report.fail.push("glue missing brep");
if (!glue.includes("wasm")) report.fail.push("glue missing wasm");
if (glue.includes("mod draw") || glue.includes("/draw/")) report.notes.push("glue has draw (unexpected for wave3a if 3b migrated)");
report.ok.push("glue light builtins removed");

// 4) install_builtin
const core = fs.readFileSync(path.join(flowFw, fs.readdirSync(flowFw).find((n) => n.includes("core") && !n.includes("extensions")), "🦀️component.rs"), "utf8");
const builtin = core.match(/pub fn install_builtin_flow_extensions[\s\S]*?\n\}/)?.[0] || "";
for (const id of SIX) {
  if (builtin.includes(`flow_extension_${id}`)) report.fail.push(`install_builtin still calls ${id}`);
}
if (!builtin.includes("flow_extension_brep::register")) report.fail.push("install_builtin missing brep");
if (builtin.includes("flow_extension_draw")) report.notes.push("install_builtin still has draw");
report.ok.push("install_builtin only remaining builtins");
if (!core.includes("fn install_first_party_light_flow_extensions_for_tests")) report.fail.push("missing test helper");

// 5) workspace members
const cargoToml = fs.readFileSync(path.join(root, "Cargo.toml"), "utf8");
for (const id of SIX) {
  const dir = fs.readdirSync(pluginExt).find((n) => n.endsWith(id));
  if (!cargoToml.includes(`/${dir}/📦️packages/🦀️rust`)) report.fail.push(`workspace missing ${id}`);
  else report.ok.push(`workspace has ${id}`);
}

// 6) flow plugin consumes
const pluginCargo = fs.readFileSync(path.join(root, sDir, "🔌️plugins", "🌊️flow", "📦️packages", "🦀️rust", "Cargo.toml"), "utf8");
if (!pluginCargo.includes('consumes = ["flow.extension"]')) report.fail.push("flow plugin missing consumes");
else report.ok.push("flow plugin consumes flow.extension");

// 7) no stale framework refs to flow_extension_math etc outside tests helper
const stale = [];
function walk(dir, depth = 0) {
  if (depth > 12) return;
  let ents; try { ents = fs.readdirSync(dir, { withFileTypes: true }); } catch { return; }
  for (const ent of ents) {
    if (["target", "node_modules", "pkg", ".git"].includes(ent.name)) continue;
    const p = path.join(dir, ent.name);
    if (ent.isDirectory()) walk(p, depth + 1);
    else if (ent.name.endsWith(".rs")) {
      const t = fs.readFileSync(p, "utf8");
      for (const id of SIX) {
        const needle = `flow_extension_${id}::`;
        if (t.includes(needle) && !p.includes("extensions") && !t.includes("install_first_party_light")) {
          // allow in helper
        }
        if (t.includes(needle)) {
          const lines = t.split(/\n/);
          lines.forEach((l, i) => {
            if (l.includes(needle) && !l.includes("semio_s_plugin")) stale.push(`${p}:${i + 1}:${l.trim().slice(0, 100)}`);
          });
        }
      }
    }
  }
}
walk(flowFw);
report.notes.push("stale flow_extension_* refs:\n" + (stale.join("\n") || "(none)"));

// 8) cargo metadata (no compile)
const meta = spawnSync("cargo", ["metadata", "--no-deps", "--format-version", "1"], { cwd: root, encoding: "utf8", maxBuffer: 50_000_000 });
if (meta.status !== 0) {
  report.fail.push("cargo metadata failed: " + (meta.stderr || "").slice(0, 500));
} else {
  const data = JSON.parse(meta.stdout);
  const names = new Set(data.packages.map((p) => p.name));
  for (const id of SIX) {
    const n = `semio-s-plugin-flow-extension-${id}`;
    if (!names.has(n)) report.fail.push(`metadata missing package ${n}`);
    else report.ok.push(`metadata has ${n}`);
  }
}

console.log(JSON.stringify(report, null, 2));
fs.writeFileSync(path.join(root, ".🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS/wave3a-validate.json"), JSON.stringify(report, null, 2));
