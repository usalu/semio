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
const report = { checks: [], errors: [] };

const fwList = fs.readdirSync(fwExt);
for (const id of SIX) {
  if (fwList.some((n) => n.endsWith(id))) report.errors.push(`fw still has ${id}`);
}
report.checks.push(`fw extensions: ${fwList.join(", ")}`);

const glue = fs.readFileSync(path.join(flowFw, "📦️packages", "🦀️rust", "📦️glue.rs"), "utf8");
report.checks.push("glue:\n" + glue);
for (const id of SIX) {
  if (glue.includes(`/${id}/`) || glue.includes(`mod ${id}`)) report.errors.push(`glue still mods ${id}`);
}

const core = fs.readFileSync(path.join(flowFw, fs.readdirSync(flowFw).find((n) => n.includes("core") && !n.includes("extensions")), "🦀️component.rs"), "utf8");
const builtin = core.match(/pub fn install_builtin_flow_extensions[\s\S]*?\n\}/)?.[0];
report.checks.push("install_builtin:\n" + builtin);
for (const id of SIX) if (builtin?.includes(id)) report.errors.push(`builtin still ${id}`);

const cargoToml = fs.readFileSync(path.join(root, "Cargo.toml"), "utf8");
for (const id of SIX) {
  const dir = fs.readdirSync(pluginExt).find((n) => n.endsWith(id));
  if (!cargoToml.includes(`/${dir}/📦️packages/🦀️rust`)) report.errors.push(`workspace missing ${id}`);
}

const pluginCargo = fs.readFileSync(path.join(root, sDir, "🔌️plugins", "🌊️flow", "📦️packages", "🦀️rust", "Cargo.toml"), "utf8");
if (!pluginCargo.includes('consumes = ["flow.extension"]')) report.errors.push("missing consumes");

const meta = spawnSync("cargo", ["metadata", "--no-deps", "--format-version", "1"], { cwd: root, encoding: "utf8", maxBuffer: 50e6 });
if (meta.status !== 0) report.errors.push("metadata: " + meta.stderr.slice(0, 400));
else {
  const names = new Set(JSON.parse(meta.stdout).packages.map((p) => p.name));
  for (const id of SIX) if (!names.has(`semio-s-plugin-flow-extension-${id}`)) report.errors.push(`meta missing ${id}`);
  report.checks.push("cargo metadata: all 6 packages present");
}

console.log(JSON.stringify(report, null, 2));
fs.writeFileSync(path.join(process.env.TICKET || ".", "wave3a-validate-final.json"), JSON.stringify(report, null, 2));
