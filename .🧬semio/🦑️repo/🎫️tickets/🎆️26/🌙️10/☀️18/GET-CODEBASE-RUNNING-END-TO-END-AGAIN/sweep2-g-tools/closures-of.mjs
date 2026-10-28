// sweep2-g: which closure tokens a sealed target key carries and which of them contain the probe files. usage: bun closures-of.mjs <project> <target> <probe-substring>...  (offline, RSS-light)
import {readFileSync, existsSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const ROOT = "/Users/ueli/Documents/semio", generations = ROOT + "/.cache/b/g/m/generations/";
const [project, target, ...probes] = process.argv.slice(2);
const cur = JSON.parse(readFileSync(generations + "current.json", "utf8"));
const manifest = JSON.parse(readFileSync(generations + "m/" + cur.manifest + ".json", "utf8"));
const fragment = JSON.parse(brotliDecompressSync(readFileSync(generations + "o/" + manifest.owners[project].fragment + ".json.br")).toString("utf8"));
const names = fragment.node.namedInputs ?? {}, seen = new Set(), tokens = new Set();
const walk = (list) => { for (const item of list ?? []) { if (typeof item === "string") { if (names[item] && !seen.has(item)) { seen.add(item); walk(names[item]); } } else if (item?.env?.startsWith?.("SEMIO_SOURCE_CLOSURE_")) tokens.add(item.env); } };
walk(fragment.node.targets[target].inputs);
for (const name of seen) { void name; }
console.log(`${project}:${target} generation ${cur.generation} namedInputs ${[...seen].join(",")} tokens ${tokens.size}`);
for (const token of tokens) {
  const path = `${ROOT}/.cache/b/g/m/build-input-closures/${token.slice("SEMIO_SOURCE_CLOSURE_".length)}.json`;
  if (!existsSync(path)) { console.log(token.slice(-8), "MISSING"); continue; }
  const closure = JSON.parse(readFileSync(path, "utf8")), files = closure.files ?? [];
  const hits = probes.map((probe) => files.filter((file) => file.includes(probe)).length);
  console.log(token.slice(-8), files.length, Object.keys(closure).join(","), "probes", hits.join("/"), "entry", JSON.stringify(closure.entry ?? closure.entries ?? closure.roots ?? closure.root ?? "").slice(0, 160));
}
