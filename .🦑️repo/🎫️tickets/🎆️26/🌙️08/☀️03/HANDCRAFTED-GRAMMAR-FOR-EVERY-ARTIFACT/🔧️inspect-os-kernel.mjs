import { readFileSync, readdirSync, existsSync } from "fs";
import { join } from "path";
function findNamed(root, needle) {
  const hit = readdirSync(root).find((e) => e.includes(needle));
  if (!hit) throw new Error(`no ${needle} in ${root}`);
  return join(root, hit);
}
const pkg = findNamed(findNamed(findNamed(findNamed(".", "framework"), "products"), "os"), "packages");
const rust = findNamed(pkg, "rust");
console.log("rust entries", readdirSync(rust));
const glue = join(rust, "📦️glue.rs");
const cargo = join(rust, "Cargo.toml");
console.log("glue exists", existsSync(glue));
const g = readFileSync(glue, "utf8");
console.log("glue len", g.length);
// show dsl-related mods
for (const line of g.split("\n")) {
  if (/dsl|grammar|LanguageSpec|passthrough/i.test(line)) console.log(line);
}
console.log("--- cargo lib ---");
console.log(readFileSync(cargo, "utf8").slice(0, 800));
// how do plugins depend on dsl?
const dagCargo = findNamed(findNamed(findNamed(findNamed(findNamed(".", "s"), "plugins"), "dag"), "packages"), "rust");
console.log("dag cargo", readFileSync(join(dagCargo, "Cargo.toml"), "utf8"));
