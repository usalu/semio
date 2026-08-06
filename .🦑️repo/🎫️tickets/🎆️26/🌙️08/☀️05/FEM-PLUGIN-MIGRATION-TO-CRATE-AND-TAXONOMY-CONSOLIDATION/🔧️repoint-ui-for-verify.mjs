import { readFileSync, writeFileSync, existsSync } from "fs";
import { join } from "path";

const ticket = process.argv[1];
const cores = [
  "/Users/ueli/Documents/semio/🧰️framework/📦️packages/🦀️rust/Cargo.toml",
  "/Users/ueli/Documents/semio/🧰️framework/⚡️implementations/🦀️rust/Cargo.toml",
];
const oldNeedle = 'ui_wgpu = { path = "../../🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🎗wgpu", package = "semio-framework-ui-wgpu" }';
// actual emoji from file
const backups = [];
for (const p of cores) {
  if (!existsSync(p)) { console.log("skip missing", p); continue; }
  let t = readFileSync(p, "utf8");
  const m = t.match(/^ui_wgpu = \{[^}]+\}/m);
  if (!m) { console.log("no ui_wgpu in", p); continue; }
  const bak = join(ticket, "scratch-core-ui-" + (p.includes("packages") ? "packages" : "impl") + ".toml.line");
  writeFileSync(bak, m[0]);
  const neu = 'ui_wgpu = { path = "../../🔨️modules/🖱️ui/📦️packages/🦀️rust", package = "semio-framework-ui", features = ["wgpu"] }';
  t = t.replace(m[0], neu);
  // typegen feature path may need update
  t = t.replace('ui_wgpu/typegen', 'ui_wgpu/typegen'); // same feature name on merged
  writeFileSync(p, t);
  console.log("repointed", p);
  console.log("  from", m[0]);
  console.log("  to  ", neu);
  backups.push(bak);
}
writeFileSync(join(ticket, "scratch-ui-repoints.json"), JSON.stringify(backups, null, 2));
