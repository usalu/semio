// 🧮️ WP0 closure census of semio-framework-os-renderer-wgpu: lib and test closures per cfg through the canonical counter, optional per-file listing.
import {readFileSync, writeFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const PKG = process.env.PKG ?? ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust";
const ENTRY = process.env.ENTRY ?? PKG + "/📚️library/🦀️.rs";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgOf = (target?: string) => execFileSync(rustc, ["--print", "cfg", ...(target ? ["--target", target] : [])], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
let stubbed = 0;
const read = async (p: string): Promise<Uint8Array | null> => {
  try { return readFileSync(p); } catch (e) {
    if ((e as NodeJS.ErrnoException).code === "ENOENT") { if (!p.endsWith(".rs")) { stubbed++; return new Uint8Array(0); } return null; }
    throw e;
  }
};
const rows: [string, string | undefined, string[]][] = [
  ["lib native", undefined, []],
  ["lib wasm32-unknown-unknown", "wasm32-unknown-unknown", []],
  ["lib wasm32-wasip2", "wasm32-wasip2", []],
  ["test native", undefined, ["test"]],
  ["test wasm32-unknown-unknown", "wasm32-unknown-unknown", ["test"]],
];
const only = process.env.ROW;
const listOut = process.env.LIST;
const listing: Record<string, unknown[]> = {};
for (const [label, target, extra] of rows) {
  if (only && label !== only) continue;
  stubbed = 0;
  const cfg = [...cfgOf(target), "debug_assertions", ...extra];
  const inputs = await readRustSourceClosureV1([ENTRY], ROOT, cfg, read, {manifestDirectory: PKG, generated: () => undefined});
  const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
  const emb = inputs.filter(i => i.kind === "embedded");
  console.log([label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, "stubbed", stubbed].join("\t"));
  listing[label] = inputs.map(i => ({kind: i.kind, path: (i as any).path ?? (i as any).file ?? "", bytes: i.bytes, lines: i.activeLines}));
}
if (listOut) writeFileSync(listOut, JSON.stringify(listing, null, 1));
