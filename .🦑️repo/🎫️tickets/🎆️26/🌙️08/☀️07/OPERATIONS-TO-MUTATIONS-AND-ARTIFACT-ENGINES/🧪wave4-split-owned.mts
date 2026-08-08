#!/usr/bin/env bun
/** Split owned 🔧️op into 🧬️mutations + slim op re-export. */
import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repo = join(import.meta.dir, "../../../../../..");
const cfg = process.argv[2];
if (!cfg) {
  console.error("usage: wave4-split-owned.mts <json-path>");
  process.exit(1);
}
const { pluginRel, artFolder, artMod, prefix, applyFrom, applyTo } = JSON.parse(readFileSync(join(repo, cfg), "utf8"));
const art = join(repo, pluginRel, "🗿️artifacts", artFolder);
const opPath = join(art, "🔧️op", "🦀️component.rs");
const mutPath = join(art, "🧬️mutations", "🦀️component.rs");
mkdirSync(join(art, "🧬️mutations"), { recursive: true });
let op = readFileSync(opPath, "utf8");
if (!existsSync(mutPath)) {
  let mut = op.replaceAll(applyFrom, applyTo);
  mut = mut.replace(/\/\/#region 🔖️HandcraftedOpCodecs[\s\S]*?\/\/#endregion 🔖️HandcraftedOpCodecs\n/g, "");
  mut = mut.replace(/\/\/#region 🔖️OpText[\s\S]*?\/\/#endregion 🔖️OpText\n/g, "");
  writeFileSync(mutPath, mut.replace("constitutional: op", "🧬️mutations facet"));
}
writeFileSync(
  opPath,
  `//! 🔧 ${artMod} — OpText/OpBinary for \`${prefix}Mutation\`.
pub use crate::artifacts::${artMod}::mutations::{${applyTo}, ${prefix}Mutation};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
`,
);
writeFileSync(join(art, "🧬️mutations", "🟦️component.ts"), `export {};\n`);
console.log("split", artMod);
