// check-gen: bun check-gen.ts [generationsDir] : graph-free look at the sealed generation: which cached targets carry the input precision decision of keys2-g rules.
import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const g = process.argv[2] ?? process.cwd() + "/.cache/b/g/m/generations/";
const cur = JSON.parse(readFileSync(g + "current.json", "utf8")), man = JSON.parse(readFileSync(g + "m/" + cur.manifest + ".json", "utf8"));
const wanted = ["@semio-tech/print", "@semio-tech/framework-rs", "@semio-tech/repo-dashboard-rs"];
console.log("generation", cur.generation);
for (const name of wanted) {
  const row = man.owners[name];
  if (!row) { console.log(name, "missing"); continue; }
  const fragment = JSON.parse(brotliDecompressSync(readFileSync(g + "o/" + row.fragment + ".json.br")).toString("utf8"));
  if (fragment.kind !== "project") { console.log(name, fragment.kind, String(fragment.node?.reason ?? "").slice(0, 160)); continue; }
  const rules = Object.entries(fragment.node.targets).map(([target, definition]: [string, any]) => [target, definition.metadata?.semio?.inputPrecision?.rule]).filter(([, rule]) => rule);
  console.log(name, "targets with a precision decision:", rules.length, JSON.stringify(rules.slice(0, 40)));
}
