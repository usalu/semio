// 🔮️ Independent serde_json oracle helpers shared by sibling extension crates get one owner crate each (standalone workspace, like the drawing oracle); every extension depends on it instead of path-mounting the source.
import {DRY, log, put, read, replaceOnce, addToSection, lockEnsure, lockClosureFile, ownerRowsAddViaGenerator, relPosix, packageNameOf, abs} from "./dd.ts";
import {existsSync} from "node:fs";
const FW = "🧰️framework/🔨️modules";
type Family = {plugin: string; owner: string; name: string; description: string; extra: {name: string; dir: string}[]; extensions: string[]; mountFile: (ext: string) => string; mount: string; alias: string; supportFile: string};
const families: Family[] = [
  {plugin: "✏️s/🔌️plugins/🏭️process/🧩️extensions", owner: "✏️s/🔌️plugins/🏭️process/🧩️extensions/🧪️tests/🔬️catalog-json/📦️packages/🦀️rust", name: "semio-s-plugin-process-catalog-json-oracle", description: "Independent serde_json oracle of the process machine catalog JSON, shared by the process extension crates", extra: [{name: "semio-framework-pack-json", dir: `${FW}/🎒️pack/🔤️json/📦️packages/🦀️rust`}], extensions: ["🔩️metal", "🤖️robotic", "🧱️concrete", "🪵️wood"], mountFile: e => `✏️s/🔌️plugins/🏭️process/🧩️extensions/${e}/🧪️tests/🔬️unit/🦀️.rs`, mount: '#[path = "../../../🧪️tests/🔬️catalog-json/🦀️.rs"]\nmod json_oracle;', alias: "json_oracle", supportFile: "✏️s/🔌️plugins/🏭️process/🧩️extensions/🧪️tests/🔬️catalog-json/🦀️.rs"},
  {plugin: "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions", owner: "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧪️tests/🔬️unit/🔮️oracle/📦️packages/🦀️rust", name: "semio-s-plugin-sourcing-catalog-oracle", description: "Independent serde_json oracle of the sourcing typology and kind catalogs, shared by the sourcing extension crates", extra: [], extensions: ["🧱️slabs", "🪟️windows", "🪵️beams"], mountFile: e => `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/${e}/🧪️tests/🔬️unit/🦀️.rs`, mount: '#[path = "../../../🧪️tests/🔬️unit/🔮️oracle/🦀️.rs"]\nmod json_oracle;', alias: "json_oracle", supportFile: "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧪️tests/🔬️unit/🔮️oracle/🦀️.rs"},
];
for (const f of families) {
  const ident = f.name.replaceAll("-", "_");
  const manifest = `[workspace]
members = ["."]

[workspace.metadata.semio.repository]
exclude-patterns = []
schema-version = 1
member-manifests = ["Cargo.toml"]

[package]
name = "${f.name}"
version = "0.1.0"
edition = "2021"
publish = false
description = "${f.description}"

[package.metadata.semio]
role = "test"

[lib]
name = "${ident}"
path = "../../🦀️.rs"

[dependencies]
serde_json = "1"
${f.extra.map(e => `${e.name} = { path = "${relPosix(f.owner, e.dir)}" }`).join("\n")}
`.replace(/\n\n$/u, "\n");
  put(`${f.owner}/Cargo.toml`, manifest, true);
  const src = read(f.supportFile);
  if (src.includes("pub(super) fn ")) put(f.supportFile, src.replaceAll("pub(super) fn ", "pub fn "));
  const deps = ["serde_json", ...f.extra.map(e => e.name)].sort();
  let first = true;
  for (const ext of f.extensions) {
    const dir = `${f.plugin}/${ext}`;
    const crate = `${dir}/📦️packages/🦀️rust`;
    replaceOnce(f.mountFile(ext), f.mount, `use ${ident} as ${f.alias};`);
    addToSection(`${crate}/Cargo.toml`, "[dev-dependencies]", `${f.name} = { path = "${relPosix(crate, f.owner)}" }`);
    if (first && !DRY) put(`${f.owner}/Cargo.lock`, lockClosureFile(`${dir}/Cargo.lock`, f.name, "0.1.0", deps), true);
    if (first && DRY) log(`(dry) create ${f.owner}/Cargo.lock`);
    first = false;
    lockEnsure(`${dir}/Cargo.lock`, f.name, "0.1.0", deps, [packageNameOf(`${crate}/Cargo.toml`)]);
  }
  await ownerRowsAddViaGenerator([{name: f.name, root: f.owner}]);
}
void abs; void existsSync;
log(DRY ? "dry run" : "written");
