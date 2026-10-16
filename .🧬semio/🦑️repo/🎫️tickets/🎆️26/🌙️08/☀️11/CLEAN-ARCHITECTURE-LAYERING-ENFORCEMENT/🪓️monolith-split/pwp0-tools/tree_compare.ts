// 🌳️ The TS module-tree view of the plugin root after the mount hubs: file set against the gate closure of the original tree and the text size of the concatenated tree.
import {listModuleTree, readModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import {readFileSync} from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const PLUG = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/";
const closure = JSON.parse(readFileSync(import.meta.dir + "/../🗑️generated/pwp0-closure.json", "utf8"));
const norm = (s: string) => s.normalize("NFC").replaceAll("️", "");
for (const tests of [false, true]) {
  const listed = listModuleTree(PLUG + "🦀️.rs", {tests}).map((p: string) => norm(p.slice(ROOT.length)));
  const want = new Set<string>(closure[tests ? "test native" : "lib native"].filter((x: any) => x.path.endsWith(".rs") && !x.path.includes("📦️packages/")).map((x: any) => norm(x.path)));
  const have = new Set(listed);
  const missing = [...want].filter(x => !have.has(x)), extra = listed.filter((x: string) => !want.has(x));
  console.log(`tests=${tests}: tree files ${listed.length}, gate closure files ${want.size}, in closure but not in tree ${missing.length} ${JSON.stringify(missing.slice(0, 4))}, in tree but not in closure ${extra.length} ${JSON.stringify(extra.slice(0, 10).map((x: string) => x.slice(-60)))}`);
  console.log("hub files in tree:", listed.filter((x: string) => x.includes("🪝mounts")).length);
  console.log("concatenated text bytes:", readModuleTreeSource(PLUG + "🦀️.rs", {tests}).length);
}
