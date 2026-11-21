/** 🧾 Measures the current BREP owners using the repository Rust source reader. */
import {readFileSync} from "node:fs";
import {dirname, normalize, resolve} from "node:path";
const {readRustSourceClosureV1} = await import(resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const run = process.argv[2];
const graph = JSON.parse(readFileSync(resolve(run, "l/unit-graph-prototype-00.json"), "utf8"));
const toPath = (unit: {pkg_id: string}) => decodeURIComponent(unit.pkg_id.split("path+file://")[1]?.split("#")[0] ?? "");
const cfg = ["debug_assertions", "fmt_debug=full", "overflow_checks", "panic=unwind", "relocation_model=pic", "target_abi=", "target_arch=wasm32", "target_endian=little", "target_env=p2", "target_family=wasm", "target_feature=bulk-memory", "target_feature=crt-static", "target_feature=multivalue", "target_feature=mutable-globals", "target_feature=nontrapping-fptoint", "target_feature=reference-types", "target_feature=sign-ext", "target_os=wasi", "target_pointer_width=32", "target_vendor=unknown"];
for (const unit of graph.units) {
  const root = toPath(unit);
  if (unit.platform !== "wasm32-wasip2" || !root.includes("/📐️brep/")) continue;
  const name = unit.pkg_id.split("#").at(-1).split("@")[0];
  try {
    const sources = await readRustSourceClosureV1([normalize(unit.target.src_path)], process.cwd(), cfg, async (path: string) => {
      try { return readFileSync(path); } catch (error: unknown) {
        if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
        throw error;
      }
    }, {manifestDirectory: root || dirname(unit.target.src_path), generated: () => {}});
    const bytes = sources.reduce((total, row) => total + row.bytes, 0);
    const lines = sources.reduce((total, row) => total + row.activeLines, 0);
    if (lines >= 3500 || bytes >= 220000 || sources.length >= 28 || /representation|solid-transaction|mesh-io/.test(name))
      console.log("[DEBUG]", name, sources.length, lines, bytes);
  } catch (error) {
    console.log("[DEBUG]", name, String(error));
    process.exitCode = 1;
  }
}
