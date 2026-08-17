import fs from "node:fs";
import path from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const TICKET = path.join(
  REPO,
  ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️07/DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT",
);

const log = [];
function note(msg) {
  log.push(msg);
  console.log(msg);
}

function walkFiles(dir, ext, out = []) {
  if (!fs.existsSync(dir)) return out;
  for (const name of fs.readdirSync(dir)) {
    const p = path.join(dir, name);
    const st = fs.statSync(p);
    if (st.isDirectory()) walkFiles(p, ext, out);
    else if (p.endsWith(ext)) out.push(p);
  }
  return out;
}

function replaceInFiles(files, pairs) {
  for (const file of files) {
    let text = fs.readFileSync(file, "utf8");
    let next = text;
    for (const [from, to] of pairs) next = next.split(from).join(to);
    if (next !== text) fs.writeFileSync(file, next);
  }
}

function mv(src, dest) {
  if (!fs.existsSync(src)) {
    note(`SKIP missing: ${src}`);
    return;
  }
  fs.mkdirSync(path.dirname(dest), { recursive: true });
  if (fs.existsSync(dest)) fs.rmSync(dest, { recursive: true, force: true });
  fs.renameSync(src, dest);
  note(`mv ${path.relative(REPO, src)} -> ${path.relative(REPO, dest)}`);
}

function rmDir(dir) {
  if (fs.existsSync(dir)) {
    fs.rmSync(dir, { recursive: true, force: true });
    note(`rm ${path.relative(REPO, dir)}`);
  }
}

function read(p) {
  return fs.readFileSync(p, "utf8");
}

function write(p, content) {
  fs.mkdirSync(path.dirname(p), { recursive: true });
  fs.writeFileSync(p, content);
}

function extractRegions(content, regionNames) {
  const out = {};
  for (const name of regionNames) {
    const startRe = new RegExp(`// #region 🔖️${name}`);
    const endRe = new RegExp(`// #endregion 🔖️${name}`);
    const start = content.search(startRe);
    const end = content.search(endRe);
    if (start === -1 || end === -1) throw new Error(`region ${name} not found`);
    out[name] = content.slice(start, end + content.match(endRe)[0].length);
  }
  return out;
}

function sliceBetween(content, startMarker, endMarker) {
  const s = content.indexOf(startMarker);
  const e = content.indexOf(endMarker);
  if (s === -1 || e === -1) throw new Error(`slice failed ${startMarker}`);
  return content.slice(s, e);
}

// —— A) FEM ——
const fem = path.join(REPO, "✏️s/🔌️plugins/🏗️fem");
const femCore = path.join(fem, "🫀️core");

note("=== FEM ===");
mv(path.join(femCore, "➗️formulation"), path.join(fem, "➗️formulation"));
mv(path.join(femCore, "🕸️mesh"), path.join(fem, "🕸️mesh"));
mv(path.join(femCore, "🔢️sparse"), path.join(fem, "🔢️sparse"));
mv(path.join(femCore, "📏️elements2d"), path.join(fem, "📏️elements2d"));
mv(path.join(femCore, "🧊️elements3d"), path.join(fem, "🧊️elements3d"));
mv(path.join(femCore, "🧮️analyses"), path.join(fem, "🧮️analyses"));
mv(path.join(femCore, "🤝️shared"), path.join(fem, "🖥️app-surface"));
fs.mkdirSync(path.join(fem, "🏗️model"), { recursive: true });
mv(path.join(femCore, "🦀️component.rs"), path.join(fem, "🏗️model/🦀️component.rs"));
rmDir(femCore);

const femGlue = path.join(fem, "📦️packages/🦀️rust/📦️glue.rs");
let femGlueText = read(femGlue);
const femCoreStart = femGlueText.indexOf("//#region 🫀️Core");
const femCoreEnd = femGlueText.indexOf("//#endregion 🫀️Core") + "//#endregion 🫀️Core".length;
if (femCoreStart === -1) throw new Error("fem core block not found");

const femNewBlock = `//#region 🏗️Kernel modules
#[path = "../../🏗️model/🦀️component.rs"]
pub mod model;
#[path = "../../🧮️analyses/🦀️component.rs"]
pub mod analyses;
#[path = "../../📏️elements2d/🦀️component.rs"]
pub mod elements2d;
#[path = "../../🧊️elements3d/🦀️component.rs"]
pub mod elements3d;
#[path = "../../➗️formulation/🦀️component.rs"]
pub mod formulation;
#[path = "../../🕸️mesh/🦀️component.rs"]
pub mod mesh;
#[path = "../../🔢️sparse/🦀️component.rs"]
pub mod sparse;
#[path = "../../🖥️app-surface/🦀️component.rs"]
pub mod app_surface;

/// 🗂️ Registers both artifacts' engines with the host.
pub fn register_all_engines() {
    crate::artifacts::fem2d::engine::register();
    crate::artifacts::fem3d::engine::register();
}
//#endregion 🏗️Kernel modules`;

femGlueText = femGlueText.slice(0, femCoreStart) + femNewBlock + femGlueText.slice(femCoreEnd);
femGlueText = femGlueText.replace("setup: core::register_all_engines,", "setup: register_all_engines,");
write(femGlue, femGlueText);

const femFiles = walkFiles(fem, ".rs");
replaceInFiles(femFiles, [
  ["crate::core::shared::", "crate::app_surface::"],
  ["crate::core::elements2d::", "crate::elements2d::"],
  ["crate::core::elements3d::", "crate::elements3d::"],
  ["crate::core::formulation::", "crate::formulation::"],
  ["crate::core::mesh::", "crate::mesh::"],
  ["crate::core::sparse::", "crate::sparse::"],
  ["crate::core::analyses::", "crate::analyses::"],
  ["pub use crate::core::elements2d", "pub use crate::elements2d"],
  ["pub use crate::core::elements3d", "pub use crate::elements3d"],
  ["crate::core::{", "crate::model::{"],
  ["crate::core::", "crate::model::"],
]);

// —— B) NORM ——
const norm = path.join(REPO, "✏️s/🔌️plugins/📕️norm");
const normCore = path.join(norm, "🫀️core");
note("=== NORM ===");
mv(path.join(normCore, "🎚️config"), path.join(norm, "🎚️config"));
mv(path.join(normCore, "🖥️app-surface"), path.join(norm, "🖥️app-surface"));
fs.mkdirSync(path.join(norm, "📄️document"), { recursive: true });
mv(path.join(normCore, "🦀️component.rs"), path.join(norm, "📄️document/🦀️component.rs"));
rmDir(normCore);

const normGlue = path.join(norm, "📦️packages/🦀️rust/📦️glue.rs");
let normGlueText = read(normGlue);
normGlueText = normGlueText.replace(
  `//#region 🫀️Core
/// 🤝️ The cross-artifact, cross-app kernel: the norm domain model plus everything all fifteen apps
/// share verbatim. Depends on no artifact and on no app.
#[path = "."]
pub mod core {
    #[path = "../../🫀️core/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🫀️core/🎚️config/🦀️component.rs"]
    mod config;
    pub use config::*;

    #[path = "../../🫀️core/🖥️app-surface/🦀️component.rs"]
    pub mod app;
}
//#endregion 🫀️Core`,
  `//#region 📄️Document kernel
#[path = "../../📄️document/🦀️component.rs"]
pub mod document;
#[path = "../../🎚️config/🦀️component.rs"]
pub mod config;
#[path = "../../🖥️app-surface/🦀️component.rs"]
pub mod app_surface;
//#endregion 📄️Document kernel`,
);
write(normGlue, normGlueText);

const normFiles = walkFiles(norm, ".rs");
replaceInFiles(normFiles, [
  ["crate::core::app::", "crate::app_surface::"],
  ["crate::core::{NormConfig", "crate::config::{NormConfig"],
  ["use crate::core::{NormConfig", "use crate::config::{NormConfig"],
  ["crate::core::NormConfigOperation", "crate::config::NormConfigOperation"],
  ["ConfigView<'_, crate::core::NormConfig>", "ConfigView<'_, crate::config::NormConfig>"],
  ["crate::core::SetDocumentOperation", "crate::document::SetDocumentOperation"],
  ["use crate::core::SetDocumentOperation", "use crate::document::SetDocumentOperation"],
  ["use crate::core::NormHost", "use crate::document::NormHost"],
  ["crate::core::NormHost", "crate::document::NormHost"],
  ["crate::core::", "crate::document::"],
]);

// —— D) BLOCK ——
const block = path.join(REPO, "✏️s/🔌️plugins/🧱️block");
note("=== BLOCK ===");
const blockCoreContent = read(path.join(block, "🫀️core/🦀️component.rs"));
write(path.join(block, "🦀️component.rs"), blockCoreContent);
rmDir(path.join(block, "🫀️core"));

const blockGlue = path.join(block, "📦️packages/🦀️rust/📦️glue.rs");
let blockGlueText = read(blockGlue);
blockGlueText = blockGlueText.replace(
  `//#region 🫀️Core
/// 🤝️ Record types shared by all three artifacts' document entities (non-constitutional cross-artifact
/// kernel — see the constitutional-split recipe's "shared code used by ≥2 artifacts" rule).
#[path = "."]
pub mod core {
    #[path = "../../🫀️core/🦀️component.rs"]
    mod component;
    pub use component::*;
}
//#endregion 🫀️Core`,
  `#[path = "../../🦀️component.rs"]
mod block_shared;
pub use block_shared::*;`,
);
write(blockGlue, blockGlueText);
replaceInFiles(walkFiles(block, ".rs"), [["crate::core::", "crate::"]]);

// —— E) SPACE ——
const space = path.join(REPO, "✏️s/🔌️plugins/🪐️space");
note("=== SPACE ===");
let spaceCoreContent = read(path.join(space, "🫀️core/🦀️component.rs"));
spaceCoreContent = spaceCoreContent.replace(
  "include_str!(\"../../../../🧰️framework/",
  "include_str!(\"../../../🧰️framework/",
);
spaceCoreContent = spaceCoreContent.replace("include_str!(\"../../🖍️draw/", "include_str!(\"../🖍️draw/");
spaceCoreContent = spaceCoreContent.replace("include_str!(\"../../✒️writer/", "include_str!(\"../✒️writer/");
write(path.join(space, "🦀️component.rs"), spaceCoreContent);
rmDir(path.join(space, "🫀️core"));

const spaceGlue = path.join(space, "📦️packages/🦀️rust/📦️glue.rs");
let spaceGlueText = read(spaceGlue);
spaceGlueText = spaceGlueText.replace(
  `//#region 🫀️Core
#[path = "."]
pub mod core {
    #[path = "../../🫀️core/🦀️component.rs"]
    mod component;
    pub use component::*;
}
//#endregion 🫀️Core`,
  `#[path = "../../🦀️component.rs"]
mod space_shared;
pub use space_shared::*;`,
);
write(spaceGlue, spaceGlueText);
replaceInFiles(walkFiles(space, ".rs"), [["crate::core::", "crate::"]]);

// —— F) FLOW EXT ——
const flowExtOld = path.join(REPO, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🫀️core");
const flowExtNew = path.join(REPO, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive");
note("=== FLOW EXT ===");
mv(flowExtOld, flowExtNew);

const flowExtCargo = path.join(flowExtNew, "📦️packages/🦀️rust/Cargo.toml");
let flowCargo = read(flowExtCargo);
flowCargo = flowCargo.replace("semio-s-plugin-flow-extension-core", "semio-s-plugin-flow-extension-primitive");
flowCargo = flowCargo.replace("semio:flow-extension-core", "semio:flow-extension-primitive");
write(flowExtCargo, flowCargo);

const flowExtScript = path.join(flowExtNew, "📦️packages/🦀️rust/📜️script.ts");
if (fs.existsSync(flowExtScript)) {
  replaceInFiles([flowExtScript], [["semio-s-plugin-flow-extension-core", "semio-s-plugin-flow-extension-primitive"]]);
}

write(path.join(TICKET, "deferred-flow-ext.json"), JSON.stringify({
  workspaceMembers: [
    {
      oldPath: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🫀️core/📦️packages/🦀️rust",
      newPath: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust",
      oldPackageName: "semio-s-plugin-flow-extension-core",
      newPackageName: "semio-s-plugin-flow-extension-primitive",
    },
  ],
  pathReferences: [
    "Cargo.toml",
    "✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml",
  ],
}, null, 2));

// —— G) IMPERATIVE EXT ——
const impExtOld = path.join(REPO, "✏️s/🔌️plugins/📜️imperative/🧩️extensions/🫀️core");
const impExtNew = path.join(REPO, "✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect");
note("=== IMPERATIVE EXT ===");
mv(impExtOld, impExtNew);

const impCargo = path.join(impExtNew, "📦️packages/🦀️rust/Cargo.toml");
let impCargoText = read(impCargo);
impCargoText = impCargoText.replace("semio-s-plugin-imperative-core", "semio-s-plugin-imperative-effect");
impCargoText = impCargoText.replace("semio:imperative-extension-core", "semio:imperative-extension-effect");
write(impCargo, impCargoText);

const impScript = path.join(impExtNew, "📦️packages/🦀️rust/📜️script.ts");
if (fs.existsSync(impScript)) {
  replaceInFiles([impScript], [["semio-s-plugin-imperative-core", "semio-s-plugin-imperative-effect"]]);
}

write(path.join(TICKET, "deferred-imperative-ext.json"), JSON.stringify({
  workspaceMembers: [
    {
      oldPath: "✏️s/🔌️plugins/📜️imperative/🧩️extensions/🫀️core/📦️packages/🦀️rust",
      newPath: "✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust",
      oldPackageName: "semio-s-plugin-imperative-core",
      newPackageName: "semio-s-plugin-imperative-effect",
    },
  ],
  externCrateRenames: [
    { old: "imperative_module_core", new: "imperative_module_effect" },
  ],
}, null, 2));

// —— C) TRINITY ——
const trinity = path.join(REPO, "✏️s/🔌️plugins/🔱️trinity");
const trinityCoreFile = path.join(trinity, "🫀️core/🦀️component.rs");
note("=== TRINITY ===");
const trinityContent = read(trinityCoreFile);

const queryableBlock = sliceBetween(trinityContent, "pub mod queryable {", "// #region 🔖️Ast");
const preamble = trinityContent.slice(0, trinityContent.indexOf("pub mod queryable {"));
const regions = extractRegions(trinityContent, [
  "Ast",
  "Lexer",
  "Language",
  "LanguageService",
  "ExampleFixture",
  "Parser",
  "SpannedAst",
  "Executor",
  "Tests",
]);

const astHeader = `//! 🌳️ Trinity jack query AST.\n\nuse serde::{Deserialize, Serialize};\nuse crate::artifacts::jack::{GraphFixture, PropertyValue};\n\n`;
write(path.join(trinity, "🌳️ast/🦀️component.rs"), astHeader + regions.Ast.replace("// #region 🔖️Ast\n", ""));

const lexerHeader = `//! 🔤️ Trinity jack lexer.\n\nuse serde::{Deserialize, Serialize};\n\n`;
write(path.join(trinity, "🔤️lexer/🦀️component.rs"), lexerHeader + regions.Lexer.replace("// #region 🔖️Lexer\n", ""));

const lsBody =
  regions.Language +
  "\n" +
  regions.LanguageService +
  "\n" +
  regions.ExampleFixture +
  "\n" +
  regions.Parser +
  "\n" +
  regions.SpannedAst;
const lsHeader = `//! 🗣️ Trinity jack language service — parse, complete, lint, hover.\n#![allow(dead_code)]\n\nuse crate::artifacts::jack::{Camera, Edge, Graph, GraphFixture, Manifest, Node, Port, PortDirection, PropertyBag, PropertyValue, port_key};
use crate::executor::{OwnedTrinityQueryableGraph, TrinityQueryableGraph};
use crate::lexer::{lex_spanned, tokenize, Token, TokenClass, TokenSpan, SpannedToken};
use crate::ast::{Assignment, Clause, Expr, Pattern, PatternEdge, PatternNode, Query, QueryResult, QueryResultKind, ReturnItem};
use math::graph::dsl::{Completion, Diagnostic, DiagnosticSeverity, Hover, SemanticToken};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

`;
write(path.join(trinity, "🗣️language-service/🦀️component.rs"), lsHeader + lsBody.replace(/\n\/\/ #region 🔖️\w+\n/g, "\n"));

const execHeader = `//! 🧮️ Trinity jack query executor.\n\n${preamble.replace("//! 🃏️", "//! 🧮️")}${queryableBlock}
use crate::ast::{Assignment, Binding, Clause, Expr, Pattern, PatternEdge, PatternNode, Query, QueryResult, QueryResultKind, ReturnItem};
use crate::language_service::parse;
use crate::lexer::{lex_spanned, Token, SpannedToken};
`;
write(
  path.join(trinity, "🧮️executor/🦀️component.rs"),
  execHeader + regions.Executor.replace("// #region 🔖️Executor\n", "") + "\n" + regions.Tests,
);

rmDir(path.join(trinity, "🫀️core"));

const trinityGlue = path.join(trinity, "📦️packages/🦀️rust/📦️glue.rs");
let trinityGlueText = read(trinityGlue);
trinityGlueText = trinityGlueText.replace(
  `//#region 🔖️Core
#[path = "."]
pub mod core {
    #[path = "../../🫀️core/🦀️component.rs"]
    mod component;
    pub use component::*;
}
//#endregion 🔖️Core`,
  `//#region 🔤️Jack kernel
#[path = "../../🌳️ast/🦀️component.rs"]
pub mod ast;
#[path = "../../🔤️lexer/🦀️component.rs"]
pub mod lexer;
#[path = "../../🧮️executor/🦀️component.rs"]
pub mod executor;
#[path = "../../🗣️language-service/🦀️component.rs"]
pub mod language_service;
//#endregion 🔤️Jack kernel`,
);
write(trinityGlue, trinityGlueText);

replaceInFiles(walkFiles(trinity, ".rs"), [
  ["crate::core::", "crate::language_service::"],
  ["use crate::language_service::{execute", "use crate::executor::{execute"],
  ["use crate::language_service::{Completion as JackCompletion, TokenSpan as JackTokenSpan}", "use crate::lexer::{TokenSpan as JackTokenSpan}; use math::graph::dsl::Completion as JackCompletion"],
  ["crate::language_service::run(", "crate::executor::run("],
]);

// fix rewrite world imports
const rewriteWorld = path.join(trinity, "🎛️apps/♻️rewrite/🌍️world/🦀️component.rs");
let rw = read(rewriteWorld);
rw = rw.replace(
  "use crate::language_service::{complete as complete_jack, execute, parse, tokenize as tokenize_jack, QueryResult};",
  "use crate::ast::QueryResult;\nuse crate::executor::execute;\nuse crate::language_service::{complete as complete_jack, parse};\nuse crate::lexer::tokenize as tokenize_jack;",
);
write(rewriteWorld, rw);

const rewriteEngine = path.join(trinity, "🗿️artifacts/♻️rewrite/⚙️engine/🦀️component.rs");
let re = read(rewriteEngine);
re = re.replace(
  "use crate::language_service::{execute, parse, Pattern, PatternEdge, PatternNode, QueryResult};",
  "use crate::ast::{Pattern, PatternEdge, PatternNode, QueryResult};\nuse crate::executor::execute;\nuse crate::language_service::parse;",
);
write(rewriteEngine, re);

const jackShell = path.join(trinity, "🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs");
if (fs.existsSync(jackShell)) {
  replaceInFiles([jackShell], [
    ["trinity::core::{run, QueryResult}", "trinity::executor::run; use trinity::ast::QueryResult"],
  ]);
}

write(path.join(TICKET, "wave1-plugin-cores-log.txt"), log.join("\n"));
note("DONE");
