// keys3-g: per cached target of the universe, from the sealed generation: does its own-project input expansion carry a cargo resolution token (SEMIO_CARGO_LOCK_*), does its command closure reach a cargo driver, what is its execution kind. Streams, RSS guard 500 MB.
// usage: bun lockcov.mjs <out.jsonl> <projects.txt>
import {readFileSync, writeFileSync, appendFileSync} from "node:fs"; import {brotliDecompressSync} from "node:zlib";
const [out, list] = process.argv.slice(2);
const wanted = new Set(readFileSync(list, "utf8").split("\n").filter(Boolean));
const g = "/Users/ueli/Documents/semio/.cache/b/g/m/generations/"; const cur = JSON.parse(readFileSync(g + "current.json", "utf8")); const m = JSON.parse(readFileSync(g + "m/" + cur.manifest + ".json", "utf8"));
const DRIVERS = ["/🏃️process/🧪️testing/🦀️cargo/", "/🏃️process/📦️artifacts/🏗️native-build/", "/🏃️process/📦️artifacts/🕸️wasm-build/", "⚡️caching/🦀️cargo/📜️script.ts", "🏃️process/🦀️cargo/"];
writeFileSync(out, "");
for (const [name, row] of Object.entries(m.owners)) {
  if (row.kind !== "project" || !wanted.has(name)) continue;
  let f; try { f = JSON.parse(brotliDecompressSync(readFileSync(g + "o/" + row.fragment + ".json.br")).toString("utf8")); } catch { continue; }
  const named = f.node?.namedInputs ?? {};
  const expand = (items, seen = new Set(), acc = {files: [], tokens: []}) => {
    for (const item of items ?? []) {
      if (typeof item === "string") { if (Object.hasOwn(named, item)) { if (!seen.has(item)) { seen.add(item); expand(named[item], seen, acc); } } else if (!item.startsWith("^")) acc.files.push(item); }
      else if (item && typeof item === "object") { if (typeof item.env === "string") acc.tokens.push(item.env); else if (typeof item.input === "string" && !item.projects && Object.hasOwn(named, item.input) && !seen.has(item.input)) { seen.add(item.input); expand(named[item.input], seen, acc); } }
    }
    return acc;
  };
  for (const [target, d] of Object.entries(f.node?.targets ?? {})) {
    if (!(d.cache === true || (d.cache && typeof d.cache === "object"))) continue;
    const acc = expand(d.inputs ?? ["default"]);
    const lock = acc.tokens.some((t) => t.startsWith("SEMIO_CARGO_LOCK_"));
    const closureFiles = (token) => { try { return JSON.parse(readFileSync("/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/" + token.slice("SEMIO_SOURCE_CLOSURE_".length) + ".json", "utf8")).files ?? []; } catch { return []; } }; const everything = [...acc.files, ...acc.tokens.filter((t) => t.startsWith("SEMIO_SOURCE_CLOSURE_")).flatMap(closureFiles)]; const driver = everything.some((p) => DRIVERS.some((x) => p.includes(x)));
    appendFileSync(out, JSON.stringify({project: name, target, kind: d.metadata?.semio?.execution?.kind ?? null, lock, driver, rule: d.metadata?.semio?.inputPrecision?.rule ?? null, cargoCrate: Object.hasOwn(named, "cargoToolchain"), command: (d.options?.command ?? "").slice(0, 120)}) + "\n");
  }
  if (process.memoryUsage().rss > 500 * 1048576) { console.error("RSS above 500 MB"); process.exit(3); }
}
