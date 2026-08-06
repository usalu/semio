import fs from "fs";
import path from "path";

const OS = fs.readFileSync("/tmp/os-path.txt","utf8").trim();
const FW = fs.readFileSync("/tmp/fw-path.txt","utf8").trim();
const TICKET = fs.readFileSync("/tmp/os-ticket-path.txt","utf8").trim();
const REPO = "/Users/ueli/Documents/semio";
const KERNEL_PKG = path.join(OS, "📦️packages/🦀️rust");
const HOST_PKG = path.join(OS, "🖥️host/📦️packages/🦀️rust");
const DB_PKG = path.join(OS, "🔨️modules/🛢️db/📦️packages/🦀️rust");
const RUN_PKG = path.join(OS, "🔨️modules/🏃️run/📦️packages/🦀️rust");
const PLUGIN_HOST_PKG = path.join(OS, "🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust");
const NEURAL_PKG = path.join(OS, "🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust");
const PLUGIN_PKG = path.join(OS, "🔨️modules/🔌️plugin/📦️packages/🦀️rust");
const DERIVE_PKG = path.join(OS, "🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust");
const log = [];
const note = (m) => { console.log(m); log.push(m); };

function relTo(fromDir, toPath) {
  return path.relative(fromDir, toPath).replaceAll("\\", "/");
}

function renameLibToGlue(pkgDir) {
  const lib = path.join(pkgDir, "📦️lib.rs");
  const glue = path.join(pkgDir, "📦️glue.rs");
  const cargoPath = path.join(pkgDir, "Cargo.toml");
  if (!fs.existsSync(cargoPath)) return;
  if (fs.existsSync(lib)) {
    if (!fs.existsSync(glue)) fs.renameSync(lib, glue);
    else { fs.writeFileSync(glue, fs.readFileSync(lib)); fs.unlinkSync(lib); }
  }
  if (fs.existsSync(cargoPath)) {
    let c = fs.readFileSync(cargoPath, "utf8");
    const n = c.replace(/path = "📦️lib\.rs"/g, 'path = "📦️glue.rs"');
    if (n !== c) { fs.writeFileSync(cargoPath, n); note("lib->glue " + path.relative(REPO, pkgDir)); }
  }
}

// ---- HOST ----
{
  const hostComp = path.join(OS, "🖥️host/🦀️component.rs");
  const glue = `//! 🖥️ Semio framework OS host — Shape V2 glue.
#![feature(linkage)]

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as spr;

#[path = "../../../🦀️component.rs"]
mod host_core;
pub use host_core::*;
`;
  fs.writeFileSync(path.join(HOST_PKG, "📦️glue.rs"), glue);
  let cargo = fs.readFileSync(path.join(HOST_PKG, "Cargo.toml"), "utf8");
  cargo = cargo.replace(/path = "📦️lib\.rs"/g, 'path = "📦️glue.rs"');
  const kp = relTo(HOST_PKG, KERNEL_PKG);
  cargo = cargo.replace(/vcs = \{ path = "[^"]+", package = "[^"]+" \}/, `vcs = { path = "${kp}", package = "semio-framework-os-kernel" }`);
  cargo = cargo.replace(/store = \{ path = "[^"]+", package = "[^"]+" \}/, `store = { path = "${kp}", package = "semio-framework-os-kernel" }`);
  cargo = cargo.replace(/protocol = \{ path = "[^"]+", package = "[^"]+" \}/, `protocol = { path = "${kp}", package = "semio-framework-os-kernel" }`);
  cargo = cargo.replace(/dsl = \{ path = "[^"]+", package = "[^"]+" \}/, `dsl = { path = "${kp}", package = "semio-framework-os-kernel" }`);
  cargo = cargo.replace(/semio-framework-plugin = \{ path = "[^"]+", package = "[^"]+" \}/,
    `semio-framework-plugin = { path = "${relTo(HOST_PKG, PLUGIN_PKG)}", package = "semio-framework-plugin" }`);
  cargo = cargo.replace(/workflow = \{ path = "[^"]+", package = "[^"]+" \}\n/, "");
  cargo = cargo.replace(/space = \{ path = "[^"]+", package = "[^"]+" \}\n/, "");
  cargo = cargo.replace(/store_sync = \{ path = "[^"]+", package = "[^"]+" \}/,
    `store_sync = { path = "${kp}", package = "semio-framework-os-kernel", features = ["sync"] }`);
  fs.writeFileSync(path.join(HOST_PKG, "Cargo.toml"), cargo);
  if (fs.existsSync(path.join(HOST_PKG, "📦️lib.rs"))) fs.unlinkSync(path.join(HOST_PKG, "📦️lib.rs"));
  note("host facade done");
}

// ---- DB ----
{
  const dbRoot = path.join(OS, "🔨️modules/🛢️db");
  const lines = ["//! 🗄️ Db facade — Shape V2 #[path] glue.", ""];
  lines.push(`extern crate semio_framework_os_kernel as pack;`);
  lines.push(`extern crate semio_framework_os_kernel as protocol;`);
  lines.push(`extern crate semio_framework_os_kernel as store;`);
  lines.push(`extern crate semio_framework_os_kernel as dsl;`);
  lines.push("");

  function addMod(absComp, modName, cfg) {
    if (!fs.existsSync(absComp)) return;
    const rel = relTo(DB_PKG, absComp);
    if (cfg) lines.push(`#[cfg(${cfg})]`);
    lines.push(`#[path = "${rel}"]`);
    lines.push(`pub mod ${modName};`);
    lines.push("");
  }

  // facade component first as root reexport
  const facade = path.join(dbRoot, "🦀️component.rs");
  if (fs.existsSync(facade)) {
    lines.push(`#[path = "${relTo(DB_PKG, facade)}"]`);
    lines.push(`mod db_facade;`);
    lines.push(`pub use db_facade::*;`);
    lines.push("");
  }

  for (const name of fs.readdirSync(dbRoot)) {
    const p = path.join(dbRoot, name);
    if (!fs.statSync(p).isDirectory()) continue;
    if (["📦️packages", "⚡️implementations"].includes(name)) continue;
    const comp = path.join(p, "🦀️component.rs");
    const key = name.replace(/^[\p{Emoji_Presentation}\p{Extended_Pictographic}\uFE0F\u200D]+/gu, "").replace(/️/g, "");
    const mod = "db_" + key.replace(/[^a-zA-Z0-9_]/g, "_");
    addMod(comp, mod);
    if (name.includes("storage") || key === "storage") {
      for (const b of fs.readdirSync(p)) {
        const bp = path.join(p, b);
        if (!fs.statSync(bp).isDirectory() || b === "⚡️implementations") continue;
        const bc = path.join(bp, "🦀️component.rs");
        const bkey = b.replace(/^[\p{Emoji_Presentation}\p{Extended_Pictographic}\uFE0F\u200D]+/gu, "").replace(/️/g, "");
        const feat = bkey.replace(/[^a-zA-Z0-9_]/g, "_");
        addMod(bc, "db_storage_" + feat, `feature = "${feat}"`);
      }
    }
  }

  fs.writeFileSync(path.join(DB_PKG, "📦️glue.rs"), lines.join("\n") + "\n");
  const newDbCargo = `[package]
name = "semio-framework-os-kernel-db"
version = "0.1.0"
edition = "2021"
rust-version = "1.88"
description = "Db facade — Shape V2 #[path] glue over owner-tree db components"

[package.metadata.semio]
role = "product"
id = "os-db"

[lints]
workspace = true

[lib]
name = "db"
path = "📦️glue.rs"

[features]
default = ["fs", "thread", "deflate", "vcs"]
fs = []
thread = []
deflate = []
vcs = []
tokio = []
cluster = []
otel = []
sqlite = []
postgres = []
neo4j = []

[dependencies]
semio-framework-os-kernel = { path = "../../../../📦️packages/🦀️rust", package = "semio-framework-os-kernel" }
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
thiserror = "2.0.12"
async-trait = "0.1"
`;
  fs.writeFileSync(path.join(DB_PKG, "Cargo.toml"), newDbCargo);
  if (fs.existsSync(path.join(DB_PKG, "📦️lib.rs"))) fs.unlinkSync(path.join(DB_PKG, "📦️lib.rs"));
  note("db facade done");
}

// ---- RUN ----
{
  fs.writeFileSync(path.join(RUN_PKG, "📦️glue.rs"),
    `//! 🏃️ Headless OS workflow runner (Shape V2 entry).\n#[path = "../../🦀️component.rs"]\nmod run_lib;\npub use run_lib::*;\n`);
  let cargo = fs.readFileSync(path.join(RUN_PKG, "Cargo.toml"), "utf8");
  cargo = cargo.replace(/path = "📦️lib\.rs"/g, 'path = "📦️glue.rs"');
  cargo = cargo.replace(/path = "[^"]*⚡️implementations[^"]*📦️bin\.rs"/, 'path = "../../📦️bin.rs"');
  const kp = relTo(RUN_PKG, KERNEL_PKG);
  cargo = cargo.replace(/path = "[^"]*⚡️implementations[^"]*"/g, `path = "${kp}"`);
  cargo = cargo.replace(/package = "semio-framework-os-kernel-[a-z0-9-]+"/g, 'package = "semio-framework-os-kernel"');
  cargo = cargo.replace(/semio-framework-plugin = \{ path = "[^"]+", package = "[^"]+" \}/,
    `semio-framework-plugin = { path = "${relTo(RUN_PKG, PLUGIN_PKG)}", package = "semio-framework-plugin" }`);
  cargo = cargo.replace(/semio-framework-plugin-host = \{ path = "[^"]+", package = "[^"]+" \}/,
    `semio-framework-plugin-host = { path = "${relTo(RUN_PKG, PLUGIN_HOST_PKG)}", package = "semio-framework-plugin-host" }`);
  cargo = cargo.replace(/semio-framework-os = \{ path = "[^"]+", package = "[^"]+" \}/,
    `semio-framework-os = { path = "${relTo(RUN_PKG, HOST_PKG)}", package = "semio-framework-os" }`);
  fs.writeFileSync(path.join(RUN_PKG, "Cargo.toml"), cargo);
  if (fs.existsSync(path.join(RUN_PKG, "📦️lib.rs"))) fs.unlinkSync(path.join(RUN_PKG, "📦️lib.rs"));
  note("run facade done");
}

// ---- PLUGIN HOST ----
{
  fs.writeFileSync(path.join(PLUGIN_HOST_PKG, "📦️glue.rs"),
    `//! 🖥️ Plugin host — Shape V2 glue.\n#[path = "../../🦀️component.rs"]\nmod component;\npub use component::*;\n`);
  let cargo = fs.readFileSync(path.join(PLUGIN_HOST_PKG, "Cargo.toml"), "utf8");
  cargo = cargo.replace(/path = "📦️lib\.rs"/g, 'path = "📦️glue.rs"');
  const kp = relTo(PLUGIN_HOST_PKG, KERNEL_PKG);
  cargo = cargo.replace(/path = "[^"]*⚡️implementations[^"]*"/g, `path = "${kp}"`);
  cargo = cargo.replace(/package = "semio-framework-os-kernel-[a-z0-9-]+"/g, 'package = "semio-framework-os-kernel"');
  fs.writeFileSync(path.join(PLUGIN_HOST_PKG, "Cargo.toml"), cargo);
  if (fs.existsSync(path.join(PLUGIN_HOST_PKG, "📦️lib.rs"))) fs.unlinkSync(path.join(PLUGIN_HOST_PKG, "📦️lib.rs"));
  note("plugin-host facade done");
}

// ---- NEURAL ----
{
  fs.writeFileSync(path.join(NEURAL_PKG, "📦️glue.rs"),
    `//! 🧠️ Neural engine — Shape V2 glue.\n#[path = "../../🦀️component.rs"]\nmod component;\npub use component::*;\n`);
  renameLibToGlue(NEURAL_PKG);
  note("neural facade done");
}

renameLibToGlue(PLUGIN_PKG);
renameLibToGlue(DERIVE_PKG);
const renderer = path.join(OS, "🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu");
if (fs.existsSync(renderer)) renameLibToGlue(renderer);

// ---- REPOINT outside-os consumers ----
note("== repoint consumers ==");
function walkToml(dir, out=[]) {
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    if (["target","node_modules",".git"].includes(ent.name)) continue;
    const p = path.join(dir, ent.name);
    if (ent.isDirectory()) walkToml(p, out);
    else if (ent.name === "Cargo.toml") out.push(p);
  }
  return out;
}
const kernelPkgs = new Set([
  "semio-framework-os-kernel-dsl","semio-framework-os-kernel-dsl-core","semio-framework-os-kernel-dsl-grammar",
  "semio-framework-os-kernel-dsl-notation","semio-framework-os-kernel-dsl-schema",
  "semio-framework-os-kernel-pack","semio-framework-os-kernel-pack-core","semio-framework-os-kernel-pack-format",
  "semio-framework-os-kernel-protocol","semio-framework-os-kernel-protocol-core","semio-framework-os-kernel-protocol-format",
  "semio-framework-os-kernel-protocol-command","semio-framework-os-kernel-protocol-causal","semio-framework-os-kernel-protocol-wire",
  "semio-framework-os-kernel-protocol-history","semio-framework-os-kernel-protocol-channel","semio-framework-os-kernel-protocol-crdt",
  "semio-framework-os-kernel-protocol-materialize","semio-framework-os-kernel-protocol-io","semio-framework-os-kernel-protocol-testkit",
  "semio-framework-os-kernel-store","semio-framework-os-kernel-store-sync","semio-framework-os-kernel-vcs",
  "semio-framework-os-kernel-semio","semio-framework-os-kernel-spr",
]);
let changed = 0;
for (const tomlPath of walkToml(REPO)) {
  const rel = path.relative(REPO, tomlPath).replaceAll("\\", "/");
  if (rel === "Cargo.toml") continue;
  if (rel.includes("/⚡️implementations/")) continue;
  let body = fs.readFileSync(tomlPath, "utf8");
  if (!body.includes("🛍️products/💻️os/") || !body.includes("⚡️implementations")) continue;
  const before = body;
  const dir = path.dirname(tomlPath);
  body = body.replace(
    /path\s*=\s*"[^"]*🛍️products\/💻️os\/⚡️implementations\/🦀️rust"/g,
    `path = "${relTo(dir, HOST_PKG)}"`,
  );
  body = body.replace(
    /path\s*=\s*"[^"]*🛍️products\/💻️os\/🔨️modules\/🔌️plugin\/⚡️implementations\/🦀️rust"/g,
    `path = "${relTo(dir, PLUGIN_PKG)}"`,
  );
  body = body.replace(
    /path\s*=\s*"[^"]*🛍️products\/💻️os\/🔨️modules\/🛢️db\/⚡️implementations\/🦀️rust"/g,
    `path = "${relTo(dir, DB_PKG)}"`,
  );
  body = body.replace(
    /path\s*=\s*"[^"]*🛍️products\/💻️os\/[^"]*⚡️implementations[^"]*"/g,
    `path = "${relTo(dir, KERNEL_PKG)}"`,
  );
  for (const pkg of kernelPkgs) {
    body = body.replaceAll(`package = "${pkg}"`, 'package = "semio-framework-os-kernel"');
  }
  if (body !== before) {
    fs.writeFileSync(tomlPath, body);
    changed++;
    note("REPOINT " + rel);
  }
}
note("repointed=" + changed);

// ---- DELETE implementations under os ----
note("== delete implementations ==");
const implDirs = [];
function walkImpl(dir) {
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, ent.name);
    if (!ent.isDirectory()) continue;
    if (ent.name === "⚡️implementations") implDirs.push(p);
    else if (ent.name !== "target" && ent.name !== "node_modules") walkImpl(p);
  }
}
walkImpl(OS);
implDirs.sort((a,b) => b.length - a.length);
let deleted = 0;
for (const d of implDirs) {
  fs.rmSync(d, { recursive: true, force: true });
  deleted++;
  note("DELETED " + path.relative(REPO, d));
  let parent = path.dirname(d);
  while (parent.startsWith(OS) && parent !== OS) {
    const ents = fs.readdirSync(parent);
    if (ents.length === 0) {
      fs.rmSync(parent, { recursive: true, force: true });
      note("DELETED_EMPTY " + path.relative(REPO, parent));
      parent = path.dirname(parent);
    } else break;
  }
}
note("deletedImplDirs=" + deleted);

// count remaining
let remaining = 0;
function countRem(dir) {
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, ent.name);
    if (!ent.isDirectory()) continue;
    if (ent.name === "⚡️implementations") remaining++;
    else if (ent.name !== "target") countRem(p);
  }
}
countRem(OS);
fs.writeFileSync(path.join(TICKET, "🧪eradicate-rest-log.txt"), log.join("\n")+"\n");
fs.writeFileSync(path.join(TICKET, "🧪impl-dirs-remaining.txt"), `remaining=${remaining}\n`);
console.log("DONE remaining="+remaining+" deleted="+deleted);
