import { measureRustCrate, crateTreeHostCfg, crateTreeWasmCfg } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
for (const dir of process.argv.slice(2)) {
  for (const [label, cfg] of [["host", crateTreeHostCfg()], ["wasm", crateTreeWasmCfg()]] as const) {
    const m = await measureRustCrate(root, `${root}/${dir}/🦀️.rs`, false, new Map(), `${root}/${dir}/📦️packages/🦀️rust`, cfg);
    console.log(label, m.files, m.bytes, m.lines, dir.split("/").slice(-2).join("/"));
  }
}
