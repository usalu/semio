// keys3-g: one JSON row per cached target of the three products from the sealed generation: direct input names, cargoToolchain flag, command, whether the project is a Cargo crate. Streams, RSS guard 500 MB.
// usage: bun inventory.mjs <out.jsonl> <projectRegex>
import {readFileSync, writeFileSync, appendFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const [out, regex] = process.argv.slice(2);
const wanted = { test: ((set) => (name) => set.has(name))(new Set(readFileSync(regex, "utf8").split("\n").filter(Boolean))) };
const generations = "/Users/ueli/Documents/semio/.cache/b/g/m/generations/";
const cur = JSON.parse(readFileSync(generations + "current.json", "utf8"));
const manifest = JSON.parse(readFileSync(generations + "m/" + cur.manifest + ".json", "utf8"));
writeFileSync(out, "");
let rows = 0;
for (const [name, row] of Object.entries(manifest.owners)) {
  if (row.kind !== "project" || !wanted.test(name)) continue;
  let fragment;
  try { fragment = JSON.parse(brotliDecompressSync(readFileSync(generations + "o/" + row.fragment + ".json.br")).toString("utf8")); } catch { continue; }
  const named = fragment.node?.namedInputs ?? {};
  for (const [target, definition] of Object.entries(fragment.node?.targets ?? {})) {
    if (!(definition.cache === true || (definition.cache && typeof definition.cache === "object"))) continue;
    const direct = (definition.inputs ?? []).filter((input) => typeof input === "string" && !input.startsWith("{")).concat((definition.inputs ?? []).filter((input) => input && typeof input === "object").map((input) => input.input ? `${input.input}@${Array.isArray(input.projects) ? input.projects.length : input.projects ?? ""}` : JSON.stringify(input).slice(0, 60)));
    appendFileSync(out, JSON.stringify({project: name, target, generation: cur.generation, root: fragment.root, cargoCrate: Object.hasOwn(named, "cargoToolchain"), command: definition.options?.command ?? definition.execution?.command, cwd: definition.options?.cwd ?? definition.execution?.cwd, executor: definition.executor, files: (definition.inputs ?? []).filter((input) => typeof input === "string" && input.startsWith("{")).length, direct, hasToolchain: direct.includes("cargoToolchain"), hasNativeTest: direct.includes("nativeTestSources"), hasNativeSources: direct.some((item) => item.startsWith("nativeSources")), metadata: definition.metadata?.semio}) + "\n");
    rows++;
  }
  if (process.memoryUsage().rss > 500 * 1048576) { console.error("RSS above 500 MB"); process.exit(3); }
}
console.error(`${rows} cached targets, generation ${cur.generation}`);
