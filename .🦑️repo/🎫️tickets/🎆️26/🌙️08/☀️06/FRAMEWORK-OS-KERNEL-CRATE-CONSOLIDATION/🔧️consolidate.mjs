#!/usr/bin/env bun
/** 🔧️ W8c OS-kernel Shape V2 consolidation (store/spr/dsl/pack/infinite/flow). */
import {
  cpSync, existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync, statSync,
} from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const OS = readFileSync("/tmp/os-path.txt", "utf8").trim();
const TICKET = readFileSync("/tmp/os-ticket-path.txt", "utf8").trim();
const PKG = join(OS, "📦️packages/🦀️rust");
const survivors = JSON.parse(
  readFileSync(readdirSync(TICKET).map((f) => join(TICKET, f)).find((p) => p.endsWith("survivors.json")), "utf8"),
);

function ensureDir(p) { mkdirSync(p, { recursive: true }); }
function write(p, body) { ensureDir(dirname(p)); writeFileSync(p, body); }

function buildRewriteMap() {
  const map = new Map();
  for (const s of survivors) map.set(s.lib, "crate::" + s.mod);
  return [...map.entries()].sort((a, b) => b[0].length - a[0].length);
}

function rewriteRust(src, rewrite) {
  let out = src;
  for (const [oldLib, newPath] of rewrite) {
    const reIdent = oldLib.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    out = out.replace(new RegExp("\\b" + reIdent + "::", "g"), newPath + "::");
    out = out.replace(new RegExp("\\buse\\s+" + reIdent + "\\s*;", "g"), "use " + newPath + ";");
    out = out.replace(new RegExp("\\buse\\s+" + reIdent + "\\s+as\\s+", "g"), "use " + newPath + " as ");
  }
  return out;
}
function emitLibRs() {
  const lines = [];
  lines.push("//! 💻️ Semio framework OS kernel — wasm-safe document model (store, spr, dsl, pack, infinite, flow).");
  lines.push("//!");
  lines.push("//! Shape V2 packaging entry: domain logic lives in owner-tree `🦀️component.rs` files.");
  lines.push("");
  const root = new Map();
  function insert(mod, cfg, dest) {
    const parts = mod.split("::");
    let node = root;
    for (let i = 0; i < parts.length; i++) {
      const p = parts[i];
      if (!node.has(p)) node.set(p, { name: p, cfg: null, dest: null, children: new Map() });
      const n = node.get(p);
      if (i === parts.length - 1) { n.dest = dest; n.cfg = cfg; }
      node = n.children;
    }
  }
  for (const s of survivors) insert(s.mod, s.cfg, s.dest);
  function emit(nodeMap, indent) {
    for (const [, n] of nodeMap) {
      const pad = "  ".repeat(indent);
      if (n.children.size === 0) {
        const rel = relative(PKG, join(OS, n.dest)).replaceAll("\\", "/");
        if (n.cfg) lines.push(pad + "#[cfg(" + n.cfg + ")]");
        lines.push(pad + "#[path = \"" + rel + "\"]");
        lines.push(pad + "pub mod " + n.name + ";");
        lines.push("");
      } else if (n.dest) {
        if (n.cfg) lines.push(pad + "#[cfg(" + n.cfg + ")]");
        lines.push(pad + "#[path = \".\"]");
        lines.push(pad + "pub mod " + n.name + " {");
        const rel = relative(PKG, join(OS, n.dest)).replaceAll("\\", "/");
        lines.push(pad + "  #[path = \"" + rel + "\"]");
        lines.push(pad + "  mod component;");
        lines.push(pad + "  pub use component::*;");
        lines.push("");
        emit(n.children, indent + 1);
        lines.push(pad + "}");
        lines.push("");
      } else {
        if (n.cfg) lines.push(pad + "#[cfg(" + n.cfg + ")]");
        lines.push(pad + "#[path = \".\"]");
        lines.push(pad + "pub mod " + n.name + " {");
        emit(n.children, indent + 1);
        lines.push(pad + "}");
        lines.push("");
      }
    }
  }
  emit(root, 0);
  return lines.join("\n");
}

function resolveDepsProperly() {
  const survivorPkgs = new Set(survivors.map((s) => s.pkg));
  survivorPkgs.add("semio-framework-os-kernel-dsl-derive");
  const deps = new Map();
  const targetDeps = new Map();
  for (const s of survivors) {
    const cargoPath = join(OS, s.oldDir, "Cargo.toml");
    const cargo = readFileSync(cargoPath, "utf8");
    const oldDir = join(OS, s.oldDir);
    const sectionRe = /^\[(target\.'cfg\(([^)]+)\)'\.)?dependencies\]/gm;
    let match;
    const starts = [];
    while ((match = sectionRe.exec(cargo))) {
      starts.push({ index: match.index, cfg: match[2] || null });
    }
    for (let i = 0; i < starts.length; i++) {
      const start = starts[i].index;
      const end = i + 1 < starts.length ? starts[i + 1].index : cargo.length;
      let body = cargo.slice(start, end);
      const nl = body.indexOf("\n");
      body = body.slice(nl + 1);
      const next = body.search(/^\[/m);
      if (next >= 0) body = body.slice(0, next);
      for (const raw of body.split("\n")) {
        const line = raw.trimEnd();
        if (!line.trim() || line.trim().startsWith("#")) continue;
        const alias = line.match(/^([A-Za-z0-9_-]+)\s*=/)?.[1];
        if (!alias) continue;
        const pkg = line.match(/package\s*=\s*"([^"]+)"/)?.[1];
        if (pkg && survivorPkgs.has(pkg)) continue;
        let newLine = line;
        const pathMatch = line.match(/path\s*=\s*"([^"]+)"/);
        if (pathMatch) {
          const absPath = resolve(oldDir, pathMatch[1]);
          const relFromPkg = relative(PKG, absPath).replaceAll("\\", "/");
          newLine = line.replace(pathMatch[1], relFromPkg);
        }
        const cfg = starts[i].cfg;
        if (cfg) {
          if (!targetDeps.has(cfg)) targetDeps.set(cfg, new Map());
          if (!targetDeps.get(cfg).has(alias)) targetDeps.get(cfg).set(alias, newLine);
        } else {
          if (!deps.has(alias)) deps.set(alias, newLine);
        }
      }
    }
  }
  return { deps, targetDeps };
}

function emitCargoToml(ext) {
  const lines = [];
  lines.push("[package]");
  lines.push("name = \"semio-framework-os-kernel\"");
  lines.push("version = \"0.1.0\"");
  lines.push("edition = \"2021\"");
  lines.push("rust-version = \"1.88\"");
  lines.push("description = \"Semio framework OS kernel — wasm-safe store/spr/dsl/pack/infinite/flow document model\"");
  lines.push("");
  lines.push("[package.metadata.semio]");
  lines.push("role = \"framework\"");
  lines.push("id = \"os-kernel\"");
  lines.push("");
  lines.push("[lints]");
  lines.push("workspace = true");
  lines.push("");
  lines.push("[lib]");
  lines.push("name = \"semio_framework_os_kernel\"");
  lines.push("crate-type = [\"rlib\", \"cdylib\"]");
  lines.push("path = \"📦️lib.rs\"");
  lines.push("");
  lines.push("[features]");
  lines.push("default = [\"deflate\"]");
  lines.push("deflate = []");
  lines.push("ureq = [\"dep:ureq\"]");
  lines.push("typegen = []");
  lines.push("");
  lines.push("[dependencies]");
  lines.push("dsl_derive = { path = \"../../🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust\", package = \"semio-framework-os-kernel-dsl-derive\" }");
  for (const [, line] of [...ext.deps.entries()].sort((a, b) => a[0].localeCompare(b[0]))) {
    lines.push(line);
  }
  lines.push("");
  for (const [cfg, map] of [...ext.targetDeps.entries()].sort((a, b) => a[0].localeCompare(b[0]))) {
    lines.push("[target.'cfg(" + cfg + ")'.dependencies]");
    for (const [, line] of [...map.entries()].sort((a, b) => a[0].localeCompare(b[0]))) lines.push(line);
    lines.push("");
  }
  lines.push("[[bin]]");
  lines.push("name = \"pack\"");
  lines.push("path = \"../../🔨️modules/🎒️pack/⌨️cli/📦️main.rs\"");
  lines.push("");
  lines.push("[[bin]]");
  lines.push("name = \"spr\"");
  lines.push("path = \"../../🔨️modules/📡️spr/⌨️cli/📦️main.rs\"");
  lines.push("");
  return lines.join("\n");
}

function copyComponents(rewrite) {
  const log = [];
  for (const s of survivors) {
    const srcLib = s.libPath || join(OS, s.oldDir, "📦️lib.rs");
    if (!existsSync(srcLib)) throw new Error("missing lib " + srcLib);
    let body = rewriteRust(readFileSync(srcLib, "utf8"), rewrite);
    const dest = join(OS, s.dest);
    write(dest, body);
    log.push("COPY " + s.lib + " -> " + s.dest);
    const oldDir = join(OS, s.oldDir);
    for (const f of readdirSync(oldDir)) {
      if (!f.endsWith(".rs")) continue;
      if (f.endsWith("lib.rs") || f.endsWith("main.rs") || f === "build.rs") continue;
      write(join(dirname(dest), f), rewriteRust(readFileSync(join(oldDir, f), "utf8"), rewrite));
      log.push("  sibling " + f);
    }
    if (existsSync(join(oldDir, "📦️main.rs"))) {
      write(join(dirname(dest), "📦️main.rs"), rewriteRust(readFileSync(join(oldDir, "📦️main.rs"), "utf8"), rewrite));
      log.push("  main.rs");
    }
    if (existsSync(join(oldDir, "build.rs"))) {
      cpSync(join(oldDir, "build.rs"), join(dirname(dest), "build.rs"));
      log.push("  build.rs");
    }
    const assets = readdirSync(oldDir).find((f) => f.endsWith("assets") || f.includes("assets"));
    if (assets && statSync(join(oldDir, assets)).isDirectory()) {
      cpSync(join(oldDir, assets), join(dirname(dest), assets), { recursive: true });
      log.push("  assets " + assets);
    }
    if (existsSync(join(oldDir, "benches"))) {
      cpSync(join(oldDir, "benches"), join(dirname(dest), "benches"), { recursive: true });
      log.push("  benches");
    }
  }
  return log;
}

function relocateDerive() {
  const old = join(OS, "🔨️modules/🗣️dsl/✨️derive/⚡️implementations/🦀️rust");
  const neu = join(OS, "🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust");
  const lib = readFileSync(join(old, "📦️lib.rs"), "utf8");
  write(join(OS, "🔨️modules/🗣️dsl/✨️derive/🦀️component.rs"), lib);
  write(join(neu, "📦️lib.rs"), "#[path = \"../../../🦀️component.rs\"]\npub mod component;\npub use component::*;\n");
  write(join(neu, "Cargo.toml"), `[package]
name = "semio-framework-os-kernel-dsl-derive"
version = "0.1.0"
edition = "2021"
rust-version = "1.88"
description = "Proc-macros compiling #[dsl(...)]-annotated struct/enum declarations into DocumentDsl/OpText impls"

[lints]
workspace = true

[lib]
name = "dsl_derive"
proc-macro = true
path = "📦️lib.rs"

[dependencies]
syn = { version = "2.0", features = ["full", "extra-traits"] }
quote = "1.0"
proc-macro2 = "1.0"
`);
  if (existsSync(join(old, "📋️project.json"))) cpSync(join(old, "📋️project.json"), join(neu, "📋️project.json"));
  if (existsSync(join(old, "📜️script.ts"))) cpSync(join(old, "📜️script.ts"), join(neu, "📜️script.ts"));
}

function relocatePlugin() {
  const old = join(OS, "🔨️modules/🔌️plugin/⚡️implementations/🦀️rust");
  const neu = join(OS, "🔨️modules/🔌️plugin/📦️packages/🦀️rust");
  const lib = readFileSync(join(old, "📦️lib.rs"), "utf8");
  write(join(OS, "🔨️modules/🔌️plugin/🦀️component.rs"), lib);
  write(join(neu, "📦️lib.rs"), "#[path = \"../../🦀️component.rs\"]\npub mod component;\npub use component::*;\n");
  const wit = readdirSync(old).find((f) => f.includes("wit"));
  if (wit) cpSync(join(old, wit), join(neu, wit), { recursive: true });
  const oldCargo = readFileSync(join(old, "Cargo.toml"), "utf8");
  let cargo = oldCargo.replace(/\[lib\][\s\S]*?(?=\n\[)/m, "[lib]\npath = \"📦️lib.rs\"\n\n");
  if (!/\[lib\]/.test(cargo)) {
    cargo = cargo.replace(/^(description = "[^"]*"\n)/m, "$1\n[lib]\npath = \"📦️lib.rs\"\n");
  }
  write(join(neu, "Cargo.toml"), cargo);
}

// main
const rewrite = buildRewriteMap();
write(join(TICKET, "🧪rewrite-map.json"), JSON.stringify(Object.fromEntries(rewrite), null, 2));
console.log("rewrite entries", rewrite.length);
ensureDir(PKG);
const log = copyComponents(rewrite);
relocateDerive();
relocatePlugin();
const ext = resolveDepsProperly();
write(join(PKG, "📦️lib.rs"), emitLibRs());
write(join(PKG, "Cargo.toml"), emitCargoToml(ext));
write(join(PKG, "📜️script.ts"), `import { createScript } from "../../../../../../📜️script.ts";

export default createScript(import.meta, {
  check: async ({ cargo }) => {
    await cargo(["check", "--manifest-path", "Cargo.toml"]);
  },
  test: async ({ cargo }) => {
    await cargo(["test", "--manifest-path", "Cargo.toml", "--lib"]);
  },
});
`);
write(join(PKG, "📋️project.json"), JSON.stringify({
  name: "@semio-tech/framework-os-kernel",
  root: "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
  sourceRoot: "🧰️framework/🛍️products/💻️os",
  projectType: "library",
  tags: ["lang:rust", "role:framework", "family:os-kernel"],
  targets: {
    check: { executor: "nx:run-commands", options: { command: "bun 📜️script.ts check", cwd: "{projectRoot}" } },
    test: { executor: "nx:run-commands", options: { command: "bun 📜️script.ts test", cwd: "{projectRoot}" } },
  },
}, null, 2) + "\n");
write(join(TICKET, "🧪consolidate-log.txt"), log.join("\n") + "\n");
console.log("copied", log.length, "items");
console.log("PKG", PKG);
console.log("deps", ext.deps.size, "targetDepCfgs", ext.targetDeps.size);
