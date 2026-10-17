// 🌳️ c4s2 module-tree probe: the readers of the registry (dashboard reuse test and registry test) now read the module tree; their needles and the extracted `fn launch` definition must equal what the unsplit snapshot gave.
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {readModuleTreeSource, listModuleTree} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import {rustTokens, rustTokenPairs} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const root = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard";
function definition(source: string, name: string): string | undefined {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), start = tokens.findIndex((token, index) => token.text === "fn" && tokens[index + 1]?.text === name);
  if (start < 0) return undefined;
  const open = tokens.findIndex((token, index) => index > start && token.text === "{"), close = pairs.get(open);
  if (open < 0 || close === undefined) throw Error("body " + name);
  return source.slice(tokens[start]!.start, tokens[close]!.end);
}
const orig = readFileSync(join(import.meta.dir, "orig", "reg.rs"), "utf8");
const tree = readModuleTreeSource(join(root, "🎮️registry/🦀️.rs"));
const files = listModuleTree(join(root, "🎮️registry/🦀️.rs"));
console.log("registry tree files", files.length);
for (const name of ["launch", "process", "member", "parameters"]) {
  const a = definition(orig, name), b = definition(tree, name);
  console.log("fn", name, "orig", a === undefined ? "missing" : a.length, "tree", b === undefined ? "missing" : b.length, "equal", a === b);
}
for (const [label, source] of [["registry", tree], ["inventory", readModuleTreeSource(join(root, "📚️inventory/🦀️.rs"))]] as const) {
  console.log(label, "graph_file", source.includes("graph_file"), "cache/build/graph.json", source.includes("cache/build/graph.json"));
}
console.log("orig registry graph_file", orig.includes("graph_file"), "cache/build/graph.json", orig.includes("cache/build/graph.json"));
console.log("inventory (single file) cache/build/graph.json", readFileSync(join(root, "📚️inventory/🦀️.rs"), "utf8").includes("cache/build/graph.json"));
