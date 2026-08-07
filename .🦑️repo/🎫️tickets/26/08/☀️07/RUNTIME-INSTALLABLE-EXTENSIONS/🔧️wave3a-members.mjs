import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const pluginExtName = fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions"));
const pluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow", pluginExtName);
const SIX = ["core", "math", "text", "logic", "dictionary", "list"];
const cargoPath = path.join(root, "Cargo.toml");
let cargo = fs.readFileSync(cargoPath, "utf8");
const bimLine = cargo.split("\n").find((l) => l.includes("bim") && l.includes("flow") && l.includes("extensions"));
if (!bimLine) throw new Error("no bim line");
let added = [];
for (const id of SIX) {
  const dir = fs.readdirSync(pluginExt).find((n) => n.endsWith(id));
  const memberPath = `✏️s/🔌️plugins/🌊️flow/${pluginExtName}/${dir}/📦️packages/🦀️rust`;
  if (cargo.includes(`"${memberPath}"`)) {
    console.log("already", id);
    continue;
  }
  const member = `    "${memberPath}",`;
  // insert after bim each time refreshing bimLine position
  cargo = cargo.replace(`    "${memberPath.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}",\n`, ""); // noop
  const lines = cargo.split("\n");
  const idx = lines.findIndex((l) => l.includes("bim") && l.includes("flow") && l.includes("extensions"));
  lines.splice(idx + 1, 0, member);
  cargo = lines.join("\n");
  added.push(id);
}
fs.writeFileSync(cargoPath, cargo);
console.log("added", added);
console.log(cargo.split("\n").filter((l) => l.includes("flow") && l.includes("extensions")).join("\n"));

// Also fix fixture_kind_infos_json body if incomplete
const fw = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const flowFw = path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🌊️flow");
const corePath = path.join(flowFw, fs.readdirSync(flowFw).find((n) => n.includes("core") && !n.includes("extensions")), "🦀️component.rs");
let core = fs.readFileSync(corePath, "utf8");
const m = core.match(/fn fixture_kind_infos_json\(\) -> String \{[\s\S]*?\n    \}/);
console.log("fixture body:\n", m?.[0]);
