/** 🏭️ Wave L2: moves every artifact's feature-gated `[[bin]]` mutation bridge into its own tool crate at
 * `<artifact>/🏭️bridge/📦️packages/🦀️rust` (own workspace root, like the plugin-level bridges), so the artifact library loses its
 * `mutation-inventory` feature. Usage: `bun 🏭️bridges.ts [--apply]`. */
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { join, relative } from "node:path";
import { repoRoot } from "./🗺️inventory.ts";

const ARTIFACTS = ["🀄️wfc/🗿️artifacts/◻️2d", "🀄️wfc/🗿️artifacts/🧊️3d", "🀄️wfc/🗿️artifacts/🖼️bitmap", "🀄️wfc/🗿️artifacts/🔲️grid2d", "🀄️wfc/🗿️artifacts/🧱️grid3d", "🪐️space/🗿️artifacts/🪐️space", "🪐️space/🗿️artifacts/🏠️home"].map((path) => `✏️s/🔌️plugins/${path}`);
const INVENTORY = "🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/📦️packages/🦀️rust";
const KERNEL = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust";

const apply = process.argv.includes("--apply");
for (const artifact of ARTIFACTS) {
  const libraryManifest = join(artifact, "📦️packages/🦀️rust/Cargo.toml");
  const library = readFileSync(join(repoRoot, libraryManifest), "utf8");
  const libraryName = library.match(/^name = "([^"]+)"/m)![1]!;
  const bin = library.match(/\n\[\[bin\]\]\nname = "([^"]+)"\npath = "\.\.\/\.\.\/🏭️bridge\/🦀️\.rs"\n(required-features = \["mutation-inventory"\]\n)?/);
  if (!bin) throw new Error(`${libraryManifest}: no bridge [[bin]]`);
  const crateDir = join(artifact, "🏭️bridge/📦️packages/🦀️rust");
  const up = (target: string) => relative(join(repoRoot, crateDir), join(repoRoot, target));
  const manifest = `# 🧭️ Own workspace root, like the plugin-level bridges: a tool binary that reads this artifact's production dispatch aggregates.
[workspace]

[package]
name = "${libraryName}-mutation-bridge"
version = "0.1.0"
edition = "2021"
publish = false
description = "🏭️ Owner-local production mutation inventory of \`${libraryName}\`."

[package.metadata.semio]
role = "tool"

[[bin]]
name = "${bin[1]}"
path = "../../🦀️.rs"

[dependencies]
${libraryName} = { path = "${up(join(artifact, "📦️packages/🦀️rust"))}" }
semio-framework-os-kernel = { path = "${up(KERNEL)}" }
semio-framework-test-mutation-inventory = { path = "${up(INVENTORY)}" }
`;
  const nextLibrary = library.replace(bin[0], "\n");
  const scriptPath = join(artifact, "🏭️bridge/📜️script.ts");
  let script = readFileSync(join(repoRoot, scriptPath), "utf8");
  const before = script;
  script = script.replace('resolve(import.meta.dir,"../📦️packages/🦀️rust/Cargo.toml")', 'resolve(import.meta.dir,"📦️packages/🦀️rust/Cargo.toml")');
  script = script.replace(`packages:["${libraryName}"]`, `packages:["${libraryName}-mutation-bridge"]`);
  script = script.replace(/"--features","(component-app-assembly,)?mutation-inventory",/g, "");
  if (script === before || script.includes("mutation-inventory\"")) throw new Error(`${scriptPath}: unexpected script shape`);
  console.log(`${libraryName}: bin ${bin[1]} -> ${crateDir}`);
  if (!apply) continue;
  if (existsSync(join(repoRoot, crateDir, "Cargo.toml"))) throw new Error(`${crateDir} exists`);
  mkdirSync(join(repoRoot, crateDir), { recursive: true });
  writeFileSync(join(repoRoot, crateDir, "Cargo.toml"), manifest);
  writeFileSync(join(repoRoot, libraryManifest), nextLibrary.replace(/\n{3,}/g, "\n\n"));
  writeFileSync(join(repoRoot, scriptPath), script);
}
