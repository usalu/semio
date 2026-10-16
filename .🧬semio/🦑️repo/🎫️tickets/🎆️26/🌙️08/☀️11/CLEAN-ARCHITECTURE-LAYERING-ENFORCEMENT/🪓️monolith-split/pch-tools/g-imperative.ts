// 📜️ Imperative: the five extension crates own their sources; the procedure snapshot depends on them instead of path-mounting their lib roots.
import {DRY, log, put, replaceOnce, addToSection, lockEnsure, relPosix, depPackages, packageNameOf, read} from "./dd.ts";
const I = "✏️s/🔌️plugins/📜️imperative";
const A = `${I}/🗿️artifacts/📜️procedure`;
const SNAP = `${A}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust`;
const SNAP_PKG = "semio-s-artifact-imperative-procedure-snapshot";
const extensions = ["🎮️control", "📣️effect", "🧠️logic", "🧮️math", "📝️text"];
const names = new Map<string, string>();
for (const e of extensions) {
  const dir = `${I}/🧩️extensions/${e}/📦️packages/🦀️rust`;
  const name = packageNameOf(`${dir}/Cargo.toml`);
  names.set(e, name);
  addToSection(`${SNAP}/Cargo.toml`, "[dependencies]", `${name} = { path = "${relPosix(SNAP, dir)}" }`);
  lockEnsure(`${A}/Cargo.lock`, name, "0.1.0", depPackages(`${dir}/Cargo.toml`, `${I}/🧩️extensions/${e}/Cargo.toml`), [SNAP_PKG]);
}
const ident = (e: string) => names.get(e)!.replaceAll("-", "_");
const alias = (e: string) => e.replace(/^[^a-z]+/u, "").replace(/[^a-z]+$/u, "");
const root = `${SNAP}/🦀️.rs`;
const text = read(root);
const mountsStart = text.indexOf('#[path = "."]\npub mod extensions {');
if (mountsStart < 0) throw new Error("extensions block not found");
let block = text.slice(mountsStart, text.indexOf("mod leaf_tests;", mountsStart) + "mod leaf_tests;".length);
const lines: string[] = [];
for (const e of extensions) {
  const a = alias(e);
  lines.push(`    pub use ${ident(e)} as ${a};`);
}
const testsMount = block.slice(block.indexOf("#[cfg(test)]"));
const replacement = `pub mod extensions {\n${[...lines].sort().join("\n")}\n\n${testsMount}`;
replaceOnce(root, block, replacement);
const effectTests = `${I}/🧩️extensions/📣️effect/🧪️tests/🔬️unit/🦀️.rs`;
replaceOnce(effectTests, "use crate::extensions::effect::{EXTENSION_ID};\nuse crate::extensions::effect::*;\n", "use super::*;\n");
log(DRY ? "dry run" : "written");
