import assert from "node:assert/strict";
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { resolve, join, dirname } from "node:path";
import { spawnSync } from "node:child_process";

let repo = import.meta.dir;
while (!existsSync(join(repo, "nx.json"))) repo = dirname(repo);
const generated = join(import.meta.dir, "🗑️generated");
mkdirSync(generated, { recursive: true });
if (process.argv[2] === "parse") {
  const ts = await import(join(repo, "node_modules/typescript/lib/typescript.js"));
  for (const path of ["🧰️framework/🔨️modules/◻️2d/🟦️.ts", "🧰️framework/🔨️modules/🧊️3d/🟦️.ts", "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️.ts", "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts", "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts"]) {
    const parsed = ts.createSourceFile(path, readFileSync(join(repo, path), "utf8"), ts.ScriptTarget.Latest, true);
    assert.equal(parsed.parseDiagnostics.length, 0, path);
  }
  const { default: executor } = await import(join(repo, "node_modules/nx/src/executors/run-commands/run-commands.impl.js"));
  const quote = (value: string) => process.platform === "win32" ? `"${value.replaceAll('"', '\\"')}"` : `'${value.replaceAll("'", "'\\''")}'`;
  const command = ["rustfmt", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", ...process.argv.slice(3)].map(quote).join(" ");
  const result = await executor({ command, cwd: repo, usePty: false, __unparsed__: [] }, { root: repo, cwd: repo, isVerbose: false });
  assert(result.success, "Rust parser");
  console.log(`[DEBUG] TypeScript parser: 5 files; Rust parser: ${process.argv.length - 3} files`);
  process.exit(0);
}
if (process.argv[2] === "checks") {
  const { default: executor } = await import(join(repo, "node_modules/nx/src/executors/run-commands/run-commands.impl.js"));
  const quote = (value: string) => process.platform === "win32" ? `"${value.replaceAll('"', '\\"')}"` : `'${value.replaceAll("'", "'\\''")}'`;
  const commands = [
    ["bun", join(repo, "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts"), "home-io-surface", "source"],
    ["bun", join(repo, "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts"), "home-directory-identity-rows-check"],
    ["bun", import.meta.filename, resolve(process.argv[3])],
  ].map((args) => args.map(quote).join(" "));
  const result = await executor({ commands, parallel: true, cwd: repo, usePty: false, __unparsed__: [] }, { root: repo, cwd: repo, isVerbose: false });
  process.exit(result.success ? 0 : 1);
}
const source = (path: string) => readFileSync(join(repo, path), "utf8");
const host = source("🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs");
const dwg = source("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🦀️.rs");
const slice = (text: string, begin: string, end: string) => {
  const start = text.indexOf(begin);
  const stop = text.indexOf(end, start);
  assert(start >= 0 && stop > start, begin);
  return text.slice(start, stop);
};
const svg = slice(host, "    /// 📏️ One transformed SVG polyline", "    /// @emoji 🧷️ Signature");
const encoder = slice(dwg, "//#region DwgTypes", "//#region DwgRead").replace(/^fn (?:entity_color_of|dwg_decode_r2010_entity|dwg_decode_r2010_entity_common|dwg_decode_r2010_entity_handles)\([\s\S]*?^}\n/gm, "");
const decoder = slice(dwg, "pub fn dwg_from_bytes(", "//#endregion DwgRead");
const helpers = slice(dwg, "pub fn dwg_drawing_to_svg(", "#[cfg(test)]\nmod polyline_io_tests");
const harness = `${svg}\n${encoder}\n${decoder}\n${helpers}\nfn main() {
    let source = r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><g id="walls" transform="translate(1,1)"><rect x="0" y="0" width="4" height="4"/></g></svg>"#;
    let paths = svg_to_polylines(source).unwrap();
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].layer, "walls");
    assert_eq!(paths[0].vertices, vec![[1.0,9.0],[5.0,9.0],[5.0,5.0],[1.0,5.0]]);
    assert!(paths[0].closed);
    let bytes = polylines_to_dwg_bytes(paths.iter().map(|path| (path.layer.as_str(), path.vertices.as_slice(), path.closed))).unwrap();
    let drawing = dwg_from_bytes(&bytes).unwrap();
    assert_eq!(drawing.entities.len(), 1);
    assert_eq!(drawing.layers[drawing.entities[0].layer].name, "walls");
    assert!(matches!(&drawing.entities[0].geometry, DwgGeometry::LwPolyline { vertices, closed: true, .. } if vertices == &paths[0].vertices));
    let (rendered, width, height) = dwg_drawing_to_svg(&drawing).unwrap();
    assert_eq!((width,height), (4,4));
    assert!(rendered.contains("M 0 0 L 4 0 L 4 4 L 0 4 Z"));
    assert!(svg_to_polylines("bad SVG").is_err());
    let empty = polylines_to_dwg_bytes(std::iter::empty()).unwrap();
    assert!(dwg_from_bytes(&empty).unwrap().entities.is_empty());
    println!("[DEBUG] SVG usvg oracle -> artifact DWG bytes -> artifact SVG: {} bytes, layer walls, exact transformed vertices, closed polygon, 4x4 extent; empty and invalid cases passed", bytes.len());
}`;
const input = join(generated, "boundary.rs");
const binary = join(generated, process.platform === "win32" ? "boundary.exe" : "boundary");
writeFileSync(input, harness);
const oracle = resolve(process.argv[2]);
const run = (command: string, args: string[]) => {
  const result = spawnSync(command, args, { cwd: repo, stdio: "inherit", timeout: 60000 });
  if (result.error) throw result.error;
  assert.equal(result.status, 0, command);
};
run("rustc", ["--edition=2021", "--crate-name", "dwg_artifact_boundary", "-Awarnings", input, "-L", `dependency=${dirname(oracle)}`, "--extern", `usvg=${oracle.replace(/[.]rmeta$/, ".rlib")}`, "--extern", `usvg=${oracle}`, "-o", binary]);
run(binary, []);
