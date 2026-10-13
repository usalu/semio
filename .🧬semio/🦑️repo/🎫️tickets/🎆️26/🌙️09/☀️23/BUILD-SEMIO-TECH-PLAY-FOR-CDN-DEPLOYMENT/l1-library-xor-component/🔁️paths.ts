/** 🔁️ Wave L1 consumer codemod: every reference to a component's deliverable (its `dist` directory, its `.wasm` name, its Cargo package in
 * build requests) moves from the library crate to the `🧩️component` crate. Usage: `bun 🔁️paths.ts [--apply]`. */
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { execSync } from "node:child_process";
import { repoRoot } from "./🗺️inventory.ts";

const apply = process.argv.includes("--apply");
const EXCLUDED = [":!.🧬semio", ":!.cursor", ":!*.lock", ":!🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧫️fixtures"];
const components = execSync("git ls-files --others --exclude-standard -- '*/🧩️component/📦️packages/🦀️rust/Cargo.toml'", { cwd: repoRoot, encoding: "utf8" }).trim().split("\n").filter(Boolean)
  .map((manifest) => ({ manifest, text: readFileSync(join(repoRoot, manifest), "utf8") }))
  .filter(({ text }) => text.includes("[package.metadata.component]"))
  .map(({ manifest, text }) => {
    const component = text.match(/^name = "([^"]+)"/m)![1]!, library = component.replace(/-component$/, "");
    return { owner: manifest.replace(/\/🧩️component\/📦️packages\/🦀️rust\/Cargo\.toml$/, ""), component, library };
  });
const grep = (pattern: string): string[] => {
  try { return execSync(`git grep -l -E ${JSON.stringify(pattern)} -- ${EXCLUDED.map((spec) => JSON.stringify(spec)).join(" ")}`, { cwd: repoRoot, encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }).trim().split("\n").filter(Boolean); } catch { return []; }
};
const escape = (text: string): string => text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const changed = new Map<string, string>();
const read = (path: string): string => changed.get(path) ?? readFileSync(join(repoRoot, path), "utf8");
const files = new Set([...grep("📦️packages/🦀️rust/dist"), ...grep("(semio_hub|semio_s_plugin)_[a-z0-9_]+\\.wasm"), ...grep("cargoPackage: \"semio-(hub|s-plugin)-")]);
for (const path of files) {
  if (!existsSync(join(repoRoot, path))) continue;
  let text = read(path);
  for (const { owner, component, library } of components) {
    text = text.replaceAll(`${owner}/📦️packages/🦀️rust/dist`, `${owner}/🧩️component/📦️packages/🦀️rust/dist`);
    text = text.replace(new RegExp(`(?<![A-Za-z0-9_])${escape(library.replaceAll("-", "_"))}\\.wasm`, "g"), `${component.replaceAll("-", "_")}.wasm`);
    text = text.replace(new RegExp(`cargoPackage: "${escape(library)}"`, "g"), `cargoPackage: "${component}"`);
  }
  if (text !== readFileSync(join(repoRoot, path), "utf8")) changed.set(path, text);
}
for (const path of changed.keys()) console.log(path);
console.log(`[DEBUG] components=${components.length} changed=${changed.size}`);
if (apply) { for (const [path, text] of changed) writeFileSync(join(repoRoot, path), text); console.log("[DEBUG] applied"); }
