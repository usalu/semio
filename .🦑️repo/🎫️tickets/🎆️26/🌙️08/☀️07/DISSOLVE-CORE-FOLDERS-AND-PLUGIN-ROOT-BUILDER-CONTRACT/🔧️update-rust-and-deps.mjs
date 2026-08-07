import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join, relative } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const RUST = join(ROOT, "🧰️framework/📦️packages/🦀️rust");
const PKG = join(ROOT, "🧰️framework/📦️packages/🟦️typescript");
const TICKET = join(
  ROOT,
  ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️07/DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT",
);

//#region Update glue.rs
{
  const gluePath = join(RUST, "📦️glue.rs");
  let glue = readFileSync(gluePath, "utf8");
  glue = glue.replace(
    `#[path = "../../🔨️modules/🛂️manifest/🦀️component.rs"]
pub mod ui;`,
    `#[path = "../../🔨️modules/�️️manifest/🦀️component.rs"]
pub mod manifest;`,
  );
  // fix typo if introduced
  glue = glue.replace("�️️manifest", "🛂️manifest");
  if (!glue.includes("pub mod manifest;")) {
    glue = glue.replace("pub mod ui;", "pub mod manifest;");
  }
  glue = glue.replace(
    "see `pub mod ui`.",
    "see `pub mod manifest`.",
  );
  glue = glue.replace(/pub use ui::\*;/, "pub use manifest::*;\npub use manifest as ui;");
  glue = glue.replace("pub use ui::kernel::{", "pub use manifest::kernel::{");
  // dedupe if run twice
  glue = glue.replace(
    /pub use manifest::\*;\npub use manifest as ui;\npub use manifest as ui;/,
    "pub use manifest::*;\npub use manifest as ui;",
  );
  writeFileSync(gluePath, glue);
  console.log("updated glue.rs");
}
//#endregion

//#region Update Cargo.toml metadata
{
  const cargoPath = join(RUST, "Cargo.toml");
  let cargo = readFileSync(cargoPath, "utf8");
  cargo = cargo.replace('name = "semio-framework-core"', 'name = "semio-framework"');
  cargo = cargo.replace('id = "core"', 'id = "framework"');
  cargo = cargo.replace(
    "see `<owner>/🤖️generated/🟦️manifest.ts`",
    "see `🔨️modules/�️️manifest/🤖️generated/