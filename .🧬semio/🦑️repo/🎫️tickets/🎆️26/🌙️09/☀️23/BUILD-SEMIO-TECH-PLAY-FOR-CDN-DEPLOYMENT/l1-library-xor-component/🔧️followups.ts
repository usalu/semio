/** 🔧️ Wave L1 follow-up codemod after `🧩️split.ts`: deletes `component-guest`/`component-extension-guest` from the plugin crate and every
 * consumer, renames the over-budget extension libraries and drops `default-features = false` rows that no longer select anything.
 * Usage: `bun 🔧️followups.ts [--apply]`. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { execSync } from "node:child_process";
import { repoRoot } from "./🗺️inventory.ts";

const apply = process.argv.includes("--apply");
const changed = new Map<string, string>();
const read = (path: string): string => changed.get(path) ?? readFileSync(join(repoRoot, path), "utf8");
const edit = (path: string, next: string): void => { if (next !== read(path)) changed.set(path, next); };
const tracked = (pattern: string, pathspec: string[] = []): string[] => {
  try { return execSync(`git grep -l -E ${JSON.stringify(pattern)} -- ${pathspec.map((spec) => JSON.stringify(spec)).join(" ")} ':!.🧬semio' ':!.cursor'`, { cwd: repoRoot, encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }).trim().split("\n").filter(Boolean); } catch { return []; }
};

const PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const SPLIT_LIBRARIES = ["semio-hub-[a-z0-9-]+", "semio-s-plugin-(?:flow|cad|imperative|playbook|process|sourcing)-[a-z0-9-]+"];
const RENAMES: [string, string][] = [
  ...["bim", "brep", "dictionary", "draw", "list", "logic", "math", "primitive", "text"].map((name): [string, string] => [`semio-s-plugin-flow-extension-${name}`, `semio-s-plugin-flow-${name}`]),
  ["semio-s-plugin-cad-aec-building-energy", "semio-s-plugin-cad-aec-energy"],
  ["semio-s-plugin-cad-aec-building-structure", "semio-s-plugin-cad-aec-structure"],
];

for (const path of tracked('component-guest"|component-guest\\b', [`${PLUGIN}/*.rs`, `${PLUGIN}/📦️packages/🦀️rust/Cargo.toml`])) {
  let text = read(path);
  text = text.replace(/all\(\s*any\(\s*feature\s*=\s*"component-guest"\s*,\s*feature\s*=\s*"component-extension-guest"\s*\)\s*,\s*/g, "all(");
  text = text.replace(/all\(\s*feature\s*=\s*"component-guest"\s*,\s*/g, "all(");
  text = text.replace(/^component-guest = \[\]\n/m, "").replace(/^component-extension-guest = \[\]\n/m, "");
  edit(path, text);
}

for (const path of tracked('"component-guest"|semio-framework-plugin/component-guest', ["*Cargo.toml"])) {
  const text = read(path).split("\n").map((line) => line
    .replace(/,\s*features\s*=\s*\["component-guest"\]/g, "")
    .replace(/features\s*=\s*\["component-guest"\]\s*,\s*/g, "")
    .replace(/"component-guest"\s*,\s*/g, "")
    .replace(/,\s*"component-guest"/g, "")
    .replace(/"semio-framework-plugin\/component-guest"\s*,\s*/g, "")
    .replace(/,\s*"semio-framework-plugin\/component-guest"/g, "")
    .replace(/\["semio-framework-plugin\/component-guest"\]/g, "[]")
    .replace(/^component-guest = \[\]$/, "\u0000")).filter((line) => line !== "\u0000").join("\n");
  edit(path, text);
}

const scale = "🧰️framework/🛍️products/💻️os/🧪️testing/⚖️scale";
edit(`${scale}/🦀️.rs`, read(`${scale}/🦀️.rs`).replace(/all\(feature = "component-guest", /g, "all("));
edit(`${scale}/📦️packages/🦀️rust/📜️script.ts`, read(`${scale}/📦️packages/🦀️rust/📜️script.ts`).replace(/, "--features", "component-guest"/g, "").replace(/, "--crate-type", "cdylib"/g, "").replace("component-guest build", "component build"));
const rebuild = `${PLUGIN}/📇️registry/🔁️rebuild/🔣️.json`;
edit(rebuild, read(rebuild).replace(/"semio-framework-plugin\/component-guest", /g, ""));
const actor = "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml";
edit(actor, read(actor).replace("The wasip2 guest path is `semio-framework-plugin`'s `component-guest` feature,", "The wasip2 guest path is `semio-framework-plugin` on `wasm32-wasip2`,"));

for (const [from, to] of RENAMES) {
  const snake = [from.replaceAll("-", "_"), to.replaceAll("-", "_")];
  for (const path of tracked(`${from}|${snake[0]}`)) edit(path, read(path).replaceAll(from, to).replaceAll(snake[0]!, snake[1]!));
}

const library = new RegExp(`^(\\s*(?:${SPLIT_LIBRARIES.join("|")})\\s*=\\s*\\{[^}]*?),\\s*default-features\\s*=\\s*false([^}]*\\})`, "gm");
for (const path of tracked("default-features *= *false", ["*Cargo.toml"])) edit(path, read(path).replace(library, "$1$2"));

for (const [path] of changed) console.log(path);
console.log(`[DEBUG] changed=${changed.size}`);
if (apply) { for (const [path, text] of changed) writeFileSync(join(repoRoot, path), text); console.log("[DEBUG] applied"); }
