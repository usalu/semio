import fs from "node:fs";
import path from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const S_ROOT = path.join(REPO, "✏️s");

const EXTRACT_SUFFIXES = [
  "🔌️plugins/🖍️draw/🔄️fsm",
  "🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural",
  "🔌️plugins/🌊️flow/🧩️extensions/🏗️bim",
  "🔌️plugins/📜️imperative/🧩️extensions/🧠️logic",
  "🔌️plugins/📜️imperative/🧩️extensions/🫀️core",
  "🔌️plugins/📜️imperative/🧩️extensions/📝️text",
  "🔌️plugins/📜️imperative/🧩️extensions/🧮️math",
  "🔌️plugins/📜️imperative/🧩️extensions/🎮️control",
  "🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams",
  "🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs",
  "🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows",
];

const MACRO_SUFFIX = "🔌️plugins/🖍️draw/🔄️fsm/✨️macros";

function walk(dir, out = []) {
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, ent.name);
    if (ent.isDirectory()) walk(p, out);
    else if (ent.name === "📦️lib.rs" && p.includes(`${path.sep}📦️packages${path.sep}🦀️rust${path.sep}`)) out.push(p);
  }
  return out;
}

function ownerFromLib(libPath) {
  return path.dirname(path.dirname(path.dirname(libPath)));
}

function ownerSuffix(ownerPath) {
  return path.relative(S_ROOT, ownerPath).split(path.sep).join("/");
}

function updateCargoToml(cargoPath) {
  let t = fs.readFileSync(cargoPath, "utf8");
  if (!t.includes("📦️lib.rs")) return false;
  t = t.replace(/path = "📦️lib\.rs"/g, "path = \"📦️glue.rs\"");
  fs.writeFileSync(cargoPath, t);
  return true;
}

function thinGlue() {
  return `//! 📦️ Package glue — wiring only; domain in owner \`🦀️component.rs\`.

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;
`;
}

function extractStandard(libPath, ownerPath) {
  const lib = fs.readFileSync(libPath, "utf8");
  const componentPath = path.join(ownerPath, "🦀️component.rs");
  if (fs.existsSync(componentPath)) {
    const existing = fs.readFileSync(componentPath, "utf8");
    if (existing.trim() === lib.trim()) {
      fs.writeFileSync(path.join(path.dirname(libPath), "📦️glue.rs"), thinGlue());
      fs.unlinkSync(libPath);
      return "extract-skip-existing";
    }
    throw new Error(`component exists with different content: ${componentPath}`);
  }
  fs.writeFileSync(componentPath, lib);
  fs.writeFileSync(path.join(path.dirname(libPath), "📦️glue.rs"), thinGlue());
  fs.unlinkSync(libPath);
  return "extracted";
}

function extractProcMacroInclude(libPath, ownerPath) {
  const lib = fs.readFileSync(libPath, "utf8");
  const componentPath = path.join(ownerPath, "🦀️component.rs");
  if (fs.existsSync(componentPath)) throw new Error(`exists ${componentPath}`);
  fs.writeFileSync(componentPath, lib);
  const glue = `//! 📦️ Package glue — includes proc-macro crate body from owner \`🦀️component.rs\`.

include!("../../🦀️component.rs");
`;
  fs.writeFileSync(path.join(path.dirname(libPath), "📦️glue.rs"), glue);
  fs.unlinkSync(libPath);
  return "extracted-macro-include";
}

const renamed = [];
const extracted = [];
const errors = [];

for (const libPath of walk(S_ROOT)) {
  const ownerPath = ownerFromLib(libPath);
  const suffix = ownerSuffix(ownerPath);
  const cargoPath = path.join(path.dirname(libPath), "Cargo.toml");
  try {
    if (suffix === MACRO_SUFFIX) {
      extracted.push({ suffix, action: extractProcMacroInclude(libPath, ownerPath) });
    } else if (EXTRACT_SUFFIXES.includes(suffix)) {
      extracted.push({ suffix, action: extractStandard(libPath, ownerPath) });
    } else {
      const gluePath = path.join(path.dirname(libPath), "📦️glue.rs");
      fs.renameSync(libPath, gluePath);
      renamed.push(suffix);
    }
    updateCargoToml(cargoPath);
  } catch (e) {
    errors.push({ suffix, error: String(e) });
  }
}

const out = { renamed: renamed.length, extracted, errors };
fs.writeFileSync(
  path.join(REPO, ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/S-PLUGIN-PACKAGE-GLUE-PURITY/🧪glue-migrate-result.json"),
  JSON.stringify(out, null, 2),
);
console.log(JSON.stringify(out, null, 2));
