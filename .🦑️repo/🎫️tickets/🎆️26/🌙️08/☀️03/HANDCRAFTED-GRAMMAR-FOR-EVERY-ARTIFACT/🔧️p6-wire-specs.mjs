#!/usr/bin/env bun
import {
  readFileSync,
  writeFileSync,
  existsSync,
  readdirSync,
  statSync,
} from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const repo = "/Users/ueli/Documents/semio";
const ticket = dirname(fileURLToPath(import.meta.url));
const scriptPath = join(repo, "📜️script.ts");

const FACETS = ["🗣️dsl", "🔧️op", "🔺️diff", "🎒️pack", "📡️spr"];
const TEXT_FACETS = new Set(["🗣️dsl", "🔧️op", "🔺️diff"]);
const BIN_FACETS = new Set(["🎒️pack", "📡️spr"]);

const GRAMMAR_BLOCK = `//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (\`dialect grammar\`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

`;

const PROTOCOL_BLOCK = `//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (\`dialect protocol\`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

`;

function extractSet(name) {
  const script = readFileSync(scriptPath, "utf8");
  const re = new RegExp(
    `const ${name}[^=]*=\\s*new Set(?:<[^>]*>)?\\(\\s*\\[([\\s\\S]*?)\\]\\s*\\)`,
  );
  const m = script.match(re);
  if (!m) throw new Error(`set not found: ${name}`);
  return [...m[1].matchAll(/"([^"]+)"/g)].map((x) => x[1]);
}

function readDialectId(specPath, kind) {
  if (!existsSync(specPath)) return null;
  const lines = readFileSync(specPath, "utf8").split("\n");
  const key = kind === "grammar" ? "grammar " : "protocol ";
  for (const line of lines) {
    const t = line.trim();
    if (t.startsWith(key)) return t.slice(key.length).trim();
  }
  return null;
}

function readExtension(dslGrammarPath) {
  if (!existsSync(dslGrammarPath)) return null;
  for (const line of readFileSync(dslGrammarPath, "utf8").split("\n")) {
    const t = line.trim();
    if (t.startsWith("extension ")) return t.slice("extension ".length).trim();
  }
  return null;
}

function rustModForArtifact(artRel) {
  const parts = artRel.split("/");
  const pluginIdx = parts.indexOf("🔌️plugins");
  const plugin = parts[pluginIdx + 1];
  const artFolder = parts[parts.length - 1];
  const gluePath = join(
    repo,
    parts.slice(0, pluginIdx + 2).join("/"),
    "📦️packages/🦀️rust/📦️glue.rs",
  );
  if (!existsSync(gluePath)) {
    throw new Error(`glue missing: ${gluePath}`);
  }
  const glue = readFileSync(gluePath, "utf8");
  const pathNeedle = `🗿️artifacts/${artFolder}/`;
  const modRe = /pub mod (\w+) \{[\s\S]*?#\[path = "\.\.\/\.\.\/🗿️artifacts\//g;
  let m;
  while ((m = modRe.exec(glue)) !== null) {
    const modName = m[1];
    const slice = glue.slice(m.index, m.index + 800);
    if (slice.includes(pathNeedle)) return modName;
  }
  throw new Error(`rust mod not found for ${artRel} (${pathNeedle})`);
}

function injectInclude(rsPath, facet) {
  const abs = join(repo, rsPath);
  if (!existsSync(abs)) return { rsPath, status: "missing-rs" };
  let t = readFileSync(abs, "utf8");
  const facetDir = dirname(abs);
  const grammarPath = join(facetDir, "📖️component.grammar.semio");
  const protocolPath = join(facetDir, "📡️component.protocol.semio");

  if (TEXT_FACETS.has(facet)) {
    if (!existsSync(grammarPath)) return { rsPath, status: "no-grammar-spec" };
    if (t.includes("COMPONENT_GRAMMAR_SEMIO")) return { rsPath, status: "already" };
    if (!t.includes("include_str!") || !t.includes("component.grammar.semio")) {
      const insertAfter = t.match(/^([\s\S]*?)(\n(?:use |pub use ))/);
      if (insertAfter) {
        t = insertAfter[1] + "\n\n" + GRAMMAR_BLOCK + insertAfter[2] + t.slice(insertAfter[0].length);
      } else {
        const nl = t.indexOf("\n");
        t = t.slice(0, nl + 1) + "\n" + GRAMMAR_BLOCK + t.slice(nl + 1);
      }
      writeFileSync(abs, t);
      return { rsPath, status: "wired-grammar" };
    }
  }
  if (BIN_FACETS.has(facet)) {
    if (!existsSync(protocolPath)) return { rsPath, status: "no-protocol-spec" };
    if (t.includes("COMPONENT_PROTOCOL_SEMIO")) return { rsPath, status: "already" };
    const insertAfter = t.match(/^([\s\S]*?)(\n(?:use |pub use ))/);
    if (insertAfter) {
      t = insertAfter[1] + "\n\n" + PROTOCOL_BLOCK + insertAfter[2] + t.slice(insertAfter[0].length);
    } else {
      const nl = t.indexOf("\n");
      t = t.slice(0, nl + 1) + "\n" + PROTOCOL_BLOCK + t.slice(nl + 1);
    }
    writeFileSync(abs, t);
    return { rsPath, status: "wired-protocol" };
  }
  return { rsPath, status: "skip" };
}

function registerBlock(mod, docId, opId, diffId, packId, sprId, extension) {
  const extLine = extension ? `        extension: Some("${extension}"),` : `        extension: None,`;
  return `
/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "${docId}",
${extLine}
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::${mod}::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::${mod}::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::${mod}::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::${mod}::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("${docId}"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${opId}",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::${mod}::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::${mod}::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::${mod}::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::${mod}::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("${opId}"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${diffId}",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::${mod}::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::${mod}::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("${diffId}"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${packId}",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::${mod}::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::${mod}::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("${packId}"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "${sprId}",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::${mod}::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::${mod}::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("${sprId}"),
    });
}
`;
}

function patchEngine(artRel) {
  const engineRel = `${artRel}/⚙️engine/🦀️component.rs`;
  const abs = join(repo, engineRel);
  if (!existsSync(abs)) return { artRel, status: "no-engine" };
  let t = readFileSync(abs, "utf8");
  const mod = rustModForArtifact(artRel);
  const base = join(repo, artRel);
  const docId = readDialectId(join(base, "🗣️dsl/📖️component.grammar.semio"), "grammar");
  const opId = readDialectId(join(base, "🔧️op/📖️component.grammar.semio"), "grammar");
  const diffId = readDialectId(join(base, "🔺️diff/📖️component.grammar.semio"), "grammar");
  const packId = readDialectId(join(base, "🎒️pack/📡️component.protocol.semio"), "protocol");
  const sprId = readDialectId(join(base, "📡️spr/📡️component.protocol.semio"), "protocol");
  const extension = readExtension(join(base, "🗣️dsl/📖️component.grammar.semio"));
  if (!docId || !opId || !diffId || !packId || !sprId) {
    return { artRel, status: "missing-spec-ids", docId, opId, diffId, packId, sprId };
  }

  if (!t.includes("fn register_pilot_languages")) {
    const block = registerBlock(mod, docId, opId, diffId, packId, sprId, extension);
    if (t.includes("//#endregion 🔖️Register")) {
      t = t.replace("//#endregion 🔖️Register", block + "\n//#endregion 🔖️Register");
    } else if (t.includes("// #endregion 🔖️Register")) {
      t = t.replace("// #endregion 🔖️Register", block + "\n// #endregion 🔖️Register");
    } else {
      t += "\n" + block;
    }
  }

  if (t.includes("pub fn register()") && !t.includes("register_pilot_languages();")) {
    t = t.replace(/pub fn register\(\) \{/, "pub fn register() {\n    register_pilot_languages();");
  }

  writeFileSync(abs, t);
  return { artRel, status: "engine-wired", mod, docId, opId, diffId, packId, sprId, extension };
}

function emptyExemptionSets() {
  let script = readFileSync(scriptPath, "utf8");
  for (const name of [
    "POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS",
    "POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS",
  ]) {
    const re = new RegExp(
      `(const ${name}[^=]*=\\s*new Set(?:<[^>]*>)?\\(\\s*\\[)([\\s\\S]*?)(\\]\\s*\\))`,
    );
    script = script.replace(re, `$1$3`);
  }
  writeFileSync(scriptPath, script);
}

const includeExempt = extractSet("POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS");
const registerExempt = extractSet("POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS");

const includeResults = [];
for (const rsRel of includeExempt) {
  const facet = FACETS.find((f) => rsRel.includes(`/${f}/`));
  if (!facet) {
    includeResults.push({ rsRel, status: "unknown-facet" });
    continue;
  }
  includeResults.push(injectInclude(rsRel, facet));
}

const engineResults = [];
for (const artRel of registerExempt) {
  engineResults.push(patchEngine(artRel));
}

emptyExemptionSets();

const summary = {
  includeTotal: includeExempt.length,
  registerTotal: registerExempt.length,
  includeByStatus: Object.groupBy(includeResults, (r) => r.status),
  engineByStatus: Object.groupBy(engineResults, (r) => r.status),
  engineFailures: engineResults.filter((r) => r.status !== "engine-wired"),
  includeNotWired: includeResults.filter(
    (r) => !["wired-grammar", "wired-protocol", "already"].includes(r.status),
  ),
};

writeFileSync(join(ticket, "🧪p6-wire-specs-log.json"), JSON.stringify(summary, null, 2));
console.log(JSON.stringify(summary, null, 2));
