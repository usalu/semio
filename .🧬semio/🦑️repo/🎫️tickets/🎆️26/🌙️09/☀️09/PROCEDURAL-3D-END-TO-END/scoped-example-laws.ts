import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join, relative, resolve } from "node:path";

const root = resolve(process.argv[2] ?? ".", "✏️s/🧑‍💻dev/🧩️composition");
const owner = join(root, "📦️packages/🦀️rust");
const test = join(root, "🧪️tests/🧩️generation3d-example-geometry/🦀️.rs");
const output = join(import.meta.dir, "🗑️generated/bridge-current", process.argv[2] ? "frozen-laws" : "geometry-laws");
const parse = async (dir: string) => Bun.TOML.parse(await readFile(join(dir, "Cargo.toml"), "utf8"));
const config = await parse(owner), workspace = await parse(root);
const sources = {...config.dependencies, ...config["dev-dependencies"]};
const dependencies = [
  "semio-framework", "semio-framework-artifact-flow-flow", "semio-framework-job", "semio-framework-pack-json",
  "semio-framework-value", "semio-os-flow", "semio-plugin", "semio-s-artifact-procedural-3d-editor-config",
  "semio-s-artifact-procedural-3d-io-text", "semio-s-artifact-procedural-3d-mode-edit", "semio-s-artifact-procedural-3d-model",
  "semio-s-artifact-procedural-3d-preview-eval", "semio-s-artifact-procedural-3d-viewer-config",
  "semio-s-artifact-procedural-3d-viewer-modes", "semio-s-plugin-flow-brep", "semio-s-plugin-flow-math",
  "semio-s-spatial-kernel-semio-session", "serde", "serde_json", "ui_scene", "parry3d",
];
const entries = dependencies.map(name => {
  const item = sources[name];
  const spec = item.workspace ? workspace.workspace.dependencies[name] : item;
  if (typeof spec === "string") return name + " = " + JSON.stringify(spec);
  const fields = {...spec};
  if (fields.path) fields.path = relative(output, resolve(item.workspace ? root : owner, fields.path)).replaceAll("\\", "/");
  const features = [...new Set([...(spec.features ?? []), ...(item.features ?? []), ...(name === "semio-framework-job" ? ["testkit"] : [])])];
  if (features.length) fields.features = features;
  return name + " = { " + Object.entries(fields).map(([key,value]) => key + " = " + JSON.stringify(value)).join(", ") + " }";
});
await mkdir(output, {recursive:true});
const target = relative(output, test).replaceAll("\\", "/");
await writeFile(join(output, "Cargo.toml"), ['[workspace]', 'resolver="2"', '[package]', 'name="generation3d-example-verification"', 'edition="2021"', 'version="0.1.0"', '[[test]]', 'name="generation3d-example-geometry"', 'path='+JSON.stringify(target), '[dev-dependencies]', ...entries].join("\n")+"\n");
await writeFile(join(output, "Cargo.lock"), await readFile(join(root, "Cargo.lock")));
