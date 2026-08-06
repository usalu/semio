import { readFileSync, readdirSync } from "fs";
import { join } from "path";
function findNamed(root, needle) {
  const entries = readdirSync(root);
  const hit = entries.find((e) => e === needle || e.endsWith(needle))
    || entries.filter((e) => e.includes(needle)).sort((a,b)=>a.length-b.length)[0];
  if (!hit) throw new Error(`no ${needle} in ${root}: ${entries.slice(0,12)}`);
  return join(root, hit);
}
const s = readdirSync(".").find((e) => e.length <= 4 && e.endsWith("s") && !e.includes("story"));
const plugins = findNamed(s, "plugins");
const dag = findNamed(plugins, "dag");
const packages = findNamed(dag, "packages");
const rust = findNamed(packages, "rust");
console.log(readFileSync(join(rust, "Cargo.toml"), "utf8"));
// check component.rs LanguageSpec and grammar tests once more
const dslComp = "�